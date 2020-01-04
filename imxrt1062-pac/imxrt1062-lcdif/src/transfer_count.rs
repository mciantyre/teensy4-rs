#[doc = "Reader of register TRANSFER_COUNT"]
pub type R = crate::R<u32, super::TRANSFER_COUNT>;
#[doc = "Writer for register TRANSFER_COUNT"]
pub type W = crate::W<u32, super::TRANSFER_COUNT>;
#[doc = "Register TRANSFER_COUNT `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::TRANSFER_COUNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "Reader of field `H_COUNT`"]
pub type H_COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `H_COUNT`"]
pub struct H_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> H_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `V_COUNT`"]
pub type V_COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `V_COUNT`"]
pub struct V_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> V_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Total valid data (pixels) in each horizontal line"]
    #[inline(always)]
    pub fn h_count(&self) -> H_COUNT_R {
        H_COUNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of horizontal lines per frame which contain valid data"]
    #[inline(always)]
    pub fn v_count(&self) -> V_COUNT_R {
        V_COUNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Total valid data (pixels) in each horizontal line"]
    #[inline(always)]
    pub fn h_count(&mut self) -> H_COUNT_W {
        H_COUNT_W { w: self }
    }
    #[doc = "Bits 16:31 - Number of horizontal lines per frame which contain valid data"]
    #[inline(always)]
    pub fn v_count(&mut self) -> V_COUNT_W {
        V_COUNT_W { w: self }
    }
}
