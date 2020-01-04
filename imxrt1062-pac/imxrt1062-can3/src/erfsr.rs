#[doc = "Reader of register ERFSR"]
pub type R = crate::R<u32, super::ERFSR>;
#[doc = "Writer for register ERFSR"]
pub type W = crate::W<u32, super::ERFSR>;
#[doc = "Register ERFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ERFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ERFEL`"]
pub type ERFEL_R = crate::R<u8, u8>;
#[doc = "Enhanced Rx FIFO full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERFF_A {
    #[doc = "0: Enhanced Rx FIFO is not full"]
    ERFF_0 = 0,
    #[doc = "1: Enhanced Rx FIFO is full"]
    ERFF_1 = 1,
}
impl From<ERFF_A> for bool {
    #[inline(always)]
    fn from(variant: ERFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERFF`"]
pub type ERFF_R = crate::R<bool, ERFF_A>;
impl ERFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFF_A {
        match self.bits {
            false => ERFF_A::ERFF_0,
            true => ERFF_A::ERFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERFF_0`"]
    #[inline(always)]
    pub fn is_erff_0(&self) -> bool {
        *self == ERFF_A::ERFF_0
    }
    #[doc = "Checks if the value of the field is `ERFF_1`"]
    #[inline(always)]
    pub fn is_erff_1(&self) -> bool {
        *self == ERFF_A::ERFF_1
    }
}
#[doc = "Enhanced Rx FIFO empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERFE_A {
    #[doc = "0: Enhanced Rx FIFO is not empty"]
    ERFE_0 = 0,
    #[doc = "1: Enhanced Rx FIFO is empty"]
    ERFE_1 = 1,
}
impl From<ERFE_A> for bool {
    #[inline(always)]
    fn from(variant: ERFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERFE`"]
pub type ERFE_R = crate::R<bool, ERFE_A>;
impl ERFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFE_A {
        match self.bits {
            false => ERFE_A::ERFE_0,
            true => ERFE_A::ERFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERFE_0`"]
    #[inline(always)]
    pub fn is_erfe_0(&self) -> bool {
        *self == ERFE_A::ERFE_0
    }
    #[doc = "Checks if the value of the field is `ERFE_1`"]
    #[inline(always)]
    pub fn is_erfe_1(&self) -> bool {
        *self == ERFE_A::ERFE_1
    }
}
#[doc = "Enhanced Rx FIFO Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERFCLR_A {
    #[doc = "0: No effect"]
    ERFCLR_0 = 0,
    #[doc = "1: Clear Enhanced Rx FIFO content"]
    ERFCLR_1 = 1,
}
impl From<ERFCLR_A> for bool {
    #[inline(always)]
    fn from(variant: ERFCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERFCLR`"]
pub type ERFCLR_R = crate::R<bool, ERFCLR_A>;
impl ERFCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFCLR_A {
        match self.bits {
            false => ERFCLR_A::ERFCLR_0,
            true => ERFCLR_A::ERFCLR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERFCLR_0`"]
    #[inline(always)]
    pub fn is_erfclr_0(&self) -> bool {
        *self == ERFCLR_A::ERFCLR_0
    }
    #[doc = "Checks if the value of the field is `ERFCLR_1`"]
    #[inline(always)]
    pub fn is_erfclr_1(&self) -> bool {
        *self == ERFCLR_A::ERFCLR_1
    }
}
#[doc = "Write proxy for field `ERFCLR`"]
pub struct ERFCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERFCLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn erfclr_0(self) -> &'a mut W {
        self.variant(ERFCLR_A::ERFCLR_0)
    }
    #[doc = "Clear Enhanced Rx FIFO content"]
    #[inline(always)]
    pub fn erfclr_1(self) -> &'a mut W {
        self.variant(ERFCLR_A::ERFCLR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Enhanced Rx FIFO Data Available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERFDA_A {
    #[doc = "0: No such occurrence"]
    ERFDA_0 = 0,
    #[doc = "1: There is at least one message stored in Enhanced Rx FIFO"]
    ERFDA_1 = 1,
}
impl From<ERFDA_A> for bool {
    #[inline(always)]
    fn from(variant: ERFDA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERFDA`"]
pub type ERFDA_R = crate::R<bool, ERFDA_A>;
impl ERFDA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFDA_A {
        match self.bits {
            false => ERFDA_A::ERFDA_0,
            true => ERFDA_A::ERFDA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERFDA_0`"]
    #[inline(always)]
    pub fn is_erfda_0(&self) -> bool {
        *self == ERFDA_A::ERFDA_0
    }
    #[doc = "Checks if the value of the field is `ERFDA_1`"]
    #[inline(always)]
    pub fn is_erfda_1(&self) -> bool {
        *self == ERFDA_A::ERFDA_1
    }
}
#[doc = "Write proxy for field `ERFDA`"]
pub struct ERFDA_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFDA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERFDA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn erfda_0(self) -> &'a mut W {
        self.variant(ERFDA_A::ERFDA_0)
    }
    #[doc = "There is at least one message stored in Enhanced Rx FIFO"]
    #[inline(always)]
    pub fn erfda_1(self) -> &'a mut W {
        self.variant(ERFDA_A::ERFDA_1)
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
#[doc = "Enhanced Rx FIFO Watermark Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERFWMI_A {
    #[doc = "0: No such occurrence"]
    ERFWMI_0 = 0,
    #[doc = "1: The number of messages in FIFO is greater than the watermark"]
    ERFWMI_1 = 1,
}
impl From<ERFWMI_A> for bool {
    #[inline(always)]
    fn from(variant: ERFWMI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERFWMI`"]
pub type ERFWMI_R = crate::R<bool, ERFWMI_A>;
impl ERFWMI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFWMI_A {
        match self.bits {
            false => ERFWMI_A::ERFWMI_0,
            true => ERFWMI_A::ERFWMI_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERFWMI_0`"]
    #[inline(always)]
    pub fn is_erfwmi_0(&self) -> bool {
        *self == ERFWMI_A::ERFWMI_0
    }
    #[doc = "Checks if the value of the field is `ERFWMI_1`"]
    #[inline(always)]
    pub fn is_erfwmi_1(&self) -> bool {
        *self == ERFWMI_A::ERFWMI_1
    }
}
#[doc = "Write proxy for field `ERFWMI`"]
pub struct ERFWMI_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFWMI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERFWMI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn erfwmi_0(self) -> &'a mut W {
        self.variant(ERFWMI_A::ERFWMI_0)
    }
    #[doc = "The number of messages in FIFO is greater than the watermark"]
    #[inline(always)]
    pub fn erfwmi_1(self) -> &'a mut W {
        self.variant(ERFWMI_A::ERFWMI_1)
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
#[doc = "Enhanced Rx FIFO Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERFOVF_A {
    #[doc = "0: No such occurrence"]
    ERFOVF_0 = 0,
    #[doc = "1: Enhanced Rx FIFO overflow"]
    ERFOVF_1 = 1,
}
impl From<ERFOVF_A> for bool {
    #[inline(always)]
    fn from(variant: ERFOVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERFOVF`"]
pub type ERFOVF_R = crate::R<bool, ERFOVF_A>;
impl ERFOVF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFOVF_A {
        match self.bits {
            false => ERFOVF_A::ERFOVF_0,
            true => ERFOVF_A::ERFOVF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERFOVF_0`"]
    #[inline(always)]
    pub fn is_erfovf_0(&self) -> bool {
        *self == ERFOVF_A::ERFOVF_0
    }
    #[doc = "Checks if the value of the field is `ERFOVF_1`"]
    #[inline(always)]
    pub fn is_erfovf_1(&self) -> bool {
        *self == ERFOVF_A::ERFOVF_1
    }
}
#[doc = "Write proxy for field `ERFOVF`"]
pub struct ERFOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFOVF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERFOVF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn erfovf_0(self) -> &'a mut W {
        self.variant(ERFOVF_A::ERFOVF_0)
    }
    #[doc = "Enhanced Rx FIFO overflow"]
    #[inline(always)]
    pub fn erfovf_1(self) -> &'a mut W {
        self.variant(ERFOVF_A::ERFOVF_1)
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
#[doc = "Enhanced Rx FIFO Underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERFUFW_A {
    #[doc = "0: No such occurrence"]
    ERFUFW_0 = 0,
    #[doc = "1: Enhanced Rx FIFO underflow"]
    ERFUFW_1 = 1,
}
impl From<ERFUFW_A> for bool {
    #[inline(always)]
    fn from(variant: ERFUFW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERFUFW`"]
pub type ERFUFW_R = crate::R<bool, ERFUFW_A>;
impl ERFUFW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERFUFW_A {
        match self.bits {
            false => ERFUFW_A::ERFUFW_0,
            true => ERFUFW_A::ERFUFW_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERFUFW_0`"]
    #[inline(always)]
    pub fn is_erfufw_0(&self) -> bool {
        *self == ERFUFW_A::ERFUFW_0
    }
    #[doc = "Checks if the value of the field is `ERFUFW_1`"]
    #[inline(always)]
    pub fn is_erfufw_1(&self) -> bool {
        *self == ERFUFW_A::ERFUFW_1
    }
}
#[doc = "Write proxy for field `ERFUFW`"]
pub struct ERFUFW_W<'a> {
    w: &'a mut W,
}
impl<'a> ERFUFW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERFUFW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn erfufw_0(self) -> &'a mut W {
        self.variant(ERFUFW_A::ERFUFW_0)
    }
    #[doc = "Enhanced Rx FIFO underflow"]
    #[inline(always)]
    pub fn erfufw_1(self) -> &'a mut W {
        self.variant(ERFUFW_A::ERFUFW_1)
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
    #[doc = "Bits 0:5 - Enhanced Rx FIFO Elements"]
    #[inline(always)]
    pub fn erfel(&self) -> ERFEL_R {
        ERFEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Enhanced Rx FIFO full"]
    #[inline(always)]
    pub fn erff(&self) -> ERFF_R {
        ERFF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enhanced Rx FIFO empty"]
    #[inline(always)]
    pub fn erfe(&self) -> ERFE_R {
        ERFE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enhanced Rx FIFO Clear"]
    #[inline(always)]
    pub fn erfclr(&self) -> ERFCLR_R {
        ERFCLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enhanced Rx FIFO Data Available"]
    #[inline(always)]
    pub fn erfda(&self) -> ERFDA_R {
        ERFDA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enhanced Rx FIFO Watermark Indication"]
    #[inline(always)]
    pub fn erfwmi(&self) -> ERFWMI_R {
        ERFWMI_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enhanced Rx FIFO Overflow"]
    #[inline(always)]
    pub fn erfovf(&self) -> ERFOVF_R {
        ERFOVF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enhanced Rx FIFO Underflow"]
    #[inline(always)]
    pub fn erfufw(&self) -> ERFUFW_R {
        ERFUFW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Enhanced Rx FIFO Clear"]
    #[inline(always)]
    pub fn erfclr(&mut self) -> ERFCLR_W {
        ERFCLR_W { w: self }
    }
    #[doc = "Bit 28 - Enhanced Rx FIFO Data Available"]
    #[inline(always)]
    pub fn erfda(&mut self) -> ERFDA_W {
        ERFDA_W { w: self }
    }
    #[doc = "Bit 29 - Enhanced Rx FIFO Watermark Indication"]
    #[inline(always)]
    pub fn erfwmi(&mut self) -> ERFWMI_W {
        ERFWMI_W { w: self }
    }
    #[doc = "Bit 30 - Enhanced Rx FIFO Overflow"]
    #[inline(always)]
    pub fn erfovf(&mut self) -> ERFOVF_W {
        ERFOVF_W { w: self }
    }
    #[doc = "Bit 31 - Enhanced Rx FIFO Underflow"]
    #[inline(always)]
    pub fn erfufw(&mut self) -> ERFUFW_W {
        ERFUFW_W { w: self }
    }
}
