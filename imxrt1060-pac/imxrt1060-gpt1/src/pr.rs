#[doc = "Reader of register PR"]
pub type R = crate::R<u32, super::PR>;
#[doc = "Writer for register PR"]
pub type W = crate::W<u32, super::PR>;
#[doc = "Register PR `reset()`'s with value 0"]
impl crate::ResetValue for super::PR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescaler bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER_A {
    #[doc = "0: Divide by 1"]
    PRESCALER_0,
    #[doc = "1: Divide by 2"]
    PRESCALER_1,
    #[doc = "4095: Divide by 4096"]
    PRESCALER_4095,
}
impl From<PRESCALER_A> for u16 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        match variant {
            PRESCALER_A::PRESCALER_0 => 0,
            PRESCALER_A::PRESCALER_1 => 1,
            PRESCALER_A::PRESCALER_4095 => 4095,
        }
    }
}
#[doc = "Reader of field `PRESCALER`"]
pub type PRESCALER_R = crate::R<u16, PRESCALER_A>;
impl PRESCALER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, PRESCALER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESCALER_A::PRESCALER_0),
            1 => Val(PRESCALER_A::PRESCALER_1),
            4095 => Val(PRESCALER_A::PRESCALER_4095),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALER_0`"]
    #[inline(always)]
    pub fn is_prescaler_0(&self) -> bool {
        *self == PRESCALER_A::PRESCALER_0
    }
    #[doc = "Checks if the value of the field is `PRESCALER_1`"]
    #[inline(always)]
    pub fn is_prescaler_1(&self) -> bool {
        *self == PRESCALER_A::PRESCALER_1
    }
    #[doc = "Checks if the value of the field is `PRESCALER_4095`"]
    #[inline(always)]
    pub fn is_prescaler_4095(&self) -> bool {
        *self == PRESCALER_A::PRESCALER_4095
    }
}
#[doc = "Write proxy for field `PRESCALER`"]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn prescaler_0(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn prescaler_1(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_1)
    }
    #[doc = "Divide by 4096"]
    #[inline(always)]
    pub fn prescaler_4095(self) -> &'a mut W {
        self.variant(PRESCALER_A::PRESCALER_4095)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Prescaler bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER24M_A {
    #[doc = "0: Divide by 1"]
    PRESCALER24M_0,
    #[doc = "1: Divide by 2"]
    PRESCALER24M_1,
    #[doc = "15: Divide by 16"]
    PRESCALER24M_15,
}
impl From<PRESCALER24M_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER24M_A) -> Self {
        match variant {
            PRESCALER24M_A::PRESCALER24M_0 => 0,
            PRESCALER24M_A::PRESCALER24M_1 => 1,
            PRESCALER24M_A::PRESCALER24M_15 => 15,
        }
    }
}
#[doc = "Reader of field `PRESCALER24M`"]
pub type PRESCALER24M_R = crate::R<u8, PRESCALER24M_A>;
impl PRESCALER24M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESCALER24M_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESCALER24M_A::PRESCALER24M_0),
            1 => Val(PRESCALER24M_A::PRESCALER24M_1),
            15 => Val(PRESCALER24M_A::PRESCALER24M_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALER24M_0`"]
    #[inline(always)]
    pub fn is_prescaler24m_0(&self) -> bool {
        *self == PRESCALER24M_A::PRESCALER24M_0
    }
    #[doc = "Checks if the value of the field is `PRESCALER24M_1`"]
    #[inline(always)]
    pub fn is_prescaler24m_1(&self) -> bool {
        *self == PRESCALER24M_A::PRESCALER24M_1
    }
    #[doc = "Checks if the value of the field is `PRESCALER24M_15`"]
    #[inline(always)]
    pub fn is_prescaler24m_15(&self) -> bool {
        *self == PRESCALER24M_A::PRESCALER24M_15
    }
}
#[doc = "Write proxy for field `PRESCALER24M`"]
pub struct PRESCALER24M_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER24M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALER24M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn prescaler24m_0(self) -> &'a mut W {
        self.variant(PRESCALER24M_A::PRESCALER24M_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn prescaler24m_1(self) -> &'a mut W {
        self.variant(PRESCALER24M_A::PRESCALER24M_1)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn prescaler24m_15(self) -> &'a mut W {
        self.variant(PRESCALER24M_A::PRESCALER24M_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Prescaler bits"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Prescaler bits"]
    #[inline(always)]
    pub fn prescaler24m(&self) -> PRESCALER24M_R {
        PRESCALER24M_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Prescaler bits"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    #[doc = "Bits 12:15 - Prescaler bits"]
    #[inline(always)]
    pub fn prescaler24m(&mut self) -> PRESCALER24M_W {
        PRESCALER24M_W { w: self }
    }
}
