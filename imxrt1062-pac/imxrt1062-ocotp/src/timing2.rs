#[doc = "Reader of register TIMING2"]
pub type R = crate::R<u32, super::TIMING2>;
#[doc = "Writer for register TIMING2"]
pub type W = crate::W<u32, super::TIMING2>;
#[doc = "Register TIMING2 `reset()`'s with value 0x01c3_0092"]
impl crate::ResetValue for super::TIMING2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01c3_0092
    }
}
#[doc = "Reader of field `RELAX_PROG`"]
pub type RELAX_PROG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RELAX_PROG`"]
pub struct RELAX_PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> RELAX_PROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `RSRVD0`"]
pub type RSRVD0_R = crate::R<u8, u8>;
#[doc = "Reader of field `RELAX_READ`"]
pub type RELAX_READ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RELAX_READ`"]
pub struct RELAX_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> RELAX_READ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RSRVD1`"]
pub type RSRVD1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - RELAX_PROG"]
    #[inline(always)]
    pub fn relax_prog(&self) -> RELAX_PROG_R {
        RELAX_PROG_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - RSRVD0"]
    #[inline(always)]
    pub fn rsrvd0(&self) -> RSRVD0_R {
        RSRVD0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - RELAX_READ"]
    #[inline(always)]
    pub fn relax_read(&self) -> RELAX_READ_R {
        RELAX_READ_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:31 - RSRVD0"]
    #[inline(always)]
    pub fn rsrvd1(&self) -> RSRVD1_R {
        RSRVD1_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RELAX_PROG"]
    #[inline(always)]
    pub fn relax_prog(&mut self) -> RELAX_PROG_W {
        RELAX_PROG_W { w: self }
    }
    #[doc = "Bits 16:21 - RELAX_READ"]
    #[inline(always)]
    pub fn relax_read(&mut self) -> RELAX_READ_W {
        RELAX_READ_W { w: self }
    }
}
