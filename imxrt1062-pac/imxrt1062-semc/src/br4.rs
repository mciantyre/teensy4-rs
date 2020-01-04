#[doc = "Reader of register BR4"]
pub type R = crate::R<u32, super::BR4>;
#[doc = "Writer for register BR4"]
pub type W = crate::W<u32, super::BR4>;
#[doc = "Register BR4 `reset()`'s with value 0x9e00_001a"]
impl crate::ResetValue for super::BR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x9e00_001a
    }
}
#[doc = "Reader of field `VLD`"]
pub type VLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLD`"]
pub struct VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> VLD_W<'a> {
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
#[doc = "Memory size\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MS_A {
    #[doc = "0: 4KB"]
    MS_0 = 0,
    #[doc = "1: 8KB"]
    MS_1 = 1,
    #[doc = "2: 16KB"]
    MS_2 = 2,
    #[doc = "3: 32KB"]
    MS_3 = 3,
    #[doc = "4: 64KB"]
    MS_4 = 4,
    #[doc = "5: 128KB"]
    MS_5 = 5,
    #[doc = "6: 256KB"]
    MS_6 = 6,
    #[doc = "7: 512KB"]
    MS_7 = 7,
    #[doc = "8: 1MB"]
    MS_8 = 8,
    #[doc = "9: 2MB"]
    MS_9 = 9,
    #[doc = "10: 4MB"]
    MS_10 = 10,
    #[doc = "11: 8MB"]
    MS_11 = 11,
    #[doc = "12: 16MB"]
    MS_12 = 12,
    #[doc = "13: 32MB"]
    MS_13 = 13,
    #[doc = "14: 64MB"]
    MS_14 = 14,
    #[doc = "15: 128MB"]
    MS_15 = 15,
    #[doc = "16: 256MB"]
    MS_16 = 16,
    #[doc = "17: 512MB"]
    MS_17 = 17,
    #[doc = "18: 1GB"]
    MS_18 = 18,
    #[doc = "19: 2GB"]
    MS_19 = 19,
    #[doc = "20: 4GB"]
    MS_20 = 20,
    #[doc = "21: 4GB"]
    MS_21 = 21,
    #[doc = "22: 4GB"]
    MS_22 = 22,
    #[doc = "23: 4GB"]
    MS_23 = 23,
    #[doc = "24: 4GB"]
    MS_24 = 24,
    #[doc = "25: 4GB"]
    MS_25 = 25,
    #[doc = "26: 4GB"]
    MS_26 = 26,
    #[doc = "27: 4GB"]
    MS_27 = 27,
    #[doc = "28: 4GB"]
    MS_28 = 28,
    #[doc = "29: 4GB"]
    MS_29 = 29,
    #[doc = "30: 4GB"]
    MS_30 = 30,
    #[doc = "31: 4GB"]
    MS_31 = 31,
}
impl From<MS_A> for u8 {
    #[inline(always)]
    fn from(variant: MS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MS`"]
