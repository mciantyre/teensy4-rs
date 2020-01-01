#[doc = "Reader of register SERQ"]
pub type R = crate::R<u8, super::SERQ>;
#[doc = "Writer for register SERQ"]
pub type W = crate::W<u8, super::SERQ>;
#[doc = "Register SERQ `reset()`'s with value 0"]
impl crate::ResetValue for super::SERQ {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SERQ`"]
pub type SERQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SERQ`"]
pub struct SERQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SERQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Set All Enable Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAER_A {
    #[doc = "0: Set only the ERQ bit specified in the SERQ field"]
    SAER_0 = 0,
    #[doc = "1: Set all bits in ERQ"]
    SAER_1 = 1,
}
impl From<SAER_A> for bool {
    #[inline(always)]
    fn from(variant: SAER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAER`"]
pub type SAER_R = crate::R<bool, SAER_A>;
impl SAER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAER_A {
        match self.bits {
            false => SAER_A::SAER_0,
            true => SAER_A::SAER_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAER_0`"]
    #[inline(always)]
    pub fn is_saer_0(&self) -> bool {
        *self == SAER_A::SAER_0
    }
    #[doc = "Checks if the value of the field is `SAER_1`"]
    #[inline(always)]
    pub fn is_saer_1(&self) -> bool {
        *self == SAER_A::SAER_1
    }
}
#[doc = "Write proxy for field `SAER`"]
pub struct SAER_W<'a> {
    w: &'a mut W,
}
impl<'a> SAER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set only the ERQ bit specified in the SERQ field"]
    #[inline(always)]
    pub fn saer_0(self) -> &'a mut W {
        self.variant(SAER_A::SAER_0)
    }
    #[doc = "Set all bits in ERQ"]
    #[inline(always)]
    pub fn saer_1(self) -> &'a mut W {
        self.variant(SAER_A::SAER_1)
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
    #[doc = "Bits 0:4 - Set Enable Request"]
    #[inline(always)]
    pub fn serq(&self) -> SERQ_R {
        SERQ_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Set All Enable Requests"]
    #[inline(always)]
    pub fn saer(&self) -> SAER_R {
        SAER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Set Enable Request"]
    #[inline(always)]
    pub fn serq(&mut self) -> SERQ_W {
        SERQ_W { w: self }
    }
    #[doc = "Bit 6 - Set All Enable Requests"]
    #[inline(always)]
    pub fn saer(&mut self) -> SAER_W {
        SAER_W { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&mut self) -> NOP_W {
        NOP_W { w: self }
    }
}
