#[doc = "Reader of register OPACR4"]
pub type R = crate::R<u32, super::OPACR4>;
#[doc = "Writer for register OPACR4"]
pub type W = crate::W<u32, super::OPACR4>;
#[doc = "Register OPACR4 `reset()`'s with value 0x4444_4444"]
impl crate::ResetValue for super::OPACR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4444_4444
    }
}
#[doc = "Off-platform Peripheral Access Control 33\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC33_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC33_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC33_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC33`"]
pub type OPAC33_R = crate::R<u8, OPAC33_A>;
impl OPAC33_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC33_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC33_A::TP0),
            1 => Val(OPAC33_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC33_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC33_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC33`"]
pub struct OPAC33_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC33_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC33_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC33_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC33_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 32\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC32_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC32_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC32_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC32`"]
pub type OPAC32_R = crate::R<u8, OPAC32_A>;
impl OPAC32_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC32_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC32_A::TP0),
            1 => Val(OPAC32_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC32_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC32_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC32`"]
pub struct OPAC32_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC32_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC32_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC32_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 33"]
    #[inline(always)]
    pub fn opac33(&self) -> OPAC33_R {
        OPAC33_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 32"]
    #[inline(always)]
    pub fn opac32(&self) -> OPAC32_R {
        OPAC32_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 33"]
    #[inline(always)]
    pub fn opac33(&mut self) -> OPAC33_W {
        OPAC33_W { w: self }
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 32"]
    #[inline(always)]
    pub fn opac32(&mut self) -> OPAC32_W {
        OPAC32_W { w: self }
    }
}
