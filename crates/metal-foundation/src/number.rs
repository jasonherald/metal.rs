//! Value and Number types for Foundation.
//!
//! Corresponds to `Foundation/NSNumber.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! class Value : public Copying<Value> {
//! public:
//!     static Value* value(const void* pValue, const char* pType);
//!     static Value* value(const void* pPointer);
//!     static Value* alloc();
//!     Value*        init(const void* pValue, const char* pType);
//!     Value*        init(const class Coder* pCoder);
//!     void          getValue(void* pValue, UInteger size) const;
//!     const char*   objCType() const;
//!     bool          isEqualToValue(Value* pValue) const;
//!     void*         pointerValue() const;
//! };
//!
//! class Number : public Copying<Number, Value> {
//! public:
//!     // Many factory methods and accessors for all numeric types
//! };
//! }
//! ```

use std::ffi::{c_char, c_void};
use std::ptr::NonNull;

use metal_sys::{class, msg_send_0, msg_send_1, msg_send_2, sel};

use crate::object::{Copying, Referencing};
use crate::objc_runtime::ComparisonResult;
use crate::string::String;
use crate::types::{Integer, UInteger};

/// An Objective-C value object.
///
/// C++ equivalent: `NS::Value`
#[repr(transparent)]
#[derive(Clone)]
pub struct Value(NonNull<c_void>);

impl Value {
    /// Create a value from bytes and type encoding.
    ///
    /// C++ equivalent: `static Value* value(const void* pValue, const char* pType)`
    #[inline]
    pub fn value(value: *const c_void, obj_c_type: *const c_char) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                class!(NSValue).as_ptr(),
                sel!(valueWithBytes:objCType:),
                value,
                obj_c_type,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Create a value from a pointer.
    ///
    /// C++ equivalent: `static Value* value(const void* pPointer)`
    #[inline]
    pub fn value_with_pointer(pointer: *const c_void) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSValue).as_ptr(), sel!(valueWithPointer:), pointer);
            Self::from_ptr(ptr)
        }
    }

    /// Allocate a new value.
    ///
    /// C++ equivalent: `static Value* alloc()`
    #[inline]
    pub fn alloc() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSValue).as_ptr(), sel!(alloc));
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with bytes and type encoding.
    ///
    /// C++ equivalent: `Value* init(const void* pValue, const char* pType)`
    #[inline]
    pub fn init_with_bytes(&self, value: *const c_void, obj_c_type: *const c_char) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_2(
                self.as_ptr(),
                sel!(initWithBytes:objCType:),
                value,
                obj_c_type,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a coder.
    ///
    /// C++ equivalent: `Value* init(const class Coder* pCoder)`
    #[inline]
    pub fn init_with_coder(&self, coder: *const c_void) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithCoder:), coder);
            Self::from_ptr(ptr)
        }
    }

    /// Get the value bytes.
    ///
    /// C++ equivalent: `void getValue(void* pValue, UInteger size) const`
    #[inline]
    pub fn get_value(&self, value: *mut c_void, size: UInteger) {
        unsafe {
            let _: () = msg_send_2(self.as_ptr(), sel!(getValue:size:), value, size);
        }
    }

    /// Get the Objective-C type encoding.
    ///
    /// C++ equivalent: `const char* objCType() const`
    #[inline]
    pub fn objc_type(&self) -> *const c_char {
        unsafe { msg_send_0(self.as_ptr(), sel!(objCType)) }
    }

    /// Check if equal to another value.
    ///
    /// C++ equivalent: `bool isEqualToValue(Value* pValue) const`
    #[inline]
    pub fn is_equal_to_value(&self, value: &Value) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(isEqualToValue:), value.as_ptr()) }
    }

    /// Get the pointer value.
    ///
    /// C++ equivalent: `void* pointerValue() const`
    #[inline]
    pub fn pointer_value(&self) -> *mut c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(pointerValue)) }
    }

    /// Create a Value from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSValue object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
}

impl Referencing for Value {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

impl Copying for Value {
    #[inline]
    fn copy(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_ptr(ptr)
        }
    }
}

unsafe impl Send for Value {}
unsafe impl Sync for Value {}

