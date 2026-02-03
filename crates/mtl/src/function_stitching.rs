//! Metal function stitching for shader linking.
//!
//! Corresponds to `Metal/MTLFunctionStitching.hpp`.
//!
//! Function stitching allows creating new functions at runtime by combining
//! existing function graphs.

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_foundation::{Referencing, UInteger};
use mtl_sys::{msg_send_0, msg_send_1, sel};

// ============================================================================
// StitchedLibraryOptions
// ============================================================================

/// Options for stitched library creation.
///
/// C++ equivalent: `MTL::StitchedLibraryOptions`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StitchedLibraryOptions(pub UInteger);

impl StitchedLibraryOptions {
    pub const NONE: Self = Self(0);
    pub const FAIL_ON_BINARY_ARCHIVE_MISS: Self = Self(1);
    pub const STORE_LIBRARY_IN_METAL_PIPELINES_SCRIPT: Self = Self(1 << 1);
}

impl std::ops::BitOr for StitchedLibraryOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for StitchedLibraryOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for StitchedLibraryOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

// ============================================================================
// FunctionStitchingAttribute
// ============================================================================

/// Base type for function stitching attributes.
///
/// C++ equivalent: `MTL::FunctionStitchingAttribute`
#[repr(transparent)]
pub struct FunctionStitchingAttribute(pub(crate) NonNull<c_void>);

impl FunctionStitchingAttribute {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal FunctionStitchingAttribute.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

impl Clone for FunctionStitchingAttribute {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for FunctionStitchingAttribute {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionStitchingAttribute {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionStitchingAttribute {}
unsafe impl Sync for FunctionStitchingAttribute {}

// ============================================================================
// FunctionStitchingAttributeAlwaysInline
// ============================================================================

/// Attribute that marks a function for inlining.
///
/// C++ equivalent: `MTL::FunctionStitchingAttributeAlwaysInline`
#[repr(transparent)]
pub struct FunctionStitchingAttributeAlwaysInline(pub(crate) NonNull<c_void>);

impl FunctionStitchingAttributeAlwaysInline {
    /// Create a new always inline attribute.
    ///
    /// C++ equivalent: `FunctionStitchingAttributeAlwaysInline* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLFunctionStitchingAttributeAlwaysInline")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal FunctionStitchingAttributeAlwaysInline.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

impl Clone for FunctionStitchingAttributeAlwaysInline {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for FunctionStitchingAttributeAlwaysInline {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionStitchingAttributeAlwaysInline {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionStitchingAttributeAlwaysInline {}
unsafe impl Sync for FunctionStitchingAttributeAlwaysInline {}

// ============================================================================
// FunctionStitchingNode
// ============================================================================

/// Base type for function stitching nodes.
///
/// C++ equivalent: `MTL::FunctionStitchingNode`
#[repr(transparent)]
pub struct FunctionStitchingNode(pub(crate) NonNull<c_void>);

impl FunctionStitchingNode {
    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal FunctionStitchingNode.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

impl Clone for FunctionStitchingNode {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy should succeed")
        }
    }
}

impl Drop for FunctionStitchingNode {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionStitchingNode {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionStitchingNode {}
unsafe impl Sync for FunctionStitchingNode {}

// ============================================================================
// FunctionStitchingInputNode
// ============================================================================

/// An input node in a function stitching graph.
///
/// C++ equivalent: `MTL::FunctionStitchingInputNode`
#[repr(transparent)]
pub struct FunctionStitchingInputNode(pub(crate) NonNull<c_void>);

impl FunctionStitchingInputNode {
    /// Create a new input node.
    ///
    /// C++ equivalent: `FunctionStitchingInputNode* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLFunctionStitchingInputNode")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create a new input node with an argument index.
    ///
    /// C++ equivalent: `FunctionStitchingInputNode* alloc()->init(NS::UInteger)`
    pub fn with_argument_index(argument_index: UInteger) -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLFunctionStitchingInputNode")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_1(ptr, sel!(initWithArgumentIndex:), argument_index);
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal FunctionStitchingInputNode.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the argument index.
    ///
    /// C++ equivalent: `NS::UInteger argumentIndex() const`
    #[inline]
    pub fn argument_index(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(argumentIndex)) }
    }

