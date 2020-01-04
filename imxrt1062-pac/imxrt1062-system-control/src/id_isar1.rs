#[doc = "Reader of register ID_ISAR1"]
pub type R = crate::R<u32, super::ID_ISAR1>;
#[doc = "Indicates the supported Extend instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTEND_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    EXTEND_INSTRS_0 = 0,
    #[doc = "1: Adds support for the SXTB, SXTH, UXTB, and UXTH instructions"]
    EXTEND_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the SXTAB, SXTAB16, SXTAH, SXTB16, UXTAB, UXTAB16, UXTAH, and UXTB16 instructions"]
    EXTEND_INSTRS_2 = 2,
}
impl From<EXTEND_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEND_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTEND_INSTRS`"]
pub type EXTEND_INSTRS_R = crate::R<u8, EXTEND_INSTRS_A>;
impl EXTEND_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTEND_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTEND_INSTRS_A::EXTEND_INSTRS_0),
            1 => Val(EXTEND_INSTRS_A::EXTEND_INSTRS_1),
            2 => Val(EXTEND_INSTRS_A::EXTEND_INSTRS_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXTEND_INSTRS_0`"]
    #[inline(always)]
    pub fn is_extend_instrs_0(&self) -> bool {
        *self == EXTEND_INSTRS_A::EXTEND_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `EXTEND_INSTRS_1`"]
    #[inline(always)]
    pub fn is_extend_instrs_1(&self) -> bool {
        *self == EXTEND_INSTRS_A::EXTEND_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `EXTEND_INSTRS_2`"]
    #[inline(always)]
    pub fn is_extend_instrs_2(&self) -> bool {
        *self == EXTEND_INSTRS_A::EXTEND_INSTRS_2
    }
}
#[doc = "Indicates the supported IfThen instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IFTHEN_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    IFTHEN_INSTRS_0 = 0,
    #[doc = "1: Adds support for the IT instructions, and for the IT bits in the PSRs"]
    IFTHEN_INSTRS_1 = 1,
}
impl From<IFTHEN_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: IFTHEN_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IFTHEN_INSTRS`"]
pub type IFTHEN_INSTRS_R = crate::R<u8, IFTHEN_INSTRS_A>;
impl IFTHEN_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IFTHEN_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IFTHEN_INSTRS_A::IFTHEN_INSTRS_0),
            1 => Val(IFTHEN_INSTRS_A::IFTHEN_INSTRS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IFTHEN_INSTRS_0`"]
    #[inline(always)]
    pub fn is_ifthen_instrs_0(&self) -> bool {
        *self == IFTHEN_INSTRS_A::IFTHEN_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `IFTHEN_INSTRS_1`"]
    #[inline(always)]
    pub fn is_ifthen_instrs_1(&self) -> bool {
        *self == IFTHEN_INSTRS_A::IFTHEN_INSTRS_1
    }
}
#[doc = "Indicates the support for data-processing instructions with long immediate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IMMEDIATE_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    IMMEDIATE_INSTRS_0 = 0,
    #[doc = "1: Adds support for the ADDW, MOVW, MOVT, and SUBW instructions"]
    IMMEDIATE_INSTRS_1 = 1,
}
impl From<IMMEDIATE_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: IMMEDIATE_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IMMEDIATE_INSTRS`"]
pub type IMMEDIATE_INSTRS_R = crate::R<u8, IMMEDIATE_INSTRS_A>;
impl IMMEDIATE_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IMMEDIATE_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IMMEDIATE_INSTRS_A::IMMEDIATE_INSTRS_0),
            1 => Val(IMMEDIATE_INSTRS_A::IMMEDIATE_INSTRS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE_INSTRS_0`"]
    #[inline(always)]
    pub fn is_immediate_instrs_0(&self) -> bool {
        *self == IMMEDIATE_INSTRS_A::IMMEDIATE_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE_INSTRS_1`"]
    #[inline(always)]
    pub fn is_immediate_instrs_1(&self) -> bool {
        *self == IMMEDIATE_INSTRS_A::IMMEDIATE_INSTRS_1
    }
}
#[doc = "Indicates the supported Interworking instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTERWORK_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    INTERWORK_INSTRS_0 = 0,
    #[doc = "1: Adds support for the BX instruction, and the T bit in the PSR"]
    INTERWORK_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the BLX instruction, and PC loads have BX-like behavior"]
    INTERWORK_INSTRS_2 = 2,
    #[doc = "3: ARMv7-M unused"]
    INTERWORK_INSTRS_3 = 3,
}
impl From<INTERWORK_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: INTERWORK_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INTERWORK_INSTRS`"]
pub type INTERWORK_INSTRS_R = crate::R<u8, INTERWORK_INSTRS_A>;
impl INTERWORK_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INTERWORK_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INTERWORK_INSTRS_A::INTERWORK_INSTRS_0),
            1 => Val(INTERWORK_INSTRS_A::INTERWORK_INSTRS_1),
            2 => Val(INTERWORK_INSTRS_A::INTERWORK_INSTRS_2),
            3 => Val(INTERWORK_INSTRS_A::INTERWORK_INSTRS_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INTERWORK_INSTRS_0`"]
    #[inline(always)]
    pub fn is_interwork_instrs_0(&self) -> bool {
        *self == INTERWORK_INSTRS_A::INTERWORK_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `INTERWORK_INSTRS_1`"]
    #[inline(always)]
    pub fn is_interwork_instrs_1(&self) -> bool {
        *self == INTERWORK_INSTRS_A::INTERWORK_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `INTERWORK_INSTRS_2`"]
    #[inline(always)]
    pub fn is_interwork_instrs_2(&self) -> bool {
        *self == INTERWORK_INSTRS_A::INTERWORK_INSTRS_2
    }
    #[doc = "Checks if the value of the field is `INTERWORK_INSTRS_3`"]
    #[inline(always)]
    pub fn is_interwork_instrs_3(&self) -> bool {
        *self == INTERWORK_INSTRS_A::INTERWORK_INSTRS_3
    }
}
impl R {
    #[doc = "Bits 12:15 - Indicates the supported Extend instructions"]
    #[inline(always)]
    pub fn extend_instrs(&self) -> EXTEND_INSTRS_R {
        EXTEND_INSTRS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the supported IfThen instructions"]
    #[inline(always)]
    pub fn ifthen_instrs(&self) -> IFTHEN_INSTRS_R {
        IFTHEN_INSTRS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the support for data-processing instructions with long immediate"]
    #[inline(always)]
    pub fn immediate_instrs(&self) -> IMMEDIATE_INSTRS_R {
        IMMEDIATE_INSTRS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the supported Interworking instructions"]
    #[inline(always)]
    pub fn interwork_instrs(&self) -> INTERWORK_INSTRS_R {
        INTERWORK_INSTRS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
