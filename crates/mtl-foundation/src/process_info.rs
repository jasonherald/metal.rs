//! Process information for Foundation.
//!
//! Corresponds to `Foundation/NSProcessInfo.hpp`.
//!
//! # C++ Equivalent
//!
//! ```cpp
//! namespace NS {
//! _NS_ENUM(NS::Integer, ProcessInfoThermalState) {
//!     ProcessInfoThermalStateNominal = 0,
//!     ProcessInfoThermalStateFair = 1,
//!     ProcessInfoThermalStateSerious = 2,
//!     ProcessInfoThermalStateCritical = 3
//! };
//!
//! _NS_OPTIONS(std::uint64_t, ActivityOptions) { ... };
//!
//! class ProcessInfo : public Referencing<ProcessInfo> {
//! public:
//!     static ProcessInfo*     processInfo();
//!     class Array*            arguments() const;
//!     class Dictionary*       environment() const;
//!     // ... many more methods
//! };
//! }
//! ```

use std::ffi::c_void;
use std::ptr::NonNull;

use mtl_sys::{class, msg_send_0, msg_send_1, msg_send_2, sel};

use crate::array::Array;
use crate::dictionary::Dictionary;
use crate::object::{Object, Referencing};
use crate::string::String;
use crate::types::{Integer, OperatingSystemVersion, TimeInterval, UInteger};

/// Thermal state of the system.
///
/// C++ equivalent: `NS::ProcessInfoThermalState`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ProcessInfoThermalState(pub Integer);

impl ProcessInfoThermalState {
    /// Nominal thermal state.
    pub const NOMINAL: Self = Self(0);
    /// Fair thermal state.
    pub const FAIR: Self = Self(1);
    /// Serious thermal state.
    pub const SERIOUS: Self = Self(2);
    /// Critical thermal state.
    pub const CRITICAL: Self = Self(3);
}

/// Activity options for process activities.
///
/// C++ equivalent: `NS::ActivityOptions`
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct ActivityOptions(pub u64);

impl ActivityOptions {
    /// Disable idle display sleep.
    pub const IDLE_DISPLAY_SLEEP_DISABLED: Self = Self(1 << 40);
    /// Disable idle system sleep.
    pub const IDLE_SYSTEM_SLEEP_DISABLED: Self = Self(1 << 20);
    /// Disable sudden termination.
    pub const SUDDEN_TERMINATION_DISABLED: Self = Self(1 << 14);
    /// Disable automatic termination.
    pub const AUTOMATIC_TERMINATION_DISABLED: Self = Self(1 << 15);
    /// User-initiated activity.
    pub const USER_INITIATED: Self = Self(0x00FFFFFF | (1 << 20));
    /// User-initiated activity allowing idle system sleep.
    pub const USER_INITIATED_ALLOWING_IDLE_SYSTEM_SLEEP: Self = Self(0x00FFFFFF);
    /// Background activity.
    pub const BACKGROUND: Self = Self(0x000000FF);
    /// Latency-critical activity.
    pub const LATENCY_CRITICAL: Self = Self(0xFF00000000);

    /// Returns the raw bits.
    #[inline]
    pub const fn bits(&self) -> u64 {
        self.0
    }

    /// Creates from raw bits.
    #[inline]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }
}

impl std::ops::BitOr for ActivityOptions {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for ActivityOptions {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

/// Device certification type.
///
/// C++ equivalent: `NS::DeviceCertification`
pub type DeviceCertification = Integer;

/// Process performance profile type.
///
/// C++ equivalent: `NS::ProcessPerformanceProfile`
pub type ProcessPerformanceProfile = Integer;

/// An Objective-C process info object.
///
/// C++ equivalent: `NS::ProcessInfo`
#[repr(transparent)]
#[derive(Clone)]
pub struct ProcessInfo(NonNull<c_void>);

impl ProcessInfo {
    /// Get the process info singleton.
    ///
    /// C++ equivalent: `static ProcessInfo* processInfo()`
    #[inline]
    pub fn process_info() -> Option<Self> {
        unsafe {
            let ptr: *mut c_void = msg_send_0(class!(NSProcessInfo).as_ptr(), sel!(processInfo));
            Self::from_ptr(ptr)
        }
    }

    /// Get the command-line arguments.
    ///
    /// C++ equivalent: `class Array* arguments() const`
    #[inline]
    pub fn arguments(&self) -> *mut Array<String> {
        unsafe { msg_send_0(self.as_ptr(), sel!(arguments)) }
    }

