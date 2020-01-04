#[doc = "Reader of register ID_PFR1"]
pub type R = crate::R<u32, super::ID_PFR1>;
#[doc = "M profile programmers' model\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PROGMODEL_A {
    #[doc = "0: ARMv7-M unused"]
    PROGMODEL_0 = 0,
    #[doc = "2: Two-stack programmers' model supported"]
    PROGMODEL_2 = 2,
}
impl From<PROGMODEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PROGMODEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PROGMODEL`"]
pub type PROGMODEL_R = crate::R<u8, PROGMODEL_A>;
impl PROGMODEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PROGMODEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PROGMODEL_A::PROGMODEL_0),
            2 => Val(PROGMODEL_A::PROGMODEL_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PROGMODEL_0`"]
    #[inline(always)]
    pub fn is_progmodel_0(&self) -> bool {
        *self == PROGMODEL_A::PROGMODEL_0
    }
    #[doc = "Checks if the value of the field is `PROGMODEL_2`"]
    #[inline(always)]
    pub fn is_progmodel_2(&self) -> bool {
        *self == PROGMODEL_A::PROGMODEL_2
    }
}
impl R {
    #[doc = "Bits 8:11 - M profile programmers' model"]
    #[inline(always)]
    pub fn progmodel(&self) -> PROGMODEL_R {
        PROGMODEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
