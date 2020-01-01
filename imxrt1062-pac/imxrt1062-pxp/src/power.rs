#[doc = "Reader of register POWER"]
pub type R = crate::R<u32, super::POWER>;
#[doc = "Writer for register POWER"]
pub type W = crate::W<u32, super::POWER>;
#[doc = "Register POWER `reset()`'s with value 0"]
impl crate::ResetValue for super::POWER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select the low power state of the ROT memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ROT_MEM_LP_STATE_A {
    #[doc = "0: Memory is not in low power state."]
    NONE = 0,
    #[doc = "1: Light Sleep Mode. Low leakage mode, maintain memory contents."]
    LS = 1,
    #[doc = "2: Deep Sleep Mode. Low leakage mode, maintain memory contents."]
    DS = 2,
    #[doc = "4: Shut Down Mode. Shut Down periphery and core, no memory retention."]
    SD = 4,
}
impl From<ROT_MEM_LP_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: ROT_MEM_LP_STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ROT_MEM_LP_STATE`"]
pub type ROT_MEM_LP_STATE_R = crate::R<u8, ROT_MEM_LP_STATE_A>;
impl ROT_MEM_LP_STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ROT_MEM_LP_STATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ROT_MEM_LP_STATE_A::NONE),
            1 => Val(ROT_MEM_LP_STATE_A::LS),
            2 => Val(ROT_MEM_LP_STATE_A::DS),
            4 => Val(ROT_MEM_LP_STATE_A::SD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ROT_MEM_LP_STATE_A::NONE
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == ROT_MEM_LP_STATE_A::LS
    }
    #[doc = "Checks if the value of the field is `DS`"]
    #[inline(always)]
    pub fn is_ds(&self) -> bool {
        *self == ROT_MEM_LP_STATE_A::DS
    }
    #[doc = "Checks if the value of the field is `SD`"]
    #[inline(always)]
    pub fn is_sd(&self) -> bool {
        *self == ROT_MEM_LP_STATE_A::SD
    }
}
#[doc = "Write proxy for field `ROT_MEM_LP_STATE`"]
pub struct ROT_MEM_LP_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROT_MEM_LP_STATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROT_MEM_LP_STATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Memory is not in low power state."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ROT_MEM_LP_STATE_A::NONE)
    }
    #[doc = "Light Sleep Mode. Low leakage mode, maintain memory contents."]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(ROT_MEM_LP_STATE_A::LS)
    }
    #[doc = "Deep Sleep Mode. Low leakage mode, maintain memory contents."]
    #[inline(always)]
    pub fn ds(self) -> &'a mut W {
        self.variant(ROT_MEM_LP_STATE_A::DS)
    }
    #[doc = "Shut Down Mode. Shut Down periphery and core, no memory retention."]
    #[inline(always)]
    pub fn sd(self) -> &'a mut W {
        self.variant(ROT_MEM_LP_STATE_A::SD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `CTRL`"]
pub type CTRL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CTRL`"]
pub struct CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:11 - Select the low power state of the ROT memory."]
    #[inline(always)]
    pub fn rot_mem_lp_state(&self) -> ROT_MEM_LP_STATE_R {
        ROT_MEM_LP_STATE_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:31 - Power control for the PXP."]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 9:11 - Select the low power state of the ROT memory."]
    #[inline(always)]
    pub fn rot_mem_lp_state(&mut self) -> ROT_MEM_LP_STATE_W {
        ROT_MEM_LP_STATE_W { w: self }
    }
    #[doc = "Bits 12:31 - Power control for the PXP."]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W {
        CTRL_W { w: self }
    }
}
