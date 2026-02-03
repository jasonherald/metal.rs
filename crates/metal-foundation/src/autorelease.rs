//! Autorelease pool for Foundation.
//!
//! Corresponds to `Foundation/NSAutoreleasePool.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! class AutoreleasePool : public Object {
//! public:
//!     static AutoreleasePool* alloc();
//!     AutoreleasePool*        init();
//!     void                    drain();
//!     void                    addObject(Object* pObject);
//!     static void             showPools();
//! };
//! }
//! ```

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_sys::{class, msg_send_0, msg_send_1, sel};

use crate::object::{Object, Referencing};

/// An Objective-C autorelease pool.
///
/// C++ equivalent: `NS::AutoreleasePool`
#[repr(transparent)]
#[derive(Clone)]
pub struct AutoreleasePool(NonNull<c_void>);

impl AutoreleasePool {
    /// Allocate a new autorelease pool.
    ///
    /// C++ equivalent: `static AutoreleasePool* alloc()`
    #[inline]
    pub fn alloc() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSAutoreleasePool).as_ptr(), sel!(alloc));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize an allocated autorelease pool.
    ///
    /// C++ equivalent: `AutoreleasePool* init()`
    #[inline]
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_ptr(ptr)
        }
    }

    /// Drain the autorelease pool.
    ///
    /// C++ equivalent: `void drain()`
    #[inline]
    pub fn drain(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(drain));
        }
    }

    /// Add an object to the autorelease pool.
    ///
    /// C++ equivalent: `void addObject(Object* pObject)`
    #[inline]
    pub fn add_object(&self, object: &Object) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(addObject:), object.as_ptr());
        }
    }

    /// Show all autorelease pools (for debugging).
    ///
    /// C++ equivalent: `static void showPools()`
    #[inline]
    pub fn show_pools() {
        unsafe {
            msg_send_0::<()>(class!(NSAutoreleasePool).as_ptr(), sel!(showPools));
        }
    }

    /// Create an AutoreleasePool from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSAutoreleasePool object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Create a new autorelease pool (convenience method).
    ///
    /// This is a convenience method that allocates and initializes a pool.
    #[inline]
    pub fn new() -> Option<Self> {
        Self::alloc()?.init()
    }
}

impl Referencing for AutoreleasePool {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

// SAFETY: NSAutoreleasePool is thread-safe
unsafe impl Send for AutoreleasePool {}
unsafe impl Sync for AutoreleasePool {}

impl std::fmt::Debug for AutoreleasePool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutoreleasePool")
            .field("ptr", &self.0)
            .finish()
    }
}

impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        self.drain();
    }
}

/// RAII wrapper for autorelease pool scopes.
///
/// This provides a more Rust-idiomatic way to use autorelease pools.
pub struct AutoreleasePoolScope {
    pool: AutoreleasePool,
}

impl AutoreleasePoolScope {
    /// Create a new autorelease pool scope.
    #[inline]
    pub fn new() -> Option<Self> {
        AutoreleasePool::new().map(|pool| Self { pool })
    }

    /// Get a reference to the underlying pool.
    #[inline]
    pub fn pool(&self) -> &AutoreleasePool {
        &self.pool
    }
}

impl Default for AutoreleasePoolScope {
    fn default() -> Self {
        Self::new().expect("Failed to create autorelease pool")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autorelease_pool_size() {
        // AutoreleasePool should be pointer-sized
        assert_eq!(
            std::mem::size_of::<AutoreleasePool>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
