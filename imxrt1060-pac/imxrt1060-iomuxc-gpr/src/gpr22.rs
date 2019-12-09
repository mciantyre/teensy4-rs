#[doc = "Reader of register GPR22"]
pub type R = crate::R<u32, super::GPR22>;
#[doc = "Writer for register GPR22"]
pub type W = crate::W<u32, super::GPR22>;
#[doc = "Register GPR22 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR22 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "lock M7_APC_AC_R2_BOT field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_M7_APC_AC_R2_BOT_A {
    #[doc = "0: Register field \\[31:1\\] is not locked"]
    LOCK_M7_APC_AC_R2_BOT_0,
    #[doc = "1: Register field \\[31:1\\] is locked (read access only)"]
    LOCK_M7_APC_AC_R2_BOT_1,
}
impl From<LOCK_M7_APC_AC_R2_BOT_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_M7_APC_AC_R2_BOT_A) -> Self {
        match variant {
            LOCK_M7_APC_AC_R2_BOT_A::LOCK_M7_APC_AC_R2_BOT_0 => false,
            LOCK_M7_APC_AC_R2_BOT_A::LOCK_M7_APC_AC_R2_BOT_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCK_M7_APC_AC_R2_BOT`"]
pub type LOCK_M7_APC_AC_R2_BOT_R = crate::R<bool, LOCK_M7_APC_AC_R2_BOT_A>;
impl LOCK_M7_APC_AC_R2_BOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_M7_APC_AC_R2_BOT_A {
        match self.bits {
            false => LOCK_M7_APC_AC_R2_BOT_A::LOCK_M7_APC_AC_R2_BOT_0,
            true => LOCK_M7_APC_AC_R2_BOT_A::LOCK_M7_APC_AC_R2_BOT_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R2_BOT_0`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r2_bot_0(&self) -> bool {
        *self == LOCK_M7_APC_AC_R2_BOT_A::LOCK_M7_APC_AC_R2_BOT_0
    }
    #[doc = "Checks if the value of the field is `LOCK_M7_APC_AC_R2_BOT_1`"]
    #[inline(always)]
    pub fn is_lock_m7_apc_ac_r2_bot_1(&self) -> bool {
        *self == LOCK_M7_APC_AC_R2_BOT_A::LOCK_M7_APC_AC_R2_BOT_1
    }
}
#[doc = "Write proxy for field `LOCK_M7_APC_AC_R2_BOT`"]
pub struct LOCK_M7_APC_AC_R2_BOT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_M7_APC_AC_R2_BOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_M7_APC_AC_R2_BOT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Register field \\[31:1\\] is not locked"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r2_bot_0(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R2_BOT_A::LOCK_M7_APC_AC_R2_BOT_0)
    }
    #[doc = "Register field \\[31:1\\] is locked (read access only)"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r2_bot_1(self) -> &'a mut W {
        self.variant(LOCK_M7_APC_AC_R2_BOT_A::LOCK_M7_APC_AC_R2_BOT_1)
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
#[doc = "Reader of field `M7_APC_AC_R2_BOT`"]
pub type M7_APC_AC_R2_BOT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `M7_APC_AC_R2_BOT`"]
pub struct M7_APC_AC_R2_BOT_W<'a> {
    w: &'a mut W,
}
impl<'a> M7_APC_AC_R2_BOT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - lock M7_APC_AC_R2_BOT field for changes"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r2_bot(&self) -> LOCK_M7_APC_AC_R2_BOT_R {
        LOCK_M7_APC_AC_R2_BOT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 3:31 - APC end address of memory region-2"]
    #[inline(always)]
    pub fn m7_apc_ac_r2_bot(&self) -> M7_APC_AC_R2_BOT_R {
        M7_APC_AC_R2_BOT_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - lock M7_APC_AC_R2_BOT field for changes"]
    #[inline(always)]
    pub fn lock_m7_apc_ac_r2_bot(&mut self) -> LOCK_M7_APC_AC_R2_BOT_W {
        LOCK_M7_APC_AC_R2_BOT_W { w: self }
    }
    #[doc = "Bits 3:31 - APC end address of memory region-2"]
    #[inline(always)]
    pub fn m7_apc_ac_r2_bot(&mut self) -> M7_APC_AC_R2_BOT_W {
        M7_APC_AC_R2_BOT_W { w: self }
    }
}
