#[doc = "Reader of register TIMING"]
pub type R = crate::R<u32, super::TIMING>;
#[doc = "Writer for register TIMING"]
pub type W = crate::W<u32, super::TIMING>;
#[doc = "Register TIMING `reset()`'s with value 0x0c0d_9755"]
impl crate::ResetValue for super::TIMING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c0d_9755
    }
}
#[doc = "Reader of field `STROBE_PROG`"]
pub type STROBE_PROG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STROBE_PROG`"]
pub struct STROBE_PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_PROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `RELAX`"]
pub type RELAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RELAX`"]
pub struct RELAX_W<'a> {
    w: &'a mut W,
}
impl<'a> RELAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `STROBE_READ`"]
pub type STROBE_READ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STROBE_READ`"]
pub struct STROBE_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_READ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `WAIT`"]
pub type WAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAIT`"]
pub struct WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 22)) | (((value as u32) & 0x3f) << 22);
        self.w
    }
}
#[doc = "Reader of field `RSRVD0`"]
pub type RSRVD0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - STROBE_PROG"]
    #[inline(always)]
    pub fn strobe_prog(&self) -> STROBE_PROG_R {
        STROBE_PROG_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - RELAX"]
    #[inline(always)]
    pub fn relax(&self) -> RELAX_R {
        RELAX_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - STROBE_READ"]
    #[inline(always)]
    pub fn strobe_read(&self) -> STROBE_READ_R {
        STROBE_READ_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - WAIT"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31 - RSRVD0"]
    #[inline(always)]
    pub fn rsrvd0(&self) -> RSRVD0_R {
        RSRVD0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - STROBE_PROG"]
    #[inline(always)]
    pub fn strobe_prog(&mut self) -> STROBE_PROG_W {
        STROBE_PROG_W { w: self }
    }
    #[doc = "Bits 12:15 - RELAX"]
    #[inline(always)]
    pub fn relax(&mut self) -> RELAX_W {
        RELAX_W { w: self }
    }
    #[doc = "Bits 16:21 - STROBE_READ"]
    #[inline(always)]
    pub fn strobe_read(&mut self) -> STROBE_READ_W {
        STROBE_READ_W { w: self }
    }
    #[doc = "Bits 22:27 - WAIT"]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W {
        WAIT_W { w: self }
    }
}
