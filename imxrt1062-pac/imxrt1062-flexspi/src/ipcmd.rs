#[doc = "Reader of register IPCMD"]
pub type R = crate::R<u32, super::IPCMD>;
#[doc = "Writer for register IPCMD"]
pub type W = crate::W<u32, super::IPCMD>;
#[doc = "Register IPCMD `reset()`'s with value 0"]
impl crate::ResetValue for super::IPCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRG`"]
pub type TRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRG`"]
pub struct TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Setting this bit will trigger an IP Command."]
    #[inline(always)]
    pub fn trg(&self) -> TRG_R {
        TRG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit will trigger an IP Command."]
    #[inline(always)]
    pub fn trg(&mut self) -> TRG_W {
        TRG_W { w: self }
    }
}
