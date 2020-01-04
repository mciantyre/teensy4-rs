#[doc = "Reader of register TCD20_NBYTES_MLOFFYES"]
pub type R = crate::R<u32, super::TCD20_NBYTES_MLOFFYES>;
#[doc = "Writer for register TCD20_NBYTES_MLOFFYES"]
pub type W = crate::W<u32, super::TCD20_NBYTES_MLOFFYES>;
#[doc = "Register TCD20_NBYTES_MLOFFYES `reset()`'s with value 0"]
impl crate::ResetValue for super::TCD20_NBYTES_MLOFFYES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NBYTES`"]
pub type NBYTES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NBYTES`"]
pub struct NBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `MLOFF`"]
pub type MLOFF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MLOFF`"]
pub struct MLOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> MLOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 10)) | (((value as u32) & 0x000f_ffff) << 10);
        self.w
    }
}
#[doc = "Destination Minor Loop Offset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMLOE_A {
    #[doc = "0: The minor loop offset is not applied to the DADDR"]
    DMLOE_0 = 0,
    #[doc = "1: The minor loop offset is applied to the DADDR"]
    DMLOE_1 = 1,
}
impl From<DMLOE_A> for bool {
    #[inline(always)]
    fn from(variant: DMLOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMLOE`"]
pub type DMLOE_R = crate::R<bool, DMLOE_A>;
impl DMLOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMLOE_A {
        match self.bits {
            false => DMLOE_A::DMLOE_0,
            true => DMLOE_A::DMLOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMLOE_0`"]
    #[inline(always)]
    pub fn is_dmloe_0(&self) -> bool {
        *self == DMLOE_A::DMLOE_0
    }
    #[doc = "Checks if the value of the field is `DMLOE_1`"]
    #[inline(always)]
    pub fn is_dmloe_1(&self) -> bool {
        *self == DMLOE_A::DMLOE_1
    }
}
#[doc = "Write proxy for field `DMLOE`"]
pub struct DMLOE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMLOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMLOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The minor loop offset is not applied to the DADDR"]
    #[inline(always)]
    pub fn dmloe_0(self) -> &'a mut W {
        self.variant(DMLOE_A::DMLOE_0)
    }
    #[doc = "The minor loop offset is applied to the DADDR"]
    #[inline(always)]
    pub fn dmloe_1(self) -> &'a mut W {
        self.variant(DMLOE_A::DMLOE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Source Minor Loop Offset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMLOE_A {
    #[doc = "0: The minor loop offset is not applied to the SADDR"]
    SMLOE_0 = 0,
    #[doc = "1: The minor loop offset is applied to the SADDR"]
    SMLOE_1 = 1,
}
impl From<SMLOE_A> for bool {
    #[inline(always)]
    fn from(variant: SMLOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMLOE`"]
pub type SMLOE_R = crate::R<bool, SMLOE_A>;
impl SMLOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMLOE_A {
        match self.bits {
            false => SMLOE_A::SMLOE_0,
            true => SMLOE_A::SMLOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMLOE_0`"]
    #[inline(always)]
    pub fn is_smloe_0(&self) -> bool {
        *self == SMLOE_A::SMLOE_0
    }
    #[doc = "Checks if the value of the field is `SMLOE_1`"]
    #[inline(always)]
    pub fn is_smloe_1(&self) -> bool {
        *self == SMLOE_A::SMLOE_1
    }
}
#[doc = "Write proxy for field `SMLOE`"]
pub struct SMLOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMLOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMLOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The minor loop offset is not applied to the SADDR"]
    #[inline(always)]
    pub fn smloe_0(self) -> &'a mut W {
        self.variant(SMLOE_A::SMLOE_0)
    }
    #[doc = "The minor loop offset is applied to the SADDR"]
    #[inline(always)]
    pub fn smloe_1(self) -> &'a mut W {
        self.variant(SMLOE_A::SMLOE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Minor Byte Transfer Count"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:29 - If SMLOE or DMLOE is set, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub fn mloff(&self) -> MLOFF_R {
        MLOFF_R::new(((self.bits >> 10) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 30 - Destination Minor Loop Offset enable"]
    #[inline(always)]
    pub fn dmloe(&self) -> DMLOE_R {
        DMLOE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub fn smloe(&self) -> SMLOE_R {
        SMLOE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Minor Byte Transfer Count"]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W {
        NBYTES_W { w: self }
    }
    #[doc = "Bits 10:29 - If SMLOE or DMLOE is set, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub fn mloff(&mut self) -> MLOFF_W {
        MLOFF_W { w: self }
    }
    #[doc = "Bit 30 - Destination Minor Loop Offset enable"]
    #[inline(always)]
    pub fn dmloe(&mut self) -> DMLOE_W {
        DMLOE_W { w: self }
    }
    #[doc = "Bit 31 - Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub fn smloe(&mut self) -> SMLOE_W {
        SMLOE_W { w: self }
    }
}