    /// Set the argument index.
    ///
    /// C++ equivalent: `void setArgumentIndex(NS::UInteger)`
    pub fn set_argument_index(&self, argument_index: UInteger) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setArgumentIndex:), argument_index);
        }
    }
}

impl Clone for FunctionStitchingInputNode {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for FunctionStitchingInputNode {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionStitchingInputNode {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionStitchingInputNode {}
unsafe impl Sync for FunctionStitchingInputNode {}

// ============================================================================
// FunctionStitchingFunctionNode
// ============================================================================

/// A function node in a function stitching graph.
///
/// C++ equivalent: `MTL::FunctionStitchingFunctionNode`
#[repr(transparent)]
pub struct FunctionStitchingFunctionNode(pub(crate) NonNull<c_void>);

impl FunctionStitchingFunctionNode {
    /// Create a new function node.
    ///
    /// C++ equivalent: `FunctionStitchingFunctionNode* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLFunctionStitchingFunctionNode")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal FunctionStitchingFunctionNode.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the name of the function.
    ///
    /// C++ equivalent: `NS::String* name() const`
    pub fn name(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(name));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Set the name of the function.
    ///
    /// C++ equivalent: `void setName(const NS::String*)`
    pub fn set_name(&self, name: &str) {
        if let Some(ns_name) = mtl_foundation::String::from_str(name) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setName:), ns_name.as_ptr());
            }
        }
    }

    /// Get the arguments as a raw NS::Array pointer.
    ///
    /// C++ equivalent: `NS::Array* arguments() const`
    #[inline]
    pub fn arguments_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(arguments)) }
    }

    /// Set the arguments.
    ///
    /// C++ equivalent: `void setArguments(const NS::Array*)`
    pub fn set_arguments_ptr(&self, arguments: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setArguments:), arguments);
        }
    }

    /// Get the control dependencies as a raw NS::Array pointer.
    ///
    /// C++ equivalent: `NS::Array* controlDependencies() const`
    #[inline]
    pub fn control_dependencies_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(controlDependencies)) }
    }

    /// Set the control dependencies.
    ///
    /// C++ equivalent: `void setControlDependencies(const NS::Array*)`
    pub fn set_control_dependencies_ptr(&self, control_dependencies: *const c_void) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setControlDependencies:),
                control_dependencies,
            );
        }
    }
}

impl Clone for FunctionStitchingFunctionNode {
    fn clone(&self) -> Self {
        unsafe {
            msg_send_0::<*mut c_void>(self.as_ptr(), sel!(retain));
        }
        Self(self.0)
    }
}

