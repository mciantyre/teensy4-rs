#[doc = "Reader of register CM7_AHBPCR"]
pub type R = crate::R<u32, super::CM7_AHBPCR>;
#[doc = "Writer for register CM7_AHBPCR"]
pub type W = crate::W<u32, super::CM7_AHBPCR>;
#[doc = "Register CM7_AHBPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CM7_AHBPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AHBP enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: AHBP disabled. When disabled all accesses are made to the AXIM interface."]
    EN_0 = 0,
    #[doc = "1: AHBP enabled."]
    EN_1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, EN_A>;
impl EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::EN_0,
            true => EN_A::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline(always)]
    pub fn is_en_0(&self) -> bool {
        *self == EN_A::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline(always)]
    pub fn is_en_1(&self) -> bool {
        *self == EN_A::EN_1
    }
}
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AHBP disabled. When disabled all accesses are made to the AXIM interface."]
    #[inline(always)]
    pub fn en_0(self) -> &'a mut W {
        self.variant(EN_A::EN_0)
    }
    #[doc = "AHBP enabled."]
    #[inline(always)]
    pub fn en_1(self) -> &'a mut W {
        self.variant(EN_A::EN_1)
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
#[doc = "AHBP size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SZ_A {
    #[doc = "0: 0MB. AHBP disabled."]
    SZ_0 = 0,
    #[doc = "1: 64MB."]
    SZ_1 = 1,
    #[doc = "2: 128MB."]
    SZ_2 = 2,
    #[doc = "3: 256MB."]
    SZ_3 = 3,
    #[doc = "4: 512MB."]
    SZ_4 = 4,
}
impl From<SZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SZ`"]
pub type SZ_R = crate::R<u8, SZ_A>;
impl SZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SZ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SZ_A::SZ_0),
            1 => Val(SZ_A::SZ_1),
            2 => Val(SZ_A::SZ_2),
            3 => Val(SZ_A::SZ_3),
            4 => Val(SZ_A::SZ_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SZ_0`"]
    #[inline(always)]
    pub fn is_sz_0(&self) -> bool {
        *self == SZ_A::SZ_0
    }
    #[doc = "Checks if the value of the field is `SZ_1`"]
    #[inline(always)]
    pub fn is_sz_1(&self) -> bool {
        *self == SZ_A::SZ_1
    }
    #[doc = "Checks if the value of the field is `SZ_2`"]
    #[inline(always)]
    pub fn is_sz_2(&self) -> bool {
        *self == SZ_A::SZ_2
    }
    #[doc = "Checks if the value of the field is `SZ_3`"]
    #[inline(always)]
    pub fn is_sz_3(&self) -> bool {
        *self == SZ_A::SZ_3
    }
    #[doc = "Checks if the value of the field is `SZ_4`"]
    #[inline(always)]
    pub fn is_sz_4(&self) -> bool {
        *self == SZ_A::SZ_4
    }
}
impl R {
    #[doc = "Bit 0 - AHBP enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - AHBP size."]
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AHBP enable."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
