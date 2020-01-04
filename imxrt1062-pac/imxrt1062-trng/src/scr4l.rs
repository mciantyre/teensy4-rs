#[doc = "Reader of register SCR4L"]
pub type R = crate::R<u32, super::SCR4L>;
#[doc = "Writer for register SCR4L"]
pub type W = crate::W<u32, super::SCR4L>;
#[doc = "Register SCR4L `reset()`'s with value 0x0040_004b"]
impl crate::ResetValue for super::SCR4L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0040_004b
    }
}
#[doc = "Reader of field `RUN4_MAX`"]
pub type RUN4_MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RUN4_MAX`"]
pub struct RUN4_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN4_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `RUN4_RNG`"]
pub type RUN4_RNG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RUN4_RNG`"]
pub struct RUN4_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN4_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Run Length 4 Maximum Limit"]
    #[inline(always)]
    pub fn run4_max(&self) -> RUN4_MAX_R {
        RUN4_MAX_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Run Length 4 Range"]
    #[inline(always)]
    pub fn run4_rng(&self) -> RUN4_RNG_R {
        RUN4_RNG_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Run Length 4 Maximum Limit"]
    #[inline(always)]
    pub fn run4_max(&mut self) -> RUN4_MAX_W {
        RUN4_MAX_W { w: self }
    }
    #[doc = "Bits 16:27 - Run Length 4 Range"]
    #[inline(always)]
    pub fn run4_rng(&mut self) -> RUN4_RNG_W {
        RUN4_RNG_W { w: self }
    }
}
