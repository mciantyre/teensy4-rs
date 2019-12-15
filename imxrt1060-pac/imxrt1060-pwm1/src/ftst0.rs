#[doc = "Reader of register FTST0"]
pub type R = crate::R<u16, super::FTST0>;
#[doc = "Writer for register FTST0"]
pub type W = crate::W<u16, super::FTST0>;
#[doc = "Register FTST0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FTST0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fault Test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTEST_A {
    #[doc = "0: No fault"]
    FTEST_0 = 0,
    #[doc = "1: Cause a simulated fault"]
    FTEST_1 = 1,
}
impl From<FTEST_A> for bool {
    #[inline(always)]
    fn from(variant: FTEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTEST`"]
pub type FTEST_R = crate::R<bool, FTEST_A>;
impl FTEST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTEST_A {
        match self.bits {
            false => FTEST_A::FTEST_0,
            true => FTEST_A::FTEST_1,
        }
    }
    #[doc = "Checks if the value of the field is `FTEST_0`"]
    #[inline(always)]
    pub fn is_ftest_0(&self) -> bool {
        *self == FTEST_A::FTEST_0
    }
    #[doc = "Checks if the value of the field is `FTEST_1`"]
    #[inline(always)]
    pub fn is_ftest_1(&self) -> bool {
        *self == FTEST_A::FTEST_1
    }
}
#[doc = "Write proxy for field `FTEST`"]
pub struct FTEST_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTEST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No fault"]
    #[inline(always)]
    pub fn ftest_0(self) -> &'a mut W {
        self.variant(FTEST_A::FTEST_0)
    }
    #[doc = "Cause a simulated fault"]
    #[inline(always)]
    pub fn ftest_1(self) -> &'a mut W {
        self.variant(FTEST_A::FTEST_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Fault Test"]
    #[inline(always)]
    pub fn ftest(&self) -> FTEST_R {
        FTEST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Test"]
    #[inline(always)]
    pub fn ftest(&mut self) -> FTEST_W {
        FTEST_W { w: self }
    }
}
