#[doc = "Reader of register BMCR0"]
pub type R = crate::R<u32, super::BMCR0>;
#[doc = "Writer for register BMCR0"]
pub type W = crate::W<u32, super::BMCR0>;
#[doc = "Register BMCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BMCR0 {
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
#[doc = "Reader of field `WSH`"]
pub type WSH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WSH`"]
pub struct WSH_W<'a> {
    w: &'a mut W,
}
impl<'a> WSH_W<'a> {
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
    #[doc = "Bits 8:15 - Weight of Slave Hit (no read/write switch)"]
    #[inline(always)]
    pub fn wsh(&self) -> WSH_R {
        WSH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Weight of Slave Hit (Read/Write switch)"]
    #[inline(always)]
    pub fn wrws(&self) -> WRWS_R {
        WRWS_R::new(((self.bits >> 16) & 0xff) as u8)
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
    #[doc = "Bits 8:15 - Weight of Slave Hit (no read/write switch)"]
    #[inline(always)]
    pub fn wsh(&mut self) -> WSH_W {
        WSH_W { w: self }
    }
    #[doc = "Bits 16:23 - Weight of Slave Hit (Read/Write switch)"]
    #[inline(always)]
    pub fn wrws(&mut self) -> WRWS_W {
        WRWS_W { w: self }
    }
}
