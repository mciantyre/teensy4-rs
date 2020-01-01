#[doc = "Reader of register SDRAMCR0"]
pub type R = crate::R<u32, super::SDRAMCR0>;
#[doc = "Writer for register SDRAMCR0"]
pub type W = crate::W<u32, super::SDRAMCR0>;
#[doc = "Register SDRAMCR0 `reset()`'s with value 0x0c26"]
impl crate::ResetValue for super::SDRAMCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c26
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
#[doc = "Burst Length\n\nValue on reset: 2"]
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
    #[doc = "4: 8"]
    BL_4 = 4,
    #[doc = "5: 8"]
    BL_5 = 5,
    #[doc = "6: 8"]
    BL_6 = 6,
    #[doc = "7: 8"]
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
    #[doc = "8"]
    #[inline(always)]
    pub fn bl_4(self) -> &'a mut W {
        self.variant(BL_A::BL_4)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn bl_5(self) -> &'a mut W {
        self.variant(BL_A::BL_5)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn bl_6(self) -> &'a mut W {
        self.variant(BL_A::BL_6)
    }
    #[doc = "8"]
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
#[doc = "Column 8 selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COL8_A {
    #[doc = "0: Column address bit number is decided by COL field."]
    COL8_0 = 0,
    #[doc = "1: Column address bit number is 8. COL field is ignored."]
    COL8_1 = 1,
}
impl From<COL8_A> for bool {
    #[inline(always)]
    fn from(variant: COL8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COL8`"]
pub type COL8_R = crate::R<bool, COL8_A>;
impl COL8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COL8_A {
        match self.bits {
            false => COL8_A::COL8_0,
            true => COL8_A::COL8_1,
        }
    }
    #[doc = "Checks if the value of the field is `COL8_0`"]
    #[inline(always)]
    pub fn is_col8_0(&self) -> bool {
        *self == COL8_A::COL8_0
    }
    #[doc = "Checks if the value of the field is `COL8_1`"]
    #[inline(always)]
    pub fn is_col8_1(&self) -> bool {
        *self == COL8_A::COL8_1
    }
}
#[doc = "Write proxy for field `COL8`"]
pub struct COL8_W<'a> {
    w: &'a mut W,
}
impl<'a> COL8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COL8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Column address bit number is decided by COL field."]
    #[inline(always)]
    pub fn col8_0(self) -> &'a mut W {
        self.variant(COL8_A::COL8_0)
    }
    #[doc = "Column address bit number is 8. COL field is ignored."]
    #[inline(always)]
    pub fn col8_1(self) -> &'a mut W {
        self.variant(COL8_A::COL8_1)
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
#[doc = "Column address bit number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COL_A {
    #[doc = "0: 12 bit"]
    COL_0 = 0,
    #[doc = "1: 11 bit"]
    COL_1 = 1,
    #[doc = "2: 10 bit"]
    COL_2 = 2,
    #[doc = "3: 9 bit"]
    COL_3 = 3,
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
    #[doc = "12 bit"]
    #[inline(always)]
    pub fn col_0(self) -> &'a mut W {
        self.variant(COL_A::COL_0)
    }
    #[doc = "11 bit"]
    #[inline(always)]
    pub fn col_1(self) -> &'a mut W {
        self.variant(COL_A::COL_1)
    }
    #[doc = "10 bit"]
    #[inline(always)]
    pub fn col_2(self) -> &'a mut W {
        self.variant(COL_A::COL_2)
    }
    #[doc = "9 bit"]
    #[inline(always)]
    pub fn col_3(self) -> &'a mut W {
        self.variant(COL_A::COL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "CAS Latency\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CL_A {
    #[doc = "0: 1"]
    CL_0 = 0,
    #[doc = "1: 1"]
    CL_1 = 1,
    #[doc = "2: 2"]
    CL_2 = 2,
    #[doc = "3: 3"]
    CL_3 = 3,
}
impl From<CL_A> for u8 {
    #[inline(always)]
    fn from(variant: CL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CL`"]
pub type CL_R = crate::R<u8, CL_A>;
impl CL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CL_A {
        match self.bits {
            0 => CL_A::CL_0,
            1 => CL_A::CL_1,
            2 => CL_A::CL_2,
            3 => CL_A::CL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CL_0`"]
    #[inline(always)]
    pub fn is_cl_0(&self) -> bool {
        *self == CL_A::CL_0
    }
    #[doc = "Checks if the value of the field is `CL_1`"]
    #[inline(always)]
    pub fn is_cl_1(&self) -> bool {
        *self == CL_A::CL_1
    }
    #[doc = "Checks if the value of the field is `CL_2`"]
    #[inline(always)]
    pub fn is_cl_2(&self) -> bool {
        *self == CL_A::CL_2
    }
    #[doc = "Checks if the value of the field is `CL_3`"]
    #[inline(always)]
    pub fn is_cl_3(&self) -> bool {
        *self == CL_A::CL_3
    }
}
#[doc = "Write proxy for field `CL`"]
pub struct CL_W<'a> {
    w: &'a mut W,
}
impl<'a> CL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn cl_0(self) -> &'a mut W {
        self.variant(CL_A::CL_0)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn cl_1(self) -> &'a mut W {
        self.variant(CL_A::CL_1)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn cl_2(self) -> &'a mut W {
        self.variant(CL_A::CL_2)
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn cl_3(self) -> &'a mut W {
        self.variant(CL_A::CL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "2 Bank selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BANK2_A {
    #[doc = "0: SDRAM device has 4 banks."]
    BANK2_0 = 0,
    #[doc = "1: SDRAM device has 2 banks."]
    BANK2_1 = 1,
}
impl From<BANK2_A> for bool {
    #[inline(always)]
    fn from(variant: BANK2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BANK2`"]
pub type BANK2_R = crate::R<bool, BANK2_A>;
impl BANK2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BANK2_A {
        match self.bits {
            false => BANK2_A::BANK2_0,
            true => BANK2_A::BANK2_1,
        }
    }
    #[doc = "Checks if the value of the field is `BANK2_0`"]
    #[inline(always)]
    pub fn is_bank2_0(&self) -> bool {
        *self == BANK2_A::BANK2_0
    }
    #[doc = "Checks if the value of the field is `BANK2_1`"]
    #[inline(always)]
    pub fn is_bank2_1(&self) -> bool {
        *self == BANK2_A::BANK2_1
    }
}
#[doc = "Write proxy for field `BANK2`"]
pub struct BANK2_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BANK2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SDRAM device has 4 banks."]
    #[inline(always)]
    pub fn bank2_0(self) -> &'a mut W {
        self.variant(BANK2_A::BANK2_0)
    }
    #[doc = "SDRAM device has 2 banks."]
    #[inline(always)]
    pub fn bank2_1(self) -> &'a mut W {
        self.variant(BANK2_A::BANK2_1)
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
    #[doc = "Bit 7 - Column 8 selection bit"]
    #[inline(always)]
    pub fn col8(&self) -> COL8_R {
        COL8_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Column address bit number"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - CAS Latency"]
    #[inline(always)]
    pub fn cl(&self) -> CL_R {
        CL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 14 - 2 Bank selection bit"]
    #[inline(always)]
    pub fn bank2(&self) -> BANK2_R {
        BANK2_R::new(((self.bits >> 14) & 0x01) != 0)
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
    #[doc = "Bit 7 - Column 8 selection bit"]
    #[inline(always)]
    pub fn col8(&mut self) -> COL8_W {
        COL8_W { w: self }
    }
    #[doc = "Bits 8:9 - Column address bit number"]
    #[inline(always)]
    pub fn col(&mut self) -> COL_W {
        COL_W { w: self }
    }
    #[doc = "Bits 10:11 - CAS Latency"]
    #[inline(always)]
    pub fn cl(&mut self) -> CL_W {
        CL_W { w: self }
    }
    #[doc = "Bit 14 - 2 Bank selection bit"]
    #[inline(always)]
    pub fn bank2(&mut self) -> BANK2_W {
        BANK2_W { w: self }
    }
}
