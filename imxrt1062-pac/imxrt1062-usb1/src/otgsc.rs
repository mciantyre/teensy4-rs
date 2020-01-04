#[doc = "Reader of register OTGSC"]
pub type R = crate::R<u32, super::OTGSC>;
#[doc = "Writer for register OTGSC"]
pub type W = crate::W<u32, super::OTGSC>;
#[doc = "Register OTGSC `reset()`'s with value 0x1120"]
impl crate::ResetValue for super::OTGSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1120
    }
}
#[doc = "Reader of field `VD`"]
pub type VD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VD`"]
pub struct VD_W<'a> {
    w: &'a mut W,
}
impl<'a> VD_W<'a> {
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
#[doc = "Reader of field `VC`"]
pub type VC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VC`"]
pub struct VC_W<'a> {
    w: &'a mut W,
}
impl<'a> VC_W<'a> {
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
#[doc = "Reader of field `OT`"]
pub type OT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OT`"]
pub struct OT_W<'a> {
    w: &'a mut W,
}
impl<'a> OT_W<'a> {
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
#[doc = "Reader of field `DP`"]
pub type DP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DP`"]
pub struct DP_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_W<'a> {
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
#[doc = "Reader of field `IDPU`"]
pub type IDPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDPU`"]
pub struct IDPU_W<'a> {
    w: &'a mut W,
}
impl<'a> IDPU_W<'a> {
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
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<bool, bool>;
#[doc = "Reader of field `AVV`"]
pub type AVV_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASV`"]
pub type ASV_R = crate::R<bool, bool>;
#[doc = "Reader of field `BSV`"]
pub type BSV_R = crate::R<bool, bool>;
#[doc = "Reader of field `BSE`"]
pub type BSE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOG_1MS`"]
pub type TOG_1MS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DPS`"]
pub type DPS_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDIS`"]
pub type IDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDIS`"]
pub struct IDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IDIS_W<'a> {
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
#[doc = "Reader of field `AVVIS`"]
pub type AVVIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVVIS`"]
pub struct AVVIS_W<'a> {
    w: &'a mut W,
}
impl<'a> AVVIS_W<'a> {
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
#[doc = "Reader of field `ASVIS`"]
pub type ASVIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASVIS`"]
pub struct ASVIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ASVIS_W<'a> {
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
#[doc = "Reader of field `BSVIS`"]
pub type BSVIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSVIS`"]
pub struct BSVIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BSVIS_W<'a> {
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
#[doc = "Reader of field `BSEIS`"]
pub type BSEIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSEIS`"]
pub struct BSEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BSEIS_W<'a> {
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
#[doc = "Reader of field `STATUS_1MS`"]
pub type STATUS_1MS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATUS_1MS`"]
pub struct STATUS_1MS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_1MS_W<'a> {
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
#[doc = "Reader of field `DPIS`"]
pub type DPIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPIS`"]
pub struct DPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPIS_W<'a> {
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
#[doc = "Reader of field `IDIE`"]
pub type IDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDIE`"]
pub struct IDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDIE_W<'a> {
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
#[doc = "Reader of field `AVVIE`"]
pub type AVVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVVIE`"]
pub struct AVVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AVVIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `ASVIE`"]
pub type ASVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASVIE`"]
pub struct ASVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASVIE_W<'a> {
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
#[doc = "Reader of field `BSVIE`"]
pub type BSVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSVIE`"]
pub struct BSVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BSVIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `BSEIE`"]
pub type BSEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSEIE`"]
pub struct BSEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BSEIE_W<'a> {
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
#[doc = "Reader of field `EN_1MS`"]
pub type EN_1MS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_1MS`"]
pub struct EN_1MS_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_1MS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `DPIE`"]
pub type DPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPIE`"]
pub struct DPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
    #[inline(always)]
    pub fn vd(&self) -> VD_R {
        VD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VBUS Charge - Read/Write"]
    #[inline(always)]
    pub fn vc(&self) -> VC_R {
        VC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OTG Termination - Read/Write"]
    #[inline(always)]
    pub fn ot(&self) -> OT_R {
        OT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Pulsing - Read/Write"]
    #[inline(always)]
    pub fn dp(&self) -> DP_R {
        DP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]"]
    #[inline(always)]
    pub fn idpu(&self) -> IDPU_R {
        IDPU_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB ID - Read Only. 0 = A device, 1 = B device"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold."]
    #[inline(always)]
    pub fn avv(&self) -> AVV_R {
        AVV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - A Session Valid - Read Only. Indicates VBus is above the A session valid threshold."]
    #[inline(always)]
    pub fn asv(&self) -> ASV_R {
        ASV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B Session Valid - Read Only. Indicates VBus is above the B session valid threshold."]
    #[inline(always)]
    pub fn bsv(&self) -> BSV_R {
        BSV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B Session End - Read Only. Indicates VBus is below the B session end threshold."]
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 1 millisecond timer toggle - Read Only. This bit toggles once per millisecond."]
    #[inline(always)]
    pub fn tog_1ms(&self) -> TOG_1MS_R {
        TOG_1MS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Data Bus Pulsing Status - Read Only"]
    #[inline(always)]
    pub fn dps(&self) -> DPS_R {
        DPS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB ID Interrupt Status - Read/Write"]
    #[inline(always)]
    pub fn idis(&self) -> IDIS_R {
        IDIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - A VBus Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn avvis(&self) -> AVVIS_R {
        AVVIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A Session Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn asvis(&self) -> ASVIS_R {
        ASVIS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B Session Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn bsvis(&self) -> BSVIS_R {
        BSVIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B Session End Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn bseis(&self) -> BSEIS_R {
        BSEIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 1 millisecond timer Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn status_1ms(&self) -> STATUS_1MS_R {
        STATUS_1MS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data Pulse Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn dpis(&self) -> DPIS_R {
        DPIS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
    #[inline(always)]
    pub fn idie(&self) -> IDIE_R {
        IDIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
    #[inline(always)]
    pub fn avvie(&self) -> AVVIE_R {
        AVVIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - A Session Valid Interrupt Enable - Read/Write"]
    #[inline(always)]
    pub fn asvie(&self) -> ASVIE_R {
        ASVIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B Session Valid Interrupt Enable - Read/Write"]
    #[inline(always)]
    pub fn bsvie(&self) -> BSVIE_R {
        BSVIE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B Session End Interrupt Enable - Read/Write. Setting this bit enables the B session end interrupt."]
    #[inline(always)]
    pub fn bseie(&self) -> BSEIE_R {
        BSEIE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 1 millisecond timer Interrupt Enable - Read/Write"]
    #[inline(always)]
    pub fn en_1ms(&self) -> EN_1MS_R {
        EN_1MS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Data Pulse Interrupt Enable"]
    #[inline(always)]
    pub fn dpie(&self) -> DPIE_R {
        DPIE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
    #[inline(always)]
    pub fn vd(&mut self) -> VD_W {
        VD_W { w: self }
    }
    #[doc = "Bit 1 - VBUS Charge - Read/Write"]
    #[inline(always)]
    pub fn vc(&mut self) -> VC_W {
        VC_W { w: self }
    }
    #[doc = "Bit 3 - OTG Termination - Read/Write"]
    #[inline(always)]
    pub fn ot(&mut self) -> OT_W {
        OT_W { w: self }
    }
    #[doc = "Bit 4 - Data Pulsing - Read/Write"]
    #[inline(always)]
    pub fn dp(&mut self) -> DP_W {
        DP_W { w: self }
    }
    #[doc = "Bit 5 - ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]"]
    #[inline(always)]
    pub fn idpu(&mut self) -> IDPU_W {
        IDPU_W { w: self }
    }
    #[doc = "Bit 16 - USB ID Interrupt Status - Read/Write"]
    #[inline(always)]
    pub fn idis(&mut self) -> IDIS_W {
        IDIS_W { w: self }
    }
    #[doc = "Bit 17 - A VBus Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn avvis(&mut self) -> AVVIS_W {
        AVVIS_W { w: self }
    }
    #[doc = "Bit 18 - A Session Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn asvis(&mut self) -> ASVIS_W {
        ASVIS_W { w: self }
    }
    #[doc = "Bit 19 - B Session Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn bsvis(&mut self) -> BSVIS_W {
        BSVIS_W { w: self }
    }
    #[doc = "Bit 20 - B Session End Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn bseis(&mut self) -> BSEIS_W {
        BSEIS_W { w: self }
    }
    #[doc = "Bit 21 - 1 millisecond timer Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn status_1ms(&mut self) -> STATUS_1MS_W {
        STATUS_1MS_W { w: self }
    }
    #[doc = "Bit 22 - Data Pulse Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn dpis(&mut self) -> DPIS_W {
        DPIS_W { w: self }
    }
    #[doc = "Bit 24 - USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
    #[inline(always)]
    pub fn idie(&mut self) -> IDIE_W {
        IDIE_W { w: self }
    }
    #[doc = "Bit 25 - A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
    #[inline(always)]
    pub fn avvie(&mut self) -> AVVIE_W {
        AVVIE_W { w: self }
    }
    #[doc = "Bit 26 - A Session Valid Interrupt Enable - Read/Write"]
    #[inline(always)]
    pub fn asvie(&mut self) -> ASVIE_W {
        ASVIE_W { w: self }
    }
    #[doc = "Bit 27 - B Session Valid Interrupt Enable - Read/Write"]
    #[inline(always)]
    pub fn bsvie(&mut self) -> BSVIE_W {
        BSVIE_W { w: self }
    }
    #[doc = "Bit 28 - B Session End Interrupt Enable - Read/Write. Setting this bit enables the B session end interrupt."]
    #[inline(always)]
    pub fn bseie(&mut self) -> BSEIE_W {
        BSEIE_W { w: self }
    }
    #[doc = "Bit 29 - 1 millisecond timer Interrupt Enable - Read/Write"]
    #[inline(always)]
    pub fn en_1ms(&mut self) -> EN_1MS_W {
        EN_1MS_W { w: self }
    }
    #[doc = "Bit 30 - Data Pulse Interrupt Enable"]
    #[inline(always)]
    pub fn dpie(&mut self) -> DPIE_W {
        DPIE_W { w: self }
    }
}
