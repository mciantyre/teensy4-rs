#[doc = "Reader of register USB1_CHRG_DETECT_SET"]
pub type R = crate::R<u32, super::USB1_CHRG_DETECT_SET>;
#[doc = "Writer for register USB1_CHRG_DETECT_SET"]
pub type W = crate::W<u32, super::USB1_CHRG_DETECT_SET>;
#[doc = "Register USB1_CHRG_DETECT_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::USB1_CHRG_DETECT_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Check the contact of USB plug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHK_CONTACT_A {
    #[doc = "0: Do not check the contact of USB plug."]
    NO_CHECK = 0,
    #[doc = "1: Check whether the USB plug has been in contact with each other"]
    CHECK = 1,
}
impl From<CHK_CONTACT_A> for bool {
    #[inline(always)]
    fn from(variant: CHK_CONTACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHK_CONTACT`"]
pub type CHK_CONTACT_R = crate::R<bool, CHK_CONTACT_A>;
impl CHK_CONTACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHK_CONTACT_A {
        match self.bits {
            false => CHK_CONTACT_A::NO_CHECK,
            true => CHK_CONTACT_A::CHECK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHECK`"]
    #[inline(always)]
    pub fn is_no_check(&self) -> bool {
        *self == CHK_CONTACT_A::NO_CHECK
    }
    #[doc = "Checks if the value of the field is `CHECK`"]
    #[inline(always)]
    pub fn is_check(&self) -> bool {
        *self == CHK_CONTACT_A::CHECK
    }
}
#[doc = "Write proxy for field `CHK_CONTACT`"]
pub struct CHK_CONTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHK_CONTACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHK_CONTACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not check the contact of USB plug."]
    #[inline(always)]
    pub fn no_check(self) -> &'a mut W {
        self.variant(CHK_CONTACT_A::NO_CHECK)
    }
    #[doc = "Check whether the USB plug has been in contact with each other"]
    #[inline(always)]
    pub fn check(self) -> &'a mut W {
        self.variant(CHK_CONTACT_A::CHECK)
    }
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
#[doc = "Check the charger connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHK_CHRG_B_A {
    #[doc = "0: Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    CHECK = 0,
    #[doc = "1: Do not check whether a charger is connected to the USB port."]
    NO_CHECK = 1,
}
impl From<CHK_CHRG_B_A> for bool {
    #[inline(always)]
    fn from(variant: CHK_CHRG_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHK_CHRG_B`"]
pub type CHK_CHRG_B_R = crate::R<bool, CHK_CHRG_B_A>;
impl CHK_CHRG_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHK_CHRG_B_A {
        match self.bits {
            false => CHK_CHRG_B_A::CHECK,
            true => CHK_CHRG_B_A::NO_CHECK,
        }
    }
    #[doc = "Checks if the value of the field is `CHECK`"]
    #[inline(always)]
    pub fn is_check(&self) -> bool {
        *self == CHK_CHRG_B_A::CHECK
    }
    #[doc = "Checks if the value of the field is `NO_CHECK`"]
    #[inline(always)]
    pub fn is_no_check(&self) -> bool {
        *self == CHK_CHRG_B_A::NO_CHECK
    }
}
#[doc = "Write proxy for field `CHK_CHRG_B`"]
pub struct CHK_CHRG_B_W<'a> {
    w: &'a mut W,
}
impl<'a> CHK_CHRG_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHK_CHRG_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    #[inline(always)]
    pub fn check(self) -> &'a mut W {
        self.variant(CHK_CHRG_B_A::CHECK)
    }
    #[doc = "Do not check whether a charger is connected to the USB port."]
    #[inline(always)]
    pub fn no_check(self) -> &'a mut W {
        self.variant(CHK_CHRG_B_A::NO_CHECK)
    }
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
#[doc = "Control the charger detector.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_B_A {
    #[doc = "0: Enable the charger detector."]
    ENABLE = 0,
    #[doc = "1: Disable the charger detector."]
    DISABLE = 1,
}
impl From<EN_B_A> for bool {
    #[inline(always)]
    fn from(variant: EN_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN_B`"]
pub type EN_B_R = crate::R<bool, EN_B_A>;
impl EN_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_B_A {
        match self.bits {
            false => EN_B_A::ENABLE,
            true => EN_B_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EN_B_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EN_B_A::DISABLE
    }
}
#[doc = "Write proxy for field `EN_B`"]
pub struct EN_B_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the charger detector."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_B_A::ENABLE)
    }
    #[doc = "Disable the charger detector."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_B_A::DISABLE)
    }
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
impl R {
    #[doc = "Bit 18 - Check the contact of USB plug"]
    #[inline(always)]
    pub fn chk_contact(&self) -> CHK_CONTACT_R {
        CHK_CONTACT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Check the charger connection"]
    #[inline(always)]
    pub fn chk_chrg_b(&self) -> CHK_CHRG_B_R {
        CHK_CHRG_B_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Control the charger detector."]
    #[inline(always)]
    pub fn en_b(&self) -> EN_B_R {
        EN_B_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Check the contact of USB plug"]
    #[inline(always)]
    pub fn chk_contact(&mut self) -> CHK_CONTACT_W {
        CHK_CONTACT_W { w: self }
    }
    #[doc = "Bit 19 - Check the charger connection"]
    #[inline(always)]
    pub fn chk_chrg_b(&mut self) -> CHK_CHRG_B_W {
        CHK_CHRG_B_W { w: self }
    }
    #[doc = "Bit 20 - Control the charger detector."]
    #[inline(always)]
    pub fn en_b(&mut self) -> EN_B_W {
        EN_B_W { w: self }
    }
}
