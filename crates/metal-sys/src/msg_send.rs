//! Architecture-aware Objective-C message sending.
//!
//! This module provides low-level message sending functions that handle
//! the different calling conventions required by the Objective-C runtime
//! on different architectures:
//!
//! - **x86_64**: Uses `objc_msgSend_fpret` for floating-point returns,
//!   `objc_msgSend_stret` for structs > 16 bytes
//! - **x86 (i386)**: Uses `objc_msgSend_fpret` for floating-point returns,
//!   `objc_msgSend_stret` for structs > 8 bytes
//! - **ARM64**: Always uses standard `objc_msgSend` (no special variants needed)
//! - **ARM32**: Uses `objc_msgSend_stret` for class types > 4 bytes
//!
//! # C++ Equivalent
//!
//! From metal-cpp Foundation/NSObject.hpp:
//! ```cpp
//! template <typename _Ret, typename... _Args>
//! _NS_INLINE _Ret NS::Object::sendMessage(const void* pObj, SEL selector, _Args... args)
//! ```

use std::ffi::c_void;

use crate::runtime::Sel;

// Link against libobjc for message sending functions
#[link(name = "objc")]
unsafe extern "C" {
    fn objc_msgSend();

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    fn objc_msgSend_fpret();

    #[cfg(not(target_arch = "aarch64"))]
    fn objc_msgSend_stret();
}

/// Struct return threshold by architecture.
///
/// Structs larger than this threshold require `objc_msgSend_stret`.
/// This matches the C++ code in NSObject.hpp:
/// ```cpp
/// constexpr size_t kStructLimit = (sizeof(std::uintptr_t) << 1);
/// ```
#[cfg(target_arch = "x86_64")]
const STRET_THRESHOLD: usize = 16;

#[cfg(target_arch = "x86")]
const STRET_THRESHOLD: usize = 8;

#[cfg(target_arch = "aarch64")]
#[allow(dead_code)]
const STRET_THRESHOLD: usize = usize::MAX; // Never use stret on ARM64

#[cfg(target_arch = "arm")]
const STRET_THRESHOLD: usize = 4;

/// Check if a type requires the stret calling convention.
#[inline]
#[allow(dead_code)]
#[allow(clippy::absurd_extreme_comparisons)]
const fn requires_stret<T>() -> bool {
    std::mem::size_of::<T>() > STRET_THRESHOLD
}

/// Check if a type is a floating-point type (for fpret on x86).
#[inline]
#[allow(dead_code)]
fn is_float<T: 'static>() -> bool {
    use std::any::TypeId;
    TypeId::of::<T>() == TypeId::of::<f32>() || TypeId::of::<T>() == TypeId::of::<f64>()
}

// =============================================================================
// Message send with 0 arguments
// =============================================================================

/// Send a message with no arguments.
///
/// # Safety
///
/// - `obj` must be a valid Objective-C object pointer or class pointer
/// - `sel` must be a valid selector for a method that takes no arguments
/// - The return type `R` must match the actual return type of the method
#[inline]
pub unsafe fn msg_send_0<R>(obj: *const c_void, sel: Sel) -> R {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    if is_float::<R>() {
        let f: unsafe extern "C" fn(*const c_void, Sel) -> R =
            unsafe { std::mem::transmute(objc_msgSend_fpret as *const c_void) };
        return unsafe { f(obj, sel) };
    }

    #[cfg(not(target_arch = "aarch64"))]
    if requires_stret::<R>() {
        let mut result = std::mem::MaybeUninit::<R>::uninit();
        let f: unsafe extern "C" fn(*mut R, *const c_void, Sel) =
            unsafe { std::mem::transmute(objc_msgSend_stret as *const c_void) };
        unsafe { f(result.as_mut_ptr(), obj, sel) };
        return unsafe { result.assume_init() };
    }

    let f: unsafe extern "C" fn(*const c_void, Sel) -> R =
        unsafe { std::mem::transmute(objc_msgSend as *const c_void) };
    unsafe { f(obj, sel) }
}

// =============================================================================
// Message send with 1 argument
// =============================================================================

