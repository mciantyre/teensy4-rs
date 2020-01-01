#[doc = "Reader of register SCML"]
pub type R = crate::R<u32, super::SCML>;
#[doc = "Writer for register SCML"]
pub type W = crate::W<u32, super::SCML>;
#[doc = "Register SCML `reset()`'s with value 0x010c_0568"]
impl crate::ResetValue for super::SCML {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x010c_0568
    }
}
#[doc = "Reader of field `MONO_MAX`"]
pub type MONO_MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MONO_MAX`"]
pub struct MONO_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `MONO_RNG`"]
pub type MONO_RNG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MONO_RNG`"]
pub struct MONO_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_RNG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Monobit Maximum Limit"]
    #[inline(always)]
    pub fn mono_max(&self) -> MONO_MAX_R {
        MONO_MAX_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Monobit Range"]
    #[inline(always)]
    pub fn mono_rng(&self) -> MONO_RNG_R {
        MONO_RNG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Monobit Maximum Limit"]
    #[inline(always)]
    pub fn mono_max(&mut self) -> MONO_MAX_W {
        MONO_MAX_W { w: self }
    }
    #[doc = "Bits 16:31 - Monobit Range"]
    #[inline(always)]
    pub fn mono_rng(&mut self) -> MONO_RNG_W {
        MONO_RNG_W { w: self }
    }
}
