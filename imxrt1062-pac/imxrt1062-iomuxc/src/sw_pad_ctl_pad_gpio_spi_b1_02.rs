#[doc = "Reader of register SW_PAD_CTL_PAD_GPIO_SPI_B1_02"]
pub type R = crate::R<u32, super::SW_PAD_CTL_PAD_GPIO_SPI_B1_02>;
#[doc = "Writer for register SW_PAD_CTL_PAD_GPIO_SPI_B1_02"]
pub type W = crate::W<u32, super::SW_PAD_CTL_PAD_GPIO_SPI_B1_02>;
#[doc = "Register SW_PAD_CTL_PAD_GPIO_SPI_B1_02 `reset()`'s with value 0x10b0"]
impl crate::ResetValue for super::SW_PAD_CTL_PAD_GPIO_SPI_B1_02 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10b0
    }
}
#[doc = "Slew Rate Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRE_A {
    #[doc = "0: Slow Slew Rate"]
    SRE_0_SLOW_SLEW_RATE,
    #[doc = "1: Fast Slew Rate"]
    SRE_1_FAST_SLEW_RATE,
}
impl From<SRE_A> for bool {
    #[inline(always)]
    fn from(variant: SRE_A) -> Self {
        match variant {
            SRE_A::SRE_0_SLOW_SLEW_RATE => false,
            SRE_A::SRE_1_FAST_SLEW_RATE => true,
        }
    }
}
#[doc = "Reader of field `SRE`"]
pub type SRE_R = crate::R<bool, SRE_A>;
impl SRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRE_A {
        match self.bits {
            false => SRE_A::SRE_0_SLOW_SLEW_RATE,
            true => SRE_A::SRE_1_FAST_SLEW_RATE,
        }
    }
    #[doc = "Checks if the value of the field is `SRE_0_SLOW_SLEW_RATE`"]
    #[inline(always)]
    pub fn is_sre_0_slow_slew_rate(&self) -> bool {
        *self == SRE_A::SRE_0_SLOW_SLEW_RATE
    }
    #[doc = "Checks if the value of the field is `SRE_1_FAST_SLEW_RATE`"]
    #[inline(always)]
    pub fn is_sre_1_fast_slew_rate(&self) -> bool {
        *self == SRE_A::SRE_1_FAST_SLEW_RATE
    }
}
#[doc = "Write proxy for field `SRE`"]
pub struct SRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slow Slew Rate"]
    #[inline(always)]
    pub fn sre_0_slow_slew_rate(self) -> &'a mut W {
        self.variant(SRE_A::SRE_0_SLOW_SLEW_RATE)
    }
    #[doc = "Fast Slew Rate"]
    #[inline(always)]
    pub fn sre_1_fast_slew_rate(self) -> &'a mut W {
        self.variant(SRE_A::SRE_1_FAST_SLEW_RATE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Drive Strength Field\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSE_A {
    #[doc = "0: output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_,
    #[doc = "1: R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V_,
    #[doc = "2: R0/2"]
    DSE_2_R0_2,
    #[doc = "3: R0/3"]
    DSE_3_R0_3,
    #[doc = "4: R0/4"]
    DSE_4_R0_4,
    #[doc = "5: R0/5"]
    DSE_5_R0_5,
    #[doc = "6: R0/6"]
    DSE_6_R0_6,
    #[doc = "7: R0/7"]
    DSE_7_R0_7,
}
impl From<DSE_A> for u8 {
    #[inline(always)]
    fn from(variant: DSE_A) -> Self {
        match variant {
            DSE_A::DSE_0_OUTPUT_DRIVER_DISABLED_ => 0,
            DSE_A::DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V_ => 1,
            DSE_A::DSE_2_R0_2 => 2,
            DSE_A::DSE_3_R0_3 => 3,
            DSE_A::DSE_4_R0_4 => 4,
            DSE_A::DSE_5_R0_5 => 5,
            DSE_A::DSE_6_R0_6 => 6,
            DSE_A::DSE_7_R0_7 => 7,
        }
    }
}
#[doc = "Reader of field `DSE`"]
pub type DSE_R = crate::R<u8, DSE_A>;
impl DSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSE_A {
        match self.bits {
            0 => DSE_A::DSE_0_OUTPUT_DRIVER_DISABLED_,
            1 => DSE_A::DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V_,
            2 => DSE_A::DSE_2_R0_2,
            3 => DSE_A::DSE_3_R0_3,
            4 => DSE_A::DSE_4_R0_4,
            5 => DSE_A::DSE_5_R0_5,
            6 => DSE_A::DSE_6_R0_6,
            7 => DSE_A::DSE_7_R0_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DSE_0_OUTPUT_DRIVER_DISABLED_`"]
    #[inline(always)]
    pub fn is_dse_0_output_driver_disabled_(&self) -> bool {
        *self == DSE_A::DSE_0_OUTPUT_DRIVER_DISABLED_
    }
    #[doc = "Checks if the value of the field is `DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V_`"]
    #[inline(always)]
    pub fn is_dse_1_r0_150_ohm_3_3v_260_ohm_1_8v_(&self) -> bool {
        *self == DSE_A::DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V_
    }
    #[doc = "Checks if the value of the field is `DSE_2_R0_2`"]
    #[inline(always)]
    pub fn is_dse_2_r0_2(&self) -> bool {
        *self == DSE_A::DSE_2_R0_2
    }
    #[doc = "Checks if the value of the field is `DSE_3_R0_3`"]
    #[inline(always)]
    pub fn is_dse_3_r0_3(&self) -> bool {
        *self == DSE_A::DSE_3_R0_3
    }
    #[doc = "Checks if the value of the field is `DSE_4_R0_4`"]
    #[inline(always)]
    pub fn is_dse_4_r0_4(&self) -> bool {
        *self == DSE_A::DSE_4_R0_4
    }
    #[doc = "Checks if the value of the field is `DSE_5_R0_5`"]
    #[inline(always)]
    pub fn is_dse_5_r0_5(&self) -> bool {
        *self == DSE_A::DSE_5_R0_5
    }
    #[doc = "Checks if the value of the field is `DSE_6_R0_6`"]
    #[inline(always)]
    pub fn is_dse_6_r0_6(&self) -> bool {
        *self == DSE_A::DSE_6_R0_6
    }
    #[doc = "Checks if the value of the field is `DSE_7_R0_7`"]
    #[inline(always)]
    pub fn is_dse_7_r0_7(&self) -> bool {
        *self == DSE_A::DSE_7_R0_7
    }
}
#[doc = "Write proxy for field `DSE`"]
pub struct DSE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "output driver disabled;"]
    #[inline(always)]
    pub fn dse_0_output_driver_disabled_(self) -> &'a mut W {
        self.variant(DSE_A::DSE_0_OUTPUT_DRIVER_DISABLED_)
    }
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    #[inline(always)]
    pub fn dse_1_r0_150_ohm_3_3v_260_ohm_1_8v_(self) -> &'a mut W {
        self.variant(DSE_A::DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V_)
    }
    #[doc = "R0/2"]
    #[inline(always)]
    pub fn dse_2_r0_2(self) -> &'a mut W {
        self.variant(DSE_A::DSE_2_R0_2)
    }
    #[doc = "R0/3"]
    #[inline(always)]
    pub fn dse_3_r0_3(self) -> &'a mut W {
        self.variant(DSE_A::DSE_3_R0_3)
    }
    #[doc = "R0/4"]
    #[inline(always)]
    pub fn dse_4_r0_4(self) -> &'a mut W {
        self.variant(DSE_A::DSE_4_R0_4)
    }
    #[doc = "R0/5"]
    #[inline(always)]
    pub fn dse_5_r0_5(self) -> &'a mut W {
        self.variant(DSE_A::DSE_5_R0_5)
    }
    #[doc = "R0/6"]
    #[inline(always)]
    pub fn dse_6_r0_6(self) -> &'a mut W {
        self.variant(DSE_A::DSE_6_R0_6)
    }
    #[doc = "R0/7"]
    #[inline(always)]
    pub fn dse_7_r0_7(self) -> &'a mut W {
        self.variant(DSE_A::DSE_7_R0_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Speed Field\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEED_A {
    #[doc = "0: low(50MHz)"]
    SPEED_0_LOW_50MHZ_,
    #[doc = "1: medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ_,
    #[doc = "2: medium(100MHz)"]
    SPEED_2_MEDIUM_100MHZ_,
    #[doc = "3: max(200MHz)"]
    SPEED_3_MAX_200MHZ_,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        match variant {
            SPEED_A::SPEED_0_LOW_50MHZ_ => 0,
            SPEED_A::SPEED_1_MEDIUM_100MHZ_ => 1,
            SPEED_A::SPEED_2_MEDIUM_100MHZ_ => 2,
            SPEED_A::SPEED_3_MAX_200MHZ_ => 3,
        }
    }
}
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, SPEED_A>;
impl SPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPEED_A {
        match self.bits {
            0 => SPEED_A::SPEED_0_LOW_50MHZ_,
            1 => SPEED_A::SPEED_1_MEDIUM_100MHZ_,
            2 => SPEED_A::SPEED_2_MEDIUM_100MHZ_,
            3 => SPEED_A::SPEED_3_MAX_200MHZ_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPEED_0_LOW_50MHZ_`"]
    #[inline(always)]
    pub fn is_speed_0_low_50mhz_(&self) -> bool {
        *self == SPEED_A::SPEED_0_LOW_50MHZ_
    }
    #[doc = "Checks if the value of the field is `SPEED_1_MEDIUM_100MHZ_`"]
    #[inline(always)]
    pub fn is_speed_1_medium_100mhz_(&self) -> bool {
        *self == SPEED_A::SPEED_1_MEDIUM_100MHZ_
    }
    #[doc = "Checks if the value of the field is `SPEED_2_MEDIUM_100MHZ_`"]
    #[inline(always)]
    pub fn is_speed_2_medium_100mhz_(&self) -> bool {
        *self == SPEED_A::SPEED_2_MEDIUM_100MHZ_
    }
    #[doc = "Checks if the value of the field is `SPEED_3_MAX_200MHZ_`"]
    #[inline(always)]
    pub fn is_speed_3_max_200mhz_(&self) -> bool {
        *self == SPEED_A::SPEED_3_MAX_200MHZ_
    }
}
#[doc = "Write proxy for field `SPEED`"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPEED_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "low(50MHz)"]
    #[inline(always)]
    pub fn speed_0_low_50mhz_(self) -> &'a mut W {
        self.variant(SPEED_A::SPEED_0_LOW_50MHZ_)
    }
    #[doc = "medium(100MHz)"]
    #[inline(always)]
    pub fn speed_1_medium_100mhz_(self) -> &'a mut W {
        self.variant(SPEED_A::SPEED_1_MEDIUM_100MHZ_)
    }
    #[doc = "medium(100MHz)"]
    #[inline(always)]
    pub fn speed_2_medium_100mhz_(self) -> &'a mut W {
        self.variant(SPEED_A::SPEED_2_MEDIUM_100MHZ_)
    }
    #[doc = "max(200MHz)"]
    #[inline(always)]
    pub fn speed_3_max_200mhz_(self) -> &'a mut W {
        self.variant(SPEED_A::SPEED_3_MAX_200MHZ_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Open Drain Enable Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODE_A {
    #[doc = "0: Open Drain Disabled"]
    ODE_0_OPEN_DRAIN_DISABLED,
    #[doc = "1: Open Drain Enabled"]
    ODE_1_OPEN_DRAIN_ENABLED,
}
impl From<ODE_A> for bool {
    #[inline(always)]
    fn from(variant: ODE_A) -> Self {
        match variant {
            ODE_A::ODE_0_OPEN_DRAIN_DISABLED => false,
            ODE_A::ODE_1_OPEN_DRAIN_ENABLED => true,
        }
    }
}
#[doc = "Reader of field `ODE`"]
pub type ODE_R = crate::R<bool, ODE_A>;
impl ODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODE_A {
        match self.bits {
            false => ODE_A::ODE_0_OPEN_DRAIN_DISABLED,
            true => ODE_A::ODE_1_OPEN_DRAIN_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ODE_0_OPEN_DRAIN_DISABLED`"]
    #[inline(always)]
    pub fn is_ode_0_open_drain_disabled(&self) -> bool {
        *self == ODE_A::ODE_0_OPEN_DRAIN_DISABLED
    }
    #[doc = "Checks if the value of the field is `ODE_1_OPEN_DRAIN_ENABLED`"]
    #[inline(always)]
    pub fn is_ode_1_open_drain_enabled(&self) -> bool {
        *self == ODE_A::ODE_1_OPEN_DRAIN_ENABLED
    }
}
#[doc = "Write proxy for field `ODE`"]
pub struct ODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Open Drain Disabled"]
    #[inline(always)]
    pub fn ode_0_open_drain_disabled(self) -> &'a mut W {
        self.variant(ODE_A::ODE_0_OPEN_DRAIN_DISABLED)
    }
    #[doc = "Open Drain Enabled"]
    #[inline(always)]
    pub fn ode_1_open_drain_enabled(self) -> &'a mut W {
        self.variant(ODE_A::ODE_1_OPEN_DRAIN_ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Pull / Keep Enable Field\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PKE_A {
    #[doc = "0: Pull/Keeper Disabled"]
    PKE_0_PULL_KEEPER_DISABLED,
    #[doc = "1: Pull/Keeper Enabled"]
    PKE_1_PULL_KEEPER_ENABLED,
}
impl From<PKE_A> for bool {
    #[inline(always)]
    fn from(variant: PKE_A) -> Self {
        match variant {
            PKE_A::PKE_0_PULL_KEEPER_DISABLED => false,
            PKE_A::PKE_1_PULL_KEEPER_ENABLED => true,
        }
    }
}
#[doc = "Reader of field `PKE`"]
pub type PKE_R = crate::R<bool, PKE_A>;
impl PKE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PKE_A {
        match self.bits {
            false => PKE_A::PKE_0_PULL_KEEPER_DISABLED,
            true => PKE_A::PKE_1_PULL_KEEPER_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `PKE_0_PULL_KEEPER_DISABLED`"]
    #[inline(always)]
    pub fn is_pke_0_pull_keeper_disabled(&self) -> bool {
        *self == PKE_A::PKE_0_PULL_KEEPER_DISABLED
    }
    #[doc = "Checks if the value of the field is `PKE_1_PULL_KEEPER_ENABLED`"]
    #[inline(always)]
    pub fn is_pke_1_pull_keeper_enabled(&self) -> bool {
        *self == PKE_A::PKE_1_PULL_KEEPER_ENABLED
    }
}
#[doc = "Write proxy for field `PKE`"]
pub struct PKE_W<'a> {
    w: &'a mut W,
}
impl<'a> PKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PKE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pull/Keeper Disabled"]
    #[inline(always)]
    pub fn pke_0_pull_keeper_disabled(self) -> &'a mut W {
        self.variant(PKE_A::PKE_0_PULL_KEEPER_DISABLED)
    }
    #[doc = "Pull/Keeper Enabled"]
    #[inline(always)]
    pub fn pke_1_pull_keeper_enabled(self) -> &'a mut W {
        self.variant(PKE_A::PKE_1_PULL_KEEPER_ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Pull / Keep Select Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUE_A {
    #[doc = "0: Keeper"]
    PUE_0_KEEPER,
    #[doc = "1: Pull"]
    PUE_1_PULL,
}
impl From<PUE_A> for bool {
    #[inline(always)]
    fn from(variant: PUE_A) -> Self {
        match variant {
            PUE_A::PUE_0_KEEPER => false,
            PUE_A::PUE_1_PULL => true,
        }
    }
}
#[doc = "Reader of field `PUE`"]
pub type PUE_R = crate::R<bool, PUE_A>;
impl PUE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUE_A {
        match self.bits {
            false => PUE_A::PUE_0_KEEPER,
            true => PUE_A::PUE_1_PULL,
        }
    }
    #[doc = "Checks if the value of the field is `PUE_0_KEEPER`"]
    #[inline(always)]
    pub fn is_pue_0_keeper(&self) -> bool {
        *self == PUE_A::PUE_0_KEEPER
    }
    #[doc = "Checks if the value of the field is `PUE_1_PULL`"]
    #[inline(always)]
    pub fn is_pue_1_pull(&self) -> bool {
        *self == PUE_A::PUE_1_PULL
    }
}
#[doc = "Write proxy for field `PUE`"]
pub struct PUE_W<'a> {
    w: &'a mut W,
}
impl<'a> PUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Keeper"]
    #[inline(always)]
    pub fn pue_0_keeper(self) -> &'a mut W {
        self.variant(PUE_A::PUE_0_KEEPER)
    }
    #[doc = "Pull"]
    #[inline(always)]
    pub fn pue_1_pull(self) -> &'a mut W {
        self.variant(PUE_A::PUE_1_PULL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Pull Up / Down Config. Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUS_A {
    #[doc = "0: 100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN,
    #[doc = "1: 47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP,
    #[doc = "2: 100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP,
    #[doc = "3: 22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP,
}
impl From<PUS_A> for u8 {
    #[inline(always)]
    fn from(variant: PUS_A) -> Self {
        match variant {
            PUS_A::PUS_0_100K_OHM_PULL_DOWN => 0,
            PUS_A::PUS_1_47K_OHM_PULL_UP => 1,
            PUS_A::PUS_2_100K_OHM_PULL_UP => 2,
            PUS_A::PUS_3_22K_OHM_PULL_UP => 3,
        }
    }
}
#[doc = "Reader of field `PUS`"]
pub type PUS_R = crate::R<u8, PUS_A>;
impl PUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUS_A {
        match self.bits {
            0 => PUS_A::PUS_0_100K_OHM_PULL_DOWN,
            1 => PUS_A::PUS_1_47K_OHM_PULL_UP,
            2 => PUS_A::PUS_2_100K_OHM_PULL_UP,
            3 => PUS_A::PUS_3_22K_OHM_PULL_UP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PUS_0_100K_OHM_PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pus_0_100k_ohm_pull_down(&self) -> bool {
        *self == PUS_A::PUS_0_100K_OHM_PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PUS_1_47K_OHM_PULL_UP`"]
    #[inline(always)]
    pub fn is_pus_1_47k_ohm_pull_up(&self) -> bool {
        *self == PUS_A::PUS_1_47K_OHM_PULL_UP
    }
    #[doc = "Checks if the value of the field is `PUS_2_100K_OHM_PULL_UP`"]
    #[inline(always)]
    pub fn is_pus_2_100k_ohm_pull_up(&self) -> bool {
        *self == PUS_A::PUS_2_100K_OHM_PULL_UP
    }
    #[doc = "Checks if the value of the field is `PUS_3_22K_OHM_PULL_UP`"]
    #[inline(always)]
    pub fn is_pus_3_22k_ohm_pull_up(&self) -> bool {
        *self == PUS_A::PUS_3_22K_OHM_PULL_UP
    }
}
#[doc = "Write proxy for field `PUS`"]
pub struct PUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "100K Ohm Pull Down"]
    #[inline(always)]
    pub fn pus_0_100k_ohm_pull_down(self) -> &'a mut W {
        self.variant(PUS_A::PUS_0_100K_OHM_PULL_DOWN)
    }
    #[doc = "47K Ohm Pull Up"]
    #[inline(always)]
    pub fn pus_1_47k_ohm_pull_up(self) -> &'a mut W {
        self.variant(PUS_A::PUS_1_47K_OHM_PULL_UP)
    }
    #[doc = "100K Ohm Pull Up"]
    #[inline(always)]
    pub fn pus_2_100k_ohm_pull_up(self) -> &'a mut W {
        self.variant(PUS_A::PUS_2_100K_OHM_PULL_UP)
    }
    #[doc = "22K Ohm Pull Up"]
    #[inline(always)]
    pub fn pus_3_22k_ohm_pull_up(self) -> &'a mut W {
        self.variant(PUS_A::PUS_3_22K_OHM_PULL_UP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Hyst. Enable Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYS_A {
    #[doc = "0: Hysteresis Disabled"]
    HYS_0_HYSTERESIS_DISABLED,
    #[doc = "1: Hysteresis Enabled"]
    HYS_1_HYSTERESIS_ENABLED,
}
impl From<HYS_A> for bool {
    #[inline(always)]
    fn from(variant: HYS_A) -> Self {
        match variant {
            HYS_A::HYS_0_HYSTERESIS_DISABLED => false,
            HYS_A::HYS_1_HYSTERESIS_ENABLED => true,
        }
    }
}
#[doc = "Reader of field `HYS`"]
pub type HYS_R = crate::R<bool, HYS_A>;
impl HYS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYS_A {
        match self.bits {
            false => HYS_A::HYS_0_HYSTERESIS_DISABLED,
            true => HYS_A::HYS_1_HYSTERESIS_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `HYS_0_HYSTERESIS_DISABLED`"]
    #[inline(always)]
    pub fn is_hys_0_hysteresis_disabled(&self) -> bool {
        *self == HYS_A::HYS_0_HYSTERESIS_DISABLED
    }
    #[doc = "Checks if the value of the field is `HYS_1_HYSTERESIS_ENABLED`"]
    #[inline(always)]
    pub fn is_hys_1_hysteresis_enabled(&self) -> bool {
        *self == HYS_A::HYS_1_HYSTERESIS_ENABLED
    }
}
#[doc = "Write proxy for field `HYS`"]
pub struct HYS_W<'a> {
    w: &'a mut W,
}
impl<'a> HYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hysteresis Disabled"]
    #[inline(always)]
    pub fn hys_0_hysteresis_disabled(self) -> &'a mut W {
        self.variant(HYS_A::HYS_0_HYSTERESIS_DISABLED)
    }
    #[doc = "Hysteresis Enabled"]
    #[inline(always)]
    pub fn hys_1_hysteresis_enabled(self) -> &'a mut W {
        self.variant(HYS_A::HYS_1_HYSTERESIS_ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slew Rate Field"]
    #[inline(always)]
    pub fn sre(&self) -> SRE_R {
        SRE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Drive Strength Field"]
    #[inline(always)]
    pub fn dse(&self) -> DSE_R {
        DSE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Speed Field"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Open Drain Enable Field"]
    #[inline(always)]
    pub fn ode(&self) -> ODE_R {
        ODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pull / Keep Enable Field"]
    #[inline(always)]
    pub fn pke(&self) -> PKE_R {
        PKE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pull / Keep Select Field"]
    #[inline(always)]
    pub fn pue(&self) -> PUE_R {
        PUE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Pull Up / Down Config. Field"]
    #[inline(always)]
    pub fn pus(&self) -> PUS_R {
        PUS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Hyst. Enable Field"]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slew Rate Field"]
    #[inline(always)]
    pub fn sre(&mut self) -> SRE_W {
        SRE_W { w: self }
    }
    #[doc = "Bits 3:5 - Drive Strength Field"]
    #[inline(always)]
    pub fn dse(&mut self) -> DSE_W {
        DSE_W { w: self }
    }
    #[doc = "Bits 6:7 - Speed Field"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bit 11 - Open Drain Enable Field"]
    #[inline(always)]
    pub fn ode(&mut self) -> ODE_W {
        ODE_W { w: self }
    }
    #[doc = "Bit 12 - Pull / Keep Enable Field"]
    #[inline(always)]
    pub fn pke(&mut self) -> PKE_W {
        PKE_W { w: self }
    }
    #[doc = "Bit 13 - Pull / Keep Select Field"]
    #[inline(always)]
    pub fn pue(&mut self) -> PUE_W {
        PUE_W { w: self }
    }
    #[doc = "Bits 14:15 - Pull Up / Down Config. Field"]
    #[inline(always)]
    pub fn pus(&mut self) -> PUS_W {
        PUS_W { w: self }
    }
    #[doc = "Bit 16 - Hyst. Enable Field"]
    #[inline(always)]
    pub fn hys(&mut self) -> HYS_W {
        HYS_W { w: self }
    }
}
