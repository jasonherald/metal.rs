//! Vertex descriptor enumerations.
//!
//! Corresponds to `Metal/MTLVertexDescriptor.hpp` and `Metal/MTLStageInputOutputDescriptor.hpp`.

use mtl_foundation::UInteger;

/// Vertex format for vertex attributes.
///
/// C++ equivalent: `MTL::VertexFormat`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VertexFormat(pub UInteger);

impl VertexFormat {
    pub const INVALID: Self = Self(0);
    pub const UCHAR2: Self = Self(1);
    pub const UCHAR3: Self = Self(2);
    pub const UCHAR4: Self = Self(3);
    pub const CHAR2: Self = Self(4);
    pub const CHAR3: Self = Self(5);
    pub const CHAR4: Self = Self(6);
    pub const UCHAR2_NORMALIZED: Self = Self(7);
    pub const UCHAR3_NORMALIZED: Self = Self(8);
    pub const UCHAR4_NORMALIZED: Self = Self(9);
    pub const CHAR2_NORMALIZED: Self = Self(10);
    pub const CHAR3_NORMALIZED: Self = Self(11);
    pub const CHAR4_NORMALIZED: Self = Self(12);
    pub const USHORT2: Self = Self(13);
    pub const USHORT3: Self = Self(14);
    pub const USHORT4: Self = Self(15);
    pub const SHORT2: Self = Self(16);
    pub const SHORT3: Self = Self(17);
    pub const SHORT4: Self = Self(18);
    pub const USHORT2_NORMALIZED: Self = Self(19);
    pub const USHORT3_NORMALIZED: Self = Self(20);
    pub const USHORT4_NORMALIZED: Self = Self(21);
    pub const SHORT2_NORMALIZED: Self = Self(22);
    pub const SHORT3_NORMALIZED: Self = Self(23);
    pub const SHORT4_NORMALIZED: Self = Self(24);
    pub const HALF2: Self = Self(25);
    pub const HALF3: Self = Self(26);
    pub const HALF4: Self = Self(27);
    pub const FLOAT: Self = Self(28);
    pub const FLOAT2: Self = Self(29);
    pub const FLOAT3: Self = Self(30);
    pub const FLOAT4: Self = Self(31);
    pub const INT: Self = Self(32);
    pub const INT2: Self = Self(33);
    pub const INT3: Self = Self(34);
    pub const INT4: Self = Self(35);
    pub const UINT: Self = Self(36);
    pub const UINT2: Self = Self(37);
    pub const UINT3: Self = Self(38);
    pub const UINT4: Self = Self(39);
    pub const INT1010102_NORMALIZED: Self = Self(40);
    pub const UINT1010102_NORMALIZED: Self = Self(41);
    pub const UCHAR4_NORMALIZED_BGRA: Self = Self(42);
    pub const UCHAR: Self = Self(45);
    pub const CHAR: Self = Self(46);
    pub const UCHAR_NORMALIZED: Self = Self(47);
    pub const CHAR_NORMALIZED: Self = Self(48);
    pub const USHORT: Self = Self(49);
    pub const SHORT: Self = Self(50);
    pub const USHORT_NORMALIZED: Self = Self(51);
    pub const SHORT_NORMALIZED: Self = Self(52);
    pub const HALF: Self = Self(53);
    pub const FLOAT_RG11B10: Self = Self(54);
    pub const FLOAT_RGB9E5: Self = Self(55);
}

/// Vertex step function for vertex buffer layouts.
///
/// C++ equivalent: `MTL::VertexStepFunction`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct VertexStepFunction(pub UInteger);

impl VertexStepFunction {
    pub const CONSTANT: Self = Self(0);
    pub const PER_VERTEX: Self = Self(1);
    pub const PER_INSTANCE: Self = Self(2);
    pub const PER_PATCH: Self = Self(3);
    pub const PER_PATCH_CONTROL_POINT: Self = Self(4);
}

/// Attribute format for stage input/output attributes.
///
/// C++ equivalent: `MTL::AttributeFormat`
///
/// Note: Shares the same values as VertexFormat.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AttributeFormat(pub UInteger);

