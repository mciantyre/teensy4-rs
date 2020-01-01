#[doc = "Reader of register NORCR3"]
pub type R = crate::R<u32, super::NORCR3>;
#[doc = "Writer for register NORCR3"]
pub type W = crate::W<u32, super::NORCR3>;
#[doc = "Register NORCR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::NORCR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASSR`"]
pub type ASSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ASSR`"]
pub struct ASSR_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `AHSR`"]
pub type AHSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHSR`"]
pub struct AHSR_W<'a> {
    w: &'a mut W,
}
impl<'a> AHSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Address setup time for synchronous read"]
    #[inline(always)]
    pub fn assr(&self) -> ASSR_R {
        ASSR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address hold time for synchronous read"]
    #[inline(always)]
    pub fn ahsr(&self) -> AHSR_R {
        AHSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup time for synchronous read"]
    #[inline(always)]
    pub fn assr(&mut self) -> ASSR_W {
        ASSR_W { w: self }
    }
    #[doc = "Bits 4:7 - Address hold time for synchronous read"]
    #[inline(always)]
    pub fn ahsr(&mut self) -> AHSR_W {
        AHSR_W { w: self }
    }
}
