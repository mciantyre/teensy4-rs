#[doc = "Reader of register CMPH"]
pub type R = crate::R<u8, super::CMPH>;
#[doc = "Writer for register CMPH"]
pub type W = crate::W<u8, super::CMPH>;
#[doc = "Register CMPH `reset()`'s with value 0xff"]
impl crate::ResetValue for super::CMPH {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `COMPAREH`"]
pub type COMPAREH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPAREH`"]
pub struct COMPAREH_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - COMPAREH"]
    #[inline(always)]
    pub fn compareh(&self) -> COMPAREH_R {
        COMPAREH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COMPAREH"]
    #[inline(always)]
    pub fn compareh(&mut self) -> COMPAREH_W {
        COMPAREH_W { w: self }
    }
}
