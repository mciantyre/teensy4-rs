#[doc = "Reader of register CBT"]
pub type R = crate::R<u32, super::CBT>;
#[doc = "Writer for register CBT"]
pub type W = crate::W<u32, super::CBT>;
#[doc = "Register CBT `reset()`'s with value 0"]
impl crate::ResetValue for super::CBT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPSEG2`"]
pub type EPSEG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPSEG2`"]
pub struct EPSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `EPSEG1`"]
pub type EPSEG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPSEG1`"]
pub struct EPSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `EPROPSEG`"]
pub type EPROPSEG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPROPSEG`"]
pub struct EPROPSEG_W<'a> {
    w: &'a mut W,
}
impl<'a> EPROPSEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `ERJW`"]
pub type ERJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERJW`"]
pub struct ERJW_W<'a> {
    w: &'a mut W,
}
impl<'a> ERJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `EPRESDIV`"]
pub type EPRESDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EPRESDIV`"]
pub struct EPRESDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRESDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 21)) | (((value as u32) & 0x03ff) << 21);
        self.w
    }
}
#[doc = "Bit Timing Format Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTF_A {
    #[doc = "0: Extended bit time definitions disabled."]
    BTF_0 = 0,
    #[doc = "1: Extended bit time definitions enabled."]
    BTF_1 = 1,
}
impl From<BTF_A> for bool {
    #[inline(always)]
    fn from(variant: BTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTF`"]
pub type BTF_R = crate::R<bool, BTF_A>;
impl BTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTF_A {
        match self.bits {
            false => BTF_A::BTF_0,
            true => BTF_A::BTF_1,
        }
    }
    #[doc = "Checks if the value of the field is `BTF_0`"]
    #[inline(always)]
    pub fn is_btf_0(&self) -> bool {
        *self == BTF_A::BTF_0
    }
    #[doc = "Checks if the value of the field is `BTF_1`"]
    #[inline(always)]
    pub fn is_btf_1(&self) -> bool {
        *self == BTF_A::BTF_1
    }
}
#[doc = "Write proxy for field `BTF`"]
pub struct BTF_W<'a> {
    w: &'a mut W,
}
impl<'a> BTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Extended bit time definitions disabled."]
    #[inline(always)]
    pub fn btf_0(self) -> &'a mut W {
        self.variant(BTF_A::BTF_0)
    }
    #[doc = "Extended bit time definitions enabled."]
    #[inline(always)]
    pub fn btf_1(self) -> &'a mut W {
        self.variant(BTF_A::BTF_1)
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
    #[doc = "Bits 0:4 - Extended Phase Segment 2"]
    #[inline(always)]
    pub fn epseg2(&self) -> EPSEG2_R {
        EPSEG2_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Extended Phase Segment 1"]
    #[inline(always)]
    pub fn epseg1(&self) -> EPSEG1_R {
        EPSEG1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:15 - Extended Propagation Segment"]
    #[inline(always)]
    pub fn epropseg(&self) -> EPROPSEG_R {
        EPROPSEG_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Extended Resync Jump Width"]
    #[inline(always)]
    pub fn erjw(&self) -> ERJW_R {
        ERJW_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:30 - Extended Prescaler Division Factor"]
    #[inline(always)]
    pub fn epresdiv(&self) -> EPRESDIV_R {
        EPRESDIV_R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Bit Timing Format Enable"]
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Extended Phase Segment 2"]
    #[inline(always)]
    pub fn epseg2(&mut self) -> EPSEG2_W {
        EPSEG2_W { w: self }
    }
    #[doc = "Bits 5:9 - Extended Phase Segment 1"]
    #[inline(always)]
    pub fn epseg1(&mut self) -> EPSEG1_W {
        EPSEG1_W { w: self }
    }
    #[doc = "Bits 10:15 - Extended Propagation Segment"]
    #[inline(always)]
    pub fn epropseg(&mut self) -> EPROPSEG_W {
        EPROPSEG_W { w: self }
    }
    #[doc = "Bits 16:20 - Extended Resync Jump Width"]
    #[inline(always)]
    pub fn erjw(&mut self) -> ERJW_W {
        ERJW_W { w: self }
    }
    #[doc = "Bits 21:30 - Extended Prescaler Division Factor"]
    #[inline(always)]
    pub fn epresdiv(&mut self) -> EPRESDIV_W {
        EPRESDIV_W { w: self }
    }
    #[doc = "Bit 31 - Bit Timing Format Enable"]
    #[inline(always)]
    pub fn btf(&mut self) -> BTF_W {
        BTF_W { w: self }
    }
}