/// An Objective-C number object.
///
/// C++ equivalent: `NS::Number`
#[repr(transparent)]
#[derive(Clone)]
pub struct Number(NonNull<c_void>);

impl Number {
    // Static factory methods

    /// Create a number from a char.
    #[inline]
    pub fn number_with_char(value: i8) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSNumber).as_ptr(), sel!(numberWithChar:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Create a number from an unsigned char.
    #[inline]
    pub fn number_with_unsigned_char(value: u8) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSNumber).as_ptr(), sel!(numberWithUnsignedChar:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Create a number from a short.
    #[inline]
    pub fn number_with_short(value: i16) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSNumber).as_ptr(), sel!(numberWithShort:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Create a number from an unsigned short.
    #[inline]
    pub fn number_with_unsigned_short(value: u16) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSNumber).as_ptr(), sel!(numberWithUnsignedShort:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Create a number from an int.
    #[inline]
    pub fn number_with_int(value: i32) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSNumber).as_ptr(), sel!(numberWithInt:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Create a number from an unsigned int.
    #[inline]
    pub fn number_with_unsigned_int(value: u32) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSNumber).as_ptr(), sel!(numberWithUnsignedInt:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Create a number from a long.
    #[inline]
    pub fn number_with_long(value: std::ffi::c_long) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSNumber).as_ptr(), sel!(numberWithLong:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Create a number from an unsigned long.
    #[inline]
    pub fn number_with_unsigned_long(value: std::ffi::c_ulong) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSNumber).as_ptr(), sel!(numberWithUnsignedLong:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Create a number from a long long.
    #[inline]
    pub fn number_with_long_long(value: i64) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSNumber).as_ptr(), sel!(numberWithLongLong:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Create a number from an unsigned long long.
    #[inline]
    pub fn number_with_unsigned_long_long(value: u64) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(
                class!(NSNumber).as_ptr(),
                sel!(numberWithUnsignedLongLong:),
                value,
            );
            Self::from_ptr(ptr)
        }
    }

    /// Create a number from a float.
    #[inline]
    pub fn number_with_float(value: f32) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSNumber).as_ptr(), sel!(numberWithFloat:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Create a number from a double.
    #[inline]
    pub fn number_with_double(value: f64) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSNumber).as_ptr(), sel!(numberWithDouble:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Create a number from a bool.
    #[inline]
    pub fn number_with_bool(value: bool) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(class!(NSNumber).as_ptr(), sel!(numberWithBool:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Allocate a new number.
    #[inline]
    pub fn alloc() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSNumber).as_ptr(), sel!(alloc));
            Self::from_ptr(ptr)
        }
    }

    // Init methods

    /// Initialize with a coder.
    #[inline]
    pub fn init_with_coder(&self, coder: *const c_void) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithCoder:), coder);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a char.
    #[inline]
    pub fn init_with_char(&self, value: i8) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithChar:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with an unsigned char.
    #[inline]
    pub fn init_with_unsigned_char(&self, value: u8) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithUnsignedChar:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a short.
    #[inline]
    pub fn init_with_short(&self, value: i16) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithShort:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with an unsigned short.
    #[inline]
    pub fn init_with_unsigned_short(&self, value: u16) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithUnsignedShort:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with an int.
    #[inline]
    pub fn init_with_int(&self, value: i32) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithInt:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with an unsigned int.
    #[inline]
    pub fn init_with_unsigned_int(&self, value: u32) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithUnsignedInt:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a long.
    #[inline]
    pub fn init_with_long(&self, value: std::ffi::c_long) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithLong:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with an unsigned long.
    #[inline]
    pub fn init_with_unsigned_long(&self, value: std::ffi::c_ulong) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithUnsignedLong:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a long long.
    #[inline]
    pub fn init_with_long_long(&self, value: i64) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithLongLong:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with an unsigned long long.
    #[inline]
    pub fn init_with_unsigned_long_long(&self, value: u64) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void =
                msg_send_1(self.as_ptr(), sel!(initWithUnsignedLongLong:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a float.
    #[inline]
    pub fn init_with_float(&self, value: f32) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithFloat:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a double.
    #[inline]
    pub fn init_with_double(&self, value: f64) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithDouble:), value);
            Self::from_ptr(ptr)
        }
    }

    /// Initialize with a bool.
    #[inline]
    pub fn init_with_bool(&self, value: bool) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_1(self.as_ptr(), sel!(initWithBool:), value);
            Self::from_ptr(ptr)
        }
    }

    // Value accessors

    /// Get the char value.
    #[inline]
    pub fn char_value(&self) -> i8 {
        unsafe { msg_send_0(self.as_ptr(), sel!(charValue)) }
    }

    /// Get the unsigned char value.
    #[inline]
    pub fn unsigned_char_value(&self) -> u8 {
        unsafe { msg_send_0(self.as_ptr(), sel!(unsignedCharValue)) }
    }

    /// Get the short value.
    #[inline]
    pub fn short_value(&self) -> i16 {
        unsafe { msg_send_0(self.as_ptr(), sel!(shortValue)) }
    }

    /// Get the unsigned short value.
    #[inline]
    pub fn unsigned_short_value(&self) -> u16 {
        unsafe { msg_send_0(self.as_ptr(), sel!(unsignedShortValue)) }
    }

    /// Get the int value.
    #[inline]
    pub fn int_value(&self) -> i32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(intValue)) }
    }

    /// Get the unsigned int value.
    #[inline]
    pub fn unsigned_int_value(&self) -> u32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(unsignedIntValue)) }
    }

    /// Get the long value.
    #[inline]
    pub fn long_value(&self) -> std::ffi::c_long {
        unsafe { msg_send_0(self.as_ptr(), sel!(longValue)) }
    }

    /// Get the unsigned long value.
    #[inline]
    pub fn unsigned_long_value(&self) -> std::ffi::c_ulong {
        unsafe { msg_send_0(self.as_ptr(), sel!(unsignedLongValue)) }
    }

    /// Get the long long value.
    #[inline]
    pub fn long_long_value(&self) -> i64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(longLongValue)) }
    }

    /// Get the unsigned long long value.
    #[inline]
    pub fn unsigned_long_long_value(&self) -> u64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(unsignedLongLongValue)) }
    }

    /// Get the float value.
    #[inline]
    pub fn float_value(&self) -> f32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(floatValue)) }
    }

    /// Get the double value.
    #[inline]
    pub fn double_value(&self) -> f64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(doubleValue)) }
    }

    /// Get the bool value.
    #[inline]
    pub fn bool_value(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(boolValue)) }
    }

    /// Get the integer value.
    #[inline]
    pub fn integer_value(&self) -> Integer {
        unsafe { msg_send_0(self.as_ptr(), sel!(integerValue)) }
    }

    /// Get the unsigned integer value.
    #[inline]
    pub fn unsigned_integer_value(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(unsignedIntegerValue)) }
    }

    /// Get the string value.
    #[inline]
    pub fn string_value(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(stringValue)) }
    }

    /// Compare with another number.
    #[inline]
    pub fn compare(&self, other: &Number) -> ComparisonResult {
        unsafe { msg_send_1(self.as_ptr(), sel!(compare:), other.as_ptr()) }
    }

    /// Check if equal to another number.
    #[inline]
    pub fn is_equal_to_number(&self, number: &Number) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(isEqualToNumber:), number.as_ptr()) }
    }

    /// Get description with locale.
    #[inline]
    pub fn description_with_locale(&self, locale: *const c_void) -> *mut String {
        unsafe { msg_send_1(self.as_ptr(), sel!(descriptionWithLocale:), locale) }
    }

    /// Create a Number from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSNumber object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
}

impl Referencing for Number {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

impl Copying for Number {
    #[inline]
    fn copy(&self) -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_ptr(ptr)
        }
    }
}

unsafe impl Send for Number {}
unsafe impl Sync for Number {}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Number")
            .field("ptr", &self.0)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_size() {
        assert_eq!(std::mem::size_of::<Number>(), std::mem::size_of::<*mut c_void>());
    }

    #[test]
    fn test_value_size() {
        assert_eq!(std::mem::size_of::<Value>(), std::mem::size_of::<*mut c_void>());
    }
}
