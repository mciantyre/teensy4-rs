#[doc = "Reader of register ROMPATCHENL"]
pub type R = crate::R<u32, super::ROMPATCHENL>;
#[doc = "Writer for register ROMPATCHENL"]
pub type W = crate::W<u32, super::ROMPATCHENL>;
#[doc = "Register ROMPATCHENL `reset()`'s with value 0"]
impl crate::ResetValue for super::ROMPATCHENL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable Address Comparator - This bit enables the corresponding address comparator to trigger an event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum ENABLE_A {
    #[doc = "0: Address comparator disabled"]
    ENABLE_0 = 0,
    #[doc = "1: Address comparator enabled, ROMC will trigger a opcode patch or data fix event upon matching of the associated address"]
    ENABLE_1 = 1,
}
impl From<ENABLE_A> for u16 {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<u16, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ENABLE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ENABLE_A::ENABLE_0),
            1 => Val(ENABLE_A::ENABLE_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_0`"]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        *self == ENABLE_A::ENABLE_0
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        *self == ENABLE_A::ENABLE_1
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Address comparator disabled"]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_0)
    }
    #[doc = "Address comparator enabled, ROMC will trigger a opcode patch or data fix event upon matching of the associated address"]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Enable Address Comparator - This bit enables the corresponding address comparator to trigger an event"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Enable Address Comparator - This bit enables the corresponding address comparator to trigger an event"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
