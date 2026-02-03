//! Bundle type for Foundation.
//!
//! Corresponds to `Foundation/NSBundle.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! class Bundle : public Referencing<Bundle> {
//! public:
//!     static Bundle*      mainBundle();
//!     static Bundle*      bundle(const class String* pPath);
//!     static Bundle*      bundle(const class URL* pURL);
//!     static class Array* allBundles();
//!     static class Array* allFrameworks();
//!     static Bundle*      alloc();
//!     Bundle*             init(const class String* pPath);
//!     Bundle*             init(const class URL* pURL);
//!     bool                load();
//!     bool                unload();
//!     bool                isLoaded() const;
//!     bool                preflightAndReturnError(class Error** pError) const;
//!     bool                loadAndReturnError(class Error** pError);
//!     // ... many path/URL methods
//!     class String*       bundleIdentifier() const;
//!     class Dictionary*   infoDictionary() const;
//!     class String*       localizedString(...) const;
//! };
//! }
//! ```

use std::ffi::c_void;
use std::ptr::NonNull;

use metal_sys::{class, msg_send_0, msg_send_1, msg_send_3, sel};

use crate::array::Array;
use crate::dictionary::Dictionary;
use crate::error::Error;
use crate::object::{Object, Referencing};
use crate::string::String;
use crate::url::Url;

/// An Objective-C bundle object.
///
/// C++ equivalent: `NS::Bundle`
#[repr(transparent)]
#[derive(Clone)]
pub struct Bundle(NonNull<c_void>);

