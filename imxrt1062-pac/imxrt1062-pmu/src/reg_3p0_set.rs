#[doc = "Reader of register REG_3P0_SET"]
pub type R = crate::R<u32, super::REG_3P0_SET>;
#[doc = "Writer for register REG_3P0_SET"]
pub type W = crate::W<u32, super::REG_3P0_SET>;
#[doc = "Register REG_3P0_SET `reset()`'s with value 0x0f74"]
impl crate::ResetValue for super::REG_3P0_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f74
    }
}
#[doc = "Reader of field `ENABLE_LINREG`"]
pub type ENABLE_LINREG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_LINREG`"]
pub struct ENABLE_LINREG_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_LINREG_W<'a> {
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
#[doc = "Reader of field `ENABLE_BO`"]
pub type ENABLE_BO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_BO`"]
pub struct ENABLE_BO_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_BO_W<'a> {
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
#[doc = "Reader of field `ENABLE_ILIMIT`"]
pub type ENABLE_ILIMIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_ILIMIT`"]
pub struct ENABLE_ILIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_ILIMIT_W<'a> {
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
#[doc = "Reader of field `BO_OFFSET`"]
pub type BO_OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BO_OFFSET`"]
pub struct BO_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> BO_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUS_SEL_A {
    #[doc = "0: Utilize VBUS OTG2 power"]
    USB_OTG2_VBUS = 0,
    #[doc = "1: Utilize VBUS OTG1 power"]
    USB_OTG1_VBUS = 1,
}
impl From<VBUS_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VBUS_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VBUS_SEL`"]
pub type VBUS_SEL_R = crate::R<bool, VBUS_SEL_A>;
impl VBUS_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUS_SEL_A {
        match self.bits {
            false => VBUS_SEL_A::USB_OTG2_VBUS,
            true => VBUS_SEL_A::USB_OTG1_VBUS,
        }
    }
    #[doc = "Checks if the value of the field is `USB_OTG2_VBUS`"]
    #[inline(always)]
    pub fn is_usb_otg2_vbus(&self) -> bool {
        *self == VBUS_SEL_A::USB_OTG2_VBUS
    }
    #[doc = "Checks if the value of the field is `USB_OTG1_VBUS`"]
    #[inline(always)]
    pub fn is_usb_otg1_vbus(&self) -> bool {
        *self == VBUS_SEL_A::USB_OTG1_VBUS
    }
}
#[doc = "Write proxy for field `VBUS_SEL`"]
pub struct VBUS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBUS_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Utilize VBUS OTG2 power"]
    #[inline(always)]
    pub fn usb_otg2_vbus(self) -> &'a mut W {
        self.variant(VBUS_SEL_A::USB_OTG2_VBUS)
    }
    #[doc = "Utilize VBUS OTG1 power"]
    #[inline(always)]
    pub fn usb_otg1_vbus(self) -> &'a mut W {
        self.variant(VBUS_SEL_A::USB_OTG1_VBUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Control bits to adjust the regulator output voltage\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTPUT_TRG_A {
    #[doc = "0: 2.625V"]
    OUTPUT_TRG_0 = 0,
    #[doc = "15: 3.000V"]
    OUTPUT_TRG_15 = 15,
    #[doc = "31: 3.400V"]
    OUTPUT_TRG_31 = 31,
}
impl From<OUTPUT_TRG_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTPUT_TRG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUTPUT_TRG`"]
pub type OUTPUT_TRG_R = crate::R<u8, OUTPUT_TRG_A>;
impl OUTPUT_TRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OUTPUT_TRG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OUTPUT_TRG_A::OUTPUT_TRG_0),
            15 => Val(OUTPUT_TRG_A::OUTPUT_TRG_15),
            31 => Val(OUTPUT_TRG_A::OUTPUT_TRG_31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_0`"]
    #[inline(always)]
    pub fn is_output_trg_0(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_0
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_15`"]
    #[inline(always)]
    pub fn is_output_trg_15(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_15
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_31`"]
    #[inline(always)]
    pub fn is_output_trg_31(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_31
    }
}
#[doc = "Write proxy for field `OUTPUT_TRG`"]
pub struct OUTPUT_TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_TRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTPUT_TRG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "2.625V"]
    #[inline(always)]
    pub fn output_trg_0(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_0)
    }
    #[doc = "3.000V"]
    #[inline(always)]
    pub fn output_trg_15(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_15)
    }
    #[doc = "3.400V"]
    #[inline(always)]
    pub fn output_trg_31(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `BO_VDD3P0`"]
pub type BO_VDD3P0_R = crate::R<bool, bool>;
#[doc = "Reader of field `OK_VDD3P0`"]
pub type OK_VDD3P0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub fn enable_linreg(&self) -> ENABLE_LINREG_R {
        ENABLE_LINREG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub fn enable_bo(&self) -> ENABLE_BO_R {
        ENABLE_BO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub fn enable_ilimit(&self) -> ENABLE_ILIMIT_R {
        ENABLE_ILIMIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub fn bo_offset(&self) -> BO_OFFSET_R {
        BO_OFFSET_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub fn vbus_sel(&self) -> VBUS_SEL_R {
        VBUS_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub fn output_trg(&self) -> OUTPUT_TRG_R {
        OUTPUT_TRG_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub fn bo_vdd3p0(&self) -> BO_VDD3P0_R {
        BO_VDD3P0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub fn ok_vdd3p0(&self) -> OK_VDD3P0_R {
        OK_VDD3P0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control bit to enable the regulator output to be set by the programmed target voltage setting and internal bandgap reference"]
    #[inline(always)]
    pub fn enable_linreg(&mut self) -> ENABLE_LINREG_W {
        ENABLE_LINREG_W { w: self }
    }
    #[doc = "Bit 1 - Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub fn enable_bo(&mut self) -> ENABLE_BO_W {
        ENABLE_BO_W { w: self }
    }
    #[doc = "Bit 2 - Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub fn enable_ilimit(&mut self) -> ENABLE_ILIMIT_W {
        ENABLE_ILIMIT_W { w: self }
    }
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub fn bo_offset(&mut self) -> BO_OFFSET_W {
        BO_OFFSET_W { w: self }
    }
    #[doc = "Bit 7 - Select input voltage source for LDO_3P0 from either USB_OTG1_VBUS or USB_OTG2_VBUS"]
    #[inline(always)]
    pub fn vbus_sel(&mut self) -> VBUS_SEL_W {
        VBUS_SEL_W { w: self }
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub fn output_trg(&mut self) -> OUTPUT_TRG_W {
        OUTPUT_TRG_W { w: self }
    }
}
