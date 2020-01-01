#[doc = "Reader of register CM7_CACR"]
pub type R = crate::R<u32, super::CM7_CACR>;
#[doc = "Writer for register CM7_CACR"]
pub type W = crate::W<u32, super::CM7_CACR>;
#[doc = "Register CM7_CACR `reset()`'s with value 0"]
impl crate::ResetValue for super::CM7_CACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shared cacheable-is-WT for data cache. Enables limited cache coherency usage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIWT_A {
    #[doc = "0: Normal Cacheable Shared locations are treated as being Non-cacheable. Default mode of operation for Shared memory."]
    SIWT_0 = 0,
    #[doc = "1: Normal Cacheable shared locations are treated as Write-Through."]
    SIWT_1 = 1,
}
impl From<SIWT_A> for bool {
    #[inline(always)]
    fn from(variant: SIWT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SIWT`"]
pub type SIWT_R = crate::R<bool, SIWT_A>;
impl SIWT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIWT_A {
        match self.bits {
            false => SIWT_A::SIWT_0,
            true => SIWT_A::SIWT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SIWT_0`"]
    #[inline(always)]
    pub fn is_siwt_0(&self) -> bool {
        *self == SIWT_A::SIWT_0
    }
    #[doc = "Checks if the value of the field is `SIWT_1`"]
    #[inline(always)]
    pub fn is_siwt_1(&self) -> bool {
        *self == SIWT_A::SIWT_1
    }
}
#[doc = "Write proxy for field `SIWT`"]
pub struct SIWT_W<'a> {
    w: &'a mut W,
}
impl<'a> SIWT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIWT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Cacheable Shared locations are treated as being Non-cacheable. Default mode of operation for Shared memory."]
    #[inline(always)]
    pub fn siwt_0(self) -> &'a mut W {
        self.variant(SIWT_A::SIWT_0)
    }
    #[doc = "Normal Cacheable shared locations are treated as Write-Through."]
    #[inline(always)]
    pub fn siwt_1(self) -> &'a mut W {
        self.variant(SIWT_A::SIWT_1)
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
#[doc = "Enables ECC in the instruction and data cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCDIS_A {
    #[doc = "0: Enables ECC in the instruction and data cache."]
    ECCDIS_0 = 0,
    #[doc = "1: Disables ECC in the instruction and data cache."]
    ECCDIS_1 = 1,
}
impl From<ECCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ECCDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECCDIS`"]
pub type ECCDIS_R = crate::R<bool, ECCDIS_A>;
impl ECCDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECCDIS_A {
        match self.bits {
            false => ECCDIS_A::ECCDIS_0,
            true => ECCDIS_A::ECCDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECCDIS_0`"]
    #[inline(always)]
    pub fn is_eccdis_0(&self) -> bool {
        *self == ECCDIS_A::ECCDIS_0
    }
    #[doc = "Checks if the value of the field is `ECCDIS_1`"]
    #[inline(always)]
    pub fn is_eccdis_1(&self) -> bool {
        *self == ECCDIS_A::ECCDIS_1
    }
}
#[doc = "Write proxy for field `ECCDIS`"]
pub struct ECCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECCDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enables ECC in the instruction and data cache."]
    #[inline(always)]
    pub fn eccdis_0(self) -> &'a mut W {
        self.variant(ECCDIS_A::ECCDIS_0)
    }
    #[doc = "Disables ECC in the instruction and data cache."]
    #[inline(always)]
    pub fn eccdis_1(self) -> &'a mut W {
        self.variant(ECCDIS_A::ECCDIS_1)
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
#[doc = "Enables Force Write-Through in the data cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEWT_A {
    #[doc = "0: Disables Force Write-Through."]
    FORCEWT_0 = 0,
    #[doc = "1: Enables Force Write-Through. All Cacheable memory regions are treated as Write-Through."]
    FORCEWT_1 = 1,
}
impl From<FORCEWT_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEWT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FORCEWT`"]
pub type FORCEWT_R = crate::R<bool, FORCEWT_A>;
impl FORCEWT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEWT_A {
        match self.bits {
            false => FORCEWT_A::FORCEWT_0,
            true => FORCEWT_A::FORCEWT_1,
        }
    }
    #[doc = "Checks if the value of the field is `FORCEWT_0`"]
    #[inline(always)]
    pub fn is_forcewt_0(&self) -> bool {
        *self == FORCEWT_A::FORCEWT_0
    }
    #[doc = "Checks if the value of the field is `FORCEWT_1`"]
    #[inline(always)]
    pub fn is_forcewt_1(&self) -> bool {
        *self == FORCEWT_A::FORCEWT_1
    }
}
#[doc = "Write proxy for field `FORCEWT`"]
pub struct FORCEWT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEWT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEWT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables Force Write-Through."]
    #[inline(always)]
    pub fn forcewt_0(self) -> &'a mut W {
        self.variant(FORCEWT_A::FORCEWT_0)
    }
    #[doc = "Enables Force Write-Through. All Cacheable memory regions are treated as Write-Through."]
    #[inline(always)]
    pub fn forcewt_1(self) -> &'a mut W {
        self.variant(FORCEWT_A::FORCEWT_1)
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
impl R {
    #[doc = "Bit 0 - Shared cacheable-is-WT for data cache. Enables limited cache coherency usage."]
    #[inline(always)]
    pub fn siwt(&self) -> SIWT_R {
        SIWT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables ECC in the instruction and data cache."]
    #[inline(always)]
    pub fn eccdis(&self) -> ECCDIS_R {
        ECCDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables Force Write-Through in the data cache."]
    #[inline(always)]
    pub fn forcewt(&self) -> FORCEWT_R {
        FORCEWT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shared cacheable-is-WT for data cache. Enables limited cache coherency usage."]
    #[inline(always)]
    pub fn siwt(&mut self) -> SIWT_W {
        SIWT_W { w: self }
    }
    #[doc = "Bit 1 - Enables ECC in the instruction and data cache."]
    #[inline(always)]
    pub fn eccdis(&mut self) -> ECCDIS_W {
        ECCDIS_W { w: self }
    }
    #[doc = "Bit 2 - Enables Force Write-Through in the data cache."]
    #[inline(always)]
    pub fn forcewt(&mut self) -> FORCEWT_W {
        FORCEWT_W { w: self }
    }
}
