#[doc = "Reader of register FILT%s"]
pub type R = crate::R<u16, super::FILT>;
#[doc = "Writer for register FILT%s"]
pub type W = crate::W<u16, super::FILT>;
#[doc = "Register FILT%s `reset()`'s with value 0"]
impl crate::ResetValue for super::FILT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FILT_PER`"]
pub type FILT_PER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILT_PER`"]
pub struct FILT_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILT_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `FILT_CNT`"]
pub type FILT_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILT_CNT`"]
pub struct FILT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FILT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Input Filter Sample Period"]
    #[inline(always)]
    pub fn filt_per(&self) -> FILT_PER_R {
        FILT_PER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Input Filter Sample Count"]
    #[inline(always)]
    pub fn filt_cnt(&self) -> FILT_CNT_R {
        FILT_CNT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input Filter Sample Period"]
    #[inline(always)]
    pub fn filt_per(&mut self) -> FILT_PER_W {
        FILT_PER_W { w: self }
    }
    #[doc = "Bits 8:10 - Input Filter Sample Count"]
    #[inline(always)]
    pub fn filt_cnt(&mut self) -> FILT_CNT_W {
        FILT_CNT_W { w: self }
    }
}
