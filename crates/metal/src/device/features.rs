//! Device feature queries.
//!
//! Corresponds to feature query methods in `MTL::Device` from `Metal/MTLDevice.hpp`.

use metal_foundation::{Referencing, UInteger};
use metal_sys::{msg_send_0, msg_send_1, sel};

use super::Device;
use crate::enums::{CounterSamplingPoint, FeatureSet, GPUFamily};

impl Device {
    // =========================================================================
    // GPU Family Support
    // =========================================================================

    /// Check if the device supports a specific GPU family.
    ///
    /// C++ equivalent: `bool supportsFamily(MTL::GPUFamily gpuFamily)`
    #[inline]
    pub fn supports_family(&self, gpu_family: GPUFamily) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(supportsFamily:), gpu_family) }
    }

    /// Check if the device supports a specific feature set (legacy).
    ///
    /// C++ equivalent: `bool supportsFeatureSet(MTL::FeatureSet featureSet)`
    #[inline]
    pub fn supports_feature_set(&self, feature_set: FeatureSet) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(supportsFeatureSet:), feature_set) }
    }

    // =========================================================================
    // Rendering Features
    // =========================================================================

    /// Check if barycentric coordinates are supported.
    ///
    /// C++ equivalent: `bool areBarycentricCoordsSupported() const`
    #[inline]
    pub fn are_barycentric_coords_supported(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(areBarycentricCoordsSupported)) }
    }

    /// Check if programmable sample positions are supported.
    ///
    /// C++ equivalent: `bool areProgrammableSamplePositionsSupported() const`
    #[inline]
    pub fn are_programmable_sample_positions_supported(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(areProgrammableSamplePositionsSupported)) }
    }

    /// Check if raster order groups are supported.
    ///
    /// C++ equivalent: `bool areRasterOrderGroupsSupported() const`
    #[inline]
    pub fn are_raster_order_groups_supported(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(areRasterOrderGroupsSupported)) }
    }

    /// Check if 32-bit float filtering is supported.
    ///
    /// C++ equivalent: `bool supports32BitFloatFiltering() const`
    #[inline]
    pub fn supports_32bit_float_filtering(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supports32BitFloatFiltering)) }
    }

    /// Check if 32-bit MSAA is supported.
    ///
    /// C++ equivalent: `bool supports32BitMSAA() const`
    #[inline]
    pub fn supports_32bit_msaa(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supports32BitMSAA)) }
    }

    /// Check if the device supports a specific texture sample count.
    ///
    /// C++ equivalent: `bool supportsTextureSampleCount(NS::UInteger sampleCount)`
    #[inline]
    pub fn supports_texture_sample_count(&self, sample_count: UInteger) -> bool {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(supportsTextureSampleCount:),
                sample_count,
            )
        }
    }

    /// Check if vertex amplification is supported for a given count.
    ///
    /// C++ equivalent: `bool supportsVertexAmplificationCount(NS::UInteger count)`
    #[inline]
    pub fn supports_vertex_amplification_count(&self, count: UInteger) -> bool {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(supportsVertexAmplificationCount:),
                count,
            )
        }
    }

    /// Check if query texture LOD is supported.
    ///
    /// C++ equivalent: `bool supportsQueryTextureLOD() const`
    #[inline]
    pub fn supports_query_texture_lod(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportsQueryTextureLOD)) }
    }

    /// Check if pull model interpolation is supported.
    ///
    /// C++ equivalent: `bool supportsPullModelInterpolation() const`
    #[inline]
    pub fn supports_pull_model_interpolation(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportsPullModelInterpolation)) }
    }

    /// Check if shader barycentric coordinates are supported.
    ///
    /// C++ equivalent: `bool supportsShaderBarycentricCoordinates() const`
    #[inline]
    pub fn supports_shader_barycentric_coordinates(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportsShaderBarycentricCoordinates)) }
    }

    // =========================================================================
    // Texture Features
    // =========================================================================

    /// Check if BC texture compression is supported.
    ///
    /// C++ equivalent: `bool supportsBCTextureCompression() const`
    #[inline]
    pub fn supports_bc_texture_compression(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportsBCTextureCompression)) }
    }

    /// Check if depth24 stencil8 pixel format is supported.
    ///
    /// C++ equivalent: `bool isDepth24Stencil8PixelFormatSupported() const`
    #[inline]
    pub fn is_depth24_stencil8_pixel_format_supported(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isDepth24Stencil8PixelFormatSupported)) }
    }

    // =========================================================================
    // Ray Tracing Features
    // =========================================================================

    /// Check if ray tracing is supported.
    ///
    /// C++ equivalent: `bool supportsRaytracing() const`
    #[inline]
    pub fn supports_raytracing(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportsRaytracing)) }
    }

    /// Check if ray tracing from render is supported.
    ///
    /// C++ equivalent: `bool supportsRaytracingFromRender() const`
    #[inline]
    pub fn supports_raytracing_from_render(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportsRaytracingFromRender)) }
    }

    /// Check if primitive motion blur is supported.
    ///
    /// C++ equivalent: `bool supportsPrimitiveMotionBlur() const`
    #[inline]
    pub fn supports_primitive_motion_blur(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportsPrimitiveMotionBlur)) }
    }

    // =========================================================================
    // Function Features
    // =========================================================================

    /// Check if function pointers are supported.
    ///
    /// C++ equivalent: `bool supportsFunctionPointers() const`
    #[inline]
    pub fn supports_function_pointers(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportsFunctionPointers)) }
    }

    /// Check if function pointers from render are supported.
    ///
    /// C++ equivalent: `bool supportsFunctionPointersFromRender() const`
    #[inline]
    pub fn supports_function_pointers_from_render(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportsFunctionPointersFromRender)) }
    }

    /// Check if dynamic libraries are supported.
    ///
    /// C++ equivalent: `bool supportsDynamicLibraries() const`
    #[inline]
    pub fn supports_dynamic_libraries(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportsDynamicLibraries)) }
    }

    /// Check if render dynamic libraries are supported.
    ///
    /// C++ equivalent: `bool supportsRenderDynamicLibraries() const`
    #[inline]
    pub fn supports_render_dynamic_libraries(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(supportsRenderDynamicLibraries)) }
    }

    // =========================================================================
    // Variable Rate Rendering Features
    // =========================================================================

    /// Check if rasterization rate maps are supported for a given layer count.
    ///
    /// C++ equivalent: `bool supportsRasterizationRateMap(NS::UInteger layerCount)`
    #[inline]
    pub fn supports_rasterization_rate_map(&self, layer_count: UInteger) -> bool {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(supportsRasterizationRateMapWithLayerCount:),
                layer_count,
            )
        }
    }

    // =========================================================================
    // Counter Sampling Features
    // =========================================================================

    /// Check if counter sampling is supported at a specific sampling point.
    ///
    /// C++ equivalent: `bool supportsCounterSampling(MTL::CounterSamplingPoint samplingPoint)`
    #[inline]
    pub fn supports_counter_sampling(&self, sampling_point: CounterSamplingPoint) -> bool {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(supportsCounterSampling:),
                sampling_point,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::system_default;

    #[test]
    fn test_feature_queries() {
        let device = system_default().expect("no Metal device");

        // Metal 3 should be supported on modern Apple Silicon
        let supports_metal3 = device.supports_family(GPUFamily::METAL3);
        println!("Supports Metal 3: {}", supports_metal3);

        // Check raytracing support
        let supports_raytracing = device.supports_raytracing();
        println!("Supports raytracing: {}", supports_raytracing);

        // Check unified memory
        let unified = device.has_unified_memory();
        println!("Has unified memory: {}", unified);
    }

    #[test]
    fn test_sample_count_support() {
        let device = system_default().expect("no Metal device");

        // Most devices support 1, 2, 4, and 8 samples
        assert!(device.supports_texture_sample_count(1));
        assert!(device.supports_texture_sample_count(4));
    }
}
