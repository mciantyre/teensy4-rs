#[doc = "Reader of register TFLG"]
pub type R = crate::R<u32, super::TFLG>;
#[doc = "Writer for register TFLG"]
pub type W = crate::W<u32, super::TFLG>;
#[doc = "Register TFLG `reset()`'s with value 0"]
impl crate::ResetValue for super::TFLG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF_A {
    #[doc = "0: Timeout has not yet occurred."]
    TIF_0 = 0,
    #[doc = "1: Timeout has occurred."]
    TIF_1 = 1,
}
impl From<TIF_A> for bool {
    #[inline(always)]
    fn from(variant: TIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIF`"]
pub type TIF_R = crate::R<bool, TIF_A>;
impl TIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF_A {
        match self.bits {
            false => TIF_A::TIF_0,
            true => TIF_A::TIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIF_0`"]
    #[inline(always)]
    pub fn is_tif_0(&self) -> bool {
        *self == TIF_A::TIF_0
    }
    #[doc = "Checks if the value of the field is `TIF_1`"]
    #[inline(always)]
    pub fn is_tif_1(&self) -> bool {
        *self == TIF_A::TIF_1
    }
}
#[doc = "Write proxy for field `TIF`"]
pub struct TIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timeout has not yet occurred."]
    #[inline(always)]
    pub fn tif_0(self) -> &'a mut W {
        self.variant(TIF_A::TIF_0)
    }
    #[doc = "Timeout has occurred."]
    #[inline(always)]
    pub fn tif_1(self) -> &'a mut W {
        self.variant(TIF_A::TIF_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W {
        TIF_W { w: self }
    }
}
