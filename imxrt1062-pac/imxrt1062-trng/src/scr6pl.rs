#[doc = "Reader of register SCR6PL"]
pub type R = crate::R<u32, super::SCR6PL>;
#[doc = "Writer for register SCR6PL"]
pub type W = crate::W<u32, super::SCR6PL>;
#[doc = "Register SCR6PL `reset()`'s with value 0x002e_002f"]
impl crate::ResetValue for super::SCR6PL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x002e_002f
    }
}
#[doc = "Reader of field `RUN6P_MAX`"]
pub type RUN6P_MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RUN6P_MAX`"]
pub struct RUN6P_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN6P_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `RUN6P_RNG`"]
pub type RUN6P_RNG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RUN6P_RNG`"]
pub struct RUN6P_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN6P_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Run Length 6+ Maximum Limit"]
    #[inline(always)]
    pub fn run6p_max(&self) -> RUN6P_MAX_R {
        RUN6P_MAX_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Run Length 6+ Range"]
    #[inline(always)]
    pub fn run6p_rng(&self) -> RUN6P_RNG_R {
        RUN6P_RNG_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Run Length 6+ Maximum Limit"]
    #[inline(always)]
    pub fn run6p_max(&mut self) -> RUN6P_MAX_W {
        RUN6P_MAX_W { w: self }
    }
    #[doc = "Bits 16:26 - Run Length 6+ Range"]
    #[inline(always)]
    pub fn run6p_rng(&mut self) -> RUN6P_RNG_W {
        RUN6P_RNG_W { w: self }
    }
}
