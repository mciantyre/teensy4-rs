#[doc = "Reader of register CH1SEMA"]
pub type R = crate::R<u32, super::CH1SEMA>;
#[doc = "Writer for register CH1SEMA"]
pub type W = crate::W<u32, super::CH1SEMA>;
#[doc = "Register CH1SEMA `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1SEMA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INCREMENT`"]
pub type INCREMENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INCREMENT`"]
pub struct INCREMENT_W<'a> {
    w: &'a mut W,
}
impl<'a> INCREMENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and the DCP hardware substracts happening on the same clock are protected"]
    #[inline(always)]
    pub fn increment(&self) -> INCREMENT_R {
        INCREMENT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This read-only field shows the current (instantaneous) value of the semaphore counter."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The value written to this field is added to the semaphore count in an atomic way, such that the simultaneous software adds and the DCP hardware substracts happening on the same clock are protected"]
    #[inline(always)]
    pub fn increment(&mut self) -> INCREMENT_W {
        INCREMENT_W { w: self }
    }
}
