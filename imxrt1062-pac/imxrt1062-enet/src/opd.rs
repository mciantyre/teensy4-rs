#[doc = "Reader of register OPD"]
pub type R = crate::R<u32, super::OPD>;
#[doc = "Writer for register OPD"]
pub type W = crate::W<u32, super::OPD>;
#[doc = "Register OPD `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::OPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "Reader of field `PAUSE_DUR`"]
pub type PAUSE_DUR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PAUSE_DUR`"]
pub struct PAUSE_DUR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE_DUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `OPCODE`"]
pub type OPCODE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Pause Duration"]
    #[inline(always)]
    pub fn pause_dur(&self) -> PAUSE_DUR_R {
        PAUSE_DUR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Opcode Field In PAUSE Frames"]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pause Duration"]
    #[inline(always)]
    pub fn pause_dur(&mut self) -> PAUSE_DUR_W {
        PAUSE_DUR_W { w: self }
    }
}
