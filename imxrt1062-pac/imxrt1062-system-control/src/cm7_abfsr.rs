#[doc = "Reader of register CM7_ABFSR"]
pub type R = crate::R<u32, super::CM7_ABFSR>;
#[doc = "Writer for register CM7_ABFSR"]
pub type W = crate::W<u32, super::CM7_ABFSR>;
#[doc = "Register CM7_ABFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CM7_ABFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ITCM`"]
pub type ITCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITCM`"]
pub struct ITCM_W<'a> {
    w: &'a mut W,
}
impl<'a> ITCM_W<'a> {
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
#[doc = "Reader of field `DTCM`"]
pub type DTCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTCM`"]
pub struct DTCM_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM_W<'a> {
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
#[doc = "Reader of field `AHBP`"]
pub type AHBP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBP`"]
pub struct AHBP_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBP_W<'a> {
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
#[doc = "Reader of field `AXIM`"]
pub type AXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXIM`"]
pub struct AXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `EPPB`"]
pub type EPPB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPPB`"]
pub struct EPPB_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AXIMTYPE_A {
    #[doc = "0: OKAY."]
    AXIMTYPE_0 = 0,
    #[doc = "1: EXOKAY."]
    AXIMTYPE_1 = 1,
    #[doc = "2: SLVERR."]
    AXIMTYPE_2 = 2,
    #[doc = "3: DECERR."]
    AXIMTYPE_3 = 3,
}
impl From<AXIMTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: AXIMTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AXIMTYPE`"]
pub type AXIMTYPE_R = crate::R<u8, AXIMTYPE_A>;
impl AXIMTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXIMTYPE_A {
        match self.bits {
            0 => AXIMTYPE_A::AXIMTYPE_0,
            1 => AXIMTYPE_A::AXIMTYPE_1,
            2 => AXIMTYPE_A::AXIMTYPE_2,
            3 => AXIMTYPE_A::AXIMTYPE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AXIMTYPE_0`"]
    #[inline(always)]
    pub fn is_aximtype_0(&self) -> bool {
        *self == AXIMTYPE_A::AXIMTYPE_0
    }
    #[doc = "Checks if the value of the field is `AXIMTYPE_1`"]
    #[inline(always)]
    pub fn is_aximtype_1(&self) -> bool {
        *self == AXIMTYPE_A::AXIMTYPE_1
    }
    #[doc = "Checks if the value of the field is `AXIMTYPE_2`"]
    #[inline(always)]
    pub fn is_aximtype_2(&self) -> bool {
        *self == AXIMTYPE_A::AXIMTYPE_2
    }
    #[doc = "Checks if the value of the field is `AXIMTYPE_3`"]
    #[inline(always)]
    pub fn is_aximtype_3(&self) -> bool {
        *self == AXIMTYPE_A::AXIMTYPE_3
    }
}
#[doc = "Write proxy for field `AXIMTYPE`"]
pub struct AXIMTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIMTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXIMTYPE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "OKAY."]
    #[inline(always)]
    pub fn aximtype_0(self) -> &'a mut W {
        self.variant(AXIMTYPE_A::AXIMTYPE_0)
    }
    #[doc = "EXOKAY."]
    #[inline(always)]
    pub fn aximtype_1(self) -> &'a mut W {
        self.variant(AXIMTYPE_A::AXIMTYPE_1)
    }
    #[doc = "SLVERR."]
    #[inline(always)]
    pub fn aximtype_2(self) -> &'a mut W {
        self.variant(AXIMTYPE_A::AXIMTYPE_2)
    }
    #[doc = "DECERR."]
    #[inline(always)]
    pub fn aximtype_3(self) -> &'a mut W {
        self.variant(AXIMTYPE_A::AXIMTYPE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Asynchronous fault on ITCM interface."]
    #[inline(always)]
    pub fn itcm(&self) -> ITCM_R {
        ITCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Asynchronous fault on DTCM interface."]
    #[inline(always)]
    pub fn dtcm(&self) -> DTCM_R {
        DTCM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Asynchronous fault on AHBP interface."]
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Asynchronous fault on AXIM interface."]
    #[inline(always)]
    pub fn axim(&self) -> AXIM_R {
        AXIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Asynchronous fault on EPPB interface."]
    #[inline(always)]
    pub fn eppb(&self) -> EPPB_R {
        EPPB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1."]
    #[inline(always)]
    pub fn aximtype(&self) -> AXIMTYPE_R {
        AXIMTYPE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Asynchronous fault on ITCM interface."]
    #[inline(always)]
    pub fn itcm(&mut self) -> ITCM_W {
        ITCM_W { w: self }
    }
    #[doc = "Bit 1 - Asynchronous fault on DTCM interface."]
    #[inline(always)]
    pub fn dtcm(&mut self) -> DTCM_W {
        DTCM_W { w: self }
    }
    #[doc = "Bit 2 - Asynchronous fault on AHBP interface."]
    #[inline(always)]
    pub fn ahbp(&mut self) -> AHBP_W {
        AHBP_W { w: self }
    }
    #[doc = "Bit 3 - Asynchronous fault on AXIM interface."]
    #[inline(always)]
    pub fn axim(&mut self) -> AXIM_W {
        AXIM_W { w: self }
    }
    #[doc = "Bit 4 - Asynchronous fault on EPPB interface."]
    #[inline(always)]
    pub fn eppb(&mut self) -> EPPB_W {
        EPPB_W { w: self }
    }
    #[doc = "Bits 8:9 - Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1."]
    #[inline(always)]
    pub fn aximtype(&mut self) -> AXIMTYPE_W {
        AXIMTYPE_W { w: self }
    }
}
