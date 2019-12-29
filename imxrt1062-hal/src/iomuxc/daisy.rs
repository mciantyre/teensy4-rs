//! Type-level support for daisy chaining
//!
//! Daisy chaining allows multiple input pads to drive the
//! sample module. Some peripherals will require that the
//! user specifies the daisy chaining behavior for the pin.
//! In those cases, the will accept a `Daisy<..>` adapter.
//! The allocation of the adapter will automatically set
//! the daisy chaining configuration in the processor.

/// Specifies that a pad has been configured with its daisy chain
/// settings. Once it's in this state, we can't get the original
/// pad back.
pub struct Daisy<Pad> {
    /// The underlying pad
    _pad: Pad,
}

impl<Pad> Daisy<Pad> {
    pub(crate) fn new(pad: Pad) -> Self {
        Daisy { _pad: pad }
    }
}

/// Trait describing a type that can apply a daisy-chain setting in
/// the IOMUX controller. Peripherals that require a daisy-chained pad should
/// provide an interface over types that implement `IntoDaisy`, then call
/// `into_daisy` to perform the configuration.
pub trait IntoDaisy: Sized {
    /// Converts `self` into a `Daisy` version of `self`. Calling this function
    /// will perform some modification of a register in the IOMUXC.
    fn into_daisy(self) -> Daisy<Self>;
}
