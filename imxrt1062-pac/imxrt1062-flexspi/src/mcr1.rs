#[doc = "Reader of register MCR1"]
pub type R = crate::R<u32, super::MCR1>;
#[doc = "Writer for register MCR1"]
pub type W = crate::W<u32, super::MCR1>;
#[doc = "Register MCR1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `AHBBUSWAIT`"]
pub type AHBBUSWAIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AHBBUSWAIT`"]
pub struct AHBBUSWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBBUSWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `SEQWAIT`"]
pub type SEQWAIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SEQWAIT`"]
pub struct SEQWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmited after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
    #[inline(always)]
    pub fn ahbbuswait(&self) -> AHBBUSWAIT_R {
        AHBBUSWAIT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
    #[inline(always)]
    pub fn seqwait(&self) -> SEQWAIT_R {
        SEQWAIT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmited after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
    #[inline(always)]
    pub fn ahbbuswait(&mut self) -> AHBBUSWAIT_W {
        AHBBUSWAIT_W { w: self }
    }
    #[doc = "Bits 16:31 - Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
    #[inline(always)]
    pub fn seqwait(&mut self) -> SEQWAIT_W {
        SEQWAIT_W { w: self }
    }
}
