#[doc = "Reader of register PRE_CHARGE_TIME"]
pub type R = crate::R<u32, super::PRE_CHARGE_TIME>;
#[doc = "Writer for register PRE_CHARGE_TIME"]
pub type W = crate::W<u32, super::PRE_CHARGE_TIME>;
#[doc = "Register PRE_CHARGE_TIME `reset()`'s with value 0"]
impl crate::ResetValue for super::PRE_CHARGE_TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRE_CHARGE_TIME`"]
pub type PRE_CHARGE_TIME_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PRE_CHARGE_TIME`"]
pub struct PRE_CHARGE_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_CHARGE_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Before detection, the top screen needs some time before being pulled up to a high voltage."]
    #[inline(always)]
    pub fn pre_charge_time(&self) -> PRE_CHARGE_TIME_R {
        PRE_CHARGE_TIME_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Before detection, the top screen needs some time before being pulled up to a high voltage."]
    #[inline(always)]
    pub fn pre_charge_time(&mut self) -> PRE_CHARGE_TIME_W {
        PRE_CHARGE_TIME_W { w: self }
    }
}
