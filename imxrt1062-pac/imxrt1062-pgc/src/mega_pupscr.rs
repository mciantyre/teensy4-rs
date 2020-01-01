#[doc = "Reader of register MEGA_PUPSCR"]
pub type R = crate::R<u32, super::MEGA_PUPSCR>;
#[doc = "Writer for register MEGA_PUPSCR"]
pub type W = crate::W<u32, super::MEGA_PUPSCR>;
#[doc = "Register MEGA_PUPSCR `reset()`'s with value 0x0f01"]
impl crate::ResetValue for super::MEGA_PUPSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f01
    }
}
#[doc = "Reader of field `SW`"]
pub type SW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW`"]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `SW2ISO`"]
pub type SW2ISO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW2ISO`"]
pub struct SW2ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> SW2ISO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b)"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - After asserting power toggle on/off signal (switch_b), the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    #[inline(always)]
    pub fn sw2iso(&self) -> SW2ISO_R {
        SW2ISO_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b)"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
    #[doc = "Bits 8:13 - After asserting power toggle on/off signal (switch_b), the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    #[inline(always)]
    pub fn sw2iso(&mut self) -> SW2ISO_W {
        SW2ISO_W { w: self }
    }
}
