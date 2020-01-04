#[doc = "Reader of register NANDCR3"]
pub type R = crate::R<u32, super::NANDCR3>;
#[doc = "Writer for register NANDCR3"]
pub type W = crate::W<u32, super::NANDCR3>;
#[doc = "Register NANDCR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::NANDCR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NDOPT1`"]
pub type NDOPT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NDOPT1`"]
pub struct NDOPT1_W<'a> {
    w: &'a mut W,
}
impl<'a> NDOPT1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `NDOPT2`"]
pub type NDOPT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NDOPT2`"]
pub struct NDOPT2_W<'a> {
    w: &'a mut W,
}
impl<'a> NDOPT2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `NDOPT3`"]
pub type NDOPT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NDOPT3`"]
pub struct NDOPT3_W<'a> {
    w: &'a mut W,
}
impl<'a> NDOPT3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLE`"]
pub type CLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLE`"]
pub struct CLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RDS`"]
pub type RDS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDS`"]
pub struct RDS_W<'a> {
    w: &'a mut W,
}
impl<'a> RDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
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
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
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
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
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
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - NAND option bit 1"]
    #[inline(always)]
    pub fn ndopt1(&self) -> NDOPT1_R {
        NDOPT1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NAND option bit 2"]
    #[inline(always)]
    pub fn ndopt2(&self) -> NDOPT2_R {
        NDOPT2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NAND option bit 3"]
    #[inline(always)]
    pub fn ndopt3(&self) -> NDOPT3_R {
        NDOPT3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NAND CLE Option"]
    #[inline(always)]
    pub fn cle(&self) -> CLE_R {
        CLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Read Data Setup cycle time."]
    #[inline(always)]
    pub fn rds(&self) -> RDS_R {
        RDS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Read Data Hold cycle time."]
    #[inline(always)]
    pub fn rdh(&self) -> RDH_R {
        RDH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Write Data Setup cycle time."]
    #[inline(always)]
    pub fn wds(&self) -> WDS_R {
        WDS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Write Data Hold cycle time."]
    #[inline(always)]
    pub fn wdh(&self) -> WDH_R {
        WDH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - NAND option bit 1"]
    #[inline(always)]
    pub fn ndopt1(&mut self) -> NDOPT1_W {
        NDOPT1_W { w: self }
    }
    #[doc = "Bit 1 - NAND option bit 2"]
    #[inline(always)]
    pub fn ndopt2(&mut self) -> NDOPT2_W {
        NDOPT2_W { w: self }
    }
    #[doc = "Bit 2 - NAND option bit 3"]
    #[inline(always)]
    pub fn ndopt3(&mut self) -> NDOPT3_W {
        NDOPT3_W { w: self }
    }
    #[doc = "Bit 3 - NAND CLE Option"]
    #[inline(always)]
    pub fn cle(&mut self) -> CLE_W {
        CLE_W { w: self }
    }
    #[doc = "Bits 16:19 - Read Data Setup cycle time."]
    #[inline(always)]
    pub fn rds(&mut self) -> RDS_W {
        RDS_W { w: self }
    }
    #[doc = "Bits 20:23 - Read Data Hold cycle time."]
    #[inline(always)]
    pub fn rdh(&mut self) -> RDH_W {
        RDH_W { w: self }
    }
    #[doc = "Bits 24:27 - Write Data Setup cycle time."]
    #[inline(always)]
    pub fn wds(&mut self) -> WDS_W {
        WDS_W { w: self }
    }
    #[doc = "Bits 28:31 - Write Data Hold cycle time."]
    #[inline(always)]
    pub fn wdh(&mut self) -> WDH_W {
        WDH_W { w: self }
    }
}
