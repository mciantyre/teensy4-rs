#[doc = "Reader of register SMFRACVAL5"]
pub type R = crate::R<u16, super::SMFRACVAL5>;
#[doc = "Writer for register SMFRACVAL5"]
pub type W = crate::W<u16, super::SMFRACVAL5>;
#[doc = "Register SMFRACVAL5 `reset()`'s with value 0"]
impl crate::ResetValue for super::SMFRACVAL5 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRACVAL5`"]
pub type FRACVAL5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRACVAL5`"]
pub struct FRACVAL5_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACVAL5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u16) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:15 - Fractional Value 5"]
    #[inline(always)]
    pub fn fracval5(&self) -> FRACVAL5_R {
        FRACVAL5_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 11:15 - Fractional Value 5"]
    #[inline(always)]
    pub fn fracval5(&mut self) -> FRACVAL5_W {
        FRACVAL5_W { w: self }
    }
}
