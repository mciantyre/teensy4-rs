#[doc = "Reader of register CBCMR"]
pub type R = crate::R<u32, super::CBCMR>;
#[doc = "Writer for register CBCMR"]
pub type W = crate::W<u32, super::CBCMR>;
#[doc = "Register CBCMR `reset()`'s with value 0x2dae_8324"]
impl crate::ResetValue for super::CBCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2dae_8324
    }
}
#[doc = "Selector for lpspi clock multiplexer\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPSPI_CLK_SEL_A {
    #[doc = "0: derive clock from PLL3 PFD1 clk"]
    LPSPI_CLK_SEL_0 = 0,
    #[doc = "1: derive clock from PLL3 PFD0"]
    LPSPI_CLK_SEL_1 = 1,
    #[doc = "2: derive clock from PLL2"]
    LPSPI_CLK_SEL_2 = 2,
    #[doc = "3: derive clock from PLL2 PFD2"]
    LPSPI_CLK_SEL_3 = 3,
}
impl From<LPSPI_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPSPI_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPSPI_CLK_SEL`"]
pub type LPSPI_CLK_SEL_R = crate::R<u8, LPSPI_CLK_SEL_A>;
impl LPSPI_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI_CLK_SEL_A {
        match self.bits {
            0 => LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_0,
            1 => LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_1,
            2 => LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_2,
            3 => LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_lpspi_clk_sel_0(&self) -> bool {
        *self == LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `LPSPI_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_lpspi_clk_sel_1(&self) -> bool {
        *self == LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `LPSPI_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_lpspi_clk_sel_2(&self) -> bool {
        *self == LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `LPSPI_CLK_SEL_3`"]
    #[inline(always)]
    pub fn is_lpspi_clk_sel_3(&self) -> bool {
        *self == LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_3
    }
}
#[doc = "Write proxy for field `LPSPI_CLK_SEL`"]
pub struct LPSPI_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "derive clock from PLL3 PFD1 clk"]
    #[inline(always)]
    pub fn lpspi_clk_sel_0(self) -> &'a mut W {
        self.variant(LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL3 PFD0"]
    #[inline(always)]
    pub fn lpspi_clk_sel_1(self) -> &'a mut W {
        self.variant(LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL2"]
    #[inline(always)]
    pub fn lpspi_clk_sel_2(self) -> &'a mut W {
        self.variant(LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_2)
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline(always)]
    pub fn lpspi_clk_sel_3(self) -> &'a mut W {
        self.variant(LPSPI_CLK_SEL_A::LPSPI_CLK_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Selector for flexspi2 clock multiplexer\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXSPI2_CLK_SEL_A {
    #[doc = "0: derive clock from PLL2 PFD2"]
    FLEXSPI2_CLK_SEL_0 = 0,
    #[doc = "1: derive clock from PLL3 PFD0"]
    FLEXSPI2_CLK_SEL_1 = 1,
    #[doc = "2: derive clock from PLL3 PFD1"]
    FLEXSPI2_CLK_SEL_2 = 2,
    #[doc = "3: derive clock from PLL2 (pll2_main_clk)"]
    FLEXSPI2_CLK_SEL_3 = 3,
}
impl From<FLEXSPI2_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXSPI2_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLEXSPI2_CLK_SEL`"]
pub type FLEXSPI2_CLK_SEL_R = crate::R<u8, FLEXSPI2_CLK_SEL_A>;
impl FLEXSPI2_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI2_CLK_SEL_A {
        match self.bits {
            0 => FLEXSPI2_CLK_SEL_A::FLEXSPI2_CLK_SEL_0,
            1 => FLEXSPI2_CLK_SEL_A::FLEXSPI2_CLK_SEL_1,
            2 => FLEXSPI2_CLK_SEL_A::FLEXSPI2_CLK_SEL_2,
            3 => FLEXSPI2_CLK_SEL_A::FLEXSPI2_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_flexspi2_clk_sel_0(&self) -> bool {
        *self == FLEXSPI2_CLK_SEL_A::FLEXSPI2_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_flexspi2_clk_sel_1(&self) -> bool {
        *self == FLEXSPI2_CLK_SEL_A::FLEXSPI2_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_flexspi2_clk_sel_2(&self) -> bool {
        *self == FLEXSPI2_CLK_SEL_A::FLEXSPI2_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_CLK_SEL_3`"]
    #[inline(always)]
    pub fn is_flexspi2_clk_sel_3(&self) -> bool {
        *self == FLEXSPI2_CLK_SEL_A::FLEXSPI2_CLK_SEL_3
    }
}
#[doc = "Write proxy for field `FLEXSPI2_CLK_SEL`"]
pub struct FLEXSPI2_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXSPI2_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXSPI2_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline(always)]
    pub fn flexspi2_clk_sel_0(self) -> &'a mut W {
        self.variant(FLEXSPI2_CLK_SEL_A::FLEXSPI2_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL3 PFD0"]
    #[inline(always)]
    pub fn flexspi2_clk_sel_1(self) -> &'a mut W {
        self.variant(FLEXSPI2_CLK_SEL_A::FLEXSPI2_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL3 PFD1"]
    #[inline(always)]
    pub fn flexspi2_clk_sel_2(self) -> &'a mut W {
        self.variant(FLEXSPI2_CLK_SEL_A::FLEXSPI2_CLK_SEL_2)
    }
    #[doc = "derive clock from PLL2 (pll2_main_clk)"]
    #[inline(always)]
    pub fn flexspi2_clk_sel_3(self) -> &'a mut W {
        self.variant(FLEXSPI2_CLK_SEL_A::FLEXSPI2_CLK_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Selector for peripheral clk2 clock multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERIPH_CLK2_SEL_A {
    #[doc = "0: derive clock from pll3_sw_clk"]
    PERIPH_CLK2_SEL_0 = 0,
    #[doc = "1: derive clock from osc_clk (pll1_ref_clk)"]
    PERIPH_CLK2_SEL_1 = 1,
    #[doc = "2: derive clock from pll2_bypass_clk"]
    PERIPH_CLK2_SEL_2 = 2,
}
impl From<PERIPH_CLK2_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIPH_CLK2_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PERIPH_CLK2_SEL`"]
pub type PERIPH_CLK2_SEL_R = crate::R<u8, PERIPH_CLK2_SEL_A>;
impl PERIPH_CLK2_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PERIPH_CLK2_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_0),
            1 => Val(PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_1),
            2 => Val(PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_SEL_0`"]
    #[inline(always)]
    pub fn is_periph_clk2_sel_0(&self) -> bool {
        *self == PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_SEL_1`"]
    #[inline(always)]
    pub fn is_periph_clk2_sel_1(&self) -> bool {
        *self == PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_1
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_SEL_2`"]
    #[inline(always)]
    pub fn is_periph_clk2_sel_2(&self) -> bool {
        *self == PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_2
    }
}
#[doc = "Write proxy for field `PERIPH_CLK2_SEL`"]
pub struct PERIPH_CLK2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPH_CLK2_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIPH_CLK2_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "derive clock from pll3_sw_clk"]
    #[inline(always)]
    pub fn periph_clk2_sel_0(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_0)
    }
    #[doc = "derive clock from osc_clk (pll1_ref_clk)"]
    #[inline(always)]
    pub fn periph_clk2_sel_1(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_1)
    }
    #[doc = "derive clock from pll2_bypass_clk"]
    #[inline(always)]
    pub fn periph_clk2_sel_2(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_SEL_A::PERIPH_CLK2_SEL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Selector for Trace clock multiplexer\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRACE_CLK_SEL_A {
    #[doc = "0: derive clock from PLL2"]
    TRACE_CLK_SEL_0 = 0,
    #[doc = "1: derive clock from PLL2 PFD2"]
    TRACE_CLK_SEL_1 = 1,
    #[doc = "2: derive clock from PLL2 PFD0"]
    TRACE_CLK_SEL_2 = 2,
    #[doc = "3: derive clock from PLL2 PFD1"]
    TRACE_CLK_SEL_3 = 3,
}
impl From<TRACE_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACE_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRACE_CLK_SEL`"]
pub type TRACE_CLK_SEL_R = crate::R<u8, TRACE_CLK_SEL_A>;
impl TRACE_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACE_CLK_SEL_A {
        match self.bits {
            0 => TRACE_CLK_SEL_A::TRACE_CLK_SEL_0,
            1 => TRACE_CLK_SEL_A::TRACE_CLK_SEL_1,
            2 => TRACE_CLK_SEL_A::TRACE_CLK_SEL_2,
            3 => TRACE_CLK_SEL_A::TRACE_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRACE_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_trace_clk_sel_0(&self) -> bool {
        *self == TRACE_CLK_SEL_A::TRACE_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `TRACE_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_trace_clk_sel_1(&self) -> bool {
        *self == TRACE_CLK_SEL_A::TRACE_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `TRACE_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_trace_clk_sel_2(&self) -> bool {
        *self == TRACE_CLK_SEL_A::TRACE_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `TRACE_CLK_SEL_3`"]
    #[inline(always)]
    pub fn is_trace_clk_sel_3(&self) -> bool {
        *self == TRACE_CLK_SEL_A::TRACE_CLK_SEL_3
    }
}
#[doc = "Write proxy for field `TRACE_CLK_SEL`"]
pub struct TRACE_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACE_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACE_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "derive clock from PLL2"]
    #[inline(always)]
    pub fn trace_clk_sel_0(self) -> &'a mut W {
        self.variant(TRACE_CLK_SEL_A::TRACE_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline(always)]
    pub fn trace_clk_sel_1(self) -> &'a mut W {
        self.variant(TRACE_CLK_SEL_A::TRACE_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL2 PFD0"]
    #[inline(always)]
    pub fn trace_clk_sel_2(self) -> &'a mut W {
        self.variant(TRACE_CLK_SEL_A::TRACE_CLK_SEL_2)
    }
    #[doc = "derive clock from PLL2 PFD1"]
    #[inline(always)]
    pub fn trace_clk_sel_3(self) -> &'a mut W {
        self.variant(TRACE_CLK_SEL_A::TRACE_CLK_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Selector for pre_periph clock multiplexer\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRE_PERIPH_CLK_SEL_A {
    #[doc = "0: derive clock from PLL2"]
    PRE_PERIPH_CLK_SEL_0 = 0,
    #[doc = "1: derive clock from PLL2 PFD2"]
    PRE_PERIPH_CLK_SEL_1 = 1,
    #[doc = "2: derive clock from PLL2 PFD0"]
    PRE_PERIPH_CLK_SEL_2 = 2,
    #[doc = "3: derive clock from divided PLL1"]
    PRE_PERIPH_CLK_SEL_3 = 3,
}
impl From<PRE_PERIPH_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRE_PERIPH_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRE_PERIPH_CLK_SEL`"]
pub type PRE_PERIPH_CLK_SEL_R = crate::R<u8, PRE_PERIPH_CLK_SEL_A>;
impl PRE_PERIPH_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRE_PERIPH_CLK_SEL_A {
        match self.bits {
            0 => PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_0,
            1 => PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_1,
            2 => PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_2,
            3 => PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRE_PERIPH_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_pre_periph_clk_sel_0(&self) -> bool {
        *self == PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PRE_PERIPH_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_pre_periph_clk_sel_1(&self) -> bool {
        *self == PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `PRE_PERIPH_CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_pre_periph_clk_sel_2(&self) -> bool {
        *self == PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_2
    }
    #[doc = "Checks if the value of the field is `PRE_PERIPH_CLK_SEL_3`"]
    #[inline(always)]
    pub fn is_pre_periph_clk_sel_3(&self) -> bool {
        *self == PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_3
    }
}
#[doc = "Write proxy for field `PRE_PERIPH_CLK_SEL`"]
pub struct PRE_PERIPH_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_PERIPH_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRE_PERIPH_CLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "derive clock from PLL2"]
    #[inline(always)]
    pub fn pre_periph_clk_sel_0(self) -> &'a mut W {
        self.variant(PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL2 PFD2"]
    #[inline(always)]
    pub fn pre_periph_clk_sel_1(self) -> &'a mut W {
        self.variant(PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_1)
    }
    #[doc = "derive clock from PLL2 PFD0"]
    #[inline(always)]
    pub fn pre_periph_clk_sel_2(self) -> &'a mut W {
        self.variant(PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_2)
    }
    #[doc = "derive clock from divided PLL1"]
    #[inline(always)]
    pub fn pre_periph_clk_sel_3(self) -> &'a mut W {
        self.variant(PRE_PERIPH_CLK_SEL_A::PRE_PERIPH_CLK_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Post-divider for LCDIF clock.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCDIF_PODF_A {
    #[doc = "0: divide by 1"]
    LCDIF_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    LCDIF_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    LCDIF_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    LCDIF_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    LCDIF_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    LCDIF_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    LCDIF_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    LCDIF_PODF_7 = 7,
}
impl From<LCDIF_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: LCDIF_PODF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LCDIF_PODF`"]
pub type LCDIF_PODF_R = crate::R<u8, LCDIF_PODF_A>;
impl LCDIF_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDIF_PODF_A {
        match self.bits {
            0 => LCDIF_PODF_A::LCDIF_PODF_0,
            1 => LCDIF_PODF_A::LCDIF_PODF_1,
            2 => LCDIF_PODF_A::LCDIF_PODF_2,
            3 => LCDIF_PODF_A::LCDIF_PODF_3,
            4 => LCDIF_PODF_A::LCDIF_PODF_4,
            5 => LCDIF_PODF_A::LCDIF_PODF_5,
            6 => LCDIF_PODF_A::LCDIF_PODF_6,
            7 => LCDIF_PODF_A::LCDIF_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_0`"]
    #[inline(always)]
    pub fn is_lcdif_podf_0(&self) -> bool {
        *self == LCDIF_PODF_A::LCDIF_PODF_0
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_1`"]
    #[inline(always)]
    pub fn is_lcdif_podf_1(&self) -> bool {
        *self == LCDIF_PODF_A::LCDIF_PODF_1
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_2`"]
    #[inline(always)]
    pub fn is_lcdif_podf_2(&self) -> bool {
        *self == LCDIF_PODF_A::LCDIF_PODF_2
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_3`"]
    #[inline(always)]
    pub fn is_lcdif_podf_3(&self) -> bool {
        *self == LCDIF_PODF_A::LCDIF_PODF_3
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_4`"]
    #[inline(always)]
    pub fn is_lcdif_podf_4(&self) -> bool {
        *self == LCDIF_PODF_A::LCDIF_PODF_4
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_5`"]
    #[inline(always)]
    pub fn is_lcdif_podf_5(&self) -> bool {
        *self == LCDIF_PODF_A::LCDIF_PODF_5
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_6`"]
    #[inline(always)]
    pub fn is_lcdif_podf_6(&self) -> bool {
        *self == LCDIF_PODF_A::LCDIF_PODF_6
    }
    #[doc = "Checks if the value of the field is `LCDIF_PODF_7`"]
    #[inline(always)]
    pub fn is_lcdif_podf_7(&self) -> bool {
        *self == LCDIF_PODF_A::LCDIF_PODF_7
    }
}
#[doc = "Write proxy for field `LCDIF_PODF`"]
pub struct LCDIF_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDIF_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCDIF_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn lcdif_podf_0(self) -> &'a mut W {
        self.variant(LCDIF_PODF_A::LCDIF_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn lcdif_podf_1(self) -> &'a mut W {
        self.variant(LCDIF_PODF_A::LCDIF_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn lcdif_podf_2(self) -> &'a mut W {
        self.variant(LCDIF_PODF_A::LCDIF_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn lcdif_podf_3(self) -> &'a mut W {
        self.variant(LCDIF_PODF_A::LCDIF_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn lcdif_podf_4(self) -> &'a mut W {
        self.variant(LCDIF_PODF_A::LCDIF_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn lcdif_podf_5(self) -> &'a mut W {
        self.variant(LCDIF_PODF_A::LCDIF_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn lcdif_podf_6(self) -> &'a mut W {
        self.variant(LCDIF_PODF_A::LCDIF_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn lcdif_podf_7(self) -> &'a mut W {
        self.variant(LCDIF_PODF_A::LCDIF_PODF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Divider for LPSPI. Divider should be updated when output clock is gated.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPSPI_PODF_A {
    #[doc = "0: divide by 1"]
    LPSPI_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    LPSPI_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    LPSPI_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    LPSPI_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    LPSPI_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    LPSPI_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    LPSPI_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    LPSPI_PODF_7 = 7,
}
impl From<LPSPI_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: LPSPI_PODF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPSPI_PODF`"]
pub type LPSPI_PODF_R = crate::R<u8, LPSPI_PODF_A>;
impl LPSPI_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSPI_PODF_A {
        match self.bits {
            0 => LPSPI_PODF_A::LPSPI_PODF_0,
            1 => LPSPI_PODF_A::LPSPI_PODF_1,
            2 => LPSPI_PODF_A::LPSPI_PODF_2,
            3 => LPSPI_PODF_A::LPSPI_PODF_3,
            4 => LPSPI_PODF_A::LPSPI_PODF_4,
            5 => LPSPI_PODF_A::LPSPI_PODF_5,
            6 => LPSPI_PODF_A::LPSPI_PODF_6,
            7 => LPSPI_PODF_A::LPSPI_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_0`"]
    #[inline(always)]
    pub fn is_lpspi_podf_0(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_0
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_1`"]
    #[inline(always)]
    pub fn is_lpspi_podf_1(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_1
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_2`"]
    #[inline(always)]
    pub fn is_lpspi_podf_2(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_2
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_3`"]
    #[inline(always)]
    pub fn is_lpspi_podf_3(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_3
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_4`"]
    #[inline(always)]
    pub fn is_lpspi_podf_4(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_4
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_5`"]
    #[inline(always)]
    pub fn is_lpspi_podf_5(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_5
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_6`"]
    #[inline(always)]
    pub fn is_lpspi_podf_6(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_6
    }
    #[doc = "Checks if the value of the field is `LPSPI_PODF_7`"]
    #[inline(always)]
    pub fn is_lpspi_podf_7(&self) -> bool {
        *self == LPSPI_PODF_A::LPSPI_PODF_7
    }
}
#[doc = "Write proxy for field `LPSPI_PODF`"]
pub struct LPSPI_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSPI_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSPI_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn lpspi_podf_0(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn lpspi_podf_1(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn lpspi_podf_2(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn lpspi_podf_3(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn lpspi_podf_4(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn lpspi_podf_5(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn lpspi_podf_6(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn lpspi_podf_7(self) -> &'a mut W {
        self.variant(LPSPI_PODF_A::LPSPI_PODF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
#[doc = "Divider for flexspi2 clock root.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXSPI2_PODF_A {
    #[doc = "0: divide by 1"]
    FLEXSPI2_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    FLEXSPI2_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    FLEXSPI2_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    FLEXSPI2_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    FLEXSPI2_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    FLEXSPI2_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    FLEXSPI2_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    FLEXSPI2_PODF_7 = 7,
}
impl From<FLEXSPI2_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXSPI2_PODF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLEXSPI2_PODF`"]
pub type FLEXSPI2_PODF_R = crate::R<u8, FLEXSPI2_PODF_A>;
impl FLEXSPI2_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXSPI2_PODF_A {
        match self.bits {
            0 => FLEXSPI2_PODF_A::FLEXSPI2_PODF_0,
            1 => FLEXSPI2_PODF_A::FLEXSPI2_PODF_1,
            2 => FLEXSPI2_PODF_A::FLEXSPI2_PODF_2,
            3 => FLEXSPI2_PODF_A::FLEXSPI2_PODF_3,
            4 => FLEXSPI2_PODF_A::FLEXSPI2_PODF_4,
            5 => FLEXSPI2_PODF_A::FLEXSPI2_PODF_5,
            6 => FLEXSPI2_PODF_A::FLEXSPI2_PODF_6,
            7 => FLEXSPI2_PODF_A::FLEXSPI2_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_PODF_0`"]
    #[inline(always)]
    pub fn is_flexspi2_podf_0(&self) -> bool {
        *self == FLEXSPI2_PODF_A::FLEXSPI2_PODF_0
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_PODF_1`"]
    #[inline(always)]
    pub fn is_flexspi2_podf_1(&self) -> bool {
        *self == FLEXSPI2_PODF_A::FLEXSPI2_PODF_1
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_PODF_2`"]
    #[inline(always)]
    pub fn is_flexspi2_podf_2(&self) -> bool {
        *self == FLEXSPI2_PODF_A::FLEXSPI2_PODF_2
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_PODF_3`"]
    #[inline(always)]
    pub fn is_flexspi2_podf_3(&self) -> bool {
        *self == FLEXSPI2_PODF_A::FLEXSPI2_PODF_3
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_PODF_4`"]
    #[inline(always)]
    pub fn is_flexspi2_podf_4(&self) -> bool {
        *self == FLEXSPI2_PODF_A::FLEXSPI2_PODF_4
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_PODF_5`"]
    #[inline(always)]
    pub fn is_flexspi2_podf_5(&self) -> bool {
        *self == FLEXSPI2_PODF_A::FLEXSPI2_PODF_5
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_PODF_6`"]
    #[inline(always)]
    pub fn is_flexspi2_podf_6(&self) -> bool {
        *self == FLEXSPI2_PODF_A::FLEXSPI2_PODF_6
    }
    #[doc = "Checks if the value of the field is `FLEXSPI2_PODF_7`"]
    #[inline(always)]
    pub fn is_flexspi2_podf_7(&self) -> bool {
        *self == FLEXSPI2_PODF_A::FLEXSPI2_PODF_7
    }
}
#[doc = "Write proxy for field `FLEXSPI2_PODF`"]
pub struct FLEXSPI2_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXSPI2_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLEXSPI2_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn flexspi2_podf_0(self) -> &'a mut W {
        self.variant(FLEXSPI2_PODF_A::FLEXSPI2_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn flexspi2_podf_1(self) -> &'a mut W {
        self.variant(FLEXSPI2_PODF_A::FLEXSPI2_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn flexspi2_podf_2(self) -> &'a mut W {
        self.variant(FLEXSPI2_PODF_A::FLEXSPI2_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn flexspi2_podf_3(self) -> &'a mut W {
        self.variant(FLEXSPI2_PODF_A::FLEXSPI2_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn flexspi2_podf_4(self) -> &'a mut W {
        self.variant(FLEXSPI2_PODF_A::FLEXSPI2_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn flexspi2_podf_5(self) -> &'a mut W {
        self.variant(FLEXSPI2_PODF_A::FLEXSPI2_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn flexspi2_podf_6(self) -> &'a mut W {
        self.variant(FLEXSPI2_PODF_A::FLEXSPI2_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn flexspi2_podf_7(self) -> &'a mut W {
        self.variant(FLEXSPI2_PODF_A::FLEXSPI2_PODF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - Selector for lpspi clock multiplexer"]
    #[inline(always)]
    pub fn lpspi_clk_sel(&self) -> LPSPI_CLK_SEL_R {
        LPSPI_CLK_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Selector for flexspi2 clock multiplexer"]
    #[inline(always)]
    pub fn flexspi2_clk_sel(&self) -> FLEXSPI2_CLK_SEL_R {
        FLEXSPI2_CLK_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Selector for peripheral clk2 clock multiplexer"]
    #[inline(always)]
    pub fn periph_clk2_sel(&self) -> PERIPH_CLK2_SEL_R {
        PERIPH_CLK2_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Selector for Trace clock multiplexer"]
    #[inline(always)]
    pub fn trace_clk_sel(&self) -> TRACE_CLK_SEL_R {
        TRACE_CLK_SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Selector for pre_periph clock multiplexer"]
    #[inline(always)]
    pub fn pre_periph_clk_sel(&self) -> PRE_PERIPH_CLK_SEL_R {
        PRE_PERIPH_CLK_SEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 23:25 - Post-divider for LCDIF clock."]
    #[inline(always)]
    pub fn lcdif_podf(&self) -> LCDIF_PODF_R {
        LCDIF_PODF_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:28 - Divider for LPSPI. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn lpspi_podf(&self) -> LPSPI_PODF_R {
        LPSPI_PODF_R::new(((self.bits >> 26) & 0x07) as u8)
    }
    #[doc = "Bits 29:31 - Divider for flexspi2 clock root."]
    #[inline(always)]
    pub fn flexspi2_podf(&self) -> FLEXSPI2_PODF_R {
        FLEXSPI2_PODF_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Selector for lpspi clock multiplexer"]
    #[inline(always)]
    pub fn lpspi_clk_sel(&mut self) -> LPSPI_CLK_SEL_W {
        LPSPI_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Selector for flexspi2 clock multiplexer"]
    #[inline(always)]
    pub fn flexspi2_clk_sel(&mut self) -> FLEXSPI2_CLK_SEL_W {
        FLEXSPI2_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - Selector for peripheral clk2 clock multiplexer"]
    #[inline(always)]
    pub fn periph_clk2_sel(&mut self) -> PERIPH_CLK2_SEL_W {
        PERIPH_CLK2_SEL_W { w: self }
    }
    #[doc = "Bits 14:15 - Selector for Trace clock multiplexer"]
    #[inline(always)]
    pub fn trace_clk_sel(&mut self) -> TRACE_CLK_SEL_W {
        TRACE_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 18:19 - Selector for pre_periph clock multiplexer"]
    #[inline(always)]
    pub fn pre_periph_clk_sel(&mut self) -> PRE_PERIPH_CLK_SEL_W {
        PRE_PERIPH_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 23:25 - Post-divider for LCDIF clock."]
    #[inline(always)]
    pub fn lcdif_podf(&mut self) -> LCDIF_PODF_W {
        LCDIF_PODF_W { w: self }
    }
    #[doc = "Bits 26:28 - Divider for LPSPI. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn lpspi_podf(&mut self) -> LPSPI_PODF_W {
        LPSPI_PODF_W { w: self }
    }
    #[doc = "Bits 29:31 - Divider for flexspi2 clock root."]
    #[inline(always)]
    pub fn flexspi2_podf(&mut self) -> FLEXSPI2_PODF_W {
        FLEXSPI2_PODF_W { w: self }
    }
}
