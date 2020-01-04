#[doc = "Reader of register CMPL"]
pub type R = crate::R<u8, super::CMPL>;
#[doc = "Writer for register CMPL"]
pub type W = crate::W<u8, super::CMPL>;
#[doc = "Register CMPL `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPAREL`"]
pub type COMPAREL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPAREL`"]
pub struct COMPAREL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - COMPAREL"]
    #[inline(always)]
    pub fn comparel(&self) -> COMPAREL_R {
        COMPAREL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COMPAREL"]
    #[inline(always)]
    pub fn comparel(&mut self) -> COMPAREL_W {
        COMPAREL_W { w: self }
    }
}
