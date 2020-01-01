#[doc = "Reader of register SCR3L"]
pub type R = crate::R<u32, super::SCR3L>;
#[doc = "Writer for register SCR3L"]
pub type W = crate::W<u32, super::SCR3L>;
#[doc = "Register SCR3L `reset()`'s with value 0x0058_007d"]
impl crate::ResetValue for super::SCR3L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0058_007d
    }
}
#[doc = "Reader of field `RUN3_MAX`"]
pub type RUN3_MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RUN3_MAX`"]
pub struct RUN3_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN3_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Reader of field `RUN3_RNG`"]
pub type RUN3_RNG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RUN3_RNG`"]
pub struct RUN3_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN3_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 16)) | (((value as u32) & 0x1fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Run Length 3 Maximum Limit"]
    #[inline(always)]
    pub fn run3_max(&self) -> RUN3_MAX_R {
        RUN3_MAX_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Run Length 3 Range"]
    #[inline(always)]
    pub fn run3_rng(&self) -> RUN3_RNG_R {
        RUN3_RNG_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Run Length 3 Maximum Limit"]
    #[inline(always)]
    pub fn run3_max(&mut self) -> RUN3_MAX_W {
        RUN3_MAX_W { w: self }
    }
    #[doc = "Bits 16:28 - Run Length 3 Range"]
    #[inline(always)]
    pub fn run3_rng(&mut self) -> RUN3_RNG_W {
        RUN3_RNG_W { w: self }
    }
}
