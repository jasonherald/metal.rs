//! Macros for selector caching, class lookup, and type definitions.

/// Cached selector lookup.
///
/// This macro creates a lazily-initialized static selector that is
/// registered with the Objective-C runtime on first use.
///
/// # Examples
///
/// ```ignore
/// // Simple selector
/// let s = sel!(init);
///
/// // Selector with colons (method with parameters)
/// let s = sel!(initWithFrame:);
///
/// // Multiple parameter selector
/// let s = sel!(newBufferWithLength:options:);
/// ```
#[macro_export]
macro_rules! sel {
    // Single identifier without colon
    ($name:ident) => {{
        static SEL: std::sync::OnceLock<$crate::Sel> = std::sync::OnceLock::new();
        *SEL.get_or_init(|| $crate::Sel::register(stringify!($name)))
    }};
    // Selector with one or more colons (e.g., newBufferWithLength:options:)
    ($($name:ident :)+) => {{
        static SEL: std::sync::OnceLock<$crate::Sel> = std::sync::OnceLock::new();
        *SEL.get_or_init(|| $crate::Sel::register(concat!($(stringify!($name), ":"),+)))
    }};
}

/// Cached class lookup.
///
/// This macro creates a lazily-initialized static class reference that is
/// looked up from the Objective-C runtime on first use.
///
/// # Panics
///
/// Panics if the class is not found in the runtime.
///
/// # Examples
///
/// ```ignore
/// let cls = class!(NSObject);
/// let cls = class!(MTLDevice);
/// ```
#[macro_export]
macro_rules! class {
    ($name:ident) => {{
        static CLS: std::sync::OnceLock<$crate::Class> = std::sync::OnceLock::new();
        *CLS.get_or_init(|| {
            $crate::Class::get(stringify!($name)).expect(concat!(
                "class ",
                stringify!($name),
                " not found"
            ))
        })
    }};
}

/// Define a Metal enum type.
///
/// Creates a newtype wrapper around a primitive integer type with
/// associated constants for each variant.
///
/// # Examples
///
/// ```ignore
/// metal_enum! {
///     /// Pixel format for textures.
///     pub enum PixelFormat: u64 {
///         Invalid = 0,
///         A8Unorm = 1,
///         R8Unorm = 10,
///     }
/// }
/// ```
#[macro_export]
macro_rules! metal_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident : $repr:ty {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident = $value:expr
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
        $vis struct $name(pub $repr);

        impl $name {
            $(
                $(#[$variant_meta])*
                pub const $variant: Self = Self($value);
            )*

            /// Returns the raw integer value.
            #[inline]
            pub const fn raw(&self) -> $repr {
                self.0
            }

            /// Creates from a raw integer value.
            #[inline]
            pub const fn from_raw(value: $repr) -> Self {
                Self(value)
            }
        }

        impl From<$repr> for $name {
            #[inline]
            fn from(value: $repr) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $repr {
            #[inline]
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

/// Define a Metal options/bitflags type.
///
/// Creates a newtype wrapper around a primitive integer type with
/// bitwise operations and associated constants for each flag.
///
/// # Examples
///
/// ```ignore
/// metal_options! {
///     /// Resource storage and caching options.
///     pub struct ResourceOptions: u64 {
///         const CPU_CACHE_MODE_DEFAULT = 0;
///         const CPU_CACHE_MODE_WRITE_COMBINED = 1 << 0;
///         const STORAGE_MODE_SHARED = 0 << 4;
///         const STORAGE_MODE_MANAGED = 1 << 4;
///         const STORAGE_MODE_PRIVATE = 2 << 4;
///     }
/// }
///
/// let opts = ResourceOptions::STORAGE_MODE_PRIVATE | ResourceOptions::CPU_CACHE_MODE_WRITE_COMBINED;
/// ```
#[macro_export]
macro_rules! metal_options {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident : $repr:ty {
            $(
                $(#[$flag_meta:meta])*
                const $flag:ident = $value:expr;
            )*
        }
    ) => {
        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
        $vis struct $name(pub $repr);

        impl $name {
            /// Empty set of flags.
            pub const NONE: Self = Self(0);

            $(
                $(#[$flag_meta])*
                pub const $flag: Self = Self($value);
            )*

            /// Returns the raw bits.
            #[inline]
            pub const fn bits(&self) -> $repr {
                self.0
            }

            /// Creates from raw bits.
            #[inline]
            pub const fn from_bits(bits: $repr) -> Self {
                Self(bits)
            }

            /// Check if empty.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.0 == 0
            }

            /// Check if contains all flags in `other`.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }

            /// Check if contains any flags in `other`.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                (self.0 & other.0) != 0
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

            /// Toggle flags.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                self.0 ^= other.0;
            }

            /// Set flags based on a boolean.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                if value {
                    self.insert(other);
                } else {
                    self.remove(other);
                }
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl std::ops::BitAnd for $name {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl std::ops::BitXor for $name {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }

        impl std::ops::Not for $name {
            type Output = Self;
            #[inline]
            fn not(self) -> Self {
                Self(!self.0)
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;
            /// Difference: `self & !rhs`
            #[inline]
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 & !rhs.0)
            }
        }

        impl std::ops::SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 &= !rhs.0;
            }
        }

        impl From<$repr> for $name {
            #[inline]
            fn from(value: $repr) -> Self {
                Self(value)
            }
        }

        impl From<$name> for $repr {
            #[inline]
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_metal_enum() {
        metal_enum! {
            pub enum TestEnum: u32 {
                Zero = 0,
                One = 1,
                Two = 2,
            }
        }

        assert_eq!(TestEnum::Zero.raw(), 0);
        assert_eq!(TestEnum::One.raw(), 1);
        assert_eq!(TestEnum::Two.raw(), 2);
        assert_eq!(TestEnum::from_raw(1), TestEnum::One);
    }

    #[test]
    fn test_metal_options() {
        metal_options! {
            pub struct TestFlags: u32 {
                const A = 1 << 0;
                const B = 1 << 1;
                const C = 1 << 2;
            }
        }

        let flags = TestFlags::A | TestFlags::B;
        assert!(flags.contains(TestFlags::A));
        assert!(flags.contains(TestFlags::B));
        assert!(!flags.contains(TestFlags::C));
        assert!(flags.intersects(TestFlags::A));
        assert!(!flags.intersects(TestFlags::C));
        assert_eq!(flags.bits(), 0b11);

        let mut flags2 = TestFlags::NONE;
        flags2.insert(TestFlags::C);
        assert!(flags2.contains(TestFlags::C));
        flags2.remove(TestFlags::C);
        assert!(flags2.is_empty());
    }
}
