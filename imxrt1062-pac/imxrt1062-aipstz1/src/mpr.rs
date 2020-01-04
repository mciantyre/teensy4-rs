#[doc = "Reader of register MPR"]
pub type R = crate::R<u32, super::MPR>;
#[doc = "Writer for register MPR"]
pub type W = crate::W<u32, super::MPR>;
#[doc = "Register MPR `reset()`'s with value 0x7700_0000"]
impl crate::ResetValue for super::MPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7700_0000
    }
}
#[doc = "Master 5 Priviledge, Buffer, Read, Write Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPROT5_A {
    #[doc = "0: Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    MPL0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    MPL1 = 1,
}
impl From<MPROT5_A> for u8 {
    #[inline(always)]
    fn from(variant: MPROT5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MPROT5`"]
pub type MPROT5_R = crate::R<u8, MPROT5_A>;
impl MPROT5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MPROT5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MPROT5_A::MPL0),
            1 => Val(MPROT5_A::MPL1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline(always)]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT5_A::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline(always)]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT5_A::MPL1
    }
}
#[doc = "Write proxy for field `MPROT5`"]
pub struct MPROT5_W<'a> {
    w: &'a mut W,
}
impl<'a> MPROT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPROT5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    #[inline(always)]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT5_A::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    #[inline(always)]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT5_A::MPL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Master 3 Priviledge, Buffer, Read, Write Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPROT3_A {
    #[doc = "0: Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    MPL0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    MPL1 = 1,
}
impl From<MPROT3_A> for u8 {
    #[inline(always)]
    fn from(variant: MPROT3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MPROT3`"]
pub type MPROT3_R = crate::R<u8, MPROT3_A>;
impl MPROT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MPROT3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MPROT3_A::MPL0),
            1 => Val(MPROT3_A::MPL1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline(always)]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT3_A::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline(always)]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT3_A::MPL1
    }
}
#[doc = "Write proxy for field `MPROT3`"]
pub struct MPROT3_W<'a> {
    w: &'a mut W,
}
impl<'a> MPROT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPROT3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    #[inline(always)]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT3_A::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    #[inline(always)]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT3_A::MPL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Master 2 Priviledge, Buffer, Read, Write Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPROT2_A {
    #[doc = "0: Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    MPL0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    MPL1 = 1,
}
impl From<MPROT2_A> for u8 {
    #[inline(always)]
    fn from(variant: MPROT2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MPROT2`"]
pub type MPROT2_R = crate::R<u8, MPROT2_A>;
impl MPROT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MPROT2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MPROT2_A::MPL0),
            1 => Val(MPROT2_A::MPL1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline(always)]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT2_A::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline(always)]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT2_A::MPL1
    }
}
#[doc = "Write proxy for field `MPROT2`"]
pub struct MPROT2_W<'a> {
    w: &'a mut W,
}
impl<'a> MPROT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPROT2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    #[inline(always)]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT2_A::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    #[inline(always)]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT2_A::MPL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Master 1 Priviledge, Buffer, Read, Write Control\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPROT1_A {
    #[doc = "0: Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    MPL0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    MPL1 = 1,
}
impl From<MPROT1_A> for u8 {
    #[inline(always)]
    fn from(variant: MPROT1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MPROT1`"]
pub type MPROT1_R = crate::R<u8, MPROT1_A>;
impl MPROT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MPROT1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MPROT1_A::MPL0),
            1 => Val(MPROT1_A::MPL1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline(always)]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT1_A::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline(always)]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT1_A::MPL1
    }
}
#[doc = "Write proxy for field `MPROT1`"]
pub struct MPROT1_W<'a> {
    w: &'a mut W,
}
impl<'a> MPROT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPROT1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    #[inline(always)]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT1_A::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    #[inline(always)]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT1_A::MPL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Master 0 Priviledge, Buffer, Read, Write Control\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPROT0_A {
    #[doc = "0: Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    MPL0 = 0,
    #[doc = "1: Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    MPL1 = 1,
}
impl From<MPROT0_A> for u8 {
    #[inline(always)]
    fn from(variant: MPROT0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MPROT0`"]
pub type MPROT0_R = crate::R<u8, MPROT0_A>;
impl MPROT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MPROT0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MPROT0_A::MPL0),
            1 => Val(MPROT0_A::MPL1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MPL0`"]
    #[inline(always)]
    pub fn is_mpl0(&self) -> bool {
        *self == MPROT0_A::MPL0
    }
    #[doc = "Checks if the value of the field is `MPL1`"]
    #[inline(always)]
    pub fn is_mpl1(&self) -> bool {
        *self == MPROT0_A::MPL1
    }
}
#[doc = "Write proxy for field `MPROT0`"]
pub struct MPROT0_W<'a> {
    w: &'a mut W,
}
impl<'a> MPROT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPROT0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Accesses from this master are forced to user-mode (ips_supervisor_access is forced to zero) regardless of the hprot\\[1\\]
access attribute."]
    #[inline(always)]
    pub fn mpl0(self) -> &'a mut W {
        self.variant(MPROT0_A::MPL0)
    }
    #[doc = "Accesses from this master are not forced to user-mode. The hprot\\[1\\]
access attribute is used directly to determine ips_supervisor_access."]
    #[inline(always)]
    pub fn mpl1(self) -> &'a mut W {
        self.variant(MPROT0_A::MPL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - Master 5 Priviledge, Buffer, Read, Write Control."]
    #[inline(always)]
    pub fn mprot5(&self) -> MPROT5_R {
        MPROT5_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Master 3 Priviledge, Buffer, Read, Write Control."]
    #[inline(always)]
    pub fn mprot3(&self) -> MPROT3_R {
        MPROT3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Master 2 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    pub fn mprot2(&self) -> MPROT2_R {
        MPROT2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Master 1 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    pub fn mprot1(&self) -> MPROT1_R {
        MPROT1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Master 0 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    pub fn mprot0(&self) -> MPROT0_R {
        MPROT0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Master 5 Priviledge, Buffer, Read, Write Control."]
    #[inline(always)]
    pub fn mprot5(&mut self) -> MPROT5_W {
        MPROT5_W { w: self }
    }
    #[doc = "Bits 16:19 - Master 3 Priviledge, Buffer, Read, Write Control."]
    #[inline(always)]
    pub fn mprot3(&mut self) -> MPROT3_W {
        MPROT3_W { w: self }
    }
    #[doc = "Bits 20:23 - Master 2 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    pub fn mprot2(&mut self) -> MPROT2_W {
        MPROT2_W { w: self }
    }
    #[doc = "Bits 24:27 - Master 1 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    pub fn mprot1(&mut self) -> MPROT1_W {
        MPROT1_W { w: self }
    }
    #[doc = "Bits 28:31 - Master 0 Priviledge, Buffer, Read, Write Control"]
    #[inline(always)]
    pub fn mprot0(&mut self) -> MPROT0_W {
        MPROT0_W { w: self }
    }
}