pub type MS_R = crate::R<u8, MS_A>;
impl MS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MS_A {
        match self.bits {
            0 => MS_A::MS_0,
            1 => MS_A::MS_1,
            2 => MS_A::MS_2,
            3 => MS_A::MS_3,
            4 => MS_A::MS_4,
            5 => MS_A::MS_5,
            6 => MS_A::MS_6,
            7 => MS_A::MS_7,
            8 => MS_A::MS_8,
            9 => MS_A::MS_9,
            10 => MS_A::MS_10,
            11 => MS_A::MS_11,
            12 => MS_A::MS_12,
            13 => MS_A::MS_13,
            14 => MS_A::MS_14,
            15 => MS_A::MS_15,
            16 => MS_A::MS_16,
            17 => MS_A::MS_17,
            18 => MS_A::MS_18,
            19 => MS_A::MS_19,
            20 => MS_A::MS_20,
            21 => MS_A::MS_21,
            22 => MS_A::MS_22,
            23 => MS_A::MS_23,
            24 => MS_A::MS_24,
            25 => MS_A::MS_25,
            26 => MS_A::MS_26,
            27 => MS_A::MS_27,
            28 => MS_A::MS_28,
            29 => MS_A::MS_29,
            30 => MS_A::MS_30,
            31 => MS_A::MS_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MS_0`"]
    #[inline(always)]
    pub fn is_ms_0(&self) -> bool {
        *self == MS_A::MS_0
    }
    #[doc = "Checks if the value of the field is `MS_1`"]
    #[inline(always)]
    pub fn is_ms_1(&self) -> bool {
        *self == MS_A::MS_1
    }
    #[doc = "Checks if the value of the field is `MS_2`"]
    #[inline(always)]
    pub fn is_ms_2(&self) -> bool {
        *self == MS_A::MS_2
    }
    #[doc = "Checks if the value of the field is `MS_3`"]
    #[inline(always)]
    pub fn is_ms_3(&self) -> bool {
        *self == MS_A::MS_3
    }
    #[doc = "Checks if the value of the field is `MS_4`"]
    #[inline(always)]
    pub fn is_ms_4(&self) -> bool {
        *self == MS_A::MS_4
    }
    #[doc = "Checks if the value of the field is `MS_5`"]
    #[inline(always)]
    pub fn is_ms_5(&self) -> bool {
        *self == MS_A::MS_5
    }
    #[doc = "Checks if the value of the field is `MS_6`"]
    #[inline(always)]
    pub fn is_ms_6(&self) -> bool {
        *self == MS_A::MS_6
    }
    #[doc = "Checks if the value of the field is `MS_7`"]
    #[inline(always)]
    pub fn is_ms_7(&self) -> bool {
        *self == MS_A::MS_7
    }
    #[doc = "Checks if the value of the field is `MS_8`"]
    #[inline(always)]
    pub fn is_ms_8(&self) -> bool {
        *self == MS_A::MS_8
    }
    #[doc = "Checks if the value of the field is `MS_9`"]
    #[inline(always)]
    pub fn is_ms_9(&self) -> bool {
        *self == MS_A::MS_9
    }
    #[doc = "Checks if the value of the field is `MS_10`"]
    #[inline(always)]
    pub fn is_ms_10(&self) -> bool {
        *self == MS_A::MS_10
    }
    #[doc = "Checks if the value of the field is `MS_11`"]
    #[inline(always)]
    pub fn is_ms_11(&self) -> bool {
        *self == MS_A::MS_11
    }
    #[doc = "Checks if the value of the field is `MS_12`"]
    #[inline(always)]
    pub fn is_ms_12(&self) -> bool {
        *self == MS_A::MS_12
    }
    #[doc = "Checks if the value of the field is `MS_13`"]
    #[inline(always)]
    pub fn is_ms_13(&self) -> bool {
        *self == MS_A::MS_13
    }
    #[doc = "Checks if the value of the field is `MS_14`"]
    #[inline(always)]
    pub fn is_ms_14(&self) -> bool {
        *self == MS_A::MS_14
    }
    #[doc = "Checks if the value of the field is `MS_15`"]
    #[inline(always)]
    pub fn is_ms_15(&self) -> bool {
        *self == MS_A::MS_15
    }
    #[doc = "Checks if the value of the field is `MS_16`"]
    #[inline(always)]
    pub fn is_ms_16(&self) -> bool {
        *self == MS_A::MS_16
    }
    #[doc = "Checks if the value of the field is `MS_17`"]
    #[inline(always)]
    pub fn is_ms_17(&self) -> bool {
        *self == MS_A::MS_17
    }
    #[doc = "Checks if the value of the field is `MS_18`"]
    #[inline(always)]
    pub fn is_ms_18(&self) -> bool {
        *self == MS_A::MS_18
    }
    #[doc = "Checks if the value of the field is `MS_19`"]
    #[inline(always)]
    pub fn is_ms_19(&self) -> bool {
        *self == MS_A::MS_19
    }
    #[doc = "Checks if the value of the field is `MS_20`"]
    #[inline(always)]
    pub fn is_ms_20(&self) -> bool {
        *self == MS_A::MS_20
    }
    #[doc = "Checks if the value of the field is `MS_21`"]
    #[inline(always)]
    pub fn is_ms_21(&self) -> bool {
        *self == MS_A::MS_21
    }
    #[doc = "Checks if the value of the field is `MS_22`"]
    #[inline(always)]
    pub fn is_ms_22(&self) -> bool {
        *self == MS_A::MS_22
    }
    #[doc = "Checks if the value of the field is `MS_23`"]
    #[inline(always)]
    pub fn is_ms_23(&self) -> bool {
        *self == MS_A::MS_23
    }
    #[doc = "Checks if the value of the field is `MS_24`"]
    #[inline(always)]
    pub fn is_ms_24(&self) -> bool {
        *self == MS_A::MS_24
    }
    #[doc = "Checks if the value of the field is `MS_25`"]
    #[inline(always)]
    pub fn is_ms_25(&self) -> bool {
        *self == MS_A::MS_25
    }
    #[doc = "Checks if the value of the field is `MS_26`"]
    #[inline(always)]
    pub fn is_ms_26(&self) -> bool {
        *self == MS_A::MS_26
    }
    #[doc = "Checks if the value of the field is `MS_27`"]
    #[inline(always)]
    pub fn is_ms_27(&self) -> bool {
        *self == MS_A::MS_27
    }
    #[doc = "Checks if the value of the field is `MS_28`"]
    #[inline(always)]
    pub fn is_ms_28(&self) -> bool {
        *self == MS_A::MS_28
    }
    #[doc = "Checks if the value of the field is `MS_29`"]
    #[inline(always)]
    pub fn is_ms_29(&self) -> bool {
        *self == MS_A::MS_29
    }
    #[doc = "Checks if the value of the field is `MS_30`"]
    #[inline(always)]
    pub fn is_ms_30(&self) -> bool {
        *self == MS_A::MS_30
    }
    #[doc = "Checks if the value of the field is `MS_31`"]
    #[inline(always)]
    pub fn is_ms_31(&self) -> bool {
        *self == MS_A::MS_31
    }
}
#[doc = "Write proxy for field `MS`"]
pub struct MS_W<'a> {
    w: &'a mut W,
}
impl<'a> MS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4KB"]
    #[inline(always)]
    pub fn ms_0(self) -> &'a mut W {
        self.variant(MS_A::MS_0)
    }
    #[doc = "8KB"]
    #[inline(always)]
    pub fn ms_1(self) -> &'a mut W {
        self.variant(MS_A::MS_1)
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn ms_2(self) -> &'a mut W {
        self.variant(MS_A::MS_2)
    }
    #[doc = "32KB"]
    #[inline(always)]
    pub fn ms_3(self) -> &'a mut W {
        self.variant(MS_A::MS_3)
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn ms_4(self) -> &'a mut W {
        self.variant(MS_A::MS_4)
    }
    #[doc = "128KB"]
    #[inline(always)]
    pub fn ms_5(self) -> &'a mut W {
        self.variant(MS_A::MS_5)
    }
    #[doc = "256KB"]
    #[inline(always)]
    pub fn ms_6(self) -> &'a mut W {
        self.variant(MS_A::MS_6)
    }
    #[doc = "512KB"]
    #[inline(always)]
    pub fn ms_7(self) -> &'a mut W {
        self.variant(MS_A::MS_7)
    }
    #[doc = "1MB"]
    #[inline(always)]
    pub fn ms_8(self) -> &'a mut W {
        self.variant(MS_A::MS_8)
    }
    #[doc = "2MB"]
    #[inline(always)]
    pub fn ms_9(self) -> &'a mut W {
        self.variant(MS_A::MS_9)
    }
    #[doc = "4MB"]
    #[inline(always)]
    pub fn ms_10(self) -> &'a mut W {
        self.variant(MS_A::MS_10)
    }
    #[doc = "8MB"]
    #[inline(always)]
    pub fn ms_11(self) -> &'a mut W {
        self.variant(MS_A::MS_11)
    }
    #[doc = "16MB"]
    #[inline(always)]
    pub fn ms_12(self) -> &'a mut W {
        self.variant(MS_A::MS_12)
    }
    #[doc = "32MB"]
    #[inline(always)]
    pub fn ms_13(self) -> &'a mut W {
        self.variant(MS_A::MS_13)
    }
    #[doc = "64MB"]
    #[inline(always)]
    pub fn ms_14(self) -> &'a mut W {
        self.variant(MS_A::MS_14)
    }
    #[doc = "128MB"]
    #[inline(always)]
    pub fn ms_15(self) -> &'a mut W {
        self.variant(MS_A::MS_15)
    }
    #[doc = "256MB"]
    #[inline(always)]
    pub fn ms_16(self) -> &'a mut W {
        self.variant(MS_A::MS_16)
    }
    #[doc = "512MB"]
    #[inline(always)]
    pub fn ms_17(self) -> &'a mut W {
        self.variant(MS_A::MS_17)
    }
    #[doc = "1GB"]
    #[inline(always)]
    pub fn ms_18(self) -> &'a mut W {
        self.variant(MS_A::MS_18)
    }
    #[doc = "2GB"]
    #[inline(always)]
    pub fn ms_19(self) -> &'a mut W {
        self.variant(MS_A::MS_19)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_20(self) -> &'a mut W {
        self.variant(MS_A::MS_20)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_21(self) -> &'a mut W {
        self.variant(MS_A::MS_21)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_22(self) -> &'a mut W {
        self.variant(MS_A::MS_22)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_23(self) -> &'a mut W {
        self.variant(MS_A::MS_23)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_24(self) -> &'a mut W {
        self.variant(MS_A::MS_24)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_25(self) -> &'a mut W {
        self.variant(MS_A::MS_25)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_26(self) -> &'a mut W {
        self.variant(MS_A::MS_26)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_27(self) -> &'a mut W {
        self.variant(MS_A::MS_27)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_28(self) -> &'a mut W {
        self.variant(MS_A::MS_28)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_29(self) -> &'a mut W {
        self.variant(MS_A::MS_29)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_30(self) -> &'a mut W {
        self.variant(MS_A::MS_30)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_31(self) -> &'a mut W {
        self.variant(MS_A::MS_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Reader of field `BA`"]
pub type BA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BA`"]
pub struct BA_W<'a> {
    w: &'a mut W,
}
impl<'a> BA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:5 - Memory size"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 12:31 - Base Address"]
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&mut self) -> VLD_W {
        VLD_W { w: self }
    }
    #[doc = "Bits 1:5 - Memory size"]
    #[inline(always)]
    pub fn ms(&mut self) -> MS_W {
        MS_W { w: self }
    }
    #[doc = "Bits 12:31 - Base Address"]
    #[inline(always)]
    pub fn ba(&mut self) -> BA_W {
        BA_W { w: self }
    }
}
