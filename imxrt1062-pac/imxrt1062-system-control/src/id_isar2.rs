#[doc = "Reader of register ID_ISAR2"]
pub type R = crate::R<u32, super::ID_ISAR2>;
#[doc = "Indicates the supported additional load and store instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOADSTORE_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    LOADSTORE_INSTRS_0 = 0,
    #[doc = "1: Adds support for the LDRD and STRD instructions"]
    LOADSTORE_INSTRS_1 = 1,
}
impl From<LOADSTORE_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: LOADSTORE_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LOADSTORE_INSTRS`"]
pub type LOADSTORE_INSTRS_R = crate::R<u8, LOADSTORE_INSTRS_A>;
impl LOADSTORE_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOADSTORE_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOADSTORE_INSTRS_A::LOADSTORE_INSTRS_0),
            1 => Val(LOADSTORE_INSTRS_A::LOADSTORE_INSTRS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOADSTORE_INSTRS_0`"]
    #[inline(always)]
    pub fn is_loadstore_instrs_0(&self) -> bool {
        *self == LOADSTORE_INSTRS_A::LOADSTORE_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `LOADSTORE_INSTRS_1`"]
    #[inline(always)]
    pub fn is_loadstore_instrs_1(&self) -> bool {
        *self == LOADSTORE_INSTRS_A::LOADSTORE_INSTRS_1
    }
}
#[doc = "Indicates the supported Memory Hint instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MEMHINT_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    MEMHINT_INSTRS_0 = 0,
    #[doc = "1: Adds support for the PLD instruction, ARMv7-M unused."]
    MEMHINT_INSTRS_1 = 1,
    #[doc = "2: As for 1, ARMv7-M unused."]
    MEMHINT_INSTRS_2 = 2,
    #[doc = "3: As for 1 or 2, and adds support for the PLI instruction."]
    MEMHINT_INSTRS_3 = 3,
}
impl From<MEMHINT_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: MEMHINT_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MEMHINT_INSTRS`"]
pub type MEMHINT_INSTRS_R = crate::R<u8, MEMHINT_INSTRS_A>;
impl MEMHINT_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MEMHINT_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MEMHINT_INSTRS_A::MEMHINT_INSTRS_0),
            1 => Val(MEMHINT_INSTRS_A::MEMHINT_INSTRS_1),
            2 => Val(MEMHINT_INSTRS_A::MEMHINT_INSTRS_2),
            3 => Val(MEMHINT_INSTRS_A::MEMHINT_INSTRS_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MEMHINT_INSTRS_0`"]
    #[inline(always)]
    pub fn is_memhint_instrs_0(&self) -> bool {
        *self == MEMHINT_INSTRS_A::MEMHINT_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MEMHINT_INSTRS_1`"]
    #[inline(always)]
    pub fn is_memhint_instrs_1(&self) -> bool {
        *self == MEMHINT_INSTRS_A::MEMHINT_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MEMHINT_INSTRS_2`"]
    #[inline(always)]
    pub fn is_memhint_instrs_2(&self) -> bool {
        *self == MEMHINT_INSTRS_A::MEMHINT_INSTRS_2
    }
    #[doc = "Checks if the value of the field is `MEMHINT_INSTRS_3`"]
    #[inline(always)]
    pub fn is_memhint_instrs_3(&self) -> bool {
        *self == MEMHINT_INSTRS_A::MEMHINT_INSTRS_3
    }
}
#[doc = "Indicates the support for multi-access interruptible instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MULTIACCESSINT_INSTRS_A {
    #[doc = "0: None supported. This means the LDM and STM instructions are not interruptible. ARMv7-M unused."]
    MULTIACCESSINT_INSTRS_0 = 0,
    #[doc = "1: LDM and STM instructions are restartable."]
    MULTIACCESSINT_INSTRS_1 = 1,
    #[doc = "2: LDM and STM instructions are continuable."]
    MULTIACCESSINT_INSTRS_2 = 2,
}
impl From<MULTIACCESSINT_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: MULTIACCESSINT_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MULTIACCESSINT_INSTRS`"]
pub type MULTIACCESSINT_INSTRS_R = crate::R<u8, MULTIACCESSINT_INSTRS_A>;
impl MULTIACCESSINT_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MULTIACCESSINT_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MULTIACCESSINT_INSTRS_A::MULTIACCESSINT_INSTRS_0),
            1 => Val(MULTIACCESSINT_INSTRS_A::MULTIACCESSINT_INSTRS_1),
            2 => Val(MULTIACCESSINT_INSTRS_A::MULTIACCESSINT_INSTRS_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MULTIACCESSINT_INSTRS_0`"]
    #[inline(always)]
    pub fn is_multiaccessint_instrs_0(&self) -> bool {
        *self == MULTIACCESSINT_INSTRS_A::MULTIACCESSINT_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MULTIACCESSINT_INSTRS_1`"]
    #[inline(always)]
    pub fn is_multiaccessint_instrs_1(&self) -> bool {
        *self == MULTIACCESSINT_INSTRS_A::MULTIACCESSINT_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MULTIACCESSINT_INSTRS_2`"]
    #[inline(always)]
    pub fn is_multiaccessint_instrs_2(&self) -> bool {
        *self == MULTIACCESSINT_INSTRS_A::MULTIACCESSINT_INSTRS_2
    }
}
#[doc = "Indicates the supported additional Multiply instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MULT_INSTRS_A {
    #[doc = "0: None supported. This means only MUL is supported. ARMv7-M unused."]
    MULT_INSTRS_0 = 0,
    #[doc = "1: Adds support for the MLA instruction, ARMv7-M unused."]
    MULT_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the MLS instruction."]
    MULT_INSTRS_2 = 2,
}
impl From<MULT_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: MULT_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MULT_INSTRS`"]
pub type MULT_INSTRS_R = crate::R<u8, MULT_INSTRS_A>;
impl MULT_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MULT_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MULT_INSTRS_A::MULT_INSTRS_0),
            1 => Val(MULT_INSTRS_A::MULT_INSTRS_1),
            2 => Val(MULT_INSTRS_A::MULT_INSTRS_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MULT_INSTRS_0`"]
    #[inline(always)]
    pub fn is_mult_instrs_0(&self) -> bool {
        *self == MULT_INSTRS_A::MULT_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MULT_INSTRS_1`"]
    #[inline(always)]
    pub fn is_mult_instrs_1(&self) -> bool {
        *self == MULT_INSTRS_A::MULT_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MULT_INSTRS_2`"]
    #[inline(always)]
    pub fn is_mult_instrs_2(&self) -> bool {
        *self == MULT_INSTRS_A::MULT_INSTRS_2
    }
}
#[doc = "Indicates the supported advanced signed Multiply instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MULTS_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    MULTS_INSTRS_0 = 0,
    #[doc = "1: Adds support for the SMULL and SMLAL instructions"]
    MULTS_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the SMLABB, SMLABT, SMLALBB, SMLALBT, SMLALTB, SMLALTT, SMLATB, SMLATT, SMLAWB, SMLAWT, SMULBB, SMULBT, SMULTB, SMULTT, SMULWB, and SMULWT instructions."]
    MULTS_INSTRS_2 = 2,
    #[doc = "3: As for 2, and adds support for the SMLAD, SMLADX, SMLALD, SMLALDX, SMLSD, SMLSDX, SMLSLD, SMLSLDX, SMMLA, SMMLAR, SMMLS, SMMLSR, SMMUL, SMMULR, SMUAD, SMUADX, SMUSD, and SMUSDX instructions."]
    MULTS_INSTRS_3 = 3,
}
impl From<MULTS_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: MULTS_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MULTS_INSTRS`"]
pub type MULTS_INSTRS_R = crate::R<u8, MULTS_INSTRS_A>;
impl MULTS_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MULTS_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MULTS_INSTRS_A::MULTS_INSTRS_0),
            1 => Val(MULTS_INSTRS_A::MULTS_INSTRS_1),
            2 => Val(MULTS_INSTRS_A::MULTS_INSTRS_2),
            3 => Val(MULTS_INSTRS_A::MULTS_INSTRS_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MULTS_INSTRS_0`"]
    #[inline(always)]
    pub fn is_mults_instrs_0(&self) -> bool {
        *self == MULTS_INSTRS_A::MULTS_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MULTS_INSTRS_1`"]
    #[inline(always)]
    pub fn is_mults_instrs_1(&self) -> bool {
        *self == MULTS_INSTRS_A::MULTS_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MULTS_INSTRS_2`"]
    #[inline(always)]
    pub fn is_mults_instrs_2(&self) -> bool {
        *self == MULTS_INSTRS_A::MULTS_INSTRS_2
    }
    #[doc = "Checks if the value of the field is `MULTS_INSTRS_3`"]
    #[inline(always)]
    pub fn is_mults_instrs_3(&self) -> bool {
        *self == MULTS_INSTRS_A::MULTS_INSTRS_3
    }
}
#[doc = "Indicates the supported advanced unsigned Multiply instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MULTU_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    MULTU_INSTRS_0 = 0,
    #[doc = "1: Adds support for the UMULL and UMLAL instructions."]
    MULTU_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the UMAAL instruction."]
    MULTU_INSTRS_2 = 2,
}
impl From<MULTU_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: MULTU_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MULTU_INSTRS`"]
pub type MULTU_INSTRS_R = crate::R<u8, MULTU_INSTRS_A>;
impl MULTU_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MULTU_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MULTU_INSTRS_A::MULTU_INSTRS_0),
            1 => Val(MULTU_INSTRS_A::MULTU_INSTRS_1),
            2 => Val(MULTU_INSTRS_A::MULTU_INSTRS_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MULTU_INSTRS_0`"]
    #[inline(always)]
    pub fn is_multu_instrs_0(&self) -> bool {
        *self == MULTU_INSTRS_A::MULTU_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `MULTU_INSTRS_1`"]
    #[inline(always)]
    pub fn is_multu_instrs_1(&self) -> bool {
        *self == MULTU_INSTRS_A::MULTU_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `MULTU_INSTRS_2`"]
    #[inline(always)]
    pub fn is_multu_instrs_2(&self) -> bool {
        *self == MULTU_INSTRS_A::MULTU_INSTRS_2
    }
}
#[doc = "Indicates the supported Reversal instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REVERSAL_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused"]
    REVERSAL_INSTRS_0 = 0,
    #[doc = "1: Adds support for the REV, REV16, and REVSH instructions, ARMv7-M unused."]
    REVERSAL_INSTRS_1 = 1,
    #[doc = "2: As for 1, and adds support for the RBIT instruction."]
    REVERSAL_INSTRS_2 = 2,
}
impl From<REVERSAL_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: REVERSAL_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REVERSAL_INSTRS`"]
pub type REVERSAL_INSTRS_R = crate::R<u8, REVERSAL_INSTRS_A>;
impl REVERSAL_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REVERSAL_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REVERSAL_INSTRS_A::REVERSAL_INSTRS_0),
            1 => Val(REVERSAL_INSTRS_A::REVERSAL_INSTRS_1),
            2 => Val(REVERSAL_INSTRS_A::REVERSAL_INSTRS_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REVERSAL_INSTRS_0`"]
    #[inline(always)]
    pub fn is_reversal_instrs_0(&self) -> bool {
        *self == REVERSAL_INSTRS_A::REVERSAL_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `REVERSAL_INSTRS_1`"]
    #[inline(always)]
    pub fn is_reversal_instrs_1(&self) -> bool {
        *self == REVERSAL_INSTRS_A::REVERSAL_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `REVERSAL_INSTRS_2`"]
    #[inline(always)]
    pub fn is_reversal_instrs_2(&self) -> bool {
        *self == REVERSAL_INSTRS_A::REVERSAL_INSTRS_2
    }
}
impl R {
    #[doc = "Bits 0:3 - Indicates the supported additional load and store instructions"]
    #[inline(always)]
    pub fn loadstore_instrs(&self) -> LOADSTORE_INSTRS_R {
        LOADSTORE_INSTRS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates the supported Memory Hint instructions"]
    #[inline(always)]
    pub fn memhint_instrs(&self) -> MEMHINT_INSTRS_R {
        MEMHINT_INSTRS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the support for multi-access interruptible instructions"]
    #[inline(always)]
    pub fn multiaccessint_instrs(&self) -> MULTIACCESSINT_INSTRS_R {
        MULTIACCESSINT_INSTRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Indicates the supported additional Multiply instructions"]
    #[inline(always)]
    pub fn mult_instrs(&self) -> MULT_INSTRS_R {
        MULT_INSTRS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the supported advanced signed Multiply instructions"]
    #[inline(always)]
    pub fn mults_instrs(&self) -> MULTS_INSTRS_R {
        MULTS_INSTRS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the supported advanced unsigned Multiply instructions"]
    #[inline(always)]
    pub fn multu_instrs(&self) -> MULTU_INSTRS_R {
        MULTU_INSTRS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Indicates the supported Reversal instructions"]
    #[inline(always)]
    pub fn reversal_instrs(&self) -> REVERSAL_INSTRS_R {
        REVERSAL_INSTRS_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
