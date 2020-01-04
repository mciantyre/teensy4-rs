#[doc = "Reader of register USB_OTG1_CTRL"]
pub type R = crate::R<u32, super::USB_OTG1_CTRL>;
#[doc = "Writer for register USB_OTG1_CTRL"]
pub type W = crate::W<u32, super::USB_OTG1_CTRL>;
#[doc = "Register USB_OTG1_CTRL `reset()`'s with value 0x3000_1000"]
impl crate::ResetValue for super::USB_OTG1_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3000_1000
    }
}
#[doc = "Disable OTG1 Overcurrent Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVER_CUR_DIS_A {
    #[doc = "0: Enables overcurrent detection"]
    OVER_CUR_DIS_0 = 0,
    #[doc = "1: Disables overcurrent detection"]
    OVER_CUR_DIS_1 = 1,
}
impl From<OVER_CUR_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: OVER_CUR_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVER_CUR_DIS`"]
pub type OVER_CUR_DIS_R = crate::R<bool, OVER_CUR_DIS_A>;
impl OVER_CUR_DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVER_CUR_DIS_A {
        match self.bits {
            false => OVER_CUR_DIS_A::OVER_CUR_DIS_0,
            true => OVER_CUR_DIS_A::OVER_CUR_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `OVER_CUR_DIS_0`"]
    #[inline(always)]
    pub fn is_over_cur_dis_0(&self) -> bool {
        *self == OVER_CUR_DIS_A::OVER_CUR_DIS_0
    }
    #[doc = "Checks if the value of the field is `OVER_CUR_DIS_1`"]
    #[inline(always)]
    pub fn is_over_cur_dis_1(&self) -> bool {
        *self == OVER_CUR_DIS_A::OVER_CUR_DIS_1
    }
}
#[doc = "Write proxy for field `OVER_CUR_DIS`"]
pub struct OVER_CUR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVER_CUR_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVER_CUR_DIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables overcurrent detection"]
    #[inline(always)]
    pub fn over_cur_dis_0(self) -> &'a mut W {
        self.variant(OVER_CUR_DIS_A::OVER_CUR_DIS_0)
    }
    #[doc = "Disables overcurrent detection"]
    #[inline(always)]
    pub fn over_cur_dis_1(self) -> &'a mut W {
        self.variant(OVER_CUR_DIS_A::OVER_CUR_DIS_1)
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
#[doc = "OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVER_CUR_POL_A {
    #[doc = "0: High active (high on this signal represents an overcurrent condition)"]
    OVER_CUR_POL_0 = 0,
    #[doc = "1: Low active (low on this signal represents an overcurrent condition)"]
    OVER_CUR_POL_1 = 1,
}
impl From<OVER_CUR_POL_A> for bool {
    #[inline(always)]
    fn from(variant: OVER_CUR_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVER_CUR_POL`"]
