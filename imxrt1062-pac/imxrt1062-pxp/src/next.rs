#[doc = "Reader of register NEXT"]
pub type R = crate::R<u32, super::NEXT>;
#[doc = "Writer for register NEXT"]
pub type W = crate::W<u32, super::NEXT>;
#[doc = "Register NEXT `reset()`'s with value 0"]
impl crate::ResetValue for super::NEXT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSVD`"]
pub type RSVD_R = crate::R<bool, bool>;
#[doc = "Reader of field `POINTER`"]
pub type POINTER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `POINTER`"]
pub struct POINTER_W<'a> {
    w: &'a mut W,
}
impl<'a> POINTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Indicates that the \"next frame\" functionality has been enabled"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reserved, always set to zero."]
    #[inline(always)]
    pub fn rsvd(&self) -> RSVD_R {
        RSVD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:31 - A pointer to a data structure containing register values to be used when processing the next frame"]
    #[inline(always)]
    pub fn pointer(&self) -> POINTER_R {
        POINTER_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - A pointer to a data structure containing register values to be used when processing the next frame"]
    #[inline(always)]
    pub fn pointer(&mut self) -> POINTER_W {
        POINTER_W { w: self }
    }
}
