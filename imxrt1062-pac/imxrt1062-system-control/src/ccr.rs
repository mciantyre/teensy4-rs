#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0x0004_0000"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0000
    }
}
#[doc = "Indicates how the processor enters Thread mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NONBASETHRDENA_A {
    #[doc = "0: processor can enter Thread mode only when no exception is active"]
    NONBASETHRDENA_0 = 0,
    #[doc = "1: processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    NONBASETHRDENA_1 = 1,
}
impl From<NONBASETHRDENA_A> for bool {
    #[inline(always)]
    fn from(variant: NONBASETHRDENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NONBASETHRDENA`"]
pub type NONBASETHRDENA_R = crate::R<bool, NONBASETHRDENA_A>;
impl NONBASETHRDENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONBASETHRDENA_A {
        match self.bits {
            false => NONBASETHRDENA_A::NONBASETHRDENA_0,
            true => NONBASETHRDENA_A::NONBASETHRDENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `NONBASETHRDENA_0`"]
    #[inline(always)]
    pub fn is_nonbasethrdena_0(&self) -> bool {
        *self == NONBASETHRDENA_A::NONBASETHRDENA_0
    }
    #[doc = "Checks if the value of the field is `NONBASETHRDENA_1`"]
    #[inline(always)]
    pub fn is_nonbasethrdena_1(&self) -> bool {
        *self == NONBASETHRDENA_A::NONBASETHRDENA_1
    }
}
#[doc = "Write proxy for field `NONBASETHRDENA`"]
pub struct NONBASETHRDENA_W<'a> {
    w: &'a mut W,
}
impl<'a> NONBASETHRDENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NONBASETHRDENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "processor can enter Thread mode only when no exception is active"]
    #[inline(always)]
    pub fn nonbasethrdena_0(self) -> &'a mut W {
        self.variant(NONBASETHRDENA_A::NONBASETHRDENA_0)
    }
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    #[inline(always)]
    pub fn nonbasethrdena_1(self) -> &'a mut W {
        self.variant(NONBASETHRDENA_A::NONBASETHRDENA_1)
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
#[doc = "Enables unprivileged software access to the STIR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERSETMPEND_A {
    #[doc = "0: disable"]
    USERSETMPEND_0 = 0,
    #[doc = "1: enable"]
    USERSETMPEND_1 = 1,
}
impl From<USERSETMPEND_A> for bool {
    #[inline(always)]
    fn from(variant: USERSETMPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USERSETMPEND`"]
pub type USERSETMPEND_R = crate::R<bool, USERSETMPEND_A>;
impl USERSETMPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USERSETMPEND_A {
        match self.bits {
            false => USERSETMPEND_A::USERSETMPEND_0,
            true => USERSETMPEND_A::USERSETMPEND_1,
        }
    }
    #[doc = "Checks if the value of the field is `USERSETMPEND_0`"]
    #[inline(always)]
    pub fn is_usersetmpend_0(&self) -> bool {
        *self == USERSETMPEND_A::USERSETMPEND_0
    }
    #[doc = "Checks if the value of the field is `USERSETMPEND_1`"]
    #[inline(always)]
    pub fn is_usersetmpend_1(&self) -> bool {
        *self == USERSETMPEND_A::USERSETMPEND_1
    }
}
#[doc = "Write proxy for field `USERSETMPEND`"]
pub struct USERSETMPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> USERSETMPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USERSETMPEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn usersetmpend_0(self) -> &'a mut W {
        self.variant(USERSETMPEND_A::USERSETMPEND_0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn usersetmpend_1(self) -> &'a mut W {
        self.variant(USERSETMPEND_A::USERSETMPEND_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Enables unaligned access traps\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGN_TRP_A {
    #[doc = "0: do not trap unaligned halfword and word accesses"]
    UNALIGN_TRP_0 = 0,
    #[doc = "1: trap unaligned halfword and word accesses"]
    UNALIGN_TRP_1 = 1,
}
impl From<UNALIGN_TRP_A> for bool {
    #[inline(always)]
    fn from(variant: UNALIGN_TRP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UNALIGN_TRP`"]
pub type UNALIGN_TRP_R = crate::R<bool, UNALIGN_TRP_A>;
impl UNALIGN_TRP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNALIGN_TRP_A {
        match self.bits {
            false => UNALIGN_TRP_A::UNALIGN_TRP_0,
            true => UNALIGN_TRP_A::UNALIGN_TRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `UNALIGN_TRP_0`"]
    #[inline(always)]
    pub fn is_unalign_trp_0(&self) -> bool {
        *self == UNALIGN_TRP_A::UNALIGN_TRP_0
    }
    #[doc = "Checks if the value of the field is `UNALIGN_TRP_1`"]
    #[inline(always)]
    pub fn is_unalign_trp_1(&self) -> bool {
        *self == UNALIGN_TRP_A::UNALIGN_TRP_1
    }
}
#[doc = "Write proxy for field `UNALIGN_TRP`"]
pub struct UNALIGN_TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> UNALIGN_TRP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNALIGN_TRP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn unalign_trp_0(self) -> &'a mut W {
        self.variant(UNALIGN_TRP_A::UNALIGN_TRP_0)
    }
    #[doc = "trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn unalign_trp_1(self) -> &'a mut W {
        self.variant(UNALIGN_TRP_A::UNALIGN_TRP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV_0_TRP_A {
    #[doc = "0: do not trap divide by 0"]
    DIV_0_TRP_0 = 0,
    #[doc = "1: trap divide by 0"]
    DIV_0_TRP_1 = 1,
}
impl From<DIV_0_TRP_A> for bool {
    #[inline(always)]
    fn from(variant: DIV_0_TRP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIV_0_TRP`"]
pub type DIV_0_TRP_R = crate::R<bool, DIV_0_TRP_A>;
impl DIV_0_TRP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV_0_TRP_A {
        match self.bits {
            false => DIV_0_TRP_A::DIV_0_TRP_0,
            true => DIV_0_TRP_A::DIV_0_TRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIV_0_TRP_0`"]
    #[inline(always)]
    pub fn is_div_0_trp_0(&self) -> bool {
        *self == DIV_0_TRP_A::DIV_0_TRP_0
    }
    #[doc = "Checks if the value of the field is `DIV_0_TRP_1`"]
    #[inline(always)]
    pub fn is_div_0_trp_1(&self) -> bool {
        *self == DIV_0_TRP_A::DIV_0_TRP_1
    }
}
#[doc = "Write proxy for field `DIV_0_TRP`"]
pub struct DIV_0_TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_0_TRP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV_0_TRP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "do not trap divide by 0"]
    #[inline(always)]
    pub fn div_0_trp_0(self) -> &'a mut W {
        self.variant(DIV_0_TRP_A::DIV_0_TRP_0)
    }
    #[doc = "trap divide by 0"]
    #[inline(always)]
    pub fn div_0_trp_1(self) -> &'a mut W {
        self.variant(DIV_0_TRP_A::DIV_0_TRP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFHFNMIGN_A {
    #[doc = "0: data bus faults caused by load and store instructions cause a lock-up"]
    BFHFNMIGN_0 = 0,
    #[doc = "1: handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    BFHFNMIGN_1 = 1,
}
impl From<BFHFNMIGN_A> for bool {
    #[inline(always)]
    fn from(variant: BFHFNMIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BFHFNMIGN`"]
pub type BFHFNMIGN_R = crate::R<bool, BFHFNMIGN_A>;
impl BFHFNMIGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFHFNMIGN_A {
        match self.bits {
            false => BFHFNMIGN_A::BFHFNMIGN_0,
            true => BFHFNMIGN_A::BFHFNMIGN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BFHFNMIGN_0`"]
    #[inline(always)]
    pub fn is_bfhfnmign_0(&self) -> bool {
        *self == BFHFNMIGN_A::BFHFNMIGN_0
    }
    #[doc = "Checks if the value of the field is `BFHFNMIGN_1`"]
    #[inline(always)]
    pub fn is_bfhfnmign_1(&self) -> bool {
        *self == BFHFNMIGN_A::BFHFNMIGN_1
    }
}
#[doc = "Write proxy for field `BFHFNMIGN`"]
pub struct BFHFNMIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> BFHFNMIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFHFNMIGN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    #[inline(always)]
    pub fn bfhfnmign_0(self) -> &'a mut W {
        self.variant(BFHFNMIGN_A::BFHFNMIGN_0)
    }
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    #[inline(always)]
    pub fn bfhfnmign_1(self) -> &'a mut W {
        self.variant(BFHFNMIGN_A::BFHFNMIGN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Indicates stack alignment on exception entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKALIGN_A {
    #[doc = "0: 4-byte aligned"]
    STKALIGN_0 = 0,
    #[doc = "1: 8-byte aligned"]
    STKALIGN_1 = 1,
}
impl From<STKALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: STKALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STKALIGN`"]
pub type STKALIGN_R = crate::R<bool, STKALIGN_A>;
impl STKALIGN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STKALIGN_A {
        match self.bits {
            false => STKALIGN_A::STKALIGN_0,
            true => STKALIGN_A::STKALIGN_1,
        }
    }
    #[doc = "Checks if the value of the field is `STKALIGN_0`"]
    #[inline(always)]
    pub fn is_stkalign_0(&self) -> bool {
        *self == STKALIGN_A::STKALIGN_0
    }
    #[doc = "Checks if the value of the field is `STKALIGN_1`"]
    #[inline(always)]
    pub fn is_stkalign_1(&self) -> bool {
        *self == STKALIGN_A::STKALIGN_1
    }
}
#[doc = "Write proxy for field `STKALIGN`"]
pub struct STKALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> STKALIGN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STKALIGN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn stkalign_0(self) -> &'a mut W {
        self.variant(STKALIGN_A::STKALIGN_0)
    }
    #[doc = "8-byte aligned"]
    #[inline(always)]
    pub fn stkalign_1(self) -> &'a mut W {
        self.variant(STKALIGN_A::STKALIGN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Enables L1 data cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DC_A {
    #[doc = "0: L1 data cache disabled"]
    DC_0 = 0,
    #[doc = "1: L1 data cache enabled"]
    DC_1 = 1,
}
impl From<DC_A> for bool {
    #[inline(always)]
    fn from(variant: DC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DC`"]
pub type DC_R = crate::R<bool, DC_A>;
impl DC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DC_A {
        match self.bits {
            false => DC_A::DC_0,
            true => DC_A::DC_1,
        }
    }
    #[doc = "Checks if the value of the field is `DC_0`"]
    #[inline(always)]
    pub fn is_dc_0(&self) -> bool {
        *self == DC_A::DC_0
    }
    #[doc = "Checks if the value of the field is `DC_1`"]
    #[inline(always)]
    pub fn is_dc_1(&self) -> bool {
        *self == DC_A::DC_1
    }
}
#[doc = "Write proxy for field `DC`"]
pub struct DC_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "L1 data cache disabled"]
    #[inline(always)]
    pub fn dc_0(self) -> &'a mut W {
        self.variant(DC_A::DC_0)
    }
    #[doc = "L1 data cache enabled"]
    #[inline(always)]
    pub fn dc_1(self) -> &'a mut W {
        self.variant(DC_A::DC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Enables L1 instruction cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC_A {
    #[doc = "0: L1 instruction cache disabled"]
    IC_0 = 0,
    #[doc = "1: L1 instruction cache enabled"]
    IC_1 = 1,
}
impl From<IC_A> for bool {
    #[inline(always)]
    fn from(variant: IC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IC`"]
pub type IC_R = crate::R<bool, IC_A>;
impl IC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC_A {
        match self.bits {
            false => IC_A::IC_0,
            true => IC_A::IC_1,
        }
    }
    #[doc = "Checks if the value of the field is `IC_0`"]
    #[inline(always)]
    pub fn is_ic_0(&self) -> bool {
        *self == IC_A::IC_0
    }
    #[doc = "Checks if the value of the field is `IC_1`"]
    #[inline(always)]
    pub fn is_ic_1(&self) -> bool {
        *self == IC_A::IC_1
    }
}
#[doc = "Write proxy for field `IC`"]
pub struct IC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "L1 instruction cache disabled"]
    #[inline(always)]
    pub fn ic_0(self) -> &'a mut W {
        self.variant(IC_A::IC_0)
    }
    #[doc = "L1 instruction cache enabled"]
    #[inline(always)]
    pub fn ic_1(self) -> &'a mut W {
        self.variant(IC_A::IC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `BP`"]
pub type BP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Indicates how the processor enters Thread mode"]
    #[inline(always)]
    pub fn nonbasethrdena(&self) -> NONBASETHRDENA_R {
        NONBASETHRDENA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables unprivileged software access to the STIR"]
    #[inline(always)]
    pub fn usersetmpend(&self) -> USERSETMPEND_R {
        USERSETMPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub fn div_0_trp(&self) -> DIV_0_TRP_R {
        DIV_0_TRP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BFHFNMIGN_R {
        BFHFNMIGN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables L1 data cache."]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables L1 instruction cache."]
    #[inline(always)]
    pub fn ic(&self) -> IC_R {
        IC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Always reads-as-one. It indicates branch prediction is enabled."]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates how the processor enters Thread mode"]
    #[inline(always)]
    pub fn nonbasethrdena(&mut self) -> NONBASETHRDENA_W {
        NONBASETHRDENA_W { w: self }
    }
    #[doc = "Bit 1 - Enables unprivileged software access to the STIR"]
    #[inline(always)]
    pub fn usersetmpend(&mut self) -> USERSETMPEND_W {
        USERSETMPEND_W { w: self }
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    pub fn unalign_trp(&mut self) -> UNALIGN_TRP_W {
        UNALIGN_TRP_W { w: self }
    }
    #[doc = "Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub fn div_0_trp(&mut self) -> DIV_0_TRP_W {
        DIV_0_TRP_W { w: self }
    }
    #[doc = "Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline(always)]
    pub fn bfhfnmign(&mut self) -> BFHFNMIGN_W {
        BFHFNMIGN_W { w: self }
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&mut self) -> STKALIGN_W {
        STKALIGN_W { w: self }
    }
    #[doc = "Bit 16 - Enables L1 data cache."]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W {
        DC_W { w: self }
    }
    #[doc = "Bit 17 - Enables L1 instruction cache."]
    #[inline(always)]
    pub fn ic(&mut self) -> IC_W {
        IC_W { w: self }
    }
}
