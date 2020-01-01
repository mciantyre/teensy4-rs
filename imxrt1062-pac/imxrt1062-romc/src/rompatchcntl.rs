#[doc = "Reader of register ROMPATCHCNTL"]
pub type R = crate::R<u32, super::ROMPATCHCNTL>;
#[doc = "Writer for register ROMPATCHCNTL"]
pub type W = crate::W<u32, super::ROMPATCHCNTL>;
#[doc = "Register ROMPATCHCNTL `reset()`'s with value 0x0840_0000"]
impl crate::ResetValue for super::ROMPATCHCNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0840_0000
    }
}
#[doc = "Data Fix Enable - Controls the use of the first 8 address comparators for 1-word data fix or for code patch routine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATAFIX_A {
    #[doc = "0: Address comparator triggers a opcode patch"]
    DATAFIX_0 = 0,
    #[doc = "1: Address comparator triggers a data fix"]
    DATAFIX_1 = 1,
}
impl From<DATAFIX_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAFIX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DATAFIX`"]
pub type DATAFIX_R = crate::R<u8, DATAFIX_A>;
impl DATAFIX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATAFIX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DATAFIX_A::DATAFIX_0),
            1 => Val(DATAFIX_A::DATAFIX_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DATAFIX_0`"]
    #[inline(always)]
    pub fn is_datafix_0(&self) -> bool {
        *self == DATAFIX_A::DATAFIX_0
    }
    #[doc = "Checks if the value of the field is `DATAFIX_1`"]
    #[inline(always)]
    pub fn is_datafix_1(&self) -> bool {
        *self == DATAFIX_A::DATAFIX_1
    }
}
#[doc = "Write proxy for field `DATAFIX`"]
pub struct DATAFIX_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAFIX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATAFIX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Address comparator triggers a opcode patch"]
    #[inline(always)]
    pub fn datafix_0(self) -> &'a mut W {
        self.variant(DATAFIX_A::DATAFIX_0)
    }
    #[doc = "Address comparator triggers a data fix"]
    #[inline(always)]
    pub fn datafix_1(self) -> &'a mut W {
        self.variant(DATAFIX_A::DATAFIX_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "ROMC Disable -- This bit, when set, disables all ROMC operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_A {
    #[doc = "0: Does not affect any ROMC functions (default)"]
    DIS_0 = 0,
    #[doc = "1: Disable all ROMC functions: data fixing, and opcode patching"]
    DIS_1 = 1,
}
impl From<DIS_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIS`"]
pub type DIS_R = crate::R<bool, DIS_A>;
impl DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_A {
        match self.bits {
            false => DIS_A::DIS_0,
            true => DIS_A::DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_0`"]
    #[inline(always)]
    pub fn is_dis_0(&self) -> bool {
        *self == DIS_A::DIS_0
    }
    #[doc = "Checks if the value of the field is `DIS_1`"]
    #[inline(always)]
    pub fn is_dis_1(&self) -> bool {
        *self == DIS_A::DIS_1
    }
}
#[doc = "Write proxy for field `DIS`"]
pub struct DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Does not affect any ROMC functions (default)"]
    #[inline(always)]
    pub fn dis_0(self) -> &'a mut W {
        self.variant(DIS_A::DIS_0)
    }
    #[doc = "Disable all ROMC functions: data fixing, and opcode patching"]
    #[inline(always)]
    pub fn dis_1(self) -> &'a mut W {
        self.variant(DIS_A::DIS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Fix Enable - Controls the use of the first 8 address comparators for 1-word data fix or for code patch routine"]
    #[inline(always)]
    pub fn datafix(&self) -> DATAFIX_R {
        DATAFIX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 29 - ROMC Disable -- This bit, when set, disables all ROMC operations"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Fix Enable - Controls the use of the first 8 address comparators for 1-word data fix or for code patch routine"]
    #[inline(always)]
    pub fn datafix(&mut self) -> DATAFIX_W {
        DATAFIX_W { w: self }
    }
    #[doc = "Bit 29 - ROMC Disable -- This bit, when set, disables all ROMC operations"]
    #[inline(always)]
    pub fn dis(&mut self) -> DIS_W {
        DIS_W { w: self }
    }
}
