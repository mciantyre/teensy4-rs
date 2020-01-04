#[doc = "Reader of register MISC0"]
pub type R = crate::R<u32, super::MISC0>;
#[doc = "Writer for register MISC0"]
pub type W = crate::W<u32, super::MISC0>;
#[doc = "Register MISC0 `reset()`'s with value 0x0400_0000"]
impl crate::ResetValue for super::MISC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400_0000
    }
}
#[doc = "Reader of field `REFTOP_PWD`"]
pub type REFTOP_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFTOP_PWD`"]
pub struct REFTOP_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> REFTOP_PWD_W<'a> {
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
#[doc = "Control bit to disable the self-bias circuit in the analog bandgap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFTOP_SELFBIASOFF_A {
    #[doc = "0: Uses coarse bias currents for startup"]
    REFTOP_SELFBIASOFF_0 = 0,
    #[doc = "1: Uses bandgap-based bias currents for best performance."]
    REFTOP_SELFBIASOFF_1 = 1,
}
impl From<REFTOP_SELFBIASOFF_A> for bool {
    #[inline(always)]
    fn from(variant: REFTOP_SELFBIASOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REFTOP_SELFBIASOFF`"]
pub type REFTOP_SELFBIASOFF_R = crate::R<bool, REFTOP_SELFBIASOFF_A>;
impl REFTOP_SELFBIASOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFTOP_SELFBIASOFF_A {
        match self.bits {
            false => REFTOP_SELFBIASOFF_A::REFTOP_SELFBIASOFF_0,
            true => REFTOP_SELFBIASOFF_A::REFTOP_SELFBIASOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFTOP_SELFBIASOFF_0`"]
    #[inline(always)]
    pub fn is_reftop_selfbiasoff_0(&self) -> bool {
        *self == REFTOP_SELFBIASOFF_A::REFTOP_SELFBIASOFF_0
    }
    #[doc = "Checks if the value of the field is `REFTOP_SELFBIASOFF_1`"]
    #[inline(always)]
    pub fn is_reftop_selfbiasoff_1(&self) -> bool {
        *self == REFTOP_SELFBIASOFF_A::REFTOP_SELFBIASOFF_1
    }
}
#[doc = "Write proxy for field `REFTOP_SELFBIASOFF`"]
pub struct REFTOP_SELFBIASOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> REFTOP_SELFBIASOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFTOP_SELFBIASOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Uses coarse bias currents for startup"]
    #[inline(always)]
    pub fn reftop_selfbiasoff_0(self) -> &'a mut W {
        self.variant(REFTOP_SELFBIASOFF_A::REFTOP_SELFBIASOFF_0)
    }
    #[doc = "Uses bandgap-based bias currents for best performance."]
    #[inline(always)]
    pub fn reftop_selfbiasoff_1(self) -> &'a mut W {
        self.variant(REFTOP_SELFBIASOFF_A::REFTOP_SELFBIASOFF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFTOP_VBGADJ_A {
    #[doc = "0: Nominal VBG"]
    REFTOP_VBGADJ_0 = 0,
    #[doc = "1: VBG+0.78%"]
    REFTOP_VBGADJ_1 = 1,
    #[doc = "2: VBG+1.56%"]
    REFTOP_VBGADJ_2 = 2,
    #[doc = "3: VBG+2.34%"]
    REFTOP_VBGADJ_3 = 3,
    #[doc = "4: VBG-0.78%"]
    REFTOP_VBGADJ_4 = 4,
    #[doc = "5: VBG-1.56%"]
    REFTOP_VBGADJ_5 = 5,
    #[doc = "6: VBG-2.34%"]
    REFTOP_VBGADJ_6 = 6,
    #[doc = "7: VBG-3.12%"]
    REFTOP_VBGADJ_7 = 7,
}
impl From<REFTOP_VBGADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: REFTOP_VBGADJ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFTOP_VBGADJ`"]
pub type REFTOP_VBGADJ_R = crate::R<u8, REFTOP_VBGADJ_A>;
impl REFTOP_VBGADJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFTOP_VBGADJ_A {
        match self.bits {
            0 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_0,
            1 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_1,
            2 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_2,
            3 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_3,
            4 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_4,
            5 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_5,
            6 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_6,
            7 => REFTOP_VBGADJ_A::REFTOP_VBGADJ_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_0`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_0(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_0
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_1`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_1(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_1
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_2`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_2(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_2
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_3`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_3(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_3
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_4`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_4(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_4
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_5`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_5(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_5
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_6`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_6(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_6
    }
    #[doc = "Checks if the value of the field is `REFTOP_VBGADJ_7`"]
    #[inline(always)]
    pub fn is_reftop_vbgadj_7(&self) -> bool {
        *self == REFTOP_VBGADJ_A::REFTOP_VBGADJ_7
    }
}
#[doc = "Write proxy for field `REFTOP_VBGADJ`"]
pub struct REFTOP_VBGADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> REFTOP_VBGADJ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFTOP_VBGADJ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Nominal VBG"]
    #[inline(always)]
    pub fn reftop_vbgadj_0(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_0)
    }
    #[doc = "VBG+0.78%"]
    #[inline(always)]
    pub fn reftop_vbgadj_1(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_1)
    }
    #[doc = "VBG+1.56%"]
    #[inline(always)]
    pub fn reftop_vbgadj_2(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_2)
    }
    #[doc = "VBG+2.34%"]
    #[inline(always)]
    pub fn reftop_vbgadj_3(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_3)
    }
    #[doc = "VBG-0.78%"]
    #[inline(always)]
    pub fn reftop_vbgadj_4(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_4)
    }
    #[doc = "VBG-1.56%"]
    #[inline(always)]
    pub fn reftop_vbgadj_5(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_5)
    }
    #[doc = "VBG-2.34%"]
    #[inline(always)]
    pub fn reftop_vbgadj_6(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_6)
    }
    #[doc = "VBG-3.12%"]
    #[inline(always)]
    pub fn reftop_vbgadj_7(self) -> &'a mut W {
        self.variant(REFTOP_VBGADJ_A::REFTOP_VBGADJ_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `REFTOP_VBGUP`"]
pub type REFTOP_VBGUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFTOP_VBGUP`"]
pub struct REFTOP_VBGUP_W<'a> {
    w: &'a mut W,
}
impl<'a> REFTOP_VBGUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Configure the analog behavior in stop mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STOP_MODE_CONFIG_A {
    #[doc = "0: SUSPEND (DSM)"]
    STOP_MODE_CONFIG_0 = 0,
    #[doc = "1: Analog regulators are ON."]
    STANDBY = 1,
    #[doc = "2: STOP (lower power)"]
    STOP_MODE_CONFIG_2 = 2,
    #[doc = "3: STOP (very lower power)"]
    STOP_MODE_CONFIG_3 = 3,
}
impl From<STOP_MODE_CONFIG_A> for u8 {
    #[inline(always)]
    fn from(variant: STOP_MODE_CONFIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STOP_MODE_CONFIG`"]
pub type STOP_MODE_CONFIG_R = crate::R<u8, STOP_MODE_CONFIG_A>;
impl STOP_MODE_CONFIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_MODE_CONFIG_A {
        match self.bits {
            0 => STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_0,
            1 => STOP_MODE_CONFIG_A::STANDBY,
            2 => STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_2,
            3 => STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP_MODE_CONFIG_0`"]
    #[inline(always)]
    pub fn is_stop_mode_config_0(&self) -> bool {
        *self == STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_0
    }
    #[doc = "Checks if the value of the field is `STANDBY`"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == STOP_MODE_CONFIG_A::STANDBY
    }
    #[doc = "Checks if the value of the field is `STOP_MODE_CONFIG_2`"]
    #[inline(always)]
    pub fn is_stop_mode_config_2(&self) -> bool {
        *self == STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_2
    }
    #[doc = "Checks if the value of the field is `STOP_MODE_CONFIG_3`"]
    #[inline(always)]
    pub fn is_stop_mode_config_3(&self) -> bool {
        *self == STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_3
    }
}
#[doc = "Write proxy for field `STOP_MODE_CONFIG`"]
pub struct STOP_MODE_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_MODE_CONFIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_MODE_CONFIG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SUSPEND (DSM)"]
    #[inline(always)]
    pub fn stop_mode_config_0(self) -> &'a mut W {
        self.variant(STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_0)
    }
    #[doc = "Analog regulators are ON."]
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(STOP_MODE_CONFIG_A::STANDBY)
    }
    #[doc = "STOP (lower power)"]
    #[inline(always)]
    pub fn stop_mode_config_2(self) -> &'a mut W {
        self.variant(STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_2)
    }
    #[doc = "STOP (very lower power)"]
    #[inline(always)]
    pub fn stop_mode_config_3(self) -> &'a mut W {
        self.variant(STOP_MODE_CONFIG_A::STOP_MODE_CONFIG_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCON_HIGH_SNVS_A {
    #[doc = "0: Turn on the switch"]
    DISCON_HIGH_SNVS_0 = 0,
    #[doc = "1: Turn off the switch"]
    DISCON_HIGH_SNVS_1 = 1,
}
impl From<DISCON_HIGH_SNVS_A> for bool {
    #[inline(always)]
    fn from(variant: DISCON_HIGH_SNVS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISCON_HIGH_SNVS`"]
pub type DISCON_HIGH_SNVS_R = crate::R<bool, DISCON_HIGH_SNVS_A>;
impl DISCON_HIGH_SNVS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCON_HIGH_SNVS_A {
        match self.bits {
            false => DISCON_HIGH_SNVS_A::DISCON_HIGH_SNVS_0,
            true => DISCON_HIGH_SNVS_A::DISCON_HIGH_SNVS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISCON_HIGH_SNVS_0`"]
    #[inline(always)]
    pub fn is_discon_high_snvs_0(&self) -> bool {
        *self == DISCON_HIGH_SNVS_A::DISCON_HIGH_SNVS_0
    }
    #[doc = "Checks if the value of the field is `DISCON_HIGH_SNVS_1`"]
    #[inline(always)]
    pub fn is_discon_high_snvs_1(&self) -> bool {
        *self == DISCON_HIGH_SNVS_A::DISCON_HIGH_SNVS_1
    }
}
#[doc = "Write proxy for field `DISCON_HIGH_SNVS`"]
pub struct DISCON_HIGH_SNVS_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCON_HIGH_SNVS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISCON_HIGH_SNVS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Turn on the switch"]
    #[inline(always)]
    pub fn discon_high_snvs_0(self) -> &'a mut W {
        self.variant(DISCON_HIGH_SNVS_A::DISCON_HIGH_SNVS_0)
    }
    #[doc = "Turn off the switch"]
    #[inline(always)]
    pub fn discon_high_snvs_1(self) -> &'a mut W {
        self.variant(DISCON_HIGH_SNVS_A::DISCON_HIGH_SNVS_1)
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
#[doc = "This field determines the bias current in the 24MHz oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSC_I_A {
    #[doc = "0: Nominal"]
    NOMINAL = 0,
    #[doc = "1: Decrease current by 12.5%"]
    MINUS_12_5_PERCENT = 1,
    #[doc = "2: Decrease current by 25.0%"]
    MINUS_25_PERCENT = 2,
    #[doc = "3: Decrease current by 37.5%"]
    MINUS_37_5_PERCENT = 3,
}
impl From<OSC_I_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC_I_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSC_I`"]
pub type OSC_I_R = crate::R<u8, OSC_I_A>;
impl OSC_I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC_I_A {
        match self.bits {
            0 => OSC_I_A::NOMINAL,
            1 => OSC_I_A::MINUS_12_5_PERCENT,
            2 => OSC_I_A::MINUS_25_PERCENT,
            3 => OSC_I_A::MINUS_37_5_PERCENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOMINAL`"]
    #[inline(always)]
    pub fn is_nominal(&self) -> bool {
        *self == OSC_I_A::NOMINAL
    }
    #[doc = "Checks if the value of the field is `MINUS_12_5_PERCENT`"]
    #[inline(always)]
    pub fn is_minus_12_5_percent(&self) -> bool {
        *self == OSC_I_A::MINUS_12_5_PERCENT
    }
    #[doc = "Checks if the value of the field is `MINUS_25_PERCENT`"]
    #[inline(always)]
    pub fn is_minus_25_percent(&self) -> bool {
        *self == OSC_I_A::MINUS_25_PERCENT
    }
    #[doc = "Checks if the value of the field is `MINUS_37_5_PERCENT`"]
    #[inline(always)]
    pub fn is_minus_37_5_percent(&self) -> bool {
        *self == OSC_I_A::MINUS_37_5_PERCENT
    }
}
#[doc = "Write proxy for field `OSC_I`"]
pub struct OSC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSC_I_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Nominal"]
    #[inline(always)]
    pub fn nominal(self) -> &'a mut W {
        self.variant(OSC_I_A::NOMINAL)
    }
    #[doc = "Decrease current by 12.5%"]
    #[inline(always)]
    pub fn minus_12_5_percent(self) -> &'a mut W {
        self.variant(OSC_I_A::MINUS_12_5_PERCENT)
    }
    #[doc = "Decrease current by 25.0%"]
    #[inline(always)]
    pub fn minus_25_percent(self) -> &'a mut W {
        self.variant(OSC_I_A::MINUS_25_PERCENT)
    }
    #[doc = "Decrease current by 37.5%"]
    #[inline(always)]
    pub fn minus_37_5_percent(self) -> &'a mut W {
        self.variant(OSC_I_A::MINUS_37_5_PERCENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `OSC_XTALOK`"]
pub type OSC_XTALOK_R = crate::R<bool, bool>;
#[doc = "Reader of field `OSC_XTALOK_EN`"]
pub type OSC_XTALOK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSC_XTALOK_EN`"]
pub struct OSC_XTALOK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_XTALOK_EN_W<'a> {
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
#[doc = "This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKGATE_CTRL_A {
    #[doc = "0: Allow the logic to automatically gate the clock when the XTAL is powered down."]
    ALLOW_AUTO_GATE = 0,
    #[doc = "1: Prevent the logic from ever gating off the clock."]
    NO_AUTO_GATE = 1,
}
impl From<CLKGATE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKGATE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKGATE_CTRL`"]
pub type CLKGATE_CTRL_R = crate::R<bool, CLKGATE_CTRL_A>;
impl CLKGATE_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKGATE_CTRL_A {
        match self.bits {
            false => CLKGATE_CTRL_A::ALLOW_AUTO_GATE,
            true => CLKGATE_CTRL_A::NO_AUTO_GATE,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOW_AUTO_GATE`"]
    #[inline(always)]
    pub fn is_allow_auto_gate(&self) -> bool {
        *self == CLKGATE_CTRL_A::ALLOW_AUTO_GATE
    }
    #[doc = "Checks if the value of the field is `NO_AUTO_GATE`"]
    #[inline(always)]
    pub fn is_no_auto_gate(&self) -> bool {
        *self == CLKGATE_CTRL_A::NO_AUTO_GATE
    }
}
#[doc = "Write proxy for field `CLKGATE_CTRL`"]
pub struct CLKGATE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGATE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKGATE_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow the logic to automatically gate the clock when the XTAL is powered down."]
    #[inline(always)]
    pub fn allow_auto_gate(self) -> &'a mut W {
        self.variant(CLKGATE_CTRL_A::ALLOW_AUTO_GATE)
    }
    #[doc = "Prevent the logic from ever gating off the clock."]
    #[inline(always)]
    pub fn no_auto_gate(self) -> &'a mut W {
        self.variant(CLKGATE_CTRL_A::NO_AUTO_GATE)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKGATE_DELAY_A {
    #[doc = "0: 0.5ms"]
    CLKGATE_DELAY_0 = 0,
    #[doc = "1: 1.0ms"]
    CLKGATE_DELAY_1 = 1,
    #[doc = "2: 2.0ms"]
    CLKGATE_DELAY_2 = 2,
    #[doc = "3: 3.0ms"]
    CLKGATE_DELAY_3 = 3,
    #[doc = "4: 4.0ms"]
    CLKGATE_DELAY_4 = 4,
    #[doc = "5: 5.0ms"]
    CLKGATE_DELAY_5 = 5,
    #[doc = "6: 6.0ms"]
    CLKGATE_DELAY_6 = 6,
    #[doc = "7: 7.0ms"]
    CLKGATE_DELAY_7 = 7,
}
impl From<CLKGATE_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKGATE_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKGATE_DELAY`"]
pub type CLKGATE_DELAY_R = crate::R<u8, CLKGATE_DELAY_A>;
impl CLKGATE_DELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKGATE_DELAY_A {
        match self.bits {
            0 => CLKGATE_DELAY_A::CLKGATE_DELAY_0,
            1 => CLKGATE_DELAY_A::CLKGATE_DELAY_1,
            2 => CLKGATE_DELAY_A::CLKGATE_DELAY_2,
            3 => CLKGATE_DELAY_A::CLKGATE_DELAY_3,
            4 => CLKGATE_DELAY_A::CLKGATE_DELAY_4,
            5 => CLKGATE_DELAY_A::CLKGATE_DELAY_5,
            6 => CLKGATE_DELAY_A::CLKGATE_DELAY_6,
            7 => CLKGATE_DELAY_A::CLKGATE_DELAY_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_0`"]
    #[inline(always)]
    pub fn is_clkgate_delay_0(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_0
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_1`"]
    #[inline(always)]
    pub fn is_clkgate_delay_1(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_1
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_2`"]
    #[inline(always)]
    pub fn is_clkgate_delay_2(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_2
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_3`"]
    #[inline(always)]
    pub fn is_clkgate_delay_3(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_3
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_4`"]
    #[inline(always)]
    pub fn is_clkgate_delay_4(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_4
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_5`"]
    #[inline(always)]
    pub fn is_clkgate_delay_5(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_5
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_6`"]
    #[inline(always)]
    pub fn is_clkgate_delay_6(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_6
    }
    #[doc = "Checks if the value of the field is `CLKGATE_DELAY_7`"]
    #[inline(always)]
    pub fn is_clkgate_delay_7(&self) -> bool {
        *self == CLKGATE_DELAY_A::CLKGATE_DELAY_7
    }
}
#[doc = "Write proxy for field `CLKGATE_DELAY`"]
pub struct CLKGATE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGATE_DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKGATE_DELAY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "0.5ms"]
    #[inline(always)]
    pub fn clkgate_delay_0(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_0)
    }
    #[doc = "1.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_1(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_1)
    }
    #[doc = "2.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_2(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_2)
    }
    #[doc = "3.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_3(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_3)
    }
    #[doc = "4.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_4(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_4)
    }
    #[doc = "5.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_5(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_5)
    }
    #[doc = "6.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_6(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_6)
    }
    #[doc = "7.0ms"]
    #[inline(always)]
    pub fn clkgate_delay_7(self) -> &'a mut W {
        self.variant(CLKGATE_DELAY_A::CLKGATE_DELAY_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "This field indicates which chip source is being used for the rtc clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_XTAL_SOURCE_A {
    #[doc = "0: Internal ring oscillator"]
    RTC_XTAL_SOURCE_0 = 0,
    #[doc = "1: RTC_XTAL"]
    RTC_XTAL_SOURCE_1 = 1,
}
impl From<RTC_XTAL_SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_XTAL_SOURCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC_XTAL_SOURCE`"]
pub type RTC_XTAL_SOURCE_R = crate::R<bool, RTC_XTAL_SOURCE_A>;
impl RTC_XTAL_SOURCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_XTAL_SOURCE_A {
        match self.bits {
            false => RTC_XTAL_SOURCE_A::RTC_XTAL_SOURCE_0,
            true => RTC_XTAL_SOURCE_A::RTC_XTAL_SOURCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTC_XTAL_SOURCE_0`"]
    #[inline(always)]
    pub fn is_rtc_xtal_source_0(&self) -> bool {
        *self == RTC_XTAL_SOURCE_A::RTC_XTAL_SOURCE_0
    }
    #[doc = "Checks if the value of the field is `RTC_XTAL_SOURCE_1`"]
    #[inline(always)]
    pub fn is_rtc_xtal_source_1(&self) -> bool {
        *self == RTC_XTAL_SOURCE_A::RTC_XTAL_SOURCE_1
    }
}
#[doc = "Reader of field `XTAL_24M_PWD`"]
pub type XTAL_24M_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTAL_24M_PWD`"]
pub struct XTAL_24M_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_24M_PWD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Predivider for the source clock of the PLL's.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VID_PLL_PREDIV_A {
    #[doc = "0: Divide by 1"]
    VID_PLL_PREDIV_0 = 0,
    #[doc = "1: Divide by 2"]
    VID_PLL_PREDIV_1 = 1,
}
impl From<VID_PLL_PREDIV_A> for bool {
    #[inline(always)]
    fn from(variant: VID_PLL_PREDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VID_PLL_PREDIV`"]
pub type VID_PLL_PREDIV_R = crate::R<bool, VID_PLL_PREDIV_A>;
impl VID_PLL_PREDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VID_PLL_PREDIV_A {
        match self.bits {
            false => VID_PLL_PREDIV_A::VID_PLL_PREDIV_0,
            true => VID_PLL_PREDIV_A::VID_PLL_PREDIV_1,
        }
    }
    #[doc = "Checks if the value of the field is `VID_PLL_PREDIV_0`"]
    #[inline(always)]
    pub fn is_vid_pll_prediv_0(&self) -> bool {
        *self == VID_PLL_PREDIV_A::VID_PLL_PREDIV_0
    }
    #[doc = "Checks if the value of the field is `VID_PLL_PREDIV_1`"]
    #[inline(always)]
    pub fn is_vid_pll_prediv_1(&self) -> bool {
        *self == VID_PLL_PREDIV_A::VID_PLL_PREDIV_1
    }
}
#[doc = "Write proxy for field `VID_PLL_PREDIV`"]
pub struct VID_PLL_PREDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> VID_PLL_PREDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VID_PLL_PREDIV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn vid_pll_prediv_0(self) -> &'a mut W {
        self.variant(VID_PLL_PREDIV_A::VID_PLL_PREDIV_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn vid_pll_prediv_1(self) -> &'a mut W {
        self.variant(VID_PLL_PREDIV_A::VID_PLL_PREDIV_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub fn reftop_pwd(&self) -> REFTOP_PWD_R {
        REFTOP_PWD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub fn reftop_selfbiasoff(&self) -> REFTOP_SELFBIASOFF_R {
        REFTOP_SELFBIASOFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - no description available"]
    #[inline(always)]
    pub fn reftop_vbgadj(&self) -> REFTOP_VBGADJ_R {
        REFTOP_VBGADJ_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[inline(always)]
    pub fn reftop_vbgup(&self) -> REFTOP_VBGUP_R {
        REFTOP_VBGUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub fn stop_mode_config(&self) -> STOP_MODE_CONFIG_R {
        STOP_MODE_CONFIG_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub fn discon_high_snvs(&self) -> DISCON_HIGH_SNVS_R {
        DISCON_HIGH_SNVS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub fn osc_i(&self) -> OSC_I_R {
        OSC_I_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Status bit that signals that the output of the 24-MHz crystal oscillator is stable"]
    #[inline(always)]
    pub fn osc_xtalok(&self) -> OSC_XTALOK_R {
        OSC_XTALOK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub fn osc_xtalok_en(&self) -> OSC_XTALOK_EN_R {
        OSC_XTALOK_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 25 - This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub fn clkgate_ctrl(&self) -> CLKGATE_CTRL_R {
        CLKGATE_CTRL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:28 - This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub fn clkgate_delay(&self) -> CLKGATE_DELAY_R {
        CLKGATE_DELAY_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bit 29 - This field indicates which chip source is being used for the rtc clock."]
    #[inline(always)]
    pub fn rtc_xtal_source(&self) -> RTC_XTAL_SOURCE_R {
        RTC_XTAL_SOURCE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub fn xtal_24m_pwd(&self) -> XTAL_24M_PWD_R {
        XTAL_24M_PWD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Predivider for the source clock of the PLL's."]
    #[inline(always)]
    pub fn vid_pll_prediv(&self) -> VID_PLL_PREDIV_R {
        VID_PLL_PREDIV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control bit to power-down the analog bandgap reference circuitry"]
    #[inline(always)]
    pub fn reftop_pwd(&mut self) -> REFTOP_PWD_W {
        REFTOP_PWD_W { w: self }
    }
    #[doc = "Bit 3 - Control bit to disable the self-bias circuit in the analog bandgap"]
    #[inline(always)]
    pub fn reftop_selfbiasoff(&mut self) -> REFTOP_SELFBIASOFF_W {
        REFTOP_SELFBIASOFF_W { w: self }
    }
    #[doc = "Bits 4:6 - no description available"]
    #[inline(always)]
    pub fn reftop_vbgadj(&mut self) -> REFTOP_VBGADJ_W {
        REFTOP_VBGADJ_W { w: self }
    }
    #[doc = "Bit 7 - Status bit that signals the analog bandgap voltage is up and stable. 1 - Stable."]
    #[inline(always)]
    pub fn reftop_vbgup(&mut self) -> REFTOP_VBGUP_W {
        REFTOP_VBGUP_W { w: self }
    }
    #[doc = "Bits 10:11 - Configure the analog behavior in stop mode."]
    #[inline(always)]
    pub fn stop_mode_config(&mut self) -> STOP_MODE_CONFIG_W {
        STOP_MODE_CONFIG_W { w: self }
    }
    #[doc = "Bit 12 - This bit controls a switch from VDD_HIGH_IN to VDD_SNVS_IN."]
    #[inline(always)]
    pub fn discon_high_snvs(&mut self) -> DISCON_HIGH_SNVS_W {
        DISCON_HIGH_SNVS_W { w: self }
    }
    #[doc = "Bits 13:14 - This field determines the bias current in the 24MHz oscillator"]
    #[inline(always)]
    pub fn osc_i(&mut self) -> OSC_I_W {
        OSC_I_W { w: self }
    }
    #[doc = "Bit 16 - This bit enables the detector that signals when the 24MHz crystal oscillator is stable"]
    #[inline(always)]
    pub fn osc_xtalok_en(&mut self) -> OSC_XTALOK_EN_W {
        OSC_XTALOK_EN_W { w: self }
    }
    #[doc = "Bit 25 - This bit allows disabling the clock gate (always ungated) for the xtal 24MHz clock that clocks the digital logic in the analog block"]
    #[inline(always)]
    pub fn clkgate_ctrl(&mut self) -> CLKGATE_CTRL_W {
        CLKGATE_CTRL_W { w: self }
    }
    #[doc = "Bits 26:28 - This field specifies the delay between powering up the XTAL 24MHz clock and releasing the clock to the digital logic inside the analog block"]
    #[inline(always)]
    pub fn clkgate_delay(&mut self) -> CLKGATE_DELAY_W {
        CLKGATE_DELAY_W { w: self }
    }
    #[doc = "Bit 30 - This field powers down the 24M crystal oscillator if set true."]
    #[inline(always)]
    pub fn xtal_24m_pwd(&mut self) -> XTAL_24M_PWD_W {
        XTAL_24M_PWD_W { w: self }
    }
    #[doc = "Bit 31 - Predivider for the source clock of the PLL's."]
    #[inline(always)]
    pub fn vid_pll_prediv(&mut self) -> VID_PLL_PREDIV_W {
        VID_PLL_PREDIV_W { w: self }
    }
}
