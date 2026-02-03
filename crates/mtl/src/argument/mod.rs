//! Metal argument types for reflection.
//!
//! Corresponds to `Metal/MTLArgument.hpp`.
//!
//! These types provide reflection information about function arguments.

mod argument;
mod array_type;
mod binding;
mod buffer_binding;
mod encoder;
mod object_payload_binding;
mod pointer_type;
mod struct_member;
mod struct_type;
mod tensor_binding;
mod tensor_reference_type;
mod texture_binding;
mod texture_reference_type;
mod threadgroup_binding;
mod type_info;

pub use argument::Argument;
pub use array_type::ArrayType;
pub use binding::Binding;
pub use buffer_binding::BufferBinding;
pub use encoder::ArgumentEncoder;
pub use object_payload_binding::ObjectPayloadBinding;
pub use pointer_type::PointerType;
pub use struct_member::StructMember;
pub use struct_type::StructType;
pub use tensor_binding::TensorBinding;
pub use tensor_reference_type::TensorReferenceType;
pub use texture_binding::TextureBinding;
pub use texture_reference_type::TextureReferenceType;
pub use threadgroup_binding::ThreadgroupBinding;
pub use type_info::Type;

use mtl_foundation::UInteger;

/// The maximum stride value for static attributes.
///
/// C++ equivalent: `MTL::AttributeStrideStatic`
pub const ATTRIBUTE_STRIDE_STATIC: UInteger = UInteger::MAX;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;

    #[test]
    fn test_type_size() {
        assert_eq!(
            std::mem::size_of::<Type>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_argument_size() {
        assert_eq!(
            std::mem::size_of::<Argument>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_binding_size() {
        assert_eq!(
            std::mem::size_of::<Binding>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_struct_type_size() {
        assert_eq!(
            std::mem::size_of::<StructType>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_array_type_size() {
        assert_eq!(
            std::mem::size_of::<ArrayType>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_pointer_type_size() {
        assert_eq!(
            std::mem::size_of::<PointerType>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_argument_encoder_size() {
        assert_eq!(
            std::mem::size_of::<ArgumentEncoder>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_attribute_stride_static() {
        assert_eq!(ATTRIBUTE_STRIDE_STATIC, UInteger::MAX);
    }
}
