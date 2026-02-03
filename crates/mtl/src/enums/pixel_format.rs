//! Pixel format enumeration.
//!
//! Corresponds to `Metal/MTLPixelFormat.hpp`.

use mtl_foundation::UInteger;

/// Pixel formats for textures and render targets.
///
/// C++ equivalent: `MTL::PixelFormat`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PixelFormat(pub UInteger);

impl PixelFormat {
    // Invalid format
    pub const INVALID: Self = Self(0);

    // Single channel - Alpha
    pub const A8_UNORM: Self = Self(1);

    // Single channel - Red (8-bit)
    pub const R8_UNORM: Self = Self(10);
    pub const R8_UNORM_SRGB: Self = Self(11);
    pub const R8_SNORM: Self = Self(12);
    pub const R8_UINT: Self = Self(13);
    pub const R8_SINT: Self = Self(14);

    // Single channel - Red (16-bit)
    pub const R16_UNORM: Self = Self(20);
    pub const R16_SNORM: Self = Self(22);
    pub const R16_UINT: Self = Self(23);
    pub const R16_SINT: Self = Self(24);
    pub const R16_FLOAT: Self = Self(25);

    // Two channel - RG (8-bit)
    pub const RG8_UNORM: Self = Self(30);
    pub const RG8_UNORM_SRGB: Self = Self(31);
    pub const RG8_SNORM: Self = Self(32);
    pub const RG8_UINT: Self = Self(33);
    pub const RG8_SINT: Self = Self(34);

    // Packed formats (16-bit)
    pub const B5G6R5_UNORM: Self = Self(40);
    pub const A1BGR5_UNORM: Self = Self(41);
    pub const ABGR4_UNORM: Self = Self(42);
    pub const BGR5A1_UNORM: Self = Self(43);

    // Single channel - Red (32-bit)
    pub const R32_UINT: Self = Self(53);
    pub const R32_SINT: Self = Self(54);
    pub const R32_FLOAT: Self = Self(55);

    // Two channel - RG (16-bit)
    pub const RG16_UNORM: Self = Self(60);
    pub const RG16_SNORM: Self = Self(62);
    pub const RG16_UINT: Self = Self(63);
    pub const RG16_SINT: Self = Self(64);
    pub const RG16_FLOAT: Self = Self(65);

    // Four channel - RGBA (8-bit)
    pub const RGBA8_UNORM: Self = Self(70);
    pub const RGBA8_UNORM_SRGB: Self = Self(71);
    pub const RGBA8_SNORM: Self = Self(72);
    pub const RGBA8_UINT: Self = Self(73);
    pub const RGBA8_SINT: Self = Self(74);

    // Four channel - BGRA (8-bit)
    pub const BGRA8_UNORM: Self = Self(80);
    pub const BGRA8_UNORM_SRGB: Self = Self(81);

    // Packed formats (32-bit)
    pub const RGB10A2_UNORM: Self = Self(90);
    pub const RGB10A2_UINT: Self = Self(91);
    pub const RG11B10_FLOAT: Self = Self(92);
    pub const RGB9E5_FLOAT: Self = Self(93);
    pub const BGR10A2_UNORM: Self = Self(94);

    // Two channel - RG (32-bit)
    pub const RG32_UINT: Self = Self(103);
    pub const RG32_SINT: Self = Self(104);
    pub const RG32_FLOAT: Self = Self(105);

    // Four channel - RGBA (16-bit)
    pub const RGBA16_UNORM: Self = Self(110);
    pub const RGBA16_SNORM: Self = Self(112);
    pub const RGBA16_UINT: Self = Self(113);
    pub const RGBA16_SINT: Self = Self(114);
    pub const RGBA16_FLOAT: Self = Self(115);

    // Four channel - RGBA (32-bit)
    pub const RGBA32_UINT: Self = Self(123);
    pub const RGBA32_SINT: Self = Self(124);
    pub const RGBA32_FLOAT: Self = Self(125);

    // BC compression
    pub const BC1_RGBA: Self = Self(130);
    pub const BC1_RGBA_SRGB: Self = Self(131);
    pub const BC2_RGBA: Self = Self(132);
    pub const BC2_RGBA_SRGB: Self = Self(133);
    pub const BC3_RGBA: Self = Self(134);
    pub const BC3_RGBA_SRGB: Self = Self(135);
    pub const BC4_R_UNORM: Self = Self(140);
    pub const BC4_R_SNORM: Self = Self(141);
    pub const BC5_RG_UNORM: Self = Self(142);
    pub const BC5_RG_SNORM: Self = Self(143);
    pub const BC6H_RGB_FLOAT: Self = Self(150);
    pub const BC6H_RGB_UFLOAT: Self = Self(151);
    pub const BC7_RGBA_UNORM: Self = Self(152);
    pub const BC7_RGBA_UNORM_SRGB: Self = Self(153);

    // PVRTC compression
    pub const PVRTC_RGB_2BPP: Self = Self(160);
    pub const PVRTC_RGB_2BPP_SRGB: Self = Self(161);
    pub const PVRTC_RGB_4BPP: Self = Self(162);
    pub const PVRTC_RGB_4BPP_SRGB: Self = Self(163);
    pub const PVRTC_RGBA_2BPP: Self = Self(164);
    pub const PVRTC_RGBA_2BPP_SRGB: Self = Self(165);
    pub const PVRTC_RGBA_4BPP: Self = Self(166);
    pub const PVRTC_RGBA_4BPP_SRGB: Self = Self(167);

