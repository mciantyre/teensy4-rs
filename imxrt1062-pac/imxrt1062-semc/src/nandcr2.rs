#[doc = "Reader of register NANDCR2"]
pub type R = crate::R<u32, super::NANDCR2>;
#[doc = "Writer for register NANDCR2"]
pub type W = crate::W<u32, super::NANDCR2>;
#[doc = "Register NANDCR2 `reset()`'s with value 0x0001_0410"]
impl crate::ResetValue for super::NANDCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0410
    }
}
#[doc = "Reader of field `TWHR`"]
pub type TWHR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TWHR`"]
pub struct TWHR_W<'a> {
    w: &'a mut W,
}
impl<'a> TWHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `TRHW`"]
pub type TRHW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRHW`"]
pub struct TRHW_W<'a> {
    w: &'a mut W,
}
impl<'a> TRHW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `TADL`"]
pub type TADL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TADL`"]
pub struct TADL_W<'a> {
    w: &'a mut W,
}
impl<'a> TADL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `TRR`"]
pub type TRR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRR`"]
pub struct TRR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `TWB`"]
pub type TWB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TWB`"]
pub struct TWB_W<'a> {
    w: &'a mut W,
}
impl<'a> TWB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - WE# HIGH to RE# LOW wait time"]
    #[inline(always)]
    pub fn twhr(&self) -> TWHR_R {
        TWHR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - RE# HIGH to WE# LOW wait time"]
    #[inline(always)]
    pub fn trhw(&self) -> TRHW_R {
        TRHW_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - ALE to WRITE Data start wait time"]
    #[inline(always)]
    pub fn tadl(&self) -> TADL_R {
        TADL_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Ready to RE# LOW min wait time"]
    #[inline(always)]
    pub fn trr(&self) -> TRR_R {
        TRR_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - WE# HIGH to busy wait time"]
    #[inline(always)]
    pub fn twb(&self) -> TWB_R {
        TWB_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - WE# HIGH to RE# LOW wait time"]
    #[inline(always)]
    pub fn twhr(&mut self) -> TWHR_W {
        TWHR_W { w: self }
    }
    #[doc = "Bits 6:11 - RE# HIGH to WE# LOW wait time"]
    #[inline(always)]
    pub fn trhw(&mut self) -> TRHW_W {
        TRHW_W { w: self }
    }
    #[doc = "Bits 12:17 - ALE to WRITE Data start wait time"]
    #[inline(always)]
    pub fn tadl(&mut self) -> TADL_W {
        TADL_W { w: self }
    }
    #[doc = "Bits 18:23 - Ready to RE# LOW min wait time"]
    #[inline(always)]
    pub fn trr(&mut self) -> TRR_W {
        TRR_W { w: self }
    }
    #[doc = "Bits 24:29 - WE# HIGH to busy wait time"]
    #[inline(always)]
    pub fn twb(&mut self) -> TWB_W {
        TWB_W { w: self }
    }
}
