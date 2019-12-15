#[doc = "Reader of register SM0FRACVAL1"]
pub type R = crate::R<u16, super::SM0FRACVAL1>;
#[doc = "Writer for register SM0FRACVAL1"]
pub type W = crate::W<u16, super::SM0FRACVAL1>;
#[doc = "Register SM0FRACVAL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SM0FRACVAL1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRACVAL1`"]
pub type FRACVAL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRACVAL1`"]
pub struct FRACVAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACVAL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u16) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:15 - Fractional Value 1 Register"]
    #[inline(always)]
    pub fn fracval1(&self) -> FRACVAL1_R {
        FRACVAL1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 11:15 - Fractional Value 1 Register"]
    #[inline(always)]
    pub fn fracval1(&mut self) -> FRACVAL1_W {
        FRACVAL1_W { w: self }
    }
}
