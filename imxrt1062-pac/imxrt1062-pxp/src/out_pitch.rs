#[doc = "Reader of register OUT_PITCH"]
pub type R = crate::R<u32, super::OUT_PITCH>;
#[doc = "Writer for register OUT_PITCH"]
pub type W = crate::W<u32, super::OUT_PITCH>;
#[doc = "Register OUT_PITCH `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT_PITCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PITCH`"]
pub type PITCH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PITCH`"]
pub struct PITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PITCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `RSVD`"]
pub type RSVD_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Indicates the number of bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub fn pitch(&self) -> PITCH_R {
        PITCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd(&self) -> RSVD_R {
        RSVD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates the number of bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub fn pitch(&mut self) -> PITCH_W {
        PITCH_W { w: self }
    }
}
