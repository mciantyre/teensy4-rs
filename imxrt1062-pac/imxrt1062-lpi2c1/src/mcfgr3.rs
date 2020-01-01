#[doc = "Reader of register MCFGR3"]
pub type R = crate::R<u32, super::MCFGR3>;
#[doc = "Writer for register MCFGR3"]
pub type W = crate::W<u32, super::MCFGR3>;
#[doc = "Register MCFGR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::MCFGR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PINLOW`"]
pub type PINLOW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PINLOW`"]
pub struct PINLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> PINLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | (((value as u32) & 0x0fff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:19 - Pin Low Timeout"]
    #[inline(always)]
    pub fn pinlow(&self) -> PINLOW_R {
        PINLOW_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:19 - Pin Low Timeout"]
    #[inline(always)]
    pub fn pinlow(&mut self) -> PINLOW_W {
        PINLOW_W { w: self }
    }
}
