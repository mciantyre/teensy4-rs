#[doc = "Reader of register CINT"]
pub type R = crate::R<u8, super::CINT>;
#[doc = "Writer for register CINT"]
pub type W = crate::W<u8, super::CINT>;
#[doc = "Register CINT `reset()`'s with value 0"]
impl crate::ResetValue for super::CINT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CINT`"]
pub type CINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CINT`"]
pub struct CINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Clear All Interrupt Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAIR_A {
    #[doc = "0: Clear only the INT bit specified in the CINT field"]
    CAIR_0 = 0,
    #[doc = "1: Clear all bits in INT"]
    CAIR_1 = 1,
}
impl From<CAIR_A> for bool {
    #[inline(always)]
    fn from(variant: CAIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAIR`"]
pub type CAIR_R = crate::R<bool, CAIR_A>;
impl CAIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAIR_A {
        match self.bits {
            false => CAIR_A::CAIR_0,
            true => CAIR_A::CAIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAIR_0`"]
    #[inline(always)]
    pub fn is_cair_0(&self) -> bool {
        *self == CAIR_A::CAIR_0
    }
    #[doc = "Checks if the value of the field is `CAIR_1`"]
    #[inline(always)]
    pub fn is_cair_1(&self) -> bool {
        *self == CAIR_A::CAIR_1
    }
}
#[doc = "Write proxy for field `CAIR`"]
pub struct CAIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear only the INT bit specified in the CINT field"]
    #[inline(always)]
    pub fn cair_0(self) -> &'a mut W {
        self.variant(CAIR_A::CAIR_0)
    }
    #[doc = "Clear all bits in INT"]
    #[inline(always)]
    pub fn cair_1(self) -> &'a mut W {
        self.variant(CAIR_A::CAIR_1)
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
    #[doc = "Bits 0:4 - Clear Interrupt Request"]
    #[inline(always)]
    pub fn cint(&self) -> CINT_R {
        CINT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Clear All Interrupt Requests"]
    #[inline(always)]
    pub fn cair(&self) -> CAIR_R {
        CAIR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clear Interrupt Request"]
    #[inline(always)]
    pub fn cint(&mut self) -> CINT_W {
        CINT_W { w: self }
    }
    #[doc = "Bit 6 - Clear All Interrupt Requests"]
    #[inline(always)]
    pub fn cair(&mut self) -> CAIR_W {
        CAIR_W { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&mut self) -> NOP_W {
        NOP_W { w: self }
    }
}
