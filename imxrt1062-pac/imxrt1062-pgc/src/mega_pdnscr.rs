#[doc = "Reader of register MEGA_PDNSCR"]
pub type R = crate::R<u32, super::MEGA_PDNSCR>;
#[doc = "Writer for register MEGA_PDNSCR"]
pub type W = crate::W<u32, super::MEGA_PDNSCR>;
#[doc = "Register MEGA_PDNSCR `reset()`'s with value 0x0101"]
impl crate::ResetValue for super::MEGA_PDNSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0101
    }
}
#[doc = "Reader of field `ISO`"]
pub type ISO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ISO`"]
pub struct ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `ISO2SW`"]
pub type ISO2SW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ISO2SW`"]
pub struct ISO2SW_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO2SW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    #[inline(always)]
    pub fn iso(&self) -> ISO_R {
        ISO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b)"]
    #[inline(always)]
    pub fn iso2sw(&self) -> ISO2SW_R {
        ISO2SW_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    #[inline(always)]
    pub fn iso(&mut self) -> ISO_W {
        ISO_W { w: self }
    }
    #[doc = "Bits 8:13 - After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b)"]
    #[inline(always)]
    pub fn iso2sw(&mut self) -> ISO2SW_W {
        ISO2SW_W { w: self }
    }
}
