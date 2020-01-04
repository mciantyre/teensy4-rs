#[doc = "Reader of register CSC1_COEF2"]
pub type R = crate::R<u32, super::CSC1_COEF2>;
#[doc = "Writer for register CSC1_COEF2"]
pub type W = crate::W<u32, super::CSC1_COEF2>;
#[doc = "Register CSC1_COEF2 `reset()`'s with value 0x079b_076c"]
impl crate::ResetValue for super::CSC1_COEF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x079b_076c
    }
}
#[doc = "Reader of field `C3`"]
pub type C3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `C3`"]
pub struct C3_W<'a> {
    w: &'a mut W,
}
impl<'a> C3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u8, u8>;
#[doc = "Reader of field `C2`"]
pub type C2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `C2`"]
pub struct C2_W<'a> {
    w: &'a mut W,
}
impl<'a> C2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:10 - Two's complement Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)"]
    #[inline(always)]
    pub fn c3(&self) -> C3_R {
        C3_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - Two's complement Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)"]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:31 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Two's complement Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)"]
    #[inline(always)]
    pub fn c3(&mut self) -> C3_W {
        C3_W { w: self }
    }
    #[doc = "Bits 16:26 - Two's complement Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)"]
    #[inline(always)]
    pub fn c2(&mut self) -> C2_W {
        C2_W { w: self }
    }
}