/// Send a message with 1 argument.
///
/// # Safety
///
/// - `obj` must be a valid Objective-C object pointer or class pointer
/// - `sel` must be a valid selector for a method that takes 1 argument
/// - The argument type `A` must match the expected parameter type
/// - The return type `R` must match the actual return type of the method
#[inline]
pub unsafe fn msg_send_1<R, A>(obj: *const c_void, sel: Sel, a: A) -> R {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    if is_float::<R>() {
        let f: unsafe extern "C" fn(*const c_void, Sel, A) -> R =
            unsafe { std::mem::transmute(objc_msgSend_fpret as *const c_void) };
        return unsafe { f(obj, sel, a) };
    }

    #[cfg(not(target_arch = "aarch64"))]
    if requires_stret::<R>() {
        let mut result = std::mem::MaybeUninit::<R>::uninit();
        let f: unsafe extern "C" fn(*mut R, *const c_void, Sel, A) =
            unsafe { std::mem::transmute(objc_msgSend_stret as *const c_void) };
        unsafe { f(result.as_mut_ptr(), obj, sel, a) };
        return unsafe { result.assume_init() };
    }

    let f: unsafe extern "C" fn(*const c_void, Sel, A) -> R =
        unsafe { std::mem::transmute(objc_msgSend as *const c_void) };
    unsafe { f(obj, sel, a) }
}

// =============================================================================
// Message send with 2 arguments
// =============================================================================

/// Send a message with 2 arguments.
///
/// # Safety
///
/// See `msg_send_0` for safety requirements.
#[inline]
pub unsafe fn msg_send_2<R, A, B>(obj: *const c_void, sel: Sel, a: A, b: B) -> R {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    if is_float::<R>() {
        let f: unsafe extern "C" fn(*const c_void, Sel, A, B) -> R =
            unsafe { std::mem::transmute(objc_msgSend_fpret as *const c_void) };
        return unsafe { f(obj, sel, a, b) };
    }

    #[cfg(not(target_arch = "aarch64"))]
    if requires_stret::<R>() {
        let mut result = std::mem::MaybeUninit::<R>::uninit();
        let f: unsafe extern "C" fn(*mut R, *const c_void, Sel, A, B) =
            unsafe { std::mem::transmute(objc_msgSend_stret as *const c_void) };
        unsafe { f(result.as_mut_ptr(), obj, sel, a, b) };
        return unsafe { result.assume_init() };
    }

    let f: unsafe extern "C" fn(*const c_void, Sel, A, B) -> R =
        unsafe { std::mem::transmute(objc_msgSend as *const c_void) };
    unsafe { f(obj, sel, a, b) }
}

// =============================================================================
// Message send with 3 arguments
// =============================================================================

/// Send a message with 3 arguments.
///
/// # Safety
///
/// See `msg_send_0` for safety requirements.
#[inline]
pub unsafe fn msg_send_3<R, A, B, C>(obj: *const c_void, sel: Sel, a: A, b: B, c: C) -> R {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    if is_float::<R>() {
        let f: unsafe extern "C" fn(*const c_void, Sel, A, B, C) -> R =
            unsafe { std::mem::transmute(objc_msgSend_fpret as *const c_void) };
        return unsafe { f(obj, sel, a, b, c) };
    }

    #[cfg(not(target_arch = "aarch64"))]
    if requires_stret::<R>() {
        let mut result = std::mem::MaybeUninit::<R>::uninit();
        let f: unsafe extern "C" fn(*mut R, *const c_void, Sel, A, B, C) =
            unsafe { std::mem::transmute(objc_msgSend_stret as *const c_void) };
        unsafe { f(result.as_mut_ptr(), obj, sel, a, b, c) };
        return unsafe { result.assume_init() };
    }

    let f: unsafe extern "C" fn(*const c_void, Sel, A, B, C) -> R =
        unsafe { std::mem::transmute(objc_msgSend as *const c_void) };
    unsafe { f(obj, sel, a, b, c) }
}

// =============================================================================
// Message send with 4 arguments
// =============================================================================

/// Send a message with 4 arguments.
///
/// # Safety
///
/// See `msg_send_0` for safety requirements.
#[inline]
pub unsafe fn msg_send_4<R, A, B, C, D>(
    obj: *const c_void,
    sel: Sel,
    a: A,
    b: B,
    c: C,
    d: D,
) -> R {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    if is_float::<R>() {
        let f: unsafe extern "C" fn(*const c_void, Sel, A, B, C, D) -> R =
            unsafe { std::mem::transmute(objc_msgSend_fpret as *const c_void) };
        return unsafe { f(obj, sel, a, b, c, d) };
    }

    #[cfg(not(target_arch = "aarch64"))]
    if requires_stret::<R>() {
        let mut result = std::mem::MaybeUninit::<R>::uninit();
        let f: unsafe extern "C" fn(*mut R, *const c_void, Sel, A, B, C, D) =
            unsafe { std::mem::transmute(objc_msgSend_stret as *const c_void) };
        unsafe { f(result.as_mut_ptr(), obj, sel, a, b, c, d) };
        return unsafe { result.assume_init() };
    }

    let f: unsafe extern "C" fn(*const c_void, Sel, A, B, C, D) -> R =
        unsafe { std::mem::transmute(objc_msgSend as *const c_void) };
    unsafe { f(obj, sel, a, b, c, d) }
}

