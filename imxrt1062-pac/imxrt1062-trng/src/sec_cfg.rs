#[doc = "Reader of register SEC_CFG"]
pub type R = crate::R<u32, super::SEC_CFG>;
#[doc = "Writer for register SEC_CFG"]
pub type W = crate::W<u32, super::SEC_CFG>;
#[doc = "Register SEC_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UNUSED0`"]
pub type UNUSED0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNUSED0`"]
pub struct UNUSED0_W<'a> {
    w: &'a mut W,
}
impl<'a> UNUSED0_W<'a> {
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
#[doc = "If set, the TRNG registers cannot be programmed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NO_PRGM_A {
    #[doc = "0: Programability of registers controlled only by the Miscellaneous Control Register's access mode bit."]
    NO_PRGM_0 = 0,
    #[doc = "1: Overides Miscellaneous Control Register access mode and prevents TRNG register programming."]
    NO_PRGM_1 = 1,
}
impl From<NO_PRGM_A> for bool {
    #[inline(always)]
    fn from(variant: NO_PRGM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NO_PRGM`"]
pub type NO_PRGM_R = crate::R<bool, NO_PRGM_A>;
impl NO_PRGM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NO_PRGM_A {
        match self.bits {
            false => NO_PRGM_A::NO_PRGM_0,
            true => NO_PRGM_A::NO_PRGM_1,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PRGM_0`"]
    #[inline(always)]
    pub fn is_no_prgm_0(&self) -> bool {
        *self == NO_PRGM_A::NO_PRGM_0
    }
    #[doc = "Checks if the value of the field is `NO_PRGM_1`"]
    #[inline(always)]
    pub fn is_no_prgm_1(&self) -> bool {
        *self == NO_PRGM_A::NO_PRGM_1
    }
}
#[doc = "Write proxy for field `NO_PRGM`"]
pub struct NO_PRGM_W<'a> {
    w: &'a mut W,
}
impl<'a> NO_PRGM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NO_PRGM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Programability of registers controlled only by the Miscellaneous Control Register's access mode bit."]
    #[inline(always)]
    pub fn no_prgm_0(self) -> &'a mut W {
        self.variant(NO_PRGM_A::NO_PRGM_0)
    }
    #[doc = "Overides Miscellaneous Control Register access mode and prevents TRNG register programming."]
    #[inline(always)]
    pub fn no_prgm_1(self) -> &'a mut W {
        self.variant(NO_PRGM_A::NO_PRGM_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `UNUSED2`"]
pub type UNUSED2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNUSED2`"]
pub struct UNUSED2_W<'a> {
    w: &'a mut W,
}
impl<'a> UNUSED2_W<'a> {
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
impl R {
    #[doc = "Bit 0 - This bit is unused. Ignore."]
    #[inline(always)]
    pub fn unused0(&self) -> UNUSED0_R {
        UNUSED0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If set, the TRNG registers cannot be programmed"]
    #[inline(always)]
    pub fn no_prgm(&self) -> NO_PRGM_R {
        NO_PRGM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is unused. Ignore."]
    #[inline(always)]
    pub fn unused2(&self) -> UNUSED2_R {
        UNUSED2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is unused. Ignore."]
    #[inline(always)]
    pub fn unused0(&mut self) -> UNUSED0_W {
        UNUSED0_W { w: self }
    }
    #[doc = "Bit 1 - If set, the TRNG registers cannot be programmed"]
    #[inline(always)]
    pub fn no_prgm(&mut self) -> NO_PRGM_W {
        NO_PRGM_W { w: self }
    }
    #[doc = "Bit 2 - This bit is unused. Ignore."]
    #[inline(always)]
    pub fn unused2(&mut self) -> UNUSED2_W {
        UNUSED2_W { w: self }
    }
}
