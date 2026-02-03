//! Library enumerations.
//!
//! Corresponds to `Metal/MTLLibrary.hpp`.

use metal_foundation::{Integer, UInteger};

/// Patch type for tessellation.
///
/// C++ equivalent: `MTL::PatchType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PatchType(pub UInteger);

impl PatchType {
    pub const NONE: Self = Self(0);
    pub const TRIANGLE: Self = Self(1);
    pub const QUAD: Self = Self(2);
}

/// Function type.
///
/// C++ equivalent: `MTL::FunctionType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FunctionType(pub UInteger);

impl FunctionType {
    pub const VERTEX: Self = Self(1);
    pub const FRAGMENT: Self = Self(2);
    pub const KERNEL: Self = Self(3);
    pub const VISIBLE: Self = Self(5);
    pub const INTERSECTION: Self = Self(6);
    pub const MESH: Self = Self(7);
    pub const OBJECT: Self = Self(8);
}

/// Metal shading language version.
///
/// C++ equivalent: `MTL::LanguageVersion`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LanguageVersion(pub UInteger);

impl LanguageVersion {
    pub const VERSION_1_0: Self = Self(65536);      // (1 << 16)
    pub const VERSION_1_1: Self = Self(65537);      // (1 << 16) + 1
    pub const VERSION_1_2: Self = Self(65538);      // (1 << 16) + 2
    pub const VERSION_2_0: Self = Self(131072);     // (2 << 16)
    pub const VERSION_2_1: Self = Self(131073);     // (2 << 16) + 1
    pub const VERSION_2_2: Self = Self(131074);     // (2 << 16) + 2
    pub const VERSION_2_3: Self = Self(131075);     // (2 << 16) + 3
    pub const VERSION_2_4: Self = Self(131076);     // (2 << 16) + 4
    pub const VERSION_3_0: Self = Self(196608);     // (3 << 16)
    pub const VERSION_3_1: Self = Self(196609);     // (3 << 16) + 1
    pub const VERSION_3_2: Self = Self(196610);     // (3 << 16) + 2
    pub const VERSION_4_0: Self = Self(262144);     // (4 << 16)
}

/// Library type.
///
/// C++ equivalent: `MTL::LibraryType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LibraryType(pub Integer);

impl LibraryType {
    pub const EXECUTABLE: Self = Self(0);
    pub const DYNAMIC: Self = Self(1);
}

/// Library optimization level.
///
/// C++ equivalent: `MTL::LibraryOptimizationLevel`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LibraryOptimizationLevel(pub Integer);

impl LibraryOptimizationLevel {
    pub const DEFAULT: Self = Self(0);
    pub const SIZE: Self = Self(1);
}

/// Compile symbol visibility.
///
/// C++ equivalent: `MTL::CompileSymbolVisibility`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CompileSymbolVisibility(pub Integer);

impl CompileSymbolVisibility {
    pub const DEFAULT: Self = Self(0);
    pub const HIDDEN: Self = Self(1);
}

/// Math mode for shader compilation.
///
/// C++ equivalent: `MTL::MathMode`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MathMode(pub Integer);

impl MathMode {
    pub const SAFE: Self = Self(0);
    pub const RELAXED: Self = Self(1);
    pub const FAST: Self = Self(2);
}

/// Math floating point functions precision.
///
/// C++ equivalent: `MTL::MathFloatingPointFunctions`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct MathFloatingPointFunctions(pub Integer);

impl MathFloatingPointFunctions {
    pub const FAST: Self = Self(0);
    pub const PRECISE: Self = Self(1);
}

/// Library error codes.
///
/// C++ equivalent: `MTL::LibraryError`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LibraryError(pub UInteger);

impl LibraryError {
    pub const UNSUPPORTED: Self = Self(1);
    pub const INTERNAL: Self = Self(2);
    pub const COMPILE_FAILURE: Self = Self(3);
    pub const COMPILE_WARNING: Self = Self(4);
    pub const FUNCTION_NOT_FOUND: Self = Self(5);
    pub const FILE_NOT_FOUND: Self = Self(6);
}

/// Dynamic library error codes.
///
/// C++ equivalent: `MTL::DynamicLibraryError`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DynamicLibraryError(pub UInteger);

impl DynamicLibraryError {
    pub const NONE: Self = Self(0);
    pub const INVALID_FILE: Self = Self(1);
    pub const COMPILATION_FAILURE: Self = Self(2);
    pub const UNRESOLVED_INSTALL_NAME: Self = Self(3);
    pub const DEPENDENCY_LOAD_FAILURE: Self = Self(4);
    pub const UNSUPPORTED: Self = Self(5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_type_values() {
        assert_eq!(FunctionType::VERTEX.0, 1);
        assert_eq!(FunctionType::FRAGMENT.0, 2);
        assert_eq!(FunctionType::KERNEL.0, 3);
        assert_eq!(FunctionType::MESH.0, 7);
    }

    #[test]
    fn test_language_version_values() {
        assert_eq!(LanguageVersion::VERSION_1_0.0, 65536);
        assert_eq!(LanguageVersion::VERSION_2_0.0, 131072);
        assert_eq!(LanguageVersion::VERSION_3_0.0, 196608);
        assert_eq!(LanguageVersion::VERSION_4_0.0, 262144);
    }

    #[test]
    fn test_library_error_values() {
        assert_eq!(LibraryError::UNSUPPORTED.0, 1);
        assert_eq!(LibraryError::COMPILE_FAILURE.0, 3);
    }

    #[test]
    fn test_patch_type_values() {
        assert_eq!(PatchType::NONE.0, 0);
        assert_eq!(PatchType::TRIANGLE.0, 1);
        assert_eq!(PatchType::QUAD.0, 2);
    }

    #[test]
    fn test_dynamic_library_error_values() {
        assert_eq!(DynamicLibraryError::NONE.0, 0);
        assert_eq!(DynamicLibraryError::INVALID_FILE.0, 1);
        assert_eq!(DynamicLibraryError::COMPILATION_FAILURE.0, 2);
        assert_eq!(DynamicLibraryError::UNRESOLVED_INSTALL_NAME.0, 3);
        assert_eq!(DynamicLibraryError::DEPENDENCY_LOAD_FAILURE.0, 4);
        assert_eq!(DynamicLibraryError::UNSUPPORTED.0, 5);
    }
}