impl Drop for FunctionStitchingFunctionNode {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionStitchingFunctionNode {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionStitchingFunctionNode {}
unsafe impl Sync for FunctionStitchingFunctionNode {}

// ============================================================================
// FunctionStitchingGraph
// ============================================================================

/// A graph defining how functions are stitched together.
///
/// C++ equivalent: `MTL::FunctionStitchingGraph`
#[repr(transparent)]
pub struct FunctionStitchingGraph(pub(crate) NonNull<c_void>);

impl FunctionStitchingGraph {
    /// Create a new stitching graph.
    ///
    /// C++ equivalent: `FunctionStitchingGraph* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLFunctionStitchingGraph")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal FunctionStitchingGraph.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the function name.
    ///
    /// C++ equivalent: `NS::String* functionName() const`
    pub fn function_name(&self) -> Option<String> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(functionName));
            if ptr.is_null() {
                return None;
            }
            let utf8_ptr: *const std::ffi::c_char =
                mtl_sys::msg_send_0(ptr as *const c_void, sel!(UTF8String));
            if utf8_ptr.is_null() {
                return None;
            }
            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            Some(c_str.to_string_lossy().into_owned())
        }
    }

    /// Set the function name.
    ///
    /// C++ equivalent: `void setFunctionName(const NS::String*)`
    pub fn set_function_name(&self, name: &str) {
        if let Some(ns_name) = mtl_foundation::String::from_str(name) {
            unsafe {
                let _: () = msg_send_1(self.as_ptr(), sel!(setFunctionName:), ns_name.as_ptr());
            }
        }
    }

    /// Get the nodes as a raw NS::Array pointer.
    ///
    /// C++ equivalent: `NS::Array* nodes() const`
    #[inline]
    pub fn nodes_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(nodes)) }
    }

    /// Set the nodes.
    ///
    /// C++ equivalent: `void setNodes(const NS::Array*)`
    pub fn set_nodes_ptr(&self, nodes: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setNodes:), nodes);
        }
    }

    /// Get the output node.
    ///
    /// C++ equivalent: `FunctionStitchingFunctionNode* outputNode() const`
    pub fn output_node(&self) -> Option<FunctionStitchingFunctionNode> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(outputNode));
            if ptr.is_null() {
                return None;
            }
            // Retain for our reference
            msg_send_0::<*mut c_void>(ptr as *const c_void, sel!(retain));
            FunctionStitchingFunctionNode::from_raw(ptr)
        }
    }

    /// Set the output node.
    ///
    /// C++ equivalent: `void setOutputNode(const MTL::FunctionStitchingFunctionNode*)`
    pub fn set_output_node(&self, output_node: &FunctionStitchingFunctionNode) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOutputNode:), output_node.as_ptr());
        }
    }

    /// Get the attributes as a raw NS::Array pointer.
    ///
    /// C++ equivalent: `NS::Array* attributes() const`
    #[inline]
    pub fn attributes_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(attributes)) }
    }

    /// Set the attributes.
    ///
    /// C++ equivalent: `void setAttributes(const NS::Array*)`
    pub fn set_attributes_ptr(&self, attributes: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setAttributes:), attributes);
        }
    }
}

impl Clone for FunctionStitchingGraph {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy should succeed")
        }
    }
}

impl Drop for FunctionStitchingGraph {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for FunctionStitchingGraph {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for FunctionStitchingGraph {}
unsafe impl Sync for FunctionStitchingGraph {}

// ============================================================================
// StitchedLibraryDescriptor
// ============================================================================

/// Descriptor for creating a stitched library.
///
/// C++ equivalent: `MTL::StitchedLibraryDescriptor`
#[repr(transparent)]
pub struct StitchedLibraryDescriptor(pub(crate) NonNull<c_void>);

impl StitchedLibraryDescriptor {
    /// Create a new stitched library descriptor.
    ///
    /// C++ equivalent: `StitchedLibraryDescriptor* alloc()->init()`
    pub fn new() -> Option<Self> {
        unsafe {
            let class = mtl_sys::Class::get("MTLStitchedLibraryDescriptor")?;
            let ptr: *mut c_void = msg_send_0(class.as_ptr(), sel!(alloc));
            if ptr.is_null() {
                return None;
            }
            let ptr: *mut c_void = msg_send_0(ptr, sel!(init));
            Self::from_raw(ptr)
        }
    }

