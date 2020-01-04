#[doc = "Reader of register CMPLD2%s"]
pub type R = crate::R<u16, super::CMPLD2>;
#[doc = "Writer for register CMPLD2%s"]
pub type W = crate::W<u16, super::CMPLD2>;
#[doc = "Register CMPLD2%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPLD2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPARATOR_LOAD_2`"]
pub type COMPARATOR_LOAD_2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMPARATOR_LOAD_2`"]
pub struct COMPARATOR_LOAD_2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARATOR_LOAD_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This read/write register is the comparator 2 preload value for the COMP2 register for the corresponding channel in a timer module"]
    #[inline(always)]
    pub fn comparator_load_2(&self) -> COMPARATOR_LOAD_2_R {
        COMPARATOR_LOAD_2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This read/write register is the comparator 2 preload value for the COMP2 register for the corresponding channel in a timer module"]
    #[inline(always)]
    pub fn comparator_load_2(&mut self) -> COMPARATOR_LOAD_2_W {
        COMPARATOR_LOAD_2_W { w: self }
    }
}
