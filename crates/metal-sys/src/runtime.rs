//! Objective-C runtime bindings.
//!
//! Provides low-level access to the Objective-C runtime for selector
//! registration and class lookup.

use std::ffi::{CString, c_char, c_void};
use std::sync::OnceLock;

/// Objective-C selector.
///
/// A selector is a unique identifier for a method name. Selectors are
/// registered with the runtime and can be compared by pointer equality.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sel(pub(crate) *const c_void);

// SAFETY: Selectors are immutable identifiers managed by the ObjC runtime
unsafe impl Send for Sel {}
unsafe impl Sync for Sel {}

/// Objective-C class.
///
/// A class is a blueprint for creating objects. Classes are looked up
/// by name from the runtime.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Class(pub(crate) *const c_void);

// SAFETY: Class pointers are immutable after registration
unsafe impl Send for Class {}
unsafe impl Sync for Class {}

// Link against libobjc
#[link(name = "objc")]
unsafe extern "C" {
    fn sel_registerName(name: *const c_char) -> Sel;
    fn objc_lookUpClass(name: *const c_char) -> *const c_void;
    fn objc_getProtocol(name: *const c_char) -> *const c_void;
    fn class_getName(cls: *const c_void) -> *const c_char;
    fn class_getInstanceMethod(cls: *const c_void, sel: Sel) -> *const c_void;
    fn class_getClassMethod(cls: *const c_void, sel: Sel) -> *const c_void;
    fn protocol_getMethodDescription(
        proto: *const c_void,
        sel: Sel,
        is_required: bool,
        is_instance: bool,
    ) -> MethodDescription;
}

/// Objective-C method description (for protocol methods).
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MethodDescription {
    /// The selector for this method, or null if not found.
    pub sel: Sel,
    /// The type encoding string, or null if not found.
    pub types: *const c_char,
}

impl Sel {
    /// Register a selector with the given name.
    ///
    /// The selector is cached by the runtime, so subsequent calls with
    /// the same name will return the same selector.
    #[inline]
    pub fn register(name: &str) -> Self {
        let c_name = CString::new(name).expect("selector name contains null byte");
        unsafe { sel_registerName(c_name.as_ptr()) }
    }

    /// Register a selector from a null-terminated C string.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid null-terminated C string.
    #[inline]
    pub unsafe fn register_cstr(name: *const c_char) -> Self {
        unsafe { sel_registerName(name) }
    }

    /// Returns `true` if this selector is null.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Returns the raw pointer value.
    #[inline]
    pub fn as_ptr(&self) -> *const c_void {
        self.0
    }
}

impl Class {
    /// Look up a class by name.
    ///
    /// Returns `None` if the class is not registered with the runtime.
    #[inline]
    pub fn get(name: &str) -> Option<Self> {
        let c_name = CString::new(name).expect("class name contains null byte");
        let ptr = unsafe { objc_lookUpClass(c_name.as_ptr()) };
        if ptr.is_null() { None } else { Some(Self(ptr)) }
    }

    /// Look up a class from a null-terminated C string.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid null-terminated C string.
    #[inline]
    pub unsafe fn get_cstr(name: *const c_char) -> Option<Self> {
        let ptr = unsafe { objc_lookUpClass(name) };
        if ptr.is_null() { None } else { Some(Self(ptr)) }
    }

    /// Returns the raw pointer value.
    #[inline]
    pub fn as_ptr(&self) -> *const c_void {
        self.0
    }

    /// Returns `true` if this class pointer is null.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Check if instances of this class respond to the given selector.
    ///
    /// Returns `true` if the class has an instance method for the selector.
    #[inline]
    pub fn instances_respond_to(&self, sel: Sel) -> bool {
        let method = unsafe { class_getInstanceMethod(self.0, sel) };
        !method.is_null()
    }

    /// Check if this class responds to the given selector (class method).
    ///
    /// Returns `true` if the class has a class method for the selector.
    #[inline]
    pub fn responds_to(&self, sel: Sel) -> bool {
        let method = unsafe { class_getClassMethod(self.0, sel) };
        !method.is_null()
    }

