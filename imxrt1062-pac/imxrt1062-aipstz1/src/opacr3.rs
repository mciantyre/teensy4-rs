#[doc = "Reader of register OPACR3"]
pub type R = crate::R<u32, super::OPACR3>;
#[doc = "Writer for register OPACR3"]
pub type W = crate::W<u32, super::OPACR3>;
#[doc = "Register OPACR3 `reset()`'s with value 0x4444_4444"]
impl crate::ResetValue for super::OPACR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4444_4444
    }
}
#[doc = "Off-platform Peripheral Access Control 31\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC31_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC31_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC31_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC31`"]
pub type OPAC31_R = crate::R<u8, OPAC31_A>;
impl OPAC31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC31_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC31_A::TP0),
            1 => Val(OPAC31_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC31_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC31_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC31`"]
pub struct OPAC31_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC31_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC31_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC31_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 30\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC30_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC30_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC30_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC30`"]
pub type OPAC30_R = crate::R<u8, OPAC30_A>;
impl OPAC30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC30_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC30_A::TP0),
            1 => Val(OPAC30_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC30_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC30_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC30`"]
pub struct OPAC30_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC30_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC30_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC30_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 29\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC29_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC29_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC29_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC29`"]
pub type OPAC29_R = crate::R<u8, OPAC29_A>;
impl OPAC29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC29_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC29_A::TP0),
            1 => Val(OPAC29_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC29_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC29_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC29`"]
pub struct OPAC29_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC29_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC29_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC29_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 28\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC28_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC28_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC28_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC28`"]
pub type OPAC28_R = crate::R<u8, OPAC28_A>;
impl OPAC28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC28_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC28_A::TP0),
            1 => Val(OPAC28_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC28_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC28_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC28`"]
pub struct OPAC28_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC28_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC28_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC28_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 27\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC27_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC27_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC27_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC27`"]
pub type OPAC27_R = crate::R<u8, OPAC27_A>;
impl OPAC27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC27_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC27_A::TP0),
            1 => Val(OPAC27_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC27_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC27_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC27`"]
pub struct OPAC27_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC27_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC27_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC27_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 26\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC26_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC26_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC26_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC26`"]
pub type OPAC26_R = crate::R<u8, OPAC26_A>;
impl OPAC26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC26_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC26_A::TP0),
            1 => Val(OPAC26_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC26_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC26_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC26`"]
pub struct OPAC26_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC26_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC26_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC26_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 25\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC25_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC25_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC25_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC25`"]
pub type OPAC25_R = crate::R<u8, OPAC25_A>;
impl OPAC25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC25_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC25_A::TP0),
            1 => Val(OPAC25_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC25_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC25_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC25`"]
pub struct OPAC25_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC25_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC25_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC25_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 24\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC24_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC24_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC24_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC24`"]
pub type OPAC24_R = crate::R<u8, OPAC24_A>;
impl OPAC24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC24_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC24_A::TP0),
            1 => Val(OPAC24_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC24_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC24_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC24`"]
pub struct OPAC24_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC24_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC24_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC24_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 31"]
    #[inline(always)]
    pub fn opac31(&self) -> OPAC31_R {
        OPAC31_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 30"]
    #[inline(always)]
    pub fn opac30(&self) -> OPAC30_R {
        OPAC30_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 29"]
    #[inline(always)]
    pub fn opac29(&self) -> OPAC29_R {
        OPAC29_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 28"]
    #[inline(always)]
    pub fn opac28(&self) -> OPAC28_R {
        OPAC28_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 27"]
    #[inline(always)]
    pub fn opac27(&self) -> OPAC27_R {
        OPAC27_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 26"]
    #[inline(always)]
    pub fn opac26(&self) -> OPAC26_R {
        OPAC26_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 25"]
    #[inline(always)]
    pub fn opac25(&self) -> OPAC25_R {
        OPAC25_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 24"]
    #[inline(always)]
    pub fn opac24(&self) -> OPAC24_R {
        OPAC24_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 31"]
    #[inline(always)]
    pub fn opac31(&mut self) -> OPAC31_W {
        OPAC31_W { w: self }
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 30"]
    #[inline(always)]
    pub fn opac30(&mut self) -> OPAC30_W {
        OPAC30_W { w: self }
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 29"]
    #[inline(always)]
    pub fn opac29(&mut self) -> OPAC29_W {
        OPAC29_W { w: self }
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 28"]
    #[inline(always)]
    pub fn opac28(&mut self) -> OPAC28_W {
        OPAC28_W { w: self }
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 27"]
    #[inline(always)]
    pub fn opac27(&mut self) -> OPAC27_W {
        OPAC27_W { w: self }
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 26"]
    #[inline(always)]
    pub fn opac26(&mut self) -> OPAC26_W {
        OPAC26_W { w: self }
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 25"]
    #[inline(always)]
    pub fn opac25(&mut self) -> OPAC25_W {
        OPAC25_W { w: self }
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 24"]
    #[inline(always)]
    pub fn opac24(&mut self) -> OPAC24_W {
        OPAC24_W { w: self }
    }
}
