//! A collection of functions to link together.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::Referencing;
use mtl_sys::{msg_send_0, msg_send_1, sel};

/// A collection of functions to link together.
///
/// C++ equivalent: `MTL::LinkedFunctions`
///
/// Linked functions allow you to specify a set of functions that should be
/// available for function pointers and callable from other functions.
#[repr(transparent)]
pub struct LinkedFunctions(pub(crate) NonNull<c_void>);

impl LinkedFunctions {
    /// Allocate a new linked functions object.
    ///
    /// C++ equivalent: `static LinkedFunctions* alloc()`
    pub fn alloc() -> Option<Self> {
        unsafe {
            let cls = mtl_sys::Class::get("MTLLinkedFunctions")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(alloc));
            Self::from_raw(ptr)
        }
    }

    /// Initialize an allocated linked functions object.
    ///
    /// C++ equivalent: `LinkedFunctions* init()`
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new linked functions object.
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }

    /// Create linked functions using the factory method.
    ///
    /// C++ equivalent: `static LinkedFunctions* linkedFunctions()`
    pub fn linked_functions() -> Option<Self> {
        unsafe {
            let cls = mtl_sys::Class::get("MTLLinkedFunctions")?;
            let ptr: *mut c_void = msg_send_0(cls.as_ptr(), sel!(linkedFunctions));
            if ptr.is_null() {
                return None;
            }
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal linked functions object.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the functions array (raw NSArray pointer).
    ///
    /// C++ equivalent: `NS::Array* functions() const`
    pub fn functions_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(functions)) }
    }

    /// Set the functions array.
    ///
    /// C++ equivalent: `void setFunctions(const NS::Array*)`
    pub fn set_functions_raw(&self, functions: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setFunctions:), functions);
        }
    }

    /// Get the binary functions array (raw NSArray pointer).
    ///
    /// C++ equivalent: `NS::Array* binaryFunctions() const`
    pub fn binary_functions_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(binaryFunctions)) }
    }

    /// Set the binary functions array.
    ///
    /// C++ equivalent: `void setBinaryFunctions(const NS::Array*)`
    pub fn set_binary_functions_raw(&self, functions: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setBinaryFunctions:), functions);
        }
    }

    /// Get the private functions array (raw NSArray pointer).
    ///
    /// C++ equivalent: `NS::Array* privateFunctions() const`
    pub fn private_functions_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(privateFunctions)) }
    }

    /// Set the private functions array.
    ///
    /// C++ equivalent: `void setPrivateFunctions(const NS::Array*)`
    pub fn set_private_functions_raw(&self, functions: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setPrivateFunctions:), functions);
        }
    }

    /// Get the function groups dictionary (raw NSDictionary pointer).
    ///
    /// C++ equivalent: `NS::Dictionary* groups() const`
    pub fn groups_raw(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(groups)) }
    }

    /// Set the function groups dictionary.
    ///
    /// C++ equivalent: `void setGroups(const NS::Dictionary*)`
    pub fn set_groups_raw(&self, groups: *const c_void) {
        unsafe {
            msg_send_1::<(), *const c_void>(self.as_ptr(), sel!(setGroups:), groups);
        }
    }
}

impl Default for LinkedFunctions {
    fn default() -> Self {
        Self::new().expect("failed to create LinkedFunctions")
    }
}

impl Clone for LinkedFunctions {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("failed to copy LinkedFunctions")
        }
    }
}

impl Drop for LinkedFunctions {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for LinkedFunctions {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for LinkedFunctions {}
unsafe impl Sync for LinkedFunctions {}

impl std::fmt::Debug for LinkedFunctions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkedFunctions").finish()
    }
}
