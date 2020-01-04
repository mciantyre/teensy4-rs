#[doc = "Reader of register SDRAMCR1"]
pub type R = crate::R<u32, super::SDRAMCR1>;
#[doc = "Writer for register SDRAMCR1"]
pub type W = crate::W<u32, super::SDRAMCR1>;
#[doc = "Register SDRAMCR1 `reset()`'s with value 0x0099_4934"]
impl crate::ResetValue for super::SDRAMCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0099_4934
    }
}
#[doc = "Reader of field `PRE2ACT`"]
pub type PRE2ACT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRE2ACT`"]
pub struct PRE2ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE2ACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ACT2RW`"]
pub type ACT2RW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT2RW`"]
pub struct ACT2RW_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT2RW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `RFRC`"]
pub type RFRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFRC`"]
pub struct RFRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RFRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `WRC`"]
pub type WRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRC`"]
pub struct WRC_W<'a> {
    w: &'a mut W,
}
impl<'a> WRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `CKEOFF`"]
pub type CKEOFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKEOFF`"]
pub struct CKEOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ACT2PRE`"]
pub type ACT2PRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT2PRE`"]
pub struct ACT2PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT2PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PRECHARGE to ACT/Refresh wait time"]
    #[inline(always)]
    pub fn pre2act(&self) -> PRE2ACT_R {
        PRE2ACT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ACT to Read/Write wait time"]
    #[inline(always)]
    pub fn act2rw(&self) -> ACT2RW_R {
        ACT2RW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Refresh recovery time"]
    #[inline(always)]
    pub fn rfrc(&self) -> RFRC_R {
        RFRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Write recovery time"]
    #[inline(always)]
    pub fn wrc(&self) -> WRC_R {
        WRC_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - CKE OFF minimum time"]
    #[inline(always)]
    pub fn ckeoff(&self) -> CKEOFF_R {
        CKEOFF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - ACT to Precharge minimum time"]
    #[inline(always)]
    pub fn act2pre(&self) -> ACT2PRE_R {
        ACT2PRE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PRECHARGE to ACT/Refresh wait time"]
    #[inline(always)]
    pub fn pre2act(&mut self) -> PRE2ACT_W {
        PRE2ACT_W { w: self }
    }
    #[doc = "Bits 4:7 - ACT to Read/Write wait time"]
    #[inline(always)]
    pub fn act2rw(&mut self) -> ACT2RW_W {
        ACT2RW_W { w: self }
    }
    #[doc = "Bits 8:12 - Refresh recovery time"]
    #[inline(always)]
    pub fn rfrc(&mut self) -> RFRC_W {
        RFRC_W { w: self }
    }
    #[doc = "Bits 13:15 - Write recovery time"]
    #[inline(always)]
    pub fn wrc(&mut self) -> WRC_W {
        WRC_W { w: self }
    }
    #[doc = "Bits 16:19 - CKE OFF minimum time"]
    #[inline(always)]
    pub fn ckeoff(&mut self) -> CKEOFF_W {
        CKEOFF_W { w: self }
    }
    #[doc = "Bits 20:23 - ACT to Precharge minimum time"]
    #[inline(always)]
    pub fn act2pre(&mut self) -> ACT2PRE_W {
        ACT2PRE_W { w: self }
    }
}
