#[doc = "Reader of register CAL"]
pub type R = crate::R<u32, super::CAL>;
#[doc = "Writer for register CAL"]
pub type W = crate::W<u32, super::CAL>;
#[doc = "Register CAL `reset()`'s with value 0"]
impl crate::ResetValue for super::CAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAL_CODE`"]
pub type CAL_CODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAL_CODE`"]
pub struct CAL_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Calibration Result Value"]
    #[inline(always)]
    pub fn cal_code(&self) -> CAL_CODE_R {
        CAL_CODE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Calibration Result Value"]
    #[inline(always)]
    pub fn cal_code(&mut self) -> CAL_CODE_W {
        CAL_CODE_W { w: self }
    }
}
