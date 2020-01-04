#[doc = "Reader of register SCR1L"]
pub type R = crate::R<u32, super::SCR1L>;
#[doc = "Writer for register SCR1L"]
pub type W = crate::W<u32, super::SCR1L>;
#[doc = "Register SCR1L `reset()`'s with value 0x00b2_0195"]
impl crate::ResetValue for super::SCR1L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00b2_0195
    }
}
#[doc = "Reader of field `RUN1_MAX`"]
pub type RUN1_MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RUN1_MAX`"]
pub struct RUN1_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN1_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
#[doc = "Reader of field `RUN1_RNG`"]
pub type RUN1_RNG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RUN1_RNG`"]
pub struct RUN1_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN1_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | (((value as u32) & 0x7fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Run Length 1 Maximum Limit"]
    #[inline(always)]
    pub fn run1_max(&self) -> RUN1_MAX_R {
        RUN1_MAX_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Run Length 1 Range"]
    #[inline(always)]
    pub fn run1_rng(&self) -> RUN1_RNG_R {
        RUN1_RNG_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Run Length 1 Maximum Limit"]
    #[inline(always)]
    pub fn run1_max(&mut self) -> RUN1_MAX_W {
        RUN1_MAX_W { w: self }
    }
    #[doc = "Bits 16:30 - Run Length 1 Range"]
    #[inline(always)]
    pub fn run1_rng(&mut self) -> RUN1_RNG_W {
        RUN1_RNG_W { w: self }
    }
}
