#[doc = "Reader of register USB2_CHRG_DETECT_STAT"]
pub type R = crate::R<u32, super::USB2_CHRG_DETECT_STAT>;
#[doc = "State of the USB plug contact detector.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLUG_CONTACT_A {
    #[doc = "0: The USB plug has not made contact."]
    NO_CONTACT = 0,
    #[doc = "1: The USB plug has made good contact."]
    GOOD_CONTACT = 1,
}
impl From<PLUG_CONTACT_A> for bool {
    #[inline(always)]
    fn from(variant: PLUG_CONTACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLUG_CONTACT`"]
pub type PLUG_CONTACT_R = crate::R<bool, PLUG_CONTACT_A>;
impl PLUG_CONTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLUG_CONTACT_A {
        match self.bits {
            false => PLUG_CONTACT_A::NO_CONTACT,
            true => PLUG_CONTACT_A::GOOD_CONTACT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CONTACT`"]
    #[inline(always)]
    pub fn is_no_contact(&self) -> bool {
        *self == PLUG_CONTACT_A::NO_CONTACT
    }
    #[doc = "Checks if the value of the field is `GOOD_CONTACT`"]
    #[inline(always)]
    pub fn is_good_contact(&self) -> bool {
        *self == PLUG_CONTACT_A::GOOD_CONTACT
    }
}
#[doc = "State of charger detection. This bit is a read only version of the state of the analog signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHRG_DETECTED_A {
    #[doc = "0: The USB port is not connected to a charger."]
    CHARGER_NOT_PRESENT = 0,
    #[doc = "1: A charger (either a dedicated charger or a host charger) is connected to the USB port."]
    CHARGER_PRESENT = 1,
}
impl From<CHRG_DETECTED_A> for bool {
    #[inline(always)]
    fn from(variant: CHRG_DETECTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHRG_DETECTED`"]
pub type CHRG_DETECTED_R = crate::R<bool, CHRG_DETECTED_A>;
impl CHRG_DETECTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHRG_DETECTED_A {
        match self.bits {
            false => CHRG_DETECTED_A::CHARGER_NOT_PRESENT,
            true => CHRG_DETECTED_A::CHARGER_PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `CHARGER_NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_charger_not_present(&self) -> bool {
        *self == CHRG_DETECTED_A::CHARGER_NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `CHARGER_PRESENT`"]
    #[inline(always)]
    pub fn is_charger_present(&self) -> bool {
        *self == CHRG_DETECTED_A::CHARGER_PRESENT
    }
}
#[doc = "Reader of field `DM_STATE`"]
pub type DM_STATE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DP_STATE`"]
pub type DP_STATE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - State of the USB plug contact detector."]
    #[inline(always)]
    pub fn plug_contact(&self) -> PLUG_CONTACT_R {
        PLUG_CONTACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - State of charger detection. This bit is a read only version of the state of the analog signal."]
    #[inline(always)]
    pub fn chrg_detected(&self) -> CHRG_DETECTED_R {
        CHRG_DETECTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DM line state output of the charger detector."]
    #[inline(always)]
    pub fn dm_state(&self) -> DM_STATE_R {
        DM_STATE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DP line state output of the charger detector."]
    #[inline(always)]
    pub fn dp_state(&self) -> DP_STATE_R {
        DP_STATE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
