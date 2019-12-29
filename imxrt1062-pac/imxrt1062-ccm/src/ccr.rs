#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0x0401_107f"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0401_107f
    }
}
#[doc = "Reader of field `OSCNT`"]
pub type OSCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSCNT`"]
pub struct OSCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "On chip oscillator enable bit - this bit value is reflected on the output cosc_en\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSC_EN_A {
    #[doc = "0: disable on chip oscillator"]
    COSC_EN_0,
    #[doc = "1: enable on chip oscillator"]
    COSC_EN_1,
}
impl From<COSC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COSC_EN_A) -> Self {
        match variant {
            COSC_EN_A::COSC_EN_0 => false,
            COSC_EN_A::COSC_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `COSC_EN`"]
pub type COSC_EN_R = crate::R<bool, COSC_EN_A>;
impl COSC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COSC_EN_A {
        match self.bits {
            false => COSC_EN_A::COSC_EN_0,
            true => COSC_EN_A::COSC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_EN_0`"]
    #[inline(always)]
    pub fn is_cosc_en_0(&self) -> bool {
        *self == COSC_EN_A::COSC_EN_0
    }
    #[doc = "Checks if the value of the field is `COSC_EN_1`"]
    #[inline(always)]
    pub fn is_cosc_en_1(&self) -> bool {
        *self == COSC_EN_A::COSC_EN_1
    }
}
#[doc = "Write proxy for field `COSC_EN`"]
pub struct COSC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COSC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COSC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disable on chip oscillator"]
    #[inline(always)]
    pub fn cosc_en_0(self) -> &'a mut W {
        self.variant(COSC_EN_A::COSC_EN_0)
    }
    #[doc = "enable on chip oscillator"]
    #[inline(always)]
    pub fn cosc_en_1(self) -> &'a mut W {
        self.variant(COSC_EN_A::COSC_EN_1)
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
#[doc = "Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ\n\nValue on reset: 32"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_BYPASS_COUNT_A {
    #[doc = "0: no delay"]
    REG_BYPASS_COUNT_0,
    #[doc = "1: 1 CKIL clock period delay"]
    REG_BYPASS_COUNT_1,
    #[doc = "63: 63 CKIL clock periods delay"]
    REG_BYPASS_COUNT_63,
}
impl From<REG_BYPASS_COUNT_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_BYPASS_COUNT_A) -> Self {
        match variant {
            REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_0 => 0,
            REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_1 => 1,
            REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_63 => 63,
        }
    }
}
#[doc = "Reader of field `REG_BYPASS_COUNT`"]
pub type REG_BYPASS_COUNT_R = crate::R<u8, REG_BYPASS_COUNT_A>;
impl REG_BYPASS_COUNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REG_BYPASS_COUNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_0),
            1 => Val(REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_1),
            63 => Val(REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_63),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG_BYPASS_COUNT_0`"]
    #[inline(always)]
    pub fn is_reg_bypass_count_0(&self) -> bool {
        *self == REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_0
    }
    #[doc = "Checks if the value of the field is `REG_BYPASS_COUNT_1`"]
    #[inline(always)]
    pub fn is_reg_bypass_count_1(&self) -> bool {
        *self == REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_1
    }
    #[doc = "Checks if the value of the field is `REG_BYPASS_COUNT_63`"]
    #[inline(always)]
    pub fn is_reg_bypass_count_63(&self) -> bool {
        *self == REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_63
    }
}
#[doc = "Write proxy for field `REG_BYPASS_COUNT`"]
pub struct REG_BYPASS_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_BYPASS_COUNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_BYPASS_COUNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "no delay"]
    #[inline(always)]
    pub fn reg_bypass_count_0(self) -> &'a mut W {
        self.variant(REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_0)
    }
    #[doc = "1 CKIL clock period delay"]
    #[inline(always)]
    pub fn reg_bypass_count_1(self) -> &'a mut W {
        self.variant(REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_1)
    }
    #[doc = "63 CKIL clock periods delay"]
    #[inline(always)]
    pub fn reg_bypass_count_63(self) -> &'a mut W {
        self.variant(REG_BYPASS_COUNT_A::REG_BYPASS_COUNT_63)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 21)) | (((value as u32) & 0x3f) << 21);
        self.w
    }
}
#[doc = "Enable for REG_BYPASS_COUNTER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBC_EN_A {
    #[doc = "0: REG_BYPASS_COUNTER disabled"]
    RBC_EN_0,
    #[doc = "1: REG_BYPASS_COUNTER enabled."]
    RBC_EN_1,
}
impl From<RBC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RBC_EN_A) -> Self {
        match variant {
            RBC_EN_A::RBC_EN_0 => false,
            RBC_EN_A::RBC_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `RBC_EN`"]
pub type RBC_EN_R = crate::R<bool, RBC_EN_A>;
impl RBC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBC_EN_A {
        match self.bits {
            false => RBC_EN_A::RBC_EN_0,
            true => RBC_EN_A::RBC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RBC_EN_0`"]
    #[inline(always)]
    pub fn is_rbc_en_0(&self) -> bool {
        *self == RBC_EN_A::RBC_EN_0
    }
    #[doc = "Checks if the value of the field is `RBC_EN_1`"]
    #[inline(always)]
    pub fn is_rbc_en_1(&self) -> bool {
        *self == RBC_EN_A::RBC_EN_1
    }
}
#[doc = "Write proxy for field `RBC_EN`"]
pub struct RBC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RBC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "REG_BYPASS_COUNTER disabled"]
    #[inline(always)]
    pub fn rbc_en_0(self) -> &'a mut W {
        self.variant(RBC_EN_A::RBC_EN_0)
    }
    #[doc = "REG_BYPASS_COUNTER enabled."]
    #[inline(always)]
    pub fn rbc_en_1(self) -> &'a mut W {
        self.variant(RBC_EN_A::RBC_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Oscillator ready counter value. These bits define value of 32KHz counter, that serve as counter for oscillator lock time (count to n+1 ckil's). This is used for oscillator lock time. Current estimation is ~5ms. This counter will be used in ignition sequence and in wake from stop sequence if sbyos bit was defined, to notify that on chip oscillator output is ready for the dpll_ip to use and only then the gate in dpll_ip can be opened."]
    #[inline(always)]
    pub fn oscnt(&self) -> OSCNT_R {
        OSCNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    #[inline(always)]
    pub fn cosc_en(&self) -> COSC_EN_R {
        COSC_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 21:26 - Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
    #[inline(always)]
    pub fn reg_bypass_count(&self) -> REG_BYPASS_COUNT_R {
        REG_BYPASS_COUNT_R::new(((self.bits >> 21) & 0x3f) as u8)
    }
    #[doc = "Bit 27 - Enable for REG_BYPASS_COUNTER"]
    #[inline(always)]
    pub fn rbc_en(&self) -> RBC_EN_R {
        RBC_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Oscillator ready counter value. These bits define value of 32KHz counter, that serve as counter for oscillator lock time (count to n+1 ckil's). This is used for oscillator lock time. Current estimation is ~5ms. This counter will be used in ignition sequence and in wake from stop sequence if sbyos bit was defined, to notify that on chip oscillator output is ready for the dpll_ip to use and only then the gate in dpll_ip can be opened."]
    #[inline(always)]
    pub fn oscnt(&mut self) -> OSCNT_W {
        OSCNT_W { w: self }
    }
    #[doc = "Bit 12 - On chip oscillator enable bit - this bit value is reflected on the output cosc_en"]
    #[inline(always)]
    pub fn cosc_en(&mut self) -> COSC_EN_W {
        COSC_EN_W { w: self }
    }
    #[doc = "Bits 21:26 - Counter for analog_reg_bypass signal assertion after standby voltage request by PMIC_STBY_REQ"]
    #[inline(always)]
    pub fn reg_bypass_count(&mut self) -> REG_BYPASS_COUNT_W {
        REG_BYPASS_COUNT_W { w: self }
    }
    #[doc = "Bit 27 - Enable for REG_BYPASS_COUNTER"]
    #[inline(always)]
    pub fn rbc_en(&mut self) -> RBC_EN_W {
        RBC_EN_W { w: self }
    }
}
