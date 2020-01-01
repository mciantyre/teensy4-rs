#[doc = "Reader of register CMPLD1%s"]
pub type R = crate::R<u16, super::CMPLD1>;
#[doc = "Writer for register CMPLD1%s"]
pub type W = crate::W<u16, super::CMPLD1>;
#[doc = "Register CMPLD1%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPLD1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPARATOR_LOAD_1`"]
pub type COMPARATOR_LOAD_1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMPARATOR_LOAD_1`"]
pub struct COMPARATOR_LOAD_1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARATOR_LOAD_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This read/write register is the comparator 1 preload value for the COMP1 register for the corresponding channel in a timer module"]
    #[inline(always)]
    pub fn comparator_load_1(&self) -> COMPARATOR_LOAD_1_R {
        COMPARATOR_LOAD_1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This read/write register is the comparator 1 preload value for the COMP1 register for the corresponding channel in a timer module"]
    #[inline(always)]
    pub fn comparator_load_1(&mut self) -> COMPARATOR_LOAD_1_W {
        COMPARATOR_LOAD_1_W { w: self }
    }
}
