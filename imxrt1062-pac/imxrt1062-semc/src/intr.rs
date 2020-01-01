#[doc = "Reader of register INTR"]
pub type R = crate::R<u32, super::INTR>;
#[doc = "Writer for register INTR"]
pub type W = crate::W<u32, super::INTR>;
#[doc = "Register INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IPCMDDONE`"]
pub type IPCMDDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPCMDDONE`"]
pub struct IPCMDDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCMDDONE_W<'a> {
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
#[doc = "Reader of field `IPCMDERR`"]
pub type IPCMDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPCMDERR`"]
pub struct IPCMDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCMDERR_W<'a> {
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
#[doc = "Reader of field `AXICMDERR`"]
pub type AXICMDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXICMDERR`"]
pub struct AXICMDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AXICMDERR_W<'a> {
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
#[doc = "Reader of field `AXIBUSERR`"]
pub type AXIBUSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXIBUSERR`"]
pub struct AXIBUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIBUSERR_W<'a> {
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
#[doc = "Reader of field `NDPAGEEND`"]
pub type NDPAGEEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NDPAGEEND`"]
pub struct NDPAGEEND_W<'a> {
    w: &'a mut W,
}
impl<'a> NDPAGEEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `NDNOPEND`"]
pub type NDNOPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NDNOPEND`"]
pub struct NDNOPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> NDNOPEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IP command normal done interrupt"]
    #[inline(always)]
    pub fn ipcmddone(&self) -> IPCMDDONE_R {
        IPCMDDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IP command error done interrupt"]
    #[inline(always)]
    pub fn ipcmderr(&self) -> IPCMDERR_R {
        IPCMDERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AXI command error interrupt"]
    #[inline(always)]
    pub fn axicmderr(&self) -> AXICMDERR_R {
        AXICMDERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AXI bus error interrupt"]
    #[inline(always)]
    pub fn axibuserr(&self) -> AXIBUSERR_R {
        AXIBUSERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This interrupt is generated when the last address of one page in NAND device is written by AXI command"]
    #[inline(always)]
    pub fn ndpageend(&self) -> NDPAGEEND_R {
        NDPAGEEND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This interrupt is generated when all pending AXI write command to NAND is finished on NAND interface."]
    #[inline(always)]
    pub fn ndnopend(&self) -> NDNOPEND_R {
        NDNOPEND_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP command normal done interrupt"]
    #[inline(always)]
    pub fn ipcmddone(&mut self) -> IPCMDDONE_W {
        IPCMDDONE_W { w: self }
    }
    #[doc = "Bit 1 - IP command error done interrupt"]
    #[inline(always)]
    pub fn ipcmderr(&mut self) -> IPCMDERR_W {
        IPCMDERR_W { w: self }
    }
    #[doc = "Bit 2 - AXI command error interrupt"]
    #[inline(always)]
    pub fn axicmderr(&mut self) -> AXICMDERR_W {
        AXICMDERR_W { w: self }
    }
    #[doc = "Bit 3 - AXI bus error interrupt"]
    #[inline(always)]
    pub fn axibuserr(&mut self) -> AXIBUSERR_W {
        AXIBUSERR_W { w: self }
    }
    #[doc = "Bit 4 - This interrupt is generated when the last address of one page in NAND device is written by AXI command"]
    #[inline(always)]
    pub fn ndpageend(&mut self) -> NDPAGEEND_W {
        NDPAGEEND_W { w: self }
    }
    #[doc = "Bit 5 - This interrupt is generated when all pending AXI write command to NAND is finished on NAND interface."]
    #[inline(always)]
    pub fn ndnopend(&mut self) -> NDNOPEND_W {
        NDNOPEND_W { w: self }
    }
}
