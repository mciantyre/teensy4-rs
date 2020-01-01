#[doc = "Reader of register OSC_CONFIG1_TOG"]
pub type R = crate::R<u32, super::OSC_CONFIG1_TOG>;
#[doc = "Writer for register OSC_CONFIG1_TOG"]
pub type W = crate::W<u32, super::OSC_CONFIG1_TOG>;
#[doc = "Register OSC_CONFIG1_TOG `reset()`'s with value 0x02ee"]
impl crate::ResetValue for super::OSC_CONFIG1_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02ee
    }
}
#[doc = "Reader of field `COUNT_RC_TRG`"]
pub type COUNT_RC_TRG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNT_RC_TRG`"]
pub struct COUNT_RC_TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_RC_TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `COUNT_RC_CUR`"]
pub type COUNT_RC_CUR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNT_RC_CUR`"]
pub struct COUNT_RC_CUR_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_RC_CUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub fn count_rc_trg(&self) -> COUNT_RC_TRG_R {
        COUNT_RC_TRG_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - The current tuning value in use."]
    #[inline(always)]
    pub fn count_rc_cur(&self) -> COUNT_RC_CUR_R {
        COUNT_RC_CUR_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub fn count_rc_trg(&mut self) -> COUNT_RC_TRG_W {
        COUNT_RC_TRG_W { w: self }
    }
    #[doc = "Bits 20:31 - The current tuning value in use."]
    #[inline(always)]
    pub fn count_rc_cur(&mut self) -> COUNT_RC_CUR_W {
        COUNT_RC_CUR_W { w: self }
    }
}
