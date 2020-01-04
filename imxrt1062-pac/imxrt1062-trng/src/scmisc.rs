#[doc = "Reader of register SCMISC"]
pub type R = crate::R<u32, super::SCMISC>;
#[doc = "Writer for register SCMISC"]
pub type W = crate::W<u32, super::SCMISC>;
#[doc = "Register SCMISC `reset()`'s with value 0x0001_0022"]
impl crate::ResetValue for super::SCMISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0022
    }
}
#[doc = "Reader of field `LRUN_MAX`"]
pub type LRUN_MAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LRUN_MAX`"]
pub struct LRUN_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> LRUN_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RTY_CT`"]
pub type RTY_CT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTY_CT`"]
pub struct RTY_CT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTY_CT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LONG RUN MAX LIMIT"]
    #[inline(always)]
    pub fn lrun_max(&self) -> LRUN_MAX_R {
        LRUN_MAX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - RETRY COUNT"]
    #[inline(always)]
    pub fn rty_ct(&self) -> RTY_CT_R {
        RTY_CT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LONG RUN MAX LIMIT"]
    #[inline(always)]
    pub fn lrun_max(&mut self) -> LRUN_MAX_W {
        LRUN_MAX_W { w: self }
    }
    #[doc = "Bits 16:19 - RETRY COUNT"]
    #[inline(always)]
    pub fn rty_ct(&mut self) -> RTY_CT_W {
        RTY_CT_W { w: self }
    }
}
