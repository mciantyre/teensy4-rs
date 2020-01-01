#[doc = "Reader of register CTRL_SET"]
pub type R = crate::R<u32, super::CTRL_SET>;
#[doc = "Writer for register CTRL_SET"]
pub type W = crate::W<u32, super::CTRL_SET>;
#[doc = "Register CTRL_SET `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::CTRL_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "Reader of field `RUN`"]
pub type RUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUN`"]
pub struct RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_W<'a> {
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
#[doc = "Used only when WORD_LENGTH = 3, i\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_FORMAT_24_BIT_A {
    #[doc = "0: Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
    ALL_24_BITS_VALID = 0,
    #[doc = "1: Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
    DROP_UPPER_2_BITS_PER_BYTE = 1,
}
impl From<DATA_FORMAT_24_BIT_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_FORMAT_24_BIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATA_FORMAT_24_BIT`"]
pub type DATA_FORMAT_24_BIT_R = crate::R<bool, DATA_FORMAT_24_BIT_A>;
impl DATA_FORMAT_24_BIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_FORMAT_24_BIT_A {
        match self.bits {
            false => DATA_FORMAT_24_BIT_A::ALL_24_BITS_VALID,
            true => DATA_FORMAT_24_BIT_A::DROP_UPPER_2_BITS_PER_BYTE,
        }
    }
    #[doc = "Checks if the value of the field is `ALL_24_BITS_VALID`"]
    #[inline(always)]
    pub fn is_all_24_bits_valid(&self) -> bool {
        *self == DATA_FORMAT_24_BIT_A::ALL_24_BITS_VALID
    }
    #[doc = "Checks if the value of the field is `DROP_UPPER_2_BITS_PER_BYTE`"]
    #[inline(always)]
    pub fn is_drop_upper_2_bits_per_byte(&self) -> bool {
        *self == DATA_FORMAT_24_BIT_A::DROP_UPPER_2_BITS_PER_BYTE
    }
}
#[doc = "Write proxy for field `DATA_FORMAT_24_BIT`"]
pub struct DATA_FORMAT_24_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_FORMAT_24_BIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_FORMAT_24_BIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
    #[inline(always)]
    pub fn all_24_bits_valid(self) -> &'a mut W {
        self.variant(DATA_FORMAT_24_BIT_A::ALL_24_BITS_VALID)
    }
    #[doc = "Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
    #[inline(always)]
    pub fn drop_upper_2_bits_per_byte(self) -> &'a mut W {
        self.variant(DATA_FORMAT_24_BIT_A::DROP_UPPER_2_BITS_PER_BYTE)
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
#[doc = "Used only when WORD_LENGTH = 2, i.e. 18-bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_FORMAT_18_BIT_A {
    #[doc = "0: Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
    LOWER_18_BITS_VALID = 0,
    #[doc = "1: Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
    UPPER_18_BITS_VALID = 1,
}
impl From<DATA_FORMAT_18_BIT_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_FORMAT_18_BIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATA_FORMAT_18_BIT`"]
pub type DATA_FORMAT_18_BIT_R = crate::R<bool, DATA_FORMAT_18_BIT_A>;
impl DATA_FORMAT_18_BIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_FORMAT_18_BIT_A {
        match self.bits {
            false => DATA_FORMAT_18_BIT_A::LOWER_18_BITS_VALID,
            true => DATA_FORMAT_18_BIT_A::UPPER_18_BITS_VALID,
        }
    }
    #[doc = "Checks if the value of the field is `LOWER_18_BITS_VALID`"]
    #[inline(always)]
    pub fn is_lower_18_bits_valid(&self) -> bool {
        *self == DATA_FORMAT_18_BIT_A::LOWER_18_BITS_VALID
    }
    #[doc = "Checks if the value of the field is `UPPER_18_BITS_VALID`"]
    #[inline(always)]
    pub fn is_upper_18_bits_valid(&self) -> bool {
        *self == DATA_FORMAT_18_BIT_A::UPPER_18_BITS_VALID
    }
}
#[doc = "Write proxy for field `DATA_FORMAT_18_BIT`"]
pub struct DATA_FORMAT_18_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_FORMAT_18_BIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_FORMAT_18_BIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
    #[inline(always)]
    pub fn lower_18_bits_valid(self) -> &'a mut W {
        self.variant(DATA_FORMAT_18_BIT_A::LOWER_18_BITS_VALID)
    }
    #[doc = "Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
    #[inline(always)]
    pub fn upper_18_bits_valid(self) -> &'a mut W {
        self.variant(DATA_FORMAT_18_BIT_A::UPPER_18_BITS_VALID)
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
#[doc = "Reader of field `DATA_FORMAT_16_BIT`"]
pub type DATA_FORMAT_16_BIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATA_FORMAT_16_BIT`"]
pub struct DATA_FORMAT_16_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_FORMAT_16_BIT_W<'a> {
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
#[doc = "Reader of field `MASTER`"]
pub type MASTER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASTER`"]
pub struct MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_W<'a> {
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
#[doc = "Reader of field `ENABLE_PXP_HANDSHAKE`"]
pub type ENABLE_PXP_HANDSHAKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_PXP_HANDSHAKE`"]
pub struct ENABLE_PXP_HANDSHAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_PXP_HANDSHAKE_W<'a> {
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
#[doc = "Input data format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WORD_LENGTH_A {
    #[doc = "0: Input data is 16 bits per pixel."]
    _16_BIT = 0,
    #[doc = "1: Input data is 8 bits wide."]
    _8_BIT = 1,
    #[doc = "2: Input data is 18 bits per pixel."]
    _18_BIT = 2,
    #[doc = "3: Input data is 24 bits per pixel."]
    _24_BIT = 3,
}
impl From<WORD_LENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WORD_LENGTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WORD_LENGTH`"]
pub type WORD_LENGTH_R = crate::R<u8, WORD_LENGTH_A>;
impl WORD_LENGTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WORD_LENGTH_A {
        match self.bits {
            0 => WORD_LENGTH_A::_16_BIT,
            1 => WORD_LENGTH_A::_8_BIT,
            2 => WORD_LENGTH_A::_18_BIT,
            3 => WORD_LENGTH_A::_24_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == WORD_LENGTH_A::_16_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == WORD_LENGTH_A::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_18_BIT`"]
    #[inline(always)]
    pub fn is_18_bit(&self) -> bool {
        *self == WORD_LENGTH_A::_18_BIT
    }
    #[doc = "Checks if the value of the field is `_24_BIT`"]
    #[inline(always)]
    pub fn is_24_bit(&self) -> bool {
        *self == WORD_LENGTH_A::_24_BIT
    }
}
#[doc = "Write proxy for field `WORD_LENGTH`"]
pub struct WORD_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_LENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORD_LENGTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input data is 16 bits per pixel."]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(WORD_LENGTH_A::_16_BIT)
    }
    #[doc = "Input data is 8 bits wide."]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(WORD_LENGTH_A::_8_BIT)
    }
    #[doc = "Input data is 18 bits per pixel."]
    #[inline(always)]
    pub fn _18_bit(self) -> &'a mut W {
        self.variant(WORD_LENGTH_A::_18_BIT)
    }
    #[doc = "Input data is 24 bits per pixel."]
    #[inline(always)]
    pub fn _24_bit(self) -> &'a mut W {
        self.variant(WORD_LENGTH_A::_24_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "LCD Data bus transfer width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LCD_DATABUS_WIDTH_A {
    #[doc = "0: 16-bit data bus mode."]
    _16_BIT = 0,
    #[doc = "1: 8-bit data bus mode."]
    _8_BIT = 1,
    #[doc = "2: 18-bit data bus mode."]
    _18_BIT = 2,
    #[doc = "3: 24-bit data bus mode."]
    _24_BIT = 3,
}
impl From<LCD_DATABUS_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: LCD_DATABUS_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LCD_DATABUS_WIDTH`"]
pub type LCD_DATABUS_WIDTH_R = crate::R<u8, LCD_DATABUS_WIDTH_A>;
impl LCD_DATABUS_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_DATABUS_WIDTH_A {
        match self.bits {
            0 => LCD_DATABUS_WIDTH_A::_16_BIT,
            1 => LCD_DATABUS_WIDTH_A::_8_BIT,
            2 => LCD_DATABUS_WIDTH_A::_18_BIT,
            3 => LCD_DATABUS_WIDTH_A::_24_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == LCD_DATABUS_WIDTH_A::_16_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == LCD_DATABUS_WIDTH_A::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_18_BIT`"]
    #[inline(always)]
    pub fn is_18_bit(&self) -> bool {
        *self == LCD_DATABUS_WIDTH_A::_18_BIT
    }
    #[doc = "Checks if the value of the field is `_24_BIT`"]
    #[inline(always)]
    pub fn is_24_bit(&self) -> bool {
        *self == LCD_DATABUS_WIDTH_A::_24_BIT
    }
}
#[doc = "Write proxy for field `LCD_DATABUS_WIDTH`"]
pub struct LCD_DATABUS_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_DATABUS_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LCD_DATABUS_WIDTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "16-bit data bus mode."]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(LCD_DATABUS_WIDTH_A::_16_BIT)
    }
    #[doc = "8-bit data bus mode."]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(LCD_DATABUS_WIDTH_A::_8_BIT)
    }
    #[doc = "18-bit data bus mode."]
    #[inline(always)]
    pub fn _18_bit(self) -> &'a mut W {
        self.variant(LCD_DATABUS_WIDTH_A::_18_BIT)
    }
    #[doc = "24-bit data bus mode."]
    #[inline(always)]
    pub fn _24_bit(self) -> &'a mut W {
        self.variant(LCD_DATABUS_WIDTH_A::_24_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSC_DATA_SWIZZLE_A {
    #[doc = "0: No byte swapping.(Little endian)"]
    NO_SWAP = 0,
    #[doc = "1: Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 1,
    #[doc = "2: Swap half-words."]
    HWD_SWAP = 2,
    #[doc = "3: Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 3,
}
impl From<CSC_DATA_SWIZZLE_A> for u8 {
    #[inline(always)]
    fn from(variant: CSC_DATA_SWIZZLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CSC_DATA_SWIZZLE`"]
pub type CSC_DATA_SWIZZLE_R = crate::R<u8, CSC_DATA_SWIZZLE_A>;
impl CSC_DATA_SWIZZLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSC_DATA_SWIZZLE_A {
        match self.bits {
            0 => CSC_DATA_SWIZZLE_A::NO_SWAP,
            1 => CSC_DATA_SWIZZLE_A::BIG_ENDIAN_SWAP,
            2 => CSC_DATA_SWIZZLE_A::HWD_SWAP,
            3 => CSC_DATA_SWIZZLE_A::HWD_BYTE_SWAP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_SWAP`"]
    #[inline(always)]
    pub fn is_no_swap(&self) -> bool {
        *self == CSC_DATA_SWIZZLE_A::NO_SWAP
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN_SWAP`"]
    #[inline(always)]
    pub fn is_big_endian_swap(&self) -> bool {
        *self == CSC_DATA_SWIZZLE_A::BIG_ENDIAN_SWAP
    }
    #[doc = "Checks if the value of the field is `HWD_SWAP`"]
    #[inline(always)]
    pub fn is_hwd_swap(&self) -> bool {
        *self == CSC_DATA_SWIZZLE_A::HWD_SWAP
    }
    #[doc = "Checks if the value of the field is `HWD_BYTE_SWAP`"]
    #[inline(always)]
    pub fn is_hwd_byte_swap(&self) -> bool {
        *self == CSC_DATA_SWIZZLE_A::HWD_BYTE_SWAP
    }
}
#[doc = "Write proxy for field `CSC_DATA_SWIZZLE`"]
pub struct CSC_DATA_SWIZZLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSC_DATA_SWIZZLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSC_DATA_SWIZZLE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No byte swapping.(Little endian)"]
    #[inline(always)]
    pub fn no_swap(self) -> &'a mut W {
        self.variant(CSC_DATA_SWIZZLE_A::NO_SWAP)
    }
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    #[inline(always)]
    pub fn big_endian_swap(self) -> &'a mut W {
        self.variant(CSC_DATA_SWIZZLE_A::BIG_ENDIAN_SWAP)
    }
    #[doc = "Swap half-words."]
    #[inline(always)]
    pub fn hwd_swap(self) -> &'a mut W {
        self.variant(CSC_DATA_SWIZZLE_A::HWD_SWAP)
    }
    #[doc = "Swap bytes within each half-word."]
    #[inline(always)]
    pub fn hwd_byte_swap(self) -> &'a mut W {
        self.variant(CSC_DATA_SWIZZLE_A::HWD_BYTE_SWAP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "This field specifies how to swap the bytes fetched by the bus master interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT_DATA_SWIZZLE_A {
    #[doc = "0: No byte swapping.(Little endian)"]
    NO_SWAP = 0,
    #[doc = "1: Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 1,
    #[doc = "2: Swap half-words."]
    HWD_SWAP = 2,
    #[doc = "3: Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 3,
}
impl From<INPUT_DATA_SWIZZLE_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT_DATA_SWIZZLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INPUT_DATA_SWIZZLE`"]
pub type INPUT_DATA_SWIZZLE_R = crate::R<u8, INPUT_DATA_SWIZZLE_A>;
impl INPUT_DATA_SWIZZLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT_DATA_SWIZZLE_A {
        match self.bits {
            0 => INPUT_DATA_SWIZZLE_A::NO_SWAP,
            1 => INPUT_DATA_SWIZZLE_A::BIG_ENDIAN_SWAP,
            2 => INPUT_DATA_SWIZZLE_A::HWD_SWAP,
            3 => INPUT_DATA_SWIZZLE_A::HWD_BYTE_SWAP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_SWAP`"]
    #[inline(always)]
    pub fn is_no_swap(&self) -> bool {
        *self == INPUT_DATA_SWIZZLE_A::NO_SWAP
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN_SWAP`"]
    #[inline(always)]
    pub fn is_big_endian_swap(&self) -> bool {
        *self == INPUT_DATA_SWIZZLE_A::BIG_ENDIAN_SWAP
    }
    #[doc = "Checks if the value of the field is `HWD_SWAP`"]
    #[inline(always)]
    pub fn is_hwd_swap(&self) -> bool {
        *self == INPUT_DATA_SWIZZLE_A::HWD_SWAP
    }
    #[doc = "Checks if the value of the field is `HWD_BYTE_SWAP`"]
    #[inline(always)]
    pub fn is_hwd_byte_swap(&self) -> bool {
        *self == INPUT_DATA_SWIZZLE_A::HWD_BYTE_SWAP
    }
}
#[doc = "Write proxy for field `INPUT_DATA_SWIZZLE`"]
pub struct INPUT_DATA_SWIZZLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_DATA_SWIZZLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT_DATA_SWIZZLE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No byte swapping.(Little endian)"]
    #[inline(always)]
    pub fn no_swap(self) -> &'a mut W {
        self.variant(INPUT_DATA_SWIZZLE_A::NO_SWAP)
    }
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    #[inline(always)]
    pub fn big_endian_swap(self) -> &'a mut W {
        self.variant(INPUT_DATA_SWIZZLE_A::BIG_ENDIAN_SWAP)
    }
    #[doc = "Swap half-words."]
    #[inline(always)]
    pub fn hwd_swap(self) -> &'a mut W {
        self.variant(INPUT_DATA_SWIZZLE_A::HWD_SWAP)
    }
    #[doc = "Swap bytes within each half-word."]
    #[inline(always)]
    pub fn hwd_byte_swap(self) -> &'a mut W {
        self.variant(INPUT_DATA_SWIZZLE_A::HWD_BYTE_SWAP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `DOTCLK_MODE`"]
pub type DOTCLK_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOTCLK_MODE`"]
pub struct DOTCLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOTCLK_MODE_W<'a> {
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
#[doc = "Reader of field `BYPASS_COUNT`"]
pub type BYPASS_COUNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS_COUNT`"]
pub struct BYPASS_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_COUNT_W<'a> {
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
#[doc = "Reader of field `SHIFT_NUM_BITS`"]
pub type SHIFT_NUM_BITS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHIFT_NUM_BITS`"]
pub struct SHIFT_NUM_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_NUM_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | (((value as u32) & 0x1f) << 21);
        self.w
    }
}
#[doc = "Use this bit to determine the direction of shift of transmit data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_SHIFT_DIR_A {
    #[doc = "0: Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_LEFT = 0,
    #[doc = "1: Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_RIGHT = 1,
}
impl From<DATA_SHIFT_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_SHIFT_DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATA_SHIFT_DIR`"]
pub type DATA_SHIFT_DIR_R = crate::R<bool, DATA_SHIFT_DIR_A>;
impl DATA_SHIFT_DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_SHIFT_DIR_A {
        match self.bits {
            false => DATA_SHIFT_DIR_A::TXDATA_SHIFT_LEFT,
            true => DATA_SHIFT_DIR_A::TXDATA_SHIFT_RIGHT,
        }
    }
    #[doc = "Checks if the value of the field is `TXDATA_SHIFT_LEFT`"]
    #[inline(always)]
    pub fn is_txdata_shift_left(&self) -> bool {
        *self == DATA_SHIFT_DIR_A::TXDATA_SHIFT_LEFT
    }
    #[doc = "Checks if the value of the field is `TXDATA_SHIFT_RIGHT`"]
    #[inline(always)]
    pub fn is_txdata_shift_right(&self) -> bool {
        *self == DATA_SHIFT_DIR_A::TXDATA_SHIFT_RIGHT
    }
}
#[doc = "Write proxy for field `DATA_SHIFT_DIR`"]
pub struct DATA_SHIFT_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_SHIFT_DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_SHIFT_DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
    #[inline(always)]
    pub fn txdata_shift_left(self) -> &'a mut W {
        self.variant(DATA_SHIFT_DIR_A::TXDATA_SHIFT_LEFT)
    }
    #[doc = "Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
    #[inline(always)]
    pub fn txdata_shift_right(self) -> &'a mut W {
        self.variant(DATA_SHIFT_DIR_A::TXDATA_SHIFT_RIGHT)
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
#[doc = "Reader of field `CLKGATE`"]
pub type CLKGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKGATE`"]
pub struct CLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGATE_W<'a> {
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
#[doc = "Reader of field `SFTRST`"]
pub type SFTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFTRST`"]
pub struct SFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTRST_W<'a> {
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
    #[doc = "Bit 0 - When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Used only when WORD_LENGTH = 3, i"]
    #[inline(always)]
    pub fn data_format_24_bit(&self) -> DATA_FORMAT_24_BIT_R {
        DATA_FORMAT_24_BIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    #[inline(always)]
    pub fn data_format_18_bit(&self) -> DATA_FORMAT_18_BIT_R {
        DATA_FORMAT_18_BIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    #[inline(always)]
    pub fn data_format_16_bit(&self) -> DATA_FORMAT_16_BIT_R {
        DATA_FORMAT_16_BIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set this bit to make the LCDIF act as a bus master"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    #[inline(always)]
    pub fn enable_pxp_handshake(&self) -> ENABLE_PXP_HANDSHAKE_R {
        ENABLE_PXP_HANDSHAKE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Input data format."]
    #[inline(always)]
    pub fn word_length(&self) -> WORD_LENGTH_R {
        WORD_LENGTH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - LCD Data bus transfer width."]
    #[inline(always)]
    pub fn lcd_databus_width(&self) -> LCD_DATABUS_WIDTH_R {
        LCD_DATABUS_WIDTH_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    #[inline(always)]
    pub fn csc_data_swizzle(&self) -> CSC_DATA_SWIZZLE_R {
        CSC_DATA_SWIZZLE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - This field specifies how to swap the bytes fetched by the bus master interface"]
    #[inline(always)]
    pub fn input_data_swizzle(&self) -> INPUT_DATA_SWIZZLE_R {
        INPUT_DATA_SWIZZLE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    #[inline(always)]
    pub fn dotclk_mode(&self) -> DOTCLK_MODE_R {
        DOTCLK_MODE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    #[inline(always)]
    pub fn bypass_count(&self) -> BYPASS_COUNT_R {
        BYPASS_COUNT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 21:25 - The data to be transmitted is shifted left or right by this number of bits."]
    #[inline(always)]
    pub fn shift_num_bits(&self) -> SHIFT_NUM_BITS_R {
        SHIFT_NUM_BITS_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Use this bit to determine the direction of shift of transmit data."]
    #[inline(always)]
    pub fn data_shift_dir(&self) -> DATA_SHIFT_DIR_R {
        DATA_SHIFT_DIR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This bit must be set to zero for normal operation"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This bit must be set to zero to enable normal operation of the LCDIF"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When this bit is set by software, the LCDIF will begin transferring data between the SoC and the display"]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W {
        RUN_W { w: self }
    }
    #[doc = "Bit 1 - Used only when WORD_LENGTH = 3, i"]
    #[inline(always)]
    pub fn data_format_24_bit(&mut self) -> DATA_FORMAT_24_BIT_W {
        DATA_FORMAT_24_BIT_W { w: self }
    }
    #[doc = "Bit 2 - Used only when WORD_LENGTH = 2, i.e. 18-bit."]
    #[inline(always)]
    pub fn data_format_18_bit(&mut self) -> DATA_FORMAT_18_BIT_W {
        DATA_FORMAT_18_BIT_W { w: self }
    }
    #[doc = "Bit 3 - When this bit is 1 and WORD_LENGTH = 0, it implies that the 16-bit data is in ARGB555 format"]
    #[inline(always)]
    pub fn data_format_16_bit(&mut self) -> DATA_FORMAT_16_BIT_W {
        DATA_FORMAT_16_BIT_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to make the LCDIF act as a bus master"]
    #[inline(always)]
    pub fn master(&mut self) -> MASTER_W {
        MASTER_W { w: self }
    }
    #[doc = "Bit 6 - If this bit is set and LCDIF_MASTER bit is set, the LCDIF will act as bus master and the handshake mechanism between LCDIF and PXP will be turned on"]
    #[inline(always)]
    pub fn enable_pxp_handshake(&mut self) -> ENABLE_PXP_HANDSHAKE_W {
        ENABLE_PXP_HANDSHAKE_W { w: self }
    }
    #[doc = "Bits 8:9 - Input data format."]
    #[inline(always)]
    pub fn word_length(&mut self) -> WORD_LENGTH_W {
        WORD_LENGTH_W { w: self }
    }
    #[doc = "Bits 10:11 - LCD Data bus transfer width."]
    #[inline(always)]
    pub fn lcd_databus_width(&mut self) -> LCD_DATABUS_WIDTH_W {
        LCD_DATABUS_WIDTH_W { w: self }
    }
    #[doc = "Bits 12:13 - This field specifies how to swap the bytes after the data has been converted into an internal representation of 24 bits per pixel and before it is transmitted over the LCD interface bus"]
    #[inline(always)]
    pub fn csc_data_swizzle(&mut self) -> CSC_DATA_SWIZZLE_W {
        CSC_DATA_SWIZZLE_W { w: self }
    }
    #[doc = "Bits 14:15 - This field specifies how to swap the bytes fetched by the bus master interface"]
    #[inline(always)]
    pub fn input_data_swizzle(&mut self) -> INPUT_DATA_SWIZZLE_W {
        INPUT_DATA_SWIZZLE_W { w: self }
    }
    #[doc = "Bit 17 - Set this bit to 1 to make the hardware go into the DOTCLK mode, i"]
    #[inline(always)]
    pub fn dotclk_mode(&mut self) -> DOTCLK_MODE_W {
        DOTCLK_MODE_W { w: self }
    }
    #[doc = "Bit 19 - When this bit is 0, it means that LCDIF will stop the block operation and turn off the RUN bit after the amount of data indicated by the LCDIF_TRANSFER_COUNT register has been transferred out"]
    #[inline(always)]
    pub fn bypass_count(&mut self) -> BYPASS_COUNT_W {
        BYPASS_COUNT_W { w: self }
    }
    #[doc = "Bits 21:25 - The data to be transmitted is shifted left or right by this number of bits."]
    #[inline(always)]
    pub fn shift_num_bits(&mut self) -> SHIFT_NUM_BITS_W {
        SHIFT_NUM_BITS_W { w: self }
    }
    #[doc = "Bit 26 - Use this bit to determine the direction of shift of transmit data."]
    #[inline(always)]
    pub fn data_shift_dir(&mut self) -> DATA_SHIFT_DIR_W {
        DATA_SHIFT_DIR_W { w: self }
    }
    #[doc = "Bit 30 - This bit must be set to zero for normal operation"]
    #[inline(always)]
    pub fn clkgate(&mut self) -> CLKGATE_W {
        CLKGATE_W { w: self }
    }
    #[doc = "Bit 31 - This bit must be set to zero to enable normal operation of the LCDIF"]
    #[inline(always)]
    pub fn sftrst(&mut self) -> SFTRST_W {
        SFTRST_W { w: self }
    }
}
