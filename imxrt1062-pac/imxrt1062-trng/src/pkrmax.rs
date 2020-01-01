#[doc = "Reader of register PKRMAX"]
pub type R = crate::R<u32, super::PKRMAX>;
#[doc = "Writer for register PKRMAX"]
pub type W = crate::W<u32, super::PKRMAX>;
#[doc = "Register PKRMAX `reset()`'s with value 0x6920"]
impl crate::ResetValue for super::PKRMAX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6920
    }
}
#[doc = "Reader of field `PKR_MAX`"]
pub type PKR_MAX_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PKR_MAX`"]
pub struct PKR_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> PKR_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Poker Maximum Limit."]
    #[inline(always)]
    pub fn pkr_max(&self) -> PKR_MAX_R {
        PKR_MAX_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Poker Maximum Limit."]
    #[inline(always)]
    pub fn pkr_max(&mut self) -> PKR_MAX_W {
        PKR_MAX_W { w: self }
    }
}
