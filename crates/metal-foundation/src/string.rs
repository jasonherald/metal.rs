//! String type for Foundation.
//!
//! Corresponds to `Foundation/NSString.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! _NS_ENUM(NS::UInteger, StringEncoding) {
//!     ASCIIStringEncoding = 1,
//!     NEXTSTEPStringEncoding = 2,
//!     JapaneseEUCStringEncoding = 3,
//!     UTF8StringEncoding = 4,
//!     ISOLatin1StringEncoding = 5,
//!     SymbolStringEncoding = 6,
//!     NonLossyASCIIStringEncoding = 7,
//!     ShiftJISStringEncoding = 8,
//!     ISOLatin2StringEncoding = 9,
//!     UnicodeStringEncoding = 10,
//!     WindowsCP1251StringEncoding = 11,
//!     WindowsCP1252StringEncoding = 12,
//!     WindowsCP1253StringEncoding = 13,
//!     WindowsCP1254StringEncoding = 14,
//!     WindowsCP1250StringEncoding = 15,
//!     ISO2022JPStringEncoding = 21,
//!     MacOSRomanStringEncoding = 30,
//!     UTF16StringEncoding = UnicodeStringEncoding,
//!     UTF16BigEndianStringEncoding = 0x90000100,
//!     UTF16LittleEndianStringEncoding = 0x94000100,
//!     UTF32StringEncoding = 0x8c000100,
//!     UTF32BigEndianStringEncoding = 0x98000100,
//!     UTF32LittleEndianStringEncoding = 0x9c000100
//! };
//!
//! _NS_OPTIONS(NS::UInteger, StringCompareOptions) {
//!     CaseInsensitiveSearch = 1,
//!     LiteralSearch = 2,
//!     BackwardsSearch = 4,
//!     AnchoredSearch = 8,
//!     NumericSearch = 64,
//!     DiacriticInsensitiveSearch = 128,
//!     WidthInsensitiveSearch = 256,
//!     ForcedOrderingSearch = 512,
//!     RegularExpressionSearch = 1024
//! };
//!
//! using unichar = unsigned short;
//!
//! class String : public Copying<String> {
//! public:
//!     static String*   string();
//!     static String*   string(const String* pString);
//!     static String*   string(const char* pString, StringEncoding encoding);
//!     static String*   alloc();
//!     String*          init();
//!     String*          init(const String* pString);
//!     String*          init(const char* pString, StringEncoding encoding);
//!     String*          init(void* pBytes, UInteger len, StringEncoding encoding, bool freeBuffer);
//!     unichar          character(UInteger index) const;
//!     UInteger         length() const;
//!     const char*      cString(StringEncoding encoding) const;
//!     const char*      utf8String() const;
//!     UInteger         maximumLengthOfBytes(StringEncoding encoding) const;
//!     UInteger         lengthOfBytes(StringEncoding encoding) const;
//!     bool             isEqualToString(const String* pString) const;
//!     Range            rangeOfString(const String* pString, StringCompareOptions options) const;
//!     const char*      fileSystemRepresentation() const;
//!     String*          stringByAppendingString(const String* pString) const;
//!     ComparisonResult caseInsensitiveCompare(const String* pString) const;
//! };
//! }
//! ```

use std::ffi::{c_char, c_void};
use std::ptr::NonNull;

use metal_sys::{class, msg_send_0, msg_send_1, msg_send_2, msg_send_4, sel};

use crate::objc_runtime::ComparisonResult;
use crate::object::{Copying, Referencing};
use crate::range::Range;
use crate::types::UInteger;

/// Unicode character type.
///
/// C++ equivalent: `NS::unichar`
pub type Unichar = u16;

/// String encoding options.
///
/// C++ equivalent: `NS::StringEncoding`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct StringEncoding(pub UInteger);

impl StringEncoding {
    /// ASCII encoding.
    ///
    /// C++ equivalent: `NS::ASCIIStringEncoding`
    pub const ASCII: Self = Self(1);

    /// NEXTSTEP encoding.
    ///
    /// C++ equivalent: `NS::NEXTSTEPStringEncoding`
    pub const NEXTSTEP: Self = Self(2);

    /// Japanese EUC encoding.
    ///
    /// C++ equivalent: `NS::JapaneseEUCStringEncoding`
    pub const JAPANESE_EUC: Self = Self(3);

    /// UTF-8 encoding.
    ///
    /// C++ equivalent: `NS::UTF8StringEncoding`
    pub const UTF8: Self = Self(4);

    /// ISO Latin 1 encoding.
    ///
    /// C++ equivalent: `NS::ISOLatin1StringEncoding`
    pub const ISO_LATIN1: Self = Self(5);

