#[doc = "Reader of register CSCDR3"]
pub type R = crate::R<u32, super::CSCDR3>;
#[doc = "Writer for register CSCDR3"]
pub type W = crate::W<u32, super::CSCDR3>;
#[doc = "Register CSCDR3 `reset()`'s with value 0x0003_0841"]
impl crate::ResetValue for super::CSCDR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0003_0841
    }
}
#[doc = "Selector for csi_mclk multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSI_CLK_SEL_A {
    #[doc = "0: derive clock from osc_clk (24M)"]
    CSI_CLK_SEL_0 = 0,
    #[doc = "1: derive clock from PLL2 PFD2"]
    CSI_CLK_SEL_1 = 1,
    #[doc = "2: derive clock from pll3_120M"]
    CSI_CLK_SEL_2 = 2,
    #[doc = "3: derive clock from PLL3 PFD1"]
    CSI_CLK_SEL_3 = 3,
}
impl From<CSI_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CSI_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CSI_CLK_SEL`"]
pub type CSI_CLK_SEL_R = crate::R<u8, CSI_CLK_SEL_A>;
impl CSI_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSI_CLK_SEL_A {
        match self.bits {
            0 => CSI_CLK_SEL_A::CSI_CLK_SEL_0,
            1 => CSI_CLK_SEL_A::CSI_CLK_SEL_1,
            2 => CSI_CLK_SEL_A::CSI_CLK_SEL_2,
            3 => CSI_CLK_SEL_A::CSI_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSI_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_csi_clk_sel_0(&self) -> bool {
        *self == CSI_CLK_SEL_A::CSI_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `CSI_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_csi_clk_sel_1(&self) -> bool {
        *self == CSI_CLK_SEL_A::CSI_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `CSI_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_csi_clk_sel_2(&self) -> bool {
        *self == CSI_CLK_SEL_A::CSI_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `CSI_CLK_SEL_3`"]
    #[inline(always)]
    pub fn is_csi_clk_sel_3(&self) -> bool {
        *self == CSI_CLK_SEL_A::CSI_CLK_SEL_3
    }
}
#[doc = "Write proxy for field `CSI_CLK_SEL`"]
pub struct CSI_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSI_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSI_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "derive clock from osc_clk (24M)"]
    #[inline(always)]
    pub fn csi_clk_sel_0(self) -> &'a mut W {
        self.variant(CSI_CLK_SEL_A::CSI_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline(always)]
    pub fn csi_clk_sel_1(self) -> &'a mut W {
        self.variant(CSI_CLK_SEL_A::CSI_CLK_SEL_1)
    }
    #[doc = "derive clock from pll3_120M"]
    #[inline(always)]
    pub fn csi_clk_sel_2(self) -> &'a mut W {
        self.variant(CSI_CLK_SEL_A::CSI_CLK_SEL_2)
    }
    #[doc = "derive clock from PLL3 PFD1"]
    #[inline(always)]
    pub fn csi_clk_sel_3(self) -> &'a mut W {
        self.variant(CSI_CLK_SEL_A::CSI_CLK_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Post divider for csi_mclk. Divider should be updated when output clock is gated.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSI_PODF_A {
    #[doc = "0: divide by 1"]
    CSI_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    CSI_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    CSI_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    CSI_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    CSI_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    CSI_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    CSI_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    CSI_PODF_7 = 7,
}
impl From<CSI_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: CSI_PODF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CSI_PODF`"]
pub type CSI_PODF_R = crate::R<u8, CSI_PODF_A>;
impl CSI_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSI_PODF_A {
        match self.bits {
            0 => CSI_PODF_A::CSI_PODF_0,
            1 => CSI_PODF_A::CSI_PODF_1,
            2 => CSI_PODF_A::CSI_PODF_2,
            3 => CSI_PODF_A::CSI_PODF_3,
            4 => CSI_PODF_A::CSI_PODF_4,
            5 => CSI_PODF_A::CSI_PODF_5,
            6 => CSI_PODF_A::CSI_PODF_6,
            7 => CSI_PODF_A::CSI_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_0`"]
    #[inline(always)]
    pub fn is_csi_podf_0(&self) -> bool {
        *self == CSI_PODF_A::CSI_PODF_0
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_1`"]
    #[inline(always)]
    pub fn is_csi_podf_1(&self) -> bool {
        *self == CSI_PODF_A::CSI_PODF_1
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_2`"]
    #[inline(always)]
    pub fn is_csi_podf_2(&self) -> bool {
        *self == CSI_PODF_A::CSI_PODF_2
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_3`"]
    #[inline(always)]
    pub fn is_csi_podf_3(&self) -> bool {
        *self == CSI_PODF_A::CSI_PODF_3
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_4`"]
    #[inline(always)]
    pub fn is_csi_podf_4(&self) -> bool {
        *self == CSI_PODF_A::CSI_PODF_4
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_5`"]
    #[inline(always)]
    pub fn is_csi_podf_5(&self) -> bool {
        *self == CSI_PODF_A::CSI_PODF_5
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_6`"]
    #[inline(always)]
    pub fn is_csi_podf_6(&self) -> bool {
        *self == CSI_PODF_A::CSI_PODF_6
    }
    #[doc = "Checks if the value of the field is `CSI_PODF_7`"]
    #[inline(always)]
    pub fn is_csi_podf_7(&self) -> bool {
        *self == CSI_PODF_A::CSI_PODF_7
    }
}
#[doc = "Write proxy for field `CSI_PODF`"]
pub struct CSI_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSI_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSI_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn csi_podf_0(self) -> &'a mut W {
        self.variant(CSI_PODF_A::CSI_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn csi_podf_1(self) -> &'a mut W {
        self.variant(CSI_PODF_A::CSI_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn csi_podf_2(self) -> &'a mut W {
        self.variant(CSI_PODF_A::CSI_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn csi_podf_3(self) -> &'a mut W {
        self.variant(CSI_PODF_A::CSI_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn csi_podf_4(self) -> &'a mut W {
        self.variant(CSI_PODF_A::CSI_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn csi_podf_5(self) -> &'a mut W {
        self.variant(CSI_PODF_A::CSI_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn csi_podf_6(self) -> &'a mut W {
        self.variant(CSI_PODF_A::CSI_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn csi_podf_7(self) -> &'a mut W {
        self.variant(CSI_PODF_A::CSI_PODF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:10 - Selector for csi_mclk multiplexer"]
    #[inline(always)]
    pub fn csi_clk_sel(&self) -> CSI_CLK_SEL_R {
        CSI_CLK_SEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:13 - Post divider for csi_mclk. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn csi_podf(&self) -> CSI_PODF_R {
        CSI_PODF_R::new(((self.bits >> 11) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 9:10 - Selector for csi_mclk multiplexer"]
    #[inline(always)]
    pub fn csi_clk_sel(&mut self) -> CSI_CLK_SEL_W {
        CSI_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Post divider for csi_mclk. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn csi_podf(&mut self) -> CSI_PODF_W {
        CSI_PODF_W { w: self }
    }
}