    // EAC compression
    pub const EAC_R11_UNORM: Self = Self(170);
    pub const EAC_R11_SNORM: Self = Self(172);
    pub const EAC_RG11_UNORM: Self = Self(174);
    pub const EAC_RG11_SNORM: Self = Self(176);
    pub const EAC_RGBA8: Self = Self(178);
    pub const EAC_RGBA8_SRGB: Self = Self(179);

    // ETC2 compression
    pub const ETC2_RGB8: Self = Self(180);
    pub const ETC2_RGB8_SRGB: Self = Self(181);
    pub const ETC2_RGB8A1: Self = Self(182);
    pub const ETC2_RGB8A1_SRGB: Self = Self(183);

    // ASTC compression - sRGB
    pub const ASTC_4X4_SRGB: Self = Self(186);
    pub const ASTC_5X4_SRGB: Self = Self(187);
    pub const ASTC_5X5_SRGB: Self = Self(188);
    pub const ASTC_6X5_SRGB: Self = Self(189);
    pub const ASTC_6X6_SRGB: Self = Self(190);
    pub const ASTC_8X5_SRGB: Self = Self(192);
    pub const ASTC_8X6_SRGB: Self = Self(193);
    pub const ASTC_8X8_SRGB: Self = Self(194);
    pub const ASTC_10X5_SRGB: Self = Self(195);
    pub const ASTC_10X6_SRGB: Self = Self(196);
    pub const ASTC_10X8_SRGB: Self = Self(197);
    pub const ASTC_10X10_SRGB: Self = Self(198);
    pub const ASTC_12X10_SRGB: Self = Self(199);
    pub const ASTC_12X12_SRGB: Self = Self(200);

    // ASTC compression - LDR
    pub const ASTC_4X4_LDR: Self = Self(204);
    pub const ASTC_5X4_LDR: Self = Self(205);
    pub const ASTC_5X5_LDR: Self = Self(206);
    pub const ASTC_6X5_LDR: Self = Self(207);
    pub const ASTC_6X6_LDR: Self = Self(208);
    pub const ASTC_8X5_LDR: Self = Self(210);
    pub const ASTC_8X6_LDR: Self = Self(211);
    pub const ASTC_8X8_LDR: Self = Self(212);
    pub const ASTC_10X5_LDR: Self = Self(213);
    pub const ASTC_10X6_LDR: Self = Self(214);
    pub const ASTC_10X8_LDR: Self = Self(215);
    pub const ASTC_10X10_LDR: Self = Self(216);
    pub const ASTC_12X10_LDR: Self = Self(217);
    pub const ASTC_12X12_LDR: Self = Self(218);

    // ASTC compression - HDR
    pub const ASTC_4X4_HDR: Self = Self(222);
    pub const ASTC_5X4_HDR: Self = Self(223);
    pub const ASTC_5X5_HDR: Self = Self(224);
    pub const ASTC_6X5_HDR: Self = Self(225);
    pub const ASTC_6X6_HDR: Self = Self(226);
    pub const ASTC_8X5_HDR: Self = Self(228);
    pub const ASTC_8X6_HDR: Self = Self(229);
    pub const ASTC_8X8_HDR: Self = Self(230);
    pub const ASTC_10X5_HDR: Self = Self(231);
    pub const ASTC_10X6_HDR: Self = Self(232);
    pub const ASTC_10X8_HDR: Self = Self(233);
    pub const ASTC_10X10_HDR: Self = Self(234);
    pub const ASTC_12X10_HDR: Self = Self(235);
    pub const ASTC_12X12_HDR: Self = Self(236);

    // YUV formats
    pub const GBGR422: Self = Self(240);
    pub const BGRG422: Self = Self(241);

    // Depth/Stencil formats
    pub const DEPTH16_UNORM: Self = Self(250);
    pub const DEPTH32_FLOAT: Self = Self(252);
    pub const STENCIL8: Self = Self(253);
    pub const DEPTH24_UNORM_STENCIL8: Self = Self(255);
    pub const DEPTH32_FLOAT_STENCIL8: Self = Self(260);
    pub const X32_STENCIL8: Self = Self(261);
    pub const X24_STENCIL8: Self = Self(262);

    // Extended range formats
    pub const BGRA10_XR: Self = Self(552);
    pub const BGRA10_XR_SRGB: Self = Self(553);
    pub const BGR10_XR: Self = Self(554);
    pub const BGR10_XR_SRGB: Self = Self(555);

    // Unspecialized
    pub const UNSPECIALIZED: Self = Self(263);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_format_values() {
        assert_eq!(PixelFormat::INVALID.0, 0);
        assert_eq!(PixelFormat::BGRA8_UNORM.0, 80);
        assert_eq!(PixelFormat::DEPTH32_FLOAT.0, 252);
        assert_eq!(PixelFormat::RGBA16_FLOAT.0, 115);
    }

    #[test]
    fn test_pixel_format_size() {
        assert_eq!(
            std::mem::size_of::<PixelFormat>(),
            std::mem::size_of::<UInteger>()
        );
    }
}
