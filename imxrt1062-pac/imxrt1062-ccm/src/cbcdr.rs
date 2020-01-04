#[doc = "Reader of register CBCDR"]
pub type R = crate::R<u32, super::CBCDR>;
#[doc = "Writer for register CBCDR"]
pub type W = crate::W<u32, super::CBCDR>;
#[doc = "Register CBCDR `reset()`'s with value 0x000a_8300"]
impl crate::ResetValue for super::CBCDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000a_8300
    }
}
#[doc = "SEMC clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_CLK_SEL_A {
    #[doc = "0: Periph_clk output will be used as SEMC clock root"]
    SEMC_CLK_SEL_0 = 0,
    #[doc = "1: SEMC alternative clock will be used as SEMC clock root"]
    SEMC_CLK_SEL_1 = 1,
}
impl From<SEMC_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEMC_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEMC_CLK_SEL`"]
pub type SEMC_CLK_SEL_R = crate::R<bool, SEMC_CLK_SEL_A>;
impl SEMC_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_CLK_SEL_A {
        match self.bits {
            false => SEMC_CLK_SEL_A::SEMC_CLK_SEL_0,
            true => SEMC_CLK_SEL_A::SEMC_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_semc_clk_sel_0(&self) -> bool {
        *self == SEMC_CLK_SEL_A::SEMC_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SEMC_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_semc_clk_sel_1(&self) -> bool {
        *self == SEMC_CLK_SEL_A::SEMC_CLK_SEL_1
    }
}
#[doc = "Write proxy for field `SEMC_CLK_SEL`"]
pub struct SEMC_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEMC_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEMC_CLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Periph_clk output will be used as SEMC clock root"]
    #[inline(always)]
    pub fn semc_clk_sel_0(self) -> &'a mut W {
        self.variant(SEMC_CLK_SEL_A::SEMC_CLK_SEL_0)
    }
    #[doc = "SEMC alternative clock will be used as SEMC clock root"]
    #[inline(always)]
    pub fn semc_clk_sel_1(self) -> &'a mut W {
        self.variant(SEMC_CLK_SEL_A::SEMC_CLK_SEL_1)
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
#[doc = "SEMC alternative clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_ALT_CLK_SEL_A {
    #[doc = "0: PLL2 PFD2 will be selected as alternative clock for SEMC root clock"]
    SEMC_ALT_CLK_SEL_0 = 0,
    #[doc = "1: PLL3 PFD1 will be selected as alternative clock for SEMC root clock"]
    SEMC_ALT_CLK_SEL_1 = 1,
}
impl From<SEMC_ALT_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEMC_ALT_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEMC_ALT_CLK_SEL`"]
pub type SEMC_ALT_CLK_SEL_R = crate::R<bool, SEMC_ALT_CLK_SEL_A>;
impl SEMC_ALT_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_ALT_CLK_SEL_A {
        match self.bits {
            false => SEMC_ALT_CLK_SEL_A::SEMC_ALT_CLK_SEL_0,
            true => SEMC_ALT_CLK_SEL_A::SEMC_ALT_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_ALT_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_semc_alt_clk_sel_0(&self) -> bool {
        *self == SEMC_ALT_CLK_SEL_A::SEMC_ALT_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SEMC_ALT_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_semc_alt_clk_sel_1(&self) -> bool {
        *self == SEMC_ALT_CLK_SEL_A::SEMC_ALT_CLK_SEL_1
    }
}
#[doc = "Write proxy for field `SEMC_ALT_CLK_SEL`"]
pub struct SEMC_ALT_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEMC_ALT_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEMC_ALT_CLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL2 PFD2 will be selected as alternative clock for SEMC root clock"]
    #[inline(always)]
    pub fn semc_alt_clk_sel_0(self) -> &'a mut W {
        self.variant(SEMC_ALT_CLK_SEL_A::SEMC_ALT_CLK_SEL_0)
    }
    #[doc = "PLL3 PFD1 will be selected as alternative clock for SEMC root clock"]
    #[inline(always)]
    pub fn semc_alt_clk_sel_1(self) -> &'a mut W {
        self.variant(SEMC_ALT_CLK_SEL_A::SEMC_ALT_CLK_SEL_1)
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
#[doc = "Divider for ipg podf.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IPG_PODF_A {
    #[doc = "0: divide by 1"]
    IPG_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    IPG_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    IPG_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    IPG_PODF_3 = 3,
}
impl From<IPG_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: IPG_PODF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IPG_PODF`"]
pub type IPG_PODF_R = crate::R<u8, IPG_PODF_A>;
impl IPG_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPG_PODF_A {
        match self.bits {
            0 => IPG_PODF_A::IPG_PODF_0,
            1 => IPG_PODF_A::IPG_PODF_1,
            2 => IPG_PODF_A::IPG_PODF_2,
            3 => IPG_PODF_A::IPG_PODF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IPG_PODF_0`"]
    #[inline(always)]
    pub fn is_ipg_podf_0(&self) -> bool {
        *self == IPG_PODF_A::IPG_PODF_0
    }
    #[doc = "Checks if the value of the field is `IPG_PODF_1`"]
    #[inline(always)]
    pub fn is_ipg_podf_1(&self) -> bool {
        *self == IPG_PODF_A::IPG_PODF_1
    }
    #[doc = "Checks if the value of the field is `IPG_PODF_2`"]
    #[inline(always)]
    pub fn is_ipg_podf_2(&self) -> bool {
        *self == IPG_PODF_A::IPG_PODF_2
    }
    #[doc = "Checks if the value of the field is `IPG_PODF_3`"]
    #[inline(always)]
    pub fn is_ipg_podf_3(&self) -> bool {
        *self == IPG_PODF_A::IPG_PODF_3
    }
}
#[doc = "Write proxy for field `IPG_PODF`"]
pub struct IPG_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> IPG_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPG_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn ipg_podf_0(self) -> &'a mut W {
        self.variant(IPG_PODF_A::IPG_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn ipg_podf_1(self) -> &'a mut W {
        self.variant(IPG_PODF_A::IPG_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn ipg_podf_2(self) -> &'a mut W {
        self.variant(IPG_PODF_A::IPG_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn ipg_podf_3(self) -> &'a mut W {
        self.variant(IPG_PODF_A::IPG_PODF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Divider for AHB PODF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AHB_PODF_A {
    #[doc = "0: divide by 1"]
    AHB_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    AHB_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    AHB_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    AHB_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    AHB_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    AHB_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    AHB_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    AHB_PODF_7 = 7,
}
impl From<AHB_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: AHB_PODF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AHB_PODF`"]
pub type AHB_PODF_R = crate::R<u8, AHB_PODF_A>;
impl AHB_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_PODF_A {
        match self.bits {
            0 => AHB_PODF_A::AHB_PODF_0,
            1 => AHB_PODF_A::AHB_PODF_1,
            2 => AHB_PODF_A::AHB_PODF_2,
            3 => AHB_PODF_A::AHB_PODF_3,
            4 => AHB_PODF_A::AHB_PODF_4,
            5 => AHB_PODF_A::AHB_PODF_5,
            6 => AHB_PODF_A::AHB_PODF_6,
            7 => AHB_PODF_A::AHB_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_0`"]
    #[inline(always)]
    pub fn is_ahb_podf_0(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_0
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_1`"]
    #[inline(always)]
    pub fn is_ahb_podf_1(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_1
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_2`"]
    #[inline(always)]
    pub fn is_ahb_podf_2(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_2
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_3`"]
    #[inline(always)]
    pub fn is_ahb_podf_3(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_3
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_4`"]
    #[inline(always)]
    pub fn is_ahb_podf_4(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_4
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_5`"]
    #[inline(always)]
    pub fn is_ahb_podf_5(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_5
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_6`"]
    #[inline(always)]
    pub fn is_ahb_podf_6(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_6
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_7`"]
    #[inline(always)]
    pub fn is_ahb_podf_7(&self) -> bool {
        *self == AHB_PODF_A::AHB_PODF_7
    }
}
#[doc = "Write proxy for field `AHB_PODF`"]
pub struct AHB_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn ahb_podf_0(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn ahb_podf_1(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn ahb_podf_2(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn ahb_podf_3(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn ahb_podf_4(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn ahb_podf_5(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn ahb_podf_6(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn ahb_podf_7(self) -> &'a mut W {
        self.variant(AHB_PODF_A::AHB_PODF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Post divider for SEMC clock\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEMC_PODF_A {
    #[doc = "0: divide by 1"]
    SEMC_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    SEMC_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    SEMC_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    SEMC_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    SEMC_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    SEMC_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    SEMC_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    SEMC_PODF_7 = 7,
}
impl From<SEMC_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: SEMC_PODF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEMC_PODF`"]
pub type SEMC_PODF_R = crate::R<u8, SEMC_PODF_A>;
impl SEMC_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_PODF_A {
        match self.bits {
            0 => SEMC_PODF_A::SEMC_PODF_0,
            1 => SEMC_PODF_A::SEMC_PODF_1,
            2 => SEMC_PODF_A::SEMC_PODF_2,
            3 => SEMC_PODF_A::SEMC_PODF_3,
            4 => SEMC_PODF_A::SEMC_PODF_4,
            5 => SEMC_PODF_A::SEMC_PODF_5,
            6 => SEMC_PODF_A::SEMC_PODF_6,
            7 => SEMC_PODF_A::SEMC_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_0`"]
    #[inline(always)]
    pub fn is_semc_podf_0(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_0
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_1`"]
    #[inline(always)]
    pub fn is_semc_podf_1(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_1
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_2`"]
    #[inline(always)]
    pub fn is_semc_podf_2(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_2
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_3`"]
    #[inline(always)]
    pub fn is_semc_podf_3(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_3
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_4`"]
    #[inline(always)]
    pub fn is_semc_podf_4(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_4
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_5`"]
    #[inline(always)]
    pub fn is_semc_podf_5(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_5
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_6`"]
    #[inline(always)]
    pub fn is_semc_podf_6(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_6
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_7`"]
    #[inline(always)]
    pub fn is_semc_podf_7(&self) -> bool {
        *self == SEMC_PODF_A::SEMC_PODF_7
    }
}
#[doc = "Write proxy for field `SEMC_PODF`"]
pub struct SEMC_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEMC_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEMC_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn semc_podf_0(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn semc_podf_1(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn semc_podf_2(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn semc_podf_3(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn semc_podf_4(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn semc_podf_5(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn semc_podf_6(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn semc_podf_7(self) -> &'a mut W {
        self.variant(SEMC_PODF_A::SEMC_PODF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Selector for peripheral main clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPH_CLK_SEL_A {
    #[doc = "0: derive clock from pre_periph_clk_sel"]
    PERIPH_CLK_SEL_0 = 0,
    #[doc = "1: derive clock from periph_clk2_clk_divided"]
    PERIPH_CLK_SEL_1 = 1,
}
impl From<PERIPH_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPH_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PERIPH_CLK_SEL`"]
pub type PERIPH_CLK_SEL_R = crate::R<bool, PERIPH_CLK_SEL_A>;
impl PERIPH_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPH_CLK_SEL_A {
        match self.bits {
            false => PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_0,
            true => PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_periph_clk_sel_0(&self) -> bool {
        *self == PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_periph_clk_sel_1(&self) -> bool {
        *self == PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_1
    }
}
#[doc = "Write proxy for field `PERIPH_CLK_SEL`"]
pub struct PERIPH_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPH_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIPH_CLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "derive clock from pre_periph_clk_sel"]
    #[inline(always)]
    pub fn periph_clk_sel_0(self) -> &'a mut W {
        self.variant(PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_0)
    }
    #[doc = "derive clock from periph_clk2_clk_divided"]
    #[inline(always)]
    pub fn periph_clk_sel_1(self) -> &'a mut W {
        self.variant(PERIPH_CLK_SEL_A::PERIPH_CLK_SEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Divider for periph_clk2_podf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERIPH_CLK2_PODF_A {
    #[doc = "0: divide by 1"]
    PERIPH_CLK2_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    PERIPH_CLK2_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    PERIPH_CLK2_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    PERIPH_CLK2_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    PERIPH_CLK2_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    PERIPH_CLK2_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    PERIPH_CLK2_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    PERIPH_CLK2_PODF_7 = 7,
}
impl From<PERIPH_CLK2_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIPH_CLK2_PODF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PERIPH_CLK2_PODF`"]
pub type PERIPH_CLK2_PODF_R = crate::R<u8, PERIPH_CLK2_PODF_A>;
impl PERIPH_CLK2_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPH_CLK2_PODF_A {
        match self.bits {
            0 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_0,
            1 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_1,
            2 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_2,
            3 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_3,
            4 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_4,
            5 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_5,
            6 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_6,
            7 => PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_0`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_0(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_1`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_1(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_1
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_2`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_2(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_2
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_3`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_3(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_3
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_4`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_4(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_4
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_5`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_5(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_5
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_6`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_6(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_6
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK2_PODF_7`"]
    #[inline(always)]
    pub fn is_periph_clk2_podf_7(&self) -> bool {
        *self == PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_7
    }
}
#[doc = "Write proxy for field `PERIPH_CLK2_PODF`"]
pub struct PERIPH_CLK2_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPH_CLK2_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIPH_CLK2_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn periph_clk2_podf_0(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn periph_clk2_podf_1(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn periph_clk2_podf_2(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn periph_clk2_podf_3(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn periph_clk2_podf_4(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn periph_clk2_podf_5(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn periph_clk2_podf_6(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn periph_clk2_podf_7(self) -> &'a mut W {
        self.variant(PERIPH_CLK2_PODF_A::PERIPH_CLK2_PODF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - SEMC clock source select"]
    #[inline(always)]
    pub fn semc_clk_sel(&self) -> SEMC_CLK_SEL_R {
        SEMC_CLK_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SEMC alternative clock select"]
    #[inline(always)]
    pub fn semc_alt_clk_sel(&self) -> SEMC_ALT_CLK_SEL_R {
        SEMC_ALT_CLK_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Divider for ipg podf."]
    #[inline(always)]
    pub fn ipg_podf(&self) -> IPG_PODF_R {
        IPG_PODF_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12 - Divider for AHB PODF"]
    #[inline(always)]
    pub fn ahb_podf(&self) -> AHB_PODF_R {
        AHB_PODF_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Post divider for SEMC clock"]
    #[inline(always)]
    pub fn semc_podf(&self) -> SEMC_PODF_R {
        SEMC_PODF_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 25 - Selector for peripheral main clock"]
    #[inline(always)]
    pub fn periph_clk_sel(&self) -> PERIPH_CLK_SEL_R {
        PERIPH_CLK_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 27:29 - Divider for periph_clk2_podf."]
    #[inline(always)]
    pub fn periph_clk2_podf(&self) -> PERIPH_CLK2_PODF_R {
        PERIPH_CLK2_PODF_R::new(((self.bits >> 27) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - SEMC clock source select"]
    #[inline(always)]
    pub fn semc_clk_sel(&mut self) -> SEMC_CLK_SEL_W {
        SEMC_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 7 - SEMC alternative clock select"]
    #[inline(always)]
    pub fn semc_alt_clk_sel(&mut self) -> SEMC_ALT_CLK_SEL_W {
        SEMC_ALT_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Divider for ipg podf."]
    #[inline(always)]
    pub fn ipg_podf(&mut self) -> IPG_PODF_W {
        IPG_PODF_W { w: self }
    }
    #[doc = "Bits 10:12 - Divider for AHB PODF"]
    #[inline(always)]
    pub fn ahb_podf(&mut self) -> AHB_PODF_W {
        AHB_PODF_W { w: self }
    }
    #[doc = "Bits 16:18 - Post divider for SEMC clock"]
    #[inline(always)]
    pub fn semc_podf(&mut self) -> SEMC_PODF_W {
        SEMC_PODF_W { w: self }
    }
    #[doc = "Bit 25 - Selector for peripheral main clock"]
    #[inline(always)]
    pub fn periph_clk_sel(&mut self) -> PERIPH_CLK_SEL_W {
        PERIPH_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 27:29 - Divider for periph_clk2_podf."]
    #[inline(always)]
    pub fn periph_clk2_podf(&mut self) -> PERIPH_CLK2_PODF_W {
        PERIPH_CLK2_PODF_W { w: self }
    }
}
