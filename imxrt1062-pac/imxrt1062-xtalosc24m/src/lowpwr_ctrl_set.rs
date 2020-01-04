#[doc = "Reader of register LOWPWR_CTRL_SET"]
pub type R = crate::R<u32, super::LOWPWR_CTRL_SET>;
#[doc = "Writer for register LOWPWR_CTRL_SET"]
pub type W = crate::W<u32, super::LOWPWR_CTRL_SET>;
#[doc = "Register LOWPWR_CTRL_SET `reset()`'s with value 0x4001"]
impl crate::ResetValue for super::LOWPWR_CTRL_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4001
    }
}
#[doc = "RC Osc. enable control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RC_OSC_EN_A {
    #[doc = "0: Use XTAL OSC to source the 24MHz clock"]
    RC_OSC_EN_0 = 0,
    #[doc = "1: Use RC OSC"]
    RC_OSC_EN_1 = 1,
}
impl From<RC_OSC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RC_OSC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RC_OSC_EN`"]
pub type RC_OSC_EN_R = crate::R<bool, RC_OSC_EN_A>;
impl RC_OSC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RC_OSC_EN_A {
        match self.bits {
            false => RC_OSC_EN_A::RC_OSC_EN_0,
            true => RC_OSC_EN_A::RC_OSC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RC_OSC_EN_0`"]
    #[inline(always)]
    pub fn is_rc_osc_en_0(&self) -> bool {
        *self == RC_OSC_EN_A::RC_OSC_EN_0
    }
    #[doc = "Checks if the value of the field is `RC_OSC_EN_1`"]
    #[inline(always)]
    pub fn is_rc_osc_en_1(&self) -> bool {
        *self == RC_OSC_EN_A::RC_OSC_EN_1
    }
}
#[doc = "Write proxy for field `RC_OSC_EN`"]
pub struct RC_OSC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RC_OSC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RC_OSC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use XTAL OSC to source the 24MHz clock"]
    #[inline(always)]
    pub fn rc_osc_en_0(self) -> &'a mut W {
        self.variant(RC_OSC_EN_A::RC_OSC_EN_0)
    }
    #[doc = "Use RC OSC"]
    #[inline(always)]
    pub fn rc_osc_en_1(self) -> &'a mut W {
        self.variant(RC_OSC_EN_A::RC_OSC_EN_1)
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
#[doc = "Select the source for the 24MHz clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_SEL_A {
    #[doc = "0: XTAL OSC"]
    OSC_SEL_0 = 0,
    #[doc = "1: RC OSC"]
    OSC_SEL_1 = 1,
}
impl From<OSC_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: OSC_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSC_SEL`"]
pub type OSC_SEL_R = crate::R<bool, OSC_SEL_A>;
impl OSC_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC_SEL_A {
        match self.bits {
            false => OSC_SEL_A::OSC_SEL_0,
            true => OSC_SEL_A::OSC_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `OSC_SEL_0`"]
    #[inline(always)]
    pub fn is_osc_sel_0(&self) -> bool {
        *self == OSC_SEL_A::OSC_SEL_0
    }
    #[doc = "Checks if the value of the field is `OSC_SEL_1`"]
    #[inline(always)]
    pub fn is_osc_sel_1(&self) -> bool {
        *self == OSC_SEL_A::OSC_SEL_1
    }
}
#[doc = "Write proxy for field `OSC_SEL`"]
pub struct OSC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSC_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XTAL OSC"]
    #[inline(always)]
    pub fn osc_sel_0(self) -> &'a mut W {
        self.variant(OSC_SEL_A::OSC_SEL_0)
    }
    #[doc = "RC OSC"]
    #[inline(always)]
    pub fn osc_sel_1(self) -> &'a mut W {
        self.variant(OSC_SEL_A::OSC_SEL_1)
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
#[doc = "Bandgap select. Not related to oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBG_SEL_A {
    #[doc = "0: Normal power bandgap"]
    LPBG_SEL_0 = 0,
    #[doc = "1: Low power bandgap"]
    LPBG_SEL_1 = 1,
}
impl From<LPBG_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: LPBG_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPBG_SEL`"]
pub type LPBG_SEL_R = crate::R<bool, LPBG_SEL_A>;
impl LPBG_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPBG_SEL_A {
        match self.bits {
            false => LPBG_SEL_A::LPBG_SEL_0,
            true => LPBG_SEL_A::LPBG_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPBG_SEL_0`"]
    #[inline(always)]
    pub fn is_lpbg_sel_0(&self) -> bool {
        *self == LPBG_SEL_A::LPBG_SEL_0
    }
    #[doc = "Checks if the value of the field is `LPBG_SEL_1`"]
    #[inline(always)]
    pub fn is_lpbg_sel_1(&self) -> bool {
        *self == LPBG_SEL_A::LPBG_SEL_1
    }
}
#[doc = "Write proxy for field `LPBG_SEL`"]
pub struct LPBG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBG_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPBG_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal power bandgap"]
    #[inline(always)]
    pub fn lpbg_sel_0(self) -> &'a mut W {
        self.variant(LPBG_SEL_A::LPBG_SEL_0)
    }
    #[doc = "Low power bandgap"]
    #[inline(always)]
    pub fn lpbg_sel_1(self) -> &'a mut W {
        self.variant(LPBG_SEL_A::LPBG_SEL_1)
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
#[doc = "Reader of field `LPBG_TEST`"]
pub type LPBG_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPBG_TEST`"]
pub struct LPBG_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBG_TEST_W<'a> {
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
#[doc = "Reader of field `REFTOP_IBIAS_OFF`"]
pub type REFTOP_IBIAS_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFTOP_IBIAS_OFF`"]
pub struct REFTOP_IBIAS_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> REFTOP_IBIAS_OFF_W<'a> {
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
#[doc = "Reader of field `L1_PWRGATE`"]
pub type L1_PWRGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L1_PWRGATE`"]
pub struct L1_PWRGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> L1_PWRGATE_W<'a> {
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
#[doc = "Reader of field `L2_PWRGATE`"]
pub type L2_PWRGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `L2_PWRGATE`"]
pub struct L2_PWRGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> L2_PWRGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CPU_PWRGATE`"]
pub type CPU_PWRGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPU_PWRGATE`"]
pub struct CPU_PWRGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_PWRGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DISPLAY_PWRGATE`"]
pub type DISPLAY_PWRGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISPLAY_PWRGATE`"]
pub struct DISPLAY_PWRGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISPLAY_PWRGATE_W<'a> {
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
#[doc = "Reader of field `RCOSC_CG_OVERRIDE`"]
pub type RCOSC_CG_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSC_CG_OVERRIDE`"]
pub struct RCOSC_CG_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_CG_OVERRIDE_W<'a> {
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
#[doc = "Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XTALOSC_PWRUP_DELAY_A {
    #[doc = "0: 0.25ms"]
    XTALOSC_PWRUP_DELAY_0 = 0,
    #[doc = "1: 0.5ms"]
    XTALOSC_PWRUP_DELAY_1 = 1,
    #[doc = "2: 1ms"]
    XTALOSC_PWRUP_DELAY_2 = 2,
    #[doc = "3: 2ms"]
    XTALOSC_PWRUP_DELAY_3 = 3,
}
impl From<XTALOSC_PWRUP_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: XTALOSC_PWRUP_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XTALOSC_PWRUP_DELAY`"]
pub type XTALOSC_PWRUP_DELAY_R = crate::R<u8, XTALOSC_PWRUP_DELAY_A>;
impl XTALOSC_PWRUP_DELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTALOSC_PWRUP_DELAY_A {
        match self.bits {
            0 => XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_0,
            1 => XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_1,
            2 => XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_2,
            3 => XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_DELAY_0`"]
    #[inline(always)]
    pub fn is_xtalosc_pwrup_delay_0(&self) -> bool {
        *self == XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_0
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_DELAY_1`"]
    #[inline(always)]
    pub fn is_xtalosc_pwrup_delay_1(&self) -> bool {
        *self == XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_1
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_DELAY_2`"]
    #[inline(always)]
    pub fn is_xtalosc_pwrup_delay_2(&self) -> bool {
        *self == XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_2
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_DELAY_3`"]
    #[inline(always)]
    pub fn is_xtalosc_pwrup_delay_3(&self) -> bool {
        *self == XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_3
    }
}
#[doc = "Write proxy for field `XTALOSC_PWRUP_DELAY`"]
pub struct XTALOSC_PWRUP_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALOSC_PWRUP_DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTALOSC_PWRUP_DELAY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "0.25ms"]
    #[inline(always)]
    pub fn xtalosc_pwrup_delay_0(self) -> &'a mut W {
        self.variant(XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_0)
    }
    #[doc = "0.5ms"]
    #[inline(always)]
    pub fn xtalosc_pwrup_delay_1(self) -> &'a mut W {
        self.variant(XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_1)
    }
    #[doc = "1ms"]
    #[inline(always)]
    pub fn xtalosc_pwrup_delay_2(self) -> &'a mut W {
        self.variant(XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_2)
    }
    #[doc = "2ms"]
    #[inline(always)]
    pub fn xtalosc_pwrup_delay_3(self) -> &'a mut W {
        self.variant(XTALOSC_PWRUP_DELAY_A::XTALOSC_PWRUP_DELAY_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Status of the 24MHz xtal oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALOSC_PWRUP_STAT_A {
    #[doc = "0: Not stable"]
    XTALOSC_PWRUP_STAT_0 = 0,
    #[doc = "1: Stable and ready to use"]
    XTALOSC_PWRUP_STAT_1 = 1,
}
impl From<XTALOSC_PWRUP_STAT_A> for bool {
    #[inline(always)]
    fn from(variant: XTALOSC_PWRUP_STAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTALOSC_PWRUP_STAT`"]
pub type XTALOSC_PWRUP_STAT_R = crate::R<bool, XTALOSC_PWRUP_STAT_A>;
impl XTALOSC_PWRUP_STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTALOSC_PWRUP_STAT_A {
        match self.bits {
            false => XTALOSC_PWRUP_STAT_A::XTALOSC_PWRUP_STAT_0,
            true => XTALOSC_PWRUP_STAT_A::XTALOSC_PWRUP_STAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_STAT_0`"]
    #[inline(always)]
    pub fn is_xtalosc_pwrup_stat_0(&self) -> bool {
        *self == XTALOSC_PWRUP_STAT_A::XTALOSC_PWRUP_STAT_0
    }
    #[doc = "Checks if the value of the field is `XTALOSC_PWRUP_STAT_1`"]
    #[inline(always)]
    pub fn is_xtalosc_pwrup_stat_1(&self) -> bool {
        *self == XTALOSC_PWRUP_STAT_A::XTALOSC_PWRUP_STAT_1
    }
}
#[doc = "Reader of field `MIX_PWRGATE`"]
pub type MIX_PWRGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIX_PWRGATE`"]
pub struct MIX_PWRGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> MIX_PWRGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `GPU_PWRGATE`"]
pub type GPU_PWRGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPU_PWRGATE`"]
pub struct GPU_PWRGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPU_PWRGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RC Osc. enable control."]
    #[inline(always)]
    pub fn rc_osc_en(&self) -> RC_OSC_EN_R {
        RC_OSC_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Select the source for the 24MHz clock."]
    #[inline(always)]
    pub fn osc_sel(&self) -> OSC_SEL_R {
        OSC_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub fn lpbg_sel(&self) -> LPBG_SEL_R {
        LPBG_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub fn lpbg_test(&self) -> LPBG_TEST_R {
        LPBG_TEST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub fn reftop_ibias_off(&self) -> REFTOP_IBIAS_OFF_R {
        REFTOP_IBIAS_OFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub fn l1_pwrgate(&self) -> L1_PWRGATE_R {
        L1_PWRGATE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub fn l2_pwrgate(&self) -> L2_PWRGATE_R {
        L2_PWRGATE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub fn cpu_pwrgate(&self) -> CPU_PWRGATE_R {
        CPU_PWRGATE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub fn display_pwrgate(&self) -> DISPLAY_PWRGATE_R {
        DISPLAY_PWRGATE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - For debug purposes only"]
    #[inline(always)]
    pub fn rcosc_cg_override(&self) -> RCOSC_CG_OVERRIDE_R {
        RCOSC_CG_OVERRIDE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub fn xtalosc_pwrup_delay(&self) -> XTALOSC_PWRUP_DELAY_R {
        XTALOSC_PWRUP_DELAY_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Status of the 24MHz xtal oscillator."]
    #[inline(always)]
    pub fn xtalosc_pwrup_stat(&self) -> XTALOSC_PWRUP_STAT_R {
        XTALOSC_PWRUP_STAT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub fn mix_pwrgate(&self) -> MIX_PWRGATE_R {
        MIX_PWRGATE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub fn gpu_pwrgate(&self) -> GPU_PWRGATE_R {
        GPU_PWRGATE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RC Osc. enable control."]
    #[inline(always)]
    pub fn rc_osc_en(&mut self) -> RC_OSC_EN_W {
        RC_OSC_EN_W { w: self }
    }
    #[doc = "Bit 4 - Select the source for the 24MHz clock."]
    #[inline(always)]
    pub fn osc_sel(&mut self) -> OSC_SEL_W {
        OSC_SEL_W { w: self }
    }
    #[doc = "Bit 5 - Bandgap select. Not related to oscillator."]
    #[inline(always)]
    pub fn lpbg_sel(&mut self) -> LPBG_SEL_W {
        LPBG_SEL_W { w: self }
    }
    #[doc = "Bit 6 - Low power bandgap test bit. Not related to oscillator."]
    #[inline(always)]
    pub fn lpbg_test(&mut self) -> LPBG_TEST_W {
        LPBG_TEST_W { w: self }
    }
    #[doc = "Bit 7 - Low power reftop ibias disable. Not related to oscillator."]
    #[inline(always)]
    pub fn reftop_ibias_off(&mut self) -> REFTOP_IBIAS_OFF_W {
        REFTOP_IBIAS_OFF_W { w: self }
    }
    #[doc = "Bit 8 - L1 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub fn l1_pwrgate(&mut self) -> L1_PWRGATE_W {
        L1_PWRGATE_W { w: self }
    }
    #[doc = "Bit 9 - L2 power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub fn l2_pwrgate(&mut self) -> L2_PWRGATE_W {
        L2_PWRGATE_W { w: self }
    }
    #[doc = "Bit 10 - CPU power gate control. Used as software override. Test purpose only Not related to oscillator."]
    #[inline(always)]
    pub fn cpu_pwrgate(&mut self) -> CPU_PWRGATE_W {
        CPU_PWRGATE_W { w: self }
    }
    #[doc = "Bit 11 - Display logic power gate control. Used as software override. Not related to oscillator."]
    #[inline(always)]
    pub fn display_pwrgate(&mut self) -> DISPLAY_PWRGATE_W {
        DISPLAY_PWRGATE_W { w: self }
    }
    #[doc = "Bit 13 - For debug purposes only"]
    #[inline(always)]
    pub fn rcosc_cg_override(&mut self) -> RCOSC_CG_OVERRIDE_W {
        RCOSC_CG_OVERRIDE_W { w: self }
    }
    #[doc = "Bits 14:15 - Specifies the time delay between when the 24MHz xtal is powered up until it is stable and ready to use"]
    #[inline(always)]
    pub fn xtalosc_pwrup_delay(&mut self) -> XTALOSC_PWRUP_DELAY_W {
        XTALOSC_PWRUP_DELAY_W { w: self }
    }
    #[doc = "Bit 17 - Display power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub fn mix_pwrgate(&mut self) -> MIX_PWRGATE_W {
        MIX_PWRGATE_W { w: self }
    }
    #[doc = "Bit 18 - GPU power gate control. Used as software mask. Set to zero to force ungated."]
    #[inline(always)]
    pub fn gpu_pwrgate(&mut self) -> GPU_PWRGATE_W {
        GPU_PWRGATE_W { w: self }
    }
}
