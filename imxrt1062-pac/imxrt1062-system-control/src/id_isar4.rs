#[doc = "Reader of register ID_ISAR4"]
pub type R = crate::R<u32, super::ID_ISAR4>;
#[doc = "Indicates the supported unprivileged instructions. These are the instruction variants indicated by a T suffix.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UNPRIV_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    UNPRIV_INSTRS_0 = 0,
    #[doc = "1: Adds support for the LDRBT, LDRT, STRBT, and STRT instructions."]
    UNPRIV_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the LDRHT, LDRSBT, LDRSHT, and STRHT instructions."]
    UNPRIV_INSTRS_2 = 2,
}
impl From<UNPRIV_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: UNPRIV_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UNPRIV_INSTRS`"]
pub type UNPRIV_INSTRS_R = crate::R<u8, UNPRIV_INSTRS_A>;
impl UNPRIV_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UNPRIV_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UNPRIV_INSTRS_A::UNPRIV_INSTRS_0),
            1 => Val(UNPRIV_INSTRS_A::UNPRIV_INSTRS_1),
            2 => Val(UNPRIV_INSTRS_A::UNPRIV_INSTRS_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNPRIV_INSTRS_0`"]
    #[inline(always)]
    pub fn is_unpriv_instrs_0(&self) -> bool {
        *self == UNPRIV_INSTRS_A::UNPRIV_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `UNPRIV_INSTRS_1`"]
    #[inline(always)]
    pub fn is_unpriv_instrs_1(&self) -> bool {
        *self == UNPRIV_INSTRS_A::UNPRIV_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `UNPRIV_INSTRS_2`"]
    #[inline(always)]
    pub fn is_unpriv_instrs_2(&self) -> bool {
        *self == UNPRIV_INSTRS_A::UNPRIV_INSTRS_2
    }
}
#[doc = "Indicates the support for instructions with shifts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WITHSHIFTS_INSTRS_A {
    #[doc = "0: Nonzero shifts supported only in MOV and shift instructions."]
    WITHSHIFTS_INSTRS_0 = 0,
    #[doc = "1: Adds support for shifts of loads and stores over the range LSL 0-3."]
    WITHSHIFTS_INSTRS_1 = 1,
    #[doc = "3: As for 1, and adds support for other constant shift options, on loads, stores, and other instructions."]
    WITHSHIFTS_INSTRS_3 = 3,
    #[doc = "4: ARMv7-M unused."]
    WITHSHIFTS_INSTRS_4 = 4,
}
impl From<WITHSHIFTS_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: WITHSHIFTS_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WITHSHIFTS_INSTRS`"]
pub type WITHSHIFTS_INSTRS_R = crate::R<u8, WITHSHIFTS_INSTRS_A>;
impl WITHSHIFTS_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WITHSHIFTS_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WITHSHIFTS_INSTRS_A::WITHSHIFTS_INSTRS_0),
            1 => Val(WITHSHIFTS_INSTRS_A::WITHSHIFTS_INSTRS_1),
            3 => Val(WITHSHIFTS_INSTRS_A::WITHSHIFTS_INSTRS_3),
            4 => Val(WITHSHIFTS_INSTRS_A::WITHSHIFTS_INSTRS_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WITHSHIFTS_INSTRS_0`"]
    #[inline(always)]
    pub fn is_withshifts_instrs_0(&self) -> bool {
        *self == WITHSHIFTS_INSTRS_A::WITHSHIFTS_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `WITHSHIFTS_INSTRS_1`"]
    #[inline(always)]
    pub fn is_withshifts_instrs_1(&self) -> bool {
        *self == WITHSHIFTS_INSTRS_A::WITHSHIFTS_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `WITHSHIFTS_INSTRS_3`"]
    #[inline(always)]
    pub fn is_withshifts_instrs_3(&self) -> bool {
        *self == WITHSHIFTS_INSTRS_A::WITHSHIFTS_INSTRS_3
    }
    #[doc = "Checks if the value of the field is `WITHSHIFTS_INSTRS_4`"]
    #[inline(always)]
    pub fn is_withshifts_instrs_4(&self) -> bool {
        *self == WITHSHIFTS_INSTRS_A::WITHSHIFTS_INSTRS_4
    }
}
#[doc = "Indicates the support for Writeback addressing modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WRITEBACK_INSTRS_A {
    #[doc = "0: Basic support. Only the LDM, STM, PUSH, and POP instructions support writeback addressing modes. ARMv7-M unused."]
    WRITEBACK_INSTRS_0 = 0,
    #[doc = "1: Adds support for all of the writeback addressing modes defined in the ARMv7-M architecture."]
    WRITEBACK_INSTRS_1 = 1,
}
impl From<WRITEBACK_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: WRITEBACK_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WRITEBACK_INSTRS`"]
pub type WRITEBACK_INSTRS_R = crate::R<u8, WRITEBACK_INSTRS_A>;
impl WRITEBACK_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WRITEBACK_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WRITEBACK_INSTRS_A::WRITEBACK_INSTRS_0),
            1 => Val(WRITEBACK_INSTRS_A::WRITEBACK_INSTRS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WRITEBACK_INSTRS_0`"]
    #[inline(always)]
    pub fn is_writeback_instrs_0(&self) -> bool {
        *self == WRITEBACK_INSTRS_A::WRITEBACK_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `WRITEBACK_INSTRS_1`"]
    #[inline(always)]
    pub fn is_writeback_instrs_1(&self) -> bool {
        *self == WRITEBACK_INSTRS_A::WRITEBACK_INSTRS_1
    }
}
#[doc = "Indicates the supported Barrier instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BARRIER_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    BARRIER_INSTRS_0 = 0,
    #[doc = "1: Adds support for the DMB, DSB, and ISB barrier instructions."]
    BARRIER_INSTRS_1 = 1,
}
impl From<BARRIER_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: BARRIER_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BARRIER_INSTRS`"]
pub type BARRIER_INSTRS_R = crate::R<u8, BARRIER_INSTRS_A>;
impl BARRIER_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BARRIER_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BARRIER_INSTRS_A::BARRIER_INSTRS_0),
            1 => Val(BARRIER_INSTRS_A::BARRIER_INSTRS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BARRIER_INSTRS_0`"]
    #[inline(always)]
    pub fn is_barrier_instrs_0(&self) -> bool {
        *self == BARRIER_INSTRS_A::BARRIER_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `BARRIER_INSTRS_1`"]
    #[inline(always)]
    pub fn is_barrier_instrs_1(&self) -> bool {
        *self == BARRIER_INSTRS_A::BARRIER_INSTRS_1
    }
}
#[doc = "Reader of field `SYNCHPRIM_INSTRS_FRAC`"]
pub type SYNCHPRIM_INSTRS_FRAC_R = crate::R<u8, u8>;
#[doc = "Indicates the supported M profile instructions to modify the PSRs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSR_M_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    PSR_M_INSTRS_0 = 0,
    #[doc = "1: Adds support for the M-profile forms of the CPS, MRS, and MSR instructions, to access the PSRs."]
    PSR_M_INSTRS_1 = 1,
}
impl From<PSR_M_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: PSR_M_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSR_M_INSTRS`"]
pub type PSR_M_INSTRS_R = crate::R<u8, PSR_M_INSTRS_A>;
impl PSR_M_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PSR_M_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PSR_M_INSTRS_A::PSR_M_INSTRS_0),
            1 => Val(PSR_M_INSTRS_A::PSR_M_INSTRS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PSR_M_INSTRS_0`"]
    #[inline(always)]
    pub fn is_psr_m_instrs_0(&self) -> bool {
        *self == PSR_M_INSTRS_A::PSR_M_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `PSR_M_INSTRS_1`"]
    #[inline(always)]
    pub fn is_psr_m_instrs_1(&self) -> bool {
        *self == PSR_M_INSTRS_A::PSR_M_INSTRS_1
    }
}
impl R {
    #[doc = "Bits 0:3 - Indicates the supported unprivileged instructions. These are the instruction variants indicated by a T suffix."]
    #[inline(always)]
    pub fn unpriv_instrs(&self) -> UNPRIV_INSTRS_R {
        UNPRIV_INSTRS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates the support for instructions with shifts"]
    #[inline(always)]
    pub fn withshifts_instrs(&self) -> WITHSHIFTS_INSTRS_R {
        WITHSHIFTS_INSTRS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the support for Writeback addressing modes"]
    #[inline(always)]
    pub fn writeback_instrs(&self) -> WRITEBACK_INSTRS_R {
        WRITEBACK_INSTRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the supported Barrier instructions"]
    #[inline(always)]
    pub fn barrier_instrs(&self) -> BARRIER_INSTRS_R {
        BARRIER_INSTRS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Together with the ID_ISAR3\\[SYNCHPRIM_INSTRS\\]
indicates the supported Synchronization Primitives"]
    #[inline(always)]
    pub fn synchprim_instrs_frac(&self) -> SYNCHPRIM_INSTRS_FRAC_R {
        SYNCHPRIM_INSTRS_FRAC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the supported M profile instructions to modify the PSRs"]
    #[inline(always)]
    pub fn psr_m_instrs(&self) -> PSR_M_INSTRS_R {
        PSR_M_INSTRS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
