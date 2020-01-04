#[doc = "Reader of register DBICR0"]
pub type R = crate::R<u32, super::DBICR0>;
#[doc = "Writer for register DBICR0"]
pub type W = crate::W<u32, super::DBICR0>;
#[doc = "Register DBICR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DBICR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    #[doc = "0: 8bit"]
    PS_0 = 0,
    #[doc = "1: 16bit"]
    PS_1 = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<bool, PS_A>;
impl PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::PS_0,
            true => PS_A::PS_1,
        }
    }
    #[doc = "Checks if the value of the field is `PS_0`"]
    #[inline(always)]
    pub fn is_ps_0(&self) -> bool {
        *self == PS_A::PS_0
    }
    #[doc = "Checks if the value of the field is `PS_1`"]
    #[inline(always)]
    pub fn is_ps_1(&self) -> bool {
        *self == PS_A::PS_1
    }
}
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "8bit"]
    #[inline(always)]
    pub fn ps_0(self) -> &'a mut W {
        self.variant(PS_A::PS_0)
    }
    #[doc = "16bit"]
    #[inline(always)]
    pub fn ps_1(self) -> &'a mut W {
        self.variant(PS_A::PS_1)
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
#[doc = "Burst Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BL_A {
    #[doc = "0: 1"]
    BL_0 = 0,
    #[doc = "1: 2"]
    BL_1 = 1,
    #[doc = "2: 4"]
    BL_2 = 2,
    #[doc = "3: 8"]
    BL_3 = 3,
    #[doc = "4: 16"]
    BL_4 = 4,
    #[doc = "5: 32"]
    BL_5 = 5,
    #[doc = "6: 64"]
    BL_6 = 6,
    #[doc = "7: 64"]
    BL_7 = 7,
}
impl From<BL_A> for u8 {
    #[inline(always)]
    fn from(variant: BL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BL`"]
pub type BL_R = crate::R<u8, BL_A>;
impl BL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BL_A {
        match self.bits {
            0 => BL_A::BL_0,
            1 => BL_A::BL_1,
            2 => BL_A::BL_2,
            3 => BL_A::BL_3,
            4 => BL_A::BL_4,
            5 => BL_A::BL_5,
            6 => BL_A::BL_6,
            7 => BL_A::BL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BL_0`"]
    #[inline(always)]
    pub fn is_bl_0(&self) -> bool {
        *self == BL_A::BL_0
    }
    #[doc = "Checks if the value of the field is `BL_1`"]
    #[inline(always)]
    pub fn is_bl_1(&self) -> bool {
        *self == BL_A::BL_1
    }
    #[doc = "Checks if the value of the field is `BL_2`"]
    #[inline(always)]
    pub fn is_bl_2(&self) -> bool {
        *self == BL_A::BL_2
    }
    #[doc = "Checks if the value of the field is `BL_3`"]
    #[inline(always)]
    pub fn is_bl_3(&self) -> bool {
        *self == BL_A::BL_3
    }
    #[doc = "Checks if the value of the field is `BL_4`"]
    #[inline(always)]
    pub fn is_bl_4(&self) -> bool {
        *self == BL_A::BL_4
    }
    #[doc = "Checks if the value of the field is `BL_5`"]
    #[inline(always)]
    pub fn is_bl_5(&self) -> bool {
        *self == BL_A::BL_5
    }
    #[doc = "Checks if the value of the field is `BL_6`"]
    #[inline(always)]
    pub fn is_bl_6(&self) -> bool {
        *self == BL_A::BL_6
    }
    #[doc = "Checks if the value of the field is `BL_7`"]
    #[inline(always)]
    pub fn is_bl_7(&self) -> bool {
        *self == BL_A::BL_7
    }
}
#[doc = "Write proxy for field `BL`"]
pub struct BL_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn bl_0(self) -> &'a mut W {
        self.variant(BL_A::BL_0)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn bl_1(self) -> &'a mut W {
        self.variant(BL_A::BL_1)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn bl_2(self) -> &'a mut W {
        self.variant(BL_A::BL_2)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn bl_3(self) -> &'a mut W {
        self.variant(BL_A::BL_3)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn bl_4(self) -> &'a mut W {
        self.variant(BL_A::BL_4)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn bl_5(self) -> &'a mut W {
        self.variant(BL_A::BL_5)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn bl_6(self) -> &'a mut W {
        self.variant(BL_A::BL_6)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn bl_7(self) -> &'a mut W {
        self.variant(BL_A::BL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Column Address bit width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COL_A {
    #[doc = "0: 12 Bits"]
    COL_0 = 0,
    #[doc = "1: 11 Bits"]
    COL_1 = 1,
    #[doc = "2: 10 Bits"]
    COL_2 = 2,
    #[doc = "3: 9 Bits"]
    COL_3 = 3,
    #[doc = "4: 8 Bits"]
    COL_4 = 4,
    #[doc = "5: 7 Bits"]
    COL_5 = 5,
    #[doc = "6: 6 Bits"]
    COL_6 = 6,
    #[doc = "7: 5 Bits"]
    COL_7 = 7,
    #[doc = "8: 4 Bits"]
    COL_8 = 8,
    #[doc = "9: 3 Bits"]
    COL_9 = 9,
    #[doc = "10: 2 Bits"]
    COL_10 = 10,
    #[doc = "11: 12 Bits"]
    COL_11 = 11,
    #[doc = "12: 12 Bits"]
    COL_12 = 12,
    #[doc = "13: 12 Bits"]
    COL_13 = 13,
    #[doc = "14: 12 Bits"]
    COL_14 = 14,
    #[doc = "15: 12 Bits"]
    COL_15 = 15,
}
impl From<COL_A> for u8 {
    #[inline(always)]
    fn from(variant: COL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COL`"]
pub type COL_R = crate::R<u8, COL_A>;
impl COL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COL_A {
        match self.bits {
            0 => COL_A::COL_0,
            1 => COL_A::COL_1,
            2 => COL_A::COL_2,
            3 => COL_A::COL_3,
            4 => COL_A::COL_4,
            5 => COL_A::COL_5,
            6 => COL_A::COL_6,
            7 => COL_A::COL_7,
            8 => COL_A::COL_8,
            9 => COL_A::COL_9,
            10 => COL_A::COL_10,
            11 => COL_A::COL_11,
            12 => COL_A::COL_12,
            13 => COL_A::COL_13,
            14 => COL_A::COL_14,
            15 => COL_A::COL_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COL_0`"]
    #[inline(always)]
    pub fn is_col_0(&self) -> bool {
        *self == COL_A::COL_0
    }
    #[doc = "Checks if the value of the field is `COL_1`"]
    #[inline(always)]
    pub fn is_col_1(&self) -> bool {
        *self == COL_A::COL_1
    }
    #[doc = "Checks if the value of the field is `COL_2`"]
    #[inline(always)]
    pub fn is_col_2(&self) -> bool {
        *self == COL_A::COL_2
    }
    #[doc = "Checks if the value of the field is `COL_3`"]
    #[inline(always)]
    pub fn is_col_3(&self) -> bool {
        *self == COL_A::COL_3
    }
    #[doc = "Checks if the value of the field is `COL_4`"]
    #[inline(always)]
    pub fn is_col_4(&self) -> bool {
        *self == COL_A::COL_4
    }
    #[doc = "Checks if the value of the field is `COL_5`"]
    #[inline(always)]
    pub fn is_col_5(&self) -> bool {
        *self == COL_A::COL_5
    }
    #[doc = "Checks if the value of the field is `COL_6`"]
    #[inline(always)]
    pub fn is_col_6(&self) -> bool {
        *self == COL_A::COL_6
    }
    #[doc = "Checks if the value of the field is `COL_7`"]
    #[inline(always)]
    pub fn is_col_7(&self) -> bool {
        *self == COL_A::COL_7
    }
    #[doc = "Checks if the value of the field is `COL_8`"]
    #[inline(always)]
    pub fn is_col_8(&self) -> bool {
        *self == COL_A::COL_8
    }
    #[doc = "Checks if the value of the field is `COL_9`"]
    #[inline(always)]
    pub fn is_col_9(&self) -> bool {
        *self == COL_A::COL_9
    }
    #[doc = "Checks if the value of the field is `COL_10`"]
    #[inline(always)]
    pub fn is_col_10(&self) -> bool {
        *self == COL_A::COL_10
    }
    #[doc = "Checks if the value of the field is `COL_11`"]
    #[inline(always)]
    pub fn is_col_11(&self) -> bool {
        *self == COL_A::COL_11
    }
    #[doc = "Checks if the value of the field is `COL_12`"]
    #[inline(always)]
    pub fn is_col_12(&self) -> bool {
        *self == COL_A::COL_12
    }
    #[doc = "Checks if the value of the field is `COL_13`"]
    #[inline(always)]
    pub fn is_col_13(&self) -> bool {
        *self == COL_A::COL_13
    }
    #[doc = "Checks if the value of the field is `COL_14`"]
    #[inline(always)]
    pub fn is_col_14(&self) -> bool {
        *self == COL_A::COL_14
    }
    #[doc = "Checks if the value of the field is `COL_15`"]
    #[inline(always)]
    pub fn is_col_15(&self) -> bool {
        *self == COL_A::COL_15
    }
}
#[doc = "Write proxy for field `COL`"]
pub struct COL_W<'a> {
    w: &'a mut W,
}
impl<'a> COL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "12 Bits"]
    #[inline(always)]
    pub fn col_0(self) -> &'a mut W {
        self.variant(COL_A::COL_0)
    }
    #[doc = "11 Bits"]
    #[inline(always)]
    pub fn col_1(self) -> &'a mut W {
        self.variant(COL_A::COL_1)
    }
    #[doc = "10 Bits"]
    #[inline(always)]
    pub fn col_2(self) -> &'a mut W {
        self.variant(COL_A::COL_2)
    }
    #[doc = "9 Bits"]
    #[inline(always)]
    pub fn col_3(self) -> &'a mut W {
        self.variant(COL_A::COL_3)
    }
    #[doc = "8 Bits"]
    #[inline(always)]
    pub fn col_4(self) -> &'a mut W {
        self.variant(COL_A::COL_4)
    }
    #[doc = "7 Bits"]
    #[inline(always)]
    pub fn col_5(self) -> &'a mut W {
        self.variant(COL_A::COL_5)
    }
    #[doc = "6 Bits"]
    #[inline(always)]
    pub fn col_6(self) -> &'a mut W {
        self.variant(COL_A::COL_6)
    }
    #[doc = "5 Bits"]
    #[inline(always)]
    pub fn col_7(self) -> &'a mut W {
        self.variant(COL_A::COL_7)
    }
    #[doc = "4 Bits"]
    #[inline(always)]
    pub fn col_8(self) -> &'a mut W {
        self.variant(COL_A::COL_8)
    }
    #[doc = "3 Bits"]
    #[inline(always)]
    pub fn col_9(self) -> &'a mut W {
        self.variant(COL_A::COL_9)
    }
    #[doc = "2 Bits"]
    #[inline(always)]
    pub fn col_10(self) -> &'a mut W {
        self.variant(COL_A::COL_10)
    }
    #[doc = "12 Bits"]
    #[inline(always)]
    pub fn col_11(self) -> &'a mut W {
        self.variant(COL_A::COL_11)
    }
    #[doc = "12 Bits"]
    #[inline(always)]
    pub fn col_12(self) -> &'a mut W {
        self.variant(COL_A::COL_12)
    }
    #[doc = "12 Bits"]
    #[inline(always)]
    pub fn col_13(self) -> &'a mut W {
        self.variant(COL_A::COL_13)
    }
    #[doc = "12 Bits"]
    #[inline(always)]
    pub fn col_14(self) -> &'a mut W {
        self.variant(COL_A::COL_14)
    }
    #[doc = "12 Bits"]
    #[inline(always)]
    pub fn col_15(self) -> &'a mut W {
        self.variant(COL_A::COL_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port Size"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Burst Length"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - Column Address bit width"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Port Size"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bits 4:6 - Burst Length"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W {
        BL_W { w: self }
    }
    #[doc = "Bits 12:15 - Column Address bit width"]
    #[inline(always)]
    pub fn col(&mut self) -> COL_W {
        COL_W { w: self }
    }
}
