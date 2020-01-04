#[doc = "Reader of register CTRL_SET"]
pub type R = crate::R<u32, super::CTRL_SET>;
#[doc = "Writer for register CTRL_SET"]
pub type W = crate::W<u32, super::CTRL_SET>;
#[doc = "Register CTRL_SET `reset()`'s with value 0xc020_0000"]
impl crate::ResetValue for super::CTRL_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc020_0000
    }
}
#[doc = "Reader of field `ENOTG_ID_CHG_IRQ`"]
pub type ENOTG_ID_CHG_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENOTG_ID_CHG_IRQ`"]
pub struct ENOTG_ID_CHG_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ENOTG_ID_CHG_IRQ_W<'a> {
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
#[doc = "Reader of field `ENHOSTDISCONDETECT`"]
pub type ENHOSTDISCONDETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENHOSTDISCONDETECT`"]
pub struct ENHOSTDISCONDETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENHOSTDISCONDETECT_W<'a> {
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
#[doc = "Reader of field `ENIRQHOSTDISCON`"]
pub type ENIRQHOSTDISCON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENIRQHOSTDISCON`"]
pub struct ENIRQHOSTDISCON_W<'a> {
    w: &'a mut W,
}
impl<'a> ENIRQHOSTDISCON_W<'a> {
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
#[doc = "Reader of field `HOSTDISCONDETECT_IRQ`"]
pub type HOSTDISCONDETECT_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOSTDISCONDETECT_IRQ`"]
pub struct HOSTDISCONDETECT_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTDISCONDETECT_IRQ_W<'a> {
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
#[doc = "Reader of field `ENDEVPLUGINDETECT`"]
pub type ENDEVPLUGINDETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENDEVPLUGINDETECT`"]
pub struct ENDEVPLUGINDETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEVPLUGINDETECT_W<'a> {
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
#[doc = "Reader of field `DEVPLUGIN_POLARITY`"]
pub type DEVPLUGIN_POLARITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEVPLUGIN_POLARITY`"]
pub struct DEVPLUGIN_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVPLUGIN_POLARITY_W<'a> {
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
#[doc = "Reader of field `OTG_ID_CHG_IRQ`"]
pub type OTG_ID_CHG_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTG_ID_CHG_IRQ`"]
pub struct OTG_ID_CHG_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> OTG_ID_CHG_IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ENOTGIDDETECT`"]
pub type ENOTGIDDETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENOTGIDDETECT`"]
pub struct ENOTGIDDETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENOTGIDDETECT_W<'a> {
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
#[doc = "Reader of field `RESUMEIRQSTICKY`"]
pub type RESUMEIRQSTICKY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUMEIRQSTICKY`"]
pub struct RESUMEIRQSTICKY_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEIRQSTICKY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ENIRQRESUMEDETECT`"]
pub type ENIRQRESUMEDETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENIRQRESUMEDETECT`"]
pub struct ENIRQRESUMEDETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENIRQRESUMEDETECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `RESUME_IRQ`"]
pub type RESUME_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESUME_IRQ`"]
pub struct RESUME_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ENIRQDEVPLUGIN`"]
pub type ENIRQDEVPLUGIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENIRQDEVPLUGIN`"]
pub struct ENIRQDEVPLUGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENIRQDEVPLUGIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DEVPLUGIN_IRQ`"]
pub type DEVPLUGIN_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEVPLUGIN_IRQ`"]
pub struct DEVPLUGIN_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVPLUGIN_IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DATA_ON_LRADC`"]
pub type DATA_ON_LRADC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_ON_LRADC`"]
pub struct DATA_ON_LRADC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_ON_LRADC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ENUTMILEVEL2`"]
pub type ENUTMILEVEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENUTMILEVEL2`"]
pub struct ENUTMILEVEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUTMILEVEL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ENUTMILEVEL3`"]
pub type ENUTMILEVEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENUTMILEVEL3`"]
pub struct ENUTMILEVEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUTMILEVEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `ENIRQWAKEUP`"]
pub type ENIRQWAKEUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENIRQWAKEUP`"]
pub struct ENIRQWAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENIRQWAKEUP_W<'a> {
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
#[doc = "Reader of field `WAKEUP_IRQ`"]
pub type WAKEUP_IRQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEUP_IRQ`"]
pub struct WAKEUP_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_IRQ_W<'a> {
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
#[doc = "Reader of field `ENAUTO_PWRON_PLL`"]
pub type ENAUTO_PWRON_PLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENAUTO_PWRON_PLL`"]
pub struct ENAUTO_PWRON_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAUTO_PWRON_PLL_W<'a> {
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
#[doc = "Reader of field `ENAUTOCLR_CLKGATE`"]
pub type ENAUTOCLR_CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENAUTOCLR_CLKGATE`"]
pub struct ENAUTOCLR_CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAUTOCLR_CLKGATE_W<'a> {
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
#[doc = "Reader of field `ENAUTOCLR_PHY_PWD`"]
pub type ENAUTOCLR_PHY_PWD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENAUTOCLR_PHY_PWD`"]
pub struct ENAUTOCLR_PHY_PWD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAUTOCLR_PHY_PWD_W<'a> {
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
#[doc = "Reader of field `ENDPDMCHG_WKUP`"]
pub type ENDPDMCHG_WKUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENDPDMCHG_WKUP`"]
pub struct ENDPDMCHG_WKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDPDMCHG_WKUP_W<'a> {
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
#[doc = "Reader of field `ENIDCHG_WKUP`"]
pub type ENIDCHG_WKUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENIDCHG_WKUP`"]
pub struct ENIDCHG_WKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENIDCHG_WKUP_W<'a> {
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
#[doc = "Reader of field `ENVBUSCHG_WKUP`"]
pub type ENVBUSCHG_WKUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENVBUSCHG_WKUP`"]
pub struct ENVBUSCHG_WKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVBUSCHG_WKUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `FSDLL_RST_EN`"]
pub type FSDLL_RST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSDLL_RST_EN`"]
pub struct FSDLL_RST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDLL_RST_EN_W<'a> {
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
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u8, u8>;
#[doc = "Reader of field `OTG_ID_VALUE`"]
pub type OTG_ID_VALUE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HOST_FORCE_LS_SE0`"]
pub type HOST_FORCE_LS_SE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_FORCE_LS_SE0`"]
pub struct HOST_FORCE_LS_SE0_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_FORCE_LS_SE0_W<'a> {
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
#[doc = "Reader of field `UTMI_SUSPENDM`"]
pub type UTMI_SUSPENDM_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLKGATE`"]
pub type CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKGATE`"]
pub struct CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGATE_W<'a> {
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
#[doc = "Reader of field `SFTRST`"]
pub type SFTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFTRST`"]
pub struct SFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTRST_W<'a> {
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
    #[doc = "Bit 0 - Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub fn enotg_id_chg_irq(&self) -> ENOTG_ID_CHG_IRQ_R {
        ENOTG_ID_CHG_IRQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub fn enhostdiscondetect(&self) -> ENHOSTDISCONDETECT_R {
        ENHOSTDISCONDETECT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub fn enirqhostdiscon(&self) -> ENIRQHOSTDISCON_R {
        ENIRQHOSTDISCON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_irq(&self) -> HOSTDISCONDETECT_IRQ_R {
        HOSTDISCONDETECT_IRQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub fn endevplugindetect(&self) -> ENDEVPLUGINDETECT_R {
        ENDEVPLUGINDETECT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub fn devplugin_polarity(&self) -> DEVPLUGIN_POLARITY_R {
        DEVPLUGIN_POLARITY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub fn otg_id_chg_irq(&self) -> OTG_ID_CHG_IRQ_R {
        OTG_ID_CHG_IRQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub fn enotgiddetect(&self) -> ENOTGIDDETECT_R {
        ENOTGIDDETECT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub fn resumeirqsticky(&self) -> RESUMEIRQSTICKY_R {
        RESUMEIRQSTICKY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub fn enirqresumedetect(&self) -> ENIRQRESUMEDETECT_R {
        ENIRQRESUMEDETECT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub fn resume_irq(&self) -> RESUME_IRQ_R {
        RESUME_IRQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub fn enirqdevplugin(&self) -> ENIRQDEVPLUGIN_R {
        ENIRQDEVPLUGIN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    pub fn devplugin_irq(&self) -> DEVPLUGIN_IRQ_R {
        DEVPLUGIN_IRQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub fn data_on_lradc(&self) -> DATA_ON_LRADC_R {
        DATA_ON_LRADC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub fn enutmilevel2(&self) -> ENUTMILEVEL2_R {
        ENUTMILEVEL2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables UTMI+ Level3"]
    #[inline(always)]
    pub fn enutmilevel3(&self) -> ENUTMILEVEL3_R {
        ENUTMILEVEL3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub fn enirqwakeup(&self) -> ENIRQWAKEUP_R {
        ENIRQWAKEUP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Indicates that there is a wakeup event"]
    #[inline(always)]
    pub fn wakeup_irq(&self) -> WAKEUP_IRQ_R {
        WAKEUP_IRQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub fn enauto_pwron_pll(&self) -> ENAUTO_PWRON_PLL_R {
        ENAUTO_PWRON_PLL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_clkgate(&self) -> ENAUTOCLR_CLKGATE_R {
        ENAUTOCLR_CLKGATE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_phy_pwd(&self) -> ENAUTOCLR_PHY_PWD_R {
        ENAUTOCLR_PHY_PWD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub fn endpdmchg_wkup(&self) -> ENDPDMCHG_WKUP_R {
        ENDPDMCHG_WKUP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub fn enidchg_wkup(&self) -> ENIDCHG_WKUP_R {
        ENIDCHG_WKUP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub fn envbuschg_wkup(&self) -> ENVBUSCHG_WKUP_R {
        ENVBUSCHG_WKUP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub fn fsdll_rst_en(&self) -> FSDLL_RST_EN_R {
        FSDLL_RST_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - Reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 27 - Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub fn otg_id_value(&self) -> OTG_ID_VALUE_R {
        OTG_ID_VALUE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub fn host_force_ls_se0(&self) -> HOST_FORCE_LS_SE0_R {
        HOST_FORCE_LS_SE0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub fn utmi_suspendm(&self) -> UTMI_SUSPENDM_R {
        UTMI_SUSPENDM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub fn enotg_id_chg_irq(&mut self) -> ENOTG_ID_CHG_IRQ_W {
        ENOTG_ID_CHG_IRQ_W { w: self }
    }
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub fn enhostdiscondetect(&mut self) -> ENHOSTDISCONDETECT_W {
        ENHOSTDISCONDETECT_W { w: self }
    }
    #[doc = "Bit 2 - Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub fn enirqhostdiscon(&mut self) -> ENIRQHOSTDISCON_W {
        ENIRQHOSTDISCON_W { w: self }
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_irq(&mut self) -> HOSTDISCONDETECT_IRQ_W {
        HOSTDISCONDETECT_IRQ_W { w: self }
    }
    #[doc = "Bit 4 - For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub fn endevplugindetect(&mut self) -> ENDEVPLUGINDETECT_W {
        ENDEVPLUGINDETECT_W { w: self }
    }
    #[doc = "Bit 5 - For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub fn devplugin_polarity(&mut self) -> DEVPLUGIN_POLARITY_W {
        DEVPLUGIN_POLARITY_W { w: self }
    }
    #[doc = "Bit 6 - OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub fn otg_id_chg_irq(&mut self) -> OTG_ID_CHG_IRQ_W {
        OTG_ID_CHG_IRQ_W { w: self }
    }
    #[doc = "Bit 7 - Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub fn enotgiddetect(&mut self) -> ENOTGIDDETECT_W {
        ENOTGIDDETECT_W { w: self }
    }
    #[doc = "Bit 8 - Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub fn resumeirqsticky(&mut self) -> RESUMEIRQSTICKY_W {
        RESUMEIRQSTICKY_W { w: self }
    }
    #[doc = "Bit 9 - Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub fn enirqresumedetect(&mut self) -> ENIRQRESUMEDETECT_W {
        ENIRQRESUMEDETECT_W { w: self }
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub fn resume_irq(&mut self) -> RESUME_IRQ_W {
        RESUME_IRQ_W { w: self }
    }
    #[doc = "Bit 11 - Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub fn enirqdevplugin(&mut self) -> ENIRQDEVPLUGIN_W {
        ENIRQDEVPLUGIN_W { w: self }
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    pub fn devplugin_irq(&mut self) -> DEVPLUGIN_IRQ_W {
        DEVPLUGIN_IRQ_W { w: self }
    }
    #[doc = "Bit 13 - Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub fn data_on_lradc(&mut self) -> DATA_ON_LRADC_W {
        DATA_ON_LRADC_W { w: self }
    }
    #[doc = "Bit 14 - Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub fn enutmilevel2(&mut self) -> ENUTMILEVEL2_W {
        ENUTMILEVEL2_W { w: self }
    }
    #[doc = "Bit 15 - Enables UTMI+ Level3"]
    #[inline(always)]
    pub fn enutmilevel3(&mut self) -> ENUTMILEVEL3_W {
        ENUTMILEVEL3_W { w: self }
    }
    #[doc = "Bit 16 - Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub fn enirqwakeup(&mut self) -> ENIRQWAKEUP_W {
        ENIRQWAKEUP_W { w: self }
    }
    #[doc = "Bit 17 - Indicates that there is a wakeup event"]
    #[inline(always)]
    pub fn wakeup_irq(&mut self) -> WAKEUP_IRQ_W {
        WAKEUP_IRQ_W { w: self }
    }
    #[doc = "Bit 18 - Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub fn enauto_pwron_pll(&mut self) -> ENAUTO_PWRON_PLL_W {
        ENAUTO_PWRON_PLL_W { w: self }
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_clkgate(&mut self) -> ENAUTOCLR_CLKGATE_W {
        ENAUTOCLR_CLKGATE_W { w: self }
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_phy_pwd(&mut self) -> ENAUTOCLR_PHY_PWD_W {
        ENAUTOCLR_PHY_PWD_W { w: self }
    }
    #[doc = "Bit 21 - Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub fn endpdmchg_wkup(&mut self) -> ENDPDMCHG_WKUP_W {
        ENDPDMCHG_WKUP_W { w: self }
    }
    #[doc = "Bit 22 - Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub fn enidchg_wkup(&mut self) -> ENIDCHG_WKUP_W {
        ENIDCHG_WKUP_W { w: self }
    }
    #[doc = "Bit 23 - Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub fn envbuschg_wkup(&mut self) -> ENVBUSCHG_WKUP_W {
        ENVBUSCHG_WKUP_W { w: self }
    }
    #[doc = "Bit 24 - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub fn fsdll_rst_en(&mut self) -> FSDLL_RST_EN_W {
        FSDLL_RST_EN_W { w: self }
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub fn host_force_ls_se0(&mut self) -> HOST_FORCE_LS_SE0_W {
        HOST_FORCE_LS_SE0_W { w: self }
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    pub fn clkgate(&mut self) -> CLKGATE_W {
        CLKGATE_W { w: self }
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    pub fn sftrst(&mut self) -> SFTRST_W {
        SFTRST_W { w: self }
    }
}
