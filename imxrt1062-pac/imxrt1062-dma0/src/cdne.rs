#[doc = "Reader of register CDNE"]
pub type R = crate::R<u8, super::CDNE>;
#[doc = "Writer for register CDNE"]
pub type W = crate::W<u8, super::CDNE>;
#[doc = "Register CDNE `reset()`'s with value 0"]
impl crate::ResetValue for super::CDNE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CDNE`"]
pub type CDNE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CDNE`"]
pub struct CDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> CDNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Clears All DONE Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CADN_A {
    #[doc = "0: Clears only the TCDn_CSR\\[DONE\\]
bit specified in the CDNE field"]
    CADN_0 = 0,
    #[doc = "1: Clears all bits in TCDn_CSR\\[DONE\\]"]
    CADN_1 = 1,
}
impl From<CADN_A> for bool {
    #[inline(always)]
    fn from(variant: CADN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CADN`"]
pub type CADN_R = crate::R<bool, CADN_A>;
impl CADN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CADN_A {
        match self.bits {
            false => CADN_A::CADN_0,
            true => CADN_A::CADN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CADN_0`"]
    #[inline(always)]
    pub fn is_cadn_0(&self) -> bool {
        *self == CADN_A::CADN_0
    }
    #[doc = "Checks if the value of the field is `CADN_1`"]
    #[inline(always)]
    pub fn is_cadn_1(&self) -> bool {
        *self == CADN_A::CADN_1
    }
}
#[doc = "Write proxy for field `CADN`"]
pub struct CADN_W<'a> {
    w: &'a mut W,
}
impl<'a> CADN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CADN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears only the TCDn_CSR\\[DONE\\]
bit specified in the CDNE field"]
    #[inline(always)]
    pub fn cadn_0(self) -> &'a mut W {
        self.variant(CADN_A::CADN_0)
    }
    #[doc = "Clears all bits in TCDn_CSR\\[DONE\\]"]
    #[inline(always)]
    pub fn cadn_1(self) -> &'a mut W {
        self.variant(CADN_A::CADN_1)
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
    #[doc = "Bits 0:4 - Clear DONE Bit"]
    #[inline(always)]
    pub fn cdne(&self) -> CDNE_R {
        CDNE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Clears All DONE Bits"]
    #[inline(always)]
    pub fn cadn(&self) -> CADN_R {
        CADN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clear DONE Bit"]
    #[inline(always)]
    pub fn cdne(&mut self) -> CDNE_W {
        CDNE_W { w: self }
    }
    #[doc = "Bit 6 - Clears All DONE Bits"]
    #[inline(always)]
    pub fn cadn(&mut self) -> CADN_W {
        CADN_W { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&mut self) -> NOP_W {
        NOP_W { w: self }
    }
}
