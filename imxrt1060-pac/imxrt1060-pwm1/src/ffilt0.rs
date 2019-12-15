#[doc = "Reader of register FFILT0"]
pub type R = crate::R<u16, super::FFILT0>;
#[doc = "Writer for register FFILT0"]
pub type W = crate::W<u16, super::FFILT0>;
#[doc = "Register FFILT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FFILT0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FILT_PER`"]
pub type FILT_PER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILT_PER`"]
pub struct FILT_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILT_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `FILT_CNT`"]
pub type FILT_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILT_CNT`"]
pub struct FILT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FILT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
#[doc = "Fault Glitch Stretch Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSTR_A {
    #[doc = "0: Fault input glitch stretching is disabled."]
    GSTR_0 = 0,
    #[doc = "1: Input fault signals will be stretched to at least 2 IPBus clock cycles."]
    GSTR_1 = 1,
}
impl From<GSTR_A> for bool {
    #[inline(always)]
    fn from(variant: GSTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GSTR`"]
pub type GSTR_R = crate::R<bool, GSTR_A>;
impl GSTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSTR_A {
        match self.bits {
            false => GSTR_A::GSTR_0,
            true => GSTR_A::GSTR_1,
        }
    }
    #[doc = "Checks if the value of the field is `GSTR_0`"]
    #[inline(always)]
    pub fn is_gstr_0(&self) -> bool {
        *self == GSTR_A::GSTR_0
    }
    #[doc = "Checks if the value of the field is `GSTR_1`"]
    #[inline(always)]
    pub fn is_gstr_1(&self) -> bool {
        *self == GSTR_A::GSTR_1
    }
}
#[doc = "Write proxy for field `GSTR`"]
pub struct GSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input glitch stretching is disabled."]
    #[inline(always)]
    pub fn gstr_0(self) -> &'a mut W {
        self.variant(GSTR_A::GSTR_0)
    }
    #[doc = "Input fault signals will be stretched to at least 2 IPBus clock cycles."]
    #[inline(always)]
    pub fn gstr_1(self) -> &'a mut W {
        self.variant(GSTR_A::GSTR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Fault Filter Period"]
    #[inline(always)]
    pub fn filt_per(&self) -> FILT_PER_R {
        FILT_PER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Fault Filter Count"]
    #[inline(always)]
    pub fn filt_cnt(&self) -> FILT_CNT_R {
        FILT_CNT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Fault Glitch Stretch Enable"]
    #[inline(always)]
    pub fn gstr(&self) -> GSTR_R {
        GSTR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Filter Period"]
    #[inline(always)]
    pub fn filt_per(&mut self) -> FILT_PER_W {
        FILT_PER_W { w: self }
    }
    #[doc = "Bits 8:10 - Fault Filter Count"]
    #[inline(always)]
    pub fn filt_cnt(&mut self) -> FILT_CNT_W {
        FILT_CNT_W { w: self }
    }
    #[doc = "Bit 15 - Fault Glitch Stretch Enable"]
    #[inline(always)]
    pub fn gstr(&mut self) -> GSTR_W {
        GSTR_W { w: self }
    }
}
