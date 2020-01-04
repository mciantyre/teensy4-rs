#[doc = "Reader of register FLSHCR4"]
pub type R = crate::R<u32, super::FLSHCR4>;
#[doc = "Writer for register FLSHCR4"]
pub type W = crate::W<u32, super::FLSHCR4>;
#[doc = "Register FLSHCR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLSHCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMOPT1_A {
    #[doc = "0: DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    WMOPT1_0 = 0,
    #[doc = "1: DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    WMOPT1_1 = 1,
}
impl From<WMOPT1_A> for bool {
    #[inline(always)]
    fn from(variant: WMOPT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WMOPT1`"]
pub type WMOPT1_R = crate::R<bool, WMOPT1_A>;
impl WMOPT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMOPT1_A {
        match self.bits {
            false => WMOPT1_A::WMOPT1_0,
            true => WMOPT1_A::WMOPT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `WMOPT1_0`"]
    #[inline(always)]
    pub fn is_wmopt1_0(&self) -> bool {
        *self == WMOPT1_A::WMOPT1_0
    }
    #[doc = "Checks if the value of the field is `WMOPT1_1`"]
    #[inline(always)]
    pub fn is_wmopt1_1(&self) -> bool {
        *self == WMOPT1_A::WMOPT1_1
    }
}
#[doc = "Write proxy for field `WMOPT1`"]
pub struct WMOPT1_W<'a> {
    w: &'a mut W,
}
impl<'a> WMOPT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WMOPT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DQS pin will be used as Write Mask when writing to external device. There is no limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    #[inline(always)]
    pub fn wmopt1_0(self) -> &'a mut W {
        self.variant(WMOPT1_A::WMOPT1_0)
    }
    #[doc = "DQS pin will not be used as Write Mask when writing to external device. There is limitation on AHB write burst start address alignment when flash is accessed in individual mode."]
    #[inline(always)]
    pub fn wmopt1_1(self) -> &'a mut W {
        self.variant(WMOPT1_A::WMOPT1_1)
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
#[doc = "Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMENA_A {
    #[doc = "0: Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    WMENA_0 = 0,
    #[doc = "1: Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    WMENA_1 = 1,
}
impl From<WMENA_A> for bool {
    #[inline(always)]
    fn from(variant: WMENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WMENA`"]
pub type WMENA_R = crate::R<bool, WMENA_A>;
impl WMENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMENA_A {
        match self.bits {
            false => WMENA_A::WMENA_0,
            true => WMENA_A::WMENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `WMENA_0`"]
    #[inline(always)]
    pub fn is_wmena_0(&self) -> bool {
        *self == WMENA_A::WMENA_0
    }
    #[doc = "Checks if the value of the field is `WMENA_1`"]
    #[inline(always)]
    pub fn is_wmena_1(&self) -> bool {
        *self == WMENA_A::WMENA_1
    }
}
#[doc = "Write proxy for field `WMENA`"]
pub struct WMENA_W<'a> {
    w: &'a mut W,
}
impl<'a> WMENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WMENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    #[inline(always)]
    pub fn wmena_0(self) -> &'a mut W {
        self.variant(WMENA_A::WMENA_0)
    }
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    #[inline(always)]
    pub fn wmena_1(self) -> &'a mut W {
        self.variant(WMENA_A::WMENA_1)
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
#[doc = "Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMENB_A {
    #[doc = "0: Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    WMENB_0 = 0,
    #[doc = "1: Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    WMENB_1 = 1,
}
impl From<WMENB_A> for bool {
    #[inline(always)]
    fn from(variant: WMENB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WMENB`"]
pub type WMENB_R = crate::R<bool, WMENB_A>;
impl WMENB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WMENB_A {
        match self.bits {
            false => WMENB_A::WMENB_0,
            true => WMENB_A::WMENB_1,
        }
    }
    #[doc = "Checks if the value of the field is `WMENB_0`"]
    #[inline(always)]
    pub fn is_wmenb_0(&self) -> bool {
        *self == WMENB_A::WMENB_0
    }
    #[doc = "Checks if the value of the field is `WMENB_1`"]
    #[inline(always)]
    pub fn is_wmenb_1(&self) -> bool {
        *self == WMENB_A::WMENB_1
    }
}
#[doc = "Write proxy for field `WMENB`"]
pub struct WMENB_W<'a> {
    w: &'a mut W,
}
impl<'a> WMENB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WMENB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write mask is disabled, DQS(RWDS) pin will be un-driven when writing to external device."]
    #[inline(always)]
    pub fn wmenb_0(self) -> &'a mut W {
        self.variant(WMENB_A::WMENB_0)
    }
    #[doc = "Write mask is enabled, DQS(RWDS) pin will be driven by FlexSPI as write mask output when writing to external device."]
    #[inline(always)]
    pub fn wmenb_1(self) -> &'a mut W {
        self.variant(WMENB_A::WMENB_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
    #[inline(always)]
    pub fn wmopt1(&self) -> WMOPT1_R {
        WMOPT1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
    #[inline(always)]
    pub fn wmena(&self) -> WMENA_R {
        WMENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
    #[inline(always)]
    pub fn wmenb(&self) -> WMENB_R {
        WMENB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
    #[inline(always)]
    pub fn wmopt1(&mut self) -> WMOPT1_W {
        WMOPT1_W { w: self }
    }
    #[doc = "Bit 2 - Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
    #[inline(always)]
    pub fn wmena(&mut self) -> WMENA_W {
        WMENA_W { w: self }
    }
    #[doc = "Bit 3 - Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
    #[inline(always)]
    pub fn wmenb(&mut self) -> WMENB_W {
        WMENB_W { w: self }
    }
}
