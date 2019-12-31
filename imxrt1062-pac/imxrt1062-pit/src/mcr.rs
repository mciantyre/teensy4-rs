#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZ_A {
    #[doc = "0: Timers continue to run in Debug mode."]
    FRZ_0 = 0,
    #[doc = "1: Timers are stopped in Debug mode."]
    FRZ_1 = 1,
}
impl From<FRZ_A> for bool {
    #[inline(always)]
    fn from(variant: FRZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRZ`"]
pub type FRZ_R = crate::R<bool, FRZ_A>;
impl FRZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZ_A {
        match self.bits {
            false => FRZ_A::FRZ_0,
            true => FRZ_A::FRZ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRZ_0`"]
    #[inline(always)]
    pub fn is_frz_0(&self) -> bool {
        *self == FRZ_A::FRZ_0
    }
    #[doc = "Checks if the value of the field is `FRZ_1`"]
    #[inline(always)]
    pub fn is_frz_1(&self) -> bool {
        *self == FRZ_A::FRZ_1
    }
}
#[doc = "Write proxy for field `FRZ`"]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timers continue to run in Debug mode."]
    #[inline(always)]
    pub fn frz_0(self) -> &'a mut W {
        self.variant(FRZ_A::FRZ_0)
    }
    #[doc = "Timers are stopped in Debug mode."]
    #[inline(always)]
    pub fn frz_1(self) -> &'a mut W {
        self.variant(FRZ_A::FRZ_1)
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
#[doc = "Module Disable - (PIT section)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIS_A {
    #[doc = "0: Clock for standard PIT timers is enabled."]
    MDIS_0 = 0,
    #[doc = "1: Clock for standard PIT timers is disabled."]
    MDIS_1 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MDIS`"]
pub type MDIS_R = crate::R<bool, MDIS_A>;
impl MDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::MDIS_0,
            true => MDIS_A::MDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MDIS_0`"]
    #[inline(always)]
    pub fn is_mdis_0(&self) -> bool {
        *self == MDIS_A::MDIS_0
    }
    #[doc = "Checks if the value of the field is `MDIS_1`"]
    #[inline(always)]
    pub fn is_mdis_1(&self) -> bool {
        *self == MDIS_A::MDIS_1
    }
}
#[doc = "Write proxy for field `MDIS`"]
pub struct MDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock for standard PIT timers is enabled."]
    #[inline(always)]
    pub fn mdis_0(self) -> &'a mut W {
        self.variant(MDIS_A::MDIS_0)
    }
    #[doc = "Clock for standard PIT timers is disabled."]
    #[inline(always)]
    pub fn mdis_1(self) -> &'a mut W {
        self.variant(MDIS_A::MDIS_1)
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
impl R {
    #[doc = "Bit 0 - Freeze"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Module Disable - (PIT section)"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Freeze"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 1 - Module Disable - (PIT section)"]
    #[inline(always)]
    pub fn mdis(&mut self) -> MDIS_W {
        MDIS_W { w: self }
    }
}