impl AttributeFormat {
    pub const INVALID: Self = Self(0);
    pub const UCHAR2: Self = Self(1);
    pub const UCHAR3: Self = Self(2);
    pub const UCHAR4: Self = Self(3);
    pub const CHAR2: Self = Self(4);
    pub const CHAR3: Self = Self(5);
    pub const CHAR4: Self = Self(6);
    pub const UCHAR2_NORMALIZED: Self = Self(7);
    pub const UCHAR3_NORMALIZED: Self = Self(8);
    pub const UCHAR4_NORMALIZED: Self = Self(9);
    pub const CHAR2_NORMALIZED: Self = Self(10);
    pub const CHAR3_NORMALIZED: Self = Self(11);
    pub const CHAR4_NORMALIZED: Self = Self(12);
    pub const USHORT2: Self = Self(13);
    pub const USHORT3: Self = Self(14);
    pub const USHORT4: Self = Self(15);
    pub const SHORT2: Self = Self(16);
    pub const SHORT3: Self = Self(17);
    pub const SHORT4: Self = Self(18);
    pub const USHORT2_NORMALIZED: Self = Self(19);
    pub const USHORT3_NORMALIZED: Self = Self(20);
    pub const USHORT4_NORMALIZED: Self = Self(21);
    pub const SHORT2_NORMALIZED: Self = Self(22);
    pub const SHORT3_NORMALIZED: Self = Self(23);
    pub const SHORT4_NORMALIZED: Self = Self(24);
    pub const HALF2: Self = Self(25);
    pub const HALF3: Self = Self(26);
    pub const HALF4: Self = Self(27);
    pub const FLOAT: Self = Self(28);
    pub const FLOAT2: Self = Self(29);
    pub const FLOAT3: Self = Self(30);
    pub const FLOAT4: Self = Self(31);
    pub const INT: Self = Self(32);
    pub const INT2: Self = Self(33);
    pub const INT3: Self = Self(34);
    pub const INT4: Self = Self(35);
    pub const UINT: Self = Self(36);
    pub const UINT2: Self = Self(37);
    pub const UINT3: Self = Self(38);
    pub const UINT4: Self = Self(39);
    pub const INT1010102_NORMALIZED: Self = Self(40);
    pub const UINT1010102_NORMALIZED: Self = Self(41);
    pub const UCHAR4_NORMALIZED_BGRA: Self = Self(42);
    pub const UCHAR: Self = Self(45);
    pub const CHAR: Self = Self(46);
    pub const UCHAR_NORMALIZED: Self = Self(47);
    pub const CHAR_NORMALIZED: Self = Self(48);
    pub const USHORT: Self = Self(49);
    pub const SHORT: Self = Self(50);
    pub const USHORT_NORMALIZED: Self = Self(51);
    pub const SHORT_NORMALIZED: Self = Self(52);
    pub const HALF: Self = Self(53);
    pub const FLOAT_RG11B10: Self = Self(54);
    pub const FLOAT_RGB9E5: Self = Self(55);
}

/// Step function for buffer layouts.
///
/// C++ equivalent: `MTL::StepFunction`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StepFunction(pub UInteger);

impl StepFunction {
    pub const CONSTANT: Self = Self(0);
    pub const PER_VERTEX: Self = Self(1);
    pub const PER_INSTANCE: Self = Self(2);
    pub const PER_PATCH: Self = Self(3);
    pub const PER_PATCH_CONTROL_POINT: Self = Self(4);
    pub const THREAD_POSITION_IN_GRID_X: Self = Self(5);
    pub const THREAD_POSITION_IN_GRID_Y: Self = Self(6);
    pub const THREAD_POSITION_IN_GRID_X_INDEXED: Self = Self(7);
    pub const THREAD_POSITION_IN_GRID_Y_INDEXED: Self = Self(8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_format_values() {
        assert_eq!(VertexFormat::INVALID.0, 0);
        assert_eq!(VertexFormat::FLOAT.0, 28);
        assert_eq!(VertexFormat::FLOAT4.0, 31);
        assert_eq!(VertexFormat::FLOAT_RGB9E5.0, 55);
    }

    #[test]
    fn test_vertex_step_function_values() {
        assert_eq!(VertexStepFunction::CONSTANT.0, 0);
        assert_eq!(VertexStepFunction::PER_VERTEX.0, 1);
        assert_eq!(VertexStepFunction::PER_PATCH_CONTROL_POINT.0, 4);
    }

    #[test]
    fn test_step_function_values() {
        assert_eq!(StepFunction::CONSTANT.0, 0);
        assert_eq!(StepFunction::THREAD_POSITION_IN_GRID_Y_INDEXED.0, 8);
    }
}
