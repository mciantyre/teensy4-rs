#[doc = "Reader of register CH2OPTS_CLR"]
pub type R = crate::R<u32, super::CH2OPTS_CLR>;
#[doc = "Writer for register CH2OPTS_CLR"]
pub type W = crate::W<u32, super::CH2OPTS_CLR>;
#[doc = "Register CH2OPTS_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH2OPTS_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RECOVERY_TIMER`"]
pub type RECOVERY_TIMER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RECOVERY_TIMER`"]
pub struct RECOVERY_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> RECOVERY_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub fn recovery_timer(&self) -> RECOVERY_TIMER_R {
        RECOVERY_TIMER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field indicates the recovery time for the channel"]
    #[inline(always)]
    pub fn recovery_timer(&mut self) -> RECOVERY_TIMER_W {
        RECOVERY_TIMER_W { w: self }
    }
}
