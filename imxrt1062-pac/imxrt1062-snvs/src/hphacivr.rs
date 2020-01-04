#[doc = "Reader of register HPHACIVR"]
pub type R = crate::R<u32, super::HPHACIVR>;
#[doc = "Writer for register HPHACIVR"]
pub type W = crate::W<u32, super::HPHACIVR>;
#[doc = "Register HPHACIVR `reset()`'s with value 0"]
impl crate::ResetValue for super::HPHACIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HAC_COUNTER_IV`"]
pub type HAC_COUNTER_IV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HAC_COUNTER_IV`"]
pub struct HAC_COUNTER_IV_W<'a> {
    w: &'a mut W,
}
impl<'a> HAC_COUNTER_IV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - High Assurance Counter Initial Value This register is used to set the starting count value to the high assurance counter"]
    #[inline(always)]
    pub fn hac_counter_iv(&self) -> HAC_COUNTER_IV_R {
        HAC_COUNTER_IV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - High Assurance Counter Initial Value This register is used to set the starting count value to the high assurance counter"]
    #[inline(always)]
    pub fn hac_counter_iv(&mut self) -> HAC_COUNTER_IV_W {
        HAC_COUNTER_IV_W { w: self }
    }
}
