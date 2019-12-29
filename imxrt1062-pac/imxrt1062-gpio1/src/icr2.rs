#[doc = "Reader of register ICR2"]
pub type R = crate::R<u32, super::ICR2>;
#[doc = "Writer for register ICR2"]
pub type W = crate::W<u32, super::ICR2>;
#[doc = "Register ICR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ICR16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR16_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR16_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR16_A) -> Self {
        match variant {
            ICR16_A::LOW_LEVEL => 0,
            ICR16_A::HIGH_LEVEL => 1,
            ICR16_A::RISING_EDGE => 2,
            ICR16_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR16`"]
pub type ICR16_R = crate::R<u8, ICR16_A>;
impl ICR16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR16_A {
        match self.bits {
            0 => ICR16_A::LOW_LEVEL,
            1 => ICR16_A::HIGH_LEVEL,
            2 => ICR16_A::RISING_EDGE,
            3 => ICR16_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR16_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR16_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR16_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR16_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR16`"]
pub struct ICR16_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR16_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR16_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR16_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR16_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR16_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "ICR17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR17_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR17_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR17_A) -> Self {
        match variant {
            ICR17_A::LOW_LEVEL => 0,
            ICR17_A::HIGH_LEVEL => 1,
            ICR17_A::RISING_EDGE => 2,
            ICR17_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR17`"]
pub type ICR17_R = crate::R<u8, ICR17_A>;
impl ICR17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR17_A {
        match self.bits {
            0 => ICR17_A::LOW_LEVEL,
            1 => ICR17_A::HIGH_LEVEL,
            2 => ICR17_A::RISING_EDGE,
            3 => ICR17_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR17_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR17_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR17_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR17_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR17`"]
pub struct ICR17_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR17_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR17_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR17_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR17_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR17_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "ICR18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR18_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR18_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR18_A) -> Self {
        match variant {
            ICR18_A::LOW_LEVEL => 0,
            ICR18_A::HIGH_LEVEL => 1,
            ICR18_A::RISING_EDGE => 2,
            ICR18_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR18`"]
pub type ICR18_R = crate::R<u8, ICR18_A>;
impl ICR18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR18_A {
        match self.bits {
            0 => ICR18_A::LOW_LEVEL,
            1 => ICR18_A::HIGH_LEVEL,
            2 => ICR18_A::RISING_EDGE,
            3 => ICR18_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR18_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR18_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR18_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR18_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR18`"]
pub struct ICR18_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR18_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR18_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR18_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR18_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR18_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "ICR19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR19_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR19_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR19_A) -> Self {
        match variant {
            ICR19_A::LOW_LEVEL => 0,
            ICR19_A::HIGH_LEVEL => 1,
            ICR19_A::RISING_EDGE => 2,
            ICR19_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR19`"]
pub type ICR19_R = crate::R<u8, ICR19_A>;
impl ICR19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR19_A {
        match self.bits {
            0 => ICR19_A::LOW_LEVEL,
            1 => ICR19_A::HIGH_LEVEL,
            2 => ICR19_A::RISING_EDGE,
            3 => ICR19_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR19_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR19_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR19_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR19_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR19`"]
pub struct ICR19_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR19_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR19_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR19_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR19_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR19_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "ICR20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR20_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR20_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR20_A) -> Self {
        match variant {
            ICR20_A::LOW_LEVEL => 0,
            ICR20_A::HIGH_LEVEL => 1,
            ICR20_A::RISING_EDGE => 2,
            ICR20_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR20`"]
pub type ICR20_R = crate::R<u8, ICR20_A>;
impl ICR20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR20_A {
        match self.bits {
            0 => ICR20_A::LOW_LEVEL,
            1 => ICR20_A::HIGH_LEVEL,
            2 => ICR20_A::RISING_EDGE,
            3 => ICR20_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR20_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR20_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR20_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR20_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR20`"]
pub struct ICR20_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR20_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR20_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR20_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR20_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR20_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "ICR21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR21_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR21_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR21_A) -> Self {
        match variant {
            ICR21_A::LOW_LEVEL => 0,
            ICR21_A::HIGH_LEVEL => 1,
            ICR21_A::RISING_EDGE => 2,
            ICR21_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR21`"]
pub type ICR21_R = crate::R<u8, ICR21_A>;
impl ICR21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR21_A {
        match self.bits {
            0 => ICR21_A::LOW_LEVEL,
            1 => ICR21_A::HIGH_LEVEL,
            2 => ICR21_A::RISING_EDGE,
            3 => ICR21_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR21_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR21_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR21_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR21_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR21`"]
pub struct ICR21_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR21_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR21_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR21_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR21_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR21_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "ICR22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR22_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR22_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR22_A) -> Self {
        match variant {
            ICR22_A::LOW_LEVEL => 0,
            ICR22_A::HIGH_LEVEL => 1,
            ICR22_A::RISING_EDGE => 2,
            ICR22_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR22`"]
pub type ICR22_R = crate::R<u8, ICR22_A>;
impl ICR22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR22_A {
        match self.bits {
            0 => ICR22_A::LOW_LEVEL,
            1 => ICR22_A::HIGH_LEVEL,
            2 => ICR22_A::RISING_EDGE,
            3 => ICR22_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR22_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR22_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR22_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR22_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR22`"]
pub struct ICR22_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR22_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR22_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR22_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR22_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR22_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "ICR23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR23_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR23_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR23_A) -> Self {
        match variant {
            ICR23_A::LOW_LEVEL => 0,
            ICR23_A::HIGH_LEVEL => 1,
            ICR23_A::RISING_EDGE => 2,
            ICR23_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR23`"]
pub type ICR23_R = crate::R<u8, ICR23_A>;
impl ICR23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR23_A {
        match self.bits {
            0 => ICR23_A::LOW_LEVEL,
            1 => ICR23_A::HIGH_LEVEL,
            2 => ICR23_A::RISING_EDGE,
            3 => ICR23_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR23_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR23_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR23_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR23_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR23`"]
pub struct ICR23_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR23_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR23_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR23_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR23_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR23_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "ICR24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR24_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR24_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR24_A) -> Self {
        match variant {
            ICR24_A::LOW_LEVEL => 0,
            ICR24_A::HIGH_LEVEL => 1,
            ICR24_A::RISING_EDGE => 2,
            ICR24_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR24`"]
pub type ICR24_R = crate::R<u8, ICR24_A>;
impl ICR24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR24_A {
        match self.bits {
            0 => ICR24_A::LOW_LEVEL,
            1 => ICR24_A::HIGH_LEVEL,
            2 => ICR24_A::RISING_EDGE,
            3 => ICR24_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR24_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR24_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR24_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR24_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR24`"]
pub struct ICR24_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR24_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR24_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR24_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR24_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR24_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "ICR25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR25_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR25_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR25_A) -> Self {
        match variant {
            ICR25_A::LOW_LEVEL => 0,
            ICR25_A::HIGH_LEVEL => 1,
            ICR25_A::RISING_EDGE => 2,
            ICR25_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR25`"]
pub type ICR25_R = crate::R<u8, ICR25_A>;
impl ICR25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR25_A {
        match self.bits {
            0 => ICR25_A::LOW_LEVEL,
            1 => ICR25_A::HIGH_LEVEL,
            2 => ICR25_A::RISING_EDGE,
            3 => ICR25_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR25_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR25_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR25_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR25_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR25`"]
pub struct ICR25_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR25_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR25_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR25_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR25_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR25_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "ICR26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR26_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR26_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR26_A) -> Self {
        match variant {
            ICR26_A::LOW_LEVEL => 0,
            ICR26_A::HIGH_LEVEL => 1,
            ICR26_A::RISING_EDGE => 2,
            ICR26_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR26`"]
pub type ICR26_R = crate::R<u8, ICR26_A>;
impl ICR26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR26_A {
        match self.bits {
            0 => ICR26_A::LOW_LEVEL,
            1 => ICR26_A::HIGH_LEVEL,
            2 => ICR26_A::RISING_EDGE,
            3 => ICR26_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR26_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR26_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR26_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR26_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR26`"]
pub struct ICR26_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR26_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR26_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR26_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR26_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR26_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "ICR27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR27_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR27_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR27_A) -> Self {
        match variant {
            ICR27_A::LOW_LEVEL => 0,
            ICR27_A::HIGH_LEVEL => 1,
            ICR27_A::RISING_EDGE => 2,
            ICR27_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR27`"]
pub type ICR27_R = crate::R<u8, ICR27_A>;
impl ICR27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR27_A {
        match self.bits {
            0 => ICR27_A::LOW_LEVEL,
            1 => ICR27_A::HIGH_LEVEL,
            2 => ICR27_A::RISING_EDGE,
            3 => ICR27_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR27_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR27_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR27_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR27_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR27`"]
pub struct ICR27_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR27_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR27_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR27_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR27_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR27_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "ICR28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR28_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR28_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR28_A) -> Self {
        match variant {
            ICR28_A::LOW_LEVEL => 0,
            ICR28_A::HIGH_LEVEL => 1,
            ICR28_A::RISING_EDGE => 2,
            ICR28_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR28`"]
pub type ICR28_R = crate::R<u8, ICR28_A>;
impl ICR28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR28_A {
        match self.bits {
            0 => ICR28_A::LOW_LEVEL,
            1 => ICR28_A::HIGH_LEVEL,
            2 => ICR28_A::RISING_EDGE,
            3 => ICR28_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR28_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR28_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR28_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR28_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR28`"]
pub struct ICR28_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR28_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR28_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR28_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR28_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR28_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "ICR29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR29_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR29_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR29_A) -> Self {
        match variant {
            ICR29_A::LOW_LEVEL => 0,
            ICR29_A::HIGH_LEVEL => 1,
            ICR29_A::RISING_EDGE => 2,
            ICR29_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR29`"]
pub type ICR29_R = crate::R<u8, ICR29_A>;
impl ICR29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR29_A {
        match self.bits {
            0 => ICR29_A::LOW_LEVEL,
            1 => ICR29_A::HIGH_LEVEL,
            2 => ICR29_A::RISING_EDGE,
            3 => ICR29_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR29_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR29_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR29_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR29_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR29`"]
pub struct ICR29_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR29_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR29_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR29_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR29_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR29_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "ICR30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR30_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR30_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR30_A) -> Self {
        match variant {
            ICR30_A::LOW_LEVEL => 0,
            ICR30_A::HIGH_LEVEL => 1,
            ICR30_A::RISING_EDGE => 2,
            ICR30_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR30`"]
pub type ICR30_R = crate::R<u8, ICR30_A>;
impl ICR30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR30_A {
        match self.bits {
            0 => ICR30_A::LOW_LEVEL,
            1 => ICR30_A::HIGH_LEVEL,
            2 => ICR30_A::RISING_EDGE,
            3 => ICR30_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR30_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR30_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR30_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR30_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR30`"]
pub struct ICR30_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR30_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR30_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR30_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR30_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR30_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "ICR31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR31_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR31_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR31_A) -> Self {
        match variant {
            ICR31_A::LOW_LEVEL => 0,
            ICR31_A::HIGH_LEVEL => 1,
            ICR31_A::RISING_EDGE => 2,
            ICR31_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR31`"]
pub type ICR31_R = crate::R<u8, ICR31_A>;
impl ICR31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR31_A {
        match self.bits {
            0 => ICR31_A::LOW_LEVEL,
            1 => ICR31_A::HIGH_LEVEL,
            2 => ICR31_A::RISING_EDGE,
            3 => ICR31_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR31_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR31_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR31_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR31_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR31`"]
pub struct ICR31_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR31_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR31_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR31_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR31_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR31_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ICR16"]
    #[inline(always)]
    pub fn icr16(&self) -> ICR16_R {
        ICR16_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - ICR17"]
    #[inline(always)]
    pub fn icr17(&self) -> ICR17_R {
        ICR17_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - ICR18"]
    #[inline(always)]
    pub fn icr18(&self) -> ICR18_R {
        ICR18_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - ICR19"]
    #[inline(always)]
    pub fn icr19(&self) -> ICR19_R {
        ICR19_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - ICR20"]
    #[inline(always)]
    pub fn icr20(&self) -> ICR20_R {
        ICR20_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - ICR21"]
    #[inline(always)]
    pub fn icr21(&self) -> ICR21_R {
        ICR21_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - ICR22"]
    #[inline(always)]
    pub fn icr22(&self) -> ICR22_R {
        ICR22_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - ICR23"]
    #[inline(always)]
    pub fn icr23(&self) -> ICR23_R {
        ICR23_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - ICR24"]
    #[inline(always)]
    pub fn icr24(&self) -> ICR24_R {
        ICR24_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - ICR25"]
    #[inline(always)]
    pub fn icr25(&self) -> ICR25_R {
        ICR25_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - ICR26"]
    #[inline(always)]
    pub fn icr26(&self) -> ICR26_R {
        ICR26_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - ICR27"]
    #[inline(always)]
    pub fn icr27(&self) -> ICR27_R {
        ICR27_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - ICR28"]
    #[inline(always)]
    pub fn icr28(&self) -> ICR28_R {
        ICR28_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - ICR29"]
    #[inline(always)]
    pub fn icr29(&self) -> ICR29_R {
        ICR29_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - ICR30"]
    #[inline(always)]
    pub fn icr30(&self) -> ICR30_R {
        ICR30_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - ICR31"]
    #[inline(always)]
    pub fn icr31(&self) -> ICR31_R {
        ICR31_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ICR16"]
    #[inline(always)]
    pub fn icr16(&mut self) -> ICR16_W {
        ICR16_W { w: self }
    }
    #[doc = "Bits 2:3 - ICR17"]
    #[inline(always)]
    pub fn icr17(&mut self) -> ICR17_W {
        ICR17_W { w: self }
    }
    #[doc = "Bits 4:5 - ICR18"]
    #[inline(always)]
    pub fn icr18(&mut self) -> ICR18_W {
        ICR18_W { w: self }
    }
    #[doc = "Bits 6:7 - ICR19"]
    #[inline(always)]
    pub fn icr19(&mut self) -> ICR19_W {
        ICR19_W { w: self }
    }
    #[doc = "Bits 8:9 - ICR20"]
    #[inline(always)]
    pub fn icr20(&mut self) -> ICR20_W {
        ICR20_W { w: self }
    }
    #[doc = "Bits 10:11 - ICR21"]
    #[inline(always)]
    pub fn icr21(&mut self) -> ICR21_W {
        ICR21_W { w: self }
    }
    #[doc = "Bits 12:13 - ICR22"]
    #[inline(always)]
    pub fn icr22(&mut self) -> ICR22_W {
        ICR22_W { w: self }
    }
    #[doc = "Bits 14:15 - ICR23"]
    #[inline(always)]
    pub fn icr23(&mut self) -> ICR23_W {
        ICR23_W { w: self }
    }
    #[doc = "Bits 16:17 - ICR24"]
    #[inline(always)]
    pub fn icr24(&mut self) -> ICR24_W {
        ICR24_W { w: self }
    }
    #[doc = "Bits 18:19 - ICR25"]
    #[inline(always)]
    pub fn icr25(&mut self) -> ICR25_W {
        ICR25_W { w: self }
    }
    #[doc = "Bits 20:21 - ICR26"]
    #[inline(always)]
    pub fn icr26(&mut self) -> ICR26_W {
        ICR26_W { w: self }
    }
    #[doc = "Bits 22:23 - ICR27"]
    #[inline(always)]
    pub fn icr27(&mut self) -> ICR27_W {
        ICR27_W { w: self }
    }
    #[doc = "Bits 24:25 - ICR28"]
    #[inline(always)]
    pub fn icr28(&mut self) -> ICR28_W {
        ICR28_W { w: self }
    }
    #[doc = "Bits 26:27 - ICR29"]
    #[inline(always)]
    pub fn icr29(&mut self) -> ICR29_W {
        ICR29_W { w: self }
    }
    #[doc = "Bits 28:29 - ICR30"]
    #[inline(always)]
    pub fn icr30(&mut self) -> ICR30_W {
        ICR30_W { w: self }
    }
    #[doc = "Bits 30:31 - ICR31"]
    #[inline(always)]
    pub fn icr31(&mut self) -> ICR31_W {
        ICR31_W { w: self }
    }
}
