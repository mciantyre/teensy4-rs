//! FCB fields
//!
//! The module implements and documents the common FCB fields.

use std::convert::TryFrom;
use std::marker::PhantomData;
use std::time::Duration;

/// `readSampleClkSrc` of the general FCB   
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum ReadSampleClockSource {
    InternalLoopback = 0x00,
    LoopbackFromDQSPad = 0x01,
    FlashProvidedDQS = 0x03,
}

/// Serial flash CS hold time
///
/// Defaults to `0x03`, the 'recommended value'
pub struct CSHoldTime([u8; 1]);

impl CSHoldTime {
    pub fn new(hold_time: u8) -> Self {
        CSHoldTime([hold_time])
    }
}

impl Default for CSHoldTime {
    fn default() -> Self {
        CSHoldTime([0x03])
    }
}

as_ref_bytes_newtype!(CSHoldTime);

/// Serial flash CS setup time
///
/// Defaults to `0x03`, the 'recommended value'
pub struct CSSetupTime([u8; 1]);

impl CSSetupTime {
    pub fn new(setup_time: u8) -> Self {
        CSSetupTime([setup_time])
    }
}

impl Default for CSSetupTime {
    fn default() -> Self {
        CSSetupTime([0x03])
    }
}

as_ref_bytes_newtype!(CSSetupTime);

/// Column address width
pub struct ColumnAddressWidth([u8; 1]);
impl ColumnAddressWidth {
    /// Returns the value that represnts 'other devices'
    pub fn other_devices() -> Self {
        ColumnAddressWidth([0])
    }
}

as_ref_bytes_newtype!(ColumnAddressWidth);

/// `deviceModeArg`, only useful when paired
/// with `DeviceModeConfiguration`.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct DeviceModeArgument([u8; 4]);

impl DeviceModeArgument {
    pub fn new(argument: u32) -> Self {
        Self(argument.to_le_bytes())
    }
}

as_ref_bytes_newtype!(DeviceModeArgument);

/// Describes both the `deviceModeCfgEnable` field, and
/// the `deviceModeArg` field, which is only valid if
/// the configuration is enabled.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DeviceModeConfiguration {
    Disabled,
    Enabled(DeviceModeArgument),
}

impl Default for DeviceModeConfiguration {
    fn default() -> Self {
        DeviceModeConfiguration::Disabled
    }
}

/// Wait time for all configuration commands
///
/// From the docs...
///
/// > Available for device that support v1.1.0 FlexSPI configuration block.
/// > If it is greater than 0, ROM will wait waitTimeCfgCommands * 100us
/// > for all device memory configuration commands instead of using read
/// > status to wait until these commands complete.
pub struct WaitTimeConfigurationCommands([u8; 2]);
impl WaitTimeConfigurationCommands {
    pub fn disable() -> Self {
        WaitTimeConfigurationCommands([0, 0])
    }

    /// Computes the wait time from the specified `wait_time`. The
    /// provided duration should be divisible by `100us`, since the
    /// value is a factor scaled by `100us`. Returns `None` if representing
    /// this as a factor of `100us` returns `0`, or if the factor cannot be
    /// expressed in a `u16`.
    pub fn from_duration(wait_time: Duration) -> Option<Self> {
        let us = wait_time.as_micros();
        if us < 100 {
            None
        } else {
            let factor = u16::try_from(us / 100).ok()?;
            Some(WaitTimeConfigurationCommands(factor.to_le_bytes()))
        }
    }
}

as_ref_bytes_newtype!(WaitTimeConfigurationCommands);

/// Sequence parameter for device mode configuration
#[derive(Default)]
pub struct DeviceModeSequence([u8; 4]);
impl DeviceModeSequence {
    /// Create a new sequence parameter for device configuration
    ///
    /// `starting_lut_index`: starting LUT index of Device mode configuration command
    /// `number_of_luts`: number of LUT sequences for Device mode configuration command
    pub fn new(number_of_luts: u8, starting_lut_index: u8) -> Self {
        DeviceModeSequence(
            ((u32::from(starting_lut_index) << 8) | u32::from(number_of_luts)).to_le_bytes(),
        )
    }
}

as_ref_bytes_newtype!(DeviceModeSequence);

/// Describes the `deviceType` field.
///
/// Only the SerialNOR is implemented; `DeviceType`
/// may also have `SerialNAND` in the future.
pub enum DeviceType {
    SerialNOR(crate::nor::ConfigurationBlock),
}

/// `sFlashPad` field
#[repr(u8)]
pub enum FlashPadType {
    Single = 1,
    Dual = 2,
    Quad = 4,
    Octal = 8,
}

/// `serialClkFreq`
#[repr(u8)]
pub enum SerialClockFrequency {
    MHz30 = 1,
    MHz50 = 2,
    MHz60 = 3,
    MHz75 = 4,
    MHz80 = 5,
    MHz100 = 6,
    MHz120 = 7,
    HMz133 = 8,
    MHz166 = 9,
}

/// Type tag for `SerialFlashSize`
pub struct A1;
/// Type tag for `SerialFlashSize`
pub struct A2;
/// Type tag for `SerialFlashSize`
pub struct B1;
/// Type tag for `SerialFlashSize`
pub struct B2;

/// `sFlashXXSize` field, where `XX` is one of `A1`,
/// `A2`, `B1`, `B2`.
///
/// If we're using SPI NAND, the size wrapped
/// in this value will be multiplied by `2`.
pub struct SerialFlashSize<Region> {
    _region: PhantomData<Region>,
    pub(crate) size: u32,
}

impl<Region> SerialFlashSize<Region> {
    /// The `actual_size` will be multiplied by `2` in
    /// the final output if we're also using SPI NAND.
    pub fn new(actual_size: u32) -> Self {
        SerialFlashSize {
            _region: PhantomData,
            size: actual_size,
        }
    }
}

impl<Region> Default for SerialFlashSize<Region> {
    fn default() -> Self {
        Self::new(0)
    }
}
