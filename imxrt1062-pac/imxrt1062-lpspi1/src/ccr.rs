#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCKDIV`"]
pub type SCKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCKDIV`"]
pub struct SCKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DBT`"]
pub type DBT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBT`"]
pub struct DBT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PCSSCK`"]
pub type PCSSCK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCSSCK`"]
pub struct PCSSCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSSCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SCKPCS`"]
pub type SCKPCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCKPCS`"]
pub struct SCKPCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKPCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SCK Divider"]
    #[inline(always)]
    pub fn sckdiv(&self) -> SCKDIV_R {
        SCKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay Between Transfers"]
    #[inline(always)]
    pub fn dbt(&self) -> DBT_R {
        DBT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PCS-to-SCK Delay"]
    #[inline(always)]
    pub fn pcssck(&self) -> PCSSCK_R {
        PCSSCK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCK-to-PCS Delay"]
    #[inline(always)]
    pub fn sckpcs(&self) -> SCKPCS_R {
        SCKPCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCK Divider"]
    #[inline(always)]
    pub fn sckdiv(&mut self) -> SCKDIV_W {
        SCKDIV_W { w: self }
    }
    #[doc = "Bits 8:15 - Delay Between Transfers"]
    #[inline(always)]
    pub fn dbt(&mut self) -> DBT_W {
        DBT_W { w: self }
    }
    #[doc = "Bits 16:23 - PCS-to-SCK Delay"]
    #[inline(always)]
    pub fn pcssck(&mut self) -> PCSSCK_W {
        PCSSCK_W { w: self }
    }
    #[doc = "Bits 24:31 - SCK-to-PCS Delay"]
    #[inline(always)]
    pub fn sckpcs(&mut self) -> SCKPCS_W {
        SCKPCS_W { w: self }
    }
}
