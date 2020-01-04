#[doc = "Reader of register EDCBT"]
pub type R = crate::R<u32, super::EDCBT>;
#[doc = "Writer for register EDCBT"]
pub type W = crate::W<u32, super::EDCBT>;
#[doc = "Register EDCBT `reset()`'s with value 0"]
impl crate::ResetValue for super::EDCBT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTSEG1`"]
pub type DTSEG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTSEG1`"]
pub struct DTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `DTSEG2`"]
pub type DTSEG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTSEG2`"]
pub struct DTSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DRJW`"]
pub type DRJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRJW`"]
pub struct DRJW_W<'a> {
    w: &'a mut W,
}
impl<'a> DRJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Data Phase Segment 1"]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Data Phase Time Segment 2"]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - Data Phase Resynchronization Jump Width"]
    #[inline(always)]
    pub fn drjw(&self) -> DRJW_R {
        DRJW_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Data Phase Segment 1"]
    #[inline(always)]
    pub fn dtseg1(&mut self) -> DTSEG1_W {
        DTSEG1_W { w: self }
    }
    #[doc = "Bits 12:15 - Data Phase Time Segment 2"]
    #[inline(always)]
    pub fn dtseg2(&mut self) -> DTSEG2_W {
        DTSEG2_W { w: self }
    }
    #[doc = "Bits 22:25 - Data Phase Resynchronization Jump Width"]
    #[inline(always)]
    pub fn drjw(&mut self) -> DRJW_W {
        DRJW_W { w: self }
    }
}
