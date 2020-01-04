#[doc = "Reader of register SCR5L"]
pub type R = crate::R<u32, super::SCR5L>;
#[doc = "Writer for register SCR5L"]
pub type W = crate::W<u32, super::SCR5L>;
#[doc = "Register SCR5L `reset()`'s with value 0x002e_002f"]
impl crate::ResetValue for super::SCR5L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x002e_002f
    }
}
#[doc = "Reader of field `RUN5_MAX`"]
pub type RUN5_MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RUN5_MAX`"]
pub struct RUN5_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN5_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `RUN5_RNG`"]
pub type RUN5_RNG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RUN5_RNG`"]
pub struct RUN5_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN5_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Run Length 5 Maximum Limit"]
    #[inline(always)]
    pub fn run5_max(&self) -> RUN5_MAX_R {
        RUN5_MAX_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Run Length 5 Range"]
    #[inline(always)]
    pub fn run5_rng(&self) -> RUN5_RNG_R {
        RUN5_RNG_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Run Length 5 Maximum Limit"]
    #[inline(always)]
    pub fn run5_max(&mut self) -> RUN5_MAX_W {
        RUN5_MAX_W { w: self }
    }
    #[doc = "Bits 16:26 - Run Length 5 Range"]
    #[inline(always)]
    pub fn run5_rng(&mut self) -> RUN5_RNG_W {
        RUN5_RNG_W { w: self }
    }
}
