#[doc = "Reader of register FSTS0"]
pub type R = crate::R<u16, super::FSTS0>;
#[doc = "Writer for register FSTS0"]
pub type W = crate::W<u16, super::FSTS0>;
#[doc = "Register FSTS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FSTS0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fault Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FFLAG_A {
    #[doc = "0: No fault on the FAULTx pin."]
    FFLAG_0 = 0,
    #[doc = "1: Fault on the FAULTx pin."]
    FFLAG_1 = 1,
}
impl From<FFLAG_A> for u8 {
    #[inline(always)]
    fn from(variant: FFLAG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FFLAG`"]
pub type FFLAG_R = crate::R<u8, FFLAG_A>;
impl FFLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FFLAG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FFLAG_A::FFLAG_0),
            1 => Val(FFLAG_A::FFLAG_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FFLAG_0`"]
    #[inline(always)]
    pub fn is_fflag_0(&self) -> bool {
        *self == FFLAG_A::FFLAG_0
    }
    #[doc = "Checks if the value of the field is `FFLAG_1`"]
    #[inline(always)]
    pub fn is_fflag_1(&self) -> bool {
        *self == FFLAG_A::FFLAG_1
    }
}
#[doc = "Write proxy for field `FFLAG`"]
pub struct FFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFLAG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No fault on the FAULTx pin."]
    #[inline(always)]
    pub fn fflag_0(self) -> &'a mut W {
        self.variant(FFLAG_A::FFLAG_0)
    }
    #[doc = "Fault on the FAULTx pin."]
    #[inline(always)]
    pub fn fflag_1(self) -> &'a mut W {
        self.variant(FFLAG_A::FFLAG_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Full Cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FFULL_A {
    #[doc = "0: PWM outputs are not re-enabled at the start of a full cycle"]
    FFULL_0 = 0,
    #[doc = "1: PWM outputs are re-enabled at the start of a full cycle"]
    FFULL_1 = 1,
}
impl From<FFULL_A> for u8 {
    #[inline(always)]
    fn from(variant: FFULL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FFULL`"]
pub type FFULL_R = crate::R<u8, FFULL_A>;
impl FFULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FFULL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FFULL_A::FFULL_0),
            1 => Val(FFULL_A::FFULL_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FFULL_0`"]
    #[inline(always)]
    pub fn is_ffull_0(&self) -> bool {
        *self == FFULL_A::FFULL_0
    }
    #[doc = "Checks if the value of the field is `FFULL_1`"]
    #[inline(always)]
    pub fn is_ffull_1(&self) -> bool {
        *self == FFULL_A::FFULL_1
    }
}
#[doc = "Write proxy for field `FFULL`"]
pub struct FFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FFULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM outputs are not re-enabled at the start of a full cycle"]
    #[inline(always)]
    pub fn ffull_0(self) -> &'a mut W {
        self.variant(FFULL_A::FFULL_0)
    }
    #[doc = "PWM outputs are re-enabled at the start of a full cycle"]
    #[inline(always)]
    pub fn ffull_1(self) -> &'a mut W {
        self.variant(FFULL_A::FFULL_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `FFPIN`"]
pub type FFPIN_R = crate::R<u8, u8>;
#[doc = "Half Cycle Fault Recovery\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FHALF_A {
    #[doc = "0: PWM outputs are not re-enabled at the start of a half cycle."]
    FHALF_0 = 0,
    #[doc = "1: PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0)."]
    FHALF_1 = 1,
}
impl From<FHALF_A> for u8 {
    #[inline(always)]
    fn from(variant: FHALF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FHALF`"]
pub type FHALF_R = crate::R<u8, FHALF_A>;
impl FHALF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FHALF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FHALF_A::FHALF_0),
            1 => Val(FHALF_A::FHALF_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FHALF_0`"]
    #[inline(always)]
    pub fn is_fhalf_0(&self) -> bool {
        *self == FHALF_A::FHALF_0
    }
    #[doc = "Checks if the value of the field is `FHALF_1`"]
    #[inline(always)]
    pub fn is_fhalf_1(&self) -> bool {
        *self == FHALF_A::FHALF_1
    }
}
#[doc = "Write proxy for field `FHALF`"]
pub struct FHALF_W<'a> {
    w: &'a mut W,
}
impl<'a> FHALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FHALF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM outputs are not re-enabled at the start of a half cycle."]
    #[inline(always)]
    pub fn fhalf_0(self) -> &'a mut W {
        self.variant(FHALF_A::FHALF_0)
    }
    #[doc = "PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0)."]
    #[inline(always)]
    pub fn fhalf_1(self) -> &'a mut W {
        self.variant(FHALF_A::FHALF_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Fault Flags"]
    #[inline(always)]
    pub fn fflag(&self) -> FFLAG_R {
        FFLAG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Full Cycle"]
    #[inline(always)]
    pub fn ffull(&self) -> FFULL_R {
        FFULL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Filtered Fault Pins"]
    #[inline(always)]
    pub fn ffpin(&self) -> FFPIN_R {
        FFPIN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Half Cycle Fault Recovery"]
    #[inline(always)]
    pub fn fhalf(&self) -> FHALF_R {
        FHALF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Fault Flags"]
    #[inline(always)]
    pub fn fflag(&mut self) -> FFLAG_W {
        FFLAG_W { w: self }
    }
    #[doc = "Bits 4:7 - Full Cycle"]
    #[inline(always)]
    pub fn ffull(&mut self) -> FFULL_W {
        FFULL_W { w: self }
    }
    #[doc = "Bits 12:15 - Half Cycle Fault Recovery"]
    #[inline(always)]
    pub fn fhalf(&mut self) -> FHALF_W {
        FHALF_W { w: self }
    }
}
