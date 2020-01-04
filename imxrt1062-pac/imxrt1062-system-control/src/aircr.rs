#[doc = "Reader of register AIRCR"]
pub type R = crate::R<u32, super::AIRCR>;
#[doc = "Writer for register AIRCR"]
pub type W = crate::W<u32, super::AIRCR>;
#[doc = "Register AIRCR `reset()`'s with value 0xfa05_0000"]
impl crate::ResetValue for super::AIRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfa05_0000
    }
}
#[doc = "Writing 1 to this bit causes a local system reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTRESET_AW {
    #[doc = "0: No change"]
    VECTRESET_0 = 0,
    #[doc = "1: Causes a local system reset"]
    VECTRESET_1 = 1,
}
impl From<VECTRESET_AW> for bool {
    #[inline(always)]
    fn from(variant: VECTRESET_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `VECTRESET`"]
pub struct VECTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTRESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VECTRESET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn vectreset_0(self) -> &'a mut W {
        self.variant(VECTRESET_AW::VECTRESET_0)
    }
    #[doc = "Causes a local system reset"]
    #[inline(always)]
    pub fn vectreset_1(self) -> &'a mut W {
        self.variant(VECTRESET_AW::VECTRESET_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Writing 1 to this bit clears all active state information for fixed and configurable exceptions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VECTCLRACTIVE_AW {
    #[doc = "0: No change"]
    VECTCLRACTIVE_0 = 0,
    #[doc = "1: Clears all active state information for fixed and configurable exceptions"]
    VECTCLRACTIVE_1 = 1,
}
impl From<VECTCLRACTIVE_AW> for bool {
    #[inline(always)]
    fn from(variant: VECTCLRACTIVE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `VECTCLRACTIVE`"]
pub struct VECTCLRACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTCLRACTIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VECTCLRACTIVE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No change"]
    #[inline(always)]
    pub fn vectclractive_0(self) -> &'a mut W {
        self.variant(VECTCLRACTIVE_AW::VECTCLRACTIVE_0)
    }
    #[doc = "Clears all active state information for fixed and configurable exceptions"]
    #[inline(always)]
    pub fn vectclractive_1(self) -> &'a mut W {
        self.variant(VECTCLRACTIVE_AW::VECTCLRACTIVE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "System reset request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRESETREQ_AW {
    #[doc = "0: no system reset request"]
    SYSRESETREQ_0 = 0,
    #[doc = "1: asserts a signal to the outer system that requests a reset"]
    SYSRESETREQ_1 = 1,
}
impl From<SYSRESETREQ_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSRESETREQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SYSRESETREQ`"]
pub struct SYSRESETREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRESETREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRESETREQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no system reset request"]
    #[inline(always)]
    pub fn sysresetreq_0(self) -> &'a mut W {
        self.variant(SYSRESETREQ_AW::SYSRESETREQ_0)
    }
    #[doc = "asserts a signal to the outer system that requests a reset"]
    #[inline(always)]
    pub fn sysresetreq_1(self) -> &'a mut W {
        self.variant(SYSRESETREQ_AW::SYSRESETREQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PRIGROUP`"]
pub type PRIGROUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIGROUP`"]
pub struct PRIGROUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIGROUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Data endianness\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANNESS_A {
    #[doc = "0: Little-endian"]
    ENDIANNESS_0 = 0,
    #[doc = "1: Big-endian"]
    ENDIANNESS_1 = 1,
}
impl From<ENDIANNESS_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIANNESS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDIANNESS`"]
pub type ENDIANNESS_R = crate::R<bool, ENDIANNESS_A>;
impl ENDIANNESS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIANNESS_A {
        match self.bits {
            false => ENDIANNESS_A::ENDIANNESS_0,
            true => ENDIANNESS_A::ENDIANNESS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENDIANNESS_0`"]
    #[inline(always)]
    pub fn is_endianness_0(&self) -> bool {
        *self == ENDIANNESS_A::ENDIANNESS_0
    }
    #[doc = "Checks if the value of the field is `ENDIANNESS_1`"]
    #[inline(always)]
    pub fn is_endianness_1(&self) -> bool {
        *self == ENDIANNESS_A::ENDIANNESS_1
    }
}
#[doc = "Reader of field `VECTKEY`"]
pub type VECTKEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VECTKEY`"]
pub struct VECTKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> VECTKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
    #[inline(always)]
    pub fn prigroup(&self) -> PRIGROUP_R {
        PRIGROUP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Data endianness"]
    #[inline(always)]
    pub fn endianness(&self) -> ENDIANNESS_R {
        ENDIANNESS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Writing 1 to this bit causes a local system reset"]
    #[inline(always)]
    pub fn vectreset(&mut self) -> VECTRESET_W {
        VECTRESET_W { w: self }
    }
    #[doc = "Bit 1 - Writing 1 to this bit clears all active state information for fixed and configurable exceptions."]
    #[inline(always)]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W {
        VECTCLRACTIVE_W { w: self }
    }
    #[doc = "Bit 2 - System reset request"]
    #[inline(always)]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W {
        SYSRESETREQ_W { w: self }
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
    #[inline(always)]
    pub fn prigroup(&mut self) -> PRIGROUP_W {
        PRIGROUP_W { w: self }
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&mut self) -> VECTKEY_W {
        VECTKEY_W { w: self }
    }
}
