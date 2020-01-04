#[doc = "Reader of register CSICR3"]
pub type R = crate::R<u32, super::CSICR3>;
#[doc = "Writer for register CSICR3"]
pub type W = crate::W<u32, super::CSICR3>;
#[doc = "Register CSICR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSICR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Automatic Error Correction Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC_AUTO_EN_A {
    #[doc = "0: Auto Error correction is disabled."]
    ECC_AUTO_EN_0 = 0,
    #[doc = "1: Auto Error correction is enabled."]
    ECC_AUTO_EN_1 = 1,
}
impl From<ECC_AUTO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ECC_AUTO_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECC_AUTO_EN`"]
pub type ECC_AUTO_EN_R = crate::R<bool, ECC_AUTO_EN_A>;
impl ECC_AUTO_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECC_AUTO_EN_A {
        match self.bits {
            false => ECC_AUTO_EN_A::ECC_AUTO_EN_0,
            true => ECC_AUTO_EN_A::ECC_AUTO_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECC_AUTO_EN_0`"]
    #[inline(always)]
    pub fn is_ecc_auto_en_0(&self) -> bool {
        *self == ECC_AUTO_EN_A::ECC_AUTO_EN_0
    }
    #[doc = "Checks if the value of the field is `ECC_AUTO_EN_1`"]
    #[inline(always)]
    pub fn is_ecc_auto_en_1(&self) -> bool {
        *self == ECC_AUTO_EN_A::ECC_AUTO_EN_1
    }
}
#[doc = "Write proxy for field `ECC_AUTO_EN`"]
pub struct ECC_AUTO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_AUTO_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECC_AUTO_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Auto Error correction is disabled."]
    #[inline(always)]
    pub fn ecc_auto_en_0(self) -> &'a mut W {
        self.variant(ECC_AUTO_EN_A::ECC_AUTO_EN_0)
    }
    #[doc = "Auto Error correction is enabled."]
    #[inline(always)]
    pub fn ecc_auto_en_1(self) -> &'a mut W {
        self.variant(ECC_AUTO_EN_A::ECC_AUTO_EN_1)
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
#[doc = "Error Detection Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC_INT_EN_A {
    #[doc = "0: No interrupt is generated when error is detected. Only the status bit ECC_INT is set."]
    ECC_INT_EN_0 = 0,
    #[doc = "1: Interrupt is generated when error is detected."]
    ECC_INT_EN_1 = 1,
}
impl From<ECC_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ECC_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECC_INT_EN`"]
pub type ECC_INT_EN_R = crate::R<bool, ECC_INT_EN_A>;
impl ECC_INT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECC_INT_EN_A {
        match self.bits {
            false => ECC_INT_EN_A::ECC_INT_EN_0,
            true => ECC_INT_EN_A::ECC_INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECC_INT_EN_0`"]
    #[inline(always)]
    pub fn is_ecc_int_en_0(&self) -> bool {
        *self == ECC_INT_EN_A::ECC_INT_EN_0
    }
    #[doc = "Checks if the value of the field is `ECC_INT_EN_1`"]
    #[inline(always)]
    pub fn is_ecc_int_en_1(&self) -> bool {
        *self == ECC_INT_EN_A::ECC_INT_EN_1
    }
}
#[doc = "Write proxy for field `ECC_INT_EN`"]
pub struct ECC_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECC_INT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt is generated when error is detected. Only the status bit ECC_INT is set."]
    #[inline(always)]
    pub fn ecc_int_en_0(self) -> &'a mut W {
        self.variant(ECC_INT_EN_A::ECC_INT_EN_0)
    }
    #[doc = "Interrupt is generated when error is detected."]
    #[inline(always)]
    pub fn ecc_int_en_1(self) -> &'a mut W {
        self.variant(ECC_INT_EN_A::ECC_INT_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Dummy Zero Packing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZERO_PACK_EN_A {
    #[doc = "0: Zero packing disabled"]
    ZERO_PACK_EN_0 = 0,
    #[doc = "1: Zero packing enabled"]
    ZERO_PACK_EN_1 = 1,
}
impl From<ZERO_PACK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ZERO_PACK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ZERO_PACK_EN`"]
pub type ZERO_PACK_EN_R = crate::R<bool, ZERO_PACK_EN_A>;
impl ZERO_PACK_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZERO_PACK_EN_A {
        match self.bits {
            false => ZERO_PACK_EN_A::ZERO_PACK_EN_0,
            true => ZERO_PACK_EN_A::ZERO_PACK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO_PACK_EN_0`"]
    #[inline(always)]
    pub fn is_zero_pack_en_0(&self) -> bool {
        *self == ZERO_PACK_EN_A::ZERO_PACK_EN_0
    }
    #[doc = "Checks if the value of the field is `ZERO_PACK_EN_1`"]
    #[inline(always)]
    pub fn is_zero_pack_en_1(&self) -> bool {
        *self == ZERO_PACK_EN_A::ZERO_PACK_EN_1
    }
}
#[doc = "Write proxy for field `ZERO_PACK_EN`"]
pub struct ZERO_PACK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZERO_PACK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZERO_PACK_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Zero packing disabled"]
    #[inline(always)]
    pub fn zero_pack_en_0(self) -> &'a mut W {
        self.variant(ZERO_PACK_EN_A::ZERO_PACK_EN_0)
    }
    #[doc = "Zero packing enabled"]
    #[inline(always)]
    pub fn zero_pack_en_1(self) -> &'a mut W {
        self.variant(ZERO_PACK_EN_A::ZERO_PACK_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Two 8-bit Sensor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWO_8BIT_SENSOR_A {
    #[doc = "0: Only one sensor is connected."]
    TWO_8BIT_SENSOR_0 = 0,
    #[doc = "1: Two 8-bit sensors are connected or one 16-bit sensor is connected."]
    TWO_8BIT_SENSOR_1 = 1,
}
impl From<TWO_8BIT_SENSOR_A> for bool {
    #[inline(always)]
    fn from(variant: TWO_8BIT_SENSOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TWO_8BIT_SENSOR`"]
pub type TWO_8BIT_SENSOR_R = crate::R<bool, TWO_8BIT_SENSOR_A>;
impl TWO_8BIT_SENSOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWO_8BIT_SENSOR_A {
        match self.bits {
            false => TWO_8BIT_SENSOR_A::TWO_8BIT_SENSOR_0,
            true => TWO_8BIT_SENSOR_A::TWO_8BIT_SENSOR_1,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_8BIT_SENSOR_0`"]
    #[inline(always)]
    pub fn is_two_8bit_sensor_0(&self) -> bool {
        *self == TWO_8BIT_SENSOR_A::TWO_8BIT_SENSOR_0
    }
    #[doc = "Checks if the value of the field is `TWO_8BIT_SENSOR_1`"]
    #[inline(always)]
    pub fn is_two_8bit_sensor_1(&self) -> bool {
        *self == TWO_8BIT_SENSOR_A::TWO_8BIT_SENSOR_1
    }
}
#[doc = "Write proxy for field `TWO_8BIT_SENSOR`"]
pub struct TWO_8BIT_SENSOR_W<'a> {
    w: &'a mut W,
}
impl<'a> TWO_8BIT_SENSOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWO_8BIT_SENSOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only one sensor is connected."]
    #[inline(always)]
    pub fn two_8bit_sensor_0(self) -> &'a mut W {
        self.variant(TWO_8BIT_SENSOR_A::TWO_8BIT_SENSOR_0)
    }
    #[doc = "Two 8-bit sensors are connected or one 16-bit sensor is connected."]
    #[inline(always)]
    pub fn two_8bit_sensor_1(self) -> &'a mut W {
        self.variant(TWO_8BIT_SENSOR_A::TWO_8BIT_SENSOR_1)
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
#[doc = "RxFIFO Full Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXFF_LEVEL_A {
    #[doc = "0: 4 Double words"]
    RXFF_LEVEL_0 = 0,
    #[doc = "1: 8 Double words"]
    RXFF_LEVEL_1 = 1,
    #[doc = "2: 16 Double words"]
    RXFF_LEVEL_2 = 2,
    #[doc = "3: 24 Double words"]
    RXFF_LEVEL_3 = 3,
    #[doc = "4: 32 Double words"]
    RXFF_LEVEL_4 = 4,
    #[doc = "5: 48 Double words"]
    RXFF_LEVEL_5 = 5,
    #[doc = "6: 64 Double words"]
    RXFF_LEVEL_6 = 6,
    #[doc = "7: 96 Double words"]
    RXFF_LEVEL_7 = 7,
}
impl From<RXFF_LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFF_LEVEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RxFF_LEVEL`"]
pub type RXFF_LEVEL_R = crate::R<u8, RXFF_LEVEL_A>;
impl RXFF_LEVEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFF_LEVEL_A {
        match self.bits {
            0 => RXFF_LEVEL_A::RXFF_LEVEL_0,
            1 => RXFF_LEVEL_A::RXFF_LEVEL_1,
            2 => RXFF_LEVEL_A::RXFF_LEVEL_2,
            3 => RXFF_LEVEL_A::RXFF_LEVEL_3,
            4 => RXFF_LEVEL_A::RXFF_LEVEL_4,
            5 => RXFF_LEVEL_A::RXFF_LEVEL_5,
            6 => RXFF_LEVEL_A::RXFF_LEVEL_6,
            7 => RXFF_LEVEL_A::RXFF_LEVEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_0`"]
    #[inline(always)]
    pub fn is_rx_ff_level_0(&self) -> bool {
        *self == RXFF_LEVEL_A::RXFF_LEVEL_0
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_1`"]
    #[inline(always)]
    pub fn is_rx_ff_level_1(&self) -> bool {
        *self == RXFF_LEVEL_A::RXFF_LEVEL_1
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_2`"]
    #[inline(always)]
    pub fn is_rx_ff_level_2(&self) -> bool {
        *self == RXFF_LEVEL_A::RXFF_LEVEL_2
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_3`"]
    #[inline(always)]
    pub fn is_rx_ff_level_3(&self) -> bool {
        *self == RXFF_LEVEL_A::RXFF_LEVEL_3
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_4`"]
    #[inline(always)]
    pub fn is_rx_ff_level_4(&self) -> bool {
        *self == RXFF_LEVEL_A::RXFF_LEVEL_4
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_5`"]
    #[inline(always)]
    pub fn is_rx_ff_level_5(&self) -> bool {
        *self == RXFF_LEVEL_A::RXFF_LEVEL_5
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_6`"]
    #[inline(always)]
    pub fn is_rx_ff_level_6(&self) -> bool {
        *self == RXFF_LEVEL_A::RXFF_LEVEL_6
    }
    #[doc = "Checks if the value of the field is `RXFF_LEVEL_7`"]
    #[inline(always)]
    pub fn is_rx_ff_level_7(&self) -> bool {
        *self == RXFF_LEVEL_A::RXFF_LEVEL_7
    }
}
#[doc = "Write proxy for field `RxFF_LEVEL`"]
pub struct RXFF_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFF_LEVEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFF_LEVEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4 Double words"]
    #[inline(always)]
    pub fn rx_ff_level_0(self) -> &'a mut W {
        self.variant(RXFF_LEVEL_A::RXFF_LEVEL_0)
    }
    #[doc = "8 Double words"]
    #[inline(always)]
    pub fn rx_ff_level_1(self) -> &'a mut W {
        self.variant(RXFF_LEVEL_A::RXFF_LEVEL_1)
    }
    #[doc = "16 Double words"]
    #[inline(always)]
    pub fn rx_ff_level_2(self) -> &'a mut W {
        self.variant(RXFF_LEVEL_A::RXFF_LEVEL_2)
    }
    #[doc = "24 Double words"]
    #[inline(always)]
    pub fn rx_ff_level_3(self) -> &'a mut W {
        self.variant(RXFF_LEVEL_A::RXFF_LEVEL_3)
    }
    #[doc = "32 Double words"]
    #[inline(always)]
    pub fn rx_ff_level_4(self) -> &'a mut W {
        self.variant(RXFF_LEVEL_A::RXFF_LEVEL_4)
    }
    #[doc = "48 Double words"]
    #[inline(always)]
    pub fn rx_ff_level_5(self) -> &'a mut W {
        self.variant(RXFF_LEVEL_A::RXFF_LEVEL_5)
    }
    #[doc = "64 Double words"]
    #[inline(always)]
    pub fn rx_ff_level_6(self) -> &'a mut W {
        self.variant(RXFF_LEVEL_A::RXFF_LEVEL_6)
    }
    #[doc = "96 Double words"]
    #[inline(always)]
    pub fn rx_ff_level_7(self) -> &'a mut W {
        self.variant(RXFF_LEVEL_A::RXFF_LEVEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Hresponse Error Enable. This bit enables the hresponse error interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRESP_ERR_EN_A {
    #[doc = "0: Disable hresponse error interrupt"]
    HRESP_ERR_EN_0 = 0,
    #[doc = "1: Enable hresponse error interrupt"]
    HRESP_ERR_EN_1 = 1,
}
impl From<HRESP_ERR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HRESP_ERR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRESP_ERR_EN`"]
pub type HRESP_ERR_EN_R = crate::R<bool, HRESP_ERR_EN_A>;
impl HRESP_ERR_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRESP_ERR_EN_A {
        match self.bits {
            false => HRESP_ERR_EN_A::HRESP_ERR_EN_0,
            true => HRESP_ERR_EN_A::HRESP_ERR_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRESP_ERR_EN_0`"]
    #[inline(always)]
    pub fn is_hresp_err_en_0(&self) -> bool {
        *self == HRESP_ERR_EN_A::HRESP_ERR_EN_0
    }
    #[doc = "Checks if the value of the field is `HRESP_ERR_EN_1`"]
    #[inline(always)]
    pub fn is_hresp_err_en_1(&self) -> bool {
        *self == HRESP_ERR_EN_A::HRESP_ERR_EN_1
    }
}
#[doc = "Write proxy for field `HRESP_ERR_EN`"]
pub struct HRESP_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HRESP_ERR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRESP_ERR_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable hresponse error interrupt"]
    #[inline(always)]
    pub fn hresp_err_en_0(self) -> &'a mut W {
        self.variant(HRESP_ERR_EN_A::HRESP_ERR_EN_0)
    }
    #[doc = "Enable hresponse error interrupt"]
    #[inline(always)]
    pub fn hresp_err_en_1(self) -> &'a mut W {
        self.variant(HRESP_ERR_EN_A::HRESP_ERR_EN_1)
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
#[doc = "STATFIFO Full Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATFF_LEVEL_A {
    #[doc = "0: 4 Double words"]
    STATFF_LEVEL_0 = 0,
    #[doc = "1: 8 Double words"]
    STATFF_LEVEL_1 = 1,
    #[doc = "2: 12 Double words"]
    STATFF_LEVEL_2 = 2,
    #[doc = "3: 16 Double words"]
    STATFF_LEVEL_3 = 3,
    #[doc = "4: 24 Double words"]
    STATFF_LEVEL_4 = 4,
    #[doc = "5: 32 Double words"]
    STATFF_LEVEL_5 = 5,
    #[doc = "6: 48 Double words"]
    STATFF_LEVEL_6 = 6,
    #[doc = "7: 64 Double words"]
    STATFF_LEVEL_7 = 7,
}
impl From<STATFF_LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: STATFF_LEVEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATFF_LEVEL`"]
pub type STATFF_LEVEL_R = crate::R<u8, STATFF_LEVEL_A>;
impl STATFF_LEVEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATFF_LEVEL_A {
        match self.bits {
            0 => STATFF_LEVEL_A::STATFF_LEVEL_0,
            1 => STATFF_LEVEL_A::STATFF_LEVEL_1,
            2 => STATFF_LEVEL_A::STATFF_LEVEL_2,
            3 => STATFF_LEVEL_A::STATFF_LEVEL_3,
            4 => STATFF_LEVEL_A::STATFF_LEVEL_4,
            5 => STATFF_LEVEL_A::STATFF_LEVEL_5,
            6 => STATFF_LEVEL_A::STATFF_LEVEL_6,
            7 => STATFF_LEVEL_A::STATFF_LEVEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_0`"]
    #[inline(always)]
    pub fn is_statff_level_0(&self) -> bool {
        *self == STATFF_LEVEL_A::STATFF_LEVEL_0
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_1`"]
    #[inline(always)]
    pub fn is_statff_level_1(&self) -> bool {
        *self == STATFF_LEVEL_A::STATFF_LEVEL_1
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_2`"]
    #[inline(always)]
    pub fn is_statff_level_2(&self) -> bool {
        *self == STATFF_LEVEL_A::STATFF_LEVEL_2
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_3`"]
    #[inline(always)]
    pub fn is_statff_level_3(&self) -> bool {
        *self == STATFF_LEVEL_A::STATFF_LEVEL_3
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_4`"]
    #[inline(always)]
    pub fn is_statff_level_4(&self) -> bool {
        *self == STATFF_LEVEL_A::STATFF_LEVEL_4
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_5`"]
    #[inline(always)]
    pub fn is_statff_level_5(&self) -> bool {
        *self == STATFF_LEVEL_A::STATFF_LEVEL_5
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_6`"]
    #[inline(always)]
    pub fn is_statff_level_6(&self) -> bool {
        *self == STATFF_LEVEL_A::STATFF_LEVEL_6
    }
    #[doc = "Checks if the value of the field is `STATFF_LEVEL_7`"]
    #[inline(always)]
    pub fn is_statff_level_7(&self) -> bool {
        *self == STATFF_LEVEL_A::STATFF_LEVEL_7
    }
}
#[doc = "Write proxy for field `STATFF_LEVEL`"]
pub struct STATFF_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STATFF_LEVEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATFF_LEVEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4 Double words"]
    #[inline(always)]
    pub fn statff_level_0(self) -> &'a mut W {
        self.variant(STATFF_LEVEL_A::STATFF_LEVEL_0)
    }
    #[doc = "8 Double words"]
    #[inline(always)]
    pub fn statff_level_1(self) -> &'a mut W {
        self.variant(STATFF_LEVEL_A::STATFF_LEVEL_1)
    }
    #[doc = "12 Double words"]
    #[inline(always)]
    pub fn statff_level_2(self) -> &'a mut W {
        self.variant(STATFF_LEVEL_A::STATFF_LEVEL_2)
    }
    #[doc = "16 Double words"]
    #[inline(always)]
    pub fn statff_level_3(self) -> &'a mut W {
        self.variant(STATFF_LEVEL_A::STATFF_LEVEL_3)
    }
    #[doc = "24 Double words"]
    #[inline(always)]
    pub fn statff_level_4(self) -> &'a mut W {
        self.variant(STATFF_LEVEL_A::STATFF_LEVEL_4)
    }
    #[doc = "32 Double words"]
    #[inline(always)]
    pub fn statff_level_5(self) -> &'a mut W {
        self.variant(STATFF_LEVEL_A::STATFF_LEVEL_5)
    }
    #[doc = "48 Double words"]
    #[inline(always)]
    pub fn statff_level_6(self) -> &'a mut W {
        self.variant(STATFF_LEVEL_A::STATFF_LEVEL_6)
    }
    #[doc = "64 Double words"]
    #[inline(always)]
    pub fn statff_level_7(self) -> &'a mut W {
        self.variant(STATFF_LEVEL_A::STATFF_LEVEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "DMA Request Enable for STATFIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_REQ_EN_SFF_A {
    #[doc = "0: Disable the dma request"]
    DMA_REQ_EN_SFF_0 = 0,
    #[doc = "1: Enable the dma request"]
    DMA_REQ_EN_SFF_1 = 1,
}
impl From<DMA_REQ_EN_SFF_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_REQ_EN_SFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_REQ_EN_SFF`"]
pub type DMA_REQ_EN_SFF_R = crate::R<bool, DMA_REQ_EN_SFF_A>;
impl DMA_REQ_EN_SFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_REQ_EN_SFF_A {
        match self.bits {
            false => DMA_REQ_EN_SFF_A::DMA_REQ_EN_SFF_0,
            true => DMA_REQ_EN_SFF_A::DMA_REQ_EN_SFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_REQ_EN_SFF_0`"]
    #[inline(always)]
    pub fn is_dma_req_en_sff_0(&self) -> bool {
        *self == DMA_REQ_EN_SFF_A::DMA_REQ_EN_SFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_REQ_EN_SFF_1`"]
    #[inline(always)]
    pub fn is_dma_req_en_sff_1(&self) -> bool {
        *self == DMA_REQ_EN_SFF_A::DMA_REQ_EN_SFF_1
    }
}
#[doc = "Write proxy for field `DMA_REQ_EN_SFF`"]
pub struct DMA_REQ_EN_SFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQ_EN_SFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_REQ_EN_SFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the dma request"]
    #[inline(always)]
    pub fn dma_req_en_sff_0(self) -> &'a mut W {
        self.variant(DMA_REQ_EN_SFF_A::DMA_REQ_EN_SFF_0)
    }
    #[doc = "Enable the dma request"]
    #[inline(always)]
    pub fn dma_req_en_sff_1(self) -> &'a mut W {
        self.variant(DMA_REQ_EN_SFF_A::DMA_REQ_EN_SFF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "DMA Request Enable for RxFIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_REQ_EN_RFF_A {
    #[doc = "0: Disable the dma request"]
    DMA_REQ_EN_RFF_0 = 0,
    #[doc = "1: Enable the dma request"]
    DMA_REQ_EN_RFF_1 = 1,
}
impl From<DMA_REQ_EN_RFF_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_REQ_EN_RFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_REQ_EN_RFF`"]
pub type DMA_REQ_EN_RFF_R = crate::R<bool, DMA_REQ_EN_RFF_A>;
impl DMA_REQ_EN_RFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_REQ_EN_RFF_A {
        match self.bits {
            false => DMA_REQ_EN_RFF_A::DMA_REQ_EN_RFF_0,
            true => DMA_REQ_EN_RFF_A::DMA_REQ_EN_RFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_REQ_EN_RFF_0`"]
    #[inline(always)]
    pub fn is_dma_req_en_rff_0(&self) -> bool {
        *self == DMA_REQ_EN_RFF_A::DMA_REQ_EN_RFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_REQ_EN_RFF_1`"]
    #[inline(always)]
    pub fn is_dma_req_en_rff_1(&self) -> bool {
        *self == DMA_REQ_EN_RFF_A::DMA_REQ_EN_RFF_1
    }
}
#[doc = "Write proxy for field `DMA_REQ_EN_RFF`"]
pub struct DMA_REQ_EN_RFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REQ_EN_RFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_REQ_EN_RFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the dma request"]
    #[inline(always)]
    pub fn dma_req_en_rff_0(self) -> &'a mut W {
        self.variant(DMA_REQ_EN_RFF_A::DMA_REQ_EN_RFF_0)
    }
    #[doc = "Enable the dma request"]
    #[inline(always)]
    pub fn dma_req_en_rff_1(self) -> &'a mut W {
        self.variant(DMA_REQ_EN_RFF_A::DMA_REQ_EN_RFF_1)
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
#[doc = "Reflash DMA Controller for STATFIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_REFLASH_SFF_A {
    #[doc = "0: No reflashing"]
    DMA_REFLASH_SFF_0 = 0,
    #[doc = "1: Reflash the embedded DMA controller"]
    DMA_REFLASH_SFF_1 = 1,
}
impl From<DMA_REFLASH_SFF_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_REFLASH_SFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_REFLASH_SFF`"]
pub type DMA_REFLASH_SFF_R = crate::R<bool, DMA_REFLASH_SFF_A>;
impl DMA_REFLASH_SFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_REFLASH_SFF_A {
        match self.bits {
            false => DMA_REFLASH_SFF_A::DMA_REFLASH_SFF_0,
            true => DMA_REFLASH_SFF_A::DMA_REFLASH_SFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_REFLASH_SFF_0`"]
    #[inline(always)]
    pub fn is_dma_reflash_sff_0(&self) -> bool {
        *self == DMA_REFLASH_SFF_A::DMA_REFLASH_SFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_REFLASH_SFF_1`"]
    #[inline(always)]
    pub fn is_dma_reflash_sff_1(&self) -> bool {
        *self == DMA_REFLASH_SFF_A::DMA_REFLASH_SFF_1
    }
}
#[doc = "Write proxy for field `DMA_REFLASH_SFF`"]
pub struct DMA_REFLASH_SFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REFLASH_SFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_REFLASH_SFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reflashing"]
    #[inline(always)]
    pub fn dma_reflash_sff_0(self) -> &'a mut W {
        self.variant(DMA_REFLASH_SFF_A::DMA_REFLASH_SFF_0)
    }
    #[doc = "Reflash the embedded DMA controller"]
    #[inline(always)]
    pub fn dma_reflash_sff_1(self) -> &'a mut W {
        self.variant(DMA_REFLASH_SFF_A::DMA_REFLASH_SFF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reflash DMA Controller for RxFIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_REFLASH_RFF_A {
    #[doc = "0: No reflashing"]
    DMA_REFLASH_RFF_0 = 0,
    #[doc = "1: Reflash the embedded DMA controller"]
    DMA_REFLASH_RFF_1 = 1,
}
impl From<DMA_REFLASH_RFF_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_REFLASH_RFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_REFLASH_RFF`"]
pub type DMA_REFLASH_RFF_R = crate::R<bool, DMA_REFLASH_RFF_A>;
impl DMA_REFLASH_RFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_REFLASH_RFF_A {
        match self.bits {
            false => DMA_REFLASH_RFF_A::DMA_REFLASH_RFF_0,
            true => DMA_REFLASH_RFF_A::DMA_REFLASH_RFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_REFLASH_RFF_0`"]
    #[inline(always)]
    pub fn is_dma_reflash_rff_0(&self) -> bool {
        *self == DMA_REFLASH_RFF_A::DMA_REFLASH_RFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_REFLASH_RFF_1`"]
    #[inline(always)]
    pub fn is_dma_reflash_rff_1(&self) -> bool {
        *self == DMA_REFLASH_RFF_A::DMA_REFLASH_RFF_1
    }
}
#[doc = "Write proxy for field `DMA_REFLASH_RFF`"]
pub struct DMA_REFLASH_RFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_REFLASH_RFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_REFLASH_RFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reflashing"]
    #[inline(always)]
    pub fn dma_reflash_rff_0(self) -> &'a mut W {
        self.variant(DMA_REFLASH_RFF_A::DMA_REFLASH_RFF_0)
    }
    #[doc = "Reflash the embedded DMA controller"]
    #[inline(always)]
    pub fn dma_reflash_rff_1(self) -> &'a mut W {
        self.variant(DMA_REFLASH_RFF_A::DMA_REFLASH_RFF_1)
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
#[doc = "Frame Count Reset. Resets the Frame Counter. (Cleared automatically after reset is done)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRMCNT_RST_A {
    #[doc = "0: Do not reset"]
    FRMCNT_RST_0 = 0,
    #[doc = "1: Reset frame counter immediately"]
    FRMCNT_RST_1 = 1,
}
impl From<FRMCNT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FRMCNT_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRMCNT_RST`"]
pub type FRMCNT_RST_R = crate::R<bool, FRMCNT_RST_A>;
impl FRMCNT_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRMCNT_RST_A {
        match self.bits {
            false => FRMCNT_RST_A::FRMCNT_RST_0,
            true => FRMCNT_RST_A::FRMCNT_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRMCNT_RST_0`"]
    #[inline(always)]
    pub fn is_frmcnt_rst_0(&self) -> bool {
        *self == FRMCNT_RST_A::FRMCNT_RST_0
    }
    #[doc = "Checks if the value of the field is `FRMCNT_RST_1`"]
    #[inline(always)]
    pub fn is_frmcnt_rst_1(&self) -> bool {
        *self == FRMCNT_RST_A::FRMCNT_RST_1
    }
}
#[doc = "Write proxy for field `FRMCNT_RST`"]
pub struct FRMCNT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMCNT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRMCNT_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not reset"]
    #[inline(always)]
    pub fn frmcnt_rst_0(self) -> &'a mut W {
        self.variant(FRMCNT_RST_A::FRMCNT_RST_0)
    }
    #[doc = "Reset frame counter immediately"]
    #[inline(always)]
    pub fn frmcnt_rst_1(self) -> &'a mut W {
        self.variant(FRMCNT_RST_A::FRMCNT_RST_1)
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
#[doc = "Reader of field `FRMCNT`"]
pub type FRMCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRMCNT`"]
pub struct FRMCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Automatic Error Correction Enable"]
    #[inline(always)]
    pub fn ecc_auto_en(&self) -> ECC_AUTO_EN_R {
        ECC_AUTO_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Error Detection Interrupt Enable"]
    #[inline(always)]
    pub fn ecc_int_en(&self) -> ECC_INT_EN_R {
        ECC_INT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Dummy Zero Packing Enable"]
    #[inline(always)]
    pub fn zero_pack_en(&self) -> ZERO_PACK_EN_R {
        ZERO_PACK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Two 8-bit Sensor Mode"]
    #[inline(always)]
    pub fn two_8bit_sensor(&self) -> TWO_8BIT_SENSOR_R {
        TWO_8BIT_SENSOR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - RxFIFO Full Level"]
    #[inline(always)]
    pub fn rx_ff_level(&self) -> RXFF_LEVEL_R {
        RXFF_LEVEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Hresponse Error Enable. This bit enables the hresponse error interrupt."]
    #[inline(always)]
    pub fn hresp_err_en(&self) -> HRESP_ERR_EN_R {
        HRESP_ERR_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - STATFIFO Full Level"]
    #[inline(always)]
    pub fn statff_level(&self) -> STATFF_LEVEL_R {
        STATFF_LEVEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - DMA Request Enable for STATFIFO"]
    #[inline(always)]
    pub fn dma_req_en_sff(&self) -> DMA_REQ_EN_SFF_R {
        DMA_REQ_EN_SFF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DMA Request Enable for RxFIFO"]
    #[inline(always)]
    pub fn dma_req_en_rff(&self) -> DMA_REQ_EN_RFF_R {
        DMA_REQ_EN_RFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reflash DMA Controller for STATFIFO"]
    #[inline(always)]
    pub fn dma_reflash_sff(&self) -> DMA_REFLASH_SFF_R {
        DMA_REFLASH_SFF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Reflash DMA Controller for RxFIFO"]
    #[inline(always)]
    pub fn dma_reflash_rff(&self) -> DMA_REFLASH_RFF_R {
        DMA_REFLASH_RFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Frame Count Reset. Resets the Frame Counter. (Cleared automatically after reset is done)"]
    #[inline(always)]
    pub fn frmcnt_rst(&self) -> FRMCNT_RST_R {
        FRMCNT_RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Frame Counter"]
    #[inline(always)]
    pub fn frmcnt(&self) -> FRMCNT_R {
        FRMCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Automatic Error Correction Enable"]
    #[inline(always)]
    pub fn ecc_auto_en(&mut self) -> ECC_AUTO_EN_W {
        ECC_AUTO_EN_W { w: self }
    }
    #[doc = "Bit 1 - Error Detection Interrupt Enable"]
    #[inline(always)]
    pub fn ecc_int_en(&mut self) -> ECC_INT_EN_W {
        ECC_INT_EN_W { w: self }
    }
    #[doc = "Bit 2 - Dummy Zero Packing Enable"]
    #[inline(always)]
    pub fn zero_pack_en(&mut self) -> ZERO_PACK_EN_W {
        ZERO_PACK_EN_W { w: self }
    }
    #[doc = "Bit 3 - Two 8-bit Sensor Mode"]
    #[inline(always)]
    pub fn two_8bit_sensor(&mut self) -> TWO_8BIT_SENSOR_W {
        TWO_8BIT_SENSOR_W { w: self }
    }
    #[doc = "Bits 4:6 - RxFIFO Full Level"]
    #[inline(always)]
    pub fn rx_ff_level(&mut self) -> RXFF_LEVEL_W {
        RXFF_LEVEL_W { w: self }
    }
    #[doc = "Bit 7 - Hresponse Error Enable. This bit enables the hresponse error interrupt."]
    #[inline(always)]
    pub fn hresp_err_en(&mut self) -> HRESP_ERR_EN_W {
        HRESP_ERR_EN_W { w: self }
    }
    #[doc = "Bits 8:10 - STATFIFO Full Level"]
    #[inline(always)]
    pub fn statff_level(&mut self) -> STATFF_LEVEL_W {
        STATFF_LEVEL_W { w: self }
    }
    #[doc = "Bit 11 - DMA Request Enable for STATFIFO"]
    #[inline(always)]
    pub fn dma_req_en_sff(&mut self) -> DMA_REQ_EN_SFF_W {
        DMA_REQ_EN_SFF_W { w: self }
    }
    #[doc = "Bit 12 - DMA Request Enable for RxFIFO"]
    #[inline(always)]
    pub fn dma_req_en_rff(&mut self) -> DMA_REQ_EN_RFF_W {
        DMA_REQ_EN_RFF_W { w: self }
    }
    #[doc = "Bit 13 - Reflash DMA Controller for STATFIFO"]
    #[inline(always)]
    pub fn dma_reflash_sff(&mut self) -> DMA_REFLASH_SFF_W {
        DMA_REFLASH_SFF_W { w: self }
    }
    #[doc = "Bit 14 - Reflash DMA Controller for RxFIFO"]
    #[inline(always)]
    pub fn dma_reflash_rff(&mut self) -> DMA_REFLASH_RFF_W {
        DMA_REFLASH_RFF_W { w: self }
    }
    #[doc = "Bit 15 - Frame Count Reset. Resets the Frame Counter. (Cleared automatically after reset is done)"]
    #[inline(always)]
    pub fn frmcnt_rst(&mut self) -> FRMCNT_RST_W {
        FRMCNT_RST_W { w: self }
    }
    #[doc = "Bits 16:31 - Frame Counter"]
    #[inline(always)]
    pub fn frmcnt(&mut self) -> FRMCNT_W {
        FRMCNT_W { w: self }
    }
}
