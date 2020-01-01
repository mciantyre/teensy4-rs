#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Reader of field `IRQ_ENABLE`"]
pub type IRQ_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ_ENABLE`"]
pub struct IRQ_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_ENABLE_W<'a> {
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
#[doc = "Reader of field `NEXT_IRQ_ENABLE`"]
pub type NEXT_IRQ_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NEXT_IRQ_ENABLE`"]
pub struct NEXT_IRQ_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXT_IRQ_ENABLE_W<'a> {
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
#[doc = "Reader of field `ENABLE_LCD_HANDSHAKE`"]
pub type ENABLE_LCD_HANDSHAKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_LCD_HANDSHAKE`"]
pub struct ENABLE_LCD_HANDSHAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_LCD_HANDSHAKE_W<'a> {
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
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u8, u8>;
#[doc = "Indicates the clockwise rotation to be applied at the output buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ROTATE_A {
    #[doc = "0: ROT_0"]
    ROT_0 = 0,
    #[doc = "1: ROT_90"]
    ROT_90 = 1,
    #[doc = "2: ROT_180"]
    ROT_180 = 2,
    #[doc = "3: ROT_270"]
    ROT_270 = 3,
}
impl From<ROTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: ROTATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ROTATE`"]
pub type ROTATE_R = crate::R<u8, ROTATE_A>;
impl ROTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROTATE_A {
        match self.bits {
            0 => ROTATE_A::ROT_0,
            1 => ROTATE_A::ROT_90,
            2 => ROTATE_A::ROT_180,
            3 => ROTATE_A::ROT_270,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ROT_0`"]
    #[inline(always)]
    pub fn is_rot_0(&self) -> bool {
        *self == ROTATE_A::ROT_0
    }
    #[doc = "Checks if the value of the field is `ROT_90`"]
    #[inline(always)]
    pub fn is_rot_90(&self) -> bool {
        *self == ROTATE_A::ROT_90
    }
    #[doc = "Checks if the value of the field is `ROT_180`"]
    #[inline(always)]
    pub fn is_rot_180(&self) -> bool {
        *self == ROTATE_A::ROT_180
    }
    #[doc = "Checks if the value of the field is `ROT_270`"]
    #[inline(always)]
    pub fn is_rot_270(&self) -> bool {
        *self == ROTATE_A::ROT_270
    }
}
#[doc = "Write proxy for field `ROTATE`"]
pub struct ROTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROTATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROTATE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ROT_0"]
    #[inline(always)]
    pub fn rot_0(self) -> &'a mut W {
        self.variant(ROTATE_A::ROT_0)
    }
    #[doc = "ROT_90"]
    #[inline(always)]
    pub fn rot_90(self) -> &'a mut W {
        self.variant(ROTATE_A::ROT_90)
    }
    #[doc = "ROT_180"]
    #[inline(always)]
    pub fn rot_180(self) -> &'a mut W {
        self.variant(ROTATE_A::ROT_180)
    }
    #[doc = "ROT_270"]
    #[inline(always)]
    pub fn rot_270(self) -> &'a mut W {
        self.variant(ROTATE_A::ROT_270)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `HFLIP`"]
pub type HFLIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFLIP`"]
pub struct HFLIP_W<'a> {
    w: &'a mut W,
}
impl<'a> HFLIP_W<'a> {
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
#[doc = "Reader of field `VFLIP`"]
pub type VFLIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VFLIP`"]
pub struct VFLIP_W<'a> {
    w: &'a mut W,
}
impl<'a> VFLIP_W<'a> {
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
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u16, u16>;
#[doc = "Reader of field `ROT_POS`"]
pub type ROT_POS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROT_POS`"]
pub struct ROT_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> ROT_POS_W<'a> {
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
#[doc = "Select the block size to process.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCK_SIZE_A {
    #[doc = "0: Process 8x8 pixel blocks."]
    _8X8 = 0,
    #[doc = "1: Process 16x16 pixel blocks."]
    _16X16 = 1,
}
impl From<BLOCK_SIZE_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCK_SIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BLOCK_SIZE`"]
pub type BLOCK_SIZE_R = crate::R<bool, BLOCK_SIZE_A>;
impl BLOCK_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLOCK_SIZE_A {
        match self.bits {
            false => BLOCK_SIZE_A::_8X8,
            true => BLOCK_SIZE_A::_16X16,
        }
    }
    #[doc = "Checks if the value of the field is `_8X8`"]
    #[inline(always)]
    pub fn is_8x8(&self) -> bool {
        *self == BLOCK_SIZE_A::_8X8
    }
    #[doc = "Checks if the value of the field is `_16X16`"]
    #[inline(always)]
    pub fn is_16x16(&self) -> bool {
        *self == BLOCK_SIZE_A::_16X16
    }
}
#[doc = "Write proxy for field `BLOCK_SIZE`"]
pub struct BLOCK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLOCK_SIZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Process 8x8 pixel blocks."]
    #[inline(always)]
    pub fn _8x8(self) -> &'a mut W {
        self.variant(BLOCK_SIZE_A::_8X8)
    }
    #[doc = "Process 16x16 pixel blocks."]
    #[inline(always)]
    pub fn _16x16(self) -> &'a mut W {
        self.variant(BLOCK_SIZE_A::_16X16)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `RSVD3`"]
pub type RSVD3_R = crate::R<u8, u8>;
#[doc = "Reader of field `EN_REPEAT`"]
pub type EN_REPEAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_REPEAT`"]
pub struct EN_REPEAT_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_REPEAT_W<'a> {
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
#[doc = "Reader of field `RSVD4`"]
pub type RSVD4_R = crate::R<bool, bool>;
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
    #[doc = "Bit 0 - Enables PXP operation with specified parameters"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn irq_enable(&self) -> IRQ_ENABLE_R {
        IRQ_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Next command interrupt enable"]
    #[inline(always)]
    pub fn next_irq_enable(&self) -> NEXT_IRQ_ENABLE_R {
        NEXT_IRQ_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable handshake with LCD controller"]
    #[inline(always)]
    pub fn enable_lcd_handshake(&self) -> ENABLE_LCD_HANDSHAKE_R {
        ENABLE_LCD_HANDSHAKE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Indicates the clockwise rotation to be applied at the output buffer"]
    #[inline(always)]
    pub fn rotate(&self) -> ROTATE_R {
        ROTATE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    #[inline(always)]
    pub fn hflip(&self) -> HFLIP_R {
        HFLIP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    #[inline(always)]
    pub fn vflip(&self) -> VFLIP_R {
        VFLIP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:21 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bit 22 - This bit controls where rotation will occur in the PXP datapath"]
    #[inline(always)]
    pub fn rot_pos(&self) -> ROT_POS_R {
        ROT_POS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Select the block size to process."]
    #[inline(always)]
    pub fn block_size(&self) -> BLOCK_SIZE_R {
        BLOCK_SIZE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd3(&self) -> RSVD3_R {
        RSVD3_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Enable the PXP to run continuously"]
    #[inline(always)]
    pub fn en_repeat(&self) -> EN_REPEAT_R {
        EN_REPEAT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd4(&self) -> RSVD4_R {
        RSVD4_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This bit must be set to zero for normal operation"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Set this bit to zero to enable normal PXP operation"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables PXP operation with specified parameters"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn irq_enable(&mut self) -> IRQ_ENABLE_W {
        IRQ_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Next command interrupt enable"]
    #[inline(always)]
    pub fn next_irq_enable(&mut self) -> NEXT_IRQ_ENABLE_W {
        NEXT_IRQ_ENABLE_W { w: self }
    }
    #[doc = "Bit 4 - Enable handshake with LCD controller"]
    #[inline(always)]
    pub fn enable_lcd_handshake(&mut self) -> ENABLE_LCD_HANDSHAKE_W {
        ENABLE_LCD_HANDSHAKE_W { w: self }
    }
    #[doc = "Bits 8:9 - Indicates the clockwise rotation to be applied at the output buffer"]
    #[inline(always)]
    pub fn rotate(&mut self) -> ROTATE_W {
        ROTATE_W { w: self }
    }
    #[doc = "Bit 10 - Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    #[inline(always)]
    pub fn hflip(&mut self) -> HFLIP_W {
        HFLIP_W { w: self }
    }
    #[doc = "Bit 11 - Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    #[inline(always)]
    pub fn vflip(&mut self) -> VFLIP_W {
        VFLIP_W { w: self }
    }
    #[doc = "Bit 22 - This bit controls where rotation will occur in the PXP datapath"]
    #[inline(always)]
    pub fn rot_pos(&mut self) -> ROT_POS_W {
        ROT_POS_W { w: self }
    }
    #[doc = "Bit 23 - Select the block size to process."]
    #[inline(always)]
    pub fn block_size(&mut self) -> BLOCK_SIZE_W {
        BLOCK_SIZE_W { w: self }
    }
    #[doc = "Bit 28 - Enable the PXP to run continuously"]
    #[inline(always)]
    pub fn en_repeat(&mut self) -> EN_REPEAT_W {
        EN_REPEAT_W { w: self }
    }
    #[doc = "Bit 30 - This bit must be set to zero for normal operation"]
    #[inline(always)]
    pub fn clkgate(&mut self) -> CLKGATE_W {
        CLKGATE_W { w: self }
    }
    #[doc = "Bit 31 - Set this bit to zero to enable normal PXP operation"]
    #[inline(always)]
    pub fn sftrst(&mut self) -> SFTRST_W {
        SFTRST_W { w: self }
    }
}
