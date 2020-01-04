#[doc = "Reader of register CSICR1"]
pub type R = crate::R<u32, super::CSICR1>;
#[doc = "Writer for register CSICR1"]
pub type W = crate::W<u32, super::CSICR1>;
#[doc = "Register CSICR1 `reset()`'s with value 0x4000_0800"]
impl crate::ResetValue for super::CSICR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000_0800
    }
}
#[doc = "Pixel Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIXEL_BIT_A {
    #[doc = "0: 8-bit data for each pixel"]
    PIXEL_BIT_0 = 0,
    #[doc = "1: 10-bit data for each pixel"]
    PIXEL_BIT_1 = 1,
}
impl From<PIXEL_BIT_A> for bool {
    #[inline(always)]
    fn from(variant: PIXEL_BIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIXEL_BIT`"]
pub type PIXEL_BIT_R = crate::R<bool, PIXEL_BIT_A>;
impl PIXEL_BIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIXEL_BIT_A {
        match self.bits {
            false => PIXEL_BIT_A::PIXEL_BIT_0,
            true => PIXEL_BIT_A::PIXEL_BIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIXEL_BIT_0`"]
    #[inline(always)]
    pub fn is_pixel_bit_0(&self) -> bool {
        *self == PIXEL_BIT_A::PIXEL_BIT_0
    }
    #[doc = "Checks if the value of the field is `PIXEL_BIT_1`"]
    #[inline(always)]
    pub fn is_pixel_bit_1(&self) -> bool {
        *self == PIXEL_BIT_A::PIXEL_BIT_1
    }
}
#[doc = "Write proxy for field `PIXEL_BIT`"]
pub struct PIXEL_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PIXEL_BIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIXEL_BIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "8-bit data for each pixel"]
    #[inline(always)]
    pub fn pixel_bit_0(self) -> &'a mut W {
        self.variant(PIXEL_BIT_A::PIXEL_BIT_0)
    }
    #[doc = "10-bit data for each pixel"]
    #[inline(always)]
    pub fn pixel_bit_1(self) -> &'a mut W {
        self.variant(PIXEL_BIT_A::PIXEL_BIT_1)
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
#[doc = "Valid Pixel Clock Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REDGE_A {
    #[doc = "0: Pixel data is latched at the falling edge of CSI_PIXCLK"]
    REDGE_0 = 0,
    #[doc = "1: Pixel data is latched at the rising edge of CSI_PIXCLK"]
    REDGE_1 = 1,
}
impl From<REDGE_A> for bool {
    #[inline(always)]
    fn from(variant: REDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REDGE`"]
pub type REDGE_R = crate::R<bool, REDGE_A>;
impl REDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REDGE_A {
        match self.bits {
            false => REDGE_A::REDGE_0,
            true => REDGE_A::REDGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `REDGE_0`"]
    #[inline(always)]
    pub fn is_redge_0(&self) -> bool {
        *self == REDGE_A::REDGE_0
    }
    #[doc = "Checks if the value of the field is `REDGE_1`"]
    #[inline(always)]
    pub fn is_redge_1(&self) -> bool {
        *self == REDGE_A::REDGE_1
    }
}
#[doc = "Write proxy for field `REDGE`"]
pub struct REDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> REDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pixel data is latched at the falling edge of CSI_PIXCLK"]
    #[inline(always)]
    pub fn redge_0(self) -> &'a mut W {
        self.variant(REDGE_A::REDGE_0)
    }
    #[doc = "Pixel data is latched at the rising edge of CSI_PIXCLK"]
    #[inline(always)]
    pub fn redge_1(self) -> &'a mut W {
        self.variant(REDGE_A::REDGE_1)
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
#[doc = "Invert Pixel Clock Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_PCLK_A {
    #[doc = "0: CSI_PIXCLK is directly applied to internal circuitry"]
    INV_PCLK_0 = 0,
    #[doc = "1: CSI_PIXCLK is inverted before applied to internal circuitry"]
    INV_PCLK_1 = 1,
}
impl From<INV_PCLK_A> for bool {
    #[inline(always)]
    fn from(variant: INV_PCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INV_PCLK`"]
pub type INV_PCLK_R = crate::R<bool, INV_PCLK_A>;
impl INV_PCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_PCLK_A {
        match self.bits {
            false => INV_PCLK_A::INV_PCLK_0,
            true => INV_PCLK_A::INV_PCLK_1,
        }
    }
    #[doc = "Checks if the value of the field is `INV_PCLK_0`"]
    #[inline(always)]
    pub fn is_inv_pclk_0(&self) -> bool {
        *self == INV_PCLK_A::INV_PCLK_0
    }
    #[doc = "Checks if the value of the field is `INV_PCLK_1`"]
    #[inline(always)]
    pub fn is_inv_pclk_1(&self) -> bool {
        *self == INV_PCLK_A::INV_PCLK_1
    }
}
#[doc = "Write proxy for field `INV_PCLK`"]
pub struct INV_PCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_PCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV_PCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CSI_PIXCLK is directly applied to internal circuitry"]
    #[inline(always)]
    pub fn inv_pclk_0(self) -> &'a mut W {
        self.variant(INV_PCLK_A::INV_PCLK_0)
    }
    #[doc = "CSI_PIXCLK is inverted before applied to internal circuitry"]
    #[inline(always)]
    pub fn inv_pclk_1(self) -> &'a mut W {
        self.variant(INV_PCLK_A::INV_PCLK_1)
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
#[doc = "Invert Data Input. This bit enables or disables internal inverters on the data lines.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV_DATA_A {
    #[doc = "0: CSI_D\\[7:0\\]
data lines are directly applied to internal circuitry"]
    INV_DATA_0 = 0,
    #[doc = "1: CSI_D\\[7:0\\]
