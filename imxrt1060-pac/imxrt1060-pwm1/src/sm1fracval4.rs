#[doc = "Reader of register SM1FRACVAL4"]
pub type R = crate::R<u16, super::SM1FRACVAL4>;
#[doc = "Writer for register SM1FRACVAL4"]
pub type W = crate::W<u16, super::SM1FRACVAL4>;
#[doc = "Register SM1FRACVAL4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SM1FRACVAL4 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRACVAL4`"]
pub type FRACVAL4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRACVAL4`"]
pub struct FRACVAL4_W<'a> {
    w: &'a mut W,
}
impl<'a> FRACVAL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u16) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:15 - Fractional Value 4"]
    #[inline(always)]
    pub fn fracval4(&self) -> FRACVAL4_R {
        FRACVAL4_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 11:15 - Fractional Value 4"]
    #[inline(always)]
    pub fn fracval4(&mut self) -> FRACVAL4_W {
        FRACVAL4_W { w: self }
    }
}
