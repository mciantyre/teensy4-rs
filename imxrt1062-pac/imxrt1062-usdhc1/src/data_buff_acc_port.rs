#[doc = "Reader of register DATA_BUFF_ACC_PORT"]
pub type R = crate::R<u32, super::DATA_BUFF_ACC_PORT>;
#[doc = "Writer for register DATA_BUFF_ACC_PORT"]
pub type W = crate::W<u32, super::DATA_BUFF_ACC_PORT>;
#[doc = "Register DATA_BUFF_ACC_PORT `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA_BUFF_ACC_PORT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATCONT`"]
pub type DATCONT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATCONT`"]
pub struct DATCONT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATCONT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Content"]
    #[inline(always)]
    pub fn datcont(&self) -> DATCONT_R {
        DATCONT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Content"]
    #[inline(always)]
    pub fn datcont(&mut self) -> DATCONT_W {
        DATCONT_W { w: self }
    }
}
