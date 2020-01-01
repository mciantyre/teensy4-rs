#[doc = "Reader of register SBUSCFG"]
pub type R = crate::R<u32, super::SBUSCFG>;
#[doc = "Writer for register SBUSCFG"]
pub type W = crate::W<u32, super::SBUSCFG>;
#[doc = "Register SBUSCFG `reset()`'s with value 0x02"]
impl crate::ResetValue for super::SBUSCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AHBBRST_A {
    #[doc = "0: Incremental burst of unspecified length only"]
    AHBBRST_0 = 0,
    #[doc = "1: INCR4 burst, then single transfer"]
    AHBBRST_1 = 1,
    #[doc = "2: INCR8 burst, INCR4 burst, then single transfer"]
    AHBBRST_2 = 2,
    #[doc = "3: INCR16 burst, INCR8 burst, INCR4 burst, then single transfer"]
    AHBBRST_3 = 3,
    #[doc = "5: INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_5 = 5,
    #[doc = "6: INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_6 = 6,
    #[doc = "7: INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_7 = 7,
}
impl From<AHBBRST_A> for u8 {
    #[inline(always)]
    fn from(variant: AHBBRST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AHBBRST`"]
pub type AHBBRST_R = crate::R<u8, AHBBRST_A>;
impl AHBBRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AHBBRST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AHBBRST_A::AHBBRST_0),
            1 => Val(AHBBRST_A::AHBBRST_1),
            2 => Val(AHBBRST_A::AHBBRST_2),
            3 => Val(AHBBRST_A::AHBBRST_3),
            5 => Val(AHBBRST_A::AHBBRST_5),
            6 => Val(AHBBRST_A::AHBBRST_6),
            7 => Val(AHBBRST_A::AHBBRST_7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AHBBRST_0`"]
    #[inline(always)]
    pub fn is_ahbbrst_0(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_0
    }
    #[doc = "Checks if the value of the field is `AHBBRST_1`"]
    #[inline(always)]
    pub fn is_ahbbrst_1(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_1
    }
    #[doc = "Checks if the value of the field is `AHBBRST_2`"]
    #[inline(always)]
    pub fn is_ahbbrst_2(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_2
    }
    #[doc = "Checks if the value of the field is `AHBBRST_3`"]
    #[inline(always)]
    pub fn is_ahbbrst_3(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_3
    }
    #[doc = "Checks if the value of the field is `AHBBRST_5`"]
    #[inline(always)]
    pub fn is_ahbbrst_5(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_5
    }
    #[doc = "Checks if the value of the field is `AHBBRST_6`"]
    #[inline(always)]
    pub fn is_ahbbrst_6(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_6
    }
    #[doc = "Checks if the value of the field is `AHBBRST_7`"]
    #[inline(always)]
    pub fn is_ahbbrst_7(&self) -> bool {
        *self == AHBBRST_A::AHBBRST_7
    }
}
#[doc = "Write proxy for field `AHBBRST`"]
pub struct AHBBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBBRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHBBRST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Incremental burst of unspecified length only"]
    #[inline(always)]
    pub fn ahbbrst_0(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_0)
    }
    #[doc = "INCR4 burst, then single transfer"]
    #[inline(always)]
    pub fn ahbbrst_1(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_1)
    }
    #[doc = "INCR8 burst, INCR4 burst, then single transfer"]
    #[inline(always)]
    pub fn ahbbrst_2(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_2)
    }
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then single transfer"]
    #[inline(always)]
    pub fn ahbbrst_3(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_3)
    }
    #[doc = "INCR4 burst, then incremental burst of unspecified length"]
    #[inline(always)]
    pub fn ahbbrst_5(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_5)
    }
    #[doc = "INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    #[inline(always)]
    pub fn ahbbrst_6(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_6)
    }
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    #[inline(always)]
    pub fn ahbbrst_7(self) -> &'a mut W {
        self.variant(AHBBRST_A::AHBBRST_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)"]
    #[inline(always)]
    pub fn ahbbrst(&self) -> AHBBRST_R {
        AHBBRST_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)"]
    #[inline(always)]
    pub fn ahbbrst(&mut self) -> AHBBRST_W {
        AHBBRST_W { w: self }
    }
}
