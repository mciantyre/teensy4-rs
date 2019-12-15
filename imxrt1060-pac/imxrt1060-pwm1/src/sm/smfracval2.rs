#[doc = "Reader of register SMFRACVAL2"]
pub type R = crate::R<u16, super::SMFRACVAL2>;
#[doc = "Writer for register SMFRACVAL2"]
pub type W = crate::W<u16, super::SMFRACVAL2>;
#[doc = "Register SMFRACVAL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SMFRACVAL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRACVAL2`"]
pub type FRACVAL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRACVAL2`"]
pub struct FRACVAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACVAL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u16) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:15 - Fractional Value 2"]
    #[inline(always)]
    pub fn fracval2(&self) -> FRACVAL2_R {
        FRACVAL2_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 11:15 - Fractional Value 2"]
    #[inline(always)]
    pub fn fracval2(&mut self) -> FRACVAL2_W {
        FRACVAL2_W { w: self }
    }
}