    /// Symbol encoding.
    ///
    /// C++ equivalent: `NS::SymbolStringEncoding`
    pub const SYMBOL: Self = Self(6);

    /// Non-lossy ASCII encoding.
    ///
    /// C++ equivalent: `NS::NonLossyASCIIStringEncoding`
    pub const NON_LOSSY_ASCII: Self = Self(7);

    /// Shift JIS encoding.
    ///
    /// C++ equivalent: `NS::ShiftJISStringEncoding`
    pub const SHIFT_JIS: Self = Self(8);

    /// ISO Latin 2 encoding.
    ///
    /// C++ equivalent: `NS::ISOLatin2StringEncoding`
    pub const ISO_LATIN2: Self = Self(9);

    /// Unicode encoding.
    ///
    /// C++ equivalent: `NS::UnicodeStringEncoding`
    pub const UNICODE: Self = Self(10);

    /// Windows CP1251 encoding.
    ///
    /// C++ equivalent: `NS::WindowsCP1251StringEncoding`
    pub const WINDOWS_CP1251: Self = Self(11);

    /// Windows CP1252 encoding.
    ///
    /// C++ equivalent: `NS::WindowsCP1252StringEncoding`
    pub const WINDOWS_CP1252: Self = Self(12);

    /// Windows CP1253 encoding.
    ///
    /// C++ equivalent: `NS::WindowsCP1253StringEncoding`
    pub const WINDOWS_CP1253: Self = Self(13);

    /// Windows CP1254 encoding.
    ///
    /// C++ equivalent: `NS::WindowsCP1254StringEncoding`
    pub const WINDOWS_CP1254: Self = Self(14);

    /// Windows CP1250 encoding.
    ///
    /// C++ equivalent: `NS::WindowsCP1250StringEncoding`
    pub const WINDOWS_CP1250: Self = Self(15);

    /// ISO 2022 JP encoding.
    ///
    /// C++ equivalent: `NS::ISO2022JPStringEncoding`
    pub const ISO2022JP: Self = Self(21);

    /// Mac OS Roman encoding.
    ///
    /// C++ equivalent: `NS::MacOSRomanStringEncoding`
    pub const MAC_OS_ROMAN: Self = Self(30);

    /// UTF-16 encoding (alias for Unicode).
    ///
    /// C++ equivalent: `NS::UTF16StringEncoding`
    pub const UTF16: Self = Self(10);

    /// UTF-16 big endian encoding.
    ///
    /// C++ equivalent: `NS::UTF16BigEndianStringEncoding`
    pub const UTF16_BIG_ENDIAN: Self = Self(0x90000100);

    /// UTF-16 little endian encoding.
    ///
    /// C++ equivalent: `NS::UTF16LittleEndianStringEncoding`
    pub const UTF16_LITTLE_ENDIAN: Self = Self(0x94000100);

    /// UTF-32 encoding.
    ///
    /// C++ equivalent: `NS::UTF32StringEncoding`
    pub const UTF32: Self = Self(0x8c000100);

    /// UTF-32 big endian encoding.
    ///
    /// C++ equivalent: `NS::UTF32BigEndianStringEncoding`
    pub const UTF32_BIG_ENDIAN: Self = Self(0x98000100);

    /// UTF-32 little endian encoding.
    ///
    /// C++ equivalent: `NS::UTF32LittleEndianStringEncoding`
    pub const UTF32_LITTLE_ENDIAN: Self = Self(0x9c000100);

    /// Returns the raw value.
    #[inline]
    pub const fn raw(&self) -> UInteger {
        self.0
    }

    /// Creates from a raw value.
    #[inline]
    pub const fn from_raw(value: UInteger) -> Self {
        Self(value)
    }
}

/// String comparison options.
///
/// C++ equivalent: `NS::StringCompareOptions`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct StringCompareOptions(pub UInteger);

impl StringCompareOptions {
    /// No options.
    pub const NONE: Self = Self(0);

    /// Case insensitive search.
    ///
    /// C++ equivalent: `NS::CaseInsensitiveSearch`
    pub const CASE_INSENSITIVE: Self = Self(1);

    /// Literal search.
    ///
    /// C++ equivalent: `NS::LiteralSearch`
    pub const LITERAL: Self = Self(2);

    /// Backwards search.
    ///
    /// C++ equivalent: `NS::BackwardsSearch`
    pub const BACKWARDS: Self = Self(4);

    /// Anchored search.
    ///
    /// C++ equivalent: `NS::AnchoredSearch`
    pub const ANCHORED: Self = Self(8);

    /// Numeric search.
    ///
    /// C++ equivalent: `NS::NumericSearch`
    pub const NUMERIC: Self = Self(64);