    /// Get the environment dictionary.
    ///
    /// C++ equivalent: `class Dictionary* environment() const`
    #[inline]
    pub fn environment(&self) -> *mut Dictionary {
        unsafe { msg_send_0(self.as_ptr(), sel!(environment)) }
    }

    /// Get the host name.
    ///
    /// C++ equivalent: `class String* hostName() const`
    #[inline]
    pub fn host_name(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(hostName)) }
    }

    /// Get the process name.
    ///
    /// C++ equivalent: `class String* processName() const`
    #[inline]
    pub fn process_name(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(processName)) }
    }

    /// Set the process name.
    ///
    /// C++ equivalent: `void setProcessName(const String* pString)`
    #[inline]
    pub fn set_process_name(&self, name: &String) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(setProcessName:), name.as_ptr());
        }
    }

    /// Get the process identifier.
    ///
    /// C++ equivalent: `int processIdentifier() const`
    #[inline]
    pub fn process_identifier(&self) -> i32 {
        unsafe { msg_send_0(self.as_ptr(), sel!(processIdentifier)) }
    }

    /// Get a globally unique string.
    ///
    /// C++ equivalent: `class String* globallyUniqueString() const`
    #[inline]
    pub fn globally_unique_string(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(globallyUniqueString)) }
    }

    /// Get the user name.
    #[inline]
    pub fn user_name(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(userName)) }
    }

    /// Get the full user name.
    #[inline]
    pub fn full_user_name(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(fullUserName)) }
    }

    /// Get the operating system identifier.
    ///
    /// C++ equivalent: `UInteger operatingSystem() const`
    #[inline]
    pub fn operating_system(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(operatingSystem)) }
    }

    /// Get the operating system version.
    ///
    /// C++ equivalent: `OperatingSystemVersion operatingSystemVersion() const`
    #[inline]
    pub fn operating_system_version(&self) -> OperatingSystemVersion {
        unsafe { msg_send_0(self.as_ptr(), sel!(operatingSystemVersion)) }
    }

    /// Get the operating system version string.
    ///
    /// C++ equivalent: `class String* operatingSystemVersionString() const`
    #[inline]
    pub fn operating_system_version_string(&self) -> *mut String {
        unsafe { msg_send_0(self.as_ptr(), sel!(operatingSystemVersionString)) }
    }

    /// Check if the operating system is at least a given version.
    ///
    /// C++ equivalent: `bool isOperatingSystemAtLeastVersion(OperatingSystemVersion version) const`
    #[inline]
    pub fn is_operating_system_at_least_version(&self, version: OperatingSystemVersion) -> bool {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(isOperatingSystemAtLeastVersion:),
                version,
            )
        }
    }

    /// Get the processor count.
    ///
    /// C++ equivalent: `UInteger processorCount() const`
    #[inline]
    pub fn processor_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(processorCount)) }
    }

    /// Get the active processor count.
    ///
    /// C++ equivalent: `UInteger activeProcessorCount() const`
    #[inline]
    pub fn active_processor_count(&self) -> UInteger {
        unsafe { msg_send_0(self.as_ptr(), sel!(activeProcessorCount)) }
    }

    /// Get the physical memory size.
    ///
    /// C++ equivalent: `unsigned long long physicalMemory() const`
    #[inline]
    pub fn physical_memory(&self) -> u64 {
        unsafe { msg_send_0(self.as_ptr(), sel!(physicalMemory)) }
    }

    /// Get the system uptime.
    ///
    /// C++ equivalent: `TimeInterval systemUptime() const`
    #[inline]
    pub fn system_uptime(&self) -> TimeInterval {
        unsafe { msg_send_0(self.as_ptr(), sel!(systemUptime)) }
    }

    /// Disable sudden termination.
    ///
    /// C++ equivalent: `void disableSuddenTermination()`
    #[inline]
    pub fn disable_sudden_termination(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(disableSuddenTermination));
        }
    }

    /// Enable sudden termination.
    ///
    /// C++ equivalent: `void enableSuddenTermination()`
    #[inline]
    pub fn enable_sudden_termination(&self) {
        unsafe {
            msg_send_0::<()>(self.as_ptr(), sel!(enableSuddenTermination));
        }
    }

    /// Disable automatic termination.
    ///
    /// C++ equivalent: `void disableAutomaticTermination(const class String* pReason)`
    #[inline]
    pub fn disable_automatic_termination(&self, reason: &String) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(disableAutomaticTermination:),
                reason.as_ptr(),
            );
        }
    }

    /// Enable automatic termination.
    ///
    /// C++ equivalent: `void enableAutomaticTermination(const class String* pReason)`
    #[inline]
    pub fn enable_automatic_termination(&self, reason: &String) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(enableAutomaticTermination:),
                reason.as_ptr(),
            );
        }
    }

    /// Check if automatic termination support is enabled.
    ///
    /// C++ equivalent: `bool automaticTerminationSupportEnabled() const`
    #[inline]
    pub fn automatic_termination_support_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(automaticTerminationSupportEnabled)) }
    }

    /// Set automatic termination support enabled.
    ///
    /// C++ equivalent: `void setAutomaticTerminationSupportEnabled(bool enabled)`
    #[inline]
    pub fn set_automatic_termination_support_enabled(&self, enabled: bool) {
        unsafe {
            let _: () = msg_send_1(
                self.as_ptr(),
                sel!(setAutomaticTerminationSupportEnabled:),
                enabled,
            );
        }
    }

    /// Begin an activity.
    ///
    /// C++ equivalent: `class Object* beginActivity(ActivityOptions options, const class String* pReason)`
    #[inline]
    pub fn begin_activity(&self, options: ActivityOptions, reason: &String) -> *mut Object {
        unsafe {
            msg_send_2(
                self.as_ptr(),
                sel!(beginActivityWithOptions:reason:),
                options.0,
                reason.as_ptr(),
            )
        }
    }

    /// End an activity.
    ///
    /// C++ equivalent: `void endActivity(class Object* pActivity)`
    #[inline]
    pub fn end_activity(&self, activity: &Object) {
        unsafe {
            let _: () = msg_send_1(self.as_ptr(), sel!(endActivity:), activity.as_ptr());
        }
    }

    /// Get the thermal state.
    ///
    /// C++ equivalent: `ProcessInfoThermalState thermalState() const`
    #[inline]
    pub fn thermal_state(&self) -> ProcessInfoThermalState {
        unsafe { msg_send_0(self.as_ptr(), sel!(thermalState)) }
    }

    /// Check if low power mode is enabled.
    ///
    /// C++ equivalent: `bool isLowPowerModeEnabled() const`
    #[inline]
    pub fn is_low_power_mode_enabled(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isLowPowerModeEnabled)) }
    }

    /// Check if this is an iOS app running on Mac.
    ///
    /// C++ equivalent: `bool isiOSAppOnMac() const`
    #[inline]
    pub fn is_ios_app_on_mac(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isiOSAppOnMac)) }
    }

    /// Check if this is a Mac Catalyst app.
    ///
    /// C++ equivalent: `bool isMacCatalystApp() const`
    #[inline]
    pub fn is_mac_catalyst_app(&self) -> bool {
        unsafe { msg_send_0(self.as_ptr(), sel!(isMacCatalystApp)) }
    }

    /// Check if the device is certified.
    ///
    /// C++ equivalent: `bool isDeviceCertified(DeviceCertification performanceTier) const`
    #[inline]
    pub fn is_device_certified(&self, performance_tier: DeviceCertification) -> bool {
        unsafe { msg_send_1(self.as_ptr(), sel!(isDeviceCertified:), performance_tier) }
    }

    /// Check if the process has a performance profile.
    ///
    /// C++ equivalent: `bool hasPerformanceProfile(ProcessPerformanceProfile performanceProfile) const`
    #[inline]
    pub fn has_performance_profile(&self, performance_profile: ProcessPerformanceProfile) -> bool {
        unsafe {
            msg_send_1(
                self.as_ptr(),
                sel!(hasPerformanceProfile:),
                performance_profile,
            )
        }
    }

    /// Create a ProcessInfo from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid Objective-C NSProcessInfo object.
    #[inline]
    pub unsafe fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }
}

impl Referencing for ProcessInfo {
    #[inline]
    fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

unsafe impl Send for ProcessInfo {}
unsafe impl Sync for ProcessInfo {}

impl std::fmt::Debug for ProcessInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProcessInfo").field("ptr", &self.0).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_info_size() {
        assert_eq!(
            std::mem::size_of::<ProcessInfo>(),
            std::mem::size_of::<*mut c_void>()
        );
    }

    #[test]
    fn test_thermal_state_values() {
        assert_eq!(ProcessInfoThermalState::NOMINAL.0, 0);
        assert_eq!(ProcessInfoThermalState::FAIR.0, 1);
        assert_eq!(ProcessInfoThermalState::SERIOUS.0, 2);
        assert_eq!(ProcessInfoThermalState::CRITICAL.0, 3);
    }
}
