#[doc = "Reader of register SM2FRACVAL3"]
pub type R = crate::R<u16, super::SM2FRACVAL3>;
#[doc = "Writer for register SM2FRACVAL3"]
pub type W = crate::W<u16, super::SM2FRACVAL3>;
#[doc = "Register SM2FRACVAL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SM2FRACVAL3 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRACVAL3`"]
pub type FRACVAL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRACVAL3`"]
pub struct FRACVAL3_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACVAL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u16) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:15 - Fractional Value 3"]
    #[inline(always)]
    pub fn fracval3(&self) -> FRACVAL3_R {
        FRACVAL3_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 11:15 - Fractional Value 3"]
    #[inline(always)]
    pub fn fracval3(&mut self) -> FRACVAL3_W {
        FRACVAL3_W { w: self }
    }
}
