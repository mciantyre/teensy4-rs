#[doc = "Reader of register FLSHCR1%s"]
pub type R = crate::R<u32, super::FLSHCR1>;
#[doc = "Writer for register FLSHCR1%s"]
pub type W = crate::W<u32, super::FLSHCR1>;
#[doc = "Register FLSHCR1%s `reset()`'s with value 0x63"]
impl crate::ResetValue for super::FLSHCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x63
    }
}
#[doc = "Reader of field `TCSS`"]
pub type TCSS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCSS`"]
pub struct TCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `TCSH`"]
pub type TCSH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCSH`"]
pub struct TCSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TCSH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `WA`"]
pub type WA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WA`"]
pub struct WA_W<'a> {
    w: &'a mut W,
}
impl<'a> WA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CAS`"]
pub type CAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAS`"]
pub struct CAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | (((value as u32) & 0x0f) << 11);
        self.w
    }
}
#[doc = "CS interval unit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSINTERVALUNIT_A {
    #[doc = "0: The CS interval unit is 1 serial clock cycle"]
    CSINTERVALUNIT_0 = 0,
    #[doc = "1: The CS interval unit is 256 serial clock cycle"]
    CSINTERVALUNIT_1 = 1,
}
impl From<CSINTERVALUNIT_A> for bool {
    #[inline(always)]
    fn from(variant: CSINTERVALUNIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSINTERVALUNIT`"]
pub type CSINTERVALUNIT_R = crate::R<bool, CSINTERVALUNIT_A>;
impl CSINTERVALUNIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSINTERVALUNIT_A {
        match self.bits {
            false => CSINTERVALUNIT_A::CSINTERVALUNIT_0,
            true => CSINTERVALUNIT_A::CSINTERVALUNIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CSINTERVALUNIT_0`"]
    #[inline(always)]
    pub fn is_csintervalunit_0(&self) -> bool {
        *self == CSINTERVALUNIT_A::CSINTERVALUNIT_0
    }
    #[doc = "Checks if the value of the field is `CSINTERVALUNIT_1`"]
    #[inline(always)]
    pub fn is_csintervalunit_1(&self) -> bool {
        *self == CSINTERVALUNIT_A::CSINTERVALUNIT_1
    }
}
#[doc = "Write proxy for field `CSINTERVALUNIT`"]
pub struct CSINTERVALUNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSINTERVALUNIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSINTERVALUNIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CS interval unit is 1 serial clock cycle"]
    #[inline(always)]
    pub fn csintervalunit_0(self) -> &'a mut W {
        self.variant(CSINTERVALUNIT_A::CSINTERVALUNIT_0)
    }
    #[doc = "The CS interval unit is 256 serial clock cycle"]
    #[inline(always)]
    pub fn csintervalunit_1(self) -> &'a mut W {
        self.variant(CSINTERVALUNIT_A::CSINTERVALUNIT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `CSINTERVAL`"]
pub type CSINTERVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CSINTERVAL`"]
pub struct CSINTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSINTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Serial Flash CS setup time."]
    #[inline(always)]
    pub fn tcss(&self) -> TCSS_R {
        TCSS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Serial Flash CS Hold time."]
    #[inline(always)]
    pub fn tcsh(&self) -> TCSH_R {
        TCSH_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - Word Addressable."]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:14 - Column Address Size."]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - CS interval unit"]
    #[inline(always)]
    pub fn csintervalunit(&self) -> CSINTERVALUNIT_R {
        CSINTERVALUNIT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    #[inline(always)]
    pub fn csinterval(&self) -> CSINTERVAL_R {
        CSINTERVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Serial Flash CS setup time."]
    #[inline(always)]
    pub fn tcss(&mut self) -> TCSS_W {
        TCSS_W { w: self }
    }
    #[doc = "Bits 5:9 - Serial Flash CS Hold time."]
    #[inline(always)]
    pub fn tcsh(&mut self) -> TCSH_W {
        TCSH_W { w: self }
    }
    #[doc = "Bit 10 - Word Addressable."]
    #[inline(always)]
    pub fn wa(&mut self) -> WA_W {
        WA_W { w: self }
    }
    #[doc = "Bits 11:14 - Column Address Size."]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W {
        CAS_W { w: self }
    }
    #[doc = "Bit 15 - CS interval unit"]
    #[inline(always)]
    pub fn csintervalunit(&mut self) -> CSINTERVALUNIT_W {
        CSINTERVALUNIT_W { w: self }
    }
    #[doc = "Bits 16:31 - This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    #[inline(always)]
    pub fn csinterval(&mut self) -> CSINTERVAL_W {
        CSINTERVAL_W { w: self }
    }
}
