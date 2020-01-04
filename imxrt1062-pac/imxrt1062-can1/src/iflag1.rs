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
#[doc = "If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BUF4TO0I_A {
    #[doc = "0: No such occurrence"]
    BUF4TO0I_0 = 0,
    #[doc = "1: Corresponding MB completed transmission/reception"]
    BUF4TO0I_1 = 1,
}
impl From<BUF4TO0I_A> for u8 {
    #[inline(always)]
    fn from(variant: BUF4TO0I_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BUF4TO0I`"]
pub type BUF4TO0I_R = crate::R<u8, BUF4TO0I_A>;
impl BUF4TO0I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BUF4TO0I_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BUF4TO0I_A::BUF4TO0I_0),
            1 => Val(BUF4TO0I_A::BUF4TO0I_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUF4TO0I_0`"]
    #[inline(always)]
    pub fn is_buf4to0i_0(&self) -> bool {
        *self == BUF4TO0I_A::BUF4TO0I_0
    }
    #[doc = "Checks if the value of the field is `BUF4TO0I_1`"]
    #[inline(always)]
    pub fn is_buf4to0i_1(&self) -> bool {
        *self == BUF4TO0I_A::BUF4TO0I_1
    }
}
#[doc = "Write proxy for field `BUF4TO0I`"]
pub struct BUF4TO0I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF4TO0I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF4TO0I_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn buf4to0i_0(self) -> &'a mut W {
        self.variant(BUF4TO0I_A::BUF4TO0I_0)
    }
    #[doc = "Corresponding MB completed transmission/reception"]
    #[inline(always)]
    pub fn buf4to0i_1(self) -> &'a mut W {
        self.variant(BUF4TO0I_A::BUF4TO0I_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF5I_A {
    #[doc = "0: No such occurrence"]
    BUF5I_0 = 0,
    #[doc = "1: MB5 completed transmission/reception or frames available in the FIFO"]
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
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn buf5i_0(self) -> &'a mut W {
        self.variant(BUF5I_A::BUF5I_0)
    }
    #[doc = "MB5 completed transmission/reception or frames available in the FIFO"]
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
#[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF6I_A {
    #[doc = "0: No such occurrence"]
    BUF6I_0 = 0,
    #[doc = "1: MB6 completed transmission/reception or FIFO almost full"]
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
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn buf6i_0(self) -> &'a mut W {
        self.variant(BUF6I_A::BUF6I_0)
    }
    #[doc = "MB6 completed transmission/reception or FIFO almost full"]
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
#[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF7I_A {
    #[doc = "0: No such occurrence"]
    BUF7I_0 = 0,
    #[doc = "1: MB7 completed transmission/reception or FIFO overflow"]
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
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn buf7i_0(self) -> &'a mut W {
        self.variant(BUF7I_A::BUF7I_0)
    }
    #[doc = "MB7 completed transmission/reception or FIFO overflow"]
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
#[doc = "Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum BUF31TO8I_A {
    #[doc = "0: No such occurrence"]
    BUF31TO8I_0 = 0,
    #[doc = "1: The corresponding MB has successfully completed transmission or reception"]
    BUF31TO8I_1 = 1,
}
impl From<BUF31TO8I_A> for u32 {
    #[inline(always)]
    fn from(variant: BUF31TO8I_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BUF31TO8I`"]
pub type BUF31TO8I_R = crate::R<u32, BUF31TO8I_A>;
impl BUF31TO8I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, BUF31TO8I_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BUF31TO8I_A::BUF31TO8I_0),
            1 => Val(BUF31TO8I_A::BUF31TO8I_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUF31TO8I_0`"]
    #[inline(always)]
    pub fn is_buf31to8i_0(&self) -> bool {
        *self == BUF31TO8I_A::BUF31TO8I_0
    }
    #[doc = "Checks if the value of the field is `BUF31TO8I_1`"]
    #[inline(always)]
    pub fn is_buf31to8i_1(&self) -> bool {
        *self == BUF31TO8I_A::BUF31TO8I_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I`"]
pub struct BUF31TO8I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn buf31to8i_0(self) -> &'a mut W {
        self.variant(BUF31TO8I_A::BUF31TO8I_0)
    }
    #[doc = "The corresponding MB has successfully completed transmission or reception"]
    #[inline(always)]
    pub fn buf31to8i_1(self) -> &'a mut W {
        self.variant(BUF31TO8I_A::BUF31TO8I_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4"]
    #[inline(always)]
    pub fn buf4to0i(&self) -> BUF4TO0I_R {
        BUF4TO0I_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB5"]
    #[inline(always)]
    pub fn buf5i(&self) -> BUF5I_R {
        BUF5I_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB6"]
    #[inline(always)]
    pub fn buf6i(&self) -> BUF6I_R {
        BUF6I_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB7"]
    #[inline(always)]
    pub fn buf7i(&self) -> BUF7I_R {
        BUF7I_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt."]
    #[inline(always)]
    pub fn buf31to8i(&self) -> BUF31TO8I_R {
        BUF31TO8I_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:4 - If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4"]
    #[inline(always)]
    pub fn buf4to0i(&mut self) -> BUF4TO0I_W {
        BUF4TO0I_W { w: self }
    }
    #[doc = "Bit 5 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB5"]
    #[inline(always)]
    pub fn buf5i(&mut self) -> BUF5I_W {
        BUF5I_W { w: self }
    }
    #[doc = "Bit 6 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB6"]
    #[inline(always)]
    pub fn buf6i(&mut self) -> BUF6I_W {
        BUF6I_W { w: self }
    }
    #[doc = "Bit 7 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB7"]
    #[inline(always)]
    pub fn buf7i(&mut self) -> BUF7I_W {
        BUF7I_W { w: self }
    }
    #[doc = "Bits 8:31 - Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt."]
    #[inline(always)]
    pub fn buf31to8i(&mut self) -> BUF31TO8I_W {
        BUF31TO8I_W { w: self }
    }
}
