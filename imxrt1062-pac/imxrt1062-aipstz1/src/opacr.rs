#[doc = "Reader of register OPACR"]
pub type R = crate::R<u32, super::OPACR>;
#[doc = "Writer for register OPACR"]
pub type W = crate::W<u32, super::OPACR>;
#[doc = "Register OPACR `reset()`'s with value 0x4444_4444"]
impl crate::ResetValue for super::OPACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4444_4444
    }
}
#[doc = "Off-platform Peripheral Access Control 7\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC7_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC7_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC7`"]
pub type OPAC7_R = crate::R<u8, OPAC7_A>;
impl OPAC7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC7_A::TP0),
            1 => Val(OPAC7_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC7_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC7_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC7`"]
pub struct OPAC7_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC7_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC7_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 6\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC6_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC6_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC6`"]
pub type OPAC6_R = crate::R<u8, OPAC6_A>;
impl OPAC6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC6_A::TP0),
            1 => Val(OPAC6_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC6_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC6_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC6`"]
pub struct OPAC6_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC6_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC6_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 5\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC5_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC5_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC5`"]
pub type OPAC5_R = crate::R<u8, OPAC5_A>;
impl OPAC5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC5_A::TP0),
            1 => Val(OPAC5_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC5_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC5_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC5`"]
pub struct OPAC5_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC5_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC5_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 4\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC4_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC4_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC4`"]
pub type OPAC4_R = crate::R<u8, OPAC4_A>;
impl OPAC4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC4_A::TP0),
            1 => Val(OPAC4_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC4_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC4_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC4`"]
pub struct OPAC4_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC4_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC4_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 3\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC3_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC3_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC3`"]
pub type OPAC3_R = crate::R<u8, OPAC3_A>;
impl OPAC3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC3_A::TP0),
            1 => Val(OPAC3_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC3_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC3_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC3`"]
pub struct OPAC3_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC3_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC3_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 2\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC2_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC2_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC2`"]
pub type OPAC2_R = crate::R<u8, OPAC2_A>;
impl OPAC2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC2_A::TP0),
            1 => Val(OPAC2_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC2_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC2_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC2`"]
pub struct OPAC2_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC2_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC2_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 1\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC1_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC1_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC1`"]
pub type OPAC1_R = crate::R<u8, OPAC1_A>;
impl OPAC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC1_A::TP0),
            1 => Val(OPAC1_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC1_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC1_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC1`"]
pub struct OPAC1_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC1_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC1_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Off-platform Peripheral Access Control 0\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAC0_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC0_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OPAC0`"]
pub type OPAC0_R = crate::R<u8, OPAC0_A>;
impl OPAC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OPAC0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OPAC0_A::TP0),
            1 => Val(OPAC0_A::TP1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC0_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC0_A::TP1
    }
}
#[doc = "Write proxy for field `OPAC0`"]
pub struct OPAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> OPAC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPAC0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC0_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC0_A::TP1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 7"]
    #[inline(always)]
    pub fn opac7(&self) -> OPAC7_R {
        OPAC7_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 6"]
    #[inline(always)]
    pub fn opac6(&self) -> OPAC6_R {
        OPAC6_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 5"]
    #[inline(always)]
    pub fn opac5(&self) -> OPAC5_R {
        OPAC5_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 4"]
    #[inline(always)]
    pub fn opac4(&self) -> OPAC4_R {
        OPAC4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 3"]
    #[inline(always)]
    pub fn opac3(&self) -> OPAC3_R {
        OPAC3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 2"]
    #[inline(always)]
    pub fn opac2(&self) -> OPAC2_R {
        OPAC2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 1"]
    #[inline(always)]
    pub fn opac1(&self) -> OPAC1_R {
        OPAC1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 0"]
    #[inline(always)]
    pub fn opac0(&self) -> OPAC0_R {
        OPAC0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 7"]
    #[inline(always)]
    pub fn opac7(&mut self) -> OPAC7_W {
        OPAC7_W { w: self }
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 6"]
    #[inline(always)]
    pub fn opac6(&mut self) -> OPAC6_W {
        OPAC6_W { w: self }
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 5"]
    #[inline(always)]
    pub fn opac5(&mut self) -> OPAC5_W {
        OPAC5_W { w: self }
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 4"]
    #[inline(always)]
    pub fn opac4(&mut self) -> OPAC4_W {
        OPAC4_W { w: self }
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 3"]
    #[inline(always)]
    pub fn opac3(&mut self) -> OPAC3_W {
        OPAC3_W { w: self }
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 2"]
    #[inline(always)]
    pub fn opac2(&mut self) -> OPAC2_W {
        OPAC2_W { w: self }
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 1"]
    #[inline(always)]
    pub fn opac1(&mut self) -> OPAC1_W {
        OPAC1_W { w: self }
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 0"]
    #[inline(always)]
    pub fn opac0(&mut self) -> OPAC0_W {
        OPAC0_W { w: self }
    }
}
