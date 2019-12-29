#[doc = "Reader of register GPR33"]
pub type R = crate::R<u32, super::GPR33>;
#[doc = "Writer for register GPR33"]
pub type W = crate::W<u32, super::GPR33>;
#[doc = "Register GPR33 `reset()`'s with value 0x07"]
impl crate::ResetValue for super::GPR33 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "OCRAM2 TrustZone (TZ) enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCRAM2_TZ_EN_A {
    #[doc = "0: The TrustZone feature is disabled. Entire OCRAM2 space is available for all access types (secure/non-secure/user/supervisor)."]
    OCRAM2_TZ_EN_0,
    #[doc = "1: The TrustZone feature is enabled. Access to address in the range specified by \\[ENDADDR:STARTADDR\\] follows the execution mode access policy described in CSU chapter."]
    OCRAM2_TZ_EN_1,
}
impl From<OCRAM2_TZ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OCRAM2_TZ_EN_A) -> Self {
        match variant {
            OCRAM2_TZ_EN_A::OCRAM2_TZ_EN_0 => false,
            OCRAM2_TZ_EN_A::OCRAM2_TZ_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `OCRAM2_TZ_EN`"]
pub type OCRAM2_TZ_EN_R = crate::R<bool, OCRAM2_TZ_EN_A>;
impl OCRAM2_TZ_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCRAM2_TZ_EN_A {
        match self.bits {
            false => OCRAM2_TZ_EN_A::OCRAM2_TZ_EN_0,
            true => OCRAM2_TZ_EN_A::OCRAM2_TZ_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM2_TZ_EN_0`"]
    #[inline(always)]
    pub fn is_ocram2_tz_en_0(&self) -> bool {
        *self == OCRAM2_TZ_EN_A::OCRAM2_TZ_EN_0
    }
    #[doc = "Checks if the value of the field is `OCRAM2_TZ_EN_1`"]
    #[inline(always)]
    pub fn is_ocram2_tz_en_1(&self) -> bool {
        *self == OCRAM2_TZ_EN_A::OCRAM2_TZ_EN_1
    }
}
#[doc = "Write proxy for field `OCRAM2_TZ_EN`"]
pub struct OCRAM2_TZ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM2_TZ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCRAM2_TZ_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The TrustZone feature is disabled. Entire OCRAM2 space is available for all access types (secure/non-secure/user/supervisor)."]
    #[inline(always)]
    pub fn ocram2_tz_en_0(self) -> &'a mut W {
        self.variant(OCRAM2_TZ_EN_A::OCRAM2_TZ_EN_0)
    }
    #[doc = "The TrustZone feature is enabled. Access to address in the range specified by \\[ENDADDR:STARTADDR\\] follows the execution mode access policy described in CSU chapter."]
    #[inline(always)]
    pub fn ocram2_tz_en_1(self) -> &'a mut W {
        self.variant(OCRAM2_TZ_EN_A::OCRAM2_TZ_EN_1)
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
#[doc = "Reader of field `OCRAM2_TZ_ADDR`"]
pub type OCRAM2_TZ_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OCRAM2_TZ_ADDR`"]
pub struct OCRAM2_TZ_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM2_TZ_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Lock OCRAM2_TZ_EN field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_OCRAM2_TZ_EN_A {
    #[doc = "0: Field is not locked"]
    LOCK_OCRAM2_TZ_EN_0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_OCRAM2_TZ_EN_1,
}
impl From<LOCK_OCRAM2_TZ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_OCRAM2_TZ_EN_A) -> Self {
        match variant {
            LOCK_OCRAM2_TZ_EN_A::LOCK_OCRAM2_TZ_EN_0 => false,
            LOCK_OCRAM2_TZ_EN_A::LOCK_OCRAM2_TZ_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCK_OCRAM2_TZ_EN`"]
pub type LOCK_OCRAM2_TZ_EN_R = crate::R<bool, LOCK_OCRAM2_TZ_EN_A>;
impl LOCK_OCRAM2_TZ_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_OCRAM2_TZ_EN_A {
        match self.bits {
            false => LOCK_OCRAM2_TZ_EN_A::LOCK_OCRAM2_TZ_EN_0,
            true => LOCK_OCRAM2_TZ_EN_A::LOCK_OCRAM2_TZ_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM2_TZ_EN_0`"]
    #[inline(always)]
    pub fn is_lock_ocram2_tz_en_0(&self) -> bool {
        *self == LOCK_OCRAM2_TZ_EN_A::LOCK_OCRAM2_TZ_EN_0
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM2_TZ_EN_1`"]
    #[inline(always)]
    pub fn is_lock_ocram2_tz_en_1(&self) -> bool {
        *self == LOCK_OCRAM2_TZ_EN_A::LOCK_OCRAM2_TZ_EN_1
    }
}
#[doc = "Write proxy for field `LOCK_OCRAM2_TZ_EN`"]
pub struct LOCK_OCRAM2_TZ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_OCRAM2_TZ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_OCRAM2_TZ_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_ocram2_tz_en_0(self) -> &'a mut W {
        self.variant(LOCK_OCRAM2_TZ_EN_A::LOCK_OCRAM2_TZ_EN_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_ocram2_tz_en_1(self) -> &'a mut W {
        self.variant(LOCK_OCRAM2_TZ_EN_A::LOCK_OCRAM2_TZ_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Lock OCRAM2_TZ_ADDR field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_OCRAM2_TZ_ADDR_A {
    #[doc = "0: Field is not locked"]
    LOCK_OCRAM2_TZ_ADDR_0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_OCRAM2_TZ_ADDR_1,
}
impl From<LOCK_OCRAM2_TZ_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_OCRAM2_TZ_ADDR_A) -> Self {
        match variant {
            LOCK_OCRAM2_TZ_ADDR_A::LOCK_OCRAM2_TZ_ADDR_0 => 0,
            LOCK_OCRAM2_TZ_ADDR_A::LOCK_OCRAM2_TZ_ADDR_1 => 1,
        }
    }
}
#[doc = "Reader of field `LOCK_OCRAM2_TZ_ADDR`"]
pub type LOCK_OCRAM2_TZ_ADDR_R = crate::R<u8, LOCK_OCRAM2_TZ_ADDR_A>;
impl LOCK_OCRAM2_TZ_ADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCK_OCRAM2_TZ_ADDR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOCK_OCRAM2_TZ_ADDR_A::LOCK_OCRAM2_TZ_ADDR_0),
            1 => Val(LOCK_OCRAM2_TZ_ADDR_A::LOCK_OCRAM2_TZ_ADDR_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM2_TZ_ADDR_0`"]
    #[inline(always)]
    pub fn is_lock_ocram2_tz_addr_0(&self) -> bool {
        *self == LOCK_OCRAM2_TZ_ADDR_A::LOCK_OCRAM2_TZ_ADDR_0
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM2_TZ_ADDR_1`"]
    #[inline(always)]
    pub fn is_lock_ocram2_tz_addr_1(&self) -> bool {
        *self == LOCK_OCRAM2_TZ_ADDR_A::LOCK_OCRAM2_TZ_ADDR_1
    }
}
#[doc = "Write proxy for field `LOCK_OCRAM2_TZ_ADDR`"]
pub struct LOCK_OCRAM2_TZ_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_OCRAM2_TZ_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_OCRAM2_TZ_ADDR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_ocram2_tz_addr_0(self) -> &'a mut W {
        self.variant(LOCK_OCRAM2_TZ_ADDR_A::LOCK_OCRAM2_TZ_ADDR_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_ocram2_tz_addr_1(self) -> &'a mut W {
        self.variant(LOCK_OCRAM2_TZ_ADDR_A::LOCK_OCRAM2_TZ_ADDR_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 17)) | (((value as u32) & 0x7f) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OCRAM2 TrustZone (TZ) enable."]
    #[inline(always)]
    pub fn ocram2_tz_en(&self) -> OCRAM2_TZ_EN_R {
        OCRAM2_TZ_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - OCRAM2 TrustZone (TZ) start address"]
    #[inline(always)]
    pub fn ocram2_tz_addr(&self) -> OCRAM2_TZ_ADDR_R {
        OCRAM2_TZ_ADDR_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Lock OCRAM2_TZ_EN field for changes"]
    #[inline(always)]
    pub fn lock_ocram2_tz_en(&self) -> LOCK_OCRAM2_TZ_EN_R {
        LOCK_OCRAM2_TZ_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:23 - Lock OCRAM2_TZ_ADDR field for changes"]
    #[inline(always)]
    pub fn lock_ocram2_tz_addr(&self) -> LOCK_OCRAM2_TZ_ADDR_R {
        LOCK_OCRAM2_TZ_ADDR_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OCRAM2 TrustZone (TZ) enable."]
    #[inline(always)]
    pub fn ocram2_tz_en(&mut self) -> OCRAM2_TZ_EN_W {
        OCRAM2_TZ_EN_W { w: self }
    }
    #[doc = "Bits 1:7 - OCRAM2 TrustZone (TZ) start address"]
    #[inline(always)]
    pub fn ocram2_tz_addr(&mut self) -> OCRAM2_TZ_ADDR_W {
        OCRAM2_TZ_ADDR_W { w: self }
    }
    #[doc = "Bit 16 - Lock OCRAM2_TZ_EN field for changes"]
    #[inline(always)]
    pub fn lock_ocram2_tz_en(&mut self) -> LOCK_OCRAM2_TZ_EN_W {
        LOCK_OCRAM2_TZ_EN_W { w: self }
    }
    #[doc = "Bits 17:23 - Lock OCRAM2_TZ_ADDR field for changes"]
    #[inline(always)]
    pub fn lock_ocram2_tz_addr(&mut self) -> LOCK_OCRAM2_TZ_ADDR_W {
        LOCK_OCRAM2_TZ_ADDR_W { w: self }
    }
}
