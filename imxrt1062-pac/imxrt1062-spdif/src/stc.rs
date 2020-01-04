#[doc = "Reader of register STC"]
pub type R = crate::R<u32, super::STC>;
#[doc = "Writer for register STC"]
pub type W = crate::W<u32, super::STC>;
#[doc = "Register STC `reset()`'s with value 0x0002_0f00"]
impl crate::ResetValue for super::STC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_0f00
    }
}
#[doc = "Divider factor (1-128)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXCLK_DF_A {
    #[doc = "0: divider factor is 1"]
    TXCLK_DF_0 = 0,
    #[doc = "1: divider factor is 2"]
    TXCLK_DF_1 = 1,
    #[doc = "127: divider factor is 128"]
    TXCLK_DF_127 = 127,
}
impl From<TXCLK_DF_A> for u8 {
    #[inline(always)]
    fn from(variant: TXCLK_DF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TxClk_DF`"]
pub type TXCLK_DF_R = crate::R<u8, TXCLK_DF_A>;
impl TXCLK_DF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXCLK_DF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXCLK_DF_A::TXCLK_DF_0),
            1 => Val(TXCLK_DF_A::TXCLK_DF_1),
            127 => Val(TXCLK_DF_A::TXCLK_DF_127),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXCLK_DF_0`"]
    #[inline(always)]
    pub fn is_tx_clk_df_0(&self) -> bool {
        *self == TXCLK_DF_A::TXCLK_DF_0
    }
    #[doc = "Checks if the value of the field is `TXCLK_DF_1`"]
    #[inline(always)]
    pub fn is_tx_clk_df_1(&self) -> bool {
        *self == TXCLK_DF_A::TXCLK_DF_1
    }
    #[doc = "Checks if the value of the field is `TXCLK_DF_127`"]
    #[inline(always)]
    pub fn is_tx_clk_df_127(&self) -> bool {
        *self == TXCLK_DF_A::TXCLK_DF_127
    }
}
#[doc = "Write proxy for field `TxClk_DF`"]
pub struct TXCLK_DF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCLK_DF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCLK_DF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "divider factor is 1"]
    #[inline(always)]
    pub fn tx_clk_df_0(self) -> &'a mut W {
        self.variant(TXCLK_DF_A::TXCLK_DF_0)
    }
    #[doc = "divider factor is 2"]
    #[inline(always)]
    pub fn tx_clk_df_1(self) -> &'a mut W {
        self.variant(TXCLK_DF_A::TXCLK_DF_1)
    }
    #[doc = "divider factor is 128"]
    #[inline(always)]
    pub fn tx_clk_df_127(self) -> &'a mut W {
        self.variant(TXCLK_DF_A::TXCLK_DF_127)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_ALL_CLK_EN_A {
    #[doc = "0: disable transfer clock."]
    TX_ALL_CLK_EN_0 = 0,
    #[doc = "1: enable transfer clock."]
    TX_ALL_CLK_EN_1 = 1,
}
impl From<TX_ALL_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_ALL_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `tx_all_clk_en`"]
pub type TX_ALL_CLK_EN_R = crate::R<bool, TX_ALL_CLK_EN_A>;
impl TX_ALL_CLK_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_ALL_CLK_EN_A {
        match self.bits {
            false => TX_ALL_CLK_EN_A::TX_ALL_CLK_EN_0,
            true => TX_ALL_CLK_EN_A::TX_ALL_CLK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TX_ALL_CLK_EN_0`"]
    #[inline(always)]
    pub fn is_tx_all_clk_en_0(&self) -> bool {
        *self == TX_ALL_CLK_EN_A::TX_ALL_CLK_EN_0
    }
    #[doc = "Checks if the value of the field is `TX_ALL_CLK_EN_1`"]
    #[inline(always)]
    pub fn is_tx_all_clk_en_1(&self) -> bool {
        *self == TX_ALL_CLK_EN_A::TX_ALL_CLK_EN_1
    }
}
#[doc = "Write proxy for field `tx_all_clk_en`"]
pub struct TX_ALL_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ALL_CLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_ALL_CLK_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disable transfer clock."]
    #[inline(always)]
    pub fn tx_all_clk_en_0(self) -> &'a mut W {
        self.variant(TX_ALL_CLK_EN_A::TX_ALL_CLK_EN_0)
    }
    #[doc = "enable transfer clock."]
    #[inline(always)]
    pub fn tx_all_clk_en_1(self) -> &'a mut W {
        self.variant(TX_ALL_CLK_EN_A::TX_ALL_CLK_EN_1)
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
#[doc = "no description available\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXCLK_SOURCE_A {
    #[doc = "0: XTALOSC input (XTALOSC clock)"]
    TXCLK_SOURCE_0 = 0,
    #[doc = "1: tx_clk input (from SPDIF0_CLK_ROOT. See CCM.)"]
    TXCLK_SOURCE_1 = 1,
    #[doc = "2: tx_clk1 (from SAI1)"]
    TXCLK_SOURCE_2 = 2,
    #[doc = "3: tx_clk2 SPDIF_EXT_CLK, from pads"]
    TXCLK_SOURCE_3 = 3,
    #[doc = "4: tx_clk3 (from SAI2)"]
    TXCLK_SOURCE_4 = 4,
    #[doc = "5: ipg_clk input (frequency divided)"]
    TXCLK_SOURCE_5 = 5,
    #[doc = "6: tx_clk4 (from SAI3)"]
    TXCLK_SOURCE_6 = 6,
}
impl From<TXCLK_SOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: TXCLK_SOURCE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TxClk_Source`"]
pub type TXCLK_SOURCE_R = crate::R<u8, TXCLK_SOURCE_A>;
impl TXCLK_SOURCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXCLK_SOURCE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXCLK_SOURCE_A::TXCLK_SOURCE_0),
            1 => Val(TXCLK_SOURCE_A::TXCLK_SOURCE_1),
            2 => Val(TXCLK_SOURCE_A::TXCLK_SOURCE_2),
            3 => Val(TXCLK_SOURCE_A::TXCLK_SOURCE_3),
            4 => Val(TXCLK_SOURCE_A::TXCLK_SOURCE_4),
            5 => Val(TXCLK_SOURCE_A::TXCLK_SOURCE_5),
            6 => Val(TXCLK_SOURCE_A::TXCLK_SOURCE_6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TXCLK_SOURCE_0`"]
    #[inline(always)]
    pub fn is_tx_clk_source_0(&self) -> bool {
        *self == TXCLK_SOURCE_A::TXCLK_SOURCE_0
    }
    #[doc = "Checks if the value of the field is `TXCLK_SOURCE_1`"]
    #[inline(always)]
    pub fn is_tx_clk_source_1(&self) -> bool {
        *self == TXCLK_SOURCE_A::TXCLK_SOURCE_1
    }
    #[doc = "Checks if the value of the field is `TXCLK_SOURCE_2`"]
    #[inline(always)]
    pub fn is_tx_clk_source_2(&self) -> bool {
        *self == TXCLK_SOURCE_A::TXCLK_SOURCE_2
    }
    #[doc = "Checks if the value of the field is `TXCLK_SOURCE_3`"]
    #[inline(always)]
    pub fn is_tx_clk_source_3(&self) -> bool {
        *self == TXCLK_SOURCE_A::TXCLK_SOURCE_3
    }
    #[doc = "Checks if the value of the field is `TXCLK_SOURCE_4`"]
    #[inline(always)]
    pub fn is_tx_clk_source_4(&self) -> bool {
        *self == TXCLK_SOURCE_A::TXCLK_SOURCE_4
    }
    #[doc = "Checks if the value of the field is `TXCLK_SOURCE_5`"]
    #[inline(always)]
    pub fn is_tx_clk_source_5(&self) -> bool {
        *self == TXCLK_SOURCE_A::TXCLK_SOURCE_5
    }
    #[doc = "Checks if the value of the field is `TXCLK_SOURCE_6`"]
    #[inline(always)]
    pub fn is_tx_clk_source_6(&self) -> bool {
        *self == TXCLK_SOURCE_A::TXCLK_SOURCE_6
    }
}
#[doc = "Write proxy for field `TxClk_Source`"]
pub struct TXCLK_SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCLK_SOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCLK_SOURCE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "XTALOSC input (XTALOSC clock)"]
    #[inline(always)]
    pub fn tx_clk_source_0(self) -> &'a mut W {
        self.variant(TXCLK_SOURCE_A::TXCLK_SOURCE_0)
    }
    #[doc = "tx_clk input (from SPDIF0_CLK_ROOT. See CCM.)"]
    #[inline(always)]
    pub fn tx_clk_source_1(self) -> &'a mut W {
        self.variant(TXCLK_SOURCE_A::TXCLK_SOURCE_1)
    }
    #[doc = "tx_clk1 (from SAI1)"]
    #[inline(always)]
    pub fn tx_clk_source_2(self) -> &'a mut W {
        self.variant(TXCLK_SOURCE_A::TXCLK_SOURCE_2)
    }
    #[doc = "tx_clk2 SPDIF_EXT_CLK, from pads"]
    #[inline(always)]
    pub fn tx_clk_source_3(self) -> &'a mut W {
        self.variant(TXCLK_SOURCE_A::TXCLK_SOURCE_3)
    }
    #[doc = "tx_clk3 (from SAI2)"]
    #[inline(always)]
    pub fn tx_clk_source_4(self) -> &'a mut W {
        self.variant(TXCLK_SOURCE_A::TXCLK_SOURCE_4)
    }
    #[doc = "ipg_clk input (frequency divided)"]
    #[inline(always)]
    pub fn tx_clk_source_5(self) -> &'a mut W {
        self.variant(TXCLK_SOURCE_A::TXCLK_SOURCE_5)
    }
    #[doc = "tx_clk4 (from SAI3)"]
    #[inline(always)]
    pub fn tx_clk_source_6(self) -> &'a mut W {
        self.variant(TXCLK_SOURCE_A::TXCLK_SOURCE_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "system clock divider factor, 2~512.\n\nValue on reset: 65"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SYSCLK_DF_A {
    #[doc = "0: no clock signal"]
    SYSCLK_DF_0 = 0,
    #[doc = "1: divider factor is 2"]
    SYSCLK_DF_1 = 1,
    #[doc = "511: divider factor is 512"]
    SYSCLK_DF_511 = 511,
}
impl From<SYSCLK_DF_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSCLK_DF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYSCLK_DF`"]
pub type SYSCLK_DF_R = crate::R<u16, SYSCLK_DF_A>;
impl SYSCLK_DF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SYSCLK_DF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSCLK_DF_A::SYSCLK_DF_0),
            1 => Val(SYSCLK_DF_A::SYSCLK_DF_1),
            511 => Val(SYSCLK_DF_A::SYSCLK_DF_511),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK_DF_0`"]
    #[inline(always)]
    pub fn is_sysclk_df_0(&self) -> bool {
        *self == SYSCLK_DF_A::SYSCLK_DF_0
    }
    #[doc = "Checks if the value of the field is `SYSCLK_DF_1`"]
    #[inline(always)]
    pub fn is_sysclk_df_1(&self) -> bool {
        *self == SYSCLK_DF_A::SYSCLK_DF_1
    }
    #[doc = "Checks if the value of the field is `SYSCLK_DF_511`"]
    #[inline(always)]
    pub fn is_sysclk_df_511(&self) -> bool {
        *self == SYSCLK_DF_A::SYSCLK_DF_511
    }
}
#[doc = "Write proxy for field `SYSCLK_DF`"]
pub struct SYSCLK_DF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCLK_DF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCLK_DF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "no clock signal"]
    #[inline(always)]
    pub fn sysclk_df_0(self) -> &'a mut W {
        self.variant(SYSCLK_DF_A::SYSCLK_DF_0)
    }
    #[doc = "divider factor is 2"]
    #[inline(always)]
    pub fn sysclk_df_1(self) -> &'a mut W {
        self.variant(SYSCLK_DF_A::SYSCLK_DF_1)
    }
    #[doc = "divider factor is 512"]
    #[inline(always)]
    pub fn sysclk_df_511(self) -> &'a mut W {
        self.variant(SYSCLK_DF_A::SYSCLK_DF_511)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 11)) | (((value as u32) & 0x01ff) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Divider factor (1-128)"]
    #[inline(always)]
    pub fn tx_clk_df(&self) -> TXCLK_DF_R {
        TXCLK_DF_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1."]
    #[inline(always)]
    pub fn tx_all_clk_en(&self) -> TX_ALL_CLK_EN_R {
        TX_ALL_CLK_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - no description available"]
    #[inline(always)]
    pub fn tx_clk_source(&self) -> TXCLK_SOURCE_R {
        TXCLK_SOURCE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:19 - system clock divider factor, 2~512."]
    #[inline(always)]
    pub fn sysclk_df(&self) -> SYSCLK_DF_R {
        SYSCLK_DF_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Divider factor (1-128)"]
    #[inline(always)]
    pub fn tx_clk_df(&mut self) -> TXCLK_DF_W {
        TXCLK_DF_W { w: self }
    }
    #[doc = "Bit 7 - Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1."]
    #[inline(always)]
    pub fn tx_all_clk_en(&mut self) -> TX_ALL_CLK_EN_W {
        TX_ALL_CLK_EN_W { w: self }
    }
    #[doc = "Bits 8:10 - no description available"]
    #[inline(always)]
    pub fn tx_clk_source(&mut self) -> TXCLK_SOURCE_W {
        TXCLK_SOURCE_W { w: self }
    }
    #[doc = "Bits 11:19 - system clock divider factor, 2~512."]
    #[inline(always)]
    pub fn sysclk_df(&mut self) -> SYSCLK_DF_W {
        SYSCLK_DF_W { w: self }
    }
}
