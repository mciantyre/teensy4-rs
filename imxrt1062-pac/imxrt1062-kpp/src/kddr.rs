#[doc = "Reader of register KDDR"]
pub type R = crate::R<u16, super::KDDR>;
#[doc = "Writer for register KDDR"]
pub type W = crate::W<u16, super::KDDR>;
#[doc = "Register KDDR `reset()`'s with value 0"]
impl crate::ResetValue for super::KDDR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Keypad Row Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KRDD_A {
    #[doc = "0: ROWn pin configured as an input."]
    INPUT = 0,
    #[doc = "1: ROWn pin configured as an output."]
    OUTPUT = 1,
}
impl From<KRDD_A> for u8 {
    #[inline(always)]
    fn from(variant: KRDD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KRDD`"]
pub type KRDD_R = crate::R<u8, KRDD_A>;
impl KRDD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KRDD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(KRDD_A::INPUT),
            1 => Val(KRDD_A::OUTPUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == KRDD_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == KRDD_A::OUTPUT
    }
}
#[doc = "Write proxy for field `KRDD`"]
pub struct KRDD_W<'a> {
    w: &'a mut W,
}
impl<'a> KRDD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KRDD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ROWn pin configured as an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(KRDD_A::INPUT)
    }
    #[doc = "ROWn pin configured as an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(KRDD_A::OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Keypad Column Data Direction Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KCDD_A {
    #[doc = "0: COLn pin is configured as an input."]
    INPUT = 0,
    #[doc = "1: COLn pin is configured as an output."]
    OUTPUT = 1,
}
impl From<KCDD_A> for u8 {
    #[inline(always)]
    fn from(variant: KCDD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KCDD`"]
pub type KCDD_R = crate::R<u8, KCDD_A>;
impl KCDD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KCDD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(KCDD_A::INPUT),
            1 => Val(KCDD_A::OUTPUT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == KCDD_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == KCDD_A::OUTPUT
    }
}
#[doc = "Write proxy for field `KCDD`"]
pub struct KCDD_W<'a> {
    w: &'a mut W,
}
impl<'a> KCDD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KCDD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "COLn pin is configured as an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(KCDD_A::INPUT)
    }
    #[doc = "COLn pin is configured as an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(KCDD_A::OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Keypad Row Data Direction"]
    #[inline(always)]
    pub fn krdd(&self) -> KRDD_R {
        KRDD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Keypad Column Data Direction Register"]
    #[inline(always)]
    pub fn kcdd(&self) -> KCDD_R {
        KCDD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Keypad Row Data Direction"]
    #[inline(always)]
    pub fn krdd(&mut self) -> KRDD_W {
        KRDD_W { w: self }
    }
    #[doc = "Bits 8:15 - Keypad Column Data Direction Register"]
    #[inline(always)]
    pub fn kcdd(&mut self) -> KCDD_W {
        KCDD_W { w: self }
    }
}
