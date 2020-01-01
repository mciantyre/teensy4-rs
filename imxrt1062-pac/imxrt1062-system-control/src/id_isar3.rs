#[doc = "Reader of register ID_ISAR3"]
pub type R = crate::R<u32, super::ID_ISAR3>;
#[doc = "Indicates the supported Saturate instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SATURATE_INSTRS_A {
    #[doc = "0: None supported"]
    SATURATE_INSTRS_0 = 0,
    #[doc = "1: Adds support for the QADD, QDADD, QDSUB, and QSUB instructions, and for the Q bit in the PSRs."]
    SATURATE_INSTRS_1 = 1,
}
impl From<SATURATE_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: SATURATE_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SATURATE_INSTRS`"]
pub type SATURATE_INSTRS_R = crate::R<u8, SATURATE_INSTRS_A>;
impl SATURATE_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SATURATE_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SATURATE_INSTRS_A::SATURATE_INSTRS_0),
            1 => Val(SATURATE_INSTRS_A::SATURATE_INSTRS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SATURATE_INSTRS_0`"]
    #[inline(always)]
    pub fn is_saturate_instrs_0(&self) -> bool {
        *self == SATURATE_INSTRS_A::SATURATE_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `SATURATE_INSTRS_1`"]
    #[inline(always)]
    pub fn is_saturate_instrs_1(&self) -> bool {
        *self == SATURATE_INSTRS_A::SATURATE_INSTRS_1
    }
}
#[doc = "Indicates the supported SIMD instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIMD_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    SIMD_INSTRS_0 = 0,
    #[doc = "1: Adds support for the SSAT and USAT instructions, and for the Q bit in the PSRs."]
    SIMD_INSTRS_1 = 1,
    #[doc = "3: As for 1, and adds support for the PKHBT, PKHTB, QADD16, QADD8, QASX, QSUB16, QSUB8, QSAX, SADD16, SADD8, SASX, SEL, SHADD16, SHADD8, SHASX, SHSUB16, SHSUB8, SHSAX, SSAT16, SSUB16, SSUB8, SSAX, SXTAB16, SXTB16, UADD16, UADD8, UASX, UHADD16, UHADD8, UHASX, UHSUB16, UHSUB8, UHSAX, UQADD16, UQADD8, UQASX, UQSUB16, UQSUB8, UQSAX, USAD8, USADA8, USAT16, USUB16, USUB8, USAX, UXTAB16, and UXTB16 instructions. Also adds support for the GE\\[3:0\\]
