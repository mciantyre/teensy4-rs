#[doc = "Reader of register GPTIMER0CTRL"]
pub type R = crate::R<u32, super::GPTIMER0CTRL>;
#[doc = "Writer for register GPTIMER0CTRL"]
pub type W = crate::W<u32, super::GPTIMER0CTRL>;
#[doc = "Register GPTIMER0CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTIMER0CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPTCNT`"]
pub type GPTCNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPTCNT`"]
pub struct GPTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPTMODE_A {
    #[doc = "0: One Shot Mode"]
    GPTMODE_0 = 0,
    #[doc = "1: Repeat Mode"]
    GPTMODE_1 = 1,
}
impl From<GPTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPTMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPTMODE`"]
pub type GPTMODE_R = crate::R<bool, GPTMODE_A>;
impl GPTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPTMODE_A {
        match self.bits {
            false => GPTMODE_A::GPTMODE_0,
            true => GPTMODE_A::GPTMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPTMODE_0`"]
    #[inline(always)]
    pub fn is_gptmode_0(&self) -> bool {
        *self == GPTMODE_A::GPTMODE_0
    }
    #[doc = "Checks if the value of the field is `GPTMODE_1`"]
    #[inline(always)]
    pub fn is_gptmode_1(&self) -> bool {
        *self == GPTMODE_A::GPTMODE_1
    }
}
#[doc = "Write proxy for field `GPTMODE`"]
pub struct GPTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPTMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One Shot Mode"]
    #[inline(always)]
    pub fn gptmode_0(self) -> &'a mut W {
        self.variant(GPTMODE_A::GPTMODE_0)
    }
    #[doc = "Repeat Mode"]
    #[inline(always)]
    pub fn gptmode_1(self) -> &'a mut W {
        self.variant(GPTMODE_A::GPTMODE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "General Purpose Timer Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPTRST_A {
    #[doc = "0: No action"]
    GPTRST_0 = 0,
    #[doc = "1: Load counter value from GPTLD bits in n_GPTIMER0LD"]
    GPTRST_1 = 1,
}
impl From<GPTRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPTRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPTRST`"]
pub type GPTRST_R = crate::R<bool, GPTRST_A>;
impl GPTRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPTRST_A {
        match self.bits {
            false => GPTRST_A::GPTRST_0,
            true => GPTRST_A::GPTRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPTRST_0`"]
    #[inline(always)]
    pub fn is_gptrst_0(&self) -> bool {
        *self == GPTRST_A::GPTRST_0
    }
    #[doc = "Checks if the value of the field is `GPTRST_1`"]
    #[inline(always)]
    pub fn is_gptrst_1(&self) -> bool {
        *self == GPTRST_A::GPTRST_1
    }
}
#[doc = "Write proxy for field `GPTRST`"]
pub struct GPTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPTRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn gptrst_0(self) -> &'a mut W {
        self.variant(GPTRST_A::GPTRST_0)
    }
    #[doc = "Load counter value from GPTLD bits in n_GPTIMER0LD"]
    #[inline(always)]
    pub fn gptrst_1(self) -> &'a mut W {
        self.variant(GPTRST_A::GPTRST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPTRUN_A {
    #[doc = "0: Stop counting"]
    GPTRUN_0 = 0,
    #[doc = "1: Run"]
    GPTRUN_1 = 1,
}
impl From<GPTRUN_A> for bool {
    #[inline(always)]
    fn from(variant: GPTRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPTRUN`"]
pub type GPTRUN_R = crate::R<bool, GPTRUN_A>;
impl GPTRUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPTRUN_A {
        match self.bits {
            false => GPTRUN_A::GPTRUN_0,
            true => GPTRUN_A::GPTRUN_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPTRUN_0`"]
    #[inline(always)]
    pub fn is_gptrun_0(&self) -> bool {
        *self == GPTRUN_A::GPTRUN_0
    }
    #[doc = "Checks if the value of the field is `GPTRUN_1`"]
    #[inline(always)]
    pub fn is_gptrun_1(&self) -> bool {
        *self == GPTRUN_A::GPTRUN_1
    }
}
#[doc = "Write proxy for field `GPTRUN`"]
pub struct GPTRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTRUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPTRUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop counting"]
    #[inline(always)]
    pub fn gptrun_0(self) -> &'a mut W {
        self.variant(GPTRUN_A::GPTRUN_0)
    }
    #[doc = "Run"]
    #[inline(always)]
    pub fn gptrun_1(self) -> &'a mut W {
        self.variant(GPTRUN_A::GPTRUN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - General Purpose Timer Counter. This field is the count value of the countdown timer."]
    #[inline(always)]
    pub fn gptcnt(&self) -> GPTCNT_R {
        GPTCNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again"]
    #[inline(always)]
    pub fn gptmode(&self) -> GPTMODE_R {
        GPTMODE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 30 - General Purpose Timer Reset"]
    #[inline(always)]
    pub fn gptrst(&self) -> GPTRST_R {
        GPTRST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
    #[inline(always)]
    pub fn gptrun(&self) -> GPTRUN_R {
        GPTRUN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - General Purpose Timer Counter. This field is the count value of the countdown timer."]
    #[inline(always)]
    pub fn gptcnt(&mut self) -> GPTCNT_W {
        GPTCNT_W { w: self }
    }
    #[doc = "Bit 24 - General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again"]
    #[inline(always)]
    pub fn gptmode(&mut self) -> GPTMODE_W {
        GPTMODE_W { w: self }
    }
    #[doc = "Bit 30 - General Purpose Timer Reset"]
    #[inline(always)]
    pub fn gptrst(&mut self) -> GPTRST_W {
        GPTRST_W { w: self }
    }
    #[doc = "Bit 31 - General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
    #[inline(always)]
    pub fn gptrun(&mut self) -> GPTRUN_W {
        GPTRUN_W { w: self }
    }
}
