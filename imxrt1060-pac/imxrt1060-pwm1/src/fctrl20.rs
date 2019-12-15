#[doc = "Reader of register FCTRL20"]
pub type R = crate::R<u16, super::FCTRL20>;
#[doc = "Writer for register FCTRL20"]
pub type W = crate::W<u16, super::FCTRL20>;
#[doc = "Register FCTRL20 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCTRL20 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "No Combinational Path From Fault Input To PWM Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOCOMB_A {
    #[doc = "0: There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs."]
    NOCOMB_0,
    #[doc = "1: The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs."]
    NOCOMB_1,
}
impl From<NOCOMB_A> for u8 {
    #[inline(always)]
    fn from(variant: NOCOMB_A) -> Self {
        match variant {
            NOCOMB_A::NOCOMB_0 => 0,
            NOCOMB_A::NOCOMB_1 => 1,
        }
    }
}
#[doc = "Reader of field `NOCOMB`"]
pub type NOCOMB_R = crate::R<u8, NOCOMB_A>;
impl NOCOMB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NOCOMB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NOCOMB_A::NOCOMB_0),
            1 => Val(NOCOMB_A::NOCOMB_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOCOMB_0`"]
    #[inline(always)]
    pub fn is_nocomb_0(&self) -> bool {
        *self == NOCOMB_A::NOCOMB_0
    }
    #[doc = "Checks if the value of the field is `NOCOMB_1`"]
    #[inline(always)]
    pub fn is_nocomb_1(&self) -> bool {
        *self == NOCOMB_A::NOCOMB_1
    }
}
#[doc = "Write proxy for field `NOCOMB`"]
pub struct NOCOMB_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCOMB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOCOMB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs."]
    #[inline(always)]
    pub fn nocomb_0(self) -> &'a mut W {
        self.variant(NOCOMB_A::NOCOMB_0)
    }
    #[doc = "The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs."]
    #[inline(always)]
    pub fn nocomb_1(self) -> &'a mut W {
        self.variant(NOCOMB_A::NOCOMB_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - No Combinational Path From Fault Input To PWM Output"]
    #[inline(always)]
    pub fn nocomb(&self) -> NOCOMB_R {
        NOCOMB_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - No Combinational Path From Fault Input To PWM Output"]
    #[inline(always)]
    pub fn nocomb(&mut self) -> NOCOMB_W {
        NOCOMB_W { w: self }
    }
}
