#[doc = "Reader of register CLIDR"]
pub type R = crate::R<u32, super::CLIDR>;
#[doc = "Indicate the type of cache implemented at level 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CL1_A {
    #[doc = "0: No cache"]
    CL1_0 = 0,
    #[doc = "1: Instruction cache only"]
    CL1_1 = 1,
    #[doc = "2: Data cache only"]
    CL1_2 = 2,
    #[doc = "3: Separate instruction and data caches"]
    CL1_3 = 3,
    #[doc = "4: Unified cache"]
    CL1_4 = 4,
}
impl From<CL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CL1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CL1`"]
pub type CL1_R = crate::R<u8, CL1_A>;
impl CL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CL1_A::CL1_0),
            1 => Val(CL1_A::CL1_1),
            2 => Val(CL1_A::CL1_2),
            3 => Val(CL1_A::CL1_3),
            4 => Val(CL1_A::CL1_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL1_0`"]
    #[inline(always)]
    pub fn is_cl1_0(&self) -> bool {
        *self == CL1_A::CL1_0
    }
    #[doc = "Checks if the value of the field is `CL1_1`"]
    #[inline(always)]
    pub fn is_cl1_1(&self) -> bool {
        *self == CL1_A::CL1_1
    }
    #[doc = "Checks if the value of the field is `CL1_2`"]
    #[inline(always)]
    pub fn is_cl1_2(&self) -> bool {
        *self == CL1_A::CL1_2
    }
    #[doc = "Checks if the value of the field is `CL1_3`"]
    #[inline(always)]
    pub fn is_cl1_3(&self) -> bool {
        *self == CL1_A::CL1_3
    }
    #[doc = "Checks if the value of the field is `CL1_4`"]
    #[inline(always)]
    pub fn is_cl1_4(&self) -> bool {
        *self == CL1_A::CL1_4
    }
}
#[doc = "Indicate the type of cache implemented at level 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CL2_A {
    #[doc = "0: No cache"]
    CL2_0 = 0,
    #[doc = "1: Instruction cache only"]
    CL2_1 = 1,
    #[doc = "2: Data cache only"]
    CL2_2 = 2,
    #[doc = "3: Separate instruction and data caches"]
    CL2_3 = 3,
    #[doc = "4: Unified cache"]
    CL2_4 = 4,
}
impl From<CL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CL2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CL2`"]
pub type CL2_R = crate::R<u8, CL2_A>;
impl CL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CL2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CL2_A::CL2_0),
            1 => Val(CL2_A::CL2_1),
            2 => Val(CL2_A::CL2_2),
            3 => Val(CL2_A::CL2_3),
            4 => Val(CL2_A::CL2_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL2_0`"]
    #[inline(always)]
    pub fn is_cl2_0(&self) -> bool {
        *self == CL2_A::CL2_0
    }
    #[doc = "Checks if the value of the field is `CL2_1`"]
    #[inline(always)]
    pub fn is_cl2_1(&self) -> bool {
        *self == CL2_A::CL2_1
    }
    #[doc = "Checks if the value of the field is `CL2_2`"]
    #[inline(always)]
    pub fn is_cl2_2(&self) -> bool {
        *self == CL2_A::CL2_2
    }
    #[doc = "Checks if the value of the field is `CL2_3`"]
    #[inline(always)]
    pub fn is_cl2_3(&self) -> bool {
        *self == CL2_A::CL2_3
    }
    #[doc = "Checks if the value of the field is `CL2_4`"]
    #[inline(always)]
    pub fn is_cl2_4(&self) -> bool {
        *self == CL2_A::CL2_4
    }
}
#[doc = "Indicate the type of cache implemented at level 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CL3_A {
    #[doc = "0: No cache"]
    CL3_0 = 0,
    #[doc = "1: Instruction cache only"]
    CL3_1 = 1,
    #[doc = "2: Data cache only"]
    CL3_2 = 2,
    #[doc = "3: Separate instruction and data caches"]
    CL3_3 = 3,
    #[doc = "4: Unified cache"]
    CL3_4 = 4,
}
impl From<CL3_A> for u8 {
    #[inline(always)]
    fn from(variant: CL3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CL3`"]
pub type CL3_R = crate::R<u8, CL3_A>;
impl CL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CL3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CL3_A::CL3_0),
            1 => Val(CL3_A::CL3_1),
            2 => Val(CL3_A::CL3_2),
            3 => Val(CL3_A::CL3_3),
            4 => Val(CL3_A::CL3_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL3_0`"]
    #[inline(always)]
    pub fn is_cl3_0(&self) -> bool {
        *self == CL3_A::CL3_0
    }
    #[doc = "Checks if the value of the field is `CL3_1`"]
    #[inline(always)]
    pub fn is_cl3_1(&self) -> bool {
        *self == CL3_A::CL3_1
    }
    #[doc = "Checks if the value of the field is `CL3_2`"]
    #[inline(always)]
    pub fn is_cl3_2(&self) -> bool {
        *self == CL3_A::CL3_2
    }
    #[doc = "Checks if the value of the field is `CL3_3`"]
    #[inline(always)]
    pub fn is_cl3_3(&self) -> bool {
        *self == CL3_A::CL3_3
    }
    #[doc = "Checks if the value of the field is `CL3_4`"]
    #[inline(always)]
    pub fn is_cl3_4(&self) -> bool {
        *self == CL3_A::CL3_4
    }
}
#[doc = "Indicate the type of cache implemented at level 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CL4_A {
    #[doc = "0: No cache"]
    CL4_0 = 0,
    #[doc = "1: Instruction cache only"]
    CL4_1 = 1,
    #[doc = "2: Data cache only"]
    CL4_2 = 2,
    #[doc = "3: Separate instruction and data caches"]
    CL4_3 = 3,
    #[doc = "4: Unified cache"]
    CL4_4 = 4,
}
impl From<CL4_A> for u8 {
    #[inline(always)]
    fn from(variant: CL4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CL4`"]
pub type CL4_R = crate::R<u8, CL4_A>;
impl CL4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CL4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CL4_A::CL4_0),
            1 => Val(CL4_A::CL4_1),
            2 => Val(CL4_A::CL4_2),
            3 => Val(CL4_A::CL4_3),
            4 => Val(CL4_A::CL4_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL4_0`"]
    #[inline(always)]
    pub fn is_cl4_0(&self) -> bool {
        *self == CL4_A::CL4_0
    }
    #[doc = "Checks if the value of the field is `CL4_1`"]
    #[inline(always)]
    pub fn is_cl4_1(&self) -> bool {
        *self == CL4_A::CL4_1
    }
    #[doc = "Checks if the value of the field is `CL4_2`"]
    #[inline(always)]
    pub fn is_cl4_2(&self) -> bool {
        *self == CL4_A::CL4_2
    }
    #[doc = "Checks if the value of the field is `CL4_3`"]
    #[inline(always)]
    pub fn is_cl4_3(&self) -> bool {
        *self == CL4_A::CL4_3
    }
    #[doc = "Checks if the value of the field is `CL4_4`"]
    #[inline(always)]
    pub fn is_cl4_4(&self) -> bool {
        *self == CL4_A::CL4_4
    }
}
#[doc = "Indicate the type of cache implemented at level 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CL5_A {
    #[doc = "0: No cache"]
    CL5_0 = 0,
    #[doc = "1: Instruction cache only"]
    CL5_1 = 1,
    #[doc = "2: Data cache only"]
    CL5_2 = 2,
    #[doc = "3: Separate instruction and data caches"]
    CL5_3 = 3,
    #[doc = "4: Unified cache"]
    CL5_4 = 4,
}
impl From<CL5_A> for u8 {
    #[inline(always)]
    fn from(variant: CL5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CL5`"]
pub type CL5_R = crate::R<u8, CL5_A>;
impl CL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CL5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CL5_A::CL5_0),
            1 => Val(CL5_A::CL5_1),
            2 => Val(CL5_A::CL5_2),
            3 => Val(CL5_A::CL5_3),
            4 => Val(CL5_A::CL5_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL5_0`"]
    #[inline(always)]
    pub fn is_cl5_0(&self) -> bool {
        *self == CL5_A::CL5_0
    }
    #[doc = "Checks if the value of the field is `CL5_1`"]
    #[inline(always)]
    pub fn is_cl5_1(&self) -> bool {
        *self == CL5_A::CL5_1
    }
    #[doc = "Checks if the value of the field is `CL5_2`"]
    #[inline(always)]
    pub fn is_cl5_2(&self) -> bool {
        *self == CL5_A::CL5_2
    }
    #[doc = "Checks if the value of the field is `CL5_3`"]
    #[inline(always)]
    pub fn is_cl5_3(&self) -> bool {
        *self == CL5_A::CL5_3
    }
    #[doc = "Checks if the value of the field is `CL5_4`"]
    #[inline(always)]
    pub fn is_cl5_4(&self) -> bool {
        *self == CL5_A::CL5_4
    }
}
#[doc = "Indicate the type of cache implemented at level 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CL6_A {
    #[doc = "0: No cache"]
    CL6_0 = 0,
    #[doc = "1: Instruction cache only"]
    CL6_1 = 1,
    #[doc = "2: Data cache only"]
    CL6_2 = 2,
    #[doc = "3: Separate instruction and data caches"]
    CL6_3 = 3,
    #[doc = "4: Unified cache"]
    CL6_4 = 4,
}
impl From<CL6_A> for u8 {
    #[inline(always)]
    fn from(variant: CL6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CL6`"]
pub type CL6_R = crate::R<u8, CL6_A>;
impl CL6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CL6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CL6_A::CL6_0),
            1 => Val(CL6_A::CL6_1),
            2 => Val(CL6_A::CL6_2),
            3 => Val(CL6_A::CL6_3),
            4 => Val(CL6_A::CL6_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL6_0`"]
    #[inline(always)]
    pub fn is_cl6_0(&self) -> bool {
        *self == CL6_A::CL6_0
    }
    #[doc = "Checks if the value of the field is `CL6_1`"]
    #[inline(always)]
    pub fn is_cl6_1(&self) -> bool {
        *self == CL6_A::CL6_1
    }
    #[doc = "Checks if the value of the field is `CL6_2`"]
    #[inline(always)]
    pub fn is_cl6_2(&self) -> bool {
        *self == CL6_A::CL6_2
    }
    #[doc = "Checks if the value of the field is `CL6_3`"]
    #[inline(always)]
    pub fn is_cl6_3(&self) -> bool {
        *self == CL6_A::CL6_3
    }
    #[doc = "Checks if the value of the field is `CL6_4`"]
    #[inline(always)]
    pub fn is_cl6_4(&self) -> bool {
        *self == CL6_A::CL6_4
    }
}
#[doc = "Indicate the type of cache implemented at level 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CL7_A {
    #[doc = "0: No cache"]
    CL7_0 = 0,
    #[doc = "1: Instruction cache only"]
    CL7_1 = 1,
    #[doc = "2: Data cache only"]
    CL7_2 = 2,
    #[doc = "3: Separate instruction and data caches"]
    CL7_3 = 3,
    #[doc = "4: Unified cache"]
    CL7_4 = 4,
}
impl From<CL7_A> for u8 {
    #[inline(always)]
    fn from(variant: CL7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CL7`"]
pub type CL7_R = crate::R<u8, CL7_A>;
impl CL7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CL7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CL7_A::CL7_0),
            1 => Val(CL7_A::CL7_1),
            2 => Val(CL7_A::CL7_2),
            3 => Val(CL7_A::CL7_3),
            4 => Val(CL7_A::CL7_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL7_0`"]
    #[inline(always)]
    pub fn is_cl7_0(&self) -> bool {
        *self == CL7_A::CL7_0
    }
    #[doc = "Checks if the value of the field is `CL7_1`"]
    #[inline(always)]
    pub fn is_cl7_1(&self) -> bool {
        *self == CL7_A::CL7_1
    }
    #[doc = "Checks if the value of the field is `CL7_2`"]
    #[inline(always)]
    pub fn is_cl7_2(&self) -> bool {
        *self == CL7_A::CL7_2
    }
    #[doc = "Checks if the value of the field is `CL7_3`"]
    #[inline(always)]
    pub fn is_cl7_3(&self) -> bool {
        *self == CL7_A::CL7_3
    }
    #[doc = "Checks if the value of the field is `CL7_4`"]
    #[inline(always)]
    pub fn is_cl7_4(&self) -> bool {
        *self == CL7_A::CL7_4
    }
}
#[doc = "Level of Unification Inner Shareable for the cache hierarchy. This field is RAZ.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOUIS_A {
    #[doc = "0: 0"]
    LOUIS_0 = 0,
    #[doc = "1: 1"]
    LOUIS_1 = 1,
    #[doc = "2: 2"]
    LOUIS_2 = 2,
    #[doc = "3: 3"]
    LOUIS_3 = 3,
    #[doc = "4: 4"]
    LOUIS_4 = 4,
    #[doc = "5: 5"]
    LOUIS_5 = 5,
    #[doc = "6: 6"]
    LOUIS_6 = 6,
    #[doc = "7: 7"]
    LOUIS_7 = 7,
}
impl From<LOUIS_A> for u8 {
    #[inline(always)]
    fn from(variant: LOUIS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOUIS`"]
pub type LOUIS_R = crate::R<u8, LOUIS_A>;
impl LOUIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOUIS_A {
        match self.bits {
            0 => LOUIS_A::LOUIS_0,
            1 => LOUIS_A::LOUIS_1,
            2 => LOUIS_A::LOUIS_2,
            3 => LOUIS_A::LOUIS_3,
            4 => LOUIS_A::LOUIS_4,
            5 => LOUIS_A::LOUIS_5,
            6 => LOUIS_A::LOUIS_6,
            7 => LOUIS_A::LOUIS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOUIS_0`"]
    #[inline(always)]
    pub fn is_louis_0(&self) -> bool {
        *self == LOUIS_A::LOUIS_0
    }
    #[doc = "Checks if the value of the field is `LOUIS_1`"]
    #[inline(always)]
    pub fn is_louis_1(&self) -> bool {
        *self == LOUIS_A::LOUIS_1
    }
    #[doc = "Checks if the value of the field is `LOUIS_2`"]
    #[inline(always)]
    pub fn is_louis_2(&self) -> bool {
        *self == LOUIS_A::LOUIS_2
    }
    #[doc = "Checks if the value of the field is `LOUIS_3`"]
    #[inline(always)]
    pub fn is_louis_3(&self) -> bool {
        *self == LOUIS_A::LOUIS_3
    }
    #[doc = "Checks if the value of the field is `LOUIS_4`"]
    #[inline(always)]
    pub fn is_louis_4(&self) -> bool {
        *self == LOUIS_A::LOUIS_4
    }
    #[doc = "Checks if the value of the field is `LOUIS_5`"]
    #[inline(always)]
    pub fn is_louis_5(&self) -> bool {
        *self == LOUIS_A::LOUIS_5
    }
    #[doc = "Checks if the value of the field is `LOUIS_6`"]
    #[inline(always)]
    pub fn is_louis_6(&self) -> bool {
        *self == LOUIS_A::LOUIS_6
    }
    #[doc = "Checks if the value of the field is `LOUIS_7`"]
    #[inline(always)]
    pub fn is_louis_7(&self) -> bool {
        *self == LOUIS_A::LOUIS_7
    }
}
#[doc = "Level of Coherency for the cache hierarchy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOC_A {
    #[doc = "0: 0"]
    LOC_0 = 0,
    #[doc = "1: 1"]
    LOC_1 = 1,
    #[doc = "2: 2"]
    LOC_2 = 2,
    #[doc = "3: 3"]
    LOC_3 = 3,
    #[doc = "4: 4"]
    LOC_4 = 4,
    #[doc = "5: 5"]
    LOC_5 = 5,
    #[doc = "6: 6"]
    LOC_6 = 6,
    #[doc = "7: 7"]
    LOC_7 = 7,
}
impl From<LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOC`"]
pub type LOC_R = crate::R<u8, LOC_A>;
impl LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOC_A {
        match self.bits {
            0 => LOC_A::LOC_0,
            1 => LOC_A::LOC_1,
            2 => LOC_A::LOC_2,
            3 => LOC_A::LOC_3,
            4 => LOC_A::LOC_4,
            5 => LOC_A::LOC_5,
            6 => LOC_A::LOC_6,
            7 => LOC_A::LOC_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOC_0`"]
    #[inline(always)]
    pub fn is_loc_0(&self) -> bool {
        *self == LOC_A::LOC_0
    }
    #[doc = "Checks if the value of the field is `LOC_1`"]
    #[inline(always)]
    pub fn is_loc_1(&self) -> bool {
        *self == LOC_A::LOC_1
    }
    #[doc = "Checks if the value of the field is `LOC_2`"]
    #[inline(always)]
    pub fn is_loc_2(&self) -> bool {
        *self == LOC_A::LOC_2
    }
    #[doc = "Checks if the value of the field is `LOC_3`"]
    #[inline(always)]
    pub fn is_loc_3(&self) -> bool {
        *self == LOC_A::LOC_3
    }
    #[doc = "Checks if the value of the field is `LOC_4`"]
    #[inline(always)]
    pub fn is_loc_4(&self) -> bool {
        *self == LOC_A::LOC_4
    }
    #[doc = "Checks if the value of the field is `LOC_5`"]
    #[inline(always)]
    pub fn is_loc_5(&self) -> bool {
        *self == LOC_A::LOC_5
    }
    #[doc = "Checks if the value of the field is `LOC_6`"]
    #[inline(always)]
    pub fn is_loc_6(&self) -> bool {
        *self == LOC_A::LOC_6
    }
    #[doc = "Checks if the value of the field is `LOC_7`"]
    #[inline(always)]
    pub fn is_loc_7(&self) -> bool {
        *self == LOC_A::LOC_7
    }
}
#[doc = "Level of Unification for the cache hierarchy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOU_A {
    #[doc = "0: 0"]
    LOU_0 = 0,
    #[doc = "1: 1"]
    LOU_1 = 1,
    #[doc = "2: 2"]
    LOU_2 = 2,
    #[doc = "3: 3"]
    LOU_3 = 3,
    #[doc = "4: 4"]
    LOU_4 = 4,
    #[doc = "5: 5"]
    LOU_5 = 5,
    #[doc = "6: 6"]
    LOU_6 = 6,
    #[doc = "7: 7"]
    LOU_7 = 7,
}
impl From<LOU_A> for u8 {
    #[inline(always)]
    fn from(variant: LOU_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOU`"]
pub type LOU_R = crate::R<u8, LOU_A>;
impl LOU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOU_A {
        match self.bits {
            0 => LOU_A::LOU_0,
            1 => LOU_A::LOU_1,
            2 => LOU_A::LOU_2,
            3 => LOU_A::LOU_3,
            4 => LOU_A::LOU_4,
            5 => LOU_A::LOU_5,
            6 => LOU_A::LOU_6,
            7 => LOU_A::LOU_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOU_0`"]
    #[inline(always)]
    pub fn is_lou_0(&self) -> bool {
        *self == LOU_A::LOU_0
    }
    #[doc = "Checks if the value of the field is `LOU_1`"]
    #[inline(always)]
    pub fn is_lou_1(&self) -> bool {
        *self == LOU_A::LOU_1
    }
    #[doc = "Checks if the value of the field is `LOU_2`"]
    #[inline(always)]
    pub fn is_lou_2(&self) -> bool {
        *self == LOU_A::LOU_2
    }
    #[doc = "Checks if the value of the field is `LOU_3`"]
    #[inline(always)]
    pub fn is_lou_3(&self) -> bool {
        *self == LOU_A::LOU_3
    }
    #[doc = "Checks if the value of the field is `LOU_4`"]
    #[inline(always)]
    pub fn is_lou_4(&self) -> bool {
        *self == LOU_A::LOU_4
    }
    #[doc = "Checks if the value of the field is `LOU_5`"]
    #[inline(always)]
    pub fn is_lou_5(&self) -> bool {
        *self == LOU_A::LOU_5
    }
    #[doc = "Checks if the value of the field is `LOU_6`"]
    #[inline(always)]
    pub fn is_lou_6(&self) -> bool {
        *self == LOU_A::LOU_6
    }
    #[doc = "Checks if the value of the field is `LOU_7`"]
    #[inline(always)]
    pub fn is_lou_7(&self) -> bool {
        *self == LOU_A::LOU_7
    }
}
impl R {
    #[doc = "Bits 0:2 - Indicate the type of cache implemented at level 1."]
    #[inline(always)]
    pub fn cl1(&self) -> CL1_R {
        CL1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Indicate the type of cache implemented at level 2."]
    #[inline(always)]
    pub fn cl2(&self) -> CL2_R {
        CL2_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - Indicate the type of cache implemented at level 3."]
    #[inline(always)]
    pub fn cl3(&self) -> CL3_R {
        CL3_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Indicate the type of cache implemented at level 4."]
    #[inline(always)]
    pub fn cl4(&self) -> CL4_R {
        CL4_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Indicate the type of cache implemented at level 5."]
    #[inline(always)]
    pub fn cl5(&self) -> CL5_R {
        CL5_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - Indicate the type of cache implemented at level 6."]
    #[inline(always)]
    pub fn cl6(&self) -> CL6_R {
        CL6_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - Indicate the type of cache implemented at level 7."]
    #[inline(always)]
    pub fn cl7(&self) -> CL7_R {
        CL7_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - Level of Unification Inner Shareable for the cache hierarchy. This field is RAZ."]
    #[inline(always)]
    pub fn louis(&self) -> LOUIS_R {
        LOUIS_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Level of Coherency for the cache hierarchy"]
    #[inline(always)]
    pub fn loc(&self) -> LOC_R {
        LOC_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:29 - Level of Unification for the cache hierarchy"]
    #[inline(always)]
    pub fn lou(&self) -> LOU_R {
        LOU_R::new(((self.bits >> 27) & 0x07) as u8)
    }
}
