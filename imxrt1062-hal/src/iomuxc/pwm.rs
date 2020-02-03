//! PWM pin labels

use crate::iomuxc::{
    gpio::{
        GPIO_B0_10, GPIO_B0_11, GPIO_B1_00, GPIO_B1_01, GPIO_EMC_04, GPIO_EMC_05, GPIO_EMC_06,
        GPIO_EMC_08,
    },
    Alt1, Alt2, Alt6,
};

/// Type tags that designate a PWM pin output designation
pub mod output {
    /// Describes an output pin
    pub trait Output {}
    /// PWM output pin 'A'
    pub struct A;
    impl Output for A {}
    /// PWM output pin 'B'
    pub struct B;
    impl Output for B {}
}

/// Type tags that designate a PWM module
pub mod module {
    /// Describes a PWM pin's associated module
    pub trait Module {}
    pub struct _1;
    impl Module for _1 {}
    pub struct _2;
    impl Module for _2 {}
    pub struct _3;
    impl Module for _3 {}
    pub struct _4;
    impl Module for _4 {}
}

/// Type tags for PWM submodules
///
/// Each PWM modules has four submodules, each having
/// two output pins.
pub mod submodule {
    use crate::pac::pwm1;

    /// Describes a PWM pin's associated submodule
    ///
    /// Each PWM module contains four submodules, 0 through 3 (inclusive).
    pub trait Submodule {
        /// The submodule's number represented as a value
        const IDX: usize;
        /// Returns the submodule register block associated with this
        /// PWM module.
        fn submodule(pwm: &pwm1::RegisterBlock) -> &pwm1::SM;
    }
    pub struct _0;
    impl Submodule for _0 {
        const IDX: usize = 0;
        fn submodule(pwm: &pwm1::RegisterBlock) -> &pwm1::SM {
            &pwm.sm0
        }
    }
    pub struct _1;
    impl Submodule for _1 {
        const IDX: usize = 1;
        fn submodule(pwm: &pwm1::RegisterBlock) -> &pwm1::SM {
            &pwm.sm1
        }
    }
    pub struct _2;
    impl Submodule for _2 {
        const IDX: usize = 2;
        fn submodule(pwm: &pwm1::RegisterBlock) -> &pwm1::SM {
            &pwm.sm2
        }
    }
    pub struct _3;
    impl Submodule for _3 {
        const IDX: usize = 3;
        fn submodule(pwm: &pwm1::RegisterBlock) -> &pwm1::SM {
            &pwm.sm3
        }
    }
}

/// Describes a pin that might be used for PWM functions
pub trait Pin {
    /// The pin's output designation; either 'A' or 'B'
    type Output: output::Output;
    /// The associated PWM module, base 0
    type Module: module::Module;
    /// The submodule of the PWM module
    type Submodule: submodule::Submodule;
}

impl Pin for GPIO_B0_10<Alt2> {
    type Output = output::A;
    type Module = module::_2; // FlexPWM2
    type Submodule = submodule::_2; // FlexPWM2
}

impl Pin for GPIO_B0_11<Alt2> {
    type Output = output::B;
    type Module = module::_2; // FlexPWM2
    type Submodule = submodule::_2; // FlexPWM2
}

impl Pin for GPIO_B1_01<Alt6> {
    type Output = output::B;
    type Module = module::_1;
    type Submodule = submodule::_3;
}

impl Pin for GPIO_B1_00<Alt6> {
    type Output = output::A;
    type Module = module::_1;
    type Submodule = submodule::_3;
}

impl Pin for GPIO_EMC_04<Alt1> {
    type Output = output::A;
    type Module = module::_4;
    type Submodule = submodule::_2;
}

impl Pin for GPIO_EMC_05<Alt1> {
    type Output = output::B;
    type Module = module::_4;
    type Submodule = submodule::_2;
}

impl Pin for GPIO_EMC_06<Alt1> {
    type Output = output::A;
    type Module = module::_2;
    type Submodule = submodule::_0;
}

impl Pin for GPIO_EMC_08<Alt1> {
    type Output = output::A;
    type Module = module::_2;
    type Submodule = submodule::_1;
}
