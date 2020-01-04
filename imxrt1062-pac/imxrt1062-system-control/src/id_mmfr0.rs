#[doc = "Reader of register ID_MMFR0"]
pub type R = crate::R<u32, super::ID_MMFR0>;
#[doc = "Indicates support for a PMSA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PMSASUPPORT_A {
    #[doc = "0: Not supported"]
    PMSASUPPORT_0 = 0,
    #[doc = "1: ARMv7-M unused"]
    PMSASUPPORT_1 = 1,
    #[doc = "2: ARMv7-M unused"]
    PMSASUPPORT_2 = 2,
    #[doc = "3: PMSAv7, providing support for a base region and subregions."]
    PMSASUPPORT_3 = 3,
}
impl From<PMSASUPPORT_A> for u8 {
    #[inline(always)]
    fn from(variant: PMSASUPPORT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PMSASUPPORT`"]
pub type PMSASUPPORT_R = crate::R<u8, PMSASUPPORT_A>;
impl PMSASUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PMSASUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PMSASUPPORT_A::PMSASUPPORT_0),
            1 => Val(PMSASUPPORT_A::PMSASUPPORT_1),
            2 => Val(PMSASUPPORT_A::PMSASUPPORT_2),
            3 => Val(PMSASUPPORT_A::PMSASUPPORT_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PMSASUPPORT_0`"]
    #[inline(always)]
    pub fn is_pmsasupport_0(&self) -> bool {
        *self == PMSASUPPORT_A::PMSASUPPORT_0
    }
    #[doc = "Checks if the value of the field is `PMSASUPPORT_1`"]
    #[inline(always)]
    pub fn is_pmsasupport_1(&self) -> bool {
        *self == PMSASUPPORT_A::PMSASUPPORT_1
    }
    #[doc = "Checks if the value of the field is `PMSASUPPORT_2`"]
    #[inline(always)]
    pub fn is_pmsasupport_2(&self) -> bool {
        *self == PMSASUPPORT_A::PMSASUPPORT_2
    }
    #[doc = "Checks if the value of the field is `PMSASUPPORT_3`"]
    #[inline(always)]
    pub fn is_pmsasupport_3(&self) -> bool {
        *self == PMSASUPPORT_A::PMSASUPPORT_3
    }
}
#[doc = "Indicates the outermost shareability domain implemented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUTERMOST_SHAREABILITY_A {
    #[doc = "0: Implemented as Non-cacheable"]
    OUTERMOST_SHAREABILITY_0 = 0,
    #[doc = "1: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_1 = 1,
    #[doc = "2: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_2 = 2,
    #[doc = "3: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_3 = 3,
    #[doc = "4: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_4 = 4,
    #[doc = "5: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_5 = 5,
    #[doc = "6: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_6 = 6,
    #[doc = "7: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_7 = 7,
    #[doc = "8: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_8 = 8,
    #[doc = "9: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_9 = 9,
    #[doc = "10: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_10 = 10,
    #[doc = "11: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_11 = 11,
    #[doc = "12: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_12 = 12,
    #[doc = "13: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_13 = 13,
    #[doc = "14: ARMv7-M unused"]
    OUTERMOST_SHAREABILITY_14 = 14,
    #[doc = "15: Shareability ignored."]
    OUTERMOST_SHAREABILITY_15 = 15,
}
impl From<OUTERMOST_SHAREABILITY_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTERMOST_SHAREABILITY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OUTERMOST_SHAREABILITY`"]
pub type OUTERMOST_SHAREABILITY_R = crate::R<u8, OUTERMOST_SHAREABILITY_A>;
impl OUTERMOST_SHAREABILITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTERMOST_SHAREABILITY_A {
        match self.bits {
            0 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_0,
            1 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_1,
            2 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_2,
            3 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_3,
            4 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_4,
            5 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_5,
            6 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_6,
            7 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_7,
            8 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_8,
            9 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_9,
            10 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_10,
            11 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_11,
            12 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_12,
            13 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_13,
            14 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_14,
            15 => OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_0`"]
    #[inline(always)]
    pub fn is_outermost_shareability_0(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_0
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_1`"]
    #[inline(always)]
    pub fn is_outermost_shareability_1(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_1
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_2`"]
    #[inline(always)]
    pub fn is_outermost_shareability_2(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_2
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_3`"]
    #[inline(always)]
    pub fn is_outermost_shareability_3(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_3
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_4`"]
    #[inline(always)]
    pub fn is_outermost_shareability_4(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_4
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_5`"]
    #[inline(always)]
    pub fn is_outermost_shareability_5(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_5
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_6`"]
    #[inline(always)]
    pub fn is_outermost_shareability_6(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_6
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_7`"]
    #[inline(always)]
    pub fn is_outermost_shareability_7(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_7
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_8`"]
    #[inline(always)]
    pub fn is_outermost_shareability_8(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_8
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_9`"]
    #[inline(always)]
    pub fn is_outermost_shareability_9(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_9
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_10`"]
    #[inline(always)]
    pub fn is_outermost_shareability_10(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_10
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_11`"]
    #[inline(always)]
    pub fn is_outermost_shareability_11(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_11
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_12`"]
    #[inline(always)]
    pub fn is_outermost_shareability_12(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_12
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_13`"]
    #[inline(always)]
    pub fn is_outermost_shareability_13(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_13
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_14`"]
    #[inline(always)]
    pub fn is_outermost_shareability_14(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_14
    }
    #[doc = "Checks if the value of the field is `OUTERMOST_SHAREABILITY_15`"]
    #[inline(always)]
    pub fn is_outermost_shareability_15(&self) -> bool {
        *self == OUTERMOST_SHAREABILITY_A::OUTERMOST_SHAREABILITY_15
    }
}
#[doc = "Indicates the number of shareability levels implemented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SHAREABILITY_LEVELS_A {
    #[doc = "0: One level of shareability implemented"]
    SHAREABILITY_LEVELS_0 = 0,
    #[doc = "1: ARMv7-M unused"]
    SHAREABILITY_LEVELS_1 = 1,
}
impl From<SHAREABILITY_LEVELS_A> for u8 {
    #[inline(always)]
    fn from(variant: SHAREABILITY_LEVELS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SHAREABILITY_LEVELS`"]
pub type SHAREABILITY_LEVELS_R = crate::R<u8, SHAREABILITY_LEVELS_A>;
impl SHAREABILITY_LEVELS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SHAREABILITY_LEVELS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SHAREABILITY_LEVELS_A::SHAREABILITY_LEVELS_0),
            1 => Val(SHAREABILITY_LEVELS_A::SHAREABILITY_LEVELS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SHAREABILITY_LEVELS_0`"]
    #[inline(always)]
    pub fn is_shareability_levels_0(&self) -> bool {
        *self == SHAREABILITY_LEVELS_A::SHAREABILITY_LEVELS_0
    }
    #[doc = "Checks if the value of the field is `SHAREABILITY_LEVELS_1`"]
    #[inline(always)]
    pub fn is_shareability_levels_1(&self) -> bool {
        *self == SHAREABILITY_LEVELS_A::SHAREABILITY_LEVELS_1
    }
}
#[doc = "Indicates the support for Tightly Coupled Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCM_SUPPORT_A {
    #[doc = "0: No tightly coupled memories implemented."]
    TCM_SUPPORT_0 = 0,
    #[doc = "1: Tightly coupled memories implemented with IMPLEMENTATION DEFINED control."]
    TCM_SUPPORT_1 = 1,
    #[doc = "2: ARMv7-M unused"]
    TCM_SUPPORT_2 = 2,
}
impl From<TCM_SUPPORT_A> for u8 {
    #[inline(always)]
    fn from(variant: TCM_SUPPORT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TCM_SUPPORT`"]
pub type TCM_SUPPORT_R = crate::R<u8, TCM_SUPPORT_A>;
impl TCM_SUPPORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TCM_SUPPORT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TCM_SUPPORT_A::TCM_SUPPORT_0),
            1 => Val(TCM_SUPPORT_A::TCM_SUPPORT_1),
            2 => Val(TCM_SUPPORT_A::TCM_SUPPORT_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCM_SUPPORT_0`"]
    #[inline(always)]
    pub fn is_tcm_support_0(&self) -> bool {
        *self == TCM_SUPPORT_A::TCM_SUPPORT_0
    }
    #[doc = "Checks if the value of the field is `TCM_SUPPORT_1`"]
    #[inline(always)]
    pub fn is_tcm_support_1(&self) -> bool {
        *self == TCM_SUPPORT_A::TCM_SUPPORT_1
    }
    #[doc = "Checks if the value of the field is `TCM_SUPPORT_2`"]
    #[inline(always)]
    pub fn is_tcm_support_2(&self) -> bool {
        *self == TCM_SUPPORT_A::TCM_SUPPORT_2
    }
}
#[doc = "Indicates the support for Auxiliary registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUXILIARY_REGISTERS_A {
    #[doc = "0: Not supported"]
    AUXILIARY_REGISTERS_0 = 0,
    #[doc = "1: Support for Auxiliary Control Register only."]
    AUXILIARY_REGISTERS_1 = 1,
    #[doc = "2: ARMv7-M unused"]
    AUXILIARY_REGISTERS_2 = 2,
}
impl From<AUXILIARY_REGISTERS_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXILIARY_REGISTERS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AUXILIARY_REGISTERS`"]
pub type AUXILIARY_REGISTERS_R = crate::R<u8, AUXILIARY_REGISTERS_A>;
impl AUXILIARY_REGISTERS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AUXILIARY_REGISTERS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AUXILIARY_REGISTERS_A::AUXILIARY_REGISTERS_0),
            1 => Val(AUXILIARY_REGISTERS_A::AUXILIARY_REGISTERS_1),
            2 => Val(AUXILIARY_REGISTERS_A::AUXILIARY_REGISTERS_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUXILIARY_REGISTERS_0`"]
    #[inline(always)]
    pub fn is_auxiliary_registers_0(&self) -> bool {
        *self == AUXILIARY_REGISTERS_A::AUXILIARY_REGISTERS_0
    }
    #[doc = "Checks if the value of the field is `AUXILIARY_REGISTERS_1`"]
    #[inline(always)]
    pub fn is_auxiliary_registers_1(&self) -> bool {
        *self == AUXILIARY_REGISTERS_A::AUXILIARY_REGISTERS_1
    }
    #[doc = "Checks if the value of the field is `AUXILIARY_REGISTERS_2`"]
    #[inline(always)]
    pub fn is_auxiliary_registers_2(&self) -> bool {
        *self == AUXILIARY_REGISTERS_A::AUXILIARY_REGISTERS_2
    }
}
impl R {
    #[doc = "Bits 4:7 - Indicates support for a PMSA"]
    #[inline(always)]
    pub fn pmsasupport(&self) -> PMSASUPPORT_R {
        PMSASUPPORT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the outermost shareability domain implemented"]
    #[inline(always)]
    pub fn outermost_shareability(&self) -> OUTERMOST_SHAREABILITY_R {
        OUTERMOST_SHAREABILITY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Indicates the number of shareability levels implemented"]
    #[inline(always)]
    pub fn shareability_levels(&self) -> SHAREABILITY_LEVELS_R {
        SHAREABILITY_LEVELS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the support for Tightly Coupled Memory"]
    #[inline(always)]
    pub fn tcm_support(&self) -> TCM_SUPPORT_R {
        TCM_SUPPORT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the support for Auxiliary registers"]
    #[inline(always)]
    pub fn auxiliary_registers(&self) -> AUXILIARY_REGISTERS_R {
        AUXILIARY_REGISTERS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
