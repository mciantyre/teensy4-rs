#[doc = "Reader of register VEND_SPEC2"]
pub type R = crate::R<u32, super::VEND_SPEC2>;
#[doc = "Writer for register VEND_SPEC2"]
pub type W = crate::W<u32, super::VEND_SPEC2>;
#[doc = "Register VEND_SPEC2 `reset()`'s with value 0x1006"]
impl crate::ResetValue for super::VEND_SPEC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1006
    }
}
#[doc = "Card Interrupt Detection Test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INT_D3_TEST_A {
    #[doc = "0: Check the card interrupt only when DATA3 is high."]
    CARD_INT_D3_TEST_0 = 0,
    #[doc = "1: Check the card interrupt by ignoring the status of DATA3."]
    CARD_INT_D3_TEST_1 = 1,
}
impl From<CARD_INT_D3_TEST_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INT_D3_TEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CARD_INT_D3_TEST`"]
pub type CARD_INT_D3_TEST_R = crate::R<bool, CARD_INT_D3_TEST_A>;
impl CARD_INT_D3_TEST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_INT_D3_TEST_A {
        match self.bits {
            false => CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_0,
            true => CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CARD_INT_D3_TEST_0`"]
    #[inline(always)]
    pub fn is_card_int_d3_test_0(&self) -> bool {
        *self == CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_0
    }
    #[doc = "Checks if the value of the field is `CARD_INT_D3_TEST_1`"]
    #[inline(always)]
    pub fn is_card_int_d3_test_1(&self) -> bool {
        *self == CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_1
    }
}
#[doc = "Write proxy for field `CARD_INT_D3_TEST`"]
pub struct CARD_INT_D3_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_INT_D3_TEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_INT_D3_TEST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Check the card interrupt only when DATA3 is high."]
    #[inline(always)]
    pub fn card_int_d3_test_0(self) -> &'a mut W {
        self.variant(CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_0)
    }
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    #[inline(always)]
    pub fn card_int_d3_test_1(self) -> &'a mut W {
        self.variant(CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TUNING_8bit_EN`"]
pub type TUNING_8BIT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TUNING_8bit_EN`"]
pub struct TUNING_8BIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_8BIT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TUNING_1bit_EN`"]
pub type TUNING_1BIT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TUNING_1bit_EN`"]
pub struct TUNING_1BIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_1BIT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "TUNING_CMD_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TUNING_CMD_EN_A {
    #[doc = "0: Auto tuning circuit does not check the CMD line."]
    TUNING_CMD_EN_0 = 0,
    #[doc = "1: Auto tuning circuit checks the CMD line."]
    TUNING_CMD_EN_1 = 1,
}
impl From<TUNING_CMD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TUNING_CMD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TUNING_CMD_EN`"]
pub type TUNING_CMD_EN_R = crate::R<bool, TUNING_CMD_EN_A>;
impl TUNING_CMD_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TUNING_CMD_EN_A {
        match self.bits {
            false => TUNING_CMD_EN_A::TUNING_CMD_EN_0,
            true => TUNING_CMD_EN_A::TUNING_CMD_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TUNING_CMD_EN_0`"]
    #[inline(always)]
    pub fn is_tuning_cmd_en_0(&self) -> bool {
        *self == TUNING_CMD_EN_A::TUNING_CMD_EN_0
    }
    #[doc = "Checks if the value of the field is `TUNING_CMD_EN_1`"]
    #[inline(always)]
    pub fn is_tuning_cmd_en_1(&self) -> bool {
        *self == TUNING_CMD_EN_A::TUNING_CMD_EN_1
    }
}
#[doc = "Write proxy for field `TUNING_CMD_EN`"]
pub struct TUNING_CMD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_CMD_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TUNING_CMD_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Auto tuning circuit does not check the CMD line."]
    #[inline(always)]
    pub fn tuning_cmd_en_0(self) -> &'a mut W {
        self.variant(TUNING_CMD_EN_A::TUNING_CMD_EN_0)
    }
    #[doc = "Auto tuning circuit checks the CMD line."]
    #[inline(always)]
    pub fn tuning_cmd_en_1(self) -> &'a mut W {
        self.variant(TUNING_CMD_EN_A::TUNING_CMD_EN_1)
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
#[doc = "Argument2 register enable for ACMD23\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMD23_ARGU2_EN_A {
    #[doc = "0: Disable"]
    ACMD23_ARGU2_EN_0 = 0,
    #[doc = "1: Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enable."]
    ACMD23_ARGU2_EN_1 = 1,
}
impl From<ACMD23_ARGU2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD23_ARGU2_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMD23_ARGU2_EN`"]
pub type ACMD23_ARGU2_EN_R = crate::R<bool, ACMD23_ARGU2_EN_A>;
impl ACMD23_ARGU2_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMD23_ARGU2_EN_A {
        match self.bits {
            false => ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_0,
            true => ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMD23_ARGU2_EN_0`"]
    #[inline(always)]
    pub fn is_acmd23_argu2_en_0(&self) -> bool {
        *self == ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMD23_ARGU2_EN_1`"]
    #[inline(always)]
    pub fn is_acmd23_argu2_en_1(&self) -> bool {
        *self == ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_1
    }
}
#[doc = "Write proxy for field `ACMD23_ARGU2_EN`"]
pub struct ACMD23_ARGU2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMD23_ARGU2_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMD23_ARGU2_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn acmd23_argu2_en_0(self) -> &'a mut W {
        self.variant(ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_0)
    }
    #[doc = "Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enable."]
    #[inline(always)]
    pub fn acmd23_argu2_en_1(self) -> &'a mut W {
        self.variant(ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PART_DLL_DEBUG`"]
pub type PART_DLL_DEBUG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PART_DLL_DEBUG`"]
pub struct PART_DLL_DEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> PART_DLL_DEBUG_W<'a> {
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
#[doc = "Reader of field `BUS_RST`"]
pub type BUS_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUS_RST`"]
pub struct BUS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_RST_W<'a> {
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
impl R {
    #[doc = "Bit 3 - Card Interrupt Detection Test"]
    #[inline(always)]
    pub fn card_int_d3_test(&self) -> CARD_INT_D3_TEST_R {
        CARD_INT_D3_TEST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TUNING_8bit_EN"]
    #[inline(always)]
    pub fn tuning_8bit_en(&self) -> TUNING_8BIT_EN_R {
        TUNING_8BIT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TUNING_1bit_EN"]
    #[inline(always)]
    pub fn tuning_1bit_en(&self) -> TUNING_1BIT_EN_R {
        TUNING_1BIT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TUNING_CMD_EN"]
    #[inline(always)]
    pub fn tuning_cmd_en(&self) -> TUNING_CMD_EN_R {
        TUNING_CMD_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Argument2 register enable for ACMD23"]
    #[inline(always)]
    pub fn acmd23_argu2_en(&self) -> ACMD23_ARGU2_EN_R {
        ACMD23_ARGU2_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - debug for part dll"]
    #[inline(always)]
    pub fn part_dll_debug(&self) -> PART_DLL_DEBUG_R {
        PART_DLL_DEBUG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - BUS reset"]
    #[inline(always)]
    pub fn bus_rst(&self) -> BUS_RST_R {
        BUS_RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Card Interrupt Detection Test"]
    #[inline(always)]
    pub fn card_int_d3_test(&mut self) -> CARD_INT_D3_TEST_W {
        CARD_INT_D3_TEST_W { w: self }
    }
    #[doc = "Bit 4 - TUNING_8bit_EN"]
    #[inline(always)]
    pub fn tuning_8bit_en(&mut self) -> TUNING_8BIT_EN_W {
        TUNING_8BIT_EN_W { w: self }
    }
    #[doc = "Bit 5 - TUNING_1bit_EN"]
    #[inline(always)]
    pub fn tuning_1bit_en(&mut self) -> TUNING_1BIT_EN_W {
        TUNING_1BIT_EN_W { w: self }
    }
    #[doc = "Bit 6 - TUNING_CMD_EN"]
    #[inline(always)]
    pub fn tuning_cmd_en(&mut self) -> TUNING_CMD_EN_W {
        TUNING_CMD_EN_W { w: self }
    }
    #[doc = "Bit 12 - Argument2 register enable for ACMD23"]
    #[inline(always)]
    pub fn acmd23_argu2_en(&mut self) -> ACMD23_ARGU2_EN_W {
        ACMD23_ARGU2_EN_W { w: self }
    }
    #[doc = "Bit 13 - debug for part dll"]
    #[inline(always)]
    pub fn part_dll_debug(&mut self) -> PART_DLL_DEBUG_W {
        PART_DLL_DEBUG_W { w: self }
    }
    #[doc = "Bit 14 - BUS reset"]
    #[inline(always)]
    pub fn bus_rst(&mut self) -> BUS_RST_W {
        BUS_RST_W { w: self }
    }
}