    /// Diacritic insensitive search.
    ///
    /// C++ equivalent: `NS::DiacriticInsensitiveSearch`
    pub const DIACRITIC_INSENSITIVE: Self = Self(128);

    /// Width insensitive search.
    ///
    /// C++ equivalent: `NS::WidthInsensitiveSearch`
    pub const WIDTH_INSENSITIVE: Self = Self(256);

    /// Forced ordering search.
    ///
    /// C++ equivalent: `NS::ForcedOrderingSearch`
    pub const FORCED_ORDERING: Self = Self(512);

    /// Regular expression search.
    ///
    /// C++ equivalent: `NS::RegularExpressionSearch`
    pub const REGULAR_EXPRESSION: Self = Self(1024);

    /// Returns the raw bits.
    #[inline]
    pub const fn bits(&self) -> UInteger {
        self.0
    }

    /// Creates from raw bits.
    #[inline]
    pub const fn from_bits(bits: UInteger) -> Self {
        Self(bits)
    }

    /// Check if empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Check if contains all flags in other.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Insert flags.
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// Remove flags.
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}

impl std::ops::BitOr for StringCompareOptions {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for StringCompareOptions {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitXor for StringCompareOptions {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for StringCompareOptions {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(!self.0)
    }
}

impl std::ops::BitOrAssign for StringCompareOptions {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAndAssign for StringCompareOptions {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

/// An Objective-C string object.
///
/// C++ equivalent: `NS::String`
#[repr(transparent)]
#[derive(Clone)]
pub struct String(NonNull<c_void>);

impl String {
    /// Create an empty string.
    ///
    /// C++ equivalent: `static String* string()`
    #[inline]
    pub fn string() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSString).as_ptr(), sel!(string));
            Self::from_ptr(ptr)
        }
    }

