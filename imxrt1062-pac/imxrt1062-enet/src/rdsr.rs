#[doc = "Reader of register RDSR"]
pub type R = crate::R<u32, super::RDSR>;
#[doc = "Writer for register RDSR"]
pub type W = crate::W<u32, super::RDSR>;
#[doc = "Register RDSR `reset()`'s with value 0"]
impl crate::ResetValue for super::RDSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `R_DES_START`"]
pub type R_DES_START_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `R_DES_START`"]
pub struct R_DES_START_W<'a> {
    w: &'a mut W,
}
impl<'a> R_DES_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - Pointer to the beginning of the receive buffer descriptor queue."]
    #[inline(always)]
    pub fn r_des_start(&self) -> R_DES_START_R {
        R_DES_START_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:31 - Pointer to the beginning of the receive buffer descriptor queue."]
    #[inline(always)]
    pub fn r_des_start(&mut self) -> R_DES_START_W {
        R_DES_START_W { w: self }
    }
}
