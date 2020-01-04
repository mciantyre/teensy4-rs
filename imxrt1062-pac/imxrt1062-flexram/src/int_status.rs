#[doc = "Reader of register INT_STATUS"]
pub type R = crate::R<u32, super::INT_STATUS>;
#[doc = "Writer for register INT_STATUS"]
pub type W = crate::W<u32, super::INT_STATUS>;
#[doc = "Register INT_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ITCM Access Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITCM_ERR_STATUS_A {
    #[doc = "0: ITCM access error does not happen"]
    ITCM_ERR_STATUS_0 = 0,
    #[doc = "1: ITCM access error happens."]
    ITCM_ERR_STATUS_1 = 1,
}
impl From<ITCM_ERR_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ITCM_ERR_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ITCM_ERR_STATUS`"]
pub type ITCM_ERR_STATUS_R = crate::R<bool, ITCM_ERR_STATUS_A>;
impl ITCM_ERR_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITCM_ERR_STATUS_A {
        match self.bits {
            false => ITCM_ERR_STATUS_A::ITCM_ERR_STATUS_0,
            true => ITCM_ERR_STATUS_A::ITCM_ERR_STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ITCM_ERR_STATUS_0`"]
    #[inline(always)]
    pub fn is_itcm_err_status_0(&self) -> bool {
        *self == ITCM_ERR_STATUS_A::ITCM_ERR_STATUS_0
    }
    #[doc = "Checks if the value of the field is `ITCM_ERR_STATUS_1`"]
    #[inline(always)]
    pub fn is_itcm_err_status_1(&self) -> bool {
        *self == ITCM_ERR_STATUS_A::ITCM_ERR_STATUS_1
    }
}
#[doc = "Write proxy for field `ITCM_ERR_STATUS`"]
pub struct ITCM_ERR_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ITCM_ERR_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITCM_ERR_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ITCM access error does not happen"]
    #[inline(always)]
    pub fn itcm_err_status_0(self) -> &'a mut W {
        self.variant(ITCM_ERR_STATUS_A::ITCM_ERR_STATUS_0)
    }
    #[doc = "ITCM access error happens."]
    #[inline(always)]
    pub fn itcm_err_status_1(self) -> &'a mut W {
        self.variant(ITCM_ERR_STATUS_A::ITCM_ERR_STATUS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "DTCM Access Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCM_ERR_STATUS_A {
    #[doc = "0: DTCM access error does not happen"]
    DTCM_ERR_STATUS_0 = 0,
    #[doc = "1: DTCM access error happens."]
    DTCM_ERR_STATUS_1 = 1,
}
impl From<DTCM_ERR_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DTCM_ERR_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTCM_ERR_STATUS`"]
pub type DTCM_ERR_STATUS_R = crate::R<bool, DTCM_ERR_STATUS_A>;
impl DTCM_ERR_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTCM_ERR_STATUS_A {
        match self.bits {
            false => DTCM_ERR_STATUS_A::DTCM_ERR_STATUS_0,
            true => DTCM_ERR_STATUS_A::DTCM_ERR_STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTCM_ERR_STATUS_0`"]
    #[inline(always)]
    pub fn is_dtcm_err_status_0(&self) -> bool {
        *self == DTCM_ERR_STATUS_A::DTCM_ERR_STATUS_0
    }
    #[doc = "Checks if the value of the field is `DTCM_ERR_STATUS_1`"]
    #[inline(always)]
    pub fn is_dtcm_err_status_1(&self) -> bool {
        *self == DTCM_ERR_STATUS_A::DTCM_ERR_STATUS_1
    }
}
#[doc = "Write proxy for field `DTCM_ERR_STATUS`"]
pub struct DTCM_ERR_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM_ERR_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTCM_ERR_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DTCM access error does not happen"]
    #[inline(always)]
    pub fn dtcm_err_status_0(self) -> &'a mut W {
        self.variant(DTCM_ERR_STATUS_A::DTCM_ERR_STATUS_0)
    }
    #[doc = "DTCM access error happens."]
    #[inline(always)]
    pub fn dtcm_err_status_1(self) -> &'a mut W {
        self.variant(DTCM_ERR_STATUS_A::DTCM_ERR_STATUS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "OCRAM Access Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCRAM_ERR_STATUS_A {
    #[doc = "0: OCRAM access error does not happen"]
    OCRAM_ERR_STATUS_0 = 0,
    #[doc = "1: OCRAM access error happens."]
    OCRAM_ERR_STATUS_1 = 1,
}
impl From<OCRAM_ERR_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: OCRAM_ERR_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OCRAM_ERR_STATUS`"]
pub type OCRAM_ERR_STATUS_R = crate::R<bool, OCRAM_ERR_STATUS_A>;
impl OCRAM_ERR_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCRAM_ERR_STATUS_A {
        match self.bits {
            false => OCRAM_ERR_STATUS_A::OCRAM_ERR_STATUS_0,
            true => OCRAM_ERR_STATUS_A::OCRAM_ERR_STATUS_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM_ERR_STATUS_0`"]
    #[inline(always)]
    pub fn is_ocram_err_status_0(&self) -> bool {
        *self == OCRAM_ERR_STATUS_A::OCRAM_ERR_STATUS_0
    }
    #[doc = "Checks if the value of the field is `OCRAM_ERR_STATUS_1`"]
    #[inline(always)]
    pub fn is_ocram_err_status_1(&self) -> bool {
        *self == OCRAM_ERR_STATUS_A::OCRAM_ERR_STATUS_1
    }
}
#[doc = "Write proxy for field `OCRAM_ERR_STATUS`"]
pub struct OCRAM_ERR_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM_ERR_STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCRAM_ERR_STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OCRAM access error does not happen"]
    #[inline(always)]
    pub fn ocram_err_status_0(self) -> &'a mut W {
        self.variant(OCRAM_ERR_STATUS_A::OCRAM_ERR_STATUS_0)
    }
    #[doc = "OCRAM access error happens."]
    #[inline(always)]
    pub fn ocram_err_status_1(self) -> &'a mut W {
        self.variant(OCRAM_ERR_STATUS_A::OCRAM_ERR_STATUS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - ITCM Access Error Status"]
    #[inline(always)]
    pub fn itcm_err_status(&self) -> ITCM_ERR_STATUS_R {
        ITCM_ERR_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DTCM Access Error Status"]
    #[inline(always)]
    pub fn dtcm_err_status(&self) -> DTCM_ERR_STATUS_R {
        DTCM_ERR_STATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OCRAM Access Error Status"]
    #[inline(always)]
    pub fn ocram_err_status(&self) -> OCRAM_ERR_STATUS_R {
        OCRAM_ERR_STATUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ITCM Access Error Status"]
    #[inline(always)]
    pub fn itcm_err_status(&mut self) -> ITCM_ERR_STATUS_W {
        ITCM_ERR_STATUS_W { w: self }
    }
    #[doc = "Bit 4 - DTCM Access Error Status"]
    #[inline(always)]
    pub fn dtcm_err_status(&mut self) -> DTCM_ERR_STATUS_W {
        DTCM_ERR_STATUS_W { w: self }
    }
    #[doc = "Bit 5 - OCRAM Access Error Status"]
    #[inline(always)]
    pub fn ocram_err_status(&mut self) -> OCRAM_ERR_STATUS_W {
        OCRAM_ERR_STATUS_W { w: self }
    }
}
