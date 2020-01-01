#[doc = "Reader of register REV"]
pub type R = crate::R<u16, super::REV>;
#[doc = "Writer for register REV"]
pub type W = crate::W<u16, super::REV>;
#[doc = "Register REV `reset()`'s with value 0"]
impl crate::ResetValue for super::REV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REV`"]
pub type REV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `REV`"]
pub struct REV_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This read/write register contains the current value of the revolution counter."]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This read/write register contains the current value of the revolution counter."]
    #[inline(always)]
    pub fn rev(&mut self) -> REV_W {
        REV_W { w: self }
    }
}
