#[doc = "Reader of register GPR25"]
pub type R = crate::R<u32, super::GPR25>;
#[doc = "Writer for register GPR25"]
pub type W = crate::W<u32, super::GPR25>;
#[doc = "Register GPR25 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR25 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "lock M7_APC_AC_R3_TOP field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_M7_APC_AC_R3_TOP_A {
    #[doc = "0: Register field \\[31:1\\] is not locked"]
    LOCK_M7_APC_AC_R3_TOP_0,
    #[doc = "1: Register field \\[31:1\\] is locked (read access only)"]
    LOCK_M7_APC_AC_R3_TOP_1,
}
impl From<LOCK_M7_APC_AC_R3_TOP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_M7_APC_AC_R3_TOP_A) -> Self {
        match variant {
            LOCK_M7_APC_AC_R3_TOP_A::LOCK_M7_APC_AC_R3_TOP_0 => false,
            LOCK_M7_APC_AC_R3_TOP_A::LOCK_M7_APC_AC_R3_TOP_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCK_M7_APC_AC_R3_TOP`"]
pub type LOCK_M7_APC_AC_R3_TOP_R = crate::R<bool, LOCK_M7_APC_AC_R3_TOP_A>;
impl LOCK_M7_APC_AC_R3_TOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_M7_APC_AC_R3_TOP_A {
        match self.bits {
            false => LOCK_M7_APC_AC_R3_TOP_A::LOCK_M7_APC_AC_R3_TOP_0,
            true => LOCK_M7_APC_AC_R3_TOP_A::LOCK_M7_APC_AC_R3_TOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R3_TOP_0`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r3_top_0(&self) -> bool {
        *self == LOCK_M7_APC_AC_R3_TOP_A::LOCK_M7_APC_AC_R3_TOP_0
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R3_TOP_1`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r3_top_1(&self) -> bool {
        *self == LOCK_M7_APC_AC_R3_TOP_A::LOCK_M7_APC_AC_R3_TOP_1
    }
}
#[doc = "Write proxy for field `LOCK_M7_APC_AC_R3_TOP`"]
pub struct LOCK_M7_APC_AC_R3_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_M7_APC_AC_R3_TOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_M7_APC_AC_R3_TOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register field \\[31:1\\] is not locked"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r3_top_0(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R3_TOP_A::LOCK_M7_APC_AC_R3_TOP_0)
    }
    #[doc = "Register field \\[31:1\\] is locked (read access only)"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r3_top_1(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R3_TOP_A::LOCK_M7_APC_AC_R3_TOP_1)
    }
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
#[doc = "Reader of field `M7_APC_AC_R3_TOP`"]
pub type M7_APC_AC_R3_TOP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `M7_APC_AC_R3_TOP`"]
pub struct M7_APC_AC_R3_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> M7_APC_AC_R3_TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - lock M7_APC_AC_R3_TOP field for changes"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r3_top(&self) -> LOCK_M7_APC_AC_R3_TOP_R {
        LOCK_M7_APC_AC_R3_TOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 3:31 - APC start address of memory region-3"]
    #[inline(always)]
    pub fn m7_apc_ac_r3_top(&self) -> M7_APC_AC_R3_TOP_R {
        M7_APC_AC_R3_TOP_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - lock M7_APC_AC_R3_TOP field for changes"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r3_top(&mut self) -> LOCK_M7_APC_AC_R3_TOP_W {
        LOCK_M7_APC_AC_R3_TOP_W { w: self }
    }
    #[doc = "Bits 3:31 - APC start address of memory region-3"]
    #[inline(always)]
    pub fn m7_apc_ac_r3_top(&mut self) -> M7_APC_AC_R3_TOP_W {
        M7_APC_AC_R3_TOP_W { w: self }
    }
}
