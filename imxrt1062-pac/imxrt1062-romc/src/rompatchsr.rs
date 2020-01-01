#[doc = "Reader of register ROMPATCHSR"]
pub type R = crate::R<u32, super::ROMPATCHSR>;
#[doc = "Writer for register ROMPATCHSR"]
pub type W = crate::W<u32, super::ROMPATCHSR>;
#[doc = "Register ROMPATCHSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ROMPATCHSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ROMC Source Number - Binary encoding of the number of the address comparator which has an address match in the most recent patch event on ROMC AHB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOURCE_A {
    #[doc = "0: Address Comparator 0 matched"]
    SOURCE_0 = 0,
    #[doc = "1: Address Comparator 1 matched"]
    SOURCE_1 = 1,
    #[doc = "15: Address Comparator 15 matched"]
    SOURCE_15 = 15,
}
impl From<SOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SOURCE`"]
pub type SOURCE_R = crate::R<u8, SOURCE_A>;
impl SOURCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SOURCE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SOURCE_A::SOURCE_0),
            1 => Val(SOURCE_A::SOURCE_1),
            15 => Val(SOURCE_A::SOURCE_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SOURCE_0`"]
    #[inline(always)]
    pub fn is_source_0(&self) -> bool {
        *self == SOURCE_A::SOURCE_0
    }
    #[doc = "Checks if the value of the field is `SOURCE_1`"]
    #[inline(always)]
    pub fn is_source_1(&self) -> bool {
        *self == SOURCE_A::SOURCE_1
    }
    #[doc = "Checks if the value of the field is `SOURCE_15`"]
    #[inline(always)]
    pub fn is_source_15(&self) -> bool {
        *self == SOURCE_A::SOURCE_15
    }
}
#[doc = "ROMC AHB Multiple Address Comparator matches Indicator - Indicates that multiple address comparator matches occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_A {
    #[doc = "0: no event or comparator collisions"]
    SW_0 = 0,
    #[doc = "1: a collision has occurred"]
    SW_1 = 1,
}
impl From<SW_A> for bool {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SW`"]
pub type SW_R = crate::R<bool, SW_A>;
impl SW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            false => SW_A::SW_0,
            true => SW_A::SW_1,
        }
    }
    #[doc = "Checks if the value of the field is `SW_0`"]
    #[inline(always)]
    pub fn is_sw_0(&self) -> bool {
        *self == SW_A::SW_0
    }
    #[doc = "Checks if the value of the field is `SW_1`"]
    #[inline(always)]
    pub fn is_sw_1(&self) -> bool {
        *self == SW_A::SW_1
    }
}
#[doc = "Write proxy for field `SW`"]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no event or comparator collisions"]
    #[inline(always)]
    pub fn sw_0(self) -> &'a mut W {
        self.variant(SW_A::SW_0)
    }
    #[doc = "a collision has occurred"]
    #[inline(always)]
    pub fn sw_1(self) -> &'a mut W {
        self.variant(SW_A::SW_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - ROMC Source Number - Binary encoding of the number of the address comparator which has an address match in the most recent patch event on ROMC AHB"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 17 - ROMC AHB Multiple Address Comparator matches Indicator - Indicates that multiple address comparator matches occurred"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - ROMC AHB Multiple Address Comparator matches Indicator - Indicates that multiple address comparator matches occurred"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
}