// =============================================================================
// Message send with 5 arguments
// =============================================================================

/// Send a message with 5 arguments.
///
/// # Safety
///
/// See `msg_send_0` for safety requirements.
#[inline]
pub unsafe fn msg_send_5<R, A, B, C, D, E>(
    obj: *const c_void,
    sel: Sel,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
) -> R {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    if is_float::<R>() {
        let f: unsafe extern "C" fn(*const c_void, Sel, A, B, C, D, E) -> R =
            unsafe { std::mem::transmute(objc_msgSend_fpret as *const c_void) };
        return unsafe { f(obj, sel, a, b, c, d, e) };
    }

    #[cfg(not(target_arch = "aarch64"))]
    if requires_stret::<R>() {
        let mut result = std::mem::MaybeUninit::<R>::uninit();
        let f: unsafe extern "C" fn(*mut R, *const c_void, Sel, A, B, C, D, E) =
            unsafe { std::mem::transmute(objc_msgSend_stret as *const c_void) };
        unsafe { f(result.as_mut_ptr(), obj, sel, a, b, c, d, e) };
        return unsafe { result.assume_init() };
    }

    let f: unsafe extern "C" fn(*const c_void, Sel, A, B, C, D, E) -> R =
        unsafe { std::mem::transmute(objc_msgSend as *const c_void) };
    unsafe { f(obj, sel, a, b, c, d, e) }
}

// =============================================================================
// Message send with 6 arguments
// =============================================================================

/// Send a message with 6 arguments.
///
/// # Safety
///
/// See `msg_send_0` for safety requirements.
#[inline]
#[allow(clippy::too_many_arguments)]
pub unsafe fn msg_send_6<R, A, B, C, D, E, F>(
    obj: *const c_void,
    sel: Sel,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f_arg: F,
) -> R {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    if is_float::<R>() {
        let func: unsafe extern "C" fn(*const c_void, Sel, A, B, C, D, E, F) -> R =
            unsafe { std::mem::transmute(objc_msgSend_fpret as *const c_void) };
        return unsafe { func(obj, sel, a, b, c, d, e, f_arg) };
    }

    #[cfg(not(target_arch = "aarch64"))]
    if requires_stret::<R>() {
        let mut result = std::mem::MaybeUninit::<R>::uninit();
        let func: unsafe extern "C" fn(*mut R, *const c_void, Sel, A, B, C, D, E, F) =
            unsafe { std::mem::transmute(objc_msgSend_stret as *const c_void) };
        unsafe { func(result.as_mut_ptr(), obj, sel, a, b, c, d, e, f_arg) };
        return unsafe { result.assume_init() };
    }

    let func: unsafe extern "C" fn(*const c_void, Sel, A, B, C, D, E, F) -> R =
        unsafe { std::mem::transmute(objc_msgSend as *const c_void) };
    unsafe { func(obj, sel, a, b, c, d, e, f_arg) }
}

// =============================================================================
// Message send with 7 arguments
// =============================================================================

/// Send a message with 7 arguments.
///
/// # Safety
///
/// See `msg_send_0` for safety requirements.
#[inline]
#[allow(clippy::too_many_arguments)]
pub unsafe fn msg_send_7<R, A, B, C, D, E, F, G>(
    obj: *const c_void,
    sel: Sel,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f_arg: F,
    g: G,
) -> R {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    if is_float::<R>() {
        let func: unsafe extern "C" fn(*const c_void, Sel, A, B, C, D, E, F, G) -> R =
            unsafe { std::mem::transmute(objc_msgSend_fpret as *const c_void) };
        return unsafe { func(obj, sel, a, b, c, d, e, f_arg, g) };
    }

    #[cfg(not(target_arch = "aarch64"))]
    if requires_stret::<R>() {
        let mut result = std::mem::MaybeUninit::<R>::uninit();
        let func: unsafe extern "C" fn(*mut R, *const c_void, Sel, A, B, C, D, E, F, G) =
            unsafe { std::mem::transmute(objc_msgSend_stret as *const c_void) };
        unsafe { func(result.as_mut_ptr(), obj, sel, a, b, c, d, e, f_arg, g) };
        return unsafe { result.assume_init() };
    }

    let func: unsafe extern "C" fn(*const c_void, Sel, A, B, C, D, E, F, G) -> R =
        unsafe { std::mem::transmute(objc_msgSend as *const c_void) };
    unsafe { func(obj, sel, a, b, c, d, e, f_arg, g) }
}

