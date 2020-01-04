#[doc = "Reader of register USB2_VBUS_DETECT_STAT"]
pub type R = crate::R<u32, super::USB2_VBUS_DETECT_STAT>;
#[doc = "Reader of field `SESSEND`"]
pub type SESSEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `BVALID`"]
pub type BVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `AVALID`"]
pub type AVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBUS_VALID`"]
pub type VBUS_VALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Session End for USB OTG"]
    #[inline(always)]
    pub fn sessend(&self) -> SESSEND_R {
        SESSEND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates VBus is valid for a B-peripheral"]
    #[inline(always)]
    pub fn bvalid(&self) -> BVALID_R {
        BVALID_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates VBus is valid for a A-peripheral"]
    #[inline(always)]
    pub fn avalid(&self) -> AVALID_R {
        AVALID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VBus valid for USB OTG"]
    #[inline(always)]
    pub fn vbus_valid(&self) -> VBUS_VALID_R {
        VBUS_VALID_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
