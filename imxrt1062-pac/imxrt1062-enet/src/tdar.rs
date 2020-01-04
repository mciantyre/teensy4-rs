#[doc = "Reader of register TDAR"]
pub type R = crate::R<u32, super::TDAR>;
#[doc = "Writer for register TDAR"]
pub type W = crate::W<u32, super::TDAR>;
#[doc = "Register TDAR `reset()`'s with value 0"]
impl crate::ResetValue for super::TDAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDAR`"]
pub type TDAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TDAR`"]
pub struct TDAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TDAR_W<'a> {
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
    #[doc = "Bit 24 - Transmit Descriptor Active"]
    #[inline(always)]
    pub fn tdar(&self) -> TDAR_R {
        TDAR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Transmit Descriptor Active"]
    #[inline(always)]
    pub fn tdar(&mut self) -> TDAR_W {
        TDAR_W { w: self }
    }
}
