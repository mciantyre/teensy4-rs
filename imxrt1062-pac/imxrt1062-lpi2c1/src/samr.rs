#[doc = "Reader of register SAMR"]
pub type R = crate::R<u32, super::SAMR>;
#[doc = "Writer for register SAMR"]
pub type W = crate::W<u32, super::SAMR>;
#[doc = "Register SAMR `reset()`'s with value 0"]
impl crate::ResetValue for super::SAMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR0`"]
pub type ADDR0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR0`"]
pub struct ADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 1)) | (((value as u32) & 0x03ff) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADDR1`"]
pub type ADDR1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR1`"]
pub struct ADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 17)) | (((value as u32) & 0x03ff) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:10 - Address 0 Value"]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 17:26 - Address 1 Value"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new(((self.bits >> 17) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Address 0 Value"]
    #[inline(always)]
    pub fn addr0(&mut self) -> ADDR0_W {
        ADDR0_W { w: self }
    }
    #[doc = "Bits 17:26 - Address 1 Value"]
    #[inline(always)]
    pub fn addr1(&mut self) -> ADDR1_W {
        ADDR1_W { w: self }
    }
}
