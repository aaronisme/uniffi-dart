use genco::prelude::*;
use uniffi_bindgen::interface::{FfiType, Type};

pub fn generate_ffi_type(ret: Option<&FfiType>) -> dart::Tokens {
    let Some(ret_type) = ret else {
        return quote!(Void)
    };
    match *ret_type {
        FfiType::UInt32 => quote!(Uint32),
        FfiType::Int8 => quote!(Uint8),
        FfiType::RustBuffer(ref inner) => match inner {
            Some(i) => quote!($i),
            _ => quote!(RustBuffer),
        },
        FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
        _ => todo!("FfiType::{:?}", ret_type),
    }
}

pub fn generate_ffi_dart_type(ret: Option<&FfiType>) -> dart::Tokens {
    let Some(ret_type) = ret else {
        return quote!(void)
    };
    match *ret_type {
        FfiType::UInt32 => quote!(int),
        FfiType::Int8 => quote!(int),
        FfiType::RustBuffer(ref inner) => match inner {
            Some(i) => quote!($i),
            _ => quote!(RustBuffer),
        },
        FfiType::RustArcPtr(_) => quote!(Pointer<Void>),
        _ => todo!("FfiType::{:?}", ret_type),
    }
}

pub fn generate_type(ty: &Type) -> dart::Tokens {
    match ty {
        Type::UInt8 | Type::UInt32 => quote!(int),
        Type::String => quote!(String),
        Type::Object(name) => quote!($name),
        Type::Boolean => quote!(bool),
        Type::Optional(inner) => quote!($(generate_type(inner))?),
        _ => todo!("Type::{:?}", ty)
        // AbiType::Num(ty) => self.generate_wrapped_num_type(*ty),
        // AbiType::Isize | AbiType::Usize => quote!(int),
        // AbiType::Bool => quote!(bool),
        // AbiType::RefStr | AbiType::String => quote!(String),
        // AbiType::RefSlice(ty) | AbiType::Vec(ty) => {
        //     quote!(List<#(self.generate_wrapped_num_type(*ty))>)
        // }
        // AbiType::Option(ty) => quote!(#(self.generate_type(ty))?),
        // AbiType::Result(ty) => self.generate_type(ty),
        // AbiType::Tuple(tuple) => match tuple.len() {
        //     0 => quote!(void),
        //     1 => self.generate_type(&tuple[0]),
        //     _ => quote!(List<dynamic>),
        // },
        // AbiType::RefObject(ty) | AbiType::Object(ty) => quote!(#ty),
        // AbiType::RefIter(ty) | AbiType::Iter(ty) => quote!(Iter<#(self.generate_type(ty))>),
        // AbiType::RefFuture(ty) | AbiType::Future(ty) => {
        //     quote!(Future<#(self.generate_type(ty))>)
        // }
        // AbiType::RefStream(ty) | AbiType::Stream(ty) => {
        //     quote!(Stream<#(self.generate_type(ty))>)
        // }
        // AbiType::Buffer(ty) => quote!(#(ffi_buffer_name_for(*ty))),
        // AbiType::List(ty) => quote!(#(format!("FfiList{}", ty))),
        // AbiType::RefEnum(ty) => quote!(#(ty)),
    }
}

pub fn convert_from_rust_buffer(ty: &Type, inner: dart::Tokens) -> dart::Tokens {
    match ty {
        Type::Object(_) => inner,
        Type::String | Type::Optional(_) => quote!($(inner).toIntList()),
        _ => inner,
    }
}

pub fn convert_to_rust_buffer(ty: &Type, inner: dart::Tokens) -> dart::Tokens {
    match ty {
        Type::Object(_) => inner,
        Type::String | Type::Optional(_) => quote!(toRustBuffer(api, $inner)),
        _ => inner,
    }
}

pub fn type_lift_fn(ty: &Type, inner: dart::Tokens) -> dart::Tokens {
    match ty {
        Type::UInt32 => inner,
        Type::Boolean => quote!(($inner) > 0),
        Type::String => quote!(liftString(api, $inner)),
        Type::Object(name) => quote!($name.lift(api, $inner)),
        Type::Optional(o) => {
            quote!(liftOptional(api, $inner, (api, v) => $(type_lift_fn(o, quote!(v)))))
        }
        _ => todo!("lift Type::{:?}", ty),
    }
}

pub fn type_lower_fn(ty: &Type, inner: dart::Tokens) -> dart::Tokens {
    match ty {
        Type::UInt32 | Type::Boolean => inner,
        Type::String => quote!(lowerString(api, $inner)),
        Type::Object(name) => quote!($name.lower(api, $inner)),
        Type::Optional(o) => {
            quote!(lowerOptional(api, $inner, (api, v) => $(type_lower_fn(o, quote!(v)))))
        }
        _ => todo!("lower Type::{:?}", ty),
    }
}
