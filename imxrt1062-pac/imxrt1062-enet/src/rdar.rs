#[doc = "Reader of register RDAR"]
pub type R = crate::R<u32, super::RDAR>;
#[doc = "Writer for register RDAR"]
pub type W = crate::W<u32, super::RDAR>;
#[doc = "Register RDAR `reset()`'s with value 0"]
impl crate::ResetValue for super::RDAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDAR`"]
pub type RDAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDAR`"]
pub struct RDAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RDAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - Receive Descriptor Active"]
    #[inline(always)]
    pub fn rdar(&self) -> RDAR_R {
        RDAR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Receive Descriptor Active"]
    #[inline(always)]
    pub fn rdar(&mut self) -> RDAR_W {
        RDAR_W { w: self }
    }
}
