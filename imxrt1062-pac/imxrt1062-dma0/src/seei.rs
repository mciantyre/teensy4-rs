#[doc = "Reader of register SEEI"]
pub type R = crate::R<u8, super::SEEI>;
#[doc = "Writer for register SEEI"]
pub type W = crate::W<u8, super::SEEI>;
#[doc = "Register SEEI `reset()`'s with value 0"]
impl crate::ResetValue for super::SEEI {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEEI`"]
pub type SEEI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEEI`"]
pub struct SEEI_W<'a> {
    w: &'a mut W,
}
impl<'a> SEEI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Sets All Enable Error Interrupts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAEE_A {
    #[doc = "0: Set only the EEI bit specified in the SEEI field."]
    SAEE_0 = 0,
    #[doc = "1: Sets all bits in EEI"]
    SAEE_1 = 1,
}
impl From<SAEE_A> for bool {
    #[inline(always)]
    fn from(variant: SAEE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAEE`"]
pub type SAEE_R = crate::R<bool, SAEE_A>;
impl SAEE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAEE_A {
        match self.bits {
            false => SAEE_A::SAEE_0,
            true => SAEE_A::SAEE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAEE_0`"]
    #[inline(always)]
    pub fn is_saee_0(&self) -> bool {
        *self == SAEE_A::SAEE_0
    }
    #[doc = "Checks if the value of the field is `SAEE_1`"]
    #[inline(always)]
    pub fn is_saee_1(&self) -> bool {
        *self == SAEE_A::SAEE_1
    }
}
#[doc = "Write proxy for field `SAEE`"]
pub struct SAEE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAEE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAEE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set only the EEI bit specified in the SEEI field."]
    #[inline(always)]
    pub fn saee_0(self) -> &'a mut W {
        self.variant(SAEE_A::SAEE_0)
    }
    #[doc = "Sets all bits in EEI"]
    #[inline(always)]
    pub fn saee_1(self) -> &'a mut W {
        self.variant(SAEE_A::SAEE_1)
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
    #[doc = "Bits 0:4 - Set Enable Error Interrupt"]
    #[inline(always)]
    pub fn seei(&self) -> SEEI_R {
        SEEI_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Sets All Enable Error Interrupts"]
    #[inline(always)]
    pub fn saee(&self) -> SAEE_R {
        SAEE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Set Enable Error Interrupt"]
    #[inline(always)]
    pub fn seei(&mut self) -> SEEI_W {
        SEEI_W { w: self }
    }
    #[doc = "Bit 6 - Sets All Enable Error Interrupts"]
    #[inline(always)]
    pub fn saee(&mut self) -> SAEE_W {
        SAEE_W { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&mut self) -> NOP_W {
        NOP_W { w: self }
    }
}
