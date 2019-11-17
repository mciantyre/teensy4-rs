#[doc = "Reader of register CCSR"]
pub type R = crate::R<u32, super::CCSR>;
#[doc = "Writer for register CCSR"]
pub type W = crate::W<u32, super::CCSR>;
#[doc = "Register CCSR `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::CCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL3_SW_CLK_SEL_A {
    #[doc = "0: pll3_main_clk"]
    PLL3_SW_CLK_SEL_0,
    #[doc = "1: pll3 bypass clock"]
    PLL3_SW_CLK_SEL_1,
}
impl From<PLL3_SW_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLL3_SW_CLK_SEL_A) -> Self {
        match variant {
            PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_0 => false,
            PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `PLL3_SW_CLK_SEL`"]
pub type PLL3_SW_CLK_SEL_R = crate::R<bool, PLL3_SW_CLK_SEL_A>;
impl PLL3_SW_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL3_SW_CLK_SEL_A {
        match self.bits {
            false => PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_0,
            true => PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLL3_SW_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_pll3_sw_clk_sel_0(&self) -> bool {
        *self == PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PLL3_SW_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_pll3_sw_clk_sel_1(&self) -> bool {
        *self == PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_1
    }
}
#[doc = "Write proxy for field `PLL3_SW_CLK_SEL`"]
pub struct PLL3_SW_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3_SW_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL3_SW_CLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "pll3_main_clk"]
    #[inline(always)]
    pub fn pll3_sw_clk_sel_0(self) -> &'a mut W {
        self.variant(PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_0)
    }
    #[doc = "pll3 bypass clock"]
    #[inline(always)]
    pub fn pll3_sw_clk_sel_1(self) -> &'a mut W {
        self.variant(PLL3_SW_CLK_SEL_A::PLL3_SW_CLK_SEL_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
    #[inline(always)]
    pub fn pll3_sw_clk_sel(&self) -> PLL3_SW_CLK_SEL_R {
        PLL3_SW_CLK_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects source to generate pll3_sw_clk. This bit should only be used for testing purposes."]
    #[inline(always)]
    pub fn pll3_sw_clk_sel(&mut self) -> PLL3_SW_CLK_SEL_W {
        PLL3_SW_CLK_SEL_W { w: self }
    }
}
