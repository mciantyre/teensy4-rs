#[doc = "Reader of register FCTRL0"]
pub type R = crate::R<u16, super::FCTRL0>;
#[doc = "Writer for register FCTRL0"]
pub type W = crate::W<u16, super::FCTRL0>;
#[doc = "Register FCTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCTRL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fault Interrupt Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FIE_A {
    #[doc = "0: FAULTx CPU interrupt requests disabled."]
    FIE_0 = 0,
    #[doc = "1: FAULTx CPU interrupt requests enabled."]
    FIE_1 = 1,
}
impl From<FIE_A> for u8 {
    #[inline(always)]
    fn from(variant: FIE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FIE`"]
pub type FIE_R = crate::R<u8, FIE_A>;
impl FIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FIE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FIE_A::FIE_0),
            1 => Val(FIE_A::FIE_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FIE_0`"]
    #[inline(always)]
    pub fn is_fie_0(&self) -> bool {
        *self == FIE_A::FIE_0
    }
    #[doc = "Checks if the value of the field is `FIE_1`"]
    #[inline(always)]
    pub fn is_fie_1(&self) -> bool {
        *self == FIE_A::FIE_1
    }
}
#[doc = "Write proxy for field `FIE`"]
pub struct FIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FAULTx CPU interrupt requests disabled."]
    #[inline(always)]
    pub fn fie_0(self) -> &'a mut W {
        self.variant(FIE_A::FIE_0)
    }
    #[doc = "FAULTx CPU interrupt requests enabled."]
    #[inline(always)]
    pub fn fie_1(self) -> &'a mut W {
        self.variant(FIE_A::FIE_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Fault Safety Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSAFE_A {
    #[doc = "0: Normal mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending on the state of FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFPINx\\]. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn)."]
    FSAFE_0 = 0,
    #[doc = "1: Safe mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear and FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the state of FSTS\\[FFULL\\]."]
    FSAFE_1 = 1,
}
impl From<FSAFE_A> for u8 {
    #[inline(always)]
    fn from(variant: FSAFE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FSAFE`"]
pub type FSAFE_R = crate::R<u8, FSAFE_A>;
impl FSAFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FSAFE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FSAFE_A::FSAFE_0),
            1 => Val(FSAFE_A::FSAFE_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FSAFE_0`"]
    #[inline(always)]
    pub fn is_fsafe_0(&self) -> bool {
        *self == FSAFE_A::FSAFE_0
    }
    #[doc = "Checks if the value of the field is `FSAFE_1`"]
    #[inline(always)]
    pub fn is_fsafe_1(&self) -> bool {
        *self == FSAFE_A::FSAFE_1
    }
}
#[doc = "Write proxy for field `FSAFE`"]
pub struct FSAFE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSAFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSAFE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending on the state of FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFPINx\\]. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn)."]
    #[inline(always)]
    pub fn fsafe_0(self) -> &'a mut W {
        self.variant(FSAFE_A::FSAFE_0)
    }
    #[doc = "Safe mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear and FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the state of FSTS\\[FFULL\\]."]
    #[inline(always)]
    pub fn fsafe_1(self) -> &'a mut W {
        self.variant(FSAFE_A::FSAFE_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Automatic Fault Clearing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FAUTO_A {
    #[doc = "0: Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending the state of FSTS\\[FFULL\\]. This is further controlled by FCTRL\\[FSAFE\\]."]
    FAUTO_0 = 0,
    #[doc = "1: Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the state of FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFLAGx\\]."]
    FAUTO_1 = 1,
}
impl From<FAUTO_A> for u8 {
    #[inline(always)]
    fn from(variant: FAUTO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FAUTO`"]
pub type FAUTO_R = crate::R<u8, FAUTO_A>;
impl FAUTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FAUTO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FAUTO_A::FAUTO_0),
            1 => Val(FAUTO_A::FAUTO_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FAUTO_0`"]
    #[inline(always)]
    pub fn is_fauto_0(&self) -> bool {
        *self == FAUTO_A::FAUTO_0
    }
    #[doc = "Checks if the value of the field is `FAUTO_1`"]
    #[inline(always)]
    pub fn is_fauto_1(&self) -> bool {
        *self == FAUTO_A::FAUTO_1
    }
}
#[doc = "Write proxy for field `FAUTO`"]
pub struct FAUTO_W<'a> {
    w: &'a mut W,
}
impl<'a> FAUTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAUTO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending the state of FSTS\\[FFULL\\]. This is further controlled by FCTRL\\[FSAFE\\]."]
    #[inline(always)]
    pub fn fauto_0(self) -> &'a mut W {
        self.variant(FAUTO_A::FAUTO_0)
    }
    #[doc = "Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the state of FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFLAGx\\]."]
    #[inline(always)]
    pub fn fauto_1(self) -> &'a mut W {
        self.variant(FAUTO_A::FAUTO_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Fault Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLVL_A {
    #[doc = "0: A logic 0 on the fault input indicates a fault condition."]
    FLVL_0 = 0,
    #[doc = "1: A logic 1 on the fault input indicates a fault condition."]
    FLVL_1 = 1,
}
impl From<FLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: FLVL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLVL`"]
pub type FLVL_R = crate::R<u8, FLVL_A>;
impl FLVL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLVL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLVL_A::FLVL_0),
            1 => Val(FLVL_A::FLVL_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLVL_0`"]
    #[inline(always)]
    pub fn is_flvl_0(&self) -> bool {
        *self == FLVL_A::FLVL_0
    }
    #[doc = "Checks if the value of the field is `FLVL_1`"]
    #[inline(always)]
    pub fn is_flvl_1(&self) -> bool {
        *self == FLVL_A::FLVL_1
    }
}
#[doc = "Write proxy for field `FLVL`"]
pub struct FLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLVL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A logic 0 on the fault input indicates a fault condition."]
    #[inline(always)]
    pub fn flvl_0(self) -> &'a mut W {
        self.variant(FLVL_A::FLVL_0)
    }
    #[doc = "A logic 1 on the fault input indicates a fault condition."]
    #[inline(always)]
    pub fn flvl_1(self) -> &'a mut W {
        self.variant(FLVL_A::FLVL_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Fault Interrupt Enables"]
    #[inline(always)]
    pub fn fie(&self) -> FIE_R {
        FIE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Fault Safety Mode"]
    #[inline(always)]
    pub fn fsafe(&self) -> FSAFE_R {
        FSAFE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Automatic Fault Clearing"]
    #[inline(always)]
    pub fn fauto(&self) -> FAUTO_R {
        FAUTO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Fault Level"]
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Fault Interrupt Enables"]
    #[inline(always)]
    pub fn fie(&mut self) -> FIE_W {
        FIE_W { w: self }
    }
    #[doc = "Bits 4:7 - Fault Safety Mode"]
    #[inline(always)]
    pub fn fsafe(&mut self) -> FSAFE_W {
        FSAFE_W { w: self }
    }
    #[doc = "Bits 8:11 - Automatic Fault Clearing"]
    #[inline(always)]
    pub fn fauto(&mut self) -> FAUTO_W {
        FAUTO_W { w: self }
    }
    #[doc = "Bits 12:15 - Fault Level"]
    #[inline(always)]
    pub fn flvl(&mut self) -> FLVL_W {
        FLVL_W { w: self }
    }
}
