#[doc = "Reader of register WTMK_LVL"]
pub type R = crate::R<u32, super::WTMK_LVL>;
#[doc = "Writer for register WTMK_LVL"]
pub type W = crate::W<u32, super::WTMK_LVL>;
#[doc = "Register WTMK_LVL `reset()`'s with value 0x0810_0810"]
impl crate::ResetValue for super::WTMK_LVL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0810_0810
    }
}
#[doc = "Reader of field `RD_WML`"]
pub type RD_WML_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_WML`"]
pub struct RD_WML_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_WML_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RD_BRST_LEN`"]
pub type RD_BRST_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_BRST_LEN`"]
pub struct RD_BRST_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_BRST_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `WR_WML`"]
pub type WR_WML_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WR_WML`"]
pub struct WR_WML_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_WML_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WR_BRST_LEN`"]
pub type WR_BRST_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WR_BRST_LEN`"]
pub struct WR_BRST_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_BRST_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Read Watermark Level"]
    #[inline(always)]
    pub fn rd_wml(&self) -> RD_WML_R {
        RD_WML_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Read Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline(always)]
    pub fn rd_brst_len(&self) -> RD_BRST_LEN_R {
        RD_BRST_LEN_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Write Watermark Level"]
    #[inline(always)]
    pub fn wr_wml(&self) -> WR_WML_R {
        WR_WML_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Write Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline(always)]
    pub fn wr_brst_len(&self) -> WR_BRST_LEN_R {
        WR_BRST_LEN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Watermark Level"]
    #[inline(always)]
    pub fn rd_wml(&mut self) -> RD_WML_W {
        RD_WML_W { w: self }
    }
    #[doc = "Bits 8:12 - Read Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline(always)]
    pub fn rd_brst_len(&mut self) -> RD_BRST_LEN_W {
        RD_BRST_LEN_W { w: self }
    }
    #[doc = "Bits 16:23 - Write Watermark Level"]
    #[inline(always)]
    pub fn wr_wml(&mut self) -> WR_WML_W {
        WR_WML_W { w: self }
    }
    #[doc = "Bits 24:28 - Write Burst Length Due to system restriction, the actual burst length may not exceed 16."]
    #[inline(always)]
    pub fn wr_brst_len(&mut self) -> WR_BRST_LEN_W {
        WR_BRST_LEN_W { w: self }
    }
}
