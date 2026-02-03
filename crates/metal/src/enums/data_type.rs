//! Data type enumeration.
//!
//! Corresponds to `Metal/MTLDataType.hpp`.

use metal_foundation::UInteger;

/// Shader data types.
///
/// C++ equivalent: `MTL::DataType`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct DataType(pub UInteger);

impl DataType {
    // Basic types
    pub const NONE: Self = Self(0);
    pub const STRUCT: Self = Self(1);
    pub const ARRAY: Self = Self(2);

    // Float types
    pub const FLOAT: Self = Self(3);
    pub const FLOAT2: Self = Self(4);
    pub const FLOAT3: Self = Self(5);
    pub const FLOAT4: Self = Self(6);

    // Float matrices
    pub const FLOAT2X2: Self = Self(7);
    pub const FLOAT2X3: Self = Self(8);
    pub const FLOAT2X4: Self = Self(9);
    pub const FLOAT3X2: Self = Self(10);
    pub const FLOAT3X3: Self = Self(11);
    pub const FLOAT3X4: Self = Self(12);
    pub const FLOAT4X2: Self = Self(13);
    pub const FLOAT4X3: Self = Self(14);
    pub const FLOAT4X4: Self = Self(15);

    // Half types
    pub const HALF: Self = Self(16);
    pub const HALF2: Self = Self(17);
    pub const HALF3: Self = Self(18);
    pub const HALF4: Self = Self(19);

    // Half matrices
    pub const HALF2X2: Self = Self(20);
    pub const HALF2X3: Self = Self(21);
    pub const HALF2X4: Self = Self(22);
    pub const HALF3X2: Self = Self(23);
    pub const HALF3X3: Self = Self(24);
    pub const HALF3X4: Self = Self(25);
    pub const HALF4X2: Self = Self(26);
    pub const HALF4X3: Self = Self(27);
    pub const HALF4X4: Self = Self(28);

    // Int types
    pub const INT: Self = Self(29);
    pub const INT2: Self = Self(30);
    pub const INT3: Self = Self(31);
    pub const INT4: Self = Self(32);

    // UInt types
    pub const UINT: Self = Self(33);
    pub const UINT2: Self = Self(34);
    pub const UINT3: Self = Self(35);
    pub const UINT4: Self = Self(36);

    // Short types
    pub const SHORT: Self = Self(37);
    pub const SHORT2: Self = Self(38);
    pub const SHORT3: Self = Self(39);
    pub const SHORT4: Self = Self(40);

    // UShort types
    pub const USHORT: Self = Self(41);
    pub const USHORT2: Self = Self(42);
    pub const USHORT3: Self = Self(43);
    pub const USHORT4: Self = Self(44);

    // Char types
    pub const CHAR: Self = Self(45);
    pub const CHAR2: Self = Self(46);
    pub const CHAR3: Self = Self(47);
    pub const CHAR4: Self = Self(48);

    // UChar types
    pub const UCHAR: Self = Self(49);
    pub const UCHAR2: Self = Self(50);
    pub const UCHAR3: Self = Self(51);
    pub const UCHAR4: Self = Self(52);

    // Bool types
    pub const BOOL: Self = Self(53);
    pub const BOOL2: Self = Self(54);
    pub const BOOL3: Self = Self(55);
    pub const BOOL4: Self = Self(56);

    // Complex types
    pub const TEXTURE: Self = Self(58);
    pub const SAMPLER: Self = Self(59);
    pub const POINTER: Self = Self(60);

    // Normalized types
    pub const R8_UNORM: Self = Self(62);
    pub const R8_SNORM: Self = Self(63);
    pub const R16_UNORM: Self = Self(64);
    pub const R16_SNORM: Self = Self(65);
    pub const RG8_UNORM: Self = Self(66);
    pub const RG8_SNORM: Self = Self(67);
    pub const RG16_UNORM: Self = Self(68);
    pub const RG16_SNORM: Self = Self(69);
    pub const RGBA8_UNORM: Self = Self(70);
    pub const RGBA8_UNORM_SRGB: Self = Self(71);
    pub const RGBA8_SNORM: Self = Self(72);
    pub const RGBA16_UNORM: Self = Self(73);
    pub const RGBA16_SNORM: Self = Self(74);
    pub const RGB10A2_UNORM: Self = Self(75);
    pub const RG11B10_FLOAT: Self = Self(76);
    pub const RGB9E5_FLOAT: Self = Self(77);

    // Pipeline types
    pub const RENDER_PIPELINE: Self = Self(78);
    pub const COMPUTE_PIPELINE: Self = Self(79);
    pub const INDIRECT_COMMAND_BUFFER: Self = Self(80);

    // Long types
    pub const LONG: Self = Self(81);
    pub const LONG2: Self = Self(82);
    pub const LONG3: Self = Self(83);
    pub const LONG4: Self = Self(84);

    // ULong types
    pub const ULONG: Self = Self(85);
    pub const ULONG2: Self = Self(86);
    pub const ULONG3: Self = Self(87);
    pub const ULONG4: Self = Self(88);

    // Function table types
    pub const VISIBLE_FUNCTION_TABLE: Self = Self(115);
    pub const INTERSECTION_FUNCTION_TABLE: Self = Self(116);
    pub const PRIMITIVE_ACCELERATION_STRUCTURE: Self = Self(117);
    pub const INSTANCE_ACCELERATION_STRUCTURE: Self = Self(118);

    // BFloat types
    pub const BFLOAT: Self = Self(121);
    pub const BFLOAT2: Self = Self(122);
    pub const BFLOAT3: Self = Self(123);
    pub const BFLOAT4: Self = Self(124);

    // Advanced types
    pub const DEPTH_STENCIL_STATE: Self = Self(139);
    pub const TENSOR: Self = Self(140);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_type_values() {
        assert_eq!(DataType::NONE.0, 0);
        assert_eq!(DataType::FLOAT.0, 3);
        assert_eq!(DataType::FLOAT4.0, 6);
        assert_eq!(DataType::TEXTURE.0, 58);
        assert_eq!(DataType::TENSOR.0, 140);
    }

    #[test]
    fn test_data_type_size() {
        assert_eq!(
            std::mem::size_of::<DataType>(),
            std::mem::size_of::<UInteger>()
        );
    }
}
