//! Options for compiling shader source code.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

use crate::enums::{
    CompileSymbolVisibility, LanguageVersion, LibraryOptimizationLevel, LibraryType,
    MathFloatingPointFunctions, MathMode,
};
use crate::types::Size;

/// Options for compiling shader source code.
///
/// C++ equivalent: `MTL::CompileOptions`
#[repr(transparent)]
pub struct CompileOptions(pub(crate) NonNull<c_void>);

impl CompileOptions {
    /// Create new compile options.
    ///
    /// C++ equivalent: `static CompileOptions* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLCompileOptions")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a CompileOptions from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal compile options object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get whether fast math is enabled.
    ///
    /// C++ equivalent: `bool fastMathEnabled() const`
    #[inline]
    pub fn fast_math_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(fastMathEnabled)) }
    }

    /// Set whether fast math is enabled.
    ///
    /// C++ equivalent: `void setFastMathEnabled(bool)`
    #[inline]
    pub fn set_fast_math_enabled(&self, enabled: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setFastMathEnabled:), enabled);
        }
    }

    /// Get the language version.
    ///
    /// C++ equivalent: `LanguageVersion languageVersion() const`
    #[inline]
    pub fn language_version(&self) -> LanguageVersion {
        unsafe { msg_send_0(self.as_ptr(), sel!(languageVersion)) }
    }

    /// Set the language version.
    ///
    /// C++ equivalent: `void setLanguageVersion(LanguageVersion)`
    #[inline]
    pub fn set_language_version(&self, version: LanguageVersion) {
        unsafe {
            msg_send_1::<(), LanguageVersion>(self.as_ptr(), sel!(setLanguageVersion:), version);
        }
    }

    /// Get whether to preserve invariance.
    ///
    /// C++ equivalent: `bool preserveInvariance() const`
    #[inline]
    pub fn preserve_invariance(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(preserveInvariance)) }
    }

    /// Set whether to preserve invariance.
    ///
    /// C++ equivalent: `void setPreserveInvariance(bool)`
    #[inline]
    pub fn set_preserve_invariance(&self, preserve: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setPreserveInvariance:), preserve);
        }
    }

    /// Get the optimization level.
    ///
    /// C++ equivalent: `LibraryOptimizationLevel optimizationLevel() const`
    #[inline]
    pub fn optimization_level(&self) -> LibraryOptimizationLevel {
        unsafe { msg_send_0(self.as_ptr(), sel!(optimizationLevel)) }
    }

    /// Set the optimization level.
    ///
    /// C++ equivalent: `void setOptimizationLevel(MTL::LibraryOptimizationLevel)`
    #[inline]
    pub fn set_optimization_level(&self, level: LibraryOptimizationLevel) {
        unsafe {
            msg_send_1::<(), LibraryOptimizationLevel>(
                self.as_ptr(),
                sel!(setOptimizationLevel:),
                level,
            );
        }
    }

    /// Get the math mode.
    ///
    /// C++ equivalent: `MathMode mathMode() const`
    #[inline]
    pub fn math_mode(&self) -> MathMode {
        unsafe { msg_send_0(self.as_ptr(), sel!(mathMode)) }
    }

    /// Set the math mode.
    ///
    /// C++ equivalent: `void setMathMode(MTL::MathMode)`
    #[inline]
    pub fn set_math_mode(&self, mode: MathMode) {
        unsafe {
            msg_send_1::<(), MathMode>(self.as_ptr(), sel!(setMathMode:), mode);
        }
    }

    /// Get the math floating point functions precision.
    ///
    /// C++ equivalent: `MathFloatingPointFunctions mathFloatingPointFunctions() const`
    #[inline]
    pub fn math_floating_point_functions(&self) -> MathFloatingPointFunctions {
        unsafe { msg_send_0(self.as_ptr(), sel!(mathFloatingPointFunctions)) }
    }

    /// Set the math floating point functions precision.
    ///
    /// C++ equivalent: `void setMathFloatingPointFunctions(MTL::MathFloatingPointFunctions)`
    #[inline]
    pub fn set_math_floating_point_functions(&self, funcs: MathFloatingPointFunctions) {
        unsafe {
            msg_send_1::<(), MathFloatingPointFunctions>(
                self.as_ptr(),
                sel!(setMathFloatingPointFunctions:),
                funcs,
            );
        }
    }

    /// Get the compile symbol visibility.
    ///
    /// C++ equivalent: `CompileSymbolVisibility compileSymbolVisibility() const`
    #[inline]
    pub fn compile_symbol_visibility(&self) -> CompileSymbolVisibility {
        unsafe { msg_send_0(self.as_ptr(), sel!(compileSymbolVisibility)) }
    }

    /// Set the compile symbol visibility.
    ///
    /// C++ equivalent: `void setCompileSymbolVisibility(MTL::CompileSymbolVisibility)`
    #[inline]
    pub fn set_compile_symbol_visibility(&self, visibility: CompileSymbolVisibility) {
        unsafe {
            msg_send_1::<(), CompileSymbolVisibility>(
                self.as_ptr(),
                sel!(setCompileSymbolVisibility:),
                visibility,
            );
        }
    }

    /// Get the library type.
    ///
    /// C++ equivalent: `LibraryType libraryType() const`
    #[inline]
    pub fn library_type(&self) -> LibraryType {
        unsafe { msg_send_0(self.as_ptr(), sel!(libraryType)) }
    }

    /// Set the library type.
    ///
    /// C++ equivalent: `void setLibraryType(MTL::LibraryType)`
    #[inline]
    pub fn set_library_type(&self, lib_type: LibraryType) {
        unsafe {
            msg_send_1::<(), LibraryType>(self.as_ptr(), sel!(setLibraryType:), lib_type);
        }
    }

    /// Get the install name (for dynamic libraries).
    ///
    /// C++ equivalent: `NS::String* installName() const`
    pub fn install_name(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(installName));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Set the install name (for dynamic libraries).
    ///
    /// C++ equivalent: `void setInstallName(const NS::String*)`
    pub fn set_install_name(&self, name: &str) {
        if let Some(ns_name) = mtl_foundation::String::from_str(name) {
            unsafe {
                msg_send_1::<(), *const c_void>(
                    self.as_ptr(),
                    sel!(setInstallName:),
                    ns_name.as_ptr(),
                );
            }
        }
    }

    /// Get whether to allow referencing undefined symbols.
    ///
    /// C++ equivalent: `bool allowReferencingUndefinedSymbols() const`
    #[inline]
    pub fn allow_referencing_undefined_symbols(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(allowReferencingUndefinedSymbols)) }
    }

    /// Set whether to allow referencing undefined symbols.
    ///
    /// C++ equivalent: `void setAllowReferencingUndefinedSymbols(bool)`
    #[inline]
    pub fn set_allow_referencing_undefined_symbols(&self, allow: bool) {
        unsafe {
            msg_send_1::<(), bool>(
                self.as_ptr(),
                sel!(setAllowReferencingUndefinedSymbols:),
                allow,
            );
        }
    }

    /// Get whether logging is enabled.
    ///
    /// C++ equivalent: `bool enableLogging() const`
    #[inline]
    pub fn enable_logging(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(enableLogging)) }
    }

    /// Set whether logging is enabled.
    ///
    /// C++ equivalent: `void setEnableLogging(bool)`
    #[inline]
    pub fn set_enable_logging(&self, enable: bool) {
        unsafe {
            msg_send_1::<(), bool>(self.as_ptr(), sel!(setEnableLogging:), enable);
        }
    }

    /// Get the maximum total threads per threadgroup.
    ///
    /// C++ equivalent: `NS::UInteger maxTotalThreadsPerThreadgroup() const`
    #[inline]
    pub fn max_total_threads_per_threadgroup(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(maxTotalThreadsPerThreadgroup)) }
    }

    /// Set the maximum total threads per threadgroup.
    ///
    /// C++ equivalent: `void setMaxTotalThreadsPerThreadgroup(NS::UInteger)`
    #[inline]
    pub fn set_max_total_threads_per_threadgroup(&self, count: UInteger) {
        unsafe {
            msg_send_1::<(), UInteger>(
                self.as_ptr(),
                sel!(setMaxTotalThreadsPerThreadgroup:),
                count,
            );
        }
    }

    /// Get the required threads per threadgroup.
    ///
    /// C++ equivalent: `Size requiredThreadsPerThreadgroup() const`
    #[inline]
    pub fn required_threads_per_threadgroup(&self) -> Size {
        unsafe { msg_send_0(self.as_ptr(), sel!(requiredThreadsPerThreadgroup)) }
    }

    /// Set the required threads per threadgroup.
    ///
    /// C++ equivalent: `void setRequiredThreadsPerThreadgroup(MTL::Size)`
    #[inline]
    pub fn set_required_threads_per_threadgroup(&self, size: Size) {
        unsafe {
            msg_send_1::<(), Size>(self.as_ptr(), sel!(setRequiredThreadsPerThreadgroup:), size);
        }
    }

    /// Get the preprocessor macros dictionary.
    ///
    /// Returns the dictionary of preprocessor macros as a raw pointer.
    /// Use `mtl_foundation::Dictionary` to work with the returned value.
    ///
    /// C++ equivalent: `NS::Dictionary* preprocessorMacros() const`
    pub fn preprocessor_macros_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(preprocessorMacros)) }
    }

    /// Set the preprocessor macros dictionary.
    ///
    /// Takes a raw pointer to an NSDictionary containing the preprocessor macros.
    ///
    /// C++ equivalent: `void setPreprocessorMacros(const NS::Dictionary*)`
    ///
    /// # Safety
    ///
    /// The dictionary pointer must be valid or null.
    pub unsafe fn set_preprocessor_macros_raw(&self, dict: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setPreprocessorMacros:), dict);
        }
    }
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self::new().expect("failed to create compile options")
    }
}

impl Clone for CompileOptions {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy compile options")
        }
    }
}

impl Drop for CompileOptions {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for CompileOptions {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CompileOptions {}
unsafe impl Sync for CompileOptions {}
