#[doc = "Reader of register OFS"]
pub type R = crate::R<u32, super::OFS>;
#[doc = "Writer for register OFS"]
pub type W = crate::W<u32, super::OFS>;
#[doc = "Register OFS `reset()`'s with value 0"]
impl crate::ResetValue for super::OFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFS`"]
pub type OFS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OFS`"]
pub struct OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Sign bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGN_A {
    #[doc = "0: The offset value is added with the raw result"]
    SIGN_0 = 0,
    #[doc = "1: The offset value is subtracted from the raw converted value"]
    SIGN_1 = 1,
}
impl From<SIGN_A> for bool {
    #[inline(always)]
    fn from(variant: SIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SIGN`"]
pub type SIGN_R = crate::R<bool, SIGN_A>;
impl SIGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGN_A {
        match self.bits {
            false => SIGN_A::SIGN_0,
            true => SIGN_A::SIGN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SIGN_0`"]
    #[inline(always)]
    pub fn is_sign_0(&self) -> bool {
        *self == SIGN_A::SIGN_0
    }
    #[doc = "Checks if the value of the field is `SIGN_1`"]
    #[inline(always)]
    pub fn is_sign_1(&self) -> bool {
        *self == SIGN_A::SIGN_1
    }
}
#[doc = "Write proxy for field `SIGN`"]
pub struct SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIGN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The offset value is added with the raw result"]
    #[inline(always)]
    pub fn sign_0(self) -> &'a mut W {
        self.variant(SIGN_A::SIGN_0)
    }
    #[doc = "The offset value is subtracted from the raw converted value"]
    #[inline(always)]
    pub fn sign_1(self) -> &'a mut W {
        self.variant(SIGN_A::SIGN_1)
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
impl R {
    #[doc = "Bits 0:11 - Offset value"]
    #[inline(always)]
    pub fn ofs(&self) -> OFS_R {
        OFS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Sign bit"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Offset value"]
    #[inline(always)]
    pub fn ofs(&mut self) -> OFS_W {
        OFS_W { w: self }
    }
    #[doc = "Bit 12 - Sign bit"]
    #[inline(always)]
    pub fn sign(&mut self) -> SIGN_W {
        SIGN_W { w: self }
    }
}
