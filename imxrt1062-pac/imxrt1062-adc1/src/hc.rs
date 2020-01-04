#[doc = "Reader of register HC%s"]
pub type R = crate::R<u32, super::HC>;
#[doc = "Writer for register HC%s"]
pub type W = crate::W<u32, super::HC>;
#[doc = "Register HC%s `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::HC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Input Channel Select\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCH_A {
    #[doc = "16: External channel selection from ADC_ETC"]
    ADCH_16 = 16,
    #[doc = "25: VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
    ADCH_25 = 25,
    #[doc = "31: Conversion Disabled. Hardware Triggers will not initiate any conversion."]
    ADCH_31 = 31,
}
impl From<ADCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCH`"]
pub type ADCH_R = crate::R<u8, ADCH_A>;
impl ADCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADCH_A> {
        use crate::Variant::*;
        match self.bits {
            16 => Val(ADCH_A::ADCH_16),
            25 => Val(ADCH_A::ADCH_25),
            31 => Val(ADCH_A::ADCH_31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADCH_16`"]
    #[inline(always)]
    pub fn is_adch_16(&self) -> bool {
        *self == ADCH_A::ADCH_16
    }
    #[doc = "Checks if the value of the field is `ADCH_25`"]
    #[inline(always)]
    pub fn is_adch_25(&self) -> bool {
        *self == ADCH_A::ADCH_25
    }
    #[doc = "Checks if the value of the field is `ADCH_31`"]
    #[inline(always)]
    pub fn is_adch_31(&self) -> bool {
        *self == ADCH_A::ADCH_31
    }
}
#[doc = "Write proxy for field `ADCH`"]
pub struct ADCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "External channel selection from ADC_ETC"]
    #[inline(always)]
    pub fn adch_16(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_16)
    }
    #[doc = "VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
    #[inline(always)]
    pub fn adch_25(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_25)
    }
    #[doc = "Conversion Disabled. Hardware Triggers will not initiate any conversion."]
    #[inline(always)]
    pub fn adch_31(self) -> &'a mut W {
        self.variant(ADCH_A::ADCH_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Conversion Complete Interrupt Enable/Disable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIEN_A {
    #[doc = "0: Conversion complete interrupt disabled"]
    AIEN_0 = 0,
    #[doc = "1: Conversion complete interrupt enabled"]
    AIEN_1 = 1,
}
impl From<AIEN_A> for bool {
    #[inline(always)]
    fn from(variant: AIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AIEN`"]
pub type AIEN_R = crate::R<bool, AIEN_A>;
impl AIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIEN_A {
        match self.bits {
            false => AIEN_A::AIEN_0,
            true => AIEN_A::AIEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AIEN_0`"]
    #[inline(always)]
    pub fn is_aien_0(&self) -> bool {
        *self == AIEN_A::AIEN_0
    }
    #[doc = "Checks if the value of the field is `AIEN_1`"]
    #[inline(always)]
    pub fn is_aien_1(&self) -> bool {
        *self == AIEN_A::AIEN_1
    }
}
#[doc = "Write proxy for field `AIEN`"]
pub struct AIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Conversion complete interrupt disabled"]
    #[inline(always)]
    pub fn aien_0(self) -> &'a mut W {
        self.variant(AIEN_A::AIEN_0)
    }
    #[doc = "Conversion complete interrupt enabled"]
    #[inline(always)]
    pub fn aien_1(self) -> &'a mut W {
        self.variant(AIEN_A::AIEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Input Channel Select"]
    #[inline(always)]
    pub fn adch(&self) -> ADCH_R {
        ADCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Conversion Complete Interrupt Enable/Disable Control"]
    #[inline(always)]
    pub fn aien(&self) -> AIEN_R {
        AIEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input Channel Select"]
    #[inline(always)]
    pub fn adch(&mut self) -> ADCH_W {
        ADCH_W { w: self }
    }
    #[doc = "Bit 7 - Conversion Complete Interrupt Enable/Disable Control"]
    #[inline(always)]
    pub fn aien(&mut self) -> AIEN_W {
        AIEN_W { w: self }
    }
}
