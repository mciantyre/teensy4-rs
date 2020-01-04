#[doc = "Reader of register READ_CTRL"]
pub type R = crate::R<u32, super::READ_CTRL>;
#[doc = "Writer for register READ_CTRL"]
pub type W = crate::W<u32, super::READ_CTRL>;
#[doc = "Register READ_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::READ_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READ_FUSE`"]
pub type READ_FUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READ_FUSE`"]
pub struct READ_FUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_FUSE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bit 0 - READ_FUSE"]
    #[inline(always)]
    pub fn read_fuse(&self) -> READ_FUSE_R {
        READ_FUSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:31 - RSVD0"]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - READ_FUSE"]
    #[inline(always)]
    pub fn read_fuse(&mut self) -> READ_FUSE_W {
        READ_FUSE_W { w: self }
    }
}
