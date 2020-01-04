#[doc = "Reader of register SCR"]
pub type R = crate::R<u32, super::SCR>;
#[doc = "Writer for register SCR"]
pub type W = crate::W<u32, super::SCR>;
#[doc = "Register SCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPONEXIT_A {
    #[doc = "0: o not sleep when returning to Thread mode"]
    SLEEPONEXIT_0 = 0,
    #[doc = "1: enter sleep, or deep sleep, on return from an ISR"]
    SLEEPONEXIT_1 = 1,
}
impl From<SLEEPONEXIT_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPONEXIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEEPONEXIT`"]
pub type SLEEPONEXIT_R = crate::R<bool, SLEEPONEXIT_A>;
impl SLEEPONEXIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPONEXIT_A {
        match self.bits {
            false => SLEEPONEXIT_A::SLEEPONEXIT_0,
            true => SLEEPONEXIT_A::SLEEPONEXIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLEEPONEXIT_0`"]
    #[inline(always)]
    pub fn is_sleeponexit_0(&self) -> bool {
        *self == SLEEPONEXIT_A::SLEEPONEXIT_0
    }
    #[doc = "Checks if the value of the field is `SLEEPONEXIT_1`"]
    #[inline(always)]
    pub fn is_sleeponexit_1(&self) -> bool {
        *self == SLEEPONEXIT_A::SLEEPONEXIT_1
    }
}
#[doc = "Write proxy for field `SLEEPONEXIT`"]
pub struct SLEEPONEXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPONEXIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPONEXIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "o not sleep when returning to Thread mode"]
    #[inline(always)]
    pub fn sleeponexit_0(self) -> &'a mut W {
        self.variant(SLEEPONEXIT_A::SLEEPONEXIT_0)
    }
    #[doc = "enter sleep, or deep sleep, on return from an ISR"]
    #[inline(always)]
    pub fn sleeponexit_1(self) -> &'a mut W {
        self.variant(SLEEPONEXIT_A::SLEEPONEXIT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Controls whether the processor uses sleep or deep sleep as its low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPDEEP_A {
    #[doc = "0: sleep"]
    SLEEPDEEP_0 = 0,
    #[doc = "1: deep sleep"]
    SLEEPDEEP_1 = 1,
}
impl From<SLEEPDEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPDEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEEPDEEP`"]
pub type SLEEPDEEP_R = crate::R<bool, SLEEPDEEP_A>;
impl SLEEPDEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPDEEP_A {
        match self.bits {
            false => SLEEPDEEP_A::SLEEPDEEP_0,
            true => SLEEPDEEP_A::SLEEPDEEP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLEEPDEEP_0`"]
    #[inline(always)]
    pub fn is_sleepdeep_0(&self) -> bool {
        *self == SLEEPDEEP_A::SLEEPDEEP_0
    }
    #[doc = "Checks if the value of the field is `SLEEPDEEP_1`"]
    #[inline(always)]
    pub fn is_sleepdeep_1(&self) -> bool {
        *self == SLEEPDEEP_A::SLEEPDEEP_1
    }
}
#[doc = "Write proxy for field `SLEEPDEEP`"]
pub struct SLEEPDEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPDEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPDEEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "sleep"]
    #[inline(always)]
    pub fn sleepdeep_0(self) -> &'a mut W {
        self.variant(SLEEPDEEP_A::SLEEPDEEP_0)
    }
    #[doc = "deep sleep"]
    #[inline(always)]
    pub fn sleepdeep_1(self) -> &'a mut W {
        self.variant(SLEEPDEEP_A::SLEEPDEEP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Send Event on Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEVONPEND_A {
    #[doc = "0: only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    SEVONPEND_0 = 0,
    #[doc = "1: enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    SEVONPEND_1 = 1,
}
impl From<SEVONPEND_A> for bool {
    #[inline(always)]
    fn from(variant: SEVONPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEVONPEND`"]
pub type SEVONPEND_R = crate::R<bool, SEVONPEND_A>;
impl SEVONPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEVONPEND_A {
        match self.bits {
            false => SEVONPEND_A::SEVONPEND_0,
            true => SEVONPEND_A::SEVONPEND_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEVONPEND_0`"]
    #[inline(always)]
    pub fn is_sevonpend_0(&self) -> bool {
        *self == SEVONPEND_A::SEVONPEND_0
    }
    #[doc = "Checks if the value of the field is `SEVONPEND_1`"]
    #[inline(always)]
    pub fn is_sevonpend_1(&self) -> bool {
        *self == SEVONPEND_A::SEVONPEND_1
    }
}
#[doc = "Write proxy for field `SEVONPEND`"]
pub struct SEVONPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SEVONPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEVONPEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn sevonpend_0(self) -> &'a mut W {
        self.variant(SEVONPEND_A::SEVONPEND_0)
    }
    #[doc = "enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    #[inline(always)]
    pub fn sevonpend_1(self) -> &'a mut W {
        self.variant(SEVONPEND_A::SEVONPEND_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Indicates sleep-on-exit when returning from Handler mode to Thread mode"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls whether the processor uses sleep or deep sleep as its low power mode"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Indicates sleep-on-exit when returning from Handler mode to Thread mode"]
    #[inline(always)]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W {
        SLEEPONEXIT_W { w: self }
    }
    #[doc = "Bit 2 - Controls whether the processor uses sleep or deep sleep as its low power mode"]
    #[inline(always)]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W {
        SLEEPDEEP_W { w: self }
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline(always)]
    pub fn sevonpend(&mut self) -> SEVONPEND_W {
        SEVONPEND_W { w: self }
    }
}
