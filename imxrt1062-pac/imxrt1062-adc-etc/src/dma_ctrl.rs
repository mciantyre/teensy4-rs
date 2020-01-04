#[doc = "Reader of register DMA_CTRL"]
pub type R = crate::R<u32, super::DMA_CTRL>;
#[doc = "Writer for register DMA_CTRL"]
pub type W = crate::W<u32, super::DMA_CTRL>;
#[doc = "Register DMA_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIG0_ENABLE`"]
pub type TRIG0_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG0_ENABLE`"]
pub struct TRIG0_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG0_ENABLE_W<'a> {
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
#[doc = "Reader of field `TRIG1_ENABLE`"]
pub type TRIG1_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG1_ENABLE`"]
pub struct TRIG1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG1_ENABLE_W<'a> {
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
#[doc = "Reader of field `TRIG2_ENABLE`"]
pub type TRIG2_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG2_ENABLE`"]
pub struct TRIG2_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG2_ENABLE_W<'a> {
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
#[doc = "Reader of field `TRIG3_ENABLE`"]
pub type TRIG3_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG3_ENABLE`"]
pub struct TRIG3_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG3_ENABLE_W<'a> {
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
#[doc = "Reader of field `TRIG4_ENABLE`"]
pub type TRIG4_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG4_ENABLE`"]
pub struct TRIG4_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG4_ENABLE_W<'a> {
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
#[doc = "Reader of field `TRIG5_ENABLE`"]
pub type TRIG5_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG5_ENABLE`"]
pub struct TRIG5_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG5_ENABLE_W<'a> {
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
#[doc = "Reader of field `TRIG6_ENABLE`"]
pub type TRIG6_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG6_ENABLE`"]
pub struct TRIG6_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG6_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TRIG7_ENABLE`"]
pub type TRIG7_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG7_ENABLE`"]
pub struct TRIG7_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG7_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TRIG0_REQ`"]
pub type TRIG0_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG0_REQ`"]
pub struct TRIG0_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG0_REQ_W<'a> {
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
#[doc = "Reader of field `TRIG1_REQ`"]
pub type TRIG1_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG1_REQ`"]
pub struct TRIG1_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG1_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TRIG2_REQ`"]
pub type TRIG2_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG2_REQ`"]
pub struct TRIG2_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG2_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TRIG3_REQ`"]
pub type TRIG3_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG3_REQ`"]
pub struct TRIG3_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG3_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `TRIG4_REQ`"]
pub type TRIG4_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG4_REQ`"]
pub struct TRIG4_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG4_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `TRIG5_REQ`"]
pub type TRIG5_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG5_REQ`"]
pub struct TRIG5_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG5_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TRIG6_REQ`"]
pub type TRIG6_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG6_REQ`"]
pub struct TRIG6_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG6_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `TRIG7_REQ`"]
pub type TRIG7_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG7_REQ`"]
pub struct TRIG7_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG7_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When TRIG0 done enable DMA request"]
    #[inline(always)]
    pub fn trig0_enable(&self) -> TRIG0_ENABLE_R {
        TRIG0_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When TRIG1 done enable DMA request"]
    #[inline(always)]
    pub fn trig1_enable(&self) -> TRIG1_ENABLE_R {
        TRIG1_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When TRIG2 done enable DMA request"]
    #[inline(always)]
    pub fn trig2_enable(&self) -> TRIG2_ENABLE_R {
        TRIG2_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When TRIG3 done enable DMA request"]
    #[inline(always)]
    pub fn trig3_enable(&self) -> TRIG3_ENABLE_R {
        TRIG3_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When TRIG4 done enable DMA request"]
    #[inline(always)]
    pub fn trig4_enable(&self) -> TRIG4_ENABLE_R {
        TRIG4_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When TRIG5 done enable DMA request"]
    #[inline(always)]
    pub fn trig5_enable(&self) -> TRIG5_ENABLE_R {
        TRIG5_ENABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When TRIG6 done enable DMA request"]
    #[inline(always)]
    pub fn trig6_enable(&self) -> TRIG6_ENABLE_R {
        TRIG6_ENABLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When TRIG7 done enable DMA request"]
    #[inline(always)]
    pub fn trig7_enable(&self) -> TRIG7_ENABLE_R {
        TRIG7_ENABLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - When TRIG0 done DMA request detection"]
    #[inline(always)]
    pub fn trig0_req(&self) -> TRIG0_REQ_R {
        TRIG0_REQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - When TRIG1 done DMA request detection"]
    #[inline(always)]
    pub fn trig1_req(&self) -> TRIG1_REQ_R {
        TRIG1_REQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - When TRIG2 done DMA request detection"]
    #[inline(always)]
    pub fn trig2_req(&self) -> TRIG2_REQ_R {
        TRIG2_REQ_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - When TRIG3 done DMA request detection"]
    #[inline(always)]
    pub fn trig3_req(&self) -> TRIG3_REQ_R {
        TRIG3_REQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - When TRIG4 done DMA request detection"]
    #[inline(always)]
    pub fn trig4_req(&self) -> TRIG4_REQ_R {
        TRIG4_REQ_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - When TRIG5 done DMA request detection"]
    #[inline(always)]
    pub fn trig5_req(&self) -> TRIG5_REQ_R {
        TRIG5_REQ_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - When TRIG6 done DMA request detection"]
    #[inline(always)]
    pub fn trig6_req(&self) -> TRIG6_REQ_R {
        TRIG6_REQ_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - When TRIG7 done DMA request detection"]
    #[inline(always)]
    pub fn trig7_req(&self) -> TRIG7_REQ_R {
        TRIG7_REQ_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When TRIG0 done enable DMA request"]
    #[inline(always)]
    pub fn trig0_enable(&mut self) -> TRIG0_ENABLE_W {
        TRIG0_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - When TRIG1 done enable DMA request"]
    #[inline(always)]
    pub fn trig1_enable(&mut self) -> TRIG1_ENABLE_W {
        TRIG1_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - When TRIG2 done enable DMA request"]
    #[inline(always)]
    pub fn trig2_enable(&mut self) -> TRIG2_ENABLE_W {
        TRIG2_ENABLE_W { w: self }
    }
    #[doc = "Bit 3 - When TRIG3 done enable DMA request"]
    #[inline(always)]
    pub fn trig3_enable(&mut self) -> TRIG3_ENABLE_W {
        TRIG3_ENABLE_W { w: self }
    }
    #[doc = "Bit 4 - When TRIG4 done enable DMA request"]
    #[inline(always)]
    pub fn trig4_enable(&mut self) -> TRIG4_ENABLE_W {
        TRIG4_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - When TRIG5 done enable DMA request"]
    #[inline(always)]
    pub fn trig5_enable(&mut self) -> TRIG5_ENABLE_W {
        TRIG5_ENABLE_W { w: self }
    }
    #[doc = "Bit 6 - When TRIG6 done enable DMA request"]
    #[inline(always)]
    pub fn trig6_enable(&mut self) -> TRIG6_ENABLE_W {
        TRIG6_ENABLE_W { w: self }
    }
    #[doc = "Bit 7 - When TRIG7 done enable DMA request"]
    #[inline(always)]
    pub fn trig7_enable(&mut self) -> TRIG7_ENABLE_W {
        TRIG7_ENABLE_W { w: self }
    }
    #[doc = "Bit 16 - When TRIG0 done DMA request detection"]
    #[inline(always)]
    pub fn trig0_req(&mut self) -> TRIG0_REQ_W {
        TRIG0_REQ_W { w: self }
    }
    #[doc = "Bit 17 - When TRIG1 done DMA request detection"]
    #[inline(always)]
    pub fn trig1_req(&mut self) -> TRIG1_REQ_W {
        TRIG1_REQ_W { w: self }
    }
    #[doc = "Bit 18 - When TRIG2 done DMA request detection"]
    #[inline(always)]
    pub fn trig2_req(&mut self) -> TRIG2_REQ_W {
        TRIG2_REQ_W { w: self }
    }
    #[doc = "Bit 19 - When TRIG3 done DMA request detection"]
    #[inline(always)]
    pub fn trig3_req(&mut self) -> TRIG3_REQ_W {
        TRIG3_REQ_W { w: self }
    }
    #[doc = "Bit 20 - When TRIG4 done DMA request detection"]
    #[inline(always)]
    pub fn trig4_req(&mut self) -> TRIG4_REQ_W {
        TRIG4_REQ_W { w: self }
    }
    #[doc = "Bit 21 - When TRIG5 done DMA request detection"]
    #[inline(always)]
    pub fn trig5_req(&mut self) -> TRIG5_REQ_W {
        TRIG5_REQ_W { w: self }
    }
    #[doc = "Bit 22 - When TRIG6 done DMA request detection"]
    #[inline(always)]
    pub fn trig6_req(&mut self) -> TRIG6_REQ_W {
        TRIG6_REQ_W { w: self }
    }
    #[doc = "Bit 23 - When TRIG7 done DMA request detection"]
    #[inline(always)]
    pub fn trig7_req(&mut self) -> TRIG7_REQ_W {
        TRIG7_REQ_W { w: self }
    }
}
