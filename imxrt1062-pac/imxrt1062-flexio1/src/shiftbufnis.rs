#[doc = "Reader of register SHIFTBUFNIS[%s]"]
pub type R = crate::R<u32, super::SHIFTBUFNIS>;
#[doc = "Writer for register SHIFTBUFNIS[%s]"]
pub type W = crate::W<u32, super::SHIFTBUFNIS>;
#[doc = "Register SHIFTBUFNIS[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTBUFNIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHIFTBUFNIS`"]
pub type SHIFTBUFNIS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SHIFTBUFNIS`"]
pub struct SHIFTBUFNIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTBUFNIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufnis(&self) -> SHIFTBUFNIS_R {
        SHIFTBUFNIS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufnis(&mut self) -> SHIFTBUFNIS_W {
        SHIFTBUFNIS_W { w: self }
    }
}
