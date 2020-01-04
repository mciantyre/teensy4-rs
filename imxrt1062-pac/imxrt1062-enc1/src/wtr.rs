#[doc = "Reader of register WTR"]
pub type R = crate::R<u16, super::WTR>;
#[doc = "Writer for register WTR"]
pub type W = crate::W<u16, super::WTR>;
#[doc = "Register WTR `reset()`'s with value 0"]
impl crate::ResetValue for super::WTR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDOG`"]
pub type WDOG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDOG`"]
pub struct WDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - WDOG\\[15:0\\]
is a binary representation of the number of clock cycles plus one that the watchdog timer counts before timing out and optionally generating an interrupt"]
    #[inline(always)]
    pub fn wdog(&self) -> WDOG_R {
        WDOG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDOG\\[15:0\\]
is a binary representation of the number of clock cycles plus one that the watchdog timer counts before timing out and optionally generating an interrupt"]
    #[inline(always)]
    pub fn wdog(&mut self) -> WDOG_W {
        WDOG_W { w: self }
    }
}
