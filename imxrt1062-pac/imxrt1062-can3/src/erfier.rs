#[doc = "Reader of register ERFIER"]
pub type R = crate::R<u32, super::ERFIER>;
#[doc = "Writer for register ERFIER"]
pub type W = crate::W<u32, super::ERFIER>;
#[doc = "Register ERFIER `reset()`'s with value 0"]
impl crate::ResetValue for super::ERFIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enhanced Rx FIFO Data Available Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERFDAIE_A {
    #[doc = "0: Enhanced Rx FIFO Data Available Interrupt is disabled"]
    ERFDAIE_0 = 0,
    #[doc = "1: Enhanced Rx FIFO Data Available Interrupt is enabled"]
    ERFDAIE_1 = 1,
}
impl From<ERFDAIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERFDAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERFDAIE`"]
pub type ERFDAIE_R = crate::R<bool, ERFDAIE_A>;
impl ERFDAIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFDAIE_A {
        match self.bits {
            false => ERFDAIE_A::ERFDAIE_0,
            true => ERFDAIE_A::ERFDAIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERFDAIE_0`"]
    #[inline(always)]
    pub fn is_erfdaie_0(&self) -> bool {
        *self == ERFDAIE_A::ERFDAIE_0
    }
    #[doc = "Checks if the value of the field is `ERFDAIE_1`"]
    #[inline(always)]
    pub fn is_erfdaie_1(&self) -> bool {
        *self == ERFDAIE_A::ERFDAIE_1
    }
}
#[doc = "Write proxy for field `ERFDAIE`"]
pub struct ERFDAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFDAIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERFDAIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enhanced Rx FIFO Data Available Interrupt is disabled"]
    #[inline(always)]
    pub fn erfdaie_0(self) -> &'a mut W {
        self.variant(ERFDAIE_A::ERFDAIE_0)
    }
    #[doc = "Enhanced Rx FIFO Data Available Interrupt is enabled"]
    #[inline(always)]
    pub fn erfdaie_1(self) -> &'a mut W {
        self.variant(ERFDAIE_A::ERFDAIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Enhanced Rx FIFO Watermark Indication Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERFWMIIE_A {
    #[doc = "0: Enhanced Rx FIFO Watermark Interrupt is disabled"]
    ERFWMIIE_0 = 0,
    #[doc = "1: Enhanced Rx FIFO Watermark Interrupt is enabled"]
    ERFWMIIE_1 = 1,
}
impl From<ERFWMIIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERFWMIIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERFWMIIE`"]
pub type ERFWMIIE_R = crate::R<bool, ERFWMIIE_A>;
impl ERFWMIIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFWMIIE_A {
        match self.bits {
            false => ERFWMIIE_A::ERFWMIIE_0,
            true => ERFWMIIE_A::ERFWMIIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERFWMIIE_0`"]
    #[inline(always)]
    pub fn is_erfwmiie_0(&self) -> bool {
        *self == ERFWMIIE_A::ERFWMIIE_0
    }
    #[doc = "Checks if the value of the field is `ERFWMIIE_1`"]
    #[inline(always)]
    pub fn is_erfwmiie_1(&self) -> bool {
        *self == ERFWMIIE_A::ERFWMIIE_1
    }
}
#[doc = "Write proxy for field `ERFWMIIE`"]
pub struct ERFWMIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFWMIIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERFWMIIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enhanced Rx FIFO Watermark Interrupt is disabled"]
    #[inline(always)]
    pub fn erfwmiie_0(self) -> &'a mut W {
        self.variant(ERFWMIIE_A::ERFWMIIE_0)
    }
    #[doc = "Enhanced Rx FIFO Watermark Interrupt is enabled"]
    #[inline(always)]
    pub fn erfwmiie_1(self) -> &'a mut W {
        self.variant(ERFWMIIE_A::ERFWMIIE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Enhanced Rx FIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERFOVFIE_A {
    #[doc = "0: Enhanced Rx FIFO Overflow is disabled"]
    ERFOVFIE_0 = 0,
    #[doc = "1: Enhanced Rx FIFO Overflow is enabled"]
    ERFOVFIE_1 = 1,
}
impl From<ERFOVFIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERFOVFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERFOVFIE`"]
pub type ERFOVFIE_R = crate::R<bool, ERFOVFIE_A>;
impl ERFOVFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFOVFIE_A {
        match self.bits {
            false => ERFOVFIE_A::ERFOVFIE_0,
            true => ERFOVFIE_A::ERFOVFIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERFOVFIE_0`"]
    #[inline(always)]
    pub fn is_erfovfie_0(&self) -> bool {
        *self == ERFOVFIE_A::ERFOVFIE_0
    }
    #[doc = "Checks if the value of the field is `ERFOVFIE_1`"]
    #[inline(always)]
    pub fn is_erfovfie_1(&self) -> bool {
        *self == ERFOVFIE_A::ERFOVFIE_1
    }
}
#[doc = "Write proxy for field `ERFOVFIE`"]
pub struct ERFOVFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFOVFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERFOVFIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enhanced Rx FIFO Overflow is disabled"]
    #[inline(always)]
    pub fn erfovfie_0(self) -> &'a mut W {
        self.variant(ERFOVFIE_A::ERFOVFIE_0)
    }
    #[doc = "Enhanced Rx FIFO Overflow is enabled"]
    #[inline(always)]
    pub fn erfovfie_1(self) -> &'a mut W {
        self.variant(ERFOVFIE_A::ERFOVFIE_1)
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
#[doc = "Enhanced Rx FIFO Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERFUFWIE_A {
    #[doc = "0: Enhanced Rx FIFO Underflow interrupt is disabled"]
    ERFUFWIE_0 = 0,
    #[doc = "1: Enhanced Rx FIFO Underflow interrupt is enabled"]
    ERFUFWIE_1 = 1,
}
impl From<ERFUFWIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERFUFWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERFUFWIE`"]
pub type ERFUFWIE_R = crate::R<bool, ERFUFWIE_A>;
impl ERFUFWIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFUFWIE_A {
        match self.bits {
            false => ERFUFWIE_A::ERFUFWIE_0,
            true => ERFUFWIE_A::ERFUFWIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERFUFWIE_0`"]
    #[inline(always)]
    pub fn is_erfufwie_0(&self) -> bool {
        *self == ERFUFWIE_A::ERFUFWIE_0
    }
    #[doc = "Checks if the value of the field is `ERFUFWIE_1`"]
    #[inline(always)]
    pub fn is_erfufwie_1(&self) -> bool {
        *self == ERFUFWIE_A::ERFUFWIE_1
    }
}
#[doc = "Write proxy for field `ERFUFWIE`"]
pub struct ERFUFWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFUFWIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERFUFWIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enhanced Rx FIFO Underflow interrupt is disabled"]
    #[inline(always)]
    pub fn erfufwie_0(self) -> &'a mut W {
        self.variant(ERFUFWIE_A::ERFUFWIE_0)
    }
    #[doc = "Enhanced Rx FIFO Underflow interrupt is enabled"]
    #[inline(always)]
    pub fn erfufwie_1(self) -> &'a mut W {
        self.variant(ERFUFWIE_A::ERFUFWIE_1)
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
    #[doc = "Bit 28 - Enhanced Rx FIFO Data Available Interrupt Enable"]
    #[inline(always)]
    pub fn erfdaie(&self) -> ERFDAIE_R {
        ERFDAIE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enhanced Rx FIFO Watermark Indication Interrupt Enable"]
    #[inline(always)]
    pub fn erfwmiie(&self) -> ERFWMIIE_R {
        ERFWMIIE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enhanced Rx FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn erfovfie(&self) -> ERFOVFIE_R {
        ERFOVFIE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enhanced Rx FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn erfufwie(&self) -> ERFUFWIE_R {
        ERFUFWIE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Enhanced Rx FIFO Data Available Interrupt Enable"]
    #[inline(always)]
    pub fn erfdaie(&mut self) -> ERFDAIE_W {
        ERFDAIE_W { w: self }
    }
    #[doc = "Bit 29 - Enhanced Rx FIFO Watermark Indication Interrupt Enable"]
    #[inline(always)]
    pub fn erfwmiie(&mut self) -> ERFWMIIE_W {
        ERFWMIIE_W { w: self }
    }
    #[doc = "Bit 30 - Enhanced Rx FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn erfovfie(&mut self) -> ERFOVFIE_W {
        ERFOVFIE_W { w: self }
    }
    #[doc = "Bit 31 - Enhanced Rx FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn erfufwie(&mut self) -> ERFUFWIE_W {
        ERFUFWIE_W { w: self }
    }
}
