#[doc = "Reader of register ATINC"]
pub type R = crate::R<u32, super::ATINC>;
#[doc = "Writer for register ATINC"]
pub type W = crate::W<u32, super::ATINC>;
#[doc = "Register ATINC `reset()`'s with value 0"]
impl crate::ResetValue for super::ATINC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INC`"]
pub type INC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INC`"]
pub struct INC_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `INC_CORR`"]
pub type INC_CORR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INC_CORR`"]
pub struct INC_CORR_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_CORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    #[inline(always)]
    pub fn inc(&self) -> INC_R {
        INC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Correction Increment Value"]
    #[inline(always)]
    pub fn inc_corr(&self) -> INC_CORR_R {
        INC_CORR_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Clock Period Of The Timestamping Clock (ts_clk) In Nanoseconds"]
    #[inline(always)]
    pub fn inc(&mut self) -> INC_W {
        INC_W { w: self }
    }
    #[doc = "Bits 8:14 - Correction Increment Value"]
    #[inline(always)]
    pub fn inc_corr(&mut self) -> INC_CORR_W {
        INC_CORR_W { w: self }
    }
}
