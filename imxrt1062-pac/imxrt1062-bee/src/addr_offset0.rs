#[doc = "Reader of register ADDR_OFFSET0"]
pub type R = crate::R<u32, super::ADDR_OFFSET0>;
#[doc = "Writer for register ADDR_OFFSET0"]
pub type W = crate::W<u32, super::ADDR_OFFSET0>;
#[doc = "Register ADDR_OFFSET0 `reset()`'s with value 0xf000"]
impl crate::ResetValue for super::ADDR_OFFSET0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf000
    }
}
#[doc = "Reader of field `ADDR_OFFSET0`"]
pub type ADDR_OFFSET0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR_OFFSET0`"]
pub struct ADDR_OFFSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_OFFSET0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADDR_OFFSET0_LOCK`"]
pub type ADDR_OFFSET0_LOCK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR_OFFSET0_LOCK`"]
pub struct ADDR_OFFSET0_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_OFFSET0_LOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Signed offset for BEE region 0"]
    #[inline(always)]
    pub fn addr_offset0(&self) -> ADDR_OFFSET0_R {
        ADDR_OFFSET0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Lock bits for addr_offset0"]
    #[inline(always)]
    pub fn addr_offset0_lock(&self) -> ADDR_OFFSET0_LOCK_R {
        ADDR_OFFSET0_LOCK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Signed offset for BEE region 0"]
    #[inline(always)]
    pub fn addr_offset0(&mut self) -> ADDR_OFFSET0_W {
        ADDR_OFFSET0_W { w: self }
    }
    #[doc = "Bits 16:31 - Lock bits for addr_offset0"]
    #[inline(always)]
    pub fn addr_offset0_lock(&mut self) -> ADDR_OFFSET0_LOCK_W {
        ADDR_OFFSET0_LOCK_W { w: self }
    }
}
