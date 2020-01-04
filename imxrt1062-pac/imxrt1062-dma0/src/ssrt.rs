#[doc = "Reader of register SSRT"]
pub type R = crate::R<u8, super::SSRT>;
#[doc = "Writer for register SSRT"]
pub type W = crate::W<u8, super::SSRT>;
#[doc = "Register SSRT `reset()`'s with value 0"]
impl crate::ResetValue for super::SSRT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSRT`"]
pub type SSRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSRT`"]
pub struct SSRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SSRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Set All START Bits (activates all channels)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAST_A {
    #[doc = "0: Set only the TCDn_CSR\\[START\\]
bit specified in the SSRT field"]
    SAST_0 = 0,
    #[doc = "1: Set all bits in TCDn_CSR\\[START\\]"]
    SAST_1 = 1,
}
impl From<SAST_A> for bool {
    #[inline(always)]
    fn from(variant: SAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAST`"]
pub type SAST_R = crate::R<bool, SAST_A>;
impl SAST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAST_A {
        match self.bits {
            false => SAST_A::SAST_0,
            true => SAST_A::SAST_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAST_0`"]
    #[inline(always)]
    pub fn is_sast_0(&self) -> bool {
        *self == SAST_A::SAST_0
    }
    #[doc = "Checks if the value of the field is `SAST_1`"]
    #[inline(always)]
    pub fn is_sast_1(&self) -> bool {
        *self == SAST_A::SAST_1
    }
}
#[doc = "Write proxy for field `SAST`"]
pub struct SAST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set only the TCDn_CSR\\[START\\]
bit specified in the SSRT field"]
    #[inline(always)]
    pub fn sast_0(self) -> &'a mut W {
        self.variant(SAST_A::SAST_0)
    }
    #[doc = "Set all bits in TCDn_CSR\\[START\\]"]
    #[inline(always)]
    pub fn sast_1(self) -> &'a mut W {
        self.variant(SAST_A::SAST_1)
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
    #[doc = "Bits 0:4 - Set START Bit"]
    #[inline(always)]
    pub fn ssrt(&self) -> SSRT_R {
        SSRT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Set All START Bits (activates all channels)"]
    #[inline(always)]
    pub fn sast(&self) -> SAST_R {
        SAST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Set START Bit"]
    #[inline(always)]
    pub fn ssrt(&mut self) -> SSRT_W {
        SSRT_W { w: self }
    }
    #[doc = "Bit 6 - Set All START Bits (activates all channels)"]
    #[inline(always)]
    pub fn sast(&mut self) -> SAST_W {
        SAST_W { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&mut self) -> NOP_W {
        NOP_W { w: self }
    }
}
