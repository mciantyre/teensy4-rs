#[doc = "Reader of register IFLAG1"]
pub type R = crate::R<u32, super::IFLAG1>;
#[doc = "Writer for register IFLAG1"]
pub type W = crate::W<u32, super::IFLAG1>;
#[doc = "Register IFLAG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IFLAG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Buffer MB0 Interrupt Or Clear Legacy FIFO bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF0I_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception when CAN_MCR\\[RFEN\\]=0."]
    BUF0I_0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception when CAN_MCR\\[RFEN\\]=0."]
    BUF0I_1 = 1,
}
impl From<BUF0I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF0I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUF0I`"]
pub type BUF0I_R = crate::R<bool, BUF0I_A>;
impl BUF0I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF0I_A {
        match self.bits {
            false => BUF0I_A::BUF0I_0,
            true => BUF0I_A::BUF0I_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUF0I_0`"]
    #[inline(always)]
    pub fn is_buf0i_0(&self) -> bool {
        *self == BUF0I_A::BUF0I_0
    }
    #[doc = "Checks if the value of the field is `BUF0I_1`"]
    #[inline(always)]
    pub fn is_buf0i_1(&self) -> bool {
        *self == BUF0I_A::BUF0I_1
    }
}
#[doc = "Write proxy for field `BUF0I`"]
pub struct BUF0I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF0I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF0I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when CAN_MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn buf0i_0(self) -> &'a mut W {
        self.variant(BUF0I_A::BUF0I_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when CAN_MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn buf0i_1(self) -> &'a mut W {
        self.variant(BUF0I_A::BUF0I_1)
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
#[doc = "Reader of field `BUF4TO1I`"]
pub type BUF4TO1I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUF4TO1I`"]
pub struct BUF4TO1I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF4TO1I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Buffer MB5 Interrupt Or \"Frames available in Legacy Rx FIFO\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF5I_A {
    #[doc = "0: No occurrence of MB5 completing transmission/reception when CAN_MCR\\[RFEN\\]=0, or of frame(s) available in the Legacy FIFO, when CAN_MCR\\[RFEN\\]=1"]
    BUF5I_0 = 0,
    #[doc = "1: MB5 completed transmission/reception when CAN_MCR\\[RFEN\\]=0, or frame(s) available in the Legacy Rx FIFO when CAN_MCR\\[RFEN\\]=1. It generates a DMA request in case of CAN_MCR\\[RFEN\\]
and CAN_MCR\\[DMA\\]
are enabled."]
    BUF5I_1 = 1,
}
impl From<BUF5I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF5I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUF5I`"]
pub type BUF5I_R = crate::R<bool, BUF5I_A>;
impl BUF5I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF5I_A {
        match self.bits {
            false => BUF5I_A::BUF5I_0,
            true => BUF5I_A::BUF5I_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUF5I_0`"]
    #[inline(always)]
    pub fn is_buf5i_0(&self) -> bool {
        *self == BUF5I_A::BUF5I_0
    }
    #[doc = "Checks if the value of the field is `BUF5I_1`"]
    #[inline(always)]
    pub fn is_buf5i_1(&self) -> bool {
        *self == BUF5I_A::BUF5I_1
    }
}
#[doc = "Write proxy for field `BUF5I`"]
pub struct BUF5I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF5I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF5I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No occurrence of MB5 completing transmission/reception when CAN_MCR\\[RFEN\\]=0, or of frame(s) available in the Legacy FIFO, when CAN_MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn buf5i_0(self) -> &'a mut W {
        self.variant(BUF5I_A::BUF5I_0)
    }
    #[doc = "MB5 completed transmission/reception when CAN_MCR\\[RFEN\\]=0, or frame(s) available in the Legacy Rx FIFO when CAN_MCR\\[RFEN\\]=1. It generates a DMA request in case of CAN_MCR\\[RFEN\\]
and CAN_MCR\\[DMA\\]
are enabled."]
    #[inline(always)]
    pub fn buf5i_1(self) -> &'a mut W {
        self.variant(BUF5I_A::BUF5I_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Buffer MB6 Interrupt Or \"Legacy Rx FIFO Warning\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF6I_A {
    #[doc = "0: No occurrence of MB6 completing transmission/reception when CAN_MCR\\[RFEN\\]=0, or of Legacy Rx FIFO almost full when CAN_MCR\\[RFEN\\]=1"]
    BUF6I_0 = 0,
    #[doc = "1: MB6 completed transmission/reception when CAN_MCR\\[RFEN\\]=0, or Legacy Rx FIFO almost full when CAN_MCR\\[RFEN\\]=1"]
    BUF6I_1 = 1,
}
impl From<BUF6I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF6I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUF6I`"]
pub type BUF6I_R = crate::R<bool, BUF6I_A>;
impl BUF6I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF6I_A {
        match self.bits {
            false => BUF6I_A::BUF6I_0,
            true => BUF6I_A::BUF6I_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUF6I_0`"]
    #[inline(always)]
    pub fn is_buf6i_0(&self) -> bool {
        *self == BUF6I_A::BUF6I_0
    }
    #[doc = "Checks if the value of the field is `BUF6I_1`"]
    #[inline(always)]
    pub fn is_buf6i_1(&self) -> bool {
        *self == BUF6I_A::BUF6I_1
    }
}
#[doc = "Write proxy for field `BUF6I`"]
pub struct BUF6I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF6I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF6I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No occurrence of MB6 completing transmission/reception when CAN_MCR\\[RFEN\\]=0, or of Legacy Rx FIFO almost full when CAN_MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn buf6i_0(self) -> &'a mut W {
        self.variant(BUF6I_A::BUF6I_0)
    }
    #[doc = "MB6 completed transmission/reception when CAN_MCR\\[RFEN\\]=0, or Legacy Rx FIFO almost full when CAN_MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn buf6i_1(self) -> &'a mut W {
        self.variant(BUF6I_A::BUF6I_1)
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
#[doc = "Buffer MB7 Interrupt Or \"Legacy Rx FIFO Overflow\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF7I_A {
    #[doc = "0: No occurrence of MB7 completing transmission/reception when CAN_MCR\\[RFEN\\]=0, or of Legacy Rx FIFO overflow when CAN_MCR\\[RFEN\\]=1"]
    BUF7I_0 = 0,
    #[doc = "1: MB7 completed transmission/reception when CAN_MCR\\[RFEN\\]=0, or Legacy Rx FIFO overflow when CAN_MCR\\[RFEN\\]=1"]
    BUF7I_1 = 1,
}
impl From<BUF7I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF7I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUF7I`"]
pub type BUF7I_R = crate::R<bool, BUF7I_A>;
impl BUF7I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF7I_A {
        match self.bits {
            false => BUF7I_A::BUF7I_0,
            true => BUF7I_A::BUF7I_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUF7I_0`"]
    #[inline(always)]
    pub fn is_buf7i_0(&self) -> bool {
        *self == BUF7I_A::BUF7I_0
    }
    #[doc = "Checks if the value of the field is `BUF7I_1`"]
    #[inline(always)]
    pub fn is_buf7i_1(&self) -> bool {
        *self == BUF7I_A::BUF7I_1
    }
}
#[doc = "Write proxy for field `BUF7I`"]
pub struct BUF7I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF7I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF7I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No occurrence of MB7 completing transmission/reception when CAN_MCR\\[RFEN\\]=0, or of Legacy Rx FIFO overflow when CAN_MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn buf7i_0(self) -> &'a mut W {
        self.variant(BUF7I_A::BUF7I_0)
    }
    #[doc = "MB7 completed transmission/reception when CAN_MCR\\[RFEN\\]=0, or Legacy Rx FIFO overflow when CAN_MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn buf7i_1(self) -> &'a mut W {
        self.variant(BUF7I_A::BUF7I_1)
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
#[doc = "Reader of field `BUF31TO8I`"]
pub type BUF31TO8I_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BUF31TO8I`"]
pub struct BUF31TO8I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Buffer MB0 Interrupt Or Clear Legacy FIFO bit"]
    #[inline(always)]
    pub fn buf0i(&self) -> BUF0I_R {
        BUF0I_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i(&self) -> BUF4TO1I_R {
        BUF4TO1I_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Buffer MB5 Interrupt Or \"Frames available in Legacy Rx FIFO\""]
    #[inline(always)]
    pub fn buf5i(&self) -> BUF5I_R {
        BUF5I_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Buffer MB6 Interrupt Or \"Legacy Rx FIFO Warning\""]
    #[inline(always)]
    pub fn buf6i(&self) -> BUF6I_R {
        BUF6I_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Buffer MB7 Interrupt Or \"Legacy Rx FIFO Overflow\""]
    #[inline(always)]
    pub fn buf7i(&self) -> BUF7I_R {
        BUF7I_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i(&self) -> BUF31TO8I_R {
        BUF31TO8I_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer MB0 Interrupt Or Clear Legacy FIFO bit"]
    #[inline(always)]
    pub fn buf0i(&mut self) -> BUF0I_W {
        BUF0I_W { w: self }
    }
    #[doc = "Bits 1:4 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i(&mut self) -> BUF4TO1I_W {
        BUF4TO1I_W { w: self }
    }
    #[doc = "Bit 5 - Buffer MB5 Interrupt Or \"Frames available in Legacy Rx FIFO\""]
    #[inline(always)]
    pub fn buf5i(&mut self) -> BUF5I_W {
        BUF5I_W { w: self }
    }
    #[doc = "Bit 6 - Buffer MB6 Interrupt Or \"Legacy Rx FIFO Warning\""]
    #[inline(always)]
    pub fn buf6i(&mut self) -> BUF6I_W {
        BUF6I_W { w: self }
    }
    #[doc = "Bit 7 - Buffer MB7 Interrupt Or \"Legacy Rx FIFO Overflow\""]
    #[inline(always)]
    pub fn buf7i(&mut self) -> BUF7I_W {
        BUF7I_W { w: self }
    }
    #[doc = "Bits 8:31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i(&mut self) -> BUF31TO8I_W {
        BUF31TO8I_W { w: self }
    }
}
