//! MTL4 Compiler task options.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, msg_send_1, sel};

/// Options for compiler tasks.
///
/// C++ equivalent: `MTL4::CompilerTaskOptions`
#[repr(transparent)]
pub struct CompilerTaskOptions(NonNull<c_void>);

impl CompilerTaskOptions {
    /// Create a CompilerTaskOptions from a raw pointer.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Create new compiler task options.
    pub fn new() -> Option<Self> {
        unsafe {
            let class = metal_sys::Class::get("MTL4CompilerTaskOptions")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Get the lookup archives (as raw pointer to NSArray).
    ///
    /// C++ equivalent: `NS::Array* lookupArchives() const`
    pub fn lookup_archives_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(lookupArchives)) }
    }

    /// Set the lookup archives (from raw pointer to NSArray).
    ///
    /// C++ equivalent: `void setLookupArchives(const NS::Array*)`
    pub fn set_lookup_archives_raw(&self, archives: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setLookupArchives:), archives);
        }
    }
}

impl Clone for CompilerTaskOptions {
    fn clone(&self) -> Self {
        unsafe {
            metal_sys::msg_send_0::<*mut c_void>(self.as_ptr(), metal_sys::sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for CompilerTaskOptions {
    fn drop(&mut self) {
        unsafe {
            metal_sys::msg_send_0::<()>(self.as_ptr(), metal_sys::sel!(release));
        }
    }
}

impl Referencing for CompilerTaskOptions {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for CompilerTaskOptions {}
unsafe impl Sync for CompilerTaskOptions {}

impl std::fmt::Debug for CompilerTaskOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompilerTaskOptions").finish()
    }
}
