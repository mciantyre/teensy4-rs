#[doc = "Reader of register FPR"]
pub type R = crate::R<u8, super::FPR>;
#[doc = "Writer for register FPR"]
pub type W = crate::W<u8, super::FPR>;
#[doc = "Register FPR `reset()`'s with value 0"]
impl crate::ResetValue for super::FPR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FILT_PER`"]
pub type FILT_PER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILT_PER`"]
pub struct FILT_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILT_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Filter Sample Period"]
    #[inline(always)]
    pub fn filt_per(&self) -> FILT_PER_R {
        FILT_PER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter Sample Period"]
    #[inline(always)]
    pub fn filt_per(&mut self) -> FILT_PER_W {
        FILT_PER_W { w: self }
    }
}
