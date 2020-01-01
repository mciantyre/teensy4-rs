#[doc = "Reader of register OPACR2"]
pub type R = crate::R<u32, super::OPACR2>;
#[doc = "Writer for register OPACR2"]
pub type W = crate::W<u32, super::OPACR2>;
#[doc = "Register OPACR2 `reset()`'s with value 0x4444_4444"]
impl crate::ResetValue for super::OPACR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4444_4444
    }
}
#[doc = "Off-platform Peripheral Access Control 23\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC23_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC23_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC23_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC23`"]
pub type OPAC23_R = crate::R<u8, OPAC23_A>;
impl OPAC23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC23_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC23_A::TP0),
            1 => Val(OPAC23_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC23_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC23_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC23`"]
pub struct OPAC23_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC23_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC23_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC23_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 22\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC22_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC22_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC22_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC22`"]
pub type OPAC22_R = crate::R<u8, OPAC22_A>;
impl OPAC22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC22_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC22_A::TP0),
            1 => Val(OPAC22_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC22_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC22_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC22`"]
pub struct OPAC22_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC22_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC22_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC22_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 21\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC21_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC21_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC21_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC21`"]
pub type OPAC21_R = crate::R<u8, OPAC21_A>;
impl OPAC21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC21_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC21_A::TP0),
            1 => Val(OPAC21_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC21_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC21_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC21`"]
pub struct OPAC21_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC21_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC21_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC21_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 20\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC20_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC20_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC20_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC20`"]
pub type OPAC20_R = crate::R<u8, OPAC20_A>;
impl OPAC20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC20_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC20_A::TP0),
            1 => Val(OPAC20_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC20_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC20_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC20`"]
pub struct OPAC20_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC20_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC20_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC20_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 19\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC19_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC19_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC19_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC19`"]
pub type OPAC19_R = crate::R<u8, OPAC19_A>;
impl OPAC19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC19_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC19_A::TP0),
            1 => Val(OPAC19_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC19_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC19_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC19`"]
pub struct OPAC19_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC19_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC19_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC19_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 18\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC18_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC18_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC18_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC18`"]
pub type OPAC18_R = crate::R<u8, OPAC18_A>;
impl OPAC18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC18_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC18_A::TP0),
            1 => Val(OPAC18_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC18_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC18_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC18`"]
pub struct OPAC18_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC18_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC18_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC18_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 17\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC17_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC17_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC17_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC17`"]
pub type OPAC17_R = crate::R<u8, OPAC17_A>;
impl OPAC17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC17_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC17_A::TP0),
            1 => Val(OPAC17_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC17_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC17_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC17`"]
pub struct OPAC17_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC17_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC17_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC17_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 16\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC16_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC16_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC16_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC16`"]
pub type OPAC16_R = crate::R<u8, OPAC16_A>;
impl OPAC16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC16_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC16_A::TP0),
            1 => Val(OPAC16_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC16_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC16_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC16`"]
pub struct OPAC16_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC16_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC16_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC16_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 23"]
    #[inline(always)]
    pub fn opac23(&self) -> OPAC23_R {
        OPAC23_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 22"]
    #[inline(always)]
    pub fn opac22(&self) -> OPAC22_R {
        OPAC22_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 21"]
    #[inline(always)]
    pub fn opac21(&self) -> OPAC21_R {
        OPAC21_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 20"]
    #[inline(always)]
    pub fn opac20(&self) -> OPAC20_R {
        OPAC20_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 19"]
    #[inline(always)]
    pub fn opac19(&self) -> OPAC19_R {
        OPAC19_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 18"]
    #[inline(always)]
    pub fn opac18(&self) -> OPAC18_R {
        OPAC18_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 17"]
    #[inline(always)]
    pub fn opac17(&self) -> OPAC17_R {
        OPAC17_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 16"]
    #[inline(always)]
    pub fn opac16(&self) -> OPAC16_R {
        OPAC16_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 23"]
    #[inline(always)]
    pub fn opac23(&mut self) -> OPAC23_W {
        OPAC23_W { w: self }
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 22"]
    #[inline(always)]
    pub fn opac22(&mut self) -> OPAC22_W {
        OPAC22_W { w: self }
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 21"]
    #[inline(always)]
    pub fn opac21(&mut self) -> OPAC21_W {
        OPAC21_W { w: self }
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 20"]
    #[inline(always)]
    pub fn opac20(&mut self) -> OPAC20_W {
        OPAC20_W { w: self }
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 19"]
    #[inline(always)]
    pub fn opac19(&mut self) -> OPAC19_W {
        OPAC19_W { w: self }
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 18"]
    #[inline(always)]
    pub fn opac18(&mut self) -> OPAC18_W {
        OPAC18_W { w: self }
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 17"]
    #[inline(always)]
    pub fn opac17(&mut self) -> OPAC17_W {
        OPAC17_W { w: self }
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 16"]
    #[inline(always)]
    pub fn opac16(&mut self) -> OPAC16_W {
        OPAC16_W { w: self }
    }
}
