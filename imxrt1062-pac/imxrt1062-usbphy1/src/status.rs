#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u8, u8>;
#[doc = "Reader of field `HOSTDISCONDETECT_STATUS`"]
pub type HOSTDISCONDETECT_STATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u8, u8>;
#[doc = "Reader of field `DEVPLUGIN_STATUS`"]
pub type DEVPLUGIN_STATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSVD2`"]
pub type RSVD2_R = crate::R<bool, bool>;
#[doc = "Reader of field `OTGID_STATUS`"]
pub type OTGID_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGID_STATUS`"]
pub struct OTGID_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGID_STATUS_W<'a> {
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
#[doc = "Reader of field `RSVD3`"]
pub type RSVD3_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESUME_STATUS`"]
pub type RESUME_STATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSVD4`"]
pub type RSVD4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - Reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected while in high-speed host mode."]
    #[inline(always)]
    pub fn hostdiscondetect_status(&self) -> HOSTDISCONDETECT_STATUS_R {
        HOSTDISCONDETECT_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Indicates that the device has been connected on the USB_DP and USB_DM lines."]
    #[inline(always)]
    pub fn devplugin_status(&self) -> DEVPLUGIN_STATUS_R {
        DEVPLUGIN_STATUS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Indicates the results of ID pin on MiniAB plug"]
    #[inline(always)]
    pub fn otgid_status(&self) -> OTGID_STATUS_R {
        OTGID_STATUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reserved."]
    #[inline(always)]
    pub fn rsvd3(&self) -> RSVD3_R {
        RSVD3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after suspend and has triggered an interrupt."]
    #[inline(always)]
    pub fn resume_status(&self) -> RESUME_STATUS_R {
        RESUME_STATUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:31 - Reserved."]
    #[inline(always)]
    pub fn rsvd4(&self) -> RSVD4_R {
        RSVD4_R::new(((self.bits >> 11) & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 8 - Indicates the results of ID pin on MiniAB plug"]
    #[inline(always)]
    pub fn otgid_status(&mut self) -> OTGID_STATUS_W {
        OTGID_STATUS_W { w: self }
    }
}
