#[doc = "Reader of register ICR1"]
pub type R = crate::R<u32, super::ICR1>;
#[doc = "Writer for register ICR1"]
pub type W = crate::W<u32, super::ICR1>;
#[doc = "Register ICR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ICR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR0_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR0_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR0_A) -> Self {
        match variant {
            ICR0_A::LOW_LEVEL => 0,
            ICR0_A::HIGH_LEVEL => 1,
            ICR0_A::RISING_EDGE => 2,
            ICR0_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR0`"]
pub type ICR0_R = crate::R<u8, ICR0_A>;
impl ICR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR0_A {
        match self.bits {
            0 => ICR0_A::LOW_LEVEL,
            1 => ICR0_A::HIGH_LEVEL,
            2 => ICR0_A::RISING_EDGE,
            3 => ICR0_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR0_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR0_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR0_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR0_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR0`"]
pub struct ICR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR0_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR0_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR0_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR0_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "ICR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR1_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR1_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR1_A) -> Self {
        match variant {
            ICR1_A::LOW_LEVEL => 0,
            ICR1_A::HIGH_LEVEL => 1,
            ICR1_A::RISING_EDGE => 2,
            ICR1_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR1`"]
pub type ICR1_R = crate::R<u8, ICR1_A>;
impl ICR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR1_A {
        match self.bits {
            0 => ICR1_A::LOW_LEVEL,
            1 => ICR1_A::HIGH_LEVEL,
            2 => ICR1_A::RISING_EDGE,
            3 => ICR1_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR1_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR1_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR1_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR1_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR1`"]
pub struct ICR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR1_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR1_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR1_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR1_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "ICR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR2_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR2_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR2_A) -> Self {
        match variant {
            ICR2_A::LOW_LEVEL => 0,
            ICR2_A::HIGH_LEVEL => 1,
            ICR2_A::RISING_EDGE => 2,
            ICR2_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR2`"]
pub type ICR2_R = crate::R<u8, ICR2_A>;
impl ICR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR2_A {
        match self.bits {
            0 => ICR2_A::LOW_LEVEL,
            1 => ICR2_A::HIGH_LEVEL,
            2 => ICR2_A::RISING_EDGE,
            3 => ICR2_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR2_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR2_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR2_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR2_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR2`"]
pub struct ICR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR2_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR2_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR2_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR2_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "ICR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR3_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR3_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR3_A) -> Self {
        match variant {
            ICR3_A::LOW_LEVEL => 0,
            ICR3_A::HIGH_LEVEL => 1,
            ICR3_A::RISING_EDGE => 2,
            ICR3_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR3`"]
pub type ICR3_R = crate::R<u8, ICR3_A>;
impl ICR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR3_A {
        match self.bits {
            0 => ICR3_A::LOW_LEVEL,
            1 => ICR3_A::HIGH_LEVEL,
            2 => ICR3_A::RISING_EDGE,
            3 => ICR3_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR3_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR3_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR3_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR3_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR3`"]
pub struct ICR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR3_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR3_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR3_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR3_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "ICR4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR4_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR4_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR4_A) -> Self {
        match variant {
            ICR4_A::LOW_LEVEL => 0,
            ICR4_A::HIGH_LEVEL => 1,
            ICR4_A::RISING_EDGE => 2,
            ICR4_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR4`"]
pub type ICR4_R = crate::R<u8, ICR4_A>;
impl ICR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR4_A {
        match self.bits {
            0 => ICR4_A::LOW_LEVEL,
            1 => ICR4_A::HIGH_LEVEL,
            2 => ICR4_A::RISING_EDGE,
            3 => ICR4_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR4_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR4_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR4_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR4_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR4`"]
pub struct ICR4_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR4_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR4_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR4_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR4_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "ICR5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR5_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR5_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR5_A) -> Self {
        match variant {
            ICR5_A::LOW_LEVEL => 0,
            ICR5_A::HIGH_LEVEL => 1,
            ICR5_A::RISING_EDGE => 2,
            ICR5_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR5`"]
pub type ICR5_R = crate::R<u8, ICR5_A>;
impl ICR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR5_A {
        match self.bits {
            0 => ICR5_A::LOW_LEVEL,
            1 => ICR5_A::HIGH_LEVEL,
            2 => ICR5_A::RISING_EDGE,
            3 => ICR5_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR5_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR5_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR5_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR5_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR5`"]
pub struct ICR5_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR5_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR5_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR5_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR5_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "ICR6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR6_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR6_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR6_A) -> Self {
        match variant {
            ICR6_A::LOW_LEVEL => 0,
            ICR6_A::HIGH_LEVEL => 1,
            ICR6_A::RISING_EDGE => 2,
            ICR6_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR6`"]
pub type ICR6_R = crate::R<u8, ICR6_A>;
impl ICR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR6_A {
        match self.bits {
            0 => ICR6_A::LOW_LEVEL,
            1 => ICR6_A::HIGH_LEVEL,
            2 => ICR6_A::RISING_EDGE,
            3 => ICR6_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR6_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR6_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR6_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR6_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR6`"]
pub struct ICR6_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR6_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR6_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR6_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR6_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "ICR7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR7_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR7_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR7_A) -> Self {
        match variant {
            ICR7_A::LOW_LEVEL => 0,
            ICR7_A::HIGH_LEVEL => 1,
            ICR7_A::RISING_EDGE => 2,
            ICR7_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR7`"]
pub type ICR7_R = crate::R<u8, ICR7_A>;
impl ICR7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR7_A {
        match self.bits {
            0 => ICR7_A::LOW_LEVEL,
            1 => ICR7_A::HIGH_LEVEL,
            2 => ICR7_A::RISING_EDGE,
            3 => ICR7_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR7_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR7_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR7_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR7_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR7`"]
pub struct ICR7_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR7_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR7_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR7_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR7_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "ICR8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR8_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR8_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR8_A) -> Self {
        match variant {
            ICR8_A::LOW_LEVEL => 0,
            ICR8_A::HIGH_LEVEL => 1,
            ICR8_A::RISING_EDGE => 2,
            ICR8_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR8`"]
pub type ICR8_R = crate::R<u8, ICR8_A>;
impl ICR8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR8_A {
        match self.bits {
            0 => ICR8_A::LOW_LEVEL,
            1 => ICR8_A::HIGH_LEVEL,
            2 => ICR8_A::RISING_EDGE,
            3 => ICR8_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR8_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR8_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR8_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR8_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR8`"]
pub struct ICR8_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR8_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR8_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR8_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR8_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR8_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "ICR9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR9_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR9_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR9_A) -> Self {
        match variant {
            ICR9_A::LOW_LEVEL => 0,
            ICR9_A::HIGH_LEVEL => 1,
            ICR9_A::RISING_EDGE => 2,
            ICR9_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR9`"]
pub type ICR9_R = crate::R<u8, ICR9_A>;
impl ICR9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR9_A {
        match self.bits {
            0 => ICR9_A::LOW_LEVEL,
            1 => ICR9_A::HIGH_LEVEL,
            2 => ICR9_A::RISING_EDGE,
            3 => ICR9_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR9_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR9_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR9_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR9_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR9`"]
pub struct ICR9_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR9_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR9_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR9_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR9_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR9_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "ICR10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR10_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR10_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR10_A) -> Self {
        match variant {
            ICR10_A::LOW_LEVEL => 0,
            ICR10_A::HIGH_LEVEL => 1,
            ICR10_A::RISING_EDGE => 2,
            ICR10_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR10`"]
pub type ICR10_R = crate::R<u8, ICR10_A>;
impl ICR10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR10_A {
        match self.bits {
            0 => ICR10_A::LOW_LEVEL,
            1 => ICR10_A::HIGH_LEVEL,
            2 => ICR10_A::RISING_EDGE,
            3 => ICR10_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR10_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR10_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR10_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR10_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR10`"]
pub struct ICR10_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR10_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR10_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR10_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR10_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "ICR11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR11_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR11_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR11_A) -> Self {
        match variant {
            ICR11_A::LOW_LEVEL => 0,
            ICR11_A::HIGH_LEVEL => 1,
            ICR11_A::RISING_EDGE => 2,
            ICR11_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR11`"]
pub type ICR11_R = crate::R<u8, ICR11_A>;
impl ICR11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR11_A {
        match self.bits {
            0 => ICR11_A::LOW_LEVEL,
            1 => ICR11_A::HIGH_LEVEL,
            2 => ICR11_A::RISING_EDGE,
            3 => ICR11_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR11_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR11_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR11_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR11_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR11`"]
pub struct ICR11_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR11_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR11_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR11_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR11_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "ICR12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR12_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR12_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR12_A) -> Self {
        match variant {
            ICR12_A::LOW_LEVEL => 0,
            ICR12_A::HIGH_LEVEL => 1,
            ICR12_A::RISING_EDGE => 2,
            ICR12_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR12`"]
pub type ICR12_R = crate::R<u8, ICR12_A>;
impl ICR12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR12_A {
        match self.bits {
            0 => ICR12_A::LOW_LEVEL,
            1 => ICR12_A::HIGH_LEVEL,
            2 => ICR12_A::RISING_EDGE,
            3 => ICR12_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR12_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR12_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR12_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR12_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR12`"]
pub struct ICR12_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR12_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR12_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR12_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR12_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR12_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "ICR13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR13_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR13_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR13_A) -> Self {
        match variant {
            ICR13_A::LOW_LEVEL => 0,
            ICR13_A::HIGH_LEVEL => 1,
            ICR13_A::RISING_EDGE => 2,
            ICR13_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR13`"]
pub type ICR13_R = crate::R<u8, ICR13_A>;
impl ICR13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR13_A {
        match self.bits {
            0 => ICR13_A::LOW_LEVEL,
            1 => ICR13_A::HIGH_LEVEL,
            2 => ICR13_A::RISING_EDGE,
            3 => ICR13_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR13_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR13_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR13_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR13_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR13`"]
pub struct ICR13_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR13_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR13_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR13_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR13_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR13_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "ICR14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR14_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR14_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR14_A) -> Self {
        match variant {
            ICR14_A::LOW_LEVEL => 0,
            ICR14_A::HIGH_LEVEL => 1,
            ICR14_A::RISING_EDGE => 2,
            ICR14_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR14`"]
pub type ICR14_R = crate::R<u8, ICR14_A>;
impl ICR14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR14_A {
        match self.bits {
            0 => ICR14_A::LOW_LEVEL,
            1 => ICR14_A::HIGH_LEVEL,
            2 => ICR14_A::RISING_EDGE,
            3 => ICR14_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR14_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR14_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR14_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR14_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR14`"]
pub struct ICR14_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR14_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR14_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR14_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR14_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR14_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "ICR15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR15_A {
    #[doc = "0: Interrupt n is low-level sensitive."]
    LOW_LEVEL,
    #[doc = "1: Interrupt n is high-level sensitive."]
    HIGH_LEVEL,
    #[doc = "2: Interrupt n is rising-edge sensitive."]
    RISING_EDGE,
    #[doc = "3: Interrupt n is falling-edge sensitive."]
    FALLING_EDGE,
}
impl From<ICR15_A> for u8 {
    #[inline(always)]
    fn from(variant: ICR15_A) -> Self {
        match variant {
            ICR15_A::LOW_LEVEL => 0,
            ICR15_A::HIGH_LEVEL => 1,
            ICR15_A::RISING_EDGE => 2,
            ICR15_A::FALLING_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `ICR15`"]
pub type ICR15_R = crate::R<u8, ICR15_A>;
impl ICR15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICR15_A {
        match self.bits {
            0 => ICR15_A::LOW_LEVEL,
            1 => ICR15_A::HIGH_LEVEL,
            2 => ICR15_A::RISING_EDGE,
            3 => ICR15_A::FALLING_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == ICR15_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == ICR15_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ICR15_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ICR15_A::FALLING_EDGE
    }
}
#[doc = "Write proxy for field `ICR15`"]
pub struct ICR15_W<'a> {
    w: &'a mut W,
}
impl<'a> ICR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICR15_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt n is low-level sensitive."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(ICR15_A::LOW_LEVEL)
    }
    #[doc = "Interrupt n is high-level sensitive."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(ICR15_A::HIGH_LEVEL)
    }
    #[doc = "Interrupt n is rising-edge sensitive."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ICR15_A::RISING_EDGE)
    }
    #[doc = "Interrupt n is falling-edge sensitive."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ICR15_A::FALLING_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ICR0"]
    #[inline(always)]
    pub fn icr0(&self) -> ICR0_R {
        ICR0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - ICR1"]
    #[inline(always)]
    pub fn icr1(&self) -> ICR1_R {
        ICR1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - ICR2"]
    #[inline(always)]
    pub fn icr2(&self) -> ICR2_R {
        ICR2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - ICR3"]
    #[inline(always)]
    pub fn icr3(&self) -> ICR3_R {
        ICR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - ICR4"]
    #[inline(always)]
    pub fn icr4(&self) -> ICR4_R {
        ICR4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - ICR5"]
    #[inline(always)]
    pub fn icr5(&self) -> ICR5_R {
        ICR5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - ICR6"]
    #[inline(always)]
    pub fn icr6(&self) -> ICR6_R {
        ICR6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - ICR7"]
    #[inline(always)]
    pub fn icr7(&self) -> ICR7_R {
        ICR7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - ICR8"]
    #[inline(always)]
    pub fn icr8(&self) -> ICR8_R {
        ICR8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - ICR9"]
    #[inline(always)]
    pub fn icr9(&self) -> ICR9_R {
        ICR9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - ICR10"]
    #[inline(always)]
    pub fn icr10(&self) -> ICR10_R {
        ICR10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - ICR11"]
    #[inline(always)]
    pub fn icr11(&self) -> ICR11_R {
        ICR11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - ICR12"]
    #[inline(always)]
    pub fn icr12(&self) -> ICR12_R {
        ICR12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - ICR13"]
    #[inline(always)]
    pub fn icr13(&self) -> ICR13_R {
        ICR13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - ICR14"]
    #[inline(always)]
    pub fn icr14(&self) -> ICR14_R {
        ICR14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - ICR15"]
    #[inline(always)]
    pub fn icr15(&self) -> ICR15_R {
        ICR15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ICR0"]
    #[inline(always)]
    pub fn icr0(&mut self) -> ICR0_W {
        ICR0_W { w: self }
    }
    #[doc = "Bits 2:3 - ICR1"]
    #[inline(always)]
    pub fn icr1(&mut self) -> ICR1_W {
        ICR1_W { w: self }
    }
    #[doc = "Bits 4:5 - ICR2"]
    #[inline(always)]
    pub fn icr2(&mut self) -> ICR2_W {
        ICR2_W { w: self }
    }
    #[doc = "Bits 6:7 - ICR3"]
    #[inline(always)]
    pub fn icr3(&mut self) -> ICR3_W {
        ICR3_W { w: self }
    }
    #[doc = "Bits 8:9 - ICR4"]
    #[inline(always)]
    pub fn icr4(&mut self) -> ICR4_W {
        ICR4_W { w: self }
    }
    #[doc = "Bits 10:11 - ICR5"]
    #[inline(always)]
    pub fn icr5(&mut self) -> ICR5_W {
        ICR5_W { w: self }
    }
    #[doc = "Bits 12:13 - ICR6"]
    #[inline(always)]
    pub fn icr6(&mut self) -> ICR6_W {
        ICR6_W { w: self }
    }
    #[doc = "Bits 14:15 - ICR7"]
    #[inline(always)]
    pub fn icr7(&mut self) -> ICR7_W {
        ICR7_W { w: self }
    }
    #[doc = "Bits 16:17 - ICR8"]
    #[inline(always)]
    pub fn icr8(&mut self) -> ICR8_W {
        ICR8_W { w: self }
    }
    #[doc = "Bits 18:19 - ICR9"]
    #[inline(always)]
    pub fn icr9(&mut self) -> ICR9_W {
        ICR9_W { w: self }
    }
    #[doc = "Bits 20:21 - ICR10"]
    #[inline(always)]
    pub fn icr10(&mut self) -> ICR10_W {
        ICR10_W { w: self }
    }
    #[doc = "Bits 22:23 - ICR11"]
    #[inline(always)]
    pub fn icr11(&mut self) -> ICR11_W {
        ICR11_W { w: self }
    }
    #[doc = "Bits 24:25 - ICR12"]
    #[inline(always)]
    pub fn icr12(&mut self) -> ICR12_W {
        ICR12_W { w: self }
    }
    #[doc = "Bits 26:27 - ICR13"]
    #[inline(always)]
    pub fn icr13(&mut self) -> ICR13_W {
        ICR13_W { w: self }
    }
    #[doc = "Bits 28:29 - ICR14"]
    #[inline(always)]
    pub fn icr14(&mut self) -> ICR14_W {
        ICR14_W { w: self }
    }
    #[doc = "Bits 30:31 - ICR15"]
    #[inline(always)]
    pub fn icr15(&mut self) -> ICR15_W {
        ICR15_W { w: self }
    }
}