    /// Create a string from another string.
    ///
    /// C++ equivalent: `static String* string(const String* pString)`
    #[inline]
    pub fn string_with_string(string: &String) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                class!(NSString).as_ptr(),
                sel!(stringWithString:),
                string.as_ptr(),
            );
            Self::from_ptr(ptr)
        }
    }

    /// Create a string from a C string with the specified encoding.
    ///
    /// C++ equivalent: `static String* string(const char* pString, StringEncoding encoding)`
    #[inline]
    pub fn string_with_cstring(cstring: *const c_char, encoding: StringEncoding) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                class!(NSString).as_ptr(),
                sel!(stringWithCString:encoding:),
                cstring,
                encoding.0,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Allocate a new string.
    ///
    /// C++ equivalent: `static String* alloc()`
    #[inline]
    pub fn alloc() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSString).as_ptr(), sel!(alloc));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize an allocated string.
    ///
    /// C++ equivalent: `String* init()`
    #[inline]
    pub fn init(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(init));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with another string.
    ///
    /// C++ equivalent: `String* init(const String* pString)`
    #[inline]
    pub fn init_with_string(&self, string: &String) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(initWithString:), string.as_ptr());
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a C string and encoding.
    ///
    /// C++ equivalent: `String* init(const char* pString, StringEncoding encoding)`
    #[inline]
    pub fn init_with_cstring(
        &self,
        cstring: *const c_char,
        encoding: StringEncoding,
    ) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(initWithCString:encoding:),
                cstring,
                encoding.0,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with bytes, length, encoding, and free buffer flag.
    ///
    /// C++ equivalent: `String* init(void* pBytes, UInteger len, StringEncoding encoding, bool freeBuffer)`
    #[inline]
    pub fn init_with_bytes_no_copy(
        &self,
        bytes: *mut c_void,
        len: UInteger,
        encoding: StringEncoding,
        free_when_done: bool,
    ) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_4(
                self.as_ptr(),
                sel!(initWithBytesNoCopy:length:encoding:freeWhenDone:),
                bytes,
                len,
                encoding.0,
                free_when_done,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Get the character at the specified index.
    ///
    /// C++ equivalent: `unichar character(UInteger index) const`
    #[inline]
    pub fn character(&self, index: UInteger) -> Unichar {
        unsafe { msg_send_1(self.as_ptr(), sel!(characterAtIndex:), index) }
    }

    /// Get the length of the string in characters.
    ///
    /// C++ equivalent: `UInteger length() const`
    #[inline]
    pub fn length(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(length)) }
    }

    /// Get the string as a C string with the specified encoding.
    ///
    /// C++ equivalent: `const char* cString(StringEncoding encoding) const`
    #[inline]
    pub fn c_string(&self, encoding: StringEncoding) -> *const c_char {
        unsafe { msg_send_1(self.as_ptr(), sel!(cStringUsingEncoding:), encoding.0) }
    }

    /// Get the string as a UTF-8 C string.
    ///
    /// C++ equivalent: `const char* utf8String() const`
    #[inline]
    pub fn utf8_string(&self) -> *const c_char {
        unsafe { msg_send_0(self.as_ptr(), sel!(UTF8String)) }
    }

    /// Get the maximum length in bytes for the specified encoding.
    ///
    /// C++ equivalent: `UInteger maximumLengthOfBytes(StringEncoding encoding) const`
    #[inline]
    pub fn maximum_length_of_bytes(&self, encoding: StringEncoding) -> UInteger {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(maximumLengthOfBytesUsingEncoding:),
                encoding.0,
            )
        }
    }

    /// Get the actual length in bytes for the specified encoding.
    ///
    /// C++ equivalent: `UInteger lengthOfBytes(StringEncoding encoding) const`
    #[inline]
    pub fn length_of_bytes(&self, encoding: StringEncoding) -> UInteger {
        unsafe { msg_send_1(self.as_ptr(), sel!(lengthOfBytesUsingEncoding:), encoding.0) }
    }

    /// Check if this string is equal to another string.
    ///
    /// C++ equivalent: `bool isEqualToString(const String* pString) const`
    #[inline]
    pub fn is_equal_to_string(&self, string: &String) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(isEqualToString:), string.as_ptr()) }
    }

    /// Find the range of a substring with the specified options.
    ///
    /// C++ equivalent: `Range rangeOfString(const String* pString, StringCompareOptions options) const`
    #[inline]
    pub fn range_of_string(&self, string: &String, options: StringCompareOptions) -> Range {
        unsafe {
            msg_send_2(
                self.as_ptr(),
                sel!(rangeOfString:options:),
                string.as_ptr(),
                options.0,
            )
        }
    }

    /// Get the file system representation of the string.
    ///
    /// C++ equivalent: `const char* fileSystemRepresentation() const`
    #[inline]
    pub fn file_system_representation(&self) -> *const c_char {
        unsafe { msg_send_0(self.as_ptr(), sel!(fileSystemRepresentation)) }
    }

    /// Create a new string by appending another string.
    ///
    /// C++ equivalent: `String* stringByAppendingString(const String* pString) const`
    #[inline]
    pub fn string_by_appending_string(&self, string: &String) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                self.as_ptr(),
                sel!(stringByAppendingString:),
                string.as_ptr(),
            );
            Self::from_ptr(ptr)
        }
    }

    /// Perform a case-insensitive comparison with another string.
    ///
    /// C++ equivalent: `ComparisonResult caseInsensitiveCompare(const String* pString) const`
    #[inline]
    pub fn case_insensitive_compare(&self, string: &String) -> ComparisonResult {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(caseInsensitiveCompare:),
                string.as_ptr(),
            )
        }
    }

    /// Create a String from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSString object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    // Convenience methods for Rust interop

    /// Create a string from a Rust &str.
    ///
    /// This is a convenience method for Rust users.
    #[inline]
    pub fn from_str(s: &str) -> Option<Self> {
        let c_str = std::ffi::CString::new(s).ok()?;
        Self::string_with_cstring(c_str.as_ptr(), StringEncoding::UTF8)
    }

    /// Convert to a Rust String.
    ///
    /// This is a convenience method for Rust users.
    #[inline]
    pub fn to_string(&self) -> Option<std::string::String> {
        let ptr = self.utf8_string();
        if ptr.is_null() {
            return None;
        }
        unsafe {
            let c_str = std::ffi::CStr::from_ptr(ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }
}

impl Referencing for String {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

impl Copying for String {
    #[inline]
    fn copy(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_ptr(ptr)
        }
    }
}

// SAFETY: NSString is thread-safe for reference counting
unsafe impl Send for String {}
unsafe impl Sync for String {}

impl std::fmt::Debug for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_string() {
            Some(s) => write!(f, "String({:?})", s),
            None => write!(f, "String(<invalid>)"),
        }
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_string() {
            Some(s) => write!(f, "{}", s),
            None => write!(f, "<invalid>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_encoding_values() {
        assert_eq!(StringEncoding::ASCII.0, 1);
        assert_eq!(StringEncoding::UTF8.0, 4);
        assert_eq!(StringEncoding::UNICODE.0, 10);
        assert_eq!(StringEncoding::UTF16.0, 10);
        assert_eq!(StringEncoding::UTF32.0, 0x8c000100);
    }

    #[test]
    fn test_string_compare_options() {
        let options = StringCompareOptions::CASE_INSENSITIVE | StringCompareOptions::BACKWARDS;
        assert!(options.contains(StringCompareOptions::CASE_INSENSITIVE));
        assert!(options.contains(StringCompareOptions::BACKWARDS));
        assert!(!options.contains(StringCompareOptions::LITERAL));
    }
}
