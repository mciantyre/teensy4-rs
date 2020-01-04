#[doc = "Reader of register REG_1P1_SET"]
pub type R = crate::R<u32, super::REG_1P1_SET>;
#[doc = "Writer for register REG_1P1_SET"]
pub type W = crate::W<u32, super::REG_1P1_SET>;
#[doc = "Register REG_1P1_SET `reset()`'s with value 0x1073"]
impl crate::ResetValue for super::REG_1P1_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1073
    }
}
#[doc = "Reader of field `ENABLE_LINREG`"]
pub type ENABLE_LINREG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_LINREG`"]
pub struct ENABLE_LINREG_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_LINREG_W<'a> {
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
#[doc = "Reader of field `ENABLE_BO`"]
pub type ENABLE_BO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_BO`"]
pub struct ENABLE_BO_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_BO_W<'a> {
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
#[doc = "Reader of field `ENABLE_ILIMIT`"]
pub type ENABLE_ILIMIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_ILIMIT`"]
pub struct ENABLE_ILIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_ILIMIT_W<'a> {
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
#[doc = "Reader of field `ENABLE_PULLDOWN`"]
pub type ENABLE_PULLDOWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_PULLDOWN`"]
pub struct ENABLE_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_PULLDOWN_W<'a> {
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
#[doc = "Reader of field `BO_OFFSET`"]
pub type BO_OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BO_OFFSET`"]
pub struct BO_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> BO_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Control bits to adjust the regulator output voltage\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTPUT_TRG_A {
    #[doc = "4: 0.8V"]
    OUTPUT_TRG_4 = 4,
    #[doc = "16: 1.1V"]
    OUTPUT_TRG_16 = 16,
}
impl From<OUTPUT_TRG_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTPUT_TRG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUTPUT_TRG`"]
pub type OUTPUT_TRG_R = crate::R<u8, OUTPUT_TRG_A>;
impl OUTPUT_TRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OUTPUT_TRG_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(OUTPUT_TRG_A::OUTPUT_TRG_4),
            16 => Val(OUTPUT_TRG_A::OUTPUT_TRG_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_4`"]
    #[inline(always)]
    pub fn is_output_trg_4(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_4
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_16`"]
    #[inline(always)]
    pub fn is_output_trg_16(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_16
    }
}
#[doc = "Write proxy for field `OUTPUT_TRG`"]
pub struct OUTPUT_TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_TRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTPUT_TRG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0.8V"]
    #[inline(always)]
    pub fn output_trg_4(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_4)
    }
    #[doc = "1.1V"]
    #[inline(always)]
    pub fn output_trg_16(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `BO_VDD1P1`"]
pub type BO_VDD1P1_R = crate::R<bool, bool>;
#[doc = "Reader of field `OK_VDD1P1`"]
pub type OK_VDD1P1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENABLE_WEAK_LINREG`"]
pub type ENABLE_WEAK_LINREG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_WEAK_LINREG`"]
pub struct ENABLE_WEAK_LINREG_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_WEAK_LINREG_W<'a> {
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
#[doc = "Selects the source for the reference voltage of the weak 1p1 regulator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELREF_WEAK_LINREG_A {
    #[doc = "0: Weak-linreg output tracks low-power-bandgap voltage"]
    SELREF_WEAK_LINREG_0 = 0,
    #[doc = "1: Weak-linreg output tracks VDD_SOC_IN voltage"]
    SELREF_WEAK_LINREG_1 = 1,
}
impl From<SELREF_WEAK_LINREG_A> for bool {
    #[inline(always)]
    fn from(variant: SELREF_WEAK_LINREG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SELREF_WEAK_LINREG`"]
pub type SELREF_WEAK_LINREG_R = crate::R<bool, SELREF_WEAK_LINREG_A>;
impl SELREF_WEAK_LINREG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELREF_WEAK_LINREG_A {
        match self.bits {
            false => SELREF_WEAK_LINREG_A::SELREF_WEAK_LINREG_0,
            true => SELREF_WEAK_LINREG_A::SELREF_WEAK_LINREG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SELREF_WEAK_LINREG_0`"]
    #[inline(always)]
    pub fn is_selref_weak_linreg_0(&self) -> bool {
        *self == SELREF_WEAK_LINREG_A::SELREF_WEAK_LINREG_0
    }
    #[doc = "Checks if the value of the field is `SELREF_WEAK_LINREG_1`"]
    #[inline(always)]
    pub fn is_selref_weak_linreg_1(&self) -> bool {
        *self == SELREF_WEAK_LINREG_A::SELREF_WEAK_LINREG_1
    }
}
#[doc = "Write proxy for field `SELREF_WEAK_LINREG`"]
pub struct SELREF_WEAK_LINREG_W<'a> {
    w: &'a mut W,
}
impl<'a> SELREF_WEAK_LINREG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELREF_WEAK_LINREG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
    #[inline(always)]
    pub fn selref_weak_linreg_0(self) -> &'a mut W {
        self.variant(SELREF_WEAK_LINREG_A::SELREF_WEAK_LINREG_0)
    }
    #[doc = "Weak-linreg output tracks VDD_SOC_IN voltage"]
    #[inline(always)]
    pub fn selref_weak_linreg_1(self) -> &'a mut W {
        self.variant(SELREF_WEAK_LINREG_A::SELREF_WEAK_LINREG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Control bit to enable the regulator output."]
    #[inline(always)]
    pub fn enable_linreg(&self) -> ENABLE_LINREG_R {
        ENABLE_LINREG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub fn enable_bo(&self) -> ENABLE_BO_R {
        ENABLE_BO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub fn enable_ilimit(&self) -> ENABLE_ILIMIT_R {
        ENABLE_ILIMIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub fn enable_pulldown(&self) -> ENABLE_PULLDOWN_R {
        ENABLE_PULLDOWN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub fn bo_offset(&self) -> BO_OFFSET_R {
        BO_OFFSET_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub fn output_trg(&self) -> OUTPUT_TRG_R {
        OUTPUT_TRG_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub fn bo_vdd1p1(&self) -> BO_VDD1P1_R {
        BO_VDD1P1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub fn ok_vdd1p1(&self) -> OK_VDD1P1_R {
        OK_VDD1P1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub fn enable_weak_linreg(&self) -> ENABLE_WEAK_LINREG_R {
        ENABLE_WEAK_LINREG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub fn selref_weak_linreg(&self) -> SELREF_WEAK_LINREG_R {
        SELREF_WEAK_LINREG_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control bit to enable the regulator output."]
    #[inline(always)]
    pub fn enable_linreg(&mut self) -> ENABLE_LINREG_W {
        ENABLE_LINREG_W { w: self }
    }
    #[doc = "Bit 1 - Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub fn enable_bo(&mut self) -> ENABLE_BO_W {
        ENABLE_BO_W { w: self }
    }
    #[doc = "Bit 2 - Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub fn enable_ilimit(&mut self) -> ENABLE_ILIMIT_W {
        ENABLE_ILIMIT_W { w: self }
    }
    #[doc = "Bit 3 - Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub fn enable_pulldown(&mut self) -> ENABLE_PULLDOWN_W {
        ENABLE_PULLDOWN_W { w: self }
    }
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub fn bo_offset(&mut self) -> BO_OFFSET_W {
        BO_OFFSET_W { w: self }
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub fn output_trg(&mut self) -> OUTPUT_TRG_W {
        OUTPUT_TRG_W { w: self }
    }
    #[doc = "Bit 18 - Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub fn enable_weak_linreg(&mut self) -> ENABLE_WEAK_LINREG_W {
        ENABLE_WEAK_LINREG_W { w: self }
    }
    #[doc = "Bit 19 - Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub fn selref_weak_linreg(&mut self) -> SELREF_WEAK_LINREG_W {
        SELREF_WEAK_LINREG_W { w: self }
    }
}
