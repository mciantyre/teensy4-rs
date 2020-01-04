#[doc = "Reader of register CSIFBUF_PARA"]
pub type R = crate::R<u32, super::CSIFBUF_PARA>;
#[doc = "Writer for register CSIFBUF_PARA"]
pub type W = crate::W<u32, super::CSIFBUF_PARA>;
#[doc = "Register CSIFBUF_PARA `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIFBUF_PARA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FBUF_STRIDE`"]
pub type FBUF_STRIDE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FBUF_STRIDE`"]
pub struct FBUF_STRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBUF_STRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DEINTERLACE_STRIDE`"]
pub type DEINTERLACE_STRIDE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DEINTERLACE_STRIDE`"]
pub struct DEINTERLACE_STRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEINTERLACE_STRIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame Buffer Parameter"]
    #[inline(always)]
    pub fn fbuf_stride(&self) -> FBUF_STRIDE_R {
        FBUF_STRIDE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DEINTERLACE_STRIDE is only used in the deinterlace mode"]
    #[inline(always)]
    pub fn deinterlace_stride(&self) -> DEINTERLACE_STRIDE_R {
        DEINTERLACE_STRIDE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Buffer Parameter"]
    #[inline(always)]
    pub fn fbuf_stride(&mut self) -> FBUF_STRIDE_W {
        FBUF_STRIDE_W { w: self }
    }
    #[doc = "Bits 16:31 - DEINTERLACE_STRIDE is only used in the deinterlace mode"]
    #[inline(always)]
    pub fn deinterlace_stride(&mut self) -> DEINTERLACE_STRIDE_W {
        DEINTERLACE_STRIDE_W { w: self }
    }
}
