#[doc = "Reader of register ID_DFR0"]
pub type R = crate::R<u32, super::ID_DFR0>;
#[doc = "Support for memory-mapped debug model for M profile processors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEBUGMODEL_A {
    #[doc = "0: Not supported"]
    DEBUGMODEL_0 = 0,
    #[doc = "1: Support for M profile Debug architecture, with memory-mapped access."]
    DEBUGMODEL_1 = 1,
}
impl From<DEBUGMODEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DEBUGMODEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DEBUGMODEL`"]
pub type DEBUGMODEL_R = crate::R<u8, DEBUGMODEL_A>;
impl DEBUGMODEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DEBUGMODEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DEBUGMODEL_A::DEBUGMODEL_0),
            1 => Val(DEBUGMODEL_A::DEBUGMODEL_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEBUGMODEL_0`"]
    #[inline(always)]
    pub fn is_debugmodel_0(&self) -> bool {
        *self == DEBUGMODEL_A::DEBUGMODEL_0
    }
    #[doc = "Checks if the value of the field is `DEBUGMODEL_1`"]
    #[inline(always)]
    pub fn is_debugmodel_1(&self) -> bool {
        *self == DEBUGMODEL_A::DEBUGMODEL_1
    }
}
impl R {
    #[doc = "Bits 20:23 - Support for memory-mapped debug model for M profile processors"]
    #[inline(always)]
    pub fn debugmodel(&self) -> DEBUGMODEL_R {
        DEBUGMODEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
