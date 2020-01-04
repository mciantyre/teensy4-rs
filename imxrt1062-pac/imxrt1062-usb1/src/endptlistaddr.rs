#[doc = "Reader of register ENDPTLISTADDR"]
pub type R = crate::R<u32, super::ENDPTLISTADDR>;
#[doc = "Writer for register ENDPTLISTADDR"]
pub type W = crate::W<u32, super::ENDPTLISTADDR>;
#[doc = "Register ENDPTLISTADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ENDPTLISTADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPBASE`"]
pub type EPBASE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EPBASE`"]
pub struct EPBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x001f_ffff << 11)) | (((value as u32) & 0x001f_ffff) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:31 - Endpoint List Pointer(Low)"]
    #[inline(always)]
    pub fn epbase(&self) -> EPBASE_R {
        EPBASE_R::new(((self.bits >> 11) & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 11:31 - Endpoint List Pointer(Low)"]
    #[inline(always)]
    pub fn epbase(&mut self) -> EPBASE_W {
        EPBASE_W { w: self }
    }
}