impl Bundle {
    /// Get the main bundle.
    ///
    /// C++ equivalent: `static Bundle* mainBundle()`
    #[inline]
    pub fn main_bundle() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSBundle).as_ptr(), sel!(mainBundle));
            Self::from_ptr(ptr)
        }
    }

    /// Get a bundle with the specified path.
    ///
    /// C++ equivalent: `static Bundle* bundle(const class String* pPath)`
    #[inline]
    pub fn bundle_with_path(path: &String) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSBundle).as_ptr(), sel!(bundleWithPath:), path.as_ptr());
            Self::from_ptr(ptr)
        }
    }

    /// Get a bundle with the specified URL.
    ///
    /// C++ equivalent: `static Bundle* bundle(const class URL* pURL)`
    #[inline]
    pub fn bundle_with_url(url: &Url) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSBundle).as_ptr(), sel!(bundleWithURL:), url.as_ptr());
            Self::from_ptr(ptr)
        }
    }

    /// Get all bundles.
    ///
    /// C++ equivalent: `static class Array* allBundles()`
    #[inline]
    pub fn all_bundles() -> *mut Array<Bundle> {
        unsafe { msg_send_0(class!(NSBundle).as_ptr(), sel!(allBundles)) }
    }

    /// Get all frameworks.
    ///
    /// C++ equivalent: `static class Array* allFrameworks()`
    #[inline]
    pub fn all_frameworks() -> *mut Array<Bundle> {
        unsafe { msg_send_0(class!(NSBundle).as_ptr(), sel!(allFrameworks)) }
    }

    /// Allocate a new bundle.
    ///
    /// C++ equivalent: `static Bundle* alloc()`
    #[inline]
    pub fn alloc() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSBundle).as_ptr(), sel!(alloc));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a path.
    ///
    /// C++ equivalent: `Bundle* init(const class String* pPath)`
    #[inline]
    pub fn init_with_path(&self, path: &String) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithPath:), path.as_ptr());
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a URL.
    ///
    /// C++ equivalent: `Bundle* init(const class URL* pURL)`
    #[inline]
    pub fn init_with_url(&self, url: &Url) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithURL:), url.as_ptr());
            Self::from_ptr(ptr)
        }
    }

    /// Load the bundle.
    ///
    /// C++ equivalent: `bool load()`
    #[inline]
    pub fn load(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(load)) }
    }

    /// Unload the bundle.
    ///
    /// C++ equivalent: `bool unload()`
    #[inline]
    pub fn unload(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(unload)) }
    }

    /// Check if the bundle is loaded.
    ///
    /// C++ equivalent: `bool isLoaded() const`
    #[inline]
    pub fn is_loaded(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isLoaded)) }
    }

    /// Preflight the bundle and return any error.
    ///
    /// C++ equivalent: `bool preflightAndReturnError(class Error** pError) const`
    #[inline]
    pub fn preflight_and_return_error(&self, error: *mut *mut Error) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(preflightAndReturnError:), error) }
    }

    /// Load the bundle and return any error.
    ///
    /// C++ equivalent: `bool loadAndReturnError(class Error** pError)`
    #[inline]
    pub fn load_and_return_error(&self, error: *mut *mut Error) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(loadAndReturnError:), error) }
    }

    /// Get the bundle URL.
    ///
    /// C++ equivalent: `class URL* bundleURL() const`
    #[inline]
    pub fn bundle_url(&self) -> *mut Url {
        unsafe { msg_send_0(self.as_ptr(), sel!(bundleURL)) }
    }

    /// Get the resource URL.
    ///
    /// C++ equivalent: `class URL* resourceURL() const`
    #[inline]
    pub fn resource_url(&self) -> *mut Url {
        unsafe { msg_send_0(self.as_ptr(), sel!(resourceURL)) }
    }

    /// Get the executable URL.
    ///
    /// C++ equivalent: `class URL* executableURL() const`
    #[inline]
    pub fn executable_url(&self) -> *mut Url {
        unsafe { msg_send_0(self.as_ptr(), sel!(executableURL)) }
    }

    /// Get the URL for an auxiliary executable.
    ///
    /// C++ equivalent: `class URL* URLForAuxiliaryExecutable(const class String* pExecutableName) const`
    #[inline]
    pub fn url_for_auxiliary_executable(&self, name: &String) -> *mut Url {
        unsafe { msg_send_1(self.as_ptr(), sel!(URLForAuxiliaryExecutable:), name.as_ptr()) }
    }

    /// Get the private frameworks URL.
    #[inline]
    pub fn private_frameworks_url(&self) -> *mut Url {
        unsafe { msg_send_0(self.as_ptr(), sel!(privateFrameworksURL)) }
    }

    /// Get the shared frameworks URL.
    #[inline]
    pub fn shared_frameworks_url(&self) -> *mut Url {
        unsafe { msg_send_0(self.as_ptr(), sel!(sharedFrameworksURL)) }
    }

    /// Get the shared support URL.
    #[inline]
    pub fn shared_support_url(&self) -> *mut Url {
        unsafe { msg_send_0(self.as_ptr(), sel!(sharedSupportURL)) }
    }

    /// Get the built-in plug-ins URL.
    #[inline]
    pub fn built_in_plug_ins_url(&self) -> *mut Url {
        unsafe { msg_send_0(self.as_ptr(), sel!(builtInPlugInsURL)) }
    }

    /// Get the App Store receipt URL.
    #[inline]
    pub fn app_store_receipt_url(&self) -> *mut Url {
        unsafe { msg_send_0(self.as_ptr(), sel!(appStoreReceiptURL)) }
    }

    /// Get the bundle path.
    ///
    /// C++ equivalent: `class String* bundlePath() const`
    #[inline]
    pub fn bundle_path(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(bundlePath)) }
    }

    /// Get the resource path.
    #[inline]
    pub fn resource_path(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(resourcePath)) }
    }

    /// Get the executable path.
    #[inline]
    pub fn executable_path(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(executablePath)) }
    }

    /// Get the path for an auxiliary executable.
    #[inline]
    pub fn path_for_auxiliary_executable(&self, name: &String) -> *mut String {
        unsafe { msg_send_1(self.as_ptr(), sel!(pathForAuxiliaryExecutable:), name.as_ptr()) }
    }

    /// Get the private frameworks path.
    #[inline]
    pub fn private_frameworks_path(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(privateFrameworksPath)) }
    }

    /// Get the shared frameworks path.
    #[inline]
    pub fn shared_frameworks_path(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(sharedFrameworksPath)) }
    }

    /// Get the shared support path.
    #[inline]
    pub fn shared_support_path(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(sharedSupportPath)) }
    }

    /// Get the built-in plug-ins path.
    #[inline]
    pub fn built_in_plug_ins_path(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(builtInPlugInsPath)) }
    }

    /// Get the bundle identifier.
    ///
    /// C++ equivalent: `class String* bundleIdentifier() const`
    #[inline]
    pub fn bundle_identifier(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(bundleIdentifier)) }
    }

    /// Get the info dictionary.
    ///
    /// C++ equivalent: `class Dictionary* infoDictionary() const`
    #[inline]
    pub fn info_dictionary(&self) -> *mut Dictionary {
        unsafe { msg_send_0(self.as_ptr(), sel!(infoDictionary)) }
    }

    /// Get the localized info dictionary.
    ///
    /// C++ equivalent: `class Dictionary* localizedInfoDictionary() const`
    #[inline]
    pub fn localized_info_dictionary(&self) -> *mut Dictionary {
        unsafe { msg_send_0(self.as_ptr(), sel!(localizedInfoDictionary)) }
    }

    /// Get an object from the info dictionary.
    ///
    /// C++ equivalent: `class Object* objectForInfoDictionaryKey(const class String* pKey)`
    #[inline]
    pub fn object_for_info_dictionary_key(&self, key: &String) -> *mut Object {
        unsafe { msg_send_1(self.as_ptr(), sel!(objectForInfoDictionaryKey:), key.as_ptr()) }
    }

    /// Get a localized string.
    ///
    /// C++ equivalent: `class String* localizedString(const class String* pKey, const class String* pValue = nullptr, const class String* pTableName = nullptr) const`
    #[inline]
    pub fn localized_string(
        &self,
        key: &String,
        value: *const String,
        table_name: *const String,
    ) -> *mut String {
        unsafe {
            msg_send_3(
                self.as_ptr(),
                sel!(localizedStringForKey:value:table:),
                key.as_ptr(),
                value,
                table_name,
            )
        }
    }

    /// Create a Bundle from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSBundle object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
}

