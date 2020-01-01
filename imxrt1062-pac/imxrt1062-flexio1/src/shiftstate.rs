#[doc = "Reader of register SHIFTSTATE"]
pub type R = crate::R<u32, super::SHIFTSTATE>;
#[doc = "Writer for register SHIFTSTATE"]
pub type W = crate::W<u32, super::SHIFTSTATE>;
#[doc = "Register SHIFTSTATE `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTSTATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Current State Pointer"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Current State Pointer"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
}
