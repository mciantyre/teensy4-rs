#[doc = "Reader of register DBICR1"]
pub type R = crate::R<u32, super::DBICR1>;
#[doc = "Writer for register DBICR1"]
pub type W = crate::W<u32, super::DBICR1>;
#[doc = "Register DBICR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DBICR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CES`"]
pub type CES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CES`"]
pub struct CES_W<'a> {
    w: &'a mut W,
}
impl<'a> CES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CEH`"]
pub type CEH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CEH`"]
pub struct CEH_W<'a> {
    w: &'a mut W,
}
impl<'a> CEH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `WEL`"]
pub type WEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WEL`"]
pub struct WEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `WEH`"]
pub type WEH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WEH`"]
pub struct WEH_W<'a> {
    w: &'a mut W,
}
impl<'a> WEH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `REL`"]
pub type REL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REL`"]
pub struct REL_W<'a> {
    w: &'a mut W,
}
impl<'a> REL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `REH`"]
pub type REH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REH`"]
pub struct REH_W<'a> {
    w: &'a mut W,
}
impl<'a> REH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 22)) | (((value as u32) & 0x3f) << 22);
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
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CSX Setup Time"]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CSX Hold Time"]
    #[inline(always)]
    pub fn ceh(&self) -> CEH_R {
        CEH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - WRX Low Time"]
    #[inline(always)]
    pub fn wel(&self) -> WEL_R {
        WEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - WRX High Time"]
    #[inline(always)]
    pub fn weh(&self) -> WEH_R {
        WEH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - RDX Low Time"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:27 - RDX High Time"]
    #[inline(always)]
    pub fn reh(&self) -> REH_R {
        REH_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31 - CSX interval min time"]
    #[inline(always)]
    pub fn ceitv(&self) -> CEITV_R {
        CEITV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CSX Setup Time"]
    #[inline(always)]
    pub fn ces(&mut self) -> CES_W {
        CES_W { w: self }
    }
    #[doc = "Bits 4:7 - CSX Hold Time"]
    #[inline(always)]
    pub fn ceh(&mut self) -> CEH_W {
        CEH_W { w: self }
    }
    #[doc = "Bits 8:11 - WRX Low Time"]
    #[inline(always)]
    pub fn wel(&mut self) -> WEL_W {
        WEL_W { w: self }
    }
    #[doc = "Bits 12:15 - WRX High Time"]
    #[inline(always)]
    pub fn weh(&mut self) -> WEH_W {
        WEH_W { w: self }
    }
    #[doc = "Bits 16:21 - RDX Low Time"]
    #[inline(always)]
    pub fn rel(&mut self) -> REL_W {
        REL_W { w: self }
    }
    #[doc = "Bits 22:27 - RDX High Time"]
    #[inline(always)]
    pub fn reh(&mut self) -> REH_W {
        REH_W { w: self }
    }
    #[doc = "Bits 28:31 - CSX interval min time"]
    #[inline(always)]
    pub fn ceitv(&mut self) -> CEITV_W {
        CEITV_W { w: self }
    }
}