pub type OVER_CUR_POL_R = crate::R<bool, OVER_CUR_POL_A>;
impl OVER_CUR_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVER_CUR_POL_A {
        match self.bits {
            false => OVER_CUR_POL_A::OVER_CUR_POL_0,
            true => OVER_CUR_POL_A::OVER_CUR_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `OVER_CUR_POL_0`"]
    #[inline(always)]
    pub fn is_over_cur_pol_0(&self) -> bool {
        *self == OVER_CUR_POL_A::OVER_CUR_POL_0
    }
    #[doc = "Checks if the value of the field is `OVER_CUR_POL_1`"]
    #[inline(always)]
    pub fn is_over_cur_pol_1(&self) -> bool {
        *self == OVER_CUR_POL_A::OVER_CUR_POL_1
    }
}
#[doc = "Write proxy for field `OVER_CUR_POL`"]
pub struct OVER_CUR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVER_CUR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVER_CUR_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "High active (high on this signal represents an overcurrent condition)"]
    #[inline(always)]
    pub fn over_cur_pol_0(self) -> &'a mut W {
        self.variant(OVER_CUR_POL_A::OVER_CUR_POL_0)
    }
    #[doc = "Low active (low on this signal represents an overcurrent condition)"]
    #[inline(always)]
    pub fn over_cur_pol_1(self) -> &'a mut W {
        self.variant(OVER_CUR_POL_A::OVER_CUR_POL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_POL_A {
    #[doc = "0: PMIC Power Pin is Low active."]
    PWR_POL_0 = 0,
    #[doc = "1: PMIC Power Pin is High active."]
    PWR_POL_1 = 1,
}
impl From<PWR_POL_A> for bool {
    #[inline(always)]
    fn from(variant: PWR_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWR_POL`"]
pub type PWR_POL_R = crate::R<bool, PWR_POL_A>;
impl PWR_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWR_POL_A {
        match self.bits {
            false => PWR_POL_A::PWR_POL_0,
            true => PWR_POL_A::PWR_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWR_POL_0`"]
    #[inline(always)]
    pub fn is_pwr_pol_0(&self) -> bool {
        *self == PWR_POL_A::PWR_POL_0
    }
    #[doc = "Checks if the value of the field is `PWR_POL_1`"]
    #[inline(always)]
    pub fn is_pwr_pol_1(&self) -> bool {
        *self == PWR_POL_A::PWR_POL_1
    }
}
#[doc = "Write proxy for field `PWR_POL`"]
pub struct PWR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWR_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PMIC Power Pin is Low active."]
    #[inline(always)]
    pub fn pwr_pol_0(self) -> &'a mut W {
        self.variant(PWR_POL_A::PWR_POL_0)
    }
    #[doc = "PMIC Power Pin is High active."]
    #[inline(always)]
    pub fn pwr_pol_1(self) -> &'a mut W {
        self.variant(PWR_POL_A::PWR_POL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIE_A {
    #[doc = "0: Interrupt Disabled"]
    WIE_0 = 0,
    #[doc = "1: Interrupt Enabled"]
    WIE_1 = 1,
}
impl From<WIE_A> for bool {
    #[inline(always)]
    fn from(variant: WIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WIE`"]
pub type WIE_R = crate::R<bool, WIE_A>;
impl WIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIE_A {
        match self.bits {
            false => WIE_A::WIE_0,
            true => WIE_A::WIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIE_0`"]
    #[inline(always)]
    pub fn is_wie_0(&self) -> bool {
        *self == WIE_A::WIE_0
    }
    #[doc = "Checks if the value of the field is `WIE_1`"]
    #[inline(always)]
    pub fn is_wie_1(&self) -> bool {
        *self == WIE_A::WIE_1
    }
}
#[doc = "Write proxy for field `WIE`"]
pub struct WIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt Disabled"]
    #[inline(always)]
    pub fn wie_0(self) -> &'a mut W {
        self.variant(WIE_A::WIE_0)
    }
    #[doc = "Interrupt Enabled"]
    #[inline(always)]
    pub fn wie_1(self) -> &'a mut W {
        self.variant(WIE_A::WIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "OTG1 Software Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUP_SW_EN_A {
    #[doc = "0: Disable"]
    WKUP_SW_EN_0 = 0,
    #[doc = "1: Enable"]
    WKUP_SW_EN_1 = 1,
}
impl From<WKUP_SW_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_SW_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WKUP_SW_EN`"]
pub type WKUP_SW_EN_R = crate::R<bool, WKUP_SW_EN_A>;
impl WKUP_SW_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUP_SW_EN_A {
        match self.bits {
            false => WKUP_SW_EN_A::WKUP_SW_EN_0,
            true => WKUP_SW_EN_A::WKUP_SW_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_SW_EN_0`"]
    #[inline(always)]
    pub fn is_wkup_sw_en_0(&self) -> bool {
        *self == WKUP_SW_EN_A::WKUP_SW_EN_0
    }
    #[doc = "Checks if the value of the field is `WKUP_SW_EN_1`"]
    #[inline(always)]
    pub fn is_wkup_sw_en_1(&self) -> bool {
        *self == WKUP_SW_EN_A::WKUP_SW_EN_1
    }
}
#[doc = "Write proxy for field `WKUP_SW_EN`"]
pub struct WKUP_SW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_SW_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUP_SW_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn wkup_sw_en_0(self) -> &'a mut W {
        self.variant(WKUP_SW_EN_A::WKUP_SW_EN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn wkup_sw_en_1(self) -> &'a mut W {
        self.variant(WKUP_SW_EN_A::WKUP_SW_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "OTG1 Software Wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUP_SW_A {
    #[doc = "0: Inactive"]
    WKUP_SW_0 = 0,
    #[doc = "1: Force wake-up"]
    WKUP_SW_1 = 1,
}
impl From<WKUP_SW_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WKUP_SW`"]
pub type WKUP_SW_R = crate::R<bool, WKUP_SW_A>;
impl WKUP_SW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUP_SW_A {
        match self.bits {
            false => WKUP_SW_A::WKUP_SW_0,
            true => WKUP_SW_A::WKUP_SW_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_SW_0`"]
    #[inline(always)]
    pub fn is_wkup_sw_0(&self) -> bool {
        *self == WKUP_SW_A::WKUP_SW_0
    }
    #[doc = "Checks if the value of the field is `WKUP_SW_1`"]
    #[inline(always)]
    pub fn is_wkup_sw_1(&self) -> bool {
        *self == WKUP_SW_A::WKUP_SW_1
    }
}
#[doc = "Write proxy for field `WKUP_SW`"]
pub struct WKUP_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUP_SW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn wkup_sw_0(self) -> &'a mut W {
        self.variant(WKUP_SW_A::WKUP_SW_0)
    }
    #[doc = "Force wake-up"]
    #[inline(always)]
    pub fn wkup_sw_1(self) -> &'a mut W {
        self.variant(WKUP_SW_A::WKUP_SW_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "OTG1 Wake-up on ID change enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUP_ID_EN_A {
    #[doc = "0: Disable"]
    WKUP_ID_EN_0 = 0,
    #[doc = "1: Enable"]
    WKUP_ID_EN_1 = 1,
}
impl From<WKUP_ID_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_ID_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WKUP_ID_EN`"]
pub type WKUP_ID_EN_R = crate::R<bool, WKUP_ID_EN_A>;
impl WKUP_ID_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUP_ID_EN_A {
        match self.bits {
            false => WKUP_ID_EN_A::WKUP_ID_EN_0,
            true => WKUP_ID_EN_A::WKUP_ID_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_ID_EN_0`"]
    #[inline(always)]
    pub fn is_wkup_id_en_0(&self) -> bool {
        *self == WKUP_ID_EN_A::WKUP_ID_EN_0
    }
    #[doc = "Checks if the value of the field is `WKUP_ID_EN_1`"]
    #[inline(always)]
    pub fn is_wkup_id_en_1(&self) -> bool {
        *self == WKUP_ID_EN_A::WKUP_ID_EN_1
    }
}
#[doc = "Write proxy for field `WKUP_ID_EN`"]
pub struct WKUP_ID_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_ID_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUP_ID_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn wkup_id_en_0(self) -> &'a mut W {
        self.variant(WKUP_ID_EN_A::WKUP_ID_EN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn wkup_id_en_1(self) -> &'a mut W {
        self.variant(WKUP_ID_EN_A::WKUP_ID_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "OTG1 wake-up on VBUS change enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUP_VBUS_EN_A {
    #[doc = "0: Disable"]
    WKUP_VBUS_EN_0 = 0,
    #[doc = "1: Enable"]
    WKUP_VBUS_EN_1 = 1,
}
impl From<WKUP_VBUS_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_VBUS_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WKUP_VBUS_EN`"]
pub type WKUP_VBUS_EN_R = crate::R<bool, WKUP_VBUS_EN_A>;
impl WKUP_VBUS_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUP_VBUS_EN_A {
        match self.bits {
            false => WKUP_VBUS_EN_A::WKUP_VBUS_EN_0,
            true => WKUP_VBUS_EN_A::WKUP_VBUS_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_VBUS_EN_0`"]
    #[inline(always)]
    pub fn is_wkup_vbus_en_0(&self) -> bool {
        *self == WKUP_VBUS_EN_A::WKUP_VBUS_EN_0
    }
    #[doc = "Checks if the value of the field is `WKUP_VBUS_EN_1`"]
    #[inline(always)]
    pub fn is_wkup_vbus_en_1(&self) -> bool {
        *self == WKUP_VBUS_EN_A::WKUP_VBUS_EN_1
    }
}
#[doc = "Write proxy for field `WKUP_VBUS_EN`"]
pub struct WKUP_VBUS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_VBUS_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUP_VBUS_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn wkup_vbus_en_0(self) -> &'a mut W {
        self.variant(WKUP_VBUS_EN_A::WKUP_VBUS_EN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn wkup_vbus_en_1(self) -> &'a mut W {
        self.variant(WKUP_VBUS_EN_A::WKUP_VBUS_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Wake-up on DPDM change enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUP_DPDM_EN_A {
    #[doc = "0: DPDM changes wake-up to be disabled only when VBUS is 0."]
    WKUP_DPDM_EN_0 = 0,
    #[doc = "1: (Default) DPDM changes wake-up to be enabled, it is for device only."]
    WKUP_DPDM_EN_1 = 1,
}
impl From<WKUP_DPDM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_DPDM_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WKUP_DPDM_EN`"]
pub type WKUP_DPDM_EN_R = crate::R<bool, WKUP_DPDM_EN_A>;
impl WKUP_DPDM_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUP_DPDM_EN_A {
        match self.bits {
            false => WKUP_DPDM_EN_A::WKUP_DPDM_EN_0,
            true => WKUP_DPDM_EN_A::WKUP_DPDM_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKUP_DPDM_EN_0`"]
    #[inline(always)]
    pub fn is_wkup_dpdm_en_0(&self) -> bool {
        *self == WKUP_DPDM_EN_A::WKUP_DPDM_EN_0
    }
    #[doc = "Checks if the value of the field is `WKUP_DPDM_EN_1`"]
    #[inline(always)]
    pub fn is_wkup_dpdm_en_1(&self) -> bool {
        *self == WKUP_DPDM_EN_A::WKUP_DPDM_EN_1
    }
}
#[doc = "Write proxy for field `WKUP_DPDM_EN`"]
pub struct WKUP_DPDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_DPDM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUP_DPDM_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DPDM changes wake-up to be disabled only when VBUS is 0."]
    #[inline(always)]
    pub fn wkup_dpdm_en_0(self) -> &'a mut W {
        self.variant(WKUP_DPDM_EN_A::WKUP_DPDM_EN_0)
    }
    #[doc = "(Default) DPDM changes wake-up to be enabled, it is for device only."]
    #[inline(always)]
    pub fn wkup_dpdm_en_1(self) -> &'a mut W {
        self.variant(WKUP_DPDM_EN_A::WKUP_DPDM_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "OTG1 Wake-up Interrupt Request This bit indicates that a wake-up interrupt request is received on the OTG1 port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIR_A {
    #[doc = "0: No wake-up interrupt request received"]
    WIR_0 = 0,
    #[doc = "1: Wake-up Interrupt Request received"]
    WIR_1 = 1,
}
impl From<WIR_A> for bool {
    #[inline(always)]
    fn from(variant: WIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WIR`"]
pub type WIR_R = crate::R<bool, WIR_A>;
impl WIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIR_A {
        match self.bits {
            false => WIR_A::WIR_0,
            true => WIR_A::WIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIR_0`"]
    #[inline(always)]
    pub fn is_wir_0(&self) -> bool {
        *self == WIR_A::WIR_0
    }
    #[doc = "Checks if the value of the field is `WIR_1`"]
    #[inline(always)]
    pub fn is_wir_1(&self) -> bool {
        *self == WIR_A::WIR_1
    }
}
impl R {
    #[doc = "Bit 7 - Disable OTG1 Overcurrent Detection"]
    #[inline(always)]
    pub fn over_cur_dis(&self) -> OVER_CUR_DIS_R {
        OVER_CUR_DIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event"]
    #[inline(always)]
    pub fn over_cur_pol(&self) -> OVER_CUR_POL_R {
        OVER_CUR_POL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
    #[inline(always)]
    pub fn pwr_pol(&self) -> PWR_POL_R {
        PWR_POL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt"]
    #[inline(always)]
    pub fn wie(&self) -> WIE_R {
        WIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14 - OTG1 Software Wake-up Enable"]
    #[inline(always)]
    pub fn wkup_sw_en(&self) -> WKUP_SW_EN_R {
        WKUP_SW_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - OTG1 Software Wake-up"]
    #[inline(always)]
    pub fn wkup_sw(&self) -> WKUP_SW_R {
        WKUP_SW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - OTG1 Wake-up on ID change enable"]
    #[inline(always)]
    pub fn wkup_id_en(&self) -> WKUP_ID_EN_R {
        WKUP_ID_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OTG1 wake-up on VBUS change enable"]
    #[inline(always)]
    pub fn wkup_vbus_en(&self) -> WKUP_VBUS_EN_R {
        WKUP_VBUS_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Wake-up on DPDM change enable"]
    #[inline(always)]
    pub fn wkup_dpdm_en(&self) -> WKUP_DPDM_EN_R {
        WKUP_DPDM_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - OTG1 Wake-up Interrupt Request This bit indicates that a wake-up interrupt request is received on the OTG1 port"]
    #[inline(always)]
    pub fn wir(&self) -> WIR_R {
        WIR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Disable OTG1 Overcurrent Detection"]
    #[inline(always)]
    pub fn over_cur_dis(&mut self) -> OVER_CUR_DIS_W {
        OVER_CUR_DIS_W { w: self }
    }
    #[doc = "Bit 8 - OTG1 Polarity of Overcurrent The polarity of OTG1 port overcurrent event"]
    #[inline(always)]
    pub fn over_cur_pol(&mut self) -> OVER_CUR_POL_W {
        OVER_CUR_POL_W { w: self }
    }
    #[doc = "Bit 9 - OTG1 Power Polarity This bit should be set according to PMIC Power Pin polarity."]
    #[inline(always)]
    pub fn pwr_pol(&mut self) -> PWR_POL_W {
        PWR_POL_W { w: self }
    }
    #[doc = "Bit 10 - OTG1 Wake-up Interrupt Enable This bit enables or disables the OTG1 wake-up interrupt"]
    #[inline(always)]
    pub fn wie(&mut self) -> WIE_W {
        WIE_W { w: self }
    }
    #[doc = "Bit 14 - OTG1 Software Wake-up Enable"]
    #[inline(always)]
    pub fn wkup_sw_en(&mut self) -> WKUP_SW_EN_W {
        WKUP_SW_EN_W { w: self }
    }
    #[doc = "Bit 15 - OTG1 Software Wake-up"]
    #[inline(always)]
    pub fn wkup_sw(&mut self) -> WKUP_SW_W {
        WKUP_SW_W { w: self }
    }
    #[doc = "Bit 16 - OTG1 Wake-up on ID change enable"]
    #[inline(always)]
    pub fn wkup_id_en(&mut self) -> WKUP_ID_EN_W {
        WKUP_ID_EN_W { w: self }
    }
    #[doc = "Bit 17 - OTG1 wake-up on VBUS change enable"]
    #[inline(always)]
    pub fn wkup_vbus_en(&mut self) -> WKUP_VBUS_EN_W {
        WKUP_VBUS_EN_W { w: self }
    }
    #[doc = "Bit 29 - Wake-up on DPDM change enable"]
    #[inline(always)]
    pub fn wkup_dpdm_en(&mut self) -> WKUP_DPDM_EN_W {
        WKUP_DPDM_EN_W { w: self }
    }
}
