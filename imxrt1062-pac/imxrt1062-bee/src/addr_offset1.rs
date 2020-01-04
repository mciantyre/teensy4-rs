#[doc = "Reader of register ADDR_OFFSET1"]
pub type R = crate::R<u32, super::ADDR_OFFSET1>;
#[doc = "Writer for register ADDR_OFFSET1"]
pub type W = crate::W<u32, super::ADDR_OFFSET1>;
#[doc = "Register ADDR_OFFSET1 `reset()`'s with value 0xf000"]
impl crate::ResetValue for super::ADDR_OFFSET1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf000
    }
}
#[doc = "Reader of field `ADDR_OFFSET1`"]
pub type ADDR_OFFSET1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR_OFFSET1`"]
pub struct ADDR_OFFSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_OFFSET1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADDR_OFFSET1_LOCK`"]
pub type ADDR_OFFSET1_LOCK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR_OFFSET1_LOCK`"]
pub struct ADDR_OFFSET1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_OFFSET1_LOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Signed offset for BEE region 1"]
    #[inline(always)]
    pub fn addr_offset1(&self) -> ADDR_OFFSET1_R {
        ADDR_OFFSET1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Lock bits for addr_offset1"]
    #[inline(always)]
    pub fn addr_offset1_lock(&self) -> ADDR_OFFSET1_LOCK_R {
        ADDR_OFFSET1_LOCK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Signed offset for BEE region 1"]
    #[inline(always)]
    pub fn addr_offset1(&mut self) -> ADDR_OFFSET1_W {
        ADDR_OFFSET1_W { w: self }
    }
    #[doc = "Bits 16:31 - Lock bits for addr_offset1"]
    #[inline(always)]
    pub fn addr_offset1_lock(&mut self) -> ADDR_OFFSET1_LOCK_W {
        ADDR_OFFSET1_LOCK_W { w: self }
    }
}
