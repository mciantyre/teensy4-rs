#[doc = "Reader of register ASYNCLISTADDR"]
pub type R = crate::R<u32, super::ASYNCLISTADDR>;
#[doc = "Writer for register ASYNCLISTADDR"]
pub type W = crate::W<u32, super::ASYNCLISTADDR>;
#[doc = "Register ASYNCLISTADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ASYNCLISTADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASYBASE`"]
pub type ASYBASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ASYBASE`"]
pub struct ASYBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:31 - Link Pointer Low (LPL)"]
    #[inline(always)]
    pub fn asybase(&self) -> ASYBASE_R {
        ASYBASE_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 5:31 - Link Pointer Low (LPL)"]
    #[inline(always)]
    pub fn asybase(&mut self) -> ASYBASE_W {
        ASYBASE_W { w: self }
    }
}
