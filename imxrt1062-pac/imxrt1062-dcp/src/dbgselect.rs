#[doc = "Reader of register DBGSELECT"]
pub type R = crate::R<u32, super::DBGSELECT>;
#[doc = "Writer for register DBGSELECT"]
pub type W = crate::W<u32, super::DBGSELECT>;
#[doc = "Register DBGSELECT `reset()`'s with value 0"]
impl crate::ResetValue for super::DBGSELECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects a value to read via the debug data register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INDEX_A {
    #[doc = "1: CONTROL"]
    CONTROL = 1,
    #[doc = "16: OTPKEY0"]
    OTPKEY0 = 16,
    #[doc = "17: OTPKEY1"]
    OTPKEY1 = 17,
    #[doc = "18: OTPKEY2"]
    OTPKEY2 = 18,
    #[doc = "19: OTPKEY3"]
    OTPKEY3 = 19,
}
impl From<INDEX_A> for u8 {
    #[inline(always)]
    fn from(variant: INDEX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INDEX`"]
pub type INDEX_R = crate::R<u8, INDEX_A>;
impl INDEX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INDEX_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(INDEX_A::CONTROL),
            16 => Val(INDEX_A::OTPKEY0),
            17 => Val(INDEX_A::OTPKEY1),
            18 => Val(INDEX_A::OTPKEY2),
            19 => Val(INDEX_A::OTPKEY3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == INDEX_A::CONTROL
    }
    #[doc = "Checks if the value of the field is `OTPKEY0`"]
    #[inline(always)]
    pub fn is_otpkey0(&self) -> bool {
        *self == INDEX_A::OTPKEY0
    }
    #[doc = "Checks if the value of the field is `OTPKEY1`"]
    #[inline(always)]
    pub fn is_otpkey1(&self) -> bool {
        *self == INDEX_A::OTPKEY1
    }
    #[doc = "Checks if the value of the field is `OTPKEY2`"]
    #[inline(always)]
    pub fn is_otpkey2(&self) -> bool {
        *self == INDEX_A::OTPKEY2
    }
    #[doc = "Checks if the value of the field is `OTPKEY3`"]
    #[inline(always)]
    pub fn is_otpkey3(&self) -> bool {
        *self == INDEX_A::OTPKEY3
    }
}
#[doc = "Write proxy for field `INDEX`"]
pub struct INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> INDEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INDEX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CONTROL"]
    #[inline(always)]
    pub fn control(self) -> &'a mut W {
        self.variant(INDEX_A::CONTROL)
    }
    #[doc = "OTPKEY0"]
    #[inline(always)]
    pub fn otpkey0(self) -> &'a mut W {
        self.variant(INDEX_A::OTPKEY0)
    }
    #[doc = "OTPKEY1"]
    #[inline(always)]
    pub fn otpkey1(self) -> &'a mut W {
        self.variant(INDEX_A::OTPKEY1)
    }
    #[doc = "OTPKEY2"]
    #[inline(always)]
    pub fn otpkey2(self) -> &'a mut W {
        self.variant(INDEX_A::OTPKEY2)
    }
    #[doc = "OTPKEY3"]
    #[inline(always)]
    pub fn otpkey3(self) -> &'a mut W {
        self.variant(INDEX_A::OTPKEY3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects a value to read via the debug data register."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects a value to read via the debug data register."]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W {
        INDEX_W { w: self }
    }
}
