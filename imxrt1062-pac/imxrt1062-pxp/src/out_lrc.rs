#[doc = "Reader of register OUT_LRC"]
pub type R = crate::R<u32, super::OUT_LRC>;
#[doc = "Writer for register OUT_LRC"]
pub type W = crate::W<u32, super::OUT_LRC>;
#[doc = "Register OUT_LRC `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_LRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Y`"]
pub type Y_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `Y`"]
pub struct Y_W<'a> {
    w: &'a mut W,
}
impl<'a> Y_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u8, u8>;
#[doc = "Reader of field `X`"]
pub type X_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `X`"]
pub struct X_W<'a> {
    w: &'a mut W,
}
impl<'a> X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | (((value as u32) & 0x3fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:13 - Indicates the number of vertical PIXELS in the output surface (non-rotated)"]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:15 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:29 - Indicates number of horizontal PIXELS in the output surface (non-rotated)"]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 30:31 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Indicates the number of vertical PIXELS in the output surface (non-rotated)"]
    #[inline(always)]
    pub fn y(&mut self) -> Y_W {
        Y_W { w: self }
    }
    #[doc = "Bits 16:29 - Indicates number of horizontal PIXELS in the output surface (non-rotated)"]
    #[inline(always)]
    pub fn x(&mut self) -> X_W {
        X_W { w: self }
    }
}