impl Referencing for Bundle {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for Bundle {}
unsafe impl Sync for Bundle {}

impl std::fmt::Debug for Bundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bundle").field("ptr", &self.0).finish()
    }
}

// Helper functions matching C++ free functions

/// Get a localized string from the main bundle.
///
/// C++ equivalent: `NS::LocalizedString(const String* pKey, const String*)`
#[inline]
pub fn localized_string(key: &String) -> *mut String {
    if let Some(bundle) = Bundle::main_bundle() {
        bundle.localized_string(key, std::ptr::null(), std::ptr::null())
    } else {
        std::ptr::null_mut()
    }
}

/// Get a localized string from a table in the main bundle.
///
/// C++ equivalent: `NS::LocalizedStringFromTable(...)`
#[inline]
pub fn localized_string_from_table(key: &String, table: &String) -> *mut String {
    if let Some(bundle) = Bundle::main_bundle() {
        bundle.localized_string(key, std::ptr::null(), table)
    } else {
        std::ptr::null_mut()
    }
}

/// Get a localized string from a table in a bundle.
///
/// C++ equivalent: `NS::LocalizedStringFromTableInBundle(...)`
#[inline]
pub fn localized_string_from_table_in_bundle(
    key: &String,
    table: &String,
    bundle: &Bundle,
) -> *mut String {
    bundle.localized_string(key, std::ptr::null(), table)
}

/// Get a localized string with a default value.
///
/// C++ equivalent: `NS::LocalizedStringWithDefaultValue(...)`
#[inline]
pub fn localized_string_with_default_value(
    key: &String,
    table: &String,
    bundle: &Bundle,
    value: &String,
) -> *mut String {
    bundle.localized_string(key, value, table)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundle_size() {
        assert_eq!(
            std::mem::size_of::<Bundle>(),
            std::mem::size_of::<*mut c_void>()
        );
    }
}
