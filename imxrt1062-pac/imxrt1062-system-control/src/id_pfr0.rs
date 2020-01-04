#[doc = "Reader of register ID_PFR0"]
pub type R = crate::R<u32, super::ID_PFR0>;
#[doc = "ARM instruction set support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE0_A {
    #[doc = "0: ARMv7-M unused"]
    STATE0_0 = 0,
    #[doc = "1: ARMv7-M unused"]
    STATE0_1 = 1,
    #[doc = "2: ARMv7-M unused"]
    STATE0_2 = 2,
    #[doc = "3: Support for Thumb encoding including Thumb-2 technology, with all basic 16-bit and 32-bit instructions."]
    STATE0_3 = 3,
}
impl From<STATE0_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATE0`"]
pub type STATE0_R = crate::R<u8, STATE0_A>;
impl STATE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATE0_A::STATE0_0),
            1 => Val(STATE0_A::STATE0_1),
            2 => Val(STATE0_A::STATE0_2),
            3 => Val(STATE0_A::STATE0_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STATE0_0`"]
    #[inline(always)]
    pub fn is_state0_0(&self) -> bool {
        *self == STATE0_A::STATE0_0
    }
    #[doc = "Checks if the value of the field is `STATE0_1`"]
    #[inline(always)]
    pub fn is_state0_1(&self) -> bool {
        *self == STATE0_A::STATE0_1
    }
    #[doc = "Checks if the value of the field is `STATE0_2`"]
    #[inline(always)]
    pub fn is_state0_2(&self) -> bool {
        *self == STATE0_A::STATE0_2
    }
    #[doc = "Checks if the value of the field is `STATE0_3`"]
    #[inline(always)]
    pub fn is_state0_3(&self) -> bool {
        *self == STATE0_A::STATE0_3
    }
}
#[doc = "Thumb instruction set support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE1_A {
    #[doc = "0: The processor does not support the ARM instruction set."]
    STATE1_0 = 0,
    #[doc = "1: ARMv7-M unused"]
    STATE1_1 = 1,
}
impl From<STATE1_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATE1`"]
pub type STATE1_R = crate::R<u8, STATE1_A>;
impl STATE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATE1_A::STATE1_0),
            1 => Val(STATE1_A::STATE1_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STATE1_0`"]
    #[inline(always)]
    pub fn is_state1_0(&self) -> bool {
        *self == STATE1_A::STATE1_0
    }
    #[doc = "Checks if the value of the field is `STATE1_1`"]
    #[inline(always)]
    pub fn is_state1_1(&self) -> bool {
        *self == STATE1_A::STATE1_1
    }
}
#[doc = "Reader of field `STATE2`"]
pub type STATE2_R = crate::R<u8, u8>;
#[doc = "Reader of field `STATE3`"]
pub type STATE3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - ARM instruction set support"]
    #[inline(always)]
    pub fn state0(&self) -> STATE0_R {
        STATE0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Thumb instruction set support"]
    #[inline(always)]
    pub fn state1(&self) -> STATE1_R {
        STATE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ARMv7-M unused"]
    #[inline(always)]
    pub fn state2(&self) -> STATE2_R {
        STATE2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - ARMv7-M unused"]
    #[inline(always)]
    pub fn state3(&self) -> STATE3_R {
        STATE3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
