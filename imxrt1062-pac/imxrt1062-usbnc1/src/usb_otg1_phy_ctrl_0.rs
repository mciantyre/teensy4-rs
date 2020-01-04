#[doc = "Reader of register USB_OTG1_PHY_CTRL_0"]
pub type R = crate::R<u32, super::USB_OTG1_PHY_CTRL_0>;
#[doc = "Writer for register USB_OTG1_PHY_CTRL_0"]
pub type W = crate::W<u32, super::USB_OTG1_PHY_CTRL_0>;
#[doc = "Register USB_OTG1_PHY_CTRL_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::USB_OTG1_PHY_CTRL_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicating whether OTG1 UTMI PHY clock is valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UTMI_CLK_VLD_A {
    #[doc = "0: Invalid"]
    UTMI_CLK_VLD_0 = 0,
    #[doc = "1: Valid"]
    UTMI_CLK_VLD_1 = 1,
}
impl From<UTMI_CLK_VLD_A> for bool {
    #[inline(always)]
    fn from(variant: UTMI_CLK_VLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UTMI_CLK_VLD`"]
pub type UTMI_CLK_VLD_R = crate::R<bool, UTMI_CLK_VLD_A>;
impl UTMI_CLK_VLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UTMI_CLK_VLD_A {
        match self.bits {
            false => UTMI_CLK_VLD_A::UTMI_CLK_VLD_0,
            true => UTMI_CLK_VLD_A::UTMI_CLK_VLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `UTMI_CLK_VLD_0`"]
    #[inline(always)]
    pub fn is_utmi_clk_vld_0(&self) -> bool {
        *self == UTMI_CLK_VLD_A::UTMI_CLK_VLD_0
    }
    #[doc = "Checks if the value of the field is `UTMI_CLK_VLD_1`"]
    #[inline(always)]
    pub fn is_utmi_clk_vld_1(&self) -> bool {
        *self == UTMI_CLK_VLD_A::UTMI_CLK_VLD_1
    }
}
#[doc = "Write proxy for field `UTMI_CLK_VLD`"]
pub struct UTMI_CLK_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> UTMI_CLK_VLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UTMI_CLK_VLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Invalid"]
    #[inline(always)]
    pub fn utmi_clk_vld_0(self) -> &'a mut W {
        self.variant(UTMI_CLK_VLD_A::UTMI_CLK_VLD_0)
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn utmi_clk_vld_1(self) -> &'a mut W {
        self.variant(UTMI_CLK_VLD_A::UTMI_CLK_VLD_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Indicating whether OTG1 UTMI PHY clock is valid"]
    #[inline(always)]
    pub fn utmi_clk_vld(&self) -> UTMI_CLK_VLD_R {
        UTMI_CLK_VLD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Indicating whether OTG1 UTMI PHY clock is valid"]
    #[inline(always)]
    pub fn utmi_clk_vld(&mut self) -> UTMI_CLK_VLD_W {
        UTMI_CLK_VLD_W { w: self }
    }
}