data lines are inverted before applied to internal circuitry"]
    INV_DATA_1 = 1,
}
impl From<INV_DATA_A> for bool {
    #[inline(always)]
    fn from(variant: INV_DATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INV_DATA`"]
pub type INV_DATA_R = crate::R<bool, INV_DATA_A>;
impl INV_DATA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_DATA_A {
        match self.bits {
            false => INV_DATA_A::INV_DATA_0,
            true => INV_DATA_A::INV_DATA_1,
        }
    }
    #[doc = "Checks if the value of the field is `INV_DATA_0`"]
    #[inline(always)]
    pub fn is_inv_data_0(&self) -> bool {
        *self == INV_DATA_A::INV_DATA_0
    }
    #[doc = "Checks if the value of the field is `INV_DATA_1`"]
    #[inline(always)]
    pub fn is_inv_data_1(&self) -> bool {
        *self == INV_DATA_A::INV_DATA_1
    }
}
#[doc = "Write proxy for field `INV_DATA`"]
pub struct INV_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_DATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV_DATA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CSI_D\\[7:0\\]
data lines are directly applied to internal circuitry"]
    #[inline(always)]
    pub fn inv_data_0(self) -> &'a mut W {
        self.variant(INV_DATA_A::INV_DATA_0)
    }
    #[doc = "CSI_D\\[7:0\\]
data lines are inverted before applied to internal circuitry"]
    #[inline(always)]
    pub fn inv_data_1(self) -> &'a mut W {
        self.variant(INV_DATA_A::INV_DATA_1)
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
#[doc = "Gated Clock Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCLK_MODE_A {
    #[doc = "0: Non-gated clock mode. All incoming pixel clocks are valid. HSYNC is ignored."]
    GCLK_MODE_0 = 0,
    #[doc = "1: Gated clock mode. Pixel clock signal is valid only when HSYNC is active."]
    GCLK_MODE_1 = 1,
}
impl From<GCLK_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: GCLK_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GCLK_MODE`"]
pub type GCLK_MODE_R = crate::R<bool, GCLK_MODE_A>;
impl GCLK_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCLK_MODE_A {
        match self.bits {
            false => GCLK_MODE_A::GCLK_MODE_0,
            true => GCLK_MODE_A::GCLK_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK_MODE_0`"]
    #[inline(always)]
    pub fn is_gclk_mode_0(&self) -> bool {
        *self == GCLK_MODE_A::GCLK_MODE_0
    }
    #[doc = "Checks if the value of the field is `GCLK_MODE_1`"]
    #[inline(always)]
    pub fn is_gclk_mode_1(&self) -> bool {
        *self == GCLK_MODE_A::GCLK_MODE_1
    }
}
#[doc = "Write proxy for field `GCLK_MODE`"]
pub struct GCLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLK_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCLK_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Non-gated clock mode. All incoming pixel clocks are valid. HSYNC is ignored."]
    #[inline(always)]
    pub fn gclk_mode_0(self) -> &'a mut W {
        self.variant(GCLK_MODE_A::GCLK_MODE_0)
    }
    #[doc = "Gated clock mode. Pixel clock signal is valid only when HSYNC is active."]
    #[inline(always)]
    pub fn gclk_mode_1(self) -> &'a mut W {
        self.variant(GCLK_MODE_A::GCLK_MODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CLR_RXFIFO`"]
pub type CLR_RXFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_RXFIFO`"]
pub struct CLR_RXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_RXFIFO_W<'a> {
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
#[doc = "Reader of field `CLR_STATFIFO`"]
pub type CLR_STATFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_STATFIFO`"]
pub struct CLR_STATFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_STATFIFO_W<'a> {
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
#[doc = "Data Packing Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACK_DIR_A {
    #[doc = "0: Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xBBBBAAAA in STAT FIFO."]
    PACK_DIR_0 = 0,
    #[doc = "1: Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xAAAABBBB in STAT FIFO."]
    PACK_DIR_1 = 1,
}
impl From<PACK_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: PACK_DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PACK_DIR`"]
pub type PACK_DIR_R = crate::R<bool, PACK_DIR_A>;
impl PACK_DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PACK_DIR_A {
        match self.bits {
            false => PACK_DIR_A::PACK_DIR_0,
            true => PACK_DIR_A::PACK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PACK_DIR_0`"]
    #[inline(always)]
    pub fn is_pack_dir_0(&self) -> bool {
        *self == PACK_DIR_A::PACK_DIR_0
    }
    #[doc = "Checks if the value of the field is `PACK_DIR_1`"]
    #[inline(always)]
    pub fn is_pack_dir_1(&self) -> bool {
        *self == PACK_DIR_A::PACK_DIR_1
    }
}
#[doc = "Write proxy for field `PACK_DIR`"]
pub struct PACK_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> PACK_DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACK_DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xBBBBAAAA in STAT FIFO."]
    #[inline(always)]
    pub fn pack_dir_0(self) -> &'a mut W {
        self.variant(PACK_DIR_A::PACK_DIR_0)
    }
    #[doc = "Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xAAAABBBB in STAT FIFO."]
    #[inline(always)]
    pub fn pack_dir_1(self) -> &'a mut W {
        self.variant(PACK_DIR_A::PACK_DIR_1)
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
#[doc = "FIFO Clear Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCC_A {
    #[doc = "0: Asynchronous FIFO clear is selected."]
    FCC_0 = 0,
    #[doc = "1: Synchronous FIFO clear is selected."]
    FCC_1 = 1,
}
impl From<FCC_A> for bool {
    #[inline(always)]
    fn from(variant: FCC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FCC`"]
pub type FCC_R = crate::R<bool, FCC_A>;
impl FCC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCC_A {
        match self.bits {
            false => FCC_A::FCC_0,
            true => FCC_A::FCC_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCC_0`"]
    #[inline(always)]
    pub fn is_fcc_0(&self) -> bool {
        *self == FCC_A::FCC_0
    }
    #[doc = "Checks if the value of the field is `FCC_1`"]
    #[inline(always)]
    pub fn is_fcc_1(&self) -> bool {
        *self == FCC_A::FCC_1
    }
}
#[doc = "Write proxy for field `FCC`"]
pub struct FCC_W<'a> {
    w: &'a mut W,
}
impl<'a> FCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Asynchronous FIFO clear is selected."]
    #[inline(always)]
    pub fn fcc_0(self) -> &'a mut W {
        self.variant(FCC_A::FCC_0)
    }
    #[doc = "Synchronous FIFO clear is selected."]
    #[inline(always)]
    pub fn fcc_1(self) -> &'a mut W {
        self.variant(FCC_A::FCC_1)
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
#[doc = "CCIR656 Interface Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIR_EN_A {
    #[doc = "0: Traditional interface is selected. Timing interface logic is used to latch data."]
    CCIR_EN_0 = 0,
    #[doc = "1: CCIR656 interface is selected."]
    CCIR_EN_1 = 1,
}
impl From<CCIR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CCIR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCIR_EN`"]
pub type CCIR_EN_R = crate::R<bool, CCIR_EN_A>;
impl CCIR_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIR_EN_A {
        match self.bits {
            false => CCIR_EN_A::CCIR_EN_0,
            true => CCIR_EN_A::CCIR_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIR_EN_0`"]
    #[inline(always)]
    pub fn is_ccir_en_0(&self) -> bool {
        *self == CCIR_EN_A::CCIR_EN_0
    }
    #[doc = "Checks if the value of the field is `CCIR_EN_1`"]
    #[inline(always)]
    pub fn is_ccir_en_1(&self) -> bool {
        *self == CCIR_EN_A::CCIR_EN_1
    }
}
#[doc = "Write proxy for field `CCIR_EN`"]
pub struct CCIR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIR_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Traditional interface is selected. Timing interface logic is used to latch data."]
    #[inline(always)]
    pub fn ccir_en_0(self) -> &'a mut W {
        self.variant(CCIR_EN_A::CCIR_EN_0)
    }
    #[doc = "CCIR656 interface is selected."]
    #[inline(always)]
    pub fn ccir_en_1(self) -> &'a mut W {
        self.variant(CCIR_EN_A::CCIR_EN_1)
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
#[doc = "HSYNC Polarity Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSYNC_POL_A {
    #[doc = "0: HSYNC is active low"]
    HSYNC_POL_0 = 0,
    #[doc = "1: HSYNC is active high"]
    HSYNC_POL_1 = 1,
}
impl From<HSYNC_POL_A> for bool {
    #[inline(always)]
    fn from(variant: HSYNC_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSYNC_POL`"]
pub type HSYNC_POL_R = crate::R<bool, HSYNC_POL_A>;
impl HSYNC_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSYNC_POL_A {
        match self.bits {
            false => HSYNC_POL_A::HSYNC_POL_0,
            true => HSYNC_POL_A::HSYNC_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `HSYNC_POL_0`"]
    #[inline(always)]
    pub fn is_hsync_pol_0(&self) -> bool {
        *self == HSYNC_POL_A::HSYNC_POL_0
    }
    #[doc = "Checks if the value of the field is `HSYNC_POL_1`"]
    #[inline(always)]
    pub fn is_hsync_pol_1(&self) -> bool {
        *self == HSYNC_POL_A::HSYNC_POL_1
    }
}
#[doc = "Write proxy for field `HSYNC_POL`"]
pub struct HSYNC_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNC_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSYNC_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSYNC is active low"]
    #[inline(always)]
    pub fn hsync_pol_0(self) -> &'a mut W {
        self.variant(HSYNC_POL_A::HSYNC_POL_0)
    }
    #[doc = "HSYNC is active high"]
    #[inline(always)]
    pub fn hsync_pol_1(self) -> &'a mut W {
        self.variant(HSYNC_POL_A::HSYNC_POL_1)
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
#[doc = "Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_INTEN_A {
    #[doc = "0: SOF interrupt disable"]
    SOF_INTEN_0 = 0,
    #[doc = "1: SOF interrupt enable"]
    SOF_INTEN_1 = 1,
}
impl From<SOF_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOF_INTEN`"]
pub type SOF_INTEN_R = crate::R<bool, SOF_INTEN_A>;
impl SOF_INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_INTEN_A {
        match self.bits {
            false => SOF_INTEN_A::SOF_INTEN_0,
            true => SOF_INTEN_A::SOF_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOF_INTEN_0`"]
    #[inline(always)]
    pub fn is_sof_inten_0(&self) -> bool {
        *self == SOF_INTEN_A::SOF_INTEN_0
    }
    #[doc = "Checks if the value of the field is `SOF_INTEN_1`"]
    #[inline(always)]
    pub fn is_sof_inten_1(&self) -> bool {
        *self == SOF_INTEN_A::SOF_INTEN_1
    }
}
#[doc = "Write proxy for field `SOF_INTEN`"]
pub struct SOF_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOF_INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SOF interrupt disable"]
    #[inline(always)]
    pub fn sof_inten_0(self) -> &'a mut W {
        self.variant(SOF_INTEN_A::SOF_INTEN_0)
    }
    #[doc = "SOF interrupt enable"]
    #[inline(always)]
    pub fn sof_inten_1(self) -> &'a mut W {
        self.variant(SOF_INTEN_A::SOF_INTEN_1)
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
#[doc = "SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_POL_A {
    #[doc = "0: SOF interrupt is generated on SOF falling edge"]
    SOF_POL_0 = 0,
    #[doc = "1: SOF interrupt is generated on SOF rising edge"]
    SOF_POL_1 = 1,
}
impl From<SOF_POL_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOF_POL`"]
pub type SOF_POL_R = crate::R<bool, SOF_POL_A>;
impl SOF_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_POL_A {
        match self.bits {
            false => SOF_POL_A::SOF_POL_0,
            true => SOF_POL_A::SOF_POL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOF_POL_0`"]
    #[inline(always)]
    pub fn is_sof_pol_0(&self) -> bool {
        *self == SOF_POL_A::SOF_POL_0
    }
    #[doc = "Checks if the value of the field is `SOF_POL_1`"]
    #[inline(always)]
    pub fn is_sof_pol_1(&self) -> bool {
        *self == SOF_POL_A::SOF_POL_1
    }
}
#[doc = "Write proxy for field `SOF_POL`"]
pub struct SOF_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOF_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SOF interrupt is generated on SOF falling edge"]
    #[inline(always)]
    pub fn sof_pol_0(self) -> &'a mut W {
        self.variant(SOF_POL_A::SOF_POL_0)
    }
    #[doc = "SOF interrupt is generated on SOF rising edge"]
    #[inline(always)]
    pub fn sof_pol_1(self) -> &'a mut W {
        self.variant(SOF_POL_A::SOF_POL_1)
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
#[doc = "RxFIFO Full Interrupt Enable. This bit enables the RxFIFO full interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFF_INTEN_A {
    #[doc = "0: RxFIFO full interrupt disable"]
    RXFF_INTEN_0 = 0,
    #[doc = "1: RxFIFO full interrupt enable"]
    RXFF_INTEN_1 = 1,
}
impl From<RXFF_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXFF_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFF_INTEN`"]
pub type RXFF_INTEN_R = crate::R<bool, RXFF_INTEN_A>;
impl RXFF_INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFF_INTEN_A {
        match self.bits {
            false => RXFF_INTEN_A::RXFF_INTEN_0,
            true => RXFF_INTEN_A::RXFF_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFF_INTEN_0`"]
    #[inline(always)]
    pub fn is_rxff_inten_0(&self) -> bool {
        *self == RXFF_INTEN_A::RXFF_INTEN_0
    }
    #[doc = "Checks if the value of the field is `RXFF_INTEN_1`"]
    #[inline(always)]
    pub fn is_rxff_inten_1(&self) -> bool {
        *self == RXFF_INTEN_A::RXFF_INTEN_1
    }
}
#[doc = "Write proxy for field `RXFF_INTEN`"]
pub struct RXFF_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFF_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFF_INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RxFIFO full interrupt disable"]
    #[inline(always)]
    pub fn rxff_inten_0(self) -> &'a mut W {
        self.variant(RXFF_INTEN_A::RXFF_INTEN_0)
    }
    #[doc = "RxFIFO full interrupt enable"]
    #[inline(always)]
    pub fn rxff_inten_1(self) -> &'a mut W {
        self.variant(RXFF_INTEN_A::RXFF_INTEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Frame Buffer1 DMA Transfer Done Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FB1_DMA_DONE_INTEN_A {
    #[doc = "0: Frame Buffer1 DMA Transfer Done interrupt disable"]
    FB1_DMA_DONE_INTEN_0 = 0,
    #[doc = "1: Frame Buffer1 DMA Transfer Done interrupt enable"]
    FB1_DMA_DONE_INTEN_1 = 1,
}
impl From<FB1_DMA_DONE_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FB1_DMA_DONE_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FB1_DMA_DONE_INTEN`"]
pub type FB1_DMA_DONE_INTEN_R = crate::R<bool, FB1_DMA_DONE_INTEN_A>;
impl FB1_DMA_DONE_INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FB1_DMA_DONE_INTEN_A {
        match self.bits {
            false => FB1_DMA_DONE_INTEN_A::FB1_DMA_DONE_INTEN_0,
            true => FB1_DMA_DONE_INTEN_A::FB1_DMA_DONE_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FB1_DMA_DONE_INTEN_0`"]
    #[inline(always)]
    pub fn is_fb1_dma_done_inten_0(&self) -> bool {
        *self == FB1_DMA_DONE_INTEN_A::FB1_DMA_DONE_INTEN_0
    }
    #[doc = "Checks if the value of the field is `FB1_DMA_DONE_INTEN_1`"]
    #[inline(always)]
    pub fn is_fb1_dma_done_inten_1(&self) -> bool {
        *self == FB1_DMA_DONE_INTEN_A::FB1_DMA_DONE_INTEN_1
    }
}
#[doc = "Write proxy for field `FB1_DMA_DONE_INTEN`"]
pub struct FB1_DMA_DONE_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FB1_DMA_DONE_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB1_DMA_DONE_INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame Buffer1 DMA Transfer Done interrupt disable"]
    #[inline(always)]
    pub fn fb1_dma_done_inten_0(self) -> &'a mut W {
        self.variant(FB1_DMA_DONE_INTEN_A::FB1_DMA_DONE_INTEN_0)
    }
    #[doc = "Frame Buffer1 DMA Transfer Done interrupt enable"]
    #[inline(always)]
    pub fn fb1_dma_done_inten_1(self) -> &'a mut W {
        self.variant(FB1_DMA_DONE_INTEN_A::FB1_DMA_DONE_INTEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Frame Buffer2 DMA Transfer Done Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FB2_DMA_DONE_INTEN_A {
    #[doc = "0: Frame Buffer2 DMA Transfer Done interrupt disable"]
    FB2_DMA_DONE_INTEN_0 = 0,
    #[doc = "1: Frame Buffer2 DMA Transfer Done interrupt enable"]
    FB2_DMA_DONE_INTEN_1 = 1,
}
impl From<FB2_DMA_DONE_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FB2_DMA_DONE_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FB2_DMA_DONE_INTEN`"]
pub type FB2_DMA_DONE_INTEN_R = crate::R<bool, FB2_DMA_DONE_INTEN_A>;
impl FB2_DMA_DONE_INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FB2_DMA_DONE_INTEN_A {
        match self.bits {
            false => FB2_DMA_DONE_INTEN_A::FB2_DMA_DONE_INTEN_0,
            true => FB2_DMA_DONE_INTEN_A::FB2_DMA_DONE_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FB2_DMA_DONE_INTEN_0`"]
    #[inline(always)]
    pub fn is_fb2_dma_done_inten_0(&self) -> bool {
        *self == FB2_DMA_DONE_INTEN_A::FB2_DMA_DONE_INTEN_0
    }
    #[doc = "Checks if the value of the field is `FB2_DMA_DONE_INTEN_1`"]
    #[inline(always)]
    pub fn is_fb2_dma_done_inten_1(&self) -> bool {
        *self == FB2_DMA_DONE_INTEN_A::FB2_DMA_DONE_INTEN_1
    }
}
#[doc = "Write proxy for field `FB2_DMA_DONE_INTEN`"]
pub struct FB2_DMA_DONE_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FB2_DMA_DONE_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FB2_DMA_DONE_INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame Buffer2 DMA Transfer Done interrupt disable"]
    #[inline(always)]
    pub fn fb2_dma_done_inten_0(self) -> &'a mut W {
        self.variant(FB2_DMA_DONE_INTEN_A::FB2_DMA_DONE_INTEN_0)
    }
    #[doc = "Frame Buffer2 DMA Transfer Done interrupt enable"]
    #[inline(always)]
    pub fn fb2_dma_done_inten_1(self) -> &'a mut W {
        self.variant(FB2_DMA_DONE_INTEN_A::FB2_DMA_DONE_INTEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "STATFIFO Full Interrupt Enable. This bit enables the STAT FIFO interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATFF_INTEN_A {
    #[doc = "0: STATFIFO full interrupt disable"]
    STATFF_INTEN_0 = 0,
    #[doc = "1: STATFIFO full interrupt enable"]
    STATFF_INTEN_1 = 1,
}
impl From<STATFF_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: STATFF_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STATFF_INTEN`"]
pub type STATFF_INTEN_R = crate::R<bool, STATFF_INTEN_A>;
impl STATFF_INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATFF_INTEN_A {
        match self.bits {
            false => STATFF_INTEN_A::STATFF_INTEN_0,
            true => STATFF_INTEN_A::STATFF_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `STATFF_INTEN_0`"]
    #[inline(always)]
    pub fn is_statff_inten_0(&self) -> bool {
        *self == STATFF_INTEN_A::STATFF_INTEN_0
    }
    #[doc = "Checks if the value of the field is `STATFF_INTEN_1`"]
    #[inline(always)]
    pub fn is_statff_inten_1(&self) -> bool {
        *self == STATFF_INTEN_A::STATFF_INTEN_1
    }
}
#[doc = "Write proxy for field `STATFF_INTEN`"]
pub struct STATFF_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STATFF_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATFF_INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "STATFIFO full interrupt disable"]
    #[inline(always)]
    pub fn statff_inten_0(self) -> &'a mut W {
        self.variant(STATFF_INTEN_A::STATFF_INTEN_0)
    }
    #[doc = "STATFIFO full interrupt enable"]
    #[inline(always)]
    pub fn statff_inten_1(self) -> &'a mut W {
        self.variant(STATFF_INTEN_A::STATFF_INTEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "STATFIFO DMA Transfer Done Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFF_DMA_DONE_INTEN_A {
    #[doc = "0: STATFIFO DMA Transfer Done interrupt disable"]
    SFF_DMA_DONE_INTEN_0 = 0,
    #[doc = "1: STATFIFO DMA Transfer Done interrupt enable"]
    SFF_DMA_DONE_INTEN_1 = 1,
}
impl From<SFF_DMA_DONE_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SFF_DMA_DONE_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SFF_DMA_DONE_INTEN`"]
pub type SFF_DMA_DONE_INTEN_R = crate::R<bool, SFF_DMA_DONE_INTEN_A>;
impl SFF_DMA_DONE_INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFF_DMA_DONE_INTEN_A {
        match self.bits {
            false => SFF_DMA_DONE_INTEN_A::SFF_DMA_DONE_INTEN_0,
            true => SFF_DMA_DONE_INTEN_A::SFF_DMA_DONE_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SFF_DMA_DONE_INTEN_0`"]
    #[inline(always)]
    pub fn is_sff_dma_done_inten_0(&self) -> bool {
        *self == SFF_DMA_DONE_INTEN_A::SFF_DMA_DONE_INTEN_0
    }
    #[doc = "Checks if the value of the field is `SFF_DMA_DONE_INTEN_1`"]
    #[inline(always)]
    pub fn is_sff_dma_done_inten_1(&self) -> bool {
        *self == SFF_DMA_DONE_INTEN_A::SFF_DMA_DONE_INTEN_1
    }
}
#[doc = "Write proxy for field `SFF_DMA_DONE_INTEN`"]
pub struct SFF_DMA_DONE_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SFF_DMA_DONE_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFF_DMA_DONE_INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "STATFIFO DMA Transfer Done interrupt disable"]
    #[inline(always)]
    pub fn sff_dma_done_inten_0(self) -> &'a mut W {
        self.variant(SFF_DMA_DONE_INTEN_A::SFF_DMA_DONE_INTEN_0)
    }
    #[doc = "STATFIFO DMA Transfer Done interrupt enable"]
    #[inline(always)]
    pub fn sff_dma_done_inten_1(self) -> &'a mut W {
        self.variant(SFF_DMA_DONE_INTEN_A::SFF_DMA_DONE_INTEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_OR_INTEN_A {
    #[doc = "0: RxFIFO overrun interrupt is disabled"]
    RF_OR_INTEN_0 = 0,
    #[doc = "1: RxFIFO overrun interrupt is enabled"]
    RF_OR_INTEN_1 = 1,
}
impl From<RF_OR_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RF_OR_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF_OR_INTEN`"]
pub type RF_OR_INTEN_R = crate::R<bool, RF_OR_INTEN_A>;
impl RF_OR_INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_OR_INTEN_A {
        match self.bits {
            false => RF_OR_INTEN_A::RF_OR_INTEN_0,
            true => RF_OR_INTEN_A::RF_OR_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RF_OR_INTEN_0`"]
    #[inline(always)]
    pub fn is_rf_or_inten_0(&self) -> bool {
        *self == RF_OR_INTEN_A::RF_OR_INTEN_0
    }
    #[doc = "Checks if the value of the field is `RF_OR_INTEN_1`"]
    #[inline(always)]
    pub fn is_rf_or_inten_1(&self) -> bool {
        *self == RF_OR_INTEN_A::RF_OR_INTEN_1
    }
}
#[doc = "Write proxy for field `RF_OR_INTEN`"]
pub struct RF_OR_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_OR_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_OR_INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RxFIFO overrun interrupt is disabled"]
    #[inline(always)]
    pub fn rf_or_inten_0(self) -> &'a mut W {
        self.variant(RF_OR_INTEN_A::RF_OR_INTEN_0)
    }
    #[doc = "RxFIFO overrun interrupt is enabled"]
    #[inline(always)]
    pub fn rf_or_inten_1(self) -> &'a mut W {
        self.variant(RF_OR_INTEN_A::RF_OR_INTEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "STAT FIFO Overrun Interrupt Enable. This bit enables the STATFIFO overrun interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SF_OR_INTEN_A {
    #[doc = "0: STATFIFO overrun interrupt is disabled"]
    SF_OR_INTEN_0 = 0,
    #[doc = "1: STATFIFO overrun interrupt is enabled"]
    SF_OR_INTEN_1 = 1,
}
impl From<SF_OR_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SF_OR_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SF_OR_INTEN`"]
pub type SF_OR_INTEN_R = crate::R<bool, SF_OR_INTEN_A>;
impl SF_OR_INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SF_OR_INTEN_A {
        match self.bits {
            false => SF_OR_INTEN_A::SF_OR_INTEN_0,
            true => SF_OR_INTEN_A::SF_OR_INTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SF_OR_INTEN_0`"]
    #[inline(always)]
    pub fn is_sf_or_inten_0(&self) -> bool {
        *self == SF_OR_INTEN_A::SF_OR_INTEN_0
    }
    #[doc = "Checks if the value of the field is `SF_OR_INTEN_1`"]
    #[inline(always)]
    pub fn is_sf_or_inten_1(&self) -> bool {
        *self == SF_OR_INTEN_A::SF_OR_INTEN_1
    }
}
#[doc = "Write proxy for field `SF_OR_INTEN`"]
pub struct SF_OR_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_OR_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SF_OR_INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "STATFIFO overrun interrupt is disabled"]
    #[inline(always)]
    pub fn sf_or_inten_0(self) -> &'a mut W {
        self.variant(SF_OR_INTEN_A::SF_OR_INTEN_0)
    }
    #[doc = "STATFIFO overrun interrupt is enabled"]
    #[inline(always)]
    pub fn sf_or_inten_1(self) -> &'a mut W {
        self.variant(SF_OR_INTEN_A::SF_OR_INTEN_1)
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
#[doc = "Change Of Image Field (COF) Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COF_INT_EN_A {
    #[doc = "0: COF interrupt is disabled"]
    COF_INT_EN_0 = 0,
    #[doc = "1: COF interrupt is enabled"]
    COF_INT_EN_1 = 1,
}
impl From<COF_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COF_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COF_INT_EN`"]
pub type COF_INT_EN_R = crate::R<bool, COF_INT_EN_A>;
impl COF_INT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COF_INT_EN_A {
        match self.bits {
            false => COF_INT_EN_A::COF_INT_EN_0,
            true => COF_INT_EN_A::COF_INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `COF_INT_EN_0`"]
    #[inline(always)]
    pub fn is_cof_int_en_0(&self) -> bool {
        *self == COF_INT_EN_A::COF_INT_EN_0
    }
    #[doc = "Checks if the value of the field is `COF_INT_EN_1`"]
    #[inline(always)]
    pub fn is_cof_int_en_1(&self) -> bool {
        *self == COF_INT_EN_A::COF_INT_EN_1
    }
}
#[doc = "Write proxy for field `COF_INT_EN`"]
pub struct COF_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COF_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COF_INT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COF interrupt is disabled"]
    #[inline(always)]
    pub fn cof_int_en_0(self) -> &'a mut W {
        self.variant(COF_INT_EN_A::COF_INT_EN_0)
    }
    #[doc = "COF interrupt is enabled"]
    #[inline(always)]
    pub fn cof_int_en_1(self) -> &'a mut W {
        self.variant(COF_INT_EN_A::COF_INT_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "CCIR Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIR_MODE_A {
    #[doc = "0: Progressive mode is selected"]
    CCIR_MODE_0 = 0,
    #[doc = "1: Interlace mode is selected"]
    CCIR_MODE_1 = 1,
}
impl From<CCIR_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CCIR_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCIR_MODE`"]
pub type CCIR_MODE_R = crate::R<bool, CCIR_MODE_A>;
impl CCIR_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIR_MODE_A {
        match self.bits {
            false => CCIR_MODE_A::CCIR_MODE_0,
            true => CCIR_MODE_A::CCIR_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIR_MODE_0`"]
    #[inline(always)]
    pub fn is_ccir_mode_0(&self) -> bool {
        *self == CCIR_MODE_A::CCIR_MODE_0
    }
    #[doc = "Checks if the value of the field is `CCIR_MODE_1`"]
    #[inline(always)]
    pub fn is_ccir_mode_1(&self) -> bool {
        *self == CCIR_MODE_A::CCIR_MODE_1
    }
}
#[doc = "Write proxy for field `CCIR_MODE`"]
pub struct CCIR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIR_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIR_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Progressive mode is selected"]
    #[inline(always)]
    pub fn ccir_mode_0(self) -> &'a mut W {
        self.variant(CCIR_MODE_A::CCIR_MODE_0)
    }
    #[doc = "Interlace mode is selected"]
    #[inline(always)]
    pub fn ccir_mode_1(self) -> &'a mut W {
        self.variant(CCIR_MODE_A::CCIR_MODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "CSI-PrP Interface Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRP_IF_EN_A {
    #[doc = "0: CSI to PrP bus is disabled"]
    PRP_IF_EN_0 = 0,
    #[doc = "1: CSI to PrP bus is enabled"]
    PRP_IF_EN_1 = 1,
}
impl From<PRP_IF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PRP_IF_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PrP_IF_EN`"]
pub type PRP_IF_EN_R = crate::R<bool, PRP_IF_EN_A>;
impl PRP_IF_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRP_IF_EN_A {
        match self.bits {
            false => PRP_IF_EN_A::PRP_IF_EN_0,
            true => PRP_IF_EN_A::PRP_IF_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PRP_IF_EN_0`"]
    #[inline(always)]
    pub fn is_pr_p_if_en_0(&self) -> bool {
        *self == PRP_IF_EN_A::PRP_IF_EN_0
    }
    #[doc = "Checks if the value of the field is `PRP_IF_EN_1`"]
    #[inline(always)]
    pub fn is_pr_p_if_en_1(&self) -> bool {
        *self == PRP_IF_EN_A::PRP_IF_EN_1
    }
}
#[doc = "Write proxy for field `PrP_IF_EN`"]
pub struct PRP_IF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRP_IF_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRP_IF_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CSI to PrP bus is disabled"]
    #[inline(always)]
    pub fn pr_p_if_en_0(self) -> &'a mut W {
        self.variant(PRP_IF_EN_A::PRP_IF_EN_0)
    }
    #[doc = "CSI to PrP bus is enabled"]
    #[inline(always)]
    pub fn pr_p_if_en_1(self) -> &'a mut W {
        self.variant(PRP_IF_EN_A::PRP_IF_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOF_INT_EN_A {
    #[doc = "0: EOF interrupt is disabled."]
    EOF_INT_EN_0 = 0,
    #[doc = "1: EOF interrupt is generated when RX count value is reached."]
    EOF_INT_EN_1 = 1,
}
impl From<EOF_INT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EOF_INT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOF_INT_EN`"]
pub type EOF_INT_EN_R = crate::R<bool, EOF_INT_EN_A>;
impl EOF_INT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOF_INT_EN_A {
        match self.bits {
            false => EOF_INT_EN_A::EOF_INT_EN_0,
            true => EOF_INT_EN_A::EOF_INT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EOF_INT_EN_0`"]
    #[inline(always)]
    pub fn is_eof_int_en_0(&self) -> bool {
        *self == EOF_INT_EN_A::EOF_INT_EN_0
    }
    #[doc = "Checks if the value of the field is `EOF_INT_EN_1`"]
    #[inline(always)]
    pub fn is_eof_int_en_1(&self) -> bool {
        *self == EOF_INT_EN_A::EOF_INT_EN_1
    }
}
#[doc = "Write proxy for field `EOF_INT_EN`"]
pub struct EOF_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EOF_INT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOF_INT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EOF interrupt is disabled."]
    #[inline(always)]
    pub fn eof_int_en_0(self) -> &'a mut W {
        self.variant(EOF_INT_EN_A::EOF_INT_EN_0)
    }
    #[doc = "EOF interrupt is generated when RX count value is reached."]
    #[inline(always)]
    pub fn eof_int_en_1(self) -> &'a mut W {
        self.variant(EOF_INT_EN_A::EOF_INT_EN_1)
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
#[doc = "External VSYNC Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_VSYNC_A {
    #[doc = "0: Internal VSYNC mode"]
    EXT_VSYNC_0 = 0,
    #[doc = "1: External VSYNC mode"]
    EXT_VSYNC_1 = 1,
}
impl From<EXT_VSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: EXT_VSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXT_VSYNC`"]
pub type EXT_VSYNC_R = crate::R<bool, EXT_VSYNC_A>;
impl EXT_VSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXT_VSYNC_A {
        match self.bits {
            false => EXT_VSYNC_A::EXT_VSYNC_0,
            true => EXT_VSYNC_A::EXT_VSYNC_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXT_VSYNC_0`"]
    #[inline(always)]
    pub fn is_ext_vsync_0(&self) -> bool {
        *self == EXT_VSYNC_A::EXT_VSYNC_0
    }
    #[doc = "Checks if the value of the field is `EXT_VSYNC_1`"]
    #[inline(always)]
    pub fn is_ext_vsync_1(&self) -> bool {
        *self == EXT_VSYNC_A::EXT_VSYNC_1
    }
}
#[doc = "Write proxy for field `EXT_VSYNC`"]
pub struct EXT_VSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_VSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXT_VSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal VSYNC mode"]
    #[inline(always)]
    pub fn ext_vsync_0(self) -> &'a mut W {
        self.variant(EXT_VSYNC_A::EXT_VSYNC_0)
    }
    #[doc = "External VSYNC mode"]
    #[inline(always)]
    pub fn ext_vsync_1(self) -> &'a mut W {
        self.variant(EXT_VSYNC_A::EXT_VSYNC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "SWAP 16-Bit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAP16_EN_A {
    #[doc = "0: Disable swapping"]
    SWAP16_EN_0 = 0,
    #[doc = "1: Enable swapping"]
    SWAP16_EN_1 = 1,
}
impl From<SWAP16_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SWAP16_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWAP16_EN`"]
pub type SWAP16_EN_R = crate::R<bool, SWAP16_EN_A>;
impl SWAP16_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWAP16_EN_A {
        match self.bits {
            false => SWAP16_EN_A::SWAP16_EN_0,
            true => SWAP16_EN_A::SWAP16_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWAP16_EN_0`"]
    #[inline(always)]
    pub fn is_swap16_en_0(&self) -> bool {
        *self == SWAP16_EN_A::SWAP16_EN_0
    }
    #[doc = "Checks if the value of the field is `SWAP16_EN_1`"]
    #[inline(always)]
    pub fn is_swap16_en_1(&self) -> bool {
        *self == SWAP16_EN_A::SWAP16_EN_1
    }
}
#[doc = "Write proxy for field `SWAP16_EN`"]
pub struct SWAP16_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP16_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWAP16_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable swapping"]
    #[inline(always)]
    pub fn swap16_en_0(self) -> &'a mut W {
        self.variant(SWAP16_EN_A::SWAP16_EN_0)
    }
    #[doc = "Enable swapping"]
    #[inline(always)]
    pub fn swap16_en_1(self) -> &'a mut W {
        self.variant(SWAP16_EN_A::SWAP16_EN_1)
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
    #[doc = "Bit 0 - Pixel Bit"]
    #[inline(always)]
    pub fn pixel_bit(&self) -> PIXEL_BIT_R {
        PIXEL_BIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Valid Pixel Clock Edge Select"]
    #[inline(always)]
    pub fn redge(&self) -> REDGE_R {
        REDGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Invert Pixel Clock Input"]
    #[inline(always)]
    pub fn inv_pclk(&self) -> INV_PCLK_R {
        INV_PCLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Invert Data Input. This bit enables or disables internal inverters on the data lines."]
    #[inline(always)]
    pub fn inv_data(&self) -> INV_DATA_R {
        INV_DATA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Gated Clock Mode Enable"]
    #[inline(always)]
    pub fn gclk_mode(&self) -> GCLK_MODE_R {
        GCLK_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Asynchronous RXFIFO Clear"]
    #[inline(always)]
    pub fn clr_rxfifo(&self) -> CLR_RXFIFO_R {
        CLR_RXFIFO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Asynchronous STATFIFO Clear"]
    #[inline(always)]
    pub fn clr_statfifo(&self) -> CLR_STATFIFO_R {
        CLR_STATFIFO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data Packing Direction"]
    #[inline(always)]
    pub fn pack_dir(&self) -> PACK_DIR_R {
        PACK_DIR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FIFO Clear Control"]
    #[inline(always)]
    pub fn fcc(&self) -> FCC_R {
        FCC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CCIR656 Interface Enable"]
    #[inline(always)]
    pub fn ccir_en(&self) -> CCIR_EN_R {
        CCIR_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSYNC Polarity Select"]
    #[inline(always)]
    pub fn hsync_pol(&self) -> HSYNC_POL_R {
        HSYNC_POL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt."]
    #[inline(always)]
    pub fn sof_inten(&self) -> SOF_INTEN_R {
        SOF_INTEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt."]
    #[inline(always)]
    pub fn sof_pol(&self) -> SOF_POL_R {
        SOF_POL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RxFIFO Full Interrupt Enable. This bit enables the RxFIFO full interrupt."]
    #[inline(always)]
    pub fn rxff_inten(&self) -> RXFF_INTEN_R {
        RXFF_INTEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Frame Buffer1 DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub fn fb1_dma_done_inten(&self) -> FB1_DMA_DONE_INTEN_R {
        FB1_DMA_DONE_INTEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Frame Buffer2 DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub fn fb2_dma_done_inten(&self) -> FB2_DMA_DONE_INTEN_R {
        FB2_DMA_DONE_INTEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - STATFIFO Full Interrupt Enable. This bit enables the STAT FIFO interrupt."]
    #[inline(always)]
    pub fn statff_inten(&self) -> STATFF_INTEN_R {
        STATFF_INTEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - STATFIFO DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub fn sff_dma_done_inten(&self) -> SFF_DMA_DONE_INTEN_R {
        SFF_DMA_DONE_INTEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt."]
    #[inline(always)]
    pub fn rf_or_inten(&self) -> RF_OR_INTEN_R {
        RF_OR_INTEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - STAT FIFO Overrun Interrupt Enable. This bit enables the STATFIFO overrun interrupt."]
    #[inline(always)]
    pub fn sf_or_inten(&self) -> SF_OR_INTEN_R {
        SF_OR_INTEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Change Of Image Field (COF) Interrupt Enable"]
    #[inline(always)]
    pub fn cof_int_en(&self) -> COF_INT_EN_R {
        COF_INT_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CCIR Mode Select"]
    #[inline(always)]
    pub fn ccir_mode(&self) -> CCIR_MODE_R {
        CCIR_MODE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CSI-PrP Interface Enable"]
    #[inline(always)]
    pub fn pr_p_if_en(&self) -> PRP_IF_EN_R {
        PRP_IF_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt."]
    #[inline(always)]
    pub fn eof_int_en(&self) -> EOF_INT_EN_R {
        EOF_INT_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - External VSYNC Enable"]
    #[inline(always)]
    pub fn ext_vsync(&self) -> EXT_VSYNC_R {
        EXT_VSYNC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SWAP 16-Bit Enable"]
    #[inline(always)]
    pub fn swap16_en(&self) -> SWAP16_EN_R {
        SWAP16_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pixel Bit"]
    #[inline(always)]
    pub fn pixel_bit(&mut self) -> PIXEL_BIT_W {
        PIXEL_BIT_W { w: self }
    }
    #[doc = "Bit 1 - Valid Pixel Clock Edge Select"]
    #[inline(always)]
    pub fn redge(&mut self) -> REDGE_W {
        REDGE_W { w: self }
    }
    #[doc = "Bit 2 - Invert Pixel Clock Input"]
    #[inline(always)]
    pub fn inv_pclk(&mut self) -> INV_PCLK_W {
        INV_PCLK_W { w: self }
    }
    #[doc = "Bit 3 - Invert Data Input. This bit enables or disables internal inverters on the data lines."]
    #[inline(always)]
    pub fn inv_data(&mut self) -> INV_DATA_W {
        INV_DATA_W { w: self }
    }
    #[doc = "Bit 4 - Gated Clock Mode Enable"]
    #[inline(always)]
    pub fn gclk_mode(&mut self) -> GCLK_MODE_W {
        GCLK_MODE_W { w: self }
    }
    #[doc = "Bit 5 - Asynchronous RXFIFO Clear"]
    #[inline(always)]
    pub fn clr_rxfifo(&mut self) -> CLR_RXFIFO_W {
        CLR_RXFIFO_W { w: self }
    }
    #[doc = "Bit 6 - Asynchronous STATFIFO Clear"]
    #[inline(always)]
    pub fn clr_statfifo(&mut self) -> CLR_STATFIFO_W {
        CLR_STATFIFO_W { w: self }
    }
    #[doc = "Bit 7 - Data Packing Direction"]
    #[inline(always)]
    pub fn pack_dir(&mut self) -> PACK_DIR_W {
        PACK_DIR_W { w: self }
    }
    #[doc = "Bit 8 - FIFO Clear Control"]
    #[inline(always)]
    pub fn fcc(&mut self) -> FCC_W {
        FCC_W { w: self }
    }
    #[doc = "Bit 10 - CCIR656 Interface Enable"]
    #[inline(always)]
    pub fn ccir_en(&mut self) -> CCIR_EN_W {
        CCIR_EN_W { w: self }
    }
    #[doc = "Bit 11 - HSYNC Polarity Select"]
    #[inline(always)]
    pub fn hsync_pol(&mut self) -> HSYNC_POL_W {
        HSYNC_POL_W { w: self }
    }
    #[doc = "Bit 16 - Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt."]
    #[inline(always)]
    pub fn sof_inten(&mut self) -> SOF_INTEN_W {
        SOF_INTEN_W { w: self }
    }
    #[doc = "Bit 17 - SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt."]
    #[inline(always)]
    pub fn sof_pol(&mut self) -> SOF_POL_W {
        SOF_POL_W { w: self }
    }
    #[doc = "Bit 18 - RxFIFO Full Interrupt Enable. This bit enables the RxFIFO full interrupt."]
    #[inline(always)]
    pub fn rxff_inten(&mut self) -> RXFF_INTEN_W {
        RXFF_INTEN_W { w: self }
    }
    #[doc = "Bit 19 - Frame Buffer1 DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub fn fb1_dma_done_inten(&mut self) -> FB1_DMA_DONE_INTEN_W {
        FB1_DMA_DONE_INTEN_W { w: self }
    }
    #[doc = "Bit 20 - Frame Buffer2 DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub fn fb2_dma_done_inten(&mut self) -> FB2_DMA_DONE_INTEN_W {
        FB2_DMA_DONE_INTEN_W { w: self }
    }
    #[doc = "Bit 21 - STATFIFO Full Interrupt Enable. This bit enables the STAT FIFO interrupt."]
    #[inline(always)]
    pub fn statff_inten(&mut self) -> STATFF_INTEN_W {
        STATFF_INTEN_W { w: self }
    }
    #[doc = "Bit 22 - STATFIFO DMA Transfer Done Interrupt Enable"]
    #[inline(always)]
    pub fn sff_dma_done_inten(&mut self) -> SFF_DMA_DONE_INTEN_W {
        SFF_DMA_DONE_INTEN_W { w: self }
    }
    #[doc = "Bit 24 - RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt."]
    #[inline(always)]
    pub fn rf_or_inten(&mut self) -> RF_OR_INTEN_W {
        RF_OR_INTEN_W { w: self }
    }
    #[doc = "Bit 25 - STAT FIFO Overrun Interrupt Enable. This bit enables the STATFIFO overrun interrupt."]
    #[inline(always)]
    pub fn sf_or_inten(&mut self) -> SF_OR_INTEN_W {
        SF_OR_INTEN_W { w: self }
    }
    #[doc = "Bit 26 - Change Of Image Field (COF) Interrupt Enable"]
    #[inline(always)]
    pub fn cof_int_en(&mut self) -> COF_INT_EN_W {
        COF_INT_EN_W { w: self }
    }
    #[doc = "Bit 27 - CCIR Mode Select"]
    #[inline(always)]
    pub fn ccir_mode(&mut self) -> CCIR_MODE_W {
        CCIR_MODE_W { w: self }
    }
    #[doc = "Bit 28 - CSI-PrP Interface Enable"]
    #[inline(always)]
    pub fn pr_p_if_en(&mut self) -> PRP_IF_EN_W {
        PRP_IF_EN_W { w: self }
    }
    #[doc = "Bit 29 - End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt."]
    #[inline(always)]
    pub fn eof_int_en(&mut self) -> EOF_INT_EN_W {
        EOF_INT_EN_W { w: self }
    }
    #[doc = "Bit 30 - External VSYNC Enable"]
    #[inline(always)]
    pub fn ext_vsync(&mut self) -> EXT_VSYNC_W {
        EXT_VSYNC_W { w: self }
    }
    #[doc = "Bit 31 - SWAP 16-Bit Enable"]
    #[inline(always)]
    pub fn swap16_en(&mut self) -> SWAP16_EN_W {
        SWAP16_EN_W { w: self }
    }
}
