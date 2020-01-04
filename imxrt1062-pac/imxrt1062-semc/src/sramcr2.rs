#[doc = "Reader of register SRAMCR2"]
pub type R = crate::R<u32, super::SRAMCR2>;
#[doc = "Writer for register SRAMCR2"]
pub type W = crate::W<u32, super::SRAMCR2>;
#[doc = "Register SRAMCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAMCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDS`"]
pub type WDS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDS`"]
pub struct WDS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `WDH`"]
pub type WDH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDH`"]
pub struct WDH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `TA`"]
pub type TA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TA`"]
pub struct TA_W<'a> {
    w: &'a mut W,
}
impl<'a> TA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `AWDH`"]
pub type AWDH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AWDH`"]
pub struct AWDH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `LC`"]
pub type LC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LC`"]
pub struct LC_W<'a> {
    w: &'a mut W,
}
impl<'a> LC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RD`"]
pub type RD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD`"]
pub struct RD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `CEITV`"]
pub type CEITV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CEITV`"]
pub struct CEITV_W<'a> {
    w: &'a mut W,
}
impl<'a> CEITV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RDH`"]
pub type RDH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDH`"]
pub struct RDH_W<'a> {
    w: &'a mut W,
}
impl<'a> RDH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Write Data setup time (WDS+1) cycle"]
    #[inline(always)]
    pub fn wds(&self) -> WDS_R {
        WDS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Write Data hold time WDH cycle"]
    #[inline(always)]
    pub fn wdh(&self) -> WDH_R {
        WDH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Turnaround time cycle"]
    #[inline(always)]
    pub fn ta(&self) -> TA_R {
        TA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Address to write data hold time cycle"]
    #[inline(always)]
    pub fn awdh(&self) -> AWDH_R {
        AWDH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Latency count"]
    #[inline(always)]
    pub fn lc(&self) -> LC_R {
        LC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Read cycle time"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CE# interval min time"]
    #[inline(always)]
    pub fn ceitv(&self) -> CEITV_R {
        CEITV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Read cycle hold time"]
    #[inline(always)]
    pub fn rdh(&self) -> RDH_R {
        RDH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Write Data setup time (WDS+1) cycle"]
    #[inline(always)]
    pub fn wds(&mut self) -> WDS_W {
        WDS_W { w: self }
    }
    #[doc = "Bits 4:7 - Write Data hold time WDH cycle"]
    #[inline(always)]
    pub fn wdh(&mut self) -> WDH_W {
        WDH_W { w: self }
    }
    #[doc = "Bits 8:11 - Turnaround time cycle"]
    #[inline(always)]
    pub fn ta(&mut self) -> TA_W {
        TA_W { w: self }
    }
    #[doc = "Bits 12:15 - Address to write data hold time cycle"]
    #[inline(always)]
    pub fn awdh(&mut self) -> AWDH_W {
        AWDH_W { w: self }
    }
    #[doc = "Bits 16:19 - Latency count"]
    #[inline(always)]
    pub fn lc(&mut self) -> LC_W {
        LC_W { w: self }
    }
    #[doc = "Bits 20:23 - Read cycle time"]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W { w: self }
    }
    #[doc = "Bits 24:27 - CE# interval min time"]
    #[inline(always)]
    pub fn ceitv(&mut self) -> CEITV_W {
        CEITV_W { w: self }
    }
    #[doc = "Bits 28:31 - Read cycle hold time"]
    #[inline(always)]
    pub fn rdh(&mut self) -> RDH_W {
        RDH_W { w: self }
    }
}
