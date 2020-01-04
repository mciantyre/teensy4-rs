#[doc = "Reader of register LPCR"]
pub type R = crate::R<u32, super::LPCR>;
#[doc = "Writer for register LPCR"]
pub type W = crate::W<u32, super::LPCR>;
#[doc = "Register LPCR `reset()`'s with value 0x20"]
impl crate::ResetValue for super::LPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTC_ENV_A {
    #[doc = "0: SRTC is disabled or invalid."]
    SRTC_ENV_0 = 0,
    #[doc = "1: SRTC is enabled and valid."]
    SRTC_ENV_1 = 1,
}
impl From<SRTC_ENV_A> for bool {
    #[inline(always)]
    fn from(variant: SRTC_ENV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRTC_ENV`"]
pub type SRTC_ENV_R = crate::R<bool, SRTC_ENV_A>;
impl SRTC_ENV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTC_ENV_A {
        match self.bits {
            false => SRTC_ENV_A::SRTC_ENV_0,
            true => SRTC_ENV_A::SRTC_ENV_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRTC_ENV_0`"]
    #[inline(always)]
    pub fn is_srtc_env_0(&self) -> bool {
        *self == SRTC_ENV_A::SRTC_ENV_0
    }
    #[doc = "Checks if the value of the field is `SRTC_ENV_1`"]
    #[inline(always)]
    pub fn is_srtc_env_1(&self) -> bool {
        *self == SRTC_ENV_A::SRTC_ENV_1
    }
}
#[doc = "Write proxy for field `SRTC_ENV`"]
pub struct SRTC_ENV_W<'a> {
    w: &'a mut W,
}
impl<'a> SRTC_ENV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRTC_ENV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRTC is disabled or invalid."]
    #[inline(always)]
    pub fn srtc_env_0(self) -> &'a mut W {
        self.variant(SRTC_ENV_A::SRTC_ENV_0)
    }
    #[doc = "SRTC is enabled and valid."]
    #[inline(always)]
    pub fn srtc_env_1(self) -> &'a mut W {
        self.variant(SRTC_ENV_A::SRTC_ENV_1)
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
#[doc = "LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTA_EN_A {
    #[doc = "0: LP time alarm interrupt is disabled."]
    LPTA_EN_0 = 0,
    #[doc = "1: LP time alarm interrupt is enabled."]
    LPTA_EN_1 = 1,
}
impl From<LPTA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTA_EN`"]
pub type LPTA_EN_R = crate::R<bool, LPTA_EN_A>;
impl LPTA_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTA_EN_A {
        match self.bits {
            false => LPTA_EN_A::LPTA_EN_0,
            true => LPTA_EN_A::LPTA_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPTA_EN_0`"]
    #[inline(always)]
    pub fn is_lpta_en_0(&self) -> bool {
        *self == LPTA_EN_A::LPTA_EN_0
    }
    #[doc = "Checks if the value of the field is `LPTA_EN_1`"]
    #[inline(always)]
    pub fn is_lpta_en_1(&self) -> bool {
        *self == LPTA_EN_A::LPTA_EN_1
    }
}
#[doc = "Write proxy for field `LPTA_EN`"]
pub struct LPTA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTA_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LP time alarm interrupt is disabled."]
    #[inline(always)]
    pub fn lpta_en_0(self) -> &'a mut W {
        self.variant(LPTA_EN_A::LPTA_EN_0)
    }
    #[doc = "LP time alarm interrupt is enabled."]
    #[inline(always)]
    pub fn lpta_en_1(self) -> &'a mut W {
        self.variant(LPTA_EN_A::LPTA_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MC_ENV_A {
    #[doc = "0: MC is disabled or invalid."]
    MC_ENV_0 = 0,
    #[doc = "1: MC is enabled and valid."]
    MC_ENV_1 = 1,
}
impl From<MC_ENV_A> for bool {
    #[inline(always)]
    fn from(variant: MC_ENV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MC_ENV`"]
pub type MC_ENV_R = crate::R<bool, MC_ENV_A>;
impl MC_ENV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MC_ENV_A {
        match self.bits {
            false => MC_ENV_A::MC_ENV_0,
            true => MC_ENV_A::MC_ENV_1,
        }
    }
    #[doc = "Checks if the value of the field is `MC_ENV_0`"]
    #[inline(always)]
    pub fn is_mc_env_0(&self) -> bool {
        *self == MC_ENV_A::MC_ENV_0
    }
    #[doc = "Checks if the value of the field is `MC_ENV_1`"]
    #[inline(always)]
    pub fn is_mc_env_1(&self) -> bool {
        *self == MC_ENV_A::MC_ENV_1
    }
}
#[doc = "Write proxy for field `MC_ENV`"]
pub struct MC_ENV_W<'a> {
    w: &'a mut W,
}
impl<'a> MC_ENV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MC_ENV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MC is disabled or invalid."]
    #[inline(always)]
    pub fn mc_env_0(self) -> &'a mut W {
        self.variant(MC_ENV_A::MC_ENV_0)
    }
    #[doc = "MC is enabled and valid."]
    #[inline(always)]
    pub fn mc_env_1(self) -> &'a mut W {
        self.variant(MC_ENV_A::MC_ENV_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `LPWUI_EN`"]
pub type LPWUI_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPWUI_EN`"]
pub struct LPWUI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPWUI_EN_W<'a> {
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
#[doc = "If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRTC_INV_EN_A {
    #[doc = "0: SRTC stays valid in the case of security violation."]
    SRTC_INV_EN_0 = 0,
    #[doc = "1: SRTC is invalidated in the case of security violation."]
    SRTC_INV_EN_1 = 1,
}
impl From<SRTC_INV_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SRTC_INV_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRTC_INV_EN`"]
pub type SRTC_INV_EN_R = crate::R<bool, SRTC_INV_EN_A>;
impl SRTC_INV_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRTC_INV_EN_A {
        match self.bits {
            false => SRTC_INV_EN_A::SRTC_INV_EN_0,
            true => SRTC_INV_EN_A::SRTC_INV_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRTC_INV_EN_0`"]
    #[inline(always)]
    pub fn is_srtc_inv_en_0(&self) -> bool {
        *self == SRTC_INV_EN_A::SRTC_INV_EN_0
    }
    #[doc = "Checks if the value of the field is `SRTC_INV_EN_1`"]
    #[inline(always)]
    pub fn is_srtc_inv_en_1(&self) -> bool {
        *self == SRTC_INV_EN_A::SRTC_INV_EN_1
    }
}
#[doc = "Write proxy for field `SRTC_INV_EN`"]
pub struct SRTC_INV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRTC_INV_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRTC_INV_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRTC stays valid in the case of security violation."]
    #[inline(always)]
    pub fn srtc_inv_en_0(self) -> &'a mut W {
        self.variant(SRTC_INV_EN_A::SRTC_INV_EN_0)
    }
    #[doc = "SRTC is invalidated in the case of security violation."]
    #[inline(always)]
    pub fn srtc_inv_en_1(self) -> &'a mut W {
        self.variant(SRTC_INV_EN_A::SRTC_INV_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Dumb PMIC Enabled When set, software can control the system power\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DP_EN_A {
    #[doc = "0: Smart PMIC enabled."]
    DP_EN_0 = 0,
    #[doc = "1: Dumb PMIC enabled."]
    DP_EN_1 = 1,
}
impl From<DP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DP_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DP_EN`"]
pub type DP_EN_R = crate::R<bool, DP_EN_A>;
impl DP_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DP_EN_A {
        match self.bits {
            false => DP_EN_A::DP_EN_0,
            true => DP_EN_A::DP_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DP_EN_0`"]
    #[inline(always)]
    pub fn is_dp_en_0(&self) -> bool {
        *self == DP_EN_A::DP_EN_0
    }
    #[doc = "Checks if the value of the field is `DP_EN_1`"]
    #[inline(always)]
    pub fn is_dp_en_1(&self) -> bool {
        *self == DP_EN_A::DP_EN_1
    }
}
#[doc = "Write proxy for field `DP_EN`"]
pub struct DP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DP_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Smart PMIC enabled."]
    #[inline(always)]
    pub fn dp_en_0(self) -> &'a mut W {
        self.variant(DP_EN_A::DP_EN_0)
    }
    #[doc = "Dumb PMIC enabled."]
    #[inline(always)]
    pub fn dp_en_1(self) -> &'a mut W {
        self.variant(DP_EN_A::DP_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOP_A {
    #[doc = "0: Leave system power on."]
    TOP_0 = 0,
    #[doc = "1: Turn off system power."]
    TOP_1 = 1,
}
impl From<TOP_A> for bool {
    #[inline(always)]
    fn from(variant: TOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TOP`"]
pub type TOP_R = crate::R<bool, TOP_A>;
impl TOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOP_A {
        match self.bits {
            false => TOP_A::TOP_0,
            true => TOP_A::TOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `TOP_0`"]
    #[inline(always)]
    pub fn is_top_0(&self) -> bool {
        *self == TOP_A::TOP_0
    }
    #[doc = "Checks if the value of the field is `TOP_1`"]
    #[inline(always)]
    pub fn is_top_1(&self) -> bool {
        *self == TOP_A::TOP_1
    }
}
#[doc = "Write proxy for field `TOP`"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Leave system power on."]
    #[inline(always)]
    pub fn top_0(self) -> &'a mut W {
        self.variant(TOP_A::TOP_0)
    }
    #[doc = "Turn off system power."]
    #[inline(always)]
    pub fn top_1(self) -> &'a mut W {
        self.variant(TOP_A::TOP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PWR_GLITCH_EN`"]
pub type PWR_GLITCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWR_GLITCH_EN`"]
pub struct PWR_GLITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_GLITCH_EN_W<'a> {
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
#[doc = "LP Calibration Enable When set, enables the SRTC calibration mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCALB_EN_A {
    #[doc = "0: SRTC Time calibration is disabled."]
    LPCALB_EN_0 = 0,
    #[doc = "1: SRTC Time calibration is enabled."]
    LPCALB_EN_1 = 1,
}
impl From<LPCALB_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPCALB_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPCALB_EN`"]
pub type LPCALB_EN_R = crate::R<bool, LPCALB_EN_A>;
impl LPCALB_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCALB_EN_A {
        match self.bits {
            false => LPCALB_EN_A::LPCALB_EN_0,
            true => LPCALB_EN_A::LPCALB_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPCALB_EN_0`"]
    #[inline(always)]
    pub fn is_lpcalb_en_0(&self) -> bool {
        *self == LPCALB_EN_A::LPCALB_EN_0
    }
    #[doc = "Checks if the value of the field is `LPCALB_EN_1`"]
    #[inline(always)]
    pub fn is_lpcalb_en_1(&self) -> bool {
        *self == LPCALB_EN_A::LPCALB_EN_1
    }
}
#[doc = "Write proxy for field `LPCALB_EN`"]
pub struct LPCALB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCALB_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCALB_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRTC Time calibration is disabled."]
    #[inline(always)]
    pub fn lpcalb_en_0(self) -> &'a mut W {
        self.variant(LPCALB_EN_A::LPCALB_EN_0)
    }
    #[doc = "SRTC Time calibration is enabled."]
    #[inline(always)]
    pub fn lpcalb_en_1(self) -> &'a mut W {
        self.variant(LPCALB_EN_A::LPCALB_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "LP Calibration Value Defines signed calibration value for SRTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPCALB_VAL_A {
    #[doc = "0: +0 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_0 = 0,
    #[doc = "1: +1 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_1 = 1,
    #[doc = "2: +2 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_2 = 2,
    #[doc = "15: +15 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_15 = 15,
    #[doc = "16: -16 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_16 = 16,
    #[doc = "17: -15 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_17 = 17,
    #[doc = "30: -2 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_30 = 30,
    #[doc = "31: -1 counts per each 32768 ticks of the counter clock"]
    LPCALB_VAL_31 = 31,
}
impl From<LPCALB_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCALB_VAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPCALB_VAL`"]
pub type LPCALB_VAL_R = crate::R<u8, LPCALB_VAL_A>;
impl LPCALB_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPCALB_VAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPCALB_VAL_A::LPCALB_VAL_0),
            1 => Val(LPCALB_VAL_A::LPCALB_VAL_1),
            2 => Val(LPCALB_VAL_A::LPCALB_VAL_2),
            15 => Val(LPCALB_VAL_A::LPCALB_VAL_15),
            16 => Val(LPCALB_VAL_A::LPCALB_VAL_16),
            17 => Val(LPCALB_VAL_A::LPCALB_VAL_17),
            30 => Val(LPCALB_VAL_A::LPCALB_VAL_30),
            31 => Val(LPCALB_VAL_A::LPCALB_VAL_31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_0`"]
    #[inline(always)]
    pub fn is_lpcalb_val_0(&self) -> bool {
        *self == LPCALB_VAL_A::LPCALB_VAL_0
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_1`"]
    #[inline(always)]
    pub fn is_lpcalb_val_1(&self) -> bool {
        *self == LPCALB_VAL_A::LPCALB_VAL_1
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_2`"]
    #[inline(always)]
    pub fn is_lpcalb_val_2(&self) -> bool {
        *self == LPCALB_VAL_A::LPCALB_VAL_2
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_15`"]
    #[inline(always)]
    pub fn is_lpcalb_val_15(&self) -> bool {
        *self == LPCALB_VAL_A::LPCALB_VAL_15
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_16`"]
    #[inline(always)]
    pub fn is_lpcalb_val_16(&self) -> bool {
        *self == LPCALB_VAL_A::LPCALB_VAL_16
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_17`"]
    #[inline(always)]
    pub fn is_lpcalb_val_17(&self) -> bool {
        *self == LPCALB_VAL_A::LPCALB_VAL_17
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_30`"]
    #[inline(always)]
    pub fn is_lpcalb_val_30(&self) -> bool {
        *self == LPCALB_VAL_A::LPCALB_VAL_30
    }
    #[doc = "Checks if the value of the field is `LPCALB_VAL_31`"]
    #[inline(always)]
    pub fn is_lpcalb_val_31(&self) -> bool {
        *self == LPCALB_VAL_A::LPCALB_VAL_31
    }
}
#[doc = "Write proxy for field `LPCALB_VAL`"]
pub struct LPCALB_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCALB_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCALB_VAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "+0 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn lpcalb_val_0(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::LPCALB_VAL_0)
    }
    #[doc = "+1 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn lpcalb_val_1(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::LPCALB_VAL_1)
    }
    #[doc = "+2 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn lpcalb_val_2(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::LPCALB_VAL_2)
    }
    #[doc = "+15 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn lpcalb_val_15(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::LPCALB_VAL_15)
    }
    #[doc = "-16 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn lpcalb_val_16(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::LPCALB_VAL_16)
    }
    #[doc = "-15 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn lpcalb_val_17(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::LPCALB_VAL_17)
    }
    #[doc = "-2 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn lpcalb_val_30(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::LPCALB_VAL_30)
    }
    #[doc = "-1 counts per each 32768 ticks of the counter clock"]
    #[inline(always)]
    pub fn lpcalb_val_31(self) -> &'a mut W {
        self.variant(LPCALB_VAL_A::LPCALB_VAL_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `BTN_PRESS_TIME`"]
pub type BTN_PRESS_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BTN_PRESS_TIME`"]
pub struct BTN_PRESS_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> BTN_PRESS_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `DEBOUNCE`"]
pub type DEBOUNCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEBOUNCE`"]
pub struct DEBOUNCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBOUNCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `ON_TIME`"]
pub type ON_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ON_TIME`"]
pub struct ON_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `PK_EN`"]
pub type PK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PK_EN`"]
pub struct PK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `PK_OVERRIDE`"]
pub type PK_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PK_OVERRIDE`"]
pub struct PK_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PK_OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `GPR_Z_DIS`"]
pub type GPR_Z_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPR_Z_DIS`"]
pub struct GPR_Z_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> GPR_Z_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
    #[inline(always)]
    pub fn srtc_env(&self) -> SRTC_ENV_R {
        SRTC_ENV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
    #[inline(always)]
    pub fn lpta_en(&self) -> LPTA_EN_R {
        LPTA_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
    #[inline(always)]
    pub fn mc_env(&self) -> MC_ENV_R {
        MC_ENV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (tamper event, MC rollover, SRTC rollover, or time alarm )"]
    #[inline(always)]
    pub fn lpwui_en(&self) -> LPWUI_EN_R {
        LPWUI_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
    #[inline(always)]
    pub fn srtc_inv_en(&self) -> SRTC_INV_EN_R {
        SRTC_INV_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Dumb PMIC Enabled When set, software can control the system power"]
    #[inline(always)]
    pub fn dp_en(&self) -> DP_EN_R {
        DP_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Power Glitch Enable By default the detection of a power glitch does not cause the pmic_en_b signal to be asserted"]
    #[inline(always)]
    pub fn pwr_glitch_en(&self) -> PWR_GLITCH_EN_R {
        PWR_GLITCH_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LP Calibration Enable When set, enables the SRTC calibration mechanism"]
    #[inline(always)]
    pub fn lpcalb_en(&self) -> LPCALB_EN_R {
        LPCALB_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:14 - LP Calibration Value Defines signed calibration value for SRTC"]
    #[inline(always)]
    pub fn lpcalb_val(&self) -> LPCALB_VAL_R {
        LPCALB_VAL_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - This field configures the button press time out values for the PMIC Logic"]
    #[inline(always)]
    pub fn btn_press_time(&self) -> BTN_PRESS_TIME_R {
        BTN_PRESS_TIME_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - This field configures the amount of debounce time for the BTN input signal"]
    #[inline(always)]
    pub fn debounce(&self) -> DEBOUNCE_R {
        DEBOUNCE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
    #[inline(always)]
    pub fn on_time(&self) -> ON_TIME_R {
        ON_TIME_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
    #[inline(always)]
    pub fn pk_en(&self) -> PK_EN_R {
        PK_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
    #[inline(always)]
    pub fn pk_override(&self) -> PK_OVERRIDE_R {
        PK_OVERRIDE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - General Purpose Registers Zeroization Disable"]
    #[inline(always)]
    pub fn gpr_z_dis(&self) -> GPR_Z_DIS_R {
        GPR_Z_DIS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Real Time Counter Enabled and Valid When set, the SRTC becomes operational"]
    #[inline(always)]
    pub fn srtc_env(&mut self) -> SRTC_ENV_W {
        SRTC_ENV_W { w: self }
    }
    #[doc = "Bit 1 - LP Time Alarm Enable When set, the SNVS functional interrupt is asserted if the LP Time Alarm Register is equal to the 32 MSBs of the secure real time counter"]
    #[inline(always)]
    pub fn lpta_en(&mut self) -> LPTA_EN_W {
        LPTA_EN_W { w: self }
    }
    #[doc = "Bit 2 - Monotonic Counter Enabled and Valid When set, the MC can be incremented (by write transaction to the LPSMCMR or LPSMCLR)"]
    #[inline(always)]
    pub fn mc_env(&mut self) -> MC_ENV_W {
        MC_ENV_W { w: self }
    }
    #[doc = "Bit 3 - LP Wake-Up Interrupt Enable This interrupt line should be connected to the external pin and is intended to inform the external chip about an SNVS_LP event (tamper event, MC rollover, SRTC rollover, or time alarm )"]
    #[inline(always)]
    pub fn lpwui_en(&mut self) -> LPWUI_EN_W {
        LPWUI_EN_W { w: self }
    }
    #[doc = "Bit 4 - If this bit is 1, in the case of a security violation the SRTC stops counting and the SRTC is invalidated (SRTC_ENV bit is cleared)"]
    #[inline(always)]
    pub fn srtc_inv_en(&mut self) -> SRTC_INV_EN_W {
        SRTC_INV_EN_W { w: self }
    }
    #[doc = "Bit 5 - Dumb PMIC Enabled When set, software can control the system power"]
    #[inline(always)]
    pub fn dp_en(&mut self) -> DP_EN_W {
        DP_EN_W { w: self }
    }
    #[doc = "Bit 6 - Turn off System Power Asserting this bit causes a signal to be sent to the Power Management IC to turn off the system power"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
    #[doc = "Bit 7 - Power Glitch Enable By default the detection of a power glitch does not cause the pmic_en_b signal to be asserted"]
    #[inline(always)]
    pub fn pwr_glitch_en(&mut self) -> PWR_GLITCH_EN_W {
        PWR_GLITCH_EN_W { w: self }
    }
    #[doc = "Bit 8 - LP Calibration Enable When set, enables the SRTC calibration mechanism"]
    #[inline(always)]
    pub fn lpcalb_en(&mut self) -> LPCALB_EN_W {
        LPCALB_EN_W { w: self }
    }
    #[doc = "Bits 10:14 - LP Calibration Value Defines signed calibration value for SRTC"]
    #[inline(always)]
    pub fn lpcalb_val(&mut self) -> LPCALB_VAL_W {
        LPCALB_VAL_W { w: self }
    }
    #[doc = "Bits 16:17 - This field configures the button press time out values for the PMIC Logic"]
    #[inline(always)]
    pub fn btn_press_time(&mut self) -> BTN_PRESS_TIME_W {
        BTN_PRESS_TIME_W { w: self }
    }
    #[doc = "Bits 18:19 - This field configures the amount of debounce time for the BTN input signal"]
    #[inline(always)]
    pub fn debounce(&mut self) -> DEBOUNCE_W {
        DEBOUNCE_W { w: self }
    }
    #[doc = "Bits 20:21 - The ON_TIME field is used to configure the period of time after BTN is asserted before pmic_en_b is asserted to turn on the SoC power"]
    #[inline(always)]
    pub fn on_time(&mut self) -> ON_TIME_W {
        ON_TIME_W { w: self }
    }
    #[doc = "Bit 22 - PMIC On Request Enable The value written to PK_EN will be asserted on output signal snvs_lp_pk_en"]
    #[inline(always)]
    pub fn pk_en(&mut self) -> PK_EN_W {
        PK_EN_W { w: self }
    }
    #[doc = "Bit 23 - PMIC On Request Override The value written to PK_OVERRIDE will be asserted on output signal snvs_lp_pk_override"]
    #[inline(always)]
    pub fn pk_override(&mut self) -> PK_OVERRIDE_W {
        PK_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 24 - General Purpose Registers Zeroization Disable"]
    #[inline(always)]
    pub fn gpr_z_dis(&mut self) -> GPR_Z_DIS_W {
        GPR_Z_DIS_W { w: self }
    }
}
