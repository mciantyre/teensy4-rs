#[doc = "Reader of register SCR2L"]
pub type R = crate::R<u32, super::SCR2L>;
#[doc = "Writer for register SCR2L"]
pub type W = crate::W<u32, super::SCR2L>;
#[doc = "Register SCR2L `reset()`'s with value 0x007a_00dc"]
impl crate::ResetValue for super::SCR2L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x007a_00dc
    }
}
#[doc = "Reader of field `RUN2_MAX`"]
pub type RUN2_MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RUN2_MAX`"]
pub struct RUN2_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN2_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `RUN2_RNG`"]
pub type RUN2_RNG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RUN2_RNG`"]
pub struct RUN2_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN2_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | (((value as u32) & 0x3fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Run Length 2 Maximum Limit"]
    #[inline(always)]
    pub fn run2_max(&self) -> RUN2_MAX_R {
        RUN2_MAX_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Run Length 2 Range"]
    #[inline(always)]
    pub fn run2_rng(&self) -> RUN2_RNG_R {
        RUN2_RNG_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Run Length 2 Maximum Limit"]
    #[inline(always)]
    pub fn run2_max(&mut self) -> RUN2_MAX_W {
        RUN2_MAX_W { w: self }
    }
    #[doc = "Bits 16:29 - Run Length 2 Range"]
    #[inline(always)]
    pub fn run2_rng(&mut self) -> RUN2_RNG_W {
        RUN2_RNG_W { w: self }
    }
}
