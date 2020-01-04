#[doc = "Reader of register SDCTL"]
pub type R = crate::R<u32, super::SDCTL>;
#[doc = "Writer for register SDCTL"]
pub type W = crate::W<u32, super::SDCTL>;
#[doc = "Register SDCTL `reset()`'s with value 0x0c80_09c4"]
impl crate::ResetValue for super::SDCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c80_09c4
    }
}
#[doc = "Reader of field `SAMP_SIZE`"]
pub type SAMP_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SAMP_SIZE`"]
pub struct SAMP_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMP_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ENT_DLY`"]
pub type ENT_DLY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ENT_DLY`"]
pub struct ENT_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> ENT_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Sample Size"]
    #[inline(always)]
    pub fn samp_size(&self) -> SAMP_SIZE_R {
        SAMP_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Entropy Delay"]
    #[inline(always)]
    pub fn ent_dly(&self) -> ENT_DLY_R {
        ENT_DLY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sample Size"]
    #[inline(always)]
    pub fn samp_size(&mut self) -> SAMP_SIZE_W {
        SAMP_SIZE_W { w: self }
    }
    #[doc = "Bits 16:31 - Entropy Delay"]
    #[inline(always)]
    pub fn ent_dly(&mut self) -> ENT_DLY_W {
        ENT_DLY_W { w: self }
    }
}
