#[doc = "Reader of register FDCBT"]
pub type R = crate::R<u32, super::FDCBT>;
#[doc = "Writer for register FDCBT"]
pub type W = crate::W<u32, super::FDCBT>;
#[doc = "Register FDCBT `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCBT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPSEG2`"]
pub type FPSEG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FPSEG2`"]
pub struct FPSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `FPSEG1`"]
pub type FPSEG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FPSEG1`"]
pub struct FPSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `FPROPSEG`"]
pub type FPROPSEG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FPROPSEG`"]
pub struct FPROPSEG_W<'a> {
    w: &'a mut W,
}
impl<'a> FPROPSEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `FRJW`"]
pub type FRJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRJW`"]
pub struct FRJW_W<'a> {
    w: &'a mut W,
}
impl<'a> FRJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `FPRESDIV`"]
pub type FPRESDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FPRESDIV`"]
pub struct FPRESDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FPRESDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Fast Phase Segment 2"]
    #[inline(always)]
    pub fn fpseg2(&self) -> FPSEG2_R {
        FPSEG2_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - Fast Phase Segment 1"]
    #[inline(always)]
    pub fn fpseg1(&self) -> FPSEG1_R {
        FPSEG1_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 10:14 - Fast Propagation Segment"]
    #[inline(always)]
    pub fn fpropseg(&self) -> FPROPSEG_R {
        FPROPSEG_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - Fast Resync Jump Width"]
    #[inline(always)]
    pub fn frjw(&self) -> FRJW_R {
        FRJW_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:29 - Fast Prescaler Division Factor"]
    #[inline(always)]
    pub fn fpresdiv(&self) -> FPRESDIV_R {
        FPRESDIV_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Fast Phase Segment 2"]
    #[inline(always)]
    pub fn fpseg2(&mut self) -> FPSEG2_W {
        FPSEG2_W { w: self }
    }
    #[doc = "Bits 5:7 - Fast Phase Segment 1"]
    #[inline(always)]
    pub fn fpseg1(&mut self) -> FPSEG1_W {
        FPSEG1_W { w: self }
    }
    #[doc = "Bits 10:14 - Fast Propagation Segment"]
    #[inline(always)]
    pub fn fpropseg(&mut self) -> FPROPSEG_W {
        FPROPSEG_W { w: self }
    }
    #[doc = "Bits 16:18 - Fast Resync Jump Width"]
    #[inline(always)]
    pub fn frjw(&mut self) -> FRJW_W {
        FRJW_W { w: self }
    }
    #[doc = "Bits 20:29 - Fast Prescaler Division Factor"]
    #[inline(always)]
    pub fn fpresdiv(&mut self) -> FPRESDIV_W {
        FPRESDIV_W { w: self }
    }
}
