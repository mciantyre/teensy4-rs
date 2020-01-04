#[doc = "Reader of register ETDC"]
pub type R = crate::R<u32, super::ETDC>;
#[doc = "Writer for register ETDC"]
pub type W = crate::W<u32, super::ETDC>;
#[doc = "Register ETDC `reset()`'s with value 0"]
impl crate::ResetValue for super::ETDC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ETDCVAL`"]
pub type ETDCVAL_R = crate::R<u8, u8>;
#[doc = "Reader of field `ETDCOFF`"]
pub type ETDCOFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETDCOFF`"]
pub struct ETDCOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ETDCOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Transceiver Delay Measurement Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMDIS_A {
    #[doc = "0: TDC measurement is enabled"]
    TDMDIS_0 = 0,
    #[doc = "1: TDC measurement is disabled"]
    TDMDIS_1 = 1,
}
impl From<TDMDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TDMDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDMDIS`"]
pub type TDMDIS_R = crate::R<bool, TDMDIS_A>;
impl TDMDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDMDIS_A {
        match self.bits {
            false => TDMDIS_A::TDMDIS_0,
            true => TDMDIS_A::TDMDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDMDIS_0`"]
    #[inline(always)]
    pub fn is_tdmdis_0(&self) -> bool {
        *self == TDMDIS_A::TDMDIS_0
    }
    #[doc = "Checks if the value of the field is `TDMDIS_1`"]
    #[inline(always)]
    pub fn is_tdmdis_1(&self) -> bool {
        *self == TDMDIS_A::TDMDIS_1
    }
}
#[doc = "Write proxy for field `TDMDIS`"]
pub struct TDMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TDMDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDMDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TDC measurement is enabled"]
    #[inline(always)]
    pub fn tdmdis_0(self) -> &'a mut W {
        self.variant(TDMDIS_A::TDMDIS_0)
    }
    #[doc = "TDC measurement is disabled"]
    #[inline(always)]
    pub fn tdmdis_1(self) -> &'a mut W {
        self.variant(TDMDIS_A::TDMDIS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Enhanced Transceiver Delay Compensation Value"]
    #[inline(always)]
    pub fn etdcval(&self) -> ETDCVAL_R {
        ETDCVAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - Enhanced Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn etdcoff(&self) -> ETDCOFF_R {
        ETDCOFF_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Transceiver Delay Measurement Disable"]
    #[inline(always)]
    pub fn tdmdis(&self) -> TDMDIS_R {
        TDMDIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:22 - Enhanced Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn etdcoff(&mut self) -> ETDCOFF_W {
        ETDCOFF_W { w: self }
    }
    #[doc = "Bit 31 - Transceiver Delay Measurement Disable"]
    #[inline(always)]
    pub fn tdmdis(&mut self) -> TDMDIS_W {
        TDMDIS_W { w: self }
    }
}
