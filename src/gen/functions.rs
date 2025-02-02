use genco::prelude::*;
use uniffi_bindgen::interface::Function;

use super::types::{
    convert_from_rust_buffer, convert_to_rust_buffer, generate_ffi_dart_type, generate_ffi_type,
    generate_type, type_lift_fn, type_lower_fn,
};
use super::utils::{fn_name, var_name};

#[allow(unused_variables)]
pub fn generate_function(api: &str, fun: &Function) -> dart::Tokens {
    let ffi = fun.ffi_func();
    let fn_name = fn_name(fun.name());
    let args = quote!($(for arg in &fun.arguments() => $(generate_type(arg.type_())) $(var_name(arg.name())),));
    let ff_name = ffi.name();
    let inner = quote! {
    rustCall(api, (res) =>
        _$(&fn_name)(
            $(for arg in &fun.arguments() => $(convert_to_rust_buffer(arg.type_(), type_lower_fn(arg.type_(), quote!($(var_name(arg.name())))))),)
        res)
    )
    };

    let (ret, body) = if let Some(ret) = fun.return_type() {
        (
            generate_type(ret),
            quote! {
                return $(type_lift_fn(ret, convert_from_rust_buffer(ret, inner)));
            },
        )
    } else {
        (quote!(void), quote!($inner;))
    };

    quote! {
        late final _$(&fn_name)Ptr = _lookup<
        NativeFunction<
            $(generate_ffi_type(ffi.return_type())) Function(
                $(for arg in &ffi.arguments() => $(generate_ffi_type(Some(&arg.type_()))),)
                Pointer<RustCallStatus>
        )>>($(format!("\"{ff_name}\"")));

        late final _$(&fn_name) = _$(&fn_name)Ptr.asFunction<
        $(generate_ffi_dart_type(ffi.return_type())) Function(
            $(for arg in &ffi.arguments() => $(generate_ffi_dart_type(Some(&arg.type_()))),)
            Pointer<RustCallStatus>
        )>();

        $ret $fn_name ($args) {
            final api = $api;
            $body
        }
    }
}
