#[doc = "Reader of register EPRS"]
pub type R = crate::R<u32, super::EPRS>;
#[doc = "Writer for register EPRS"]
pub type W = crate::W<u32, super::EPRS>;
#[doc = "Register EPRS `reset()`'s with value 0"]
impl crate::ResetValue for super::EPRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENPRESDIV`"]
pub type ENPRESDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ENPRESDIV`"]
pub struct ENPRESDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ENPRESDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `EDPRESDIV`"]
pub type EDPRESDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EDPRESDIV`"]
pub struct EDPRESDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> EDPRESDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Extended Nominal Prescaler Division Factor"]
    #[inline(always)]
    pub fn enpresdiv(&self) -> ENPRESDIV_R {
        ENPRESDIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Extended Data Phase Prescaler Division Factor"]
    #[inline(always)]
    pub fn edpresdiv(&self) -> EDPRESDIV_R {
        EDPRESDIV_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Extended Nominal Prescaler Division Factor"]
    #[inline(always)]
    pub fn enpresdiv(&mut self) -> ENPRESDIV_W {
        ENPRESDIV_W { w: self }
    }
    #[doc = "Bits 16:25 - Extended Data Phase Prescaler Division Factor"]
    #[inline(always)]
    pub fn edpresdiv(&mut self) -> EDPRESDIV_W {
        EDPRESDIV_W { w: self }
    }
}
