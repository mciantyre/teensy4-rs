#[doc = "Reader of register CPACR"]
pub type R = crate::R<u32, super::CPACR>;
#[doc = "Writer for register CPACR"]
pub type W = crate::W<u32, super::CPACR>;
#[doc = "Register CPACR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Access privileges for coprocessor 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP0_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP0_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP0_1 = 1,
    #[doc = "3: Full access."]
    CP0_3 = 3,
}
impl From<CP0_A> for u8 {
    #[inline(always)]
    fn from(variant: CP0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP0`"]
pub type CP0_R = crate::R<u8, CP0_A>;
impl CP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP0_A::CP0_0),
            1 => Val(CP0_A::CP0_1),
            3 => Val(CP0_A::CP0_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP0_0`"]
    #[inline(always)]
    pub fn is_cp0_0(&self) -> bool {
        *self == CP0_A::CP0_0
    }
    #[doc = "Checks if the value of the field is `CP0_1`"]
    #[inline(always)]
    pub fn is_cp0_1(&self) -> bool {
        *self == CP0_A::CP0_1
    }
    #[doc = "Checks if the value of the field is `CP0_3`"]
    #[inline(always)]
    pub fn is_cp0_3(&self) -> bool {
        *self == CP0_A::CP0_3
    }
}
#[doc = "Write proxy for field `CP0`"]
pub struct CP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp0_0(self) -> &'a mut W {
        self.variant(CP0_A::CP0_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp0_1(self) -> &'a mut W {
        self.variant(CP0_A::CP0_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp0_3(self) -> &'a mut W {
        self.variant(CP0_A::CP0_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP1_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP1_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP1_1 = 1,
    #[doc = "3: Full access."]
    CP1_3 = 3,
}
impl From<CP1_A> for u8 {
    #[inline(always)]
    fn from(variant: CP1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP1`"]
pub type CP1_R = crate::R<u8, CP1_A>;
impl CP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP1_A::CP1_0),
            1 => Val(CP1_A::CP1_1),
            3 => Val(CP1_A::CP1_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP1_0`"]
    #[inline(always)]
    pub fn is_cp1_0(&self) -> bool {
        *self == CP1_A::CP1_0
    }
    #[doc = "Checks if the value of the field is `CP1_1`"]
    #[inline(always)]
    pub fn is_cp1_1(&self) -> bool {
        *self == CP1_A::CP1_1
    }
    #[doc = "Checks if the value of the field is `CP1_3`"]
    #[inline(always)]
    pub fn is_cp1_3(&self) -> bool {
        *self == CP1_A::CP1_3
    }
}
#[doc = "Write proxy for field `CP1`"]
pub struct CP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp1_0(self) -> &'a mut W {
        self.variant(CP1_A::CP1_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp1_1(self) -> &'a mut W {
        self.variant(CP1_A::CP1_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp1_3(self) -> &'a mut W {
        self.variant(CP1_A::CP1_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP2_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP2_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP2_1 = 1,
    #[doc = "3: Full access."]
    CP2_3 = 3,
}
impl From<CP2_A> for u8 {
    #[inline(always)]
    fn from(variant: CP2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP2`"]
pub type CP2_R = crate::R<u8, CP2_A>;
impl CP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP2_A::CP2_0),
            1 => Val(CP2_A::CP2_1),
            3 => Val(CP2_A::CP2_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP2_0`"]
    #[inline(always)]
    pub fn is_cp2_0(&self) -> bool {
        *self == CP2_A::CP2_0
    }
    #[doc = "Checks if the value of the field is `CP2_1`"]
    #[inline(always)]
    pub fn is_cp2_1(&self) -> bool {
        *self == CP2_A::CP2_1
    }
    #[doc = "Checks if the value of the field is `CP2_3`"]
    #[inline(always)]
    pub fn is_cp2_3(&self) -> bool {
        *self == CP2_A::CP2_3
    }
}
#[doc = "Write proxy for field `CP2`"]
pub struct CP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp2_0(self) -> &'a mut W {
        self.variant(CP2_A::CP2_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp2_1(self) -> &'a mut W {
        self.variant(CP2_A::CP2_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp2_3(self) -> &'a mut W {
        self.variant(CP2_A::CP2_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP3_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP3_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP3_1 = 1,
    #[doc = "3: Full access."]
    CP3_3 = 3,
}
impl From<CP3_A> for u8 {
    #[inline(always)]
    fn from(variant: CP3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP3`"]
pub type CP3_R = crate::R<u8, CP3_A>;
impl CP3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP3_A::CP3_0),
            1 => Val(CP3_A::CP3_1),
            3 => Val(CP3_A::CP3_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP3_0`"]
    #[inline(always)]
    pub fn is_cp3_0(&self) -> bool {
        *self == CP3_A::CP3_0
    }
    #[doc = "Checks if the value of the field is `CP3_1`"]
    #[inline(always)]
    pub fn is_cp3_1(&self) -> bool {
        *self == CP3_A::CP3_1
    }
    #[doc = "Checks if the value of the field is `CP3_3`"]
    #[inline(always)]
    pub fn is_cp3_3(&self) -> bool {
        *self == CP3_A::CP3_3
    }
}
#[doc = "Write proxy for field `CP3`"]
pub struct CP3_W<'a> {
    w: &'a mut W,
}
impl<'a> CP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp3_0(self) -> &'a mut W {
        self.variant(CP3_A::CP3_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp3_1(self) -> &'a mut W {
        self.variant(CP3_A::CP3_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp3_3(self) -> &'a mut W {
        self.variant(CP3_A::CP3_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP4_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP4_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP4_1 = 1,
    #[doc = "3: Full access."]
    CP4_3 = 3,
}
impl From<CP4_A> for u8 {
    #[inline(always)]
    fn from(variant: CP4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP4`"]
pub type CP4_R = crate::R<u8, CP4_A>;
impl CP4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP4_A::CP4_0),
            1 => Val(CP4_A::CP4_1),
            3 => Val(CP4_A::CP4_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP4_0`"]
    #[inline(always)]
    pub fn is_cp4_0(&self) -> bool {
        *self == CP4_A::CP4_0
    }
    #[doc = "Checks if the value of the field is `CP4_1`"]
    #[inline(always)]
    pub fn is_cp4_1(&self) -> bool {
        *self == CP4_A::CP4_1
    }
    #[doc = "Checks if the value of the field is `CP4_3`"]
    #[inline(always)]
    pub fn is_cp4_3(&self) -> bool {
        *self == CP4_A::CP4_3
    }
}
#[doc = "Write proxy for field `CP4`"]
pub struct CP4_W<'a> {
    w: &'a mut W,
}
impl<'a> CP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp4_0(self) -> &'a mut W {
        self.variant(CP4_A::CP4_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp4_1(self) -> &'a mut W {
        self.variant(CP4_A::CP4_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp4_3(self) -> &'a mut W {
        self.variant(CP4_A::CP4_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP5_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP5_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP5_1 = 1,
    #[doc = "3: Full access."]
    CP5_3 = 3,
}
impl From<CP5_A> for u8 {
    #[inline(always)]
    fn from(variant: CP5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP5`"]
pub type CP5_R = crate::R<u8, CP5_A>;
impl CP5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP5_A::CP5_0),
            1 => Val(CP5_A::CP5_1),
            3 => Val(CP5_A::CP5_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP5_0`"]
    #[inline(always)]
    pub fn is_cp5_0(&self) -> bool {
        *self == CP5_A::CP5_0
    }
    #[doc = "Checks if the value of the field is `CP5_1`"]
    #[inline(always)]
    pub fn is_cp5_1(&self) -> bool {
        *self == CP5_A::CP5_1
    }
    #[doc = "Checks if the value of the field is `CP5_3`"]
    #[inline(always)]
    pub fn is_cp5_3(&self) -> bool {
        *self == CP5_A::CP5_3
    }
}
#[doc = "Write proxy for field `CP5`"]
pub struct CP5_W<'a> {
    w: &'a mut W,
}
impl<'a> CP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp5_0(self) -> &'a mut W {
        self.variant(CP5_A::CP5_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp5_1(self) -> &'a mut W {
        self.variant(CP5_A::CP5_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp5_3(self) -> &'a mut W {
        self.variant(CP5_A::CP5_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP6_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP6_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP6_1 = 1,
    #[doc = "3: Full access."]
    CP6_3 = 3,
}
impl From<CP6_A> for u8 {
    #[inline(always)]
    fn from(variant: CP6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP6`"]
pub type CP6_R = crate::R<u8, CP6_A>;
impl CP6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP6_A::CP6_0),
            1 => Val(CP6_A::CP6_1),
            3 => Val(CP6_A::CP6_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP6_0`"]
    #[inline(always)]
    pub fn is_cp6_0(&self) -> bool {
        *self == CP6_A::CP6_0
    }
    #[doc = "Checks if the value of the field is `CP6_1`"]
    #[inline(always)]
    pub fn is_cp6_1(&self) -> bool {
        *self == CP6_A::CP6_1
    }
    #[doc = "Checks if the value of the field is `CP6_3`"]
    #[inline(always)]
    pub fn is_cp6_3(&self) -> bool {
        *self == CP6_A::CP6_3
    }
}
#[doc = "Write proxy for field `CP6`"]
pub struct CP6_W<'a> {
    w: &'a mut W,
}
impl<'a> CP6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp6_0(self) -> &'a mut W {
        self.variant(CP6_A::CP6_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp6_1(self) -> &'a mut W {
        self.variant(CP6_A::CP6_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp6_3(self) -> &'a mut W {
        self.variant(CP6_A::CP6_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP7_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP7_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP7_1 = 1,
    #[doc = "3: Full access."]
    CP7_3 = 3,
}
impl From<CP7_A> for u8 {
    #[inline(always)]
    fn from(variant: CP7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP7`"]
pub type CP7_R = crate::R<u8, CP7_A>;
impl CP7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP7_A::CP7_0),
            1 => Val(CP7_A::CP7_1),
            3 => Val(CP7_A::CP7_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP7_0`"]
    #[inline(always)]
    pub fn is_cp7_0(&self) -> bool {
        *self == CP7_A::CP7_0
    }
    #[doc = "Checks if the value of the field is `CP7_1`"]
    #[inline(always)]
    pub fn is_cp7_1(&self) -> bool {
        *self == CP7_A::CP7_1
    }
    #[doc = "Checks if the value of the field is `CP7_3`"]
    #[inline(always)]
    pub fn is_cp7_3(&self) -> bool {
        *self == CP7_A::CP7_3
    }
}
#[doc = "Write proxy for field `CP7`"]
pub struct CP7_W<'a> {
    w: &'a mut W,
}
impl<'a> CP7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp7_0(self) -> &'a mut W {
        self.variant(CP7_A::CP7_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp7_1(self) -> &'a mut W {
        self.variant(CP7_A::CP7_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp7_3(self) -> &'a mut W {
        self.variant(CP7_A::CP7_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP10_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP10_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP10_1 = 1,
    #[doc = "3: Full access."]
    CP10_3 = 3,
}
impl From<CP10_A> for u8 {
    #[inline(always)]
    fn from(variant: CP10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP10`"]
pub type CP10_R = crate::R<u8, CP10_A>;
impl CP10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP10_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP10_A::CP10_0),
            1 => Val(CP10_A::CP10_1),
            3 => Val(CP10_A::CP10_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP10_0`"]
    #[inline(always)]
    pub fn is_cp10_0(&self) -> bool {
        *self == CP10_A::CP10_0
    }
    #[doc = "Checks if the value of the field is `CP10_1`"]
    #[inline(always)]
    pub fn is_cp10_1(&self) -> bool {
        *self == CP10_A::CP10_1
    }
    #[doc = "Checks if the value of the field is `CP10_3`"]
    #[inline(always)]
    pub fn is_cp10_3(&self) -> bool {
        *self == CP10_A::CP10_3
    }
}
#[doc = "Write proxy for field `CP10`"]
pub struct CP10_W<'a> {
    w: &'a mut W,
}
impl<'a> CP10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp10_0(self) -> &'a mut W {
        self.variant(CP10_A::CP10_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp10_1(self) -> &'a mut W {
        self.variant(CP10_A::CP10_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp10_3(self) -> &'a mut W {
        self.variant(CP10_A::CP10_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Access privileges for coprocessor 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CP11_A {
    #[doc = "0: Access denied. Any attempted access generates a NOCP UsageFault."]
    CP11_0 = 0,
    #[doc = "1: Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    CP11_1 = 1,
    #[doc = "3: Full access."]
    CP11_3 = 3,
}
impl From<CP11_A> for u8 {
    #[inline(always)]
    fn from(variant: CP11_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CP11`"]
pub type CP11_R = crate::R<u8, CP11_A>;
impl CP11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CP11_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CP11_A::CP11_0),
            1 => Val(CP11_A::CP11_1),
            3 => Val(CP11_A::CP11_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CP11_0`"]
    #[inline(always)]
    pub fn is_cp11_0(&self) -> bool {
        *self == CP11_A::CP11_0
    }
    #[doc = "Checks if the value of the field is `CP11_1`"]
    #[inline(always)]
    pub fn is_cp11_1(&self) -> bool {
        *self == CP11_A::CP11_1
    }
    #[doc = "Checks if the value of the field is `CP11_3`"]
    #[inline(always)]
    pub fn is_cp11_3(&self) -> bool {
        *self == CP11_A::CP11_3
    }
}
#[doc = "Write proxy for field `CP11`"]
pub struct CP11_W<'a> {
    w: &'a mut W,
}
impl<'a> CP11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CP11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Access denied. Any attempted access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp11_0(self) -> &'a mut W {
        self.variant(CP11_A::CP11_0)
    }
    #[doc = "Privileged access only. An unprivileged access generates a NOCP UsageFault."]
    #[inline(always)]
    pub fn cp11_1(self) -> &'a mut W {
        self.variant(CP11_A::CP11_1)
    }
    #[doc = "Full access."]
    #[inline(always)]
    pub fn cp11_3(self) -> &'a mut W {
        self.variant(CP11_A::CP11_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Access privileges for coprocessor 0."]
    #[inline(always)]
    pub fn cp0(&self) -> CP0_R {
        CP0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Access privileges for coprocessor 1."]
    #[inline(always)]
    pub fn cp1(&self) -> CP1_R {
        CP1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Access privileges for coprocessor 2."]
    #[inline(always)]
    pub fn cp2(&self) -> CP2_R {
        CP2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Access privileges for coprocessor 3."]
    #[inline(always)]
    pub fn cp3(&self) -> CP3_R {
        CP3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Access privileges for coprocessor 4."]
    #[inline(always)]
    pub fn cp4(&self) -> CP4_R {
        CP4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Access privileges for coprocessor 5."]
    #[inline(always)]
    pub fn cp5(&self) -> CP5_R {
        CP5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Access privileges for coprocessor 6."]
    #[inline(always)]
    pub fn cp6(&self) -> CP6_R {
        CP6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Access privileges for coprocessor 7."]
    #[inline(always)]
    pub fn cp7(&self) -> CP7_R {
        CP7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline(always)]
    pub fn cp10(&self) -> CP10_R {
        CP10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline(always)]
    pub fn cp11(&self) -> CP11_R {
        CP11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Access privileges for coprocessor 0."]
    #[inline(always)]
    pub fn cp0(&mut self) -> CP0_W {
        CP0_W { w: self }
    }
    #[doc = "Bits 2:3 - Access privileges for coprocessor 1."]
    #[inline(always)]
    pub fn cp1(&mut self) -> CP1_W {
        CP1_W { w: self }
    }
    #[doc = "Bits 4:5 - Access privileges for coprocessor 2."]
    #[inline(always)]
    pub fn cp2(&mut self) -> CP2_W {
        CP2_W { w: self }
    }
    #[doc = "Bits 6:7 - Access privileges for coprocessor 3."]
    #[inline(always)]
    pub fn cp3(&mut self) -> CP3_W {
        CP3_W { w: self }
    }
    #[doc = "Bits 8:9 - Access privileges for coprocessor 4."]
    #[inline(always)]
    pub fn cp4(&mut self) -> CP4_W {
        CP4_W { w: self }
    }
    #[doc = "Bits 10:11 - Access privileges for coprocessor 5."]
    #[inline(always)]
    pub fn cp5(&mut self) -> CP5_W {
        CP5_W { w: self }
    }
    #[doc = "Bits 12:13 - Access privileges for coprocessor 6."]
    #[inline(always)]
    pub fn cp6(&mut self) -> CP6_W {
        CP6_W { w: self }
    }
    #[doc = "Bits 14:15 - Access privileges for coprocessor 7."]
    #[inline(always)]
    pub fn cp7(&mut self) -> CP7_W {
        CP7_W { w: self }
    }
    #[doc = "Bits 20:21 - Access privileges for coprocessor 10."]
    #[inline(always)]
    pub fn cp10(&mut self) -> CP10_W {
        CP10_W { w: self }
    }
    #[doc = "Bits 22:23 - Access privileges for coprocessor 11."]
    #[inline(always)]
    pub fn cp11(&mut self) -> CP11_W {
        CP11_W { w: self }
    }
}
