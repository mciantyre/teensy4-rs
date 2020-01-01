#[doc = "Reader of register FORCE_EVENT"]
pub type R = crate::R<u32, super::FORCE_EVENT>;
#[doc = "Writer for register FORCE_EVENT"]
pub type W = crate::W<u32, super::FORCE_EVENT>;
#[doc = "Register FORCE_EVENT `reset()`'s with value 0"]
impl crate::ResetValue for super::FORCE_EVENT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FEVTAC12NE`"]
pub type FEVTAC12NE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTAC12NE`"]
pub struct FEVTAC12NE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTAC12NE_W<'a> {
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
#[doc = "Reader of field `FEVTAC12TOE`"]
pub type FEVTAC12TOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTAC12TOE`"]
pub struct FEVTAC12TOE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTAC12TOE_W<'a> {
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
#[doc = "Reader of field `FEVTAC12CE`"]
pub type FEVTAC12CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTAC12CE`"]
pub struct FEVTAC12CE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTAC12CE_W<'a> {
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
#[doc = "Reader of field `FEVTAC12EBE`"]
pub type FEVTAC12EBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTAC12EBE`"]
pub struct FEVTAC12EBE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTAC12EBE_W<'a> {
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
#[doc = "Reader of field `FEVTAC12IE`"]
pub type FEVTAC12IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTAC12IE`"]
pub struct FEVTAC12IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTAC12IE_W<'a> {
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
#[doc = "Reader of field `FEVTCNIBAC12E`"]
pub type FEVTCNIBAC12E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTCNIBAC12E`"]
pub struct FEVTCNIBAC12E_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTCNIBAC12E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FEVTCTOE`"]
pub type FEVTCTOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTCTOE`"]
pub struct FEVTCTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTCTOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `FEVTCCE`"]
pub type FEVTCCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTCCE`"]
pub struct FEVTCCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTCCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `FEVTCEBE`"]
pub type FEVTCEBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTCEBE`"]
pub struct FEVTCEBE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTCEBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `FEVTCIE`"]
pub type FEVTCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTCIE`"]
pub struct FEVTCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTCIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `FEVTDTOE`"]
pub type FEVTDTOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTDTOE`"]
pub struct FEVTDTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTDTOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `FEVTDCE`"]
pub type FEVTDCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTDCE`"]
pub struct FEVTDCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTDCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `FEVTDEBE`"]
pub type FEVTDEBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTDEBE`"]
pub struct FEVTDEBE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTDEBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `FEVTAC12E`"]
pub type FEVTAC12E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTAC12E`"]
pub struct FEVTAC12E_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTAC12E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `FEVTTNE`"]
pub type FEVTTNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTTNE`"]
pub struct FEVTTNE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTTNE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `FEVTDMAE`"]
pub type FEVTDMAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTDMAE`"]
pub struct FEVTDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTDMAE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `FEVTCINT`"]
pub type FEVTCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEVTCINT`"]
pub struct FEVTCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> FEVTCINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Force Event Auto Command 12 Not Executed"]
    #[inline(always)]
    pub fn fevtac12ne(&self) -> FEVTAC12NE_R {
        FEVTAC12NE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force Event Auto Command 12 Time Out Error"]
    #[inline(always)]
    pub fn fevtac12toe(&self) -> FEVTAC12TOE_R {
        FEVTAC12TOE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force Event Auto Command 12 CRC Error"]
    #[inline(always)]
    pub fn fevtac12ce(&self) -> FEVTAC12CE_R {
        FEVTAC12CE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force Event Auto Command 12 End Bit Error"]
    #[inline(always)]
    pub fn fevtac12ebe(&self) -> FEVTAC12EBE_R {
        FEVTAC12EBE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force Event Auto Command 12 Index Error"]
    #[inline(always)]
    pub fn fevtac12ie(&self) -> FEVTAC12IE_R {
        FEVTAC12IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force Event Command Not Executed By Auto Command 12 Error"]
    #[inline(always)]
    pub fn fevtcnibac12e(&self) -> FEVTCNIBAC12E_R {
        FEVTCNIBAC12E_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Force Event Command Time Out Error"]
    #[inline(always)]
    pub fn fevtctoe(&self) -> FEVTCTOE_R {
        FEVTCTOE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Force Event Command CRC Error"]
    #[inline(always)]
    pub fn fevtcce(&self) -> FEVTCCE_R {
        FEVTCCE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Force Event Command End Bit Error"]
    #[inline(always)]
    pub fn fevtcebe(&self) -> FEVTCEBE_R {
        FEVTCEBE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Force Event Command Index Error"]
    #[inline(always)]
    pub fn fevtcie(&self) -> FEVTCIE_R {
        FEVTCIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Force Event Data Time Out Error"]
    #[inline(always)]
    pub fn fevtdtoe(&self) -> FEVTDTOE_R {
        FEVTDTOE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Force Event Data CRC Error"]
    #[inline(always)]
    pub fn fevtdce(&self) -> FEVTDCE_R {
        FEVTDCE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Force Event Data End Bit Error"]
    #[inline(always)]
    pub fn fevtdebe(&self) -> FEVTDEBE_R {
        FEVTDEBE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Force Event Auto Command 12 Error"]
    #[inline(always)]
    pub fn fevtac12e(&self) -> FEVTAC12E_R {
        FEVTAC12E_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Force Tuning Error"]
    #[inline(always)]
    pub fn fevttne(&self) -> FEVTTNE_R {
        FEVTTNE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Force Event DMA Error"]
    #[inline(always)]
    pub fn fevtdmae(&self) -> FEVTDMAE_R {
        FEVTDMAE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Force Event Card Interrupt"]
    #[inline(always)]
    pub fn fevtcint(&self) -> FEVTCINT_R {
        FEVTCINT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event Auto Command 12 Not Executed"]
    #[inline(always)]
    pub fn fevtac12ne(&mut self) -> FEVTAC12NE_W {
        FEVTAC12NE_W { w: self }
    }
    #[doc = "Bit 1 - Force Event Auto Command 12 Time Out Error"]
    #[inline(always)]
    pub fn fevtac12toe(&mut self) -> FEVTAC12TOE_W {
        FEVTAC12TOE_W { w: self }
    }
    #[doc = "Bit 2 - Force Event Auto Command 12 CRC Error"]
    #[inline(always)]
    pub fn fevtac12ce(&mut self) -> FEVTAC12CE_W {
        FEVTAC12CE_W { w: self }
    }
    #[doc = "Bit 3 - Force Event Auto Command 12 End Bit Error"]
    #[inline(always)]
    pub fn fevtac12ebe(&mut self) -> FEVTAC12EBE_W {
        FEVTAC12EBE_W { w: self }
    }
    #[doc = "Bit 4 - Force Event Auto Command 12 Index Error"]
    #[inline(always)]
    pub fn fevtac12ie(&mut self) -> FEVTAC12IE_W {
        FEVTAC12IE_W { w: self }
    }
    #[doc = "Bit 7 - Force Event Command Not Executed By Auto Command 12 Error"]
    #[inline(always)]
    pub fn fevtcnibac12e(&mut self) -> FEVTCNIBAC12E_W {
        FEVTCNIBAC12E_W { w: self }
    }
    #[doc = "Bit 16 - Force Event Command Time Out Error"]
    #[inline(always)]
    pub fn fevtctoe(&mut self) -> FEVTCTOE_W {
        FEVTCTOE_W { w: self }
    }
    #[doc = "Bit 17 - Force Event Command CRC Error"]
    #[inline(always)]
    pub fn fevtcce(&mut self) -> FEVTCCE_W {
        FEVTCCE_W { w: self }
    }
    #[doc = "Bit 18 - Force Event Command End Bit Error"]
    #[inline(always)]
    pub fn fevtcebe(&mut self) -> FEVTCEBE_W {
        FEVTCEBE_W { w: self }
    }
    #[doc = "Bit 19 - Force Event Command Index Error"]
    #[inline(always)]
    pub fn fevtcie(&mut self) -> FEVTCIE_W {
        FEVTCIE_W { w: self }
    }
    #[doc = "Bit 20 - Force Event Data Time Out Error"]
    #[inline(always)]
    pub fn fevtdtoe(&mut self) -> FEVTDTOE_W {
        FEVTDTOE_W { w: self }
    }
    #[doc = "Bit 21 - Force Event Data CRC Error"]
    #[inline(always)]
    pub fn fevtdce(&mut self) -> FEVTDCE_W {
        FEVTDCE_W { w: self }
    }
    #[doc = "Bit 22 - Force Event Data End Bit Error"]
    #[inline(always)]
    pub fn fevtdebe(&mut self) -> FEVTDEBE_W {
        FEVTDEBE_W { w: self }
    }
    #[doc = "Bit 24 - Force Event Auto Command 12 Error"]
    #[inline(always)]
    pub fn fevtac12e(&mut self) -> FEVTAC12E_W {
        FEVTAC12E_W { w: self }
    }
    #[doc = "Bit 26 - Force Tuning Error"]
    #[inline(always)]
    pub fn fevttne(&mut self) -> FEVTTNE_W {
        FEVTTNE_W { w: self }
    }
    #[doc = "Bit 28 - Force Event DMA Error"]
    #[inline(always)]
    pub fn fevtdmae(&mut self) -> FEVTDMAE_W {
        FEVTDMAE_W { w: self }
    }
    #[doc = "Bit 31 - Force Event Card Interrupt"]
    #[inline(always)]
    pub fn fevtcint(&mut self) -> FEVTCINT_W {
        FEVTCINT_W { w: self }
    }
}
