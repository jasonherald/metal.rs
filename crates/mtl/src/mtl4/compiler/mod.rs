//! MTL4 Compiler implementation.
//!
//! Corresponds to `Metal/MTL4Compiler.hpp`.

mod compiler;
mod descriptor;
mod task_options;

pub use compiler::Compiler;
pub use descriptor::CompilerDescriptor;
pub use task_options::CompilerTaskOptions;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;

    #[test]
    fn test_compiler_descriptor_size() {
        assert_eq!(
            std::mem::size_of::<CompilerDescriptor>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_compiler_task_options_size() {
        assert_eq!(
            std::mem::size_of::<CompilerTaskOptions>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_compiler_size() {
        assert_eq!(
            std::mem::size_of::<Compiler>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