    /// Get the name of this class.
    ///
    /// # Safety
    ///
    /// The class pointer must be valid.
    pub unsafe fn name(&self) -> &'static str {
        let name_ptr = unsafe { class_getName(self.0) };
        if name_ptr.is_null() {
            "<unknown>"
        } else {
            unsafe { std::ffi::CStr::from_ptr(name_ptr) }
                .to_str()
                .unwrap_or("<invalid utf8>")
        }
    }
}

/// Objective-C protocol.
///
/// A protocol defines a set of methods that a class can implement.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Protocol(pub(crate) *const c_void);

// SAFETY: Protocol pointers are immutable after registration
unsafe impl Send for Protocol {}
unsafe impl Sync for Protocol {}

impl Protocol {
    /// Look up a protocol by name.
    ///
    /// Returns `None` if the protocol is not registered with the runtime.
    #[inline]
    pub fn get(name: &str) -> Option<Self> {
        let c_name = CString::new(name).expect("protocol name contains null byte");
        let ptr = unsafe { objc_getProtocol(c_name.as_ptr()) };
        if ptr.is_null() { None } else { Some(Self(ptr)) }
    }

    /// Check if this protocol declares an instance method with the given selector.
    ///
    /// Checks both required and optional methods.
    #[inline]
    pub fn has_instance_method(&self, sel: Sel) -> bool {
        // Check required methods first
        let desc = unsafe { protocol_getMethodDescription(self.0, sel, true, true) };
        if !desc.sel.is_null() {
            return true;
        }
        // Check optional methods
        let desc = unsafe { protocol_getMethodDescription(self.0, sel, false, true) };
        !desc.sel.is_null()
    }

    /// Check if this protocol declares a class method with the given selector.
    ///
    /// Checks both required and optional methods.
    #[inline]
    pub fn has_class_method(&self, sel: Sel) -> bool {
        // Check required methods first
        let desc = unsafe { protocol_getMethodDescription(self.0, sel, true, false) };
        if !desc.sel.is_null() {
            return true;
        }
        // Check optional methods
        let desc = unsafe { protocol_getMethodDescription(self.0, sel, false, false) };
        !desc.sel.is_null()
    }

    /// Returns `true` if this protocol pointer is null.
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// Returns the raw pointer value.
    #[inline]
    pub fn as_ptr(&self) -> *const c_void {
        self.0
    }
}

/// Look up a protocol by name.
///
/// Returns the protocol pointer or null if not found.
#[inline]
pub fn get_protocol(name: &str) -> *const c_void {
    let c_name = CString::new(name).expect("protocol name contains null byte");
    unsafe { objc_getProtocol(c_name.as_ptr()) }
}

/// Helper type for caching selectors.
#[derive(Default)]
pub struct CachedSel {
    inner: OnceLock<Sel>,
}

impl CachedSel {
    /// Create a new uninitialized cached selector.
    pub const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }

    /// Get the selector, initializing it if needed.
    #[inline]
    pub fn get(&self, name: &str) -> Sel {
        *self.inner.get_or_init(|| Sel::register(name))
    }
}

/// Helper type for caching classes.
#[derive(Default)]
pub struct CachedClass {
    inner: OnceLock<Class>,
}

impl CachedClass {
    /// Create a new uninitialized cached class.
    pub const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }

    /// Get the class, initializing it if needed.
    ///
    /// # Panics
    ///
    /// Panics if the class is not found.
    #[inline]
    pub fn get(&self, name: &str) -> Class {
        *self
            .inner
            .get_or_init(|| Class::get(name).unwrap_or_else(|| panic!("class {} not found", name)))
    }

    /// Get the class if available, initializing it if needed.
    #[inline]
    pub fn try_get(&self, name: &str) -> Option<Class> {
        // First check if already initialized
        if let Some(cls) = self.inner.get() {
            return Some(*cls);
        }
        // Try to get and cache the class
        let cls = Class::get(name)?;
        Some(*self.inner.get_or_init(|| cls))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_registration() {
        let sel1 = Sel::register("init");
        let sel2 = Sel::register("init");
        assert_eq!(sel1, sel2);
        assert!(!sel1.is_null());
    }

    #[test]
    fn test_class_lookup() {
        let cls = Class::get("NSObject");
        assert!(cls.is_some());
        assert!(!cls.unwrap().is_null());
    }

    #[test]
    fn test_class_not_found() {
        let cls = Class::get("NonExistentClass12345");
        assert!(cls.is_none());
    }
}
