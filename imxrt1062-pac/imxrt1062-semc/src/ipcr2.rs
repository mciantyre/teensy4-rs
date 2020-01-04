#[doc = "Reader of register IPCR2"]
pub type R = crate::R<u32, super::IPCR2>;
#[doc = "Writer for register IPCR2"]
pub type W = crate::W<u32, super::IPCR2>;
#[doc = "Register IPCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::IPCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Byte Mask for Byte 0 (IPTXD bit 7:0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BM0_A {
    #[doc = "0: Byte Unmasked"]
    BM0_0 = 0,
    #[doc = "1: Byte Masked"]
    BM0_1 = 1,
}
impl From<BM0_A> for bool {
    #[inline(always)]
    fn from(variant: BM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BM0`"]
pub type BM0_R = crate::R<bool, BM0_A>;
impl BM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BM0_A {
        match self.bits {
            false => BM0_A::BM0_0,
            true => BM0_A::BM0_1,
        }
    }
    #[doc = "Checks if the value of the field is `BM0_0`"]
    #[inline(always)]
    pub fn is_bm0_0(&self) -> bool {
        *self == BM0_A::BM0_0
    }
    #[doc = "Checks if the value of the field is `BM0_1`"]
    #[inline(always)]
    pub fn is_bm0_1(&self) -> bool {
        *self == BM0_A::BM0_1
    }
}
#[doc = "Write proxy for field `BM0`"]
pub struct BM0_W<'a> {
    w: &'a mut W,
}
impl<'a> BM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Byte Unmasked"]
    #[inline(always)]
    pub fn bm0_0(self) -> &'a mut W {
        self.variant(BM0_A::BM0_0)
    }
    #[doc = "Byte Masked"]
    #[inline(always)]
    pub fn bm0_1(self) -> &'a mut W {
        self.variant(BM0_A::BM0_1)
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
#[doc = "Byte Mask for Byte 1 (IPTXD bit 15:8)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BM1_A {
    #[doc = "0: Byte Unmasked"]
    BM1_0 = 0,
    #[doc = "1: Byte Masked"]
    BM1_1 = 1,
}
impl From<BM1_A> for bool {
    #[inline(always)]
    fn from(variant: BM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BM1`"]
pub type BM1_R = crate::R<bool, BM1_A>;
impl BM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BM1_A {
        match self.bits {
            false => BM1_A::BM1_0,
            true => BM1_A::BM1_1,
        }
    }
    #[doc = "Checks if the value of the field is `BM1_0`"]
    #[inline(always)]
    pub fn is_bm1_0(&self) -> bool {
        *self == BM1_A::BM1_0
    }
    #[doc = "Checks if the value of the field is `BM1_1`"]
    #[inline(always)]
    pub fn is_bm1_1(&self) -> bool {
        *self == BM1_A::BM1_1
    }
}
#[doc = "Write proxy for field `BM1`"]
pub struct BM1_W<'a> {
    w: &'a mut W,
}
impl<'a> BM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Byte Unmasked"]
    #[inline(always)]
    pub fn bm1_0(self) -> &'a mut W {
        self.variant(BM1_A::BM1_0)
    }
    #[doc = "Byte Masked"]
    #[inline(always)]
    pub fn bm1_1(self) -> &'a mut W {
        self.variant(BM1_A::BM1_1)
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
#[doc = "Byte Mask for Byte 2 (IPTXD bit 23:16)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BM2_A {
    #[doc = "0: Byte Unmasked"]
    BM2_0 = 0,
    #[doc = "1: Byte Masked"]
    BM2_1 = 1,
}
impl From<BM2_A> for bool {
    #[inline(always)]
    fn from(variant: BM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BM2`"]
pub type BM2_R = crate::R<bool, BM2_A>;
impl BM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BM2_A {
        match self.bits {
            false => BM2_A::BM2_0,
            true => BM2_A::BM2_1,
        }
    }
    #[doc = "Checks if the value of the field is `BM2_0`"]
    #[inline(always)]
    pub fn is_bm2_0(&self) -> bool {
        *self == BM2_A::BM2_0
    }
    #[doc = "Checks if the value of the field is `BM2_1`"]
    #[inline(always)]
    pub fn is_bm2_1(&self) -> bool {
        *self == BM2_A::BM2_1
    }
}
#[doc = "Write proxy for field `BM2`"]
pub struct BM2_W<'a> {
    w: &'a mut W,
}
impl<'a> BM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Byte Unmasked"]
    #[inline(always)]
    pub fn bm2_0(self) -> &'a mut W {
        self.variant(BM2_A::BM2_0)
    }
    #[doc = "Byte Masked"]
    #[inline(always)]
    pub fn bm2_1(self) -> &'a mut W {
        self.variant(BM2_A::BM2_1)
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
#[doc = "Byte Mask for Byte 3 (IPTXD bit 31:24)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BM3_A {
    #[doc = "0: Byte Unmasked"]
    BM3_0 = 0,
    #[doc = "1: Byte Masked"]
    BM3_1 = 1,
}
impl From<BM3_A> for bool {
    #[inline(always)]
    fn from(variant: BM3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BM3`"]
pub type BM3_R = crate::R<bool, BM3_A>;
impl BM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BM3_A {
        match self.bits {
            false => BM3_A::BM3_0,
            true => BM3_A::BM3_1,
        }
    }
    #[doc = "Checks if the value of the field is `BM3_0`"]
    #[inline(always)]
    pub fn is_bm3_0(&self) -> bool {
        *self == BM3_A::BM3_0
    }
    #[doc = "Checks if the value of the field is `BM3_1`"]
    #[inline(always)]
    pub fn is_bm3_1(&self) -> bool {
        *self == BM3_A::BM3_1
    }
}
#[doc = "Write proxy for field `BM3`"]
pub struct BM3_W<'a> {
    w: &'a mut W,
}
impl<'a> BM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BM3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Byte Unmasked"]
    #[inline(always)]
    pub fn bm3_0(self) -> &'a mut W {
        self.variant(BM3_A::BM3_0)
    }
    #[doc = "Byte Masked"]
    #[inline(always)]
    pub fn bm3_1(self) -> &'a mut W {
        self.variant(BM3_A::BM3_1)
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
impl R {
    #[doc = "Bit 0 - Byte Mask for Byte 0 (IPTXD bit 7:0)"]
    #[inline(always)]
    pub fn bm0(&self) -> BM0_R {
        BM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Byte Mask for Byte 1 (IPTXD bit 15:8)"]
    #[inline(always)]
    pub fn bm1(&self) -> BM1_R {
        BM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Byte Mask for Byte 2 (IPTXD bit 23:16)"]
    #[inline(always)]
    pub fn bm2(&self) -> BM2_R {
        BM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Byte Mask for Byte 3 (IPTXD bit 31:24)"]
    #[inline(always)]
    pub fn bm3(&self) -> BM3_R {
        BM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Byte Mask for Byte 0 (IPTXD bit 7:0)"]
    #[inline(always)]
    pub fn bm0(&mut self) -> BM0_W {
        BM0_W { w: self }
    }
    #[doc = "Bit 1 - Byte Mask for Byte 1 (IPTXD bit 15:8)"]
    #[inline(always)]
    pub fn bm1(&mut self) -> BM1_W {
        BM1_W { w: self }
    }
    #[doc = "Bit 2 - Byte Mask for Byte 2 (IPTXD bit 23:16)"]
    #[inline(always)]
    pub fn bm2(&mut self) -> BM2_W {
        BM2_W { w: self }
    }
    #[doc = "Bit 3 - Byte Mask for Byte 3 (IPTXD bit 31:24)"]
    #[inline(always)]
    pub fn bm3(&mut self) -> BM3_W {
        BM3_W { w: self }
    }
}