bits in the PSRs."]
    SIMD_INSTRS_3 = 3,
}
impl From<SIMD_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: SIMD_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIMD_INSTRS`"]
pub type SIMD_INSTRS_R = crate::R<u8, SIMD_INSTRS_A>;
impl SIMD_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SIMD_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SIMD_INSTRS_A::SIMD_INSTRS_0),
            1 => Val(SIMD_INSTRS_A::SIMD_INSTRS_1),
            3 => Val(SIMD_INSTRS_A::SIMD_INSTRS_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SIMD_INSTRS_0`"]
    #[inline(always)]
    pub fn is_simd_instrs_0(&self) -> bool {
        *self == SIMD_INSTRS_A::SIMD_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `SIMD_INSTRS_1`"]
    #[inline(always)]
    pub fn is_simd_instrs_1(&self) -> bool {
        *self == SIMD_INSTRS_A::SIMD_INSTRS_1
    }
    #[doc = "Checks if the value of the field is `SIMD_INSTRS_3`"]
    #[inline(always)]
    pub fn is_simd_instrs_3(&self) -> bool {
        *self == SIMD_INSTRS_A::SIMD_INSTRS_3
    }
}
#[doc = "Indicates the supported SVC instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SVC_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    SVC_INSTRS_0 = 0,
    #[doc = "1: Adds support for the SVC instruction."]
    SVC_INSTRS_1 = 1,
}
impl From<SVC_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: SVC_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SVC_INSTRS`"]
pub type SVC_INSTRS_R = crate::R<u8, SVC_INSTRS_A>;
impl SVC_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SVC_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SVC_INSTRS_A::SVC_INSTRS_0),
            1 => Val(SVC_INSTRS_A::SVC_INSTRS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SVC_INSTRS_0`"]
    #[inline(always)]
    pub fn is_svc_instrs_0(&self) -> bool {
        *self == SVC_INSTRS_A::SVC_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `SVC_INSTRS_1`"]
    #[inline(always)]
    pub fn is_svc_instrs_1(&self) -> bool {
        *self == SVC_INSTRS_A::SVC_INSTRS_1
    }
}
#[doc = "Reader of field `SYNCHPRIM_INSTRS`"]
pub type SYNCHPRIM_INSTRS_R = crate::R<u8, u8>;
#[doc = "Indicates the supported Table Branch instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TABBRANCH_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    TABBRANCH_INSTRS_0 = 0,
    #[doc = "1: Adds support for the TBB and TBH instructions."]
    TABBRANCH_INSTRS_1 = 1,
}
impl From<TABBRANCH_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: TABBRANCH_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TABBRANCH_INSTRS`"]
pub type TABBRANCH_INSTRS_R = crate::R<u8, TABBRANCH_INSTRS_A>;
impl TABBRANCH_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TABBRANCH_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TABBRANCH_INSTRS_A::TABBRANCH_INSTRS_0),
            1 => Val(TABBRANCH_INSTRS_A::TABBRANCH_INSTRS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TABBRANCH_INSTRS_0`"]
    #[inline(always)]
    pub fn is_tabbranch_instrs_0(&self) -> bool {
        *self == TABBRANCH_INSTRS_A::TABBRANCH_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `TABBRANCH_INSTRS_1`"]
    #[inline(always)]
    pub fn is_tabbranch_instrs_1(&self) -> bool {
        *self == TABBRANCH_INSTRS_A::TABBRANCH_INSTRS_1
    }
}
#[doc = "Indicates the supported non flag-setting MOV instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum THUMBCOPY_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    THUMBCOPY_INSTRS_0 = 0,
    #[doc = "1: Adds support for encoding T1 of the MOV (register) instruction copying from a low register to a low register."]
    THUMBCOPY_INSTRS_1 = 1,
}
impl From<THUMBCOPY_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: THUMBCOPY_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `THUMBCOPY_INSTRS`"]
pub type THUMBCOPY_INSTRS_R = crate::R<u8, THUMBCOPY_INSTRS_A>;
impl THUMBCOPY_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, THUMBCOPY_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(THUMBCOPY_INSTRS_A::THUMBCOPY_INSTRS_0),
            1 => Val(THUMBCOPY_INSTRS_A::THUMBCOPY_INSTRS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `THUMBCOPY_INSTRS_0`"]
    #[inline(always)]
    pub fn is_thumbcopy_instrs_0(&self) -> bool {
        *self == THUMBCOPY_INSTRS_A::THUMBCOPY_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `THUMBCOPY_INSTRS_1`"]
    #[inline(always)]
    pub fn is_thumbcopy_instrs_1(&self) -> bool {
        *self == THUMBCOPY_INSTRS_A::THUMBCOPY_INSTRS_1
    }
}
#[doc = "Indicates the supported non flag-setting MOV instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRUENOP_INSTRS_A {
    #[doc = "0: None supported, ARMv7-M unused."]
    TRUENOP_INSTRS_0 = 0,
    #[doc = "1: Adds support for encoding T1 of the MOV (register) instruction copying from a low register to a low register."]
    TRUENOP_INSTRS_1 = 1,
}
impl From<TRUENOP_INSTRS_A> for u8 {
    #[inline(always)]
    fn from(variant: TRUENOP_INSTRS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRUENOP_INSTRS`"]
pub type TRUENOP_INSTRS_R = crate::R<u8, TRUENOP_INSTRS_A>;
impl TRUENOP_INSTRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRUENOP_INSTRS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRUENOP_INSTRS_A::TRUENOP_INSTRS_0),
            1 => Val(TRUENOP_INSTRS_A::TRUENOP_INSTRS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TRUENOP_INSTRS_0`"]
    #[inline(always)]
    pub fn is_truenop_instrs_0(&self) -> bool {
        *self == TRUENOP_INSTRS_A::TRUENOP_INSTRS_0
    }
    #[doc = "Checks if the value of the field is `TRUENOP_INSTRS_1`"]
    #[inline(always)]
    pub fn is_truenop_instrs_1(&self) -> bool {
        *self == TRUENOP_INSTRS_A::TRUENOP_INSTRS_1
    }
}
impl R {
    #[doc = "Bits 0:3 - Indicates the supported Saturate instructions"]
    #[inline(always)]
    pub fn saturate_instrs(&self) -> SATURATE_INSTRS_R {
        SATURATE_INSTRS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates the supported SIMD instructions"]
    #[inline(always)]
    pub fn simd_instrs(&self) -> SIMD_INSTRS_R {
        SIMD_INSTRS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the supported SVC instructions"]
    #[inline(always)]
    pub fn svc_instrs(&self) -> SVC_INSTRS_R {
        SVC_INSTRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Together with the ID_ISAR4\\[SYNCHPRIM_INSTRS_FRAC\\]
indicates the supported Synchronization Primitives"]
    #[inline(always)]
    pub fn synchprim_instrs(&self) -> SYNCHPRIM_INSTRS_R {
        SYNCHPRIM_INSTRS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the supported Table Branch instructions"]
    #[inline(always)]
    pub fn tabbranch_instrs(&self) -> TABBRANCH_INSTRS_R {
        TABBRANCH_INSTRS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the supported non flag-setting MOV instructions"]
    #[inline(always)]
    pub fn thumbcopy_instrs(&self) -> THUMBCOPY_INSTRS_R {
        THUMBCOPY_INSTRS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the supported non flag-setting MOV instructions"]
    #[inline(always)]
    pub fn truenop_instrs(&self) -> TRUENOP_INSTRS_R {
        TRUENOP_INSTRS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