// =============================================================================
// Message send with 8 arguments
// =============================================================================

/// Send a message with 8 arguments.
///
/// # Safety
///
/// See `msg_send_0` for safety requirements.
#[inline]
#[allow(clippy::too_many_arguments)]
pub unsafe fn msg_send_8<R, A, B, C, D, E, F, G, H>(
    obj: *const c_void,
    sel: Sel,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f_arg: F,
    g: G,
    h: H,
) -> R {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    if is_float::<R>() {
        let func: unsafe extern "C" fn(*const c_void, Sel, A, B, C, D, E, F, G, H) -> R =
            unsafe { std::mem::transmute(objc_msgSend_fpret as *const c_void) };
        return unsafe { func(obj, sel, a, b, c, d, e, f_arg, g, h) };
    }

    #[cfg(not(target_arch = "aarch64"))]
    if requires_stret::<R>() {
        let mut result = std::mem::MaybeUninit::<R>::uninit();
        let func: unsafe extern "C" fn(*mut R, *const c_void, Sel, A, B, C, D, E, F, G, H) =
            unsafe { std::mem::transmute(objc_msgSend_stret as *const c_void) };
        unsafe { func(result.as_mut_ptr(), obj, sel, a, b, c, d, e, f_arg, g, h) };
        return unsafe { result.assume_init() };
    }

    let func: unsafe extern "C" fn(*const c_void, Sel, A, B, C, D, E, F, G, H) -> R =
        unsafe { std::mem::transmute(objc_msgSend as *const c_void) };
    unsafe { func(obj, sel, a, b, c, d, e, f_arg, g, h) }
}

// =============================================================================
// Message send with 9 arguments (for texture loading and similar)
// =============================================================================

/// Send a message with 9 arguments.
///
/// # Safety
///
/// See `msg_send_0` for safety requirements.
#[inline]
#[allow(clippy::too_many_arguments)]
pub unsafe fn msg_send_9<R, A, B, C, D, E, F, G, H, I>(
    obj: *const c_void,
    sel: Sel,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f_arg: F,
    g: G,
    h: H,
    i: I,
) -> R {
    // For 9+ args, we don't bother with fpret optimization (rare case)
    #[cfg(not(target_arch = "aarch64"))]
    if requires_stret::<R>() {
        let mut result = std::mem::MaybeUninit::<R>::uninit();
        let func: unsafe extern "C" fn(*mut R, *const c_void, Sel, A, B, C, D, E, F, G, H, I) =
            unsafe { std::mem::transmute(objc_msgSend_stret as *const c_void) };
        unsafe { func(result.as_mut_ptr(), obj, sel, a, b, c, d, e, f_arg, g, h, i) };
        return unsafe { result.assume_init() };
    }

    let func: unsafe extern "C" fn(*const c_void, Sel, A, B, C, D, E, F, G, H, I) -> R =
        unsafe { std::mem::transmute(objc_msgSend as *const c_void) };
    unsafe { func(obj, sel, a, b, c, d, e, f_arg, g, h, i) }
}

// =============================================================================
// Message send with 10 arguments (for IO command buffer texture loading)
// =============================================================================

/// Send a message with 10 arguments.
///
/// # Safety
///
/// See `msg_send_0` for safety requirements.
#[inline]
#[allow(clippy::too_many_arguments)]
pub unsafe fn msg_send_10<R, A, B, C, D, E, F, G, H, I, J>(
    obj: *const c_void,
    sel: Sel,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f_arg: F,
    g: G,
    h: H,
    i: I,
    j: J,
) -> R {
    #[cfg(not(target_arch = "aarch64"))]
    if requires_stret::<R>() {
        let mut result = std::mem::MaybeUninit::<R>::uninit();
        let func: unsafe extern "C" fn(
            *mut R,
            *const c_void,
            Sel,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
        ) = unsafe { std::mem::transmute(objc_msgSend_stret as *const c_void) };
        unsafe {
            func(
                result.as_mut_ptr(),
                obj,
                sel,
                a,
                b,
                c,
                d,
                e,
                f_arg,
                g,
                h,
                i,
                j,
            )
        };
        return unsafe { result.assume_init() };
    }

    let func: unsafe extern "C" fn(*const c_void, Sel, A, B, C, D, E, F, G, H, I, J) -> R =
        unsafe { std::mem::transmute(objc_msgSend as *const c_void) };
    unsafe { func(obj, sel, a, b, c, d, e, f_arg, g, h, i, j) }
}
