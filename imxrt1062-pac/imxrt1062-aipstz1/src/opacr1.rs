#[doc = "Reader of register OPACR1"]
pub type R = crate::R<u32, super::OPACR1>;
#[doc = "Writer for register OPACR1"]
pub type W = crate::W<u32, super::OPACR1>;
#[doc = "Register OPACR1 `reset()`'s with value 0x4444_4444"]
impl crate::ResetValue for super::OPACR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4444_4444
    }
}
#[doc = "Off-platform Peripheral Access Control 15\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC15_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC15_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC15_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC15`"]
pub type OPAC15_R = crate::R<u8, OPAC15_A>;
impl OPAC15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC15_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC15_A::TP0),
            1 => Val(OPAC15_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC15_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC15_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC15`"]
pub struct OPAC15_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC15_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC15_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 14\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC14_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC14_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC14_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC14`"]
pub type OPAC14_R = crate::R<u8, OPAC14_A>;
impl OPAC14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC14_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC14_A::TP0),
            1 => Val(OPAC14_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC14_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC14_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC14`"]
pub struct OPAC14_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC14_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC14_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 13\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC13_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC13_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC13_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC13`"]
pub type OPAC13_R = crate::R<u8, OPAC13_A>;
impl OPAC13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC13_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC13_A::TP0),
            1 => Val(OPAC13_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC13_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC13_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC13`"]
pub struct OPAC13_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC13_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC13_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 12\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC12_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC12_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC12_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC12`"]
pub type OPAC12_R = crate::R<u8, OPAC12_A>;
impl OPAC12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC12_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC12_A::TP0),
            1 => Val(OPAC12_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC12_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC12_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC12`"]
pub struct OPAC12_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC12_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC12_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 11\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC11_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC11_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC11_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC11`"]
pub type OPAC11_R = crate::R<u8, OPAC11_A>;
impl OPAC11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC11_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC11_A::TP0),
            1 => Val(OPAC11_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC11_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC11_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC11`"]
pub struct OPAC11_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC11_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC11_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 10\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC10_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC10_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC10`"]
pub type OPAC10_R = crate::R<u8, OPAC10_A>;
impl OPAC10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC10_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC10_A::TP0),
            1 => Val(OPAC10_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC10_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC10_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC10`"]
pub struct OPAC10_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC10_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC10_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 9\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC9_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC9_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC9`"]
pub type OPAC9_R = crate::R<u8, OPAC9_A>;
impl OPAC9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC9_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC9_A::TP0),
            1 => Val(OPAC9_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC9_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC9_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC9`"]
pub struct OPAC9_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC9_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC9_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 8\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC8_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC8_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC8`"]
pub type OPAC8_R = crate::R<u8, OPAC8_A>;
impl OPAC8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC8_A::TP0),
            1 => Val(OPAC8_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC8_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC8_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC8`"]
pub struct OPAC8_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC8_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC8_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 15"]
    #[inline(always)]
    pub fn opac15(&self) -> OPAC15_R {
        OPAC15_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 14"]
    #[inline(always)]
    pub fn opac14(&self) -> OPAC14_R {
        OPAC14_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 13"]
    #[inline(always)]
    pub fn opac13(&self) -> OPAC13_R {
        OPAC13_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 12"]
    #[inline(always)]
    pub fn opac12(&self) -> OPAC12_R {
        OPAC12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 11"]
    #[inline(always)]
    pub fn opac11(&self) -> OPAC11_R {
        OPAC11_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 10"]
    #[inline(always)]
    pub fn opac10(&self) -> OPAC10_R {
        OPAC10_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 9"]
    #[inline(always)]
    pub fn opac9(&self) -> OPAC9_R {
        OPAC9_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 8"]
    #[inline(always)]
    pub fn opac8(&self) -> OPAC8_R {
        OPAC8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 15"]
    #[inline(always)]
    pub fn opac15(&mut self) -> OPAC15_W {
        OPAC15_W { w: self }
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 14"]
    #[inline(always)]
    pub fn opac14(&mut self) -> OPAC14_W {
        OPAC14_W { w: self }
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 13"]
    #[inline(always)]
    pub fn opac13(&mut self) -> OPAC13_W {
        OPAC13_W { w: self }
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 12"]
    #[inline(always)]
    pub fn opac12(&mut self) -> OPAC12_W {
        OPAC12_W { w: self }
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 11"]
    #[inline(always)]
    pub fn opac11(&mut self) -> OPAC11_W {
        OPAC11_W { w: self }
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 10"]
    #[inline(always)]
    pub fn opac10(&mut self) -> OPAC10_W {
        OPAC10_W { w: self }
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 9"]
    #[inline(always)]
    pub fn opac9(&mut self) -> OPAC9_W {
        OPAC9_W { w: self }
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 8"]
    #[inline(always)]
    pub fn opac8(&mut self) -> OPAC8_W {
        OPAC8_W { w: self }
    }
}