    /// Create from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Metal StitchedLibraryDescriptor.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Get the raw pointer.
    #[inline]
    pub fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Get the function graphs as a raw NS::Array pointer.
    ///
    /// C++ equivalent: `NS::Array* functionGraphs() const`
    #[inline]
    pub fn function_graphs_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(functionGraphs)) }
    }

    /// Set the function graphs.
    ///
    /// C++ equivalent: `void setFunctionGraphs(const NS::Array*)`
    pub fn set_function_graphs_ptr(&self, function_graphs: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFunctionGraphs:), function_graphs);
        }
    }

    /// Get the functions as a raw NS::Array pointer.
    ///
    /// C++ equivalent: `NS::Array* functions() const`
    #[inline]
    pub fn functions_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(functions)) }
    }

    /// Set the functions.
    ///
    /// C++ equivalent: `void setFunctions(const NS::Array*)`
    pub fn set_functions_ptr(&self, functions: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setFunctions:), functions);
        }
    }

    /// Get the binary archives as a raw NS::Array pointer.
    ///
    /// C++ equivalent: `NS::Array* binaryArchives() const`
    #[inline]
    pub fn binary_archives_ptr(&self) -> *const c_void {
        unsafe { msg_send_0(self.as_ptr(), sel!(binaryArchives)) }
    }

    /// Set the binary archives.
    ///
    /// C++ equivalent: `void setBinaryArchives(const NS::Array*)`
    pub fn set_binary_archives_ptr(&self, binary_archives: *const c_void) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setBinaryArchives:), binary_archives);
        }
    }

    /// Get the options.
    ///
    /// C++ equivalent: `StitchedLibraryOptions options() const`
    #[inline]
    pub fn options(&self) -> StitchedLibraryOptions {
        unsafe { msg_send_0(self.as_ptr(), sel!(options)) }
    }

    /// Set the options.
    ///
    /// C++ equivalent: `void setOptions(MTL::StitchedLibraryOptions)`
    pub fn set_options(&self, options: StitchedLibraryOptions) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setOptions:), options);
        }
    }
}

impl Default for StitchedLibraryDescriptor {
    fn default() -> Self {
        Self::new().expect("failed to create StitchedLibraryDescriptor")
    }
}

impl Clone for StitchedLibraryDescriptor {
    fn clone(&self) -> Self {
        unsafe {
            let ptr: *mut c_void = msg_send_0(self.as_ptr(), sel!(copy));
            Self::from_raw(ptr).expect("copy should succeed")
        }
    }
}

impl Drop for StitchedLibraryDescriptor {
    fn drop(&mut self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(release));
        }
    }
}

impl Referencing for StitchedLibraryDescriptor {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for StitchedLibraryDescriptor {}
unsafe impl Sync for StitchedLibraryDescriptor {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stitched_library_options_values() {
        assert_eq!(StitchedLibraryOptions::NONE.0, 0);
        assert_eq!(StitchedLibraryOptions::FAIL_ON_BINARY_ARCHIVE_MISS.0, 1);
        assert_eq!(
            StitchedLibraryOptions::STORE_LIBRARY_IN_METAL_PIPELINES_SCRIPT.0,
            2
        );
    }

    #[test]
    fn test_stitched_library_options_bitor() {
        let options = StitchedLibraryOptions::FAIL_ON_BINARY_ARCHIVE_MISS
            | StitchedLibraryOptions::STORE_LIBRARY_IN_METAL_PIPELINES_SCRIPT;
        assert_eq!(options.0, 3);
    }

    #[test]
    fn test_function_stitching_attribute_size() {
        assert_eq!(
            std::mem::size_of::<FunctionStitchingAttribute>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_function_stitching_input_node_creation() {
        let node = FunctionStitchingInputNode::new();
        assert!(node.is_some());
    }

    #[test]
    fn test_function_stitching_input_node_with_index() {
        let node = FunctionStitchingInputNode::with_argument_index(5);
        assert!(node.is_some());
        if let Some(n) = node {
            assert_eq!(n.argument_index(), 5);
        }
    }

    #[test]
    fn test_function_stitching_function_node_creation() {
        let node = FunctionStitchingFunctionNode::new();
        assert!(node.is_some());
    }

    #[test]
    fn test_function_stitching_graph_creation() {
        let graph = FunctionStitchingGraph::new();
        assert!(graph.is_some());
    }

    #[test]
    fn test_stitched_library_descriptor_creation() {
        let descriptor = StitchedLibraryDescriptor::new();
        assert!(descriptor.is_some());
    }

    #[test]
    fn test_stitched_library_descriptor_options() {
        let descriptor = StitchedLibraryDescriptor::new().unwrap();
        descriptor.set_options(StitchedLibraryOptions::FAIL_ON_BINARY_ARCHIVE_MISS);
        assert_eq!(
            descriptor.options(),
            StitchedLibraryOptions::FAIL_ON_BINARY_ARCHIVE_MISS
        );
    }
}
