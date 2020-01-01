#[doc = "Reader of register CSC1_COEF1"]
pub type R = crate::R<u32, super::CSC1_COEF1>;
#[doc = "Writer for register CSC1_COEF1"]
pub type W = crate::W<u32, super::CSC1_COEF1>;
#[doc = "Register CSC1_COEF1 `reset()`'s with value 0x0123_0208"]
impl crate::ResetValue for super::CSC1_COEF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0123_0208
    }
}
#[doc = "Reader of field `C4`"]
pub type C4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `C4`"]
pub struct C4_W<'a> {
    w: &'a mut W,
}
impl<'a> C4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u8, u8>;
#[doc = "Reader of field `C1`"]
pub type C1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `C1`"]
pub struct C1_W<'a> {
    w: &'a mut W,
}
impl<'a> C1_W<'a> {
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
    #[doc = "Bits 0:10 - Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017)"]
    #[inline(always)]
    pub fn c4(&self) -> C4_R {
        C4_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596)"]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:31 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017)"]
    #[inline(always)]
    pub fn c4(&mut self) -> C4_W {
        C4_W { w: self }
    }
    #[doc = "Bits 16:26 - Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596)"]
    #[inline(always)]
    pub fn c1(&mut self) -> C1_W {
        C1_W { w: self }
    }
}
