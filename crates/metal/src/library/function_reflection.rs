//! Function reflection information.

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_foundation::Referencing;
use metal_sys::{msg_send_0, sel};

/// Function reflection information.
///
/// C++ equivalent: `MTL::FunctionReflection`
///
/// Contains reflection information about a function.
#[repr(transparent)]
pub struct FunctionReflection(pub(crate) NonNull<c_void>);

impl FunctionReflection {
    /// Allocate a new function reflection.
    ///
    /// C++ equivalent: `static FunctionReflection* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = metal_sys::Class::get("MTLFunctionReflection")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated function reflection.
    ///
    /// C++ equivalent: `FunctionReflection* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new function reflection.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal function reflection object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the function bindings (raw NSArray pointer).
    ///
    /// C++ equivalent: `NS::Array* bindings() const`
    pub fn bindings_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(bindings)) }
    }
}

impl Clone for FunctionReflection {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for FunctionReflection {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionReflection {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionReflection {}
unsafe impl Sync for FunctionReflection {}

impl std::fmt::Debug for FunctionReflection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FunctionReflection").finish()
    }
}
