#[doc = "Reader of register USB1_VBUS_DETECT_SET"]
pub type R = crate::R<u32, super::USB1_VBUS_DETECT_SET>;
#[doc = "Writer for register USB1_VBUS_DETECT_SET"]
pub type W = crate::W<u32, super::USB1_VBUS_DETECT_SET>;
#[doc = "Register USB1_VBUS_DETECT_SET `reset()`'s with value 0x0010_0004"]
impl crate::ResetValue for super::USB1_VBUS_DETECT_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_0004
    }
}
#[doc = "Set the threshold for the VBUSVALID comparator\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VBUSVALID_THRESH_A {
    #[doc = "0: 4.0V"]
    _4V0 = 0,
    #[doc = "1: 4.1V"]
    _4V1 = 1,
    #[doc = "2: 4.2V"]
    _4V2 = 2,
    #[doc = "3: 4.3V"]
    _4V3 = 3,
    #[doc = "4: 4.4V (default)"]
    _4V4 = 4,
    #[doc = "5: 4.5V"]
    _4V5 = 5,
    #[doc = "6: 4.6V"]
    _4V6 = 6,
    #[doc = "7: 4.7V"]
    _4V7 = 7,
}
impl From<VBUSVALID_THRESH_A> for u8 {
    #[inline(always)]
    fn from(variant: VBUSVALID_THRESH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VBUSVALID_THRESH`"]
pub type VBUSVALID_THRESH_R = crate::R<u8, VBUSVALID_THRESH_A>;
impl VBUSVALID_THRESH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSVALID_THRESH_A {
        match self.bits {
            0 => VBUSVALID_THRESH_A::_4V0,
            1 => VBUSVALID_THRESH_A::_4V1,
            2 => VBUSVALID_THRESH_A::_4V2,
            3 => VBUSVALID_THRESH_A::_4V3,
            4 => VBUSVALID_THRESH_A::_4V4,
            5 => VBUSVALID_THRESH_A::_4V5,
            6 => VBUSVALID_THRESH_A::_4V6,
            7 => VBUSVALID_THRESH_A::_4V7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4V0`"]
    #[inline(always)]
    pub fn is_4v0(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V0
    }
    #[doc = "Checks if the value of the field is `_4V1`"]
    #[inline(always)]
    pub fn is_4v1(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V1
    }
    #[doc = "Checks if the value of the field is `_4V2`"]
    #[inline(always)]
    pub fn is_4v2(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V2
    }
    #[doc = "Checks if the value of the field is `_4V3`"]
    #[inline(always)]
    pub fn is_4v3(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V3
    }
    #[doc = "Checks if the value of the field is `_4V4`"]
    #[inline(always)]
    pub fn is_4v4(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V4
    }
    #[doc = "Checks if the value of the field is `_4V5`"]
    #[inline(always)]
    pub fn is_4v5(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V5
    }
    #[doc = "Checks if the value of the field is `_4V6`"]
    #[inline(always)]
    pub fn is_4v6(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V6
    }
    #[doc = "Checks if the value of the field is `_4V7`"]
    #[inline(always)]
    pub fn is_4v7(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V7
    }
}
#[doc = "Write proxy for field `VBUSVALID_THRESH`"]
pub struct VBUSVALID_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVALID_THRESH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUSVALID_THRESH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4.0V"]
    #[inline(always)]
    pub fn _4v0(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V0)
    }
    #[doc = "4.1V"]
    #[inline(always)]
    pub fn _4v1(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V1)
    }
    #[doc = "4.2V"]
    #[inline(always)]
    pub fn _4v2(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V2)
    }
    #[doc = "4.3V"]
    #[inline(always)]
    pub fn _4v3(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V3)
    }
    #[doc = "4.4V (default)"]
    #[inline(always)]
    pub fn _4v4(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V4)
    }
    #[doc = "4.5V"]
    #[inline(always)]
    pub fn _4v5(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V5)
    }
    #[doc = "4.6V"]
    #[inline(always)]
    pub fn _4v6(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V6)
    }
    #[doc = "4.7V"]
    #[inline(always)]
    pub fn _4v7(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `VBUSVALID_PWRUP_CMPS`"]
pub type VBUSVALID_PWRUP_CMPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSVALID_PWRUP_CMPS`"]
pub struct VBUSVALID_PWRUP_CMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVALID_PWRUP_CMPS_W<'a> {
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
#[doc = "Reader of field `DISCHARGE_VBUS`"]
pub type DISCHARGE_VBUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCHARGE_VBUS`"]
pub struct DISCHARGE_VBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCHARGE_VBUS_W<'a> {
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
#[doc = "Reader of field `CHARGE_VBUS`"]
pub type CHARGE_VBUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHARGE_VBUS`"]
pub struct CHARGE_VBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHARGE_VBUS_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub fn vbusvalid_thresh(&self) -> VBUSVALID_THRESH_R {
        VBUSVALID_THRESH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 20 - Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub fn vbusvalid_pwrup_cmps(&self) -> VBUSVALID_PWRUP_CMPS_R {
        VBUSVALID_PWRUP_CMPS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USB OTG discharge VBUS."]
    #[inline(always)]
    pub fn discharge_vbus(&self) -> DISCHARGE_VBUS_R {
        DISCHARGE_VBUS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB OTG charge VBUS."]
    #[inline(always)]
    pub fn charge_vbus(&self) -> CHARGE_VBUS_R {
        CHARGE_VBUS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub fn vbusvalid_thresh(&mut self) -> VBUSVALID_THRESH_W {
        VBUSVALID_THRESH_W { w: self }
    }
    #[doc = "Bit 20 - Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub fn vbusvalid_pwrup_cmps(&mut self) -> VBUSVALID_PWRUP_CMPS_W {
        VBUSVALID_PWRUP_CMPS_W { w: self }
    }
    #[doc = "Bit 26 - USB OTG discharge VBUS."]
    #[inline(always)]
    pub fn discharge_vbus(&mut self) -> DISCHARGE_VBUS_W {
        DISCHARGE_VBUS_W { w: self }
    }
    #[doc = "Bit 27 - USB OTG charge VBUS."]
    #[inline(always)]
    pub fn charge_vbus(&mut self) -> CHARGE_VBUS_W {
        CHARGE_VBUS_W { w: self }
    }
}
