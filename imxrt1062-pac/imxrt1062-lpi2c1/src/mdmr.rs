#[doc = "Reader of register MDMR"]
pub type R = crate::R<u32, super::MDMR>;
#[doc = "Writer for register MDMR"]
pub type W = crate::W<u32, super::MDMR>;
#[doc = "Register MDMR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MATCH0`"]
pub type MATCH0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MATCH0`"]
pub struct MATCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `MATCH1`"]
pub type MATCH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MATCH1`"]
pub struct MATCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Match 0 Value"]
    #[inline(always)]
    pub fn match0(&self) -> MATCH0_R {
        MATCH0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Match 1 Value"]
    #[inline(always)]
    pub fn match1(&self) -> MATCH1_R {
        MATCH1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Match 0 Value"]
    #[inline(always)]
    pub fn match0(&mut self) -> MATCH0_W {
        MATCH0_W { w: self }
    }
    #[doc = "Bits 16:23 - Match 1 Value"]
    #[inline(always)]
    pub fn match1(&mut self) -> MATCH1_W {
        MATCH1_W { w: self }
    }
}
