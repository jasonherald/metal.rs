//! Metal shader libraries.
//!
//! Corresponds to `Metal/MTLLibrary.hpp` and `Metal/MTLFunctionDescriptor.hpp`.
//!
//! Libraries contain compiled shader functions.

mod attribute;
mod compile_options;
mod dynamic_library;
mod function;
mod function_constant;
mod function_constant_values;
mod function_descriptor;
mod function_reflection;
mod intersection_function_descriptor;
mod library;
mod linked_functions;
mod vertex_attribute;

pub use attribute::Attribute;
pub use compile_options::CompileOptions;
pub use dynamic_library::DynamicLibrary;
pub use function::Function;
pub use function_constant::FunctionConstant;
pub use function_constant_values::FunctionConstantValues;
pub use function_descriptor::FunctionDescriptor;
pub use function_reflection::FunctionReflection;
pub use intersection_function_descriptor::IntersectionFunctionDescriptor;
pub use library::Library;
pub use linked_functions::LinkedFunctions;
pub use vertex_attribute::VertexAttribute;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;

    #[test]
    fn test_library_size() {
        assert_eq!(
            std::mem::size_of::<Library>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_function_size() {
        assert_eq!(
            std::mem::size_of::<Function>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_compile_options_creation() {
        let opts = CompileOptions::new();
        assert!(opts.is_some());
    }

    #[test]
    fn test_function_constant_values_size() {
        assert_eq!(
            std::mem::size_of::<FunctionConstantValues>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_linked_functions_size() {
        assert_eq!(
            std::mem::size_of::<LinkedFunctions>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_dynamic_library_size() {
        assert_eq!(
            std::mem::size_of::<DynamicLibrary>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_vertex_attribute_size() {
        assert_eq!(
            std::mem::size_of::<VertexAttribute>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_attribute_size() {
        assert_eq!(
            std::mem::size_of::<Attribute>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_function_constant_size() {
        assert_eq!(
            std::mem::size_of::<FunctionConstant>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_function_reflection_size() {
        assert_eq!(
            std::mem::size_of::<FunctionReflection>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_function_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<FunctionDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_intersection_function_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<IntersectionFunctionDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
