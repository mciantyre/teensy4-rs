#[doc = "Reader of register ENCBT"]
pub type R = crate::R<u32, super::ENCBT>;
#[doc = "Writer for register ENCBT"]
pub type W = crate::W<u32, super::ENCBT>;
#[doc = "Register ENCBT `reset()`'s with value 0"]
impl crate::ResetValue for super::ENCBT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NTSEG1`"]
pub type NTSEG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NTSEG1`"]
pub struct NTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> NTSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `NTSEG2`"]
pub type NTSEG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NTSEG2`"]
pub struct NTSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> NTSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 12)) | (((value as u32) & 0x7f) << 12);
        self.w
    }
}
#[doc = "Reader of field `NRJW`"]
pub type NRJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NRJW`"]
pub struct NRJW_W<'a> {
    w: &'a mut W,
}
impl<'a> NRJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 22)) | (((value as u32) & 0x7f) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Nominal Time Segment 1"]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:18 - Nominal Time Segment 2"]
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bits 22:28 - Nominal Resynchronization Jump Width"]
    #[inline(always)]
    pub fn nrjw(&self) -> NRJW_R {
        NRJW_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Nominal Time Segment 1"]
    #[inline(always)]
    pub fn ntseg1(&mut self) -> NTSEG1_W {
        NTSEG1_W { w: self }
    }
    #[doc = "Bits 12:18 - Nominal Time Segment 2"]
    #[inline(always)]
    pub fn ntseg2(&mut self) -> NTSEG2_W {
        NTSEG2_W { w: self }
    }
    #[doc = "Bits 22:28 - Nominal Resynchronization Jump Width"]
    #[inline(always)]
    pub fn nrjw(&mut self) -> NRJW_W {
        NRJW_W { w: self }
    }
}
