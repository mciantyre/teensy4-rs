#[doc = "Reader of register DONE0_1_IRQ"]
pub type R = crate::R<u32, super::DONE0_1_IRQ>;
#[doc = "Writer for register DONE0_1_IRQ"]
pub type W = crate::W<u32, super::DONE0_1_IRQ>;
#[doc = "Register DONE0_1_IRQ `reset()`'s with value 0"]
impl crate::ResetValue for super::DONE0_1_IRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRIG0_DONE0`"]
pub type TRIG0_DONE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG0_DONE0`"]
pub struct TRIG0_DONE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG0_DONE0_W<'a> {
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
#[doc = "Reader of field `TRIG1_DONE0`"]
pub type TRIG1_DONE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG1_DONE0`"]
pub struct TRIG1_DONE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG1_DONE0_W<'a> {
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
#[doc = "Reader of field `TRIG2_DONE0`"]
pub type TRIG2_DONE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG2_DONE0`"]
pub struct TRIG2_DONE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG2_DONE0_W<'a> {
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
#[doc = "Reader of field `TRIG3_DONE0`"]
pub type TRIG3_DONE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG3_DONE0`"]
pub struct TRIG3_DONE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG3_DONE0_W<'a> {
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
#[doc = "Reader of field `TRIG4_DONE0`"]
pub type TRIG4_DONE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG4_DONE0`"]
pub struct TRIG4_DONE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG4_DONE0_W<'a> {
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
#[doc = "Reader of field `TRIG5_DONE0`"]
pub type TRIG5_DONE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG5_DONE0`"]
pub struct TRIG5_DONE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG5_DONE0_W<'a> {
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
#[doc = "Reader of field `TRIG6_DONE0`"]
pub type TRIG6_DONE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG6_DONE0`"]
pub struct TRIG6_DONE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG6_DONE0_W<'a> {
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
#[doc = "Reader of field `TRIG7_DONE0`"]
pub type TRIG7_DONE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG7_DONE0`"]
pub struct TRIG7_DONE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG7_DONE0_W<'a> {
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
#[doc = "Reader of field `TRIG0_DONE1`"]
pub type TRIG0_DONE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG0_DONE1`"]
pub struct TRIG0_DONE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG0_DONE1_W<'a> {
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
#[doc = "Reader of field `TRIG1_DONE1`"]
pub type TRIG1_DONE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG1_DONE1`"]
pub struct TRIG1_DONE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG1_DONE1_W<'a> {
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
#[doc = "Reader of field `TRIG2_DONE1`"]
pub type TRIG2_DONE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG2_DONE1`"]
pub struct TRIG2_DONE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG2_DONE1_W<'a> {
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
#[doc = "Reader of field `TRIG3_DONE1`"]
pub type TRIG3_DONE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG3_DONE1`"]
pub struct TRIG3_DONE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG3_DONE1_W<'a> {
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
#[doc = "Reader of field `TRIG4_DONE1`"]
pub type TRIG4_DONE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG4_DONE1`"]
pub struct TRIG4_DONE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG4_DONE1_W<'a> {
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
#[doc = "Reader of field `TRIG5_DONE1`"]
pub type TRIG5_DONE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG5_DONE1`"]
pub struct TRIG5_DONE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG5_DONE1_W<'a> {
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
#[doc = "Reader of field `TRIG6_DONE1`"]
pub type TRIG6_DONE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG6_DONE1`"]
pub struct TRIG6_DONE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG6_DONE1_W<'a> {
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
#[doc = "Reader of field `TRIG7_DONE1`"]
pub type TRIG7_DONE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIG7_DONE1`"]
pub struct TRIG7_DONE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG7_DONE1_W<'a> {
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
    #[doc = "Bit 0 - TRIG0 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig0_done0(&self) -> TRIG0_DONE0_R {
        TRIG0_DONE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TRIG1 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig1_done0(&self) -> TRIG1_DONE0_R {
        TRIG1_DONE0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TRIG2 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig2_done0(&self) -> TRIG2_DONE0_R {
        TRIG2_DONE0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TRIG3 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig3_done0(&self) -> TRIG3_DONE0_R {
        TRIG3_DONE0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TRIG4 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig4_done0(&self) -> TRIG4_DONE0_R {
        TRIG4_DONE0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TRIG5 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig5_done0(&self) -> TRIG5_DONE0_R {
        TRIG5_DONE0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TRIG6 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig6_done0(&self) -> TRIG6_DONE0_R {
        TRIG6_DONE0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TRIG7 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig7_done0(&self) -> TRIG7_DONE0_R {
        TRIG7_DONE0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TRIG0 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig0_done1(&self) -> TRIG0_DONE1_R {
        TRIG0_DONE1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TRIG1 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig1_done1(&self) -> TRIG1_DONE1_R {
        TRIG1_DONE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TRIG2 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig2_done1(&self) -> TRIG2_DONE1_R {
        TRIG2_DONE1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TRIG3 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig3_done1(&self) -> TRIG3_DONE1_R {
        TRIG3_DONE1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TRIG4 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig4_done1(&self) -> TRIG4_DONE1_R {
        TRIG4_DONE1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TRIG5 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig5_done1(&self) -> TRIG5_DONE1_R {
        TRIG5_DONE1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TRIG6 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig6_done1(&self) -> TRIG6_DONE1_R {
        TRIG6_DONE1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TRIG7 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig7_done1(&self) -> TRIG7_DONE1_R {
        TRIG7_DONE1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRIG0 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig0_done0(&mut self) -> TRIG0_DONE0_W {
        TRIG0_DONE0_W { w: self }
    }
    #[doc = "Bit 1 - TRIG1 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig1_done0(&mut self) -> TRIG1_DONE0_W {
        TRIG1_DONE0_W { w: self }
    }
    #[doc = "Bit 2 - TRIG2 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig2_done0(&mut self) -> TRIG2_DONE0_W {
        TRIG2_DONE0_W { w: self }
    }
    #[doc = "Bit 3 - TRIG3 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig3_done0(&mut self) -> TRIG3_DONE0_W {
        TRIG3_DONE0_W { w: self }
    }
    #[doc = "Bit 4 - TRIG4 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig4_done0(&mut self) -> TRIG4_DONE0_W {
        TRIG4_DONE0_W { w: self }
    }
    #[doc = "Bit 5 - TRIG5 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig5_done0(&mut self) -> TRIG5_DONE0_W {
        TRIG5_DONE0_W { w: self }
    }
    #[doc = "Bit 6 - TRIG6 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig6_done0(&mut self) -> TRIG6_DONE0_W {
        TRIG6_DONE0_W { w: self }
    }
    #[doc = "Bit 7 - TRIG7 done0 interrupt detection"]
    #[inline(always)]
    pub fn trig7_done0(&mut self) -> TRIG7_DONE0_W {
        TRIG7_DONE0_W { w: self }
    }
    #[doc = "Bit 16 - TRIG0 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig0_done1(&mut self) -> TRIG0_DONE1_W {
        TRIG0_DONE1_W { w: self }
    }
    #[doc = "Bit 17 - TRIG1 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig1_done1(&mut self) -> TRIG1_DONE1_W {
        TRIG1_DONE1_W { w: self }
    }
    #[doc = "Bit 18 - TRIG2 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig2_done1(&mut self) -> TRIG2_DONE1_W {
        TRIG2_DONE1_W { w: self }
    }
    #[doc = "Bit 19 - TRIG3 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig3_done1(&mut self) -> TRIG3_DONE1_W {
        TRIG3_DONE1_W { w: self }
    }
    #[doc = "Bit 20 - TRIG4 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig4_done1(&mut self) -> TRIG4_DONE1_W {
        TRIG4_DONE1_W { w: self }
    }
    #[doc = "Bit 21 - TRIG5 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig5_done1(&mut self) -> TRIG5_DONE1_W {
        TRIG5_DONE1_W { w: self }
    }
    #[doc = "Bit 22 - TRIG6 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig6_done1(&mut self) -> TRIG6_DONE1_W {
        TRIG6_DONE1_W { w: self }
    }
    #[doc = "Bit 23 - TRIG7 done1 interrupt detection"]
    #[inline(always)]
    pub fn trig7_done1(&mut self) -> TRIG7_DONE1_W {
        TRIG7_DONE1_W { w: self }
    }
}
