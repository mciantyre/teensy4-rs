#[doc = "Reader of register BMCR1"]
pub type R = crate::R<u32, super::BMCR1>;
#[doc = "Writer for register BMCR1"]
pub type W = crate::W<u32, super::BMCR1>;
#[doc = "Register BMCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BMCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WQOS`"]
pub type WQOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WQOS`"]
pub struct WQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WQOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `WAGE`"]
pub type WAGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAGE`"]
pub struct WAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `WPH`"]
pub type WPH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WPH`"]
pub struct WPH_W<'a> {
    w: &'a mut W,
}
impl<'a> WPH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `WRWS`"]
pub type WRWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRWS`"]
pub struct WRWS_W<'a> {
    w: &'a mut W,
}
impl<'a> WRWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WBR`"]
pub type WBR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WBR`"]
pub struct WBR_W<'a> {
    w: &'a mut W,
}
impl<'a> WBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Weight of QoS"]
    #[inline(always)]
    pub fn wqos(&self) -> WQOS_R {
        WQOS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Weight of Aging"]
    #[inline(always)]
    pub fn wage(&self) -> WAGE_R {
        WAGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Weight of Page Hit"]
    #[inline(always)]
    pub fn wph(&self) -> WPH_R {
        WPH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Weight of Read/Write switch"]
    #[inline(always)]
    pub fn wrws(&self) -> WRWS_R {
        WRWS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Weight of Bank Rotation"]
    #[inline(always)]
    pub fn wbr(&self) -> WBR_R {
        WBR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Weight of QoS"]
    #[inline(always)]
    pub fn wqos(&mut self) -> WQOS_W {
        WQOS_W { w: self }
    }
    #[doc = "Bits 4:7 - Weight of Aging"]
    #[inline(always)]
    pub fn wage(&mut self) -> WAGE_W {
        WAGE_W { w: self }
    }
    #[doc = "Bits 8:15 - Weight of Page Hit"]
    #[inline(always)]
    pub fn wph(&mut self) -> WPH_W {
        WPH_W { w: self }
    }
    #[doc = "Bits 16:23 - Weight of Read/Write switch"]
    #[inline(always)]
    pub fn wrws(&mut self) -> WRWS_W {
        WRWS_W { w: self }
    }
    #[doc = "Bits 24:31 - Weight of Bank Rotation"]
    #[inline(always)]
    pub fn wbr(&mut self) -> WBR_W {
        WBR_W { w: self }
    }
}
