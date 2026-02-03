//! Texture enumerations.
//!
//! Corresponds to `Metal/MTLTexture.hpp`.

use mtl_foundation::{Integer, UInteger};

/// Types of textures.
///
/// C++ equivalent: `MTL::TextureType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextureType(pub UInteger);

impl TextureType {
    pub const TYPE_1D: Self = Self(0);
    pub const TYPE_1D_ARRAY: Self = Self(1);
    pub const TYPE_2D: Self = Self(2);
    pub const TYPE_2D_ARRAY: Self = Self(3);
    pub const TYPE_2D_MULTISAMPLE: Self = Self(4);
    pub const TYPE_CUBE: Self = Self(5);
    pub const TYPE_CUBE_ARRAY: Self = Self(6);
    pub const TYPE_3D: Self = Self(7);
    pub const TYPE_2D_MULTISAMPLE_ARRAY: Self = Self(8);
    pub const TYPE_TEXTURE_BUFFER: Self = Self(9);
}

/// Texture swizzle values.
///
/// C++ equivalent: `MTL::TextureSwizzle`
///
/// Note: This uses uint8_t in C++, not NS::UInteger
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextureSwizzle(pub u8);

impl TextureSwizzle {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const RED: Self = Self(2);
    pub const GREEN: Self = Self(3);
    pub const BLUE: Self = Self(4);
    pub const ALPHA: Self = Self(5);
}

/// Texture compression type.
///
/// C++ equivalent: `MTL::TextureCompressionType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextureCompressionType(pub Integer);

impl TextureCompressionType {
    pub const LOSSLESS: Self = Self(0);
    pub const LOSSY: Self = Self(1);
}

/// Texture usage flags (bitflags).
///
/// C++ equivalent: `MTL::TextureUsage`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextureUsage(pub UInteger);

impl TextureUsage {
    pub const UNKNOWN: Self = Self(0);
    pub const SHADER_READ: Self = Self(1);
    pub const SHADER_WRITE: Self = Self(1 << 1);
    pub const RENDER_TARGET: Self = Self(1 << 2);
    pub const PIXEL_FORMAT_VIEW: Self = Self(1 << 4);
    pub const SHADER_ATOMIC: Self = Self(1 << 5);

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
}

impl std::ops::BitOr for TextureUsage {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for TextureUsage {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for TextureUsage {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Texture swizzle channels structure.
///
/// C++ equivalent: `MTL::TextureSwizzleChannels`
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TextureSwizzleChannels {
    pub red: TextureSwizzle,
    pub green: TextureSwizzle,
    pub blue: TextureSwizzle,
    pub alpha: TextureSwizzle,
}

impl TextureSwizzleChannels {
    /// Create a new TextureSwizzleChannels.
    #[inline]
    pub const fn new(
        red: TextureSwizzle,
        green: TextureSwizzle,
        blue: TextureSwizzle,
        alpha: TextureSwizzle,
    ) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Create a new TextureSwizzleChannels (C++ style factory method).
    ///
    /// C++ equivalent: `static TextureSwizzleChannels Make(...)`
    #[inline]
    pub const fn make(
        r: TextureSwizzle,
        g: TextureSwizzle,
        b: TextureSwizzle,
        a: TextureSwizzle,
    ) -> Self {
        Self::new(r, g, b, a)
    }

    /// Get the default swizzle channels (identity mapping).
    ///
    /// C++ equivalent: `static TextureSwizzleChannels Default()`
    #[inline]
    pub const fn default_channels() -> Self {
        Self {
            red: TextureSwizzle::RED,
            green: TextureSwizzle::GREEN,
            blue: TextureSwizzle::BLUE,
            alpha: TextureSwizzle::ALPHA,
        }
    }
}

impl Default for TextureSwizzleChannels {
    fn default() -> Self {
        Self::default_channels()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_type_values() {
        assert_eq!(TextureType::TYPE_1D.0, 0);
        assert_eq!(TextureType::TYPE_2D.0, 2);
        assert_eq!(TextureType::TYPE_3D.0, 7);
        assert_eq!(TextureType::TYPE_TEXTURE_BUFFER.0, 9);
    }

    #[test]
    fn test_texture_swizzle_size() {
        // TextureSwizzle uses u8 in C++
        assert_eq!(std::mem::size_of::<TextureSwizzle>(), 1);
    }

    #[test]
    fn test_texture_swizzle_channels_size() {
        assert_eq!(std::mem::size_of::<TextureSwizzleChannels>(), 4);
    }

    #[test]
    fn test_texture_usage_bitor() {
        let usage = TextureUsage::SHADER_READ | TextureUsage::RENDER_TARGET;
        assert!(usage.contains(TextureUsage::SHADER_READ));
        assert!(usage.contains(TextureUsage::RENDER_TARGET));
    }
}
