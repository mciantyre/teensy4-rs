#[doc = "Reader of register CERR"]
pub type R = crate::R<u8, super::CERR>;
#[doc = "Writer for register CERR"]
pub type W = crate::W<u8, super::CERR>;
#[doc = "Register CERR `reset()`'s with value 0"]
impl crate::ResetValue for super::CERR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CERR`"]
pub type CERR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CERR`"]
pub struct CERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Clear All Error Indicators\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAEI_A {
    #[doc = "0: Clear only the ERR bit specified in the CERR field"]
    CAEI_0 = 0,
    #[doc = "1: Clear all bits in ERR"]
    CAEI_1 = 1,
}
impl From<CAEI_A> for bool {
    #[inline(always)]
    fn from(variant: CAEI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAEI`"]
pub type CAEI_R = crate::R<bool, CAEI_A>;
impl CAEI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAEI_A {
        match self.bits {
            false => CAEI_A::CAEI_0,
            true => CAEI_A::CAEI_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAEI_0`"]
    #[inline(always)]
    pub fn is_caei_0(&self) -> bool {
        *self == CAEI_A::CAEI_0
    }
    #[doc = "Checks if the value of the field is `CAEI_1`"]
    #[inline(always)]
    pub fn is_caei_1(&self) -> bool {
        *self == CAEI_A::CAEI_1
    }
}
#[doc = "Write proxy for field `CAEI`"]
pub struct CAEI_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAEI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear only the ERR bit specified in the CERR field"]
    #[inline(always)]
    pub fn caei_0(self) -> &'a mut W {
        self.variant(CAEI_A::CAEI_0)
    }
    #[doc = "Clear all bits in ERR"]
    #[inline(always)]
    pub fn caei_1(self) -> &'a mut W {
        self.variant(CAEI_A::CAEI_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "No Op enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOP_A {
    #[doc = "0: Normal operation"]
    NOP_0 = 0,
    #[doc = "1: No operation, ignore the other bits in this register"]
    NOP_1 = 1,
}
impl From<NOP_A> for bool {
    #[inline(always)]
    fn from(variant: NOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NOP`"]
pub type NOP_R = crate::R<bool, NOP_A>;
impl NOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOP_A {
        match self.bits {
            false => NOP_A::NOP_0,
            true => NOP_A::NOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP_0`"]
    #[inline(always)]
    pub fn is_nop_0(&self) -> bool {
        *self == NOP_A::NOP_0
    }
    #[doc = "Checks if the value of the field is `NOP_1`"]
    #[inline(always)]
    pub fn is_nop_1(&self) -> bool {
        *self == NOP_A::NOP_1
    }
}
#[doc = "Write proxy for field `NOP`"]
pub struct NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn nop_0(self) -> &'a mut W {
        self.variant(NOP_A::NOP_0)
    }
    #[doc = "No operation, ignore the other bits in this register"]
    #[inline(always)]
    pub fn nop_1(self) -> &'a mut W {
        self.variant(NOP_A::NOP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Clear Error Indicator"]
    #[inline(always)]
    pub fn cerr(&self) -> CERR_R {
        CERR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Clear All Error Indicators"]
    #[inline(always)]
    pub fn caei(&self) -> CAEI_R {
        CAEI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clear Error Indicator"]
    #[inline(always)]
    pub fn cerr(&mut self) -> CERR_W {
        CERR_W { w: self }
    }
    #[doc = "Bit 6 - Clear All Error Indicators"]
    #[inline(always)]
    pub fn caei(&mut self) -> CAEI_W {
        CAEI_W { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&mut self) -> NOP_W {
        NOP_W { w: self }
    }
}
