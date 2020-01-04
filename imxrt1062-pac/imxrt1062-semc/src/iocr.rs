#[doc = "Reader of register IOCR"]
pub type R = crate::R<u32, super::IOCR>;
#[doc = "Writer for register IOCR"]
pub type W = crate::W<u32, super::IOCR>;
#[doc = "Register IOCR `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SEMC_A8 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_A8_A {
    #[doc = "0: SDRAM Address bit (A8)"]
    MUX_A8_0 = 0,
    #[doc = "1: NAND CE#"]
    MUX_A8_1 = 1,
    #[doc = "2: NOR CE#"]
    MUX_A8_2 = 2,
    #[doc = "3: PSRAM CE#"]
    MUX_A8_3 = 3,
    #[doc = "4: DBI CSX"]
    MUX_A8_4 = 4,
    #[doc = "5: SDRAM Address bit (A8)"]
    MUX_A8_5 = 5,
    #[doc = "6: SDRAM Address bit (A8)"]
    MUX_A8_6 = 6,
    #[doc = "7: SDRAM Address bit (A8)"]
    MUX_A8_7 = 7,
}
impl From<MUX_A8_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_A8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUX_A8`"]
pub type MUX_A8_R = crate::R<u8, MUX_A8_A>;
impl MUX_A8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_A8_A {
        match self.bits {
            0 => MUX_A8_A::MUX_A8_0,
            1 => MUX_A8_A::MUX_A8_1,
            2 => MUX_A8_A::MUX_A8_2,
            3 => MUX_A8_A::MUX_A8_3,
            4 => MUX_A8_A::MUX_A8_4,
            5 => MUX_A8_A::MUX_A8_5,
            6 => MUX_A8_A::MUX_A8_6,
            7 => MUX_A8_A::MUX_A8_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_A8_0`"]
    #[inline(always)]
    pub fn is_mux_a8_0(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_0
    }
    #[doc = "Checks if the value of the field is `MUX_A8_1`"]
    #[inline(always)]
    pub fn is_mux_a8_1(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_1
    }
    #[doc = "Checks if the value of the field is `MUX_A8_2`"]
    #[inline(always)]
    pub fn is_mux_a8_2(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_2
    }
    #[doc = "Checks if the value of the field is `MUX_A8_3`"]
    #[inline(always)]
    pub fn is_mux_a8_3(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_3
    }
    #[doc = "Checks if the value of the field is `MUX_A8_4`"]
    #[inline(always)]
    pub fn is_mux_a8_4(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_4
    }
    #[doc = "Checks if the value of the field is `MUX_A8_5`"]
    #[inline(always)]
    pub fn is_mux_a8_5(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_5
    }
    #[doc = "Checks if the value of the field is `MUX_A8_6`"]
    #[inline(always)]
    pub fn is_mux_a8_6(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_6
    }
    #[doc = "Checks if the value of the field is `MUX_A8_7`"]
    #[inline(always)]
    pub fn is_mux_a8_7(&self) -> bool {
        *self == MUX_A8_A::MUX_A8_7
    }
}
#[doc = "Write proxy for field `MUX_A8`"]
pub struct MUX_A8_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_A8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_A8_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SDRAM Address bit (A8)"]
    #[inline(always)]
    pub fn mux_a8_0(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_0)
    }
    #[doc = "NAND CE#"]
    #[inline(always)]
    pub fn mux_a8_1(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_1)
    }
    #[doc = "NOR CE#"]
    #[inline(always)]
    pub fn mux_a8_2(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_2)
    }
    #[doc = "PSRAM CE#"]
    #[inline(always)]
    pub fn mux_a8_3(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_3)
    }
    #[doc = "DBI CSX"]
    #[inline(always)]
    pub fn mux_a8_4(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_4)
    }
    #[doc = "SDRAM Address bit (A8)"]
    #[inline(always)]
    pub fn mux_a8_5(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_5)
    }
    #[doc = "SDRAM Address bit (A8)"]
    #[inline(always)]
    pub fn mux_a8_6(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_6)
    }
    #[doc = "SDRAM Address bit (A8)"]
    #[inline(always)]
    pub fn mux_a8_7(self) -> &'a mut W {
        self.variant(MUX_A8_A::MUX_A8_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "SEMC_CSX0 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_CSX0_A {
    #[doc = "0: NOR/PSRAM Address bit 24 (A24)"]
    MUX_CSX0_0 = 0,
    #[doc = "1: SDRAM CS1"]
    MUX_CSX0_1 = 1,
    #[doc = "2: SDRAM CS2"]
    MUX_CSX0_2 = 2,
    #[doc = "3: SDRAM CS3"]
    MUX_CSX0_3 = 3,
    #[doc = "4: NAND CE#"]
    MUX_CSX0_4 = 4,
    #[doc = "5: NOR CE#"]
    MUX_CSX0_5 = 5,
    #[doc = "6: PSRAM CE#"]
    MUX_CSX0_6 = 6,
    #[doc = "7: DBI CSX"]
    MUX_CSX0_7 = 7,
}
impl From<MUX_CSX0_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_CSX0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUX_CSX0`"]
pub type MUX_CSX0_R = crate::R<u8, MUX_CSX0_A>;
impl MUX_CSX0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_CSX0_A {
        match self.bits {
            0 => MUX_CSX0_A::MUX_CSX0_0,
            1 => MUX_CSX0_A::MUX_CSX0_1,
            2 => MUX_CSX0_A::MUX_CSX0_2,
            3 => MUX_CSX0_A::MUX_CSX0_3,
            4 => MUX_CSX0_A::MUX_CSX0_4,
            5 => MUX_CSX0_A::MUX_CSX0_5,
            6 => MUX_CSX0_A::MUX_CSX0_6,
            7 => MUX_CSX0_A::MUX_CSX0_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_0`"]
    #[inline(always)]
    pub fn is_mux_csx0_0(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_0
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_1`"]
    #[inline(always)]
    pub fn is_mux_csx0_1(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_1
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_2`"]
    #[inline(always)]
    pub fn is_mux_csx0_2(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_2
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_3`"]
    #[inline(always)]
    pub fn is_mux_csx0_3(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_3
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_4`"]
    #[inline(always)]
    pub fn is_mux_csx0_4(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_4
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_5`"]
    #[inline(always)]
    pub fn is_mux_csx0_5(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_5
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_6`"]
    #[inline(always)]
    pub fn is_mux_csx0_6(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_6
    }
    #[doc = "Checks if the value of the field is `MUX_CSX0_7`"]
    #[inline(always)]
    pub fn is_mux_csx0_7(&self) -> bool {
        *self == MUX_CSX0_A::MUX_CSX0_7
    }
}
#[doc = "Write proxy for field `MUX_CSX0`"]
pub struct MUX_CSX0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_CSX0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_CSX0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NOR/PSRAM Address bit 24 (A24)"]
    #[inline(always)]
    pub fn mux_csx0_0(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_0)
    }
    #[doc = "SDRAM CS1"]
    #[inline(always)]
    pub fn mux_csx0_1(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline(always)]
    pub fn mux_csx0_2(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline(always)]
    pub fn mux_csx0_3(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_3)
    }
    #[doc = "NAND CE#"]
    #[inline(always)]
    pub fn mux_csx0_4(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_4)
    }
    #[doc = "NOR CE#"]
    #[inline(always)]
    pub fn mux_csx0_5(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_5)
    }
    #[doc = "PSRAM CE#"]
    #[inline(always)]
    pub fn mux_csx0_6(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_6)
    }
    #[doc = "DBI CSX"]
    #[inline(always)]
    pub fn mux_csx0_7(self) -> &'a mut W {
        self.variant(MUX_CSX0_A::MUX_CSX0_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "SEMC_CSX1 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_CSX1_A {
    #[doc = "0: NOR/PSRAM Address bit 25 (A25)"]
    MUX_CSX1_0 = 0,
    #[doc = "1: SDRAM CS1"]
    MUX_CSX1_1 = 1,
    #[doc = "2: SDRAM CS2"]
    MUX_CSX1_2 = 2,
    #[doc = "3: SDRAM CS3"]
    MUX_CSX1_3 = 3,
    #[doc = "4: NAND CE#"]
    MUX_CSX1_4 = 4,
    #[doc = "5: NOR CE#"]
    MUX_CSX1_5 = 5,
    #[doc = "6: PSRAM CE#"]
    MUX_CSX1_6 = 6,
    #[doc = "7: DBI CSX"]
    MUX_CSX1_7 = 7,
}
impl From<MUX_CSX1_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_CSX1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUX_CSX1`"]
pub type MUX_CSX1_R = crate::R<u8, MUX_CSX1_A>;
impl MUX_CSX1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_CSX1_A {
        match self.bits {
            0 => MUX_CSX1_A::MUX_CSX1_0,
            1 => MUX_CSX1_A::MUX_CSX1_1,
            2 => MUX_CSX1_A::MUX_CSX1_2,
            3 => MUX_CSX1_A::MUX_CSX1_3,
            4 => MUX_CSX1_A::MUX_CSX1_4,
            5 => MUX_CSX1_A::MUX_CSX1_5,
            6 => MUX_CSX1_A::MUX_CSX1_6,
            7 => MUX_CSX1_A::MUX_CSX1_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_0`"]
    #[inline(always)]
    pub fn is_mux_csx1_0(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_0
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_1`"]
    #[inline(always)]
    pub fn is_mux_csx1_1(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_1
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_2`"]
    #[inline(always)]
    pub fn is_mux_csx1_2(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_2
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_3`"]
    #[inline(always)]
    pub fn is_mux_csx1_3(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_3
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_4`"]
    #[inline(always)]
    pub fn is_mux_csx1_4(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_4
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_5`"]
    #[inline(always)]
    pub fn is_mux_csx1_5(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_5
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_6`"]
    #[inline(always)]
    pub fn is_mux_csx1_6(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_6
    }
    #[doc = "Checks if the value of the field is `MUX_CSX1_7`"]
    #[inline(always)]
    pub fn is_mux_csx1_7(&self) -> bool {
        *self == MUX_CSX1_A::MUX_CSX1_7
    }
}
#[doc = "Write proxy for field `MUX_CSX1`"]
pub struct MUX_CSX1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_CSX1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_CSX1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NOR/PSRAM Address bit 25 (A25)"]
    #[inline(always)]
    pub fn mux_csx1_0(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_0)
    }
    #[doc = "SDRAM CS1"]
    #[inline(always)]
    pub fn mux_csx1_1(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline(always)]
    pub fn mux_csx1_2(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline(always)]
    pub fn mux_csx1_3(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_3)
    }
    #[doc = "NAND CE#"]
    #[inline(always)]
    pub fn mux_csx1_4(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_4)
    }
    #[doc = "NOR CE#"]
    #[inline(always)]
    pub fn mux_csx1_5(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_5)
    }
    #[doc = "PSRAM CE#"]
    #[inline(always)]
    pub fn mux_csx1_6(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_6)
    }
    #[doc = "DBI CSX"]
    #[inline(always)]
    pub fn mux_csx1_7(self) -> &'a mut W {
        self.variant(MUX_CSX1_A::MUX_CSX1_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "SEMC_CSX2 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_CSX2_A {
    #[doc = "0: NOR/PSRAM Address bit 26 (A26)"]
    MUX_CSX2_0 = 0,
    #[doc = "1: SDRAM CS1"]
    MUX_CSX2_1 = 1,
    #[doc = "2: SDRAM CS2"]
    MUX_CSX2_2 = 2,
    #[doc = "3: SDRAM CS3"]
    MUX_CSX2_3 = 3,
    #[doc = "4: NAND CE#"]
    MUX_CSX2_4 = 4,
    #[doc = "5: NOR CE#"]
    MUX_CSX2_5 = 5,
    #[doc = "6: PSRAM CE#"]
    MUX_CSX2_6 = 6,
    #[doc = "7: DBI CSX"]
    MUX_CSX2_7 = 7,
}
impl From<MUX_CSX2_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_CSX2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUX_CSX2`"]
pub type MUX_CSX2_R = crate::R<u8, MUX_CSX2_A>;
impl MUX_CSX2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_CSX2_A {
        match self.bits {
            0 => MUX_CSX2_A::MUX_CSX2_0,
            1 => MUX_CSX2_A::MUX_CSX2_1,
            2 => MUX_CSX2_A::MUX_CSX2_2,
            3 => MUX_CSX2_A::MUX_CSX2_3,
            4 => MUX_CSX2_A::MUX_CSX2_4,
            5 => MUX_CSX2_A::MUX_CSX2_5,
            6 => MUX_CSX2_A::MUX_CSX2_6,
            7 => MUX_CSX2_A::MUX_CSX2_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_0`"]
    #[inline(always)]
    pub fn is_mux_csx2_0(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_0
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_1`"]
    #[inline(always)]
    pub fn is_mux_csx2_1(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_1
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_2`"]
    #[inline(always)]
    pub fn is_mux_csx2_2(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_2
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_3`"]
    #[inline(always)]
    pub fn is_mux_csx2_3(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_3
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_4`"]
    #[inline(always)]
    pub fn is_mux_csx2_4(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_4
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_5`"]
    #[inline(always)]
    pub fn is_mux_csx2_5(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_5
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_6`"]
    #[inline(always)]
    pub fn is_mux_csx2_6(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_6
    }
    #[doc = "Checks if the value of the field is `MUX_CSX2_7`"]
    #[inline(always)]
    pub fn is_mux_csx2_7(&self) -> bool {
        *self == MUX_CSX2_A::MUX_CSX2_7
    }
}
#[doc = "Write proxy for field `MUX_CSX2`"]
pub struct MUX_CSX2_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_CSX2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_CSX2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NOR/PSRAM Address bit 26 (A26)"]
    #[inline(always)]
    pub fn mux_csx2_0(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_0)
    }
    #[doc = "SDRAM CS1"]
    #[inline(always)]
    pub fn mux_csx2_1(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline(always)]
    pub fn mux_csx2_2(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline(always)]
    pub fn mux_csx2_3(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_3)
    }
    #[doc = "NAND CE#"]
    #[inline(always)]
    pub fn mux_csx2_4(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_4)
    }
    #[doc = "NOR CE#"]
    #[inline(always)]
    pub fn mux_csx2_5(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_5)
    }
    #[doc = "PSRAM CE#"]
    #[inline(always)]
    pub fn mux_csx2_6(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_6)
    }
    #[doc = "DBI CSX"]
    #[inline(always)]
    pub fn mux_csx2_7(self) -> &'a mut W {
        self.variant(MUX_CSX2_A::MUX_CSX2_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "SEMC_CSX3 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_CSX3_A {
    #[doc = "0: NOR/PSRAM Address bit 27 (A27)"]
    MUX_CSX3_0 = 0,
    #[doc = "1: SDRAM CS1"]
    MUX_CSX3_1 = 1,
    #[doc = "2: SDRAM CS2"]
    MUX_CSX3_2 = 2,
    #[doc = "3: SDRAM CS3"]
    MUX_CSX3_3 = 3,
    #[doc = "4: NAND CE#"]
    MUX_CSX3_4 = 4,
    #[doc = "5: NOR CE#"]
    MUX_CSX3_5 = 5,
    #[doc = "6: PSRAM CE#"]
    MUX_CSX3_6 = 6,
    #[doc = "7: DBI CSX"]
    MUX_CSX3_7 = 7,
}
impl From<MUX_CSX3_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_CSX3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUX_CSX3`"]
pub type MUX_CSX3_R = crate::R<u8, MUX_CSX3_A>;
impl MUX_CSX3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_CSX3_A {
        match self.bits {
            0 => MUX_CSX3_A::MUX_CSX3_0,
            1 => MUX_CSX3_A::MUX_CSX3_1,
            2 => MUX_CSX3_A::MUX_CSX3_2,
            3 => MUX_CSX3_A::MUX_CSX3_3,
            4 => MUX_CSX3_A::MUX_CSX3_4,
            5 => MUX_CSX3_A::MUX_CSX3_5,
            6 => MUX_CSX3_A::MUX_CSX3_6,
            7 => MUX_CSX3_A::MUX_CSX3_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_0`"]
    #[inline(always)]
    pub fn is_mux_csx3_0(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_0
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_1`"]
    #[inline(always)]
    pub fn is_mux_csx3_1(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_1
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_2`"]
    #[inline(always)]
    pub fn is_mux_csx3_2(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_2
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_3`"]
    #[inline(always)]
    pub fn is_mux_csx3_3(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_3
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_4`"]
    #[inline(always)]
    pub fn is_mux_csx3_4(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_4
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_5`"]
    #[inline(always)]
    pub fn is_mux_csx3_5(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_5
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_6`"]
    #[inline(always)]
    pub fn is_mux_csx3_6(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_6
    }
    #[doc = "Checks if the value of the field is `MUX_CSX3_7`"]
    #[inline(always)]
    pub fn is_mux_csx3_7(&self) -> bool {
        *self == MUX_CSX3_A::MUX_CSX3_7
    }
}
#[doc = "Write proxy for field `MUX_CSX3`"]
pub struct MUX_CSX3_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_CSX3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_CSX3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NOR/PSRAM Address bit 27 (A27)"]
    #[inline(always)]
    pub fn mux_csx3_0(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_0)
    }
    #[doc = "SDRAM CS1"]
    #[inline(always)]
    pub fn mux_csx3_1(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline(always)]
    pub fn mux_csx3_2(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline(always)]
    pub fn mux_csx3_3(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_3)
    }
    #[doc = "NAND CE#"]
    #[inline(always)]
    pub fn mux_csx3_4(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_4)
    }
    #[doc = "NOR CE#"]
    #[inline(always)]
    pub fn mux_csx3_5(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_5)
    }
    #[doc = "PSRAM CE#"]
    #[inline(always)]
    pub fn mux_csx3_6(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_6)
    }
    #[doc = "DBI CSX"]
    #[inline(always)]
    pub fn mux_csx3_7(self) -> &'a mut W {
        self.variant(MUX_CSX3_A::MUX_CSX3_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "SEMC_RDY function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_RDY_A {
    #[doc = "0: NAND Ready/Wait# input"]
    MUX_RDY_0 = 0,
    #[doc = "1: SDRAM CS1"]
    MUX_RDY_1 = 1,
    #[doc = "2: SDRAM CS2"]
    MUX_RDY_2 = 2,
    #[doc = "3: SDRAM CS3"]
    MUX_RDY_3 = 3,
    #[doc = "4: NOR CE#"]
    MUX_RDY_4 = 4,
    #[doc = "5: PSRAM CE#"]
    MUX_RDY_5 = 5,
    #[doc = "6: DBI CSX"]
    MUX_RDY_6 = 6,
    #[doc = "7: NOR/PSRAM Address bit 27"]
    MUX_RDY_7 = 7,
}
impl From<MUX_RDY_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_RDY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUX_RDY`"]
pub type MUX_RDY_R = crate::R<u8, MUX_RDY_A>;
impl MUX_RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_RDY_A {
        match self.bits {
            0 => MUX_RDY_A::MUX_RDY_0,
            1 => MUX_RDY_A::MUX_RDY_1,
            2 => MUX_RDY_A::MUX_RDY_2,
            3 => MUX_RDY_A::MUX_RDY_3,
            4 => MUX_RDY_A::MUX_RDY_4,
            5 => MUX_RDY_A::MUX_RDY_5,
            6 => MUX_RDY_A::MUX_RDY_6,
            7 => MUX_RDY_A::MUX_RDY_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_0`"]
    #[inline(always)]
    pub fn is_mux_rdy_0(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_0
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_1`"]
    #[inline(always)]
    pub fn is_mux_rdy_1(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_1
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_2`"]
    #[inline(always)]
    pub fn is_mux_rdy_2(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_2
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_3`"]
    #[inline(always)]
    pub fn is_mux_rdy_3(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_3
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_4`"]
    #[inline(always)]
    pub fn is_mux_rdy_4(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_4
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_5`"]
    #[inline(always)]
    pub fn is_mux_rdy_5(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_5
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_6`"]
    #[inline(always)]
    pub fn is_mux_rdy_6(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_6
    }
    #[doc = "Checks if the value of the field is `MUX_RDY_7`"]
    #[inline(always)]
    pub fn is_mux_rdy_7(&self) -> bool {
        *self == MUX_RDY_A::MUX_RDY_7
    }
}
#[doc = "Write proxy for field `MUX_RDY`"]
pub struct MUX_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_RDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_RDY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NAND Ready/Wait# input"]
    #[inline(always)]
    pub fn mux_rdy_0(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_0)
    }
    #[doc = "SDRAM CS1"]
    #[inline(always)]
    pub fn mux_rdy_1(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_1)
    }
    #[doc = "SDRAM CS2"]
    #[inline(always)]
    pub fn mux_rdy_2(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_2)
    }
    #[doc = "SDRAM CS3"]
    #[inline(always)]
    pub fn mux_rdy_3(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_3)
    }
    #[doc = "NOR CE#"]
    #[inline(always)]
    pub fn mux_rdy_4(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_4)
    }
    #[doc = "PSRAM CE#"]
    #[inline(always)]
    pub fn mux_rdy_5(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_5)
    }
    #[doc = "DBI CSX"]
    #[inline(always)]
    pub fn mux_rdy_6(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_6)
    }
    #[doc = "NOR/PSRAM Address bit 27"]
    #[inline(always)]
    pub fn mux_rdy_7(self) -> &'a mut W {
        self.variant(MUX_RDY_A::MUX_RDY_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "SEMC_CLKX0 function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_CLKX0_A {
    #[doc = "0: NOR clock"]
    MUX_CLKX0_0 = 0,
    #[doc = "1: SRAM clock"]
    MUX_CLKX0_1 = 1,
}
impl From<MUX_CLKX0_A> for bool {
    #[inline(always)]
    fn from(variant: MUX_CLKX0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MUX_CLKX0`"]
pub type MUX_CLKX0_R = crate::R<bool, MUX_CLKX0_A>;
impl MUX_CLKX0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_CLKX0_A {
        match self.bits {
            false => MUX_CLKX0_A::MUX_CLKX0_0,
            true => MUX_CLKX0_A::MUX_CLKX0_1,
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CLKX0_0`"]
    #[inline(always)]
    pub fn is_mux_clkx0_0(&self) -> bool {
        *self == MUX_CLKX0_A::MUX_CLKX0_0
    }
    #[doc = "Checks if the value of the field is `MUX_CLKX0_1`"]
    #[inline(always)]
    pub fn is_mux_clkx0_1(&self) -> bool {
        *self == MUX_CLKX0_A::MUX_CLKX0_1
    }
}
#[doc = "Write proxy for field `MUX_CLKX0`"]
pub struct MUX_CLKX0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_CLKX0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_CLKX0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NOR clock"]
    #[inline(always)]
    pub fn mux_clkx0_0(self) -> &'a mut W {
        self.variant(MUX_CLKX0_A::MUX_CLKX0_0)
    }
    #[doc = "SRAM clock"]
    #[inline(always)]
    pub fn mux_clkx0_1(self) -> &'a mut W {
        self.variant(MUX_CLKX0_A::MUX_CLKX0_1)
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
#[doc = "SEMC_CLKX1 function selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_CLKX1_A {
    #[doc = "0: NOR clock"]
    MUX_CLKX1_0 = 0,
    #[doc = "1: SRAM clock"]
    MUX_CLKX1_1 = 1,
}
impl From<MUX_CLKX1_A> for bool {
    #[inline(always)]
    fn from(variant: MUX_CLKX1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MUX_CLKX1`"]
pub type MUX_CLKX1_R = crate::R<bool, MUX_CLKX1_A>;
impl MUX_CLKX1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_CLKX1_A {
        match self.bits {
            false => MUX_CLKX1_A::MUX_CLKX1_0,
            true => MUX_CLKX1_A::MUX_CLKX1_1,
        }
    }
    #[doc = "Checks if the value of the field is `MUX_CLKX1_0`"]
    #[inline(always)]
    pub fn is_mux_clkx1_0(&self) -> bool {
        *self == MUX_CLKX1_A::MUX_CLKX1_0
    }
    #[doc = "Checks if the value of the field is `MUX_CLKX1_1`"]
    #[inline(always)]
    pub fn is_mux_clkx1_1(&self) -> bool {
        *self == MUX_CLKX1_A::MUX_CLKX1_1
    }
}
#[doc = "Write proxy for field `MUX_CLKX1`"]
pub struct MUX_CLKX1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_CLKX1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_CLKX1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NOR clock"]
    #[inline(always)]
    pub fn mux_clkx1_0(self) -> &'a mut W {
        self.variant(MUX_CLKX1_A::MUX_CLKX1_0)
    }
    #[doc = "SRAM clock"]
    #[inline(always)]
    pub fn mux_clkx1_1(self) -> &'a mut W {
        self.variant(MUX_CLKX1_A::MUX_CLKX1_1)
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
impl R {
    #[doc = "Bits 0:2 - SEMC_A8 output selection"]
    #[inline(always)]
    pub fn mux_a8(&self) -> MUX_A8_R {
        MUX_A8_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - SEMC_CSX0 output selection"]
    #[inline(always)]
    pub fn mux_csx0(&self) -> MUX_CSX0_R {
        MUX_CSX0_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - SEMC_CSX1 output selection"]
    #[inline(always)]
    pub fn mux_csx1(&self) -> MUX_CSX1_R {
        MUX_CSX1_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - SEMC_CSX2 output selection"]
    #[inline(always)]
    pub fn mux_csx2(&self) -> MUX_CSX2_R {
        MUX_CSX2_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - SEMC_CSX3 output selection"]
    #[inline(always)]
    pub fn mux_csx3(&self) -> MUX_CSX3_R {
        MUX_CSX3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - SEMC_RDY function selection"]
    #[inline(always)]
    pub fn mux_rdy(&self) -> MUX_RDY_R {
        MUX_RDY_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bit 24 - SEMC_CLKX0 function selection"]
    #[inline(always)]
    pub fn mux_clkx0(&self) -> MUX_CLKX0_R {
        MUX_CLKX0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SEMC_CLKX1 function selection"]
    #[inline(always)]
    pub fn mux_clkx1(&self) -> MUX_CLKX1_R {
        MUX_CLKX1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SEMC_A8 output selection"]
    #[inline(always)]
    pub fn mux_a8(&mut self) -> MUX_A8_W {
        MUX_A8_W { w: self }
    }
    #[doc = "Bits 3:5 - SEMC_CSX0 output selection"]
    #[inline(always)]
    pub fn mux_csx0(&mut self) -> MUX_CSX0_W {
        MUX_CSX0_W { w: self }
    }
    #[doc = "Bits 6:8 - SEMC_CSX1 output selection"]
    #[inline(always)]
    pub fn mux_csx1(&mut self) -> MUX_CSX1_W {
        MUX_CSX1_W { w: self }
    }
    #[doc = "Bits 9:11 - SEMC_CSX2 output selection"]
    #[inline(always)]
    pub fn mux_csx2(&mut self) -> MUX_CSX2_W {
        MUX_CSX2_W { w: self }
    }
    #[doc = "Bits 12:14 - SEMC_CSX3 output selection"]
    #[inline(always)]
    pub fn mux_csx3(&mut self) -> MUX_CSX3_W {
        MUX_CSX3_W { w: self }
    }
    #[doc = "Bits 15:17 - SEMC_RDY function selection"]
    #[inline(always)]
    pub fn mux_rdy(&mut self) -> MUX_RDY_W {
        MUX_RDY_W { w: self }
    }
    #[doc = "Bit 24 - SEMC_CLKX0 function selection"]
    #[inline(always)]
    pub fn mux_clkx0(&mut self) -> MUX_CLKX0_W {
        MUX_CLKX0_W { w: self }
    }
    #[doc = "Bit 25 - SEMC_CLKX1 function selection"]
    #[inline(always)]
    pub fn mux_clkx1(&mut self) -> MUX_CLKX1_W {
        MUX_CLKX1_W { w: self }
    }
}
