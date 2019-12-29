#[doc = "Reader of register PLL_USB1_SET"]
pub type R = crate::R<u32, super::PLL_USB1_SET>;
#[doc = "Writer for register PLL_USB1_SET"]
pub type W = crate::W<u32, super::PLL_USB1_SET>;
#[doc = "Register PLL_USB1_SET `reset()`'s with value 0x0001_2000"]
impl crate::ResetValue for super::PLL_USB1_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_2000
    }
}
#[doc = "Reader of field `DIV_SELECT`"]
pub type DIV_SELECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIV_SELECT`"]
pub struct DIV_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_SELECT_W<'a> {
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
#[doc = "Powers the 9-phase PLL outputs for USBPHYn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_USB_CLKS_A {
    #[doc = "0: PLL outputs for USBPHYn off."]
    EN_USB_CLKS_0,
    #[doc = "1: PLL outputs for USBPHYn on."]
    EN_USB_CLKS_1,
}
impl From<EN_USB_CLKS_A> for bool {
    #[inline(always)]
    fn from(variant: EN_USB_CLKS_A) -> Self {
        match variant {
            EN_USB_CLKS_A::EN_USB_CLKS_0 => false,
            EN_USB_CLKS_A::EN_USB_CLKS_1 => true,
        }
    }
}
#[doc = "Reader of field `EN_USB_CLKS`"]
pub type EN_USB_CLKS_R = crate::R<bool, EN_USB_CLKS_A>;
impl EN_USB_CLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_USB_CLKS_A {
        match self.bits {
            false => EN_USB_CLKS_A::EN_USB_CLKS_0,
            true => EN_USB_CLKS_A::EN_USB_CLKS_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_USB_CLKS_0`"]
    #[inline(always)]
    pub fn is_en_usb_clks_0(&self) -> bool {
        *self == EN_USB_CLKS_A::EN_USB_CLKS_0
    }
    #[doc = "Checks if the value of the field is `EN_USB_CLKS_1`"]
    #[inline(always)]
    pub fn is_en_usb_clks_1(&self) -> bool {
        *self == EN_USB_CLKS_A::EN_USB_CLKS_1
    }
}
#[doc = "Write proxy for field `EN_USB_CLKS`"]
pub struct EN_USB_CLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_USB_CLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_USB_CLKS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL outputs for USBPHYn off."]
    #[inline(always)]
    pub fn en_usb_clks_0(self) -> &'a mut W {
        self.variant(EN_USB_CLKS_A::EN_USB_CLKS_0)
    }
    #[doc = "PLL outputs for USBPHYn on."]
    #[inline(always)]
    pub fn en_usb_clks_1(self) -> &'a mut W {
        self.variant(EN_USB_CLKS_A::EN_USB_CLKS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `POWER`"]
pub type POWER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POWER`"]
pub struct POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_W<'a> {
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
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Determines the bypass source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_CLK_SRC_A {
    #[doc = "0: Select the 24MHz oscillator as source."]
    REF_CLK_24M,
    #[doc = "1: Select the CLK1_N / CLK1_P as source."]
    CLK1,
}
impl From<BYPASS_CLK_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: BYPASS_CLK_SRC_A) -> Self {
        match variant {
            BYPASS_CLK_SRC_A::REF_CLK_24M => 0,
            BYPASS_CLK_SRC_A::CLK1 => 1,
        }
    }
}
#[doc = "Reader of field `BYPASS_CLK_SRC`"]
pub type BYPASS_CLK_SRC_R = crate::R<u8, BYPASS_CLK_SRC_A>;
impl BYPASS_CLK_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BYPASS_CLK_SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BYPASS_CLK_SRC_A::REF_CLK_24M),
            1 => Val(BYPASS_CLK_SRC_A::CLK1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REF_CLK_24M`"]
    #[inline(always)]
    pub fn is_ref_clk_24m(&self) -> bool {
        *self == BYPASS_CLK_SRC_A::REF_CLK_24M
    }
    #[doc = "Checks if the value of the field is `CLK1`"]
    #[inline(always)]
    pub fn is_clk1(&self) -> bool {
        *self == BYPASS_CLK_SRC_A::CLK1
    }
}
#[doc = "Write proxy for field `BYPASS_CLK_SRC`"]
pub struct BYPASS_CLK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_CLK_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_CLK_SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select the 24MHz oscillator as source."]
    #[inline(always)]
    pub fn ref_clk_24m(self) -> &'a mut W {
        self.variant(BYPASS_CLK_SRC_A::REF_CLK_24M)
    }
    #[doc = "Select the CLK1_N / CLK1_P as source."]
    #[inline(always)]
    pub fn clk1(self) -> &'a mut W {
        self.variant(BYPASS_CLK_SRC_A::CLK1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub fn div_select(&self) -> DIV_SELECT_R {
        DIV_SELECT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Powers the 9-phase PLL outputs for USBPHYn"]
    #[inline(always)]
    pub fn en_usb_clks(&self) -> EN_USB_CLKS_R {
        EN_USB_CLKS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[inline(always)]
    pub fn power(&self) -> POWER_R {
        POWER_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable the PLL clock output."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Determines the bypass source."]
    #[inline(always)]
    pub fn bypass_clk_src(&self) -> BYPASS_CLK_SRC_R {
        BYPASS_CLK_SRC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Bypass the PLL."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 1 - PLL is currently locked. 0 - PLL is not currently locked."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This field controls the PLL loop divider. 0 - Fout=Fref*20; 1 - Fout=Fref*22."]
    #[inline(always)]
    pub fn div_select(&mut self) -> DIV_SELECT_W {
        DIV_SELECT_W { w: self }
    }
    #[doc = "Bit 6 - Powers the 9-phase PLL outputs for USBPHYn"]
    #[inline(always)]
    pub fn en_usb_clks(&mut self) -> EN_USB_CLKS_W {
        EN_USB_CLKS_W { w: self }
    }
    #[doc = "Bit 12 - Powers up the PLL. This bit will be set automatically when USBPHY0 remote wakeup event happens."]
    #[inline(always)]
    pub fn power(&mut self) -> POWER_W {
        POWER_W { w: self }
    }
    #[doc = "Bit 13 - Enable the PLL clock output."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 14:15 - Determines the bypass source."]
    #[inline(always)]
    pub fn bypass_clk_src(&mut self) -> BYPASS_CLK_SRC_W {
        BYPASS_CLK_SRC_W { w: self }
    }
    #[doc = "Bit 16 - Bypass the PLL."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
}
