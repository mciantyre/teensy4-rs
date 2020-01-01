#[doc = "Reader of register CFSR"]
pub type R = crate::R<u32, super::CFSR>;
#[doc = "Writer for register CFSR"]
pub type W = crate::W<u32, super::CFSR>;
#[doc = "Register CFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Instruction access violation flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IACCVIOL_A {
    #[doc = "0: no instruction access violation fault"]
    IACCVIOL_0 = 0,
    #[doc = "1: the processor attempted an instruction fetch from a location that does not permit execution"]
    IACCVIOL_1 = 1,
}
impl From<IACCVIOL_A> for bool {
    #[inline(always)]
    fn from(variant: IACCVIOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IACCVIOL`"]
pub type IACCVIOL_R = crate::R<bool, IACCVIOL_A>;
impl IACCVIOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IACCVIOL_A {
        match self.bits {
            false => IACCVIOL_A::IACCVIOL_0,
            true => IACCVIOL_A::IACCVIOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `IACCVIOL_0`"]
    #[inline(always)]
    pub fn is_iaccviol_0(&self) -> bool {
        *self == IACCVIOL_A::IACCVIOL_0
    }
    #[doc = "Checks if the value of the field is `IACCVIOL_1`"]
    #[inline(always)]
    pub fn is_iaccviol_1(&self) -> bool {
        *self == IACCVIOL_A::IACCVIOL_1
    }
}
#[doc = "Write proxy for field `IACCVIOL`"]
pub struct IACCVIOL_W<'a> {
    w: &'a mut W,
}
impl<'a> IACCVIOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IACCVIOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no instruction access violation fault"]
    #[inline(always)]
    pub fn iaccviol_0(self) -> &'a mut W {
        self.variant(IACCVIOL_A::IACCVIOL_0)
    }
    #[doc = "the processor attempted an instruction fetch from a location that does not permit execution"]
    #[inline(always)]
    pub fn iaccviol_1(self) -> &'a mut W {
        self.variant(IACCVIOL_A::IACCVIOL_1)
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
#[doc = "Data access violation flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACCVIOL_A {
    #[doc = "0: no data access violation fault"]
    DACCVIOL_0 = 0,
    #[doc = "1: the processor attempted a load or store at a location that does not permit the operation"]
    DACCVIOL_1 = 1,
}
impl From<DACCVIOL_A> for bool {
    #[inline(always)]
    fn from(variant: DACCVIOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DACCVIOL`"]
pub type DACCVIOL_R = crate::R<bool, DACCVIOL_A>;
impl DACCVIOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACCVIOL_A {
        match self.bits {
            false => DACCVIOL_A::DACCVIOL_0,
            true => DACCVIOL_A::DACCVIOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACCVIOL_0`"]
    #[inline(always)]
    pub fn is_daccviol_0(&self) -> bool {
        *self == DACCVIOL_A::DACCVIOL_0
    }
    #[doc = "Checks if the value of the field is `DACCVIOL_1`"]
    #[inline(always)]
    pub fn is_daccviol_1(&self) -> bool {
        *self == DACCVIOL_A::DACCVIOL_1
    }
}
#[doc = "Write proxy for field `DACCVIOL`"]
pub struct DACCVIOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCVIOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACCVIOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no data access violation fault"]
    #[inline(always)]
    pub fn daccviol_0(self) -> &'a mut W {
        self.variant(DACCVIOL_A::DACCVIOL_0)
    }
    #[doc = "the processor attempted a load or store at a location that does not permit the operation"]
    #[inline(always)]
    pub fn daccviol_1(self) -> &'a mut W {
        self.variant(DACCVIOL_A::DACCVIOL_1)
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
#[doc = "MemManage fault on unstacking for a return from exception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUNSTKERR_A {
    #[doc = "0: no unstacking fault"]
    MUNSTKERR_0 = 0,
    #[doc = "1: unstack for an exception return has caused one or more access violations"]
    MUNSTKERR_1 = 1,
}
impl From<MUNSTKERR_A> for bool {
    #[inline(always)]
    fn from(variant: MUNSTKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MUNSTKERR`"]
pub type MUNSTKERR_R = crate::R<bool, MUNSTKERR_A>;
impl MUNSTKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUNSTKERR_A {
        match self.bits {
            false => MUNSTKERR_A::MUNSTKERR_0,
            true => MUNSTKERR_A::MUNSTKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `MUNSTKERR_0`"]
    #[inline(always)]
    pub fn is_munstkerr_0(&self) -> bool {
        *self == MUNSTKERR_A::MUNSTKERR_0
    }
    #[doc = "Checks if the value of the field is `MUNSTKERR_1`"]
    #[inline(always)]
    pub fn is_munstkerr_1(&self) -> bool {
        *self == MUNSTKERR_A::MUNSTKERR_1
    }
}
#[doc = "Write proxy for field `MUNSTKERR`"]
pub struct MUNSTKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MUNSTKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUNSTKERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no unstacking fault"]
    #[inline(always)]
    pub fn munstkerr_0(self) -> &'a mut W {
        self.variant(MUNSTKERR_A::MUNSTKERR_0)
    }
    #[doc = "unstack for an exception return has caused one or more access violations"]
    #[inline(always)]
    pub fn munstkerr_1(self) -> &'a mut W {
        self.variant(MUNSTKERR_A::MUNSTKERR_1)
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
#[doc = "MemManage fault on stacking for exception entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTKERR_A {
    #[doc = "0: no stacking fault"]
    MSTKERR_0 = 0,
    #[doc = "1: stacking for an exception entry has caused one or more access violations"]
    MSTKERR_1 = 1,
}
impl From<MSTKERR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTKERR`"]
pub type MSTKERR_R = crate::R<bool, MSTKERR_A>;
impl MSTKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTKERR_A {
        match self.bits {
            false => MSTKERR_A::MSTKERR_0,
            true => MSTKERR_A::MSTKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `MSTKERR_0`"]
    #[inline(always)]
    pub fn is_mstkerr_0(&self) -> bool {
        *self == MSTKERR_A::MSTKERR_0
    }
    #[doc = "Checks if the value of the field is `MSTKERR_1`"]
    #[inline(always)]
    pub fn is_mstkerr_1(&self) -> bool {
        *self == MSTKERR_A::MSTKERR_1
    }
}
#[doc = "Write proxy for field `MSTKERR`"]
pub struct MSTKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTKERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no stacking fault"]
    #[inline(always)]
    pub fn mstkerr_0(self) -> &'a mut W {
        self.variant(MSTKERR_A::MSTKERR_0)
    }
    #[doc = "stacking for an exception entry has caused one or more access violations"]
    #[inline(always)]
    pub fn mstkerr_1(self) -> &'a mut W {
        self.variant(MSTKERR_A::MSTKERR_1)
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
#[doc = "MemManage fault occurred during floating-point lazy state preservation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MLSPERR_A {
    #[doc = "0: No MemManage fault occurred during floating-point lazy state preservation"]
    MLSPERR_0 = 0,
    #[doc = "1: A MemManage fault occurred during floating-point lazy state preservation"]
    MLSPERR_1 = 1,
}
impl From<MLSPERR_A> for bool {
    #[inline(always)]
    fn from(variant: MLSPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MLSPERR`"]
pub type MLSPERR_R = crate::R<bool, MLSPERR_A>;
impl MLSPERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MLSPERR_A {
        match self.bits {
            false => MLSPERR_A::MLSPERR_0,
            true => MLSPERR_A::MLSPERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `MLSPERR_0`"]
    #[inline(always)]
    pub fn is_mlsperr_0(&self) -> bool {
        *self == MLSPERR_A::MLSPERR_0
    }
    #[doc = "Checks if the value of the field is `MLSPERR_1`"]
    #[inline(always)]
    pub fn is_mlsperr_1(&self) -> bool {
        *self == MLSPERR_A::MLSPERR_1
    }
}
#[doc = "Write proxy for field `MLSPERR`"]
pub struct MLSPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MLSPERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MLSPERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr_0(self) -> &'a mut W {
        self.variant(MLSPERR_A::MLSPERR_0)
    }
    #[doc = "A MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr_1(self) -> &'a mut W {
        self.variant(MLSPERR_A::MLSPERR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "MemManage Fault Address Register (MMFAR) valid flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMARVALID_A {
    #[doc = "0: value in MMAR is not a valid fault address"]
    MMARVALID_0 = 0,
    #[doc = "1: MMAR holds a valid fault address"]
    MMARVALID_1 = 1,
}
impl From<MMARVALID_A> for bool {
    #[inline(always)]
    fn from(variant: MMARVALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MMARVALID`"]
pub type MMARVALID_R = crate::R<bool, MMARVALID_A>;
impl MMARVALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMARVALID_A {
        match self.bits {
            false => MMARVALID_A::MMARVALID_0,
            true => MMARVALID_A::MMARVALID_1,
        }
    }
    #[doc = "Checks if the value of the field is `MMARVALID_0`"]
    #[inline(always)]
    pub fn is_mmarvalid_0(&self) -> bool {
        *self == MMARVALID_A::MMARVALID_0
    }
    #[doc = "Checks if the value of the field is `MMARVALID_1`"]
    #[inline(always)]
    pub fn is_mmarvalid_1(&self) -> bool {
        *self == MMARVALID_A::MMARVALID_1
    }
}
#[doc = "Write proxy for field `MMARVALID`"]
pub struct MMARVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> MMARVALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMARVALID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "value in MMAR is not a valid fault address"]
    #[inline(always)]
    pub fn mmarvalid_0(self) -> &'a mut W {
        self.variant(MMARVALID_A::MMARVALID_0)
    }
    #[doc = "MMAR holds a valid fault address"]
    #[inline(always)]
    pub fn mmarvalid_1(self) -> &'a mut W {
        self.variant(MMARVALID_A::MMARVALID_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Instruction bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBUSERR_A {
    #[doc = "0: no instruction bus error"]
    IBUSERR_0 = 0,
    #[doc = "1: instruction bus error"]
    IBUSERR_1 = 1,
}
impl From<IBUSERR_A> for bool {
    #[inline(always)]
    fn from(variant: IBUSERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IBUSERR`"]
pub type IBUSERR_R = crate::R<bool, IBUSERR_A>;
impl IBUSERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBUSERR_A {
        match self.bits {
            false => IBUSERR_A::IBUSERR_0,
            true => IBUSERR_A::IBUSERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `IBUSERR_0`"]
    #[inline(always)]
    pub fn is_ibuserr_0(&self) -> bool {
        *self == IBUSERR_A::IBUSERR_0
    }
    #[doc = "Checks if the value of the field is `IBUSERR_1`"]
    #[inline(always)]
    pub fn is_ibuserr_1(&self) -> bool {
        *self == IBUSERR_A::IBUSERR_1
    }
}
#[doc = "Write proxy for field `IBUSERR`"]
pub struct IBUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUSERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IBUSERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no instruction bus error"]
    #[inline(always)]
    pub fn ibuserr_0(self) -> &'a mut W {
        self.variant(IBUSERR_A::IBUSERR_0)
    }
    #[doc = "instruction bus error"]
    #[inline(always)]
    pub fn ibuserr_1(self) -> &'a mut W {
        self.variant(IBUSERR_A::IBUSERR_1)
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
#[doc = "Precise data bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRECISERR_A {
    #[doc = "0: no precise data bus error"]
    PRECISERR_0 = 0,
    #[doc = "1: a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    PRECISERR_1 = 1,
}
impl From<PRECISERR_A> for bool {
    #[inline(always)]
    fn from(variant: PRECISERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRECISERR`"]
pub type PRECISERR_R = crate::R<bool, PRECISERR_A>;
impl PRECISERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRECISERR_A {
        match self.bits {
            false => PRECISERR_A::PRECISERR_0,
            true => PRECISERR_A::PRECISERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `PRECISERR_0`"]
    #[inline(always)]
    pub fn is_preciserr_0(&self) -> bool {
        *self == PRECISERR_A::PRECISERR_0
    }
    #[doc = "Checks if the value of the field is `PRECISERR_1`"]
    #[inline(always)]
    pub fn is_preciserr_1(&self) -> bool {
        *self == PRECISERR_A::PRECISERR_1
    }
}
#[doc = "Write proxy for field `PRECISERR`"]
pub struct PRECISERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRECISERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRECISERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no precise data bus error"]
    #[inline(always)]
    pub fn preciserr_0(self) -> &'a mut W {
        self.variant(PRECISERR_A::PRECISERR_0)
    }
    #[doc = "a data bus error has occurred, and the PC value stacked for the exception return points to the instruction that caused the fault"]
    #[inline(always)]
    pub fn preciserr_1(self) -> &'a mut W {
        self.variant(PRECISERR_A::PRECISERR_1)
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
#[doc = "Imprecise data bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMPRECISERR_A {
    #[doc = "0: no imprecise data bus error"]
    IMPRECISERR_0 = 0,
    #[doc = "1: a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    IMPRECISERR_1 = 1,
}
impl From<IMPRECISERR_A> for bool {
    #[inline(always)]
    fn from(variant: IMPRECISERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IMPRECISERR`"]
pub type IMPRECISERR_R = crate::R<bool, IMPRECISERR_A>;
impl IMPRECISERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMPRECISERR_A {
        match self.bits {
            false => IMPRECISERR_A::IMPRECISERR_0,
            true => IMPRECISERR_A::IMPRECISERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `IMPRECISERR_0`"]
    #[inline(always)]
    pub fn is_impreciserr_0(&self) -> bool {
        *self == IMPRECISERR_A::IMPRECISERR_0
    }
    #[doc = "Checks if the value of the field is `IMPRECISERR_1`"]
    #[inline(always)]
    pub fn is_impreciserr_1(&self) -> bool {
        *self == IMPRECISERR_A::IMPRECISERR_1
    }
}
#[doc = "Write proxy for field `IMPRECISERR`"]
pub struct IMPRECISERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMPRECISERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMPRECISERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no imprecise data bus error"]
    #[inline(always)]
    pub fn impreciserr_0(self) -> &'a mut W {
        self.variant(IMPRECISERR_A::IMPRECISERR_0)
    }
    #[doc = "a data bus error has occurred, but the return address in the stack frame is not related to the instruction that caused the error"]
    #[inline(always)]
    pub fn impreciserr_1(self) -> &'a mut W {
        self.variant(IMPRECISERR_A::IMPRECISERR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "BusFault on unstacking for a return from exception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNSTKERR_A {
    #[doc = "0: no unstacking fault"]
    UNSTKERR_0 = 0,
    #[doc = "1: unstack for an exception return has caused one or more BusFaults"]
    UNSTKERR_1 = 1,
}
impl From<UNSTKERR_A> for bool {
    #[inline(always)]
    fn from(variant: UNSTKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UNSTKERR`"]
pub type UNSTKERR_R = crate::R<bool, UNSTKERR_A>;
impl UNSTKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNSTKERR_A {
        match self.bits {
            false => UNSTKERR_A::UNSTKERR_0,
            true => UNSTKERR_A::UNSTKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `UNSTKERR_0`"]
    #[inline(always)]
    pub fn is_unstkerr_0(&self) -> bool {
        *self == UNSTKERR_A::UNSTKERR_0
    }
    #[doc = "Checks if the value of the field is `UNSTKERR_1`"]
    #[inline(always)]
    pub fn is_unstkerr_1(&self) -> bool {
        *self == UNSTKERR_A::UNSTKERR_1
    }
}
#[doc = "Write proxy for field `UNSTKERR`"]
pub struct UNSTKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> UNSTKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNSTKERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no unstacking fault"]
    #[inline(always)]
    pub fn unstkerr_0(self) -> &'a mut W {
        self.variant(UNSTKERR_A::UNSTKERR_0)
    }
    #[doc = "unstack for an exception return has caused one or more BusFaults"]
    #[inline(always)]
    pub fn unstkerr_1(self) -> &'a mut W {
        self.variant(UNSTKERR_A::UNSTKERR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "BusFault on stacking for exception entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKERR_A {
    #[doc = "0: no stacking fault"]
    STKERR_0 = 0,
    #[doc = "1: stacking for an exception entry has caused one or more BusFaults"]
    STKERR_1 = 1,
}
impl From<STKERR_A> for bool {
    #[inline(always)]
    fn from(variant: STKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STKERR`"]
pub type STKERR_R = crate::R<bool, STKERR_A>;
impl STKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STKERR_A {
        match self.bits {
            false => STKERR_A::STKERR_0,
            true => STKERR_A::STKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `STKERR_0`"]
    #[inline(always)]
    pub fn is_stkerr_0(&self) -> bool {
        *self == STKERR_A::STKERR_0
    }
    #[doc = "Checks if the value of the field is `STKERR_1`"]
    #[inline(always)]
    pub fn is_stkerr_1(&self) -> bool {
        *self == STKERR_A::STKERR_1
    }
}
#[doc = "Write proxy for field `STKERR`"]
pub struct STKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> STKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STKERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no stacking fault"]
    #[inline(always)]
    pub fn stkerr_0(self) -> &'a mut W {
        self.variant(STKERR_A::STKERR_0)
    }
    #[doc = "stacking for an exception entry has caused one or more BusFaults"]
    #[inline(always)]
    pub fn stkerr_1(self) -> &'a mut W {
        self.variant(STKERR_A::STKERR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Bus fault occurred during floating-point lazy state preservation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPERR_A {
    #[doc = "0: No bus fault occurred during floating-point lazy state preservation"]
    LSPERR_0 = 0,
    #[doc = "1: A bus fault occurred during floating-point lazy state preservation"]
    LSPERR_1 = 1,
}
impl From<LSPERR_A> for bool {
    #[inline(always)]
    fn from(variant: LSPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSPERR`"]
pub type LSPERR_R = crate::R<bool, LSPERR_A>;
impl LSPERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSPERR_A {
        match self.bits {
            false => LSPERR_A::LSPERR_0,
            true => LSPERR_A::LSPERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `LSPERR_0`"]
    #[inline(always)]
    pub fn is_lsperr_0(&self) -> bool {
        *self == LSPERR_A::LSPERR_0
    }
    #[doc = "Checks if the value of the field is `LSPERR_1`"]
    #[inline(always)]
    pub fn is_lsperr_1(&self) -> bool {
        *self == LSPERR_A::LSPERR_1
    }
}
#[doc = "Write proxy for field `LSPERR`"]
pub struct LSPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSPERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn lsperr_0(self) -> &'a mut W {
        self.variant(LSPERR_A::LSPERR_0)
    }
    #[doc = "A bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn lsperr_1(self) -> &'a mut W {
        self.variant(LSPERR_A::LSPERR_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "BusFault Address Register (BFAR) valid flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFARVALID_A {
    #[doc = "0: value in BFAR is not a valid fault address"]
    BFARVALID_0 = 0,
    #[doc = "1: BFAR holds a valid fault address"]
    BFARVALID_1 = 1,
}
impl From<BFARVALID_A> for bool {
    #[inline(always)]
    fn from(variant: BFARVALID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BFARVALID`"]
pub type BFARVALID_R = crate::R<bool, BFARVALID_A>;
impl BFARVALID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFARVALID_A {
        match self.bits {
            false => BFARVALID_A::BFARVALID_0,
            true => BFARVALID_A::BFARVALID_1,
        }
    }
    #[doc = "Checks if the value of the field is `BFARVALID_0`"]
    #[inline(always)]
    pub fn is_bfarvalid_0(&self) -> bool {
        *self == BFARVALID_A::BFARVALID_0
    }
    #[doc = "Checks if the value of the field is `BFARVALID_1`"]
    #[inline(always)]
    pub fn is_bfarvalid_1(&self) -> bool {
        *self == BFARVALID_A::BFARVALID_1
    }
}
#[doc = "Write proxy for field `BFARVALID`"]
pub struct BFARVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> BFARVALID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFARVALID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "value in BFAR is not a valid fault address"]
    #[inline(always)]
    pub fn bfarvalid_0(self) -> &'a mut W {
        self.variant(BFARVALID_A::BFARVALID_0)
    }
    #[doc = "BFAR holds a valid fault address"]
    #[inline(always)]
    pub fn bfarvalid_1(self) -> &'a mut W {
        self.variant(BFARVALID_A::BFARVALID_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Undefined instruction UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDEFINSTR_A {
    #[doc = "0: no undefined instruction UsageFault"]
    UNDEFINSTR_0 = 0,
    #[doc = "1: the processor has attempted to execute an undefined instruction"]
    UNDEFINSTR_1 = 1,
}
impl From<UNDEFINSTR_A> for bool {
    #[inline(always)]
    fn from(variant: UNDEFINSTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UNDEFINSTR`"]
pub type UNDEFINSTR_R = crate::R<bool, UNDEFINSTR_A>;
impl UNDEFINSTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDEFINSTR_A {
        match self.bits {
            false => UNDEFINSTR_A::UNDEFINSTR_0,
            true => UNDEFINSTR_A::UNDEFINSTR_1,
        }
    }
    #[doc = "Checks if the value of the field is `UNDEFINSTR_0`"]
    #[inline(always)]
    pub fn is_undefinstr_0(&self) -> bool {
        *self == UNDEFINSTR_A::UNDEFINSTR_0
    }
    #[doc = "Checks if the value of the field is `UNDEFINSTR_1`"]
    #[inline(always)]
    pub fn is_undefinstr_1(&self) -> bool {
        *self == UNDEFINSTR_A::UNDEFINSTR_1
    }
}
#[doc = "Write proxy for field `UNDEFINSTR`"]
pub struct UNDEFINSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDEFINSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNDEFINSTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no undefined instruction UsageFault"]
    #[inline(always)]
    pub fn undefinstr_0(self) -> &'a mut W {
        self.variant(UNDEFINSTR_A::UNDEFINSTR_0)
    }
    #[doc = "the processor has attempted to execute an undefined instruction"]
    #[inline(always)]
    pub fn undefinstr_1(self) -> &'a mut W {
        self.variant(UNDEFINSTR_A::UNDEFINSTR_1)
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
#[doc = "Invalid state UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVSTATE_A {
    #[doc = "0: no invalid state UsageFault"]
    INVSTATE_0 = 0,
    #[doc = "1: the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    INVSTATE_1 = 1,
}
impl From<INVSTATE_A> for bool {
    #[inline(always)]
    fn from(variant: INVSTATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INVSTATE`"]
pub type INVSTATE_R = crate::R<bool, INVSTATE_A>;
impl INVSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVSTATE_A {
        match self.bits {
            false => INVSTATE_A::INVSTATE_0,
            true => INVSTATE_A::INVSTATE_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVSTATE_0`"]
    #[inline(always)]
    pub fn is_invstate_0(&self) -> bool {
        *self == INVSTATE_A::INVSTATE_0
    }
    #[doc = "Checks if the value of the field is `INVSTATE_1`"]
    #[inline(always)]
    pub fn is_invstate_1(&self) -> bool {
        *self == INVSTATE_A::INVSTATE_1
    }
}
#[doc = "Write proxy for field `INVSTATE`"]
pub struct INVSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVSTATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVSTATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no invalid state UsageFault"]
    #[inline(always)]
    pub fn invstate_0(self) -> &'a mut W {
        self.variant(INVSTATE_A::INVSTATE_0)
    }
    #[doc = "the processor has attempted to execute an instruction that makes illegal use of the EPSR"]
    #[inline(always)]
    pub fn invstate_1(self) -> &'a mut W {
        self.variant(INVSTATE_A::INVSTATE_1)
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
#[doc = "Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVPC_A {
    #[doc = "0: no invalid PC load UsageFault"]
    INVPC_0 = 0,
    #[doc = "1: the processor has attempted an illegal load of EXC_RETURN to the PC"]
    INVPC_1 = 1,
}
impl From<INVPC_A> for bool {
    #[inline(always)]
    fn from(variant: INVPC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INVPC`"]
pub type INVPC_R = crate::R<bool, INVPC_A>;
impl INVPC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVPC_A {
        match self.bits {
            false => INVPC_A::INVPC_0,
            true => INVPC_A::INVPC_1,
        }
    }
    #[doc = "Checks if the value of the field is `INVPC_0`"]
    #[inline(always)]
    pub fn is_invpc_0(&self) -> bool {
        *self == INVPC_A::INVPC_0
    }
    #[doc = "Checks if the value of the field is `INVPC_1`"]
    #[inline(always)]
    pub fn is_invpc_1(&self) -> bool {
        *self == INVPC_A::INVPC_1
    }
}
#[doc = "Write proxy for field `INVPC`"]
pub struct INVPC_W<'a> {
    w: &'a mut W,
}
impl<'a> INVPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVPC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no invalid PC load UsageFault"]
    #[inline(always)]
    pub fn invpc_0(self) -> &'a mut W {
        self.variant(INVPC_A::INVPC_0)
    }
    #[doc = "the processor has attempted an illegal load of EXC_RETURN to the PC"]
    #[inline(always)]
    pub fn invpc_1(self) -> &'a mut W {
        self.variant(INVPC_A::INVPC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "No coprocessor UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOCP_A {
    #[doc = "0: no UsageFault caused by attempting to access a coprocessor"]
    NOCP_0 = 0,
    #[doc = "1: the processor has attempted to access a coprocessor"]
    NOCP_1 = 1,
}
impl From<NOCP_A> for bool {
    #[inline(always)]
    fn from(variant: NOCP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NOCP`"]
pub type NOCP_R = crate::R<bool, NOCP_A>;
impl NOCP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOCP_A {
        match self.bits {
            false => NOCP_A::NOCP_0,
            true => NOCP_A::NOCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `NOCP_0`"]
    #[inline(always)]
    pub fn is_nocp_0(&self) -> bool {
        *self == NOCP_A::NOCP_0
    }
    #[doc = "Checks if the value of the field is `NOCP_1`"]
    #[inline(always)]
    pub fn is_nocp_1(&self) -> bool {
        *self == NOCP_A::NOCP_1
    }
}
#[doc = "Write proxy for field `NOCP`"]
pub struct NOCP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOCP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no UsageFault caused by attempting to access a coprocessor"]
    #[inline(always)]
    pub fn nocp_0(self) -> &'a mut W {
        self.variant(NOCP_A::NOCP_0)
    }
    #[doc = "the processor has attempted to access a coprocessor"]
    #[inline(always)]
    pub fn nocp_1(self) -> &'a mut W {
        self.variant(NOCP_A::NOCP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Unaligned access UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGNED_A {
    #[doc = "0: no unaligned access fault, or unaligned access trapping not enabled"]
    UNALIGNED_0 = 0,
    #[doc = "1: the processor has made an unaligned memory access"]
    UNALIGNED_1 = 1,
}
impl From<UNALIGNED_A> for bool {
    #[inline(always)]
    fn from(variant: UNALIGNED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UNALIGNED`"]
pub type UNALIGNED_R = crate::R<bool, UNALIGNED_A>;
impl UNALIGNED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNALIGNED_A {
        match self.bits {
            false => UNALIGNED_A::UNALIGNED_0,
            true => UNALIGNED_A::UNALIGNED_1,
        }
    }
    #[doc = "Checks if the value of the field is `UNALIGNED_0`"]
    #[inline(always)]
    pub fn is_unaligned_0(&self) -> bool {
        *self == UNALIGNED_A::UNALIGNED_0
    }
    #[doc = "Checks if the value of the field is `UNALIGNED_1`"]
    #[inline(always)]
    pub fn is_unaligned_1(&self) -> bool {
        *self == UNALIGNED_A::UNALIGNED_1
    }
}
#[doc = "Write proxy for field `UNALIGNED`"]
pub struct UNALIGNED_W<'a> {
    w: &'a mut W,
}
impl<'a> UNALIGNED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNALIGNED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no unaligned access fault, or unaligned access trapping not enabled"]
    #[inline(always)]
    pub fn unaligned_0(self) -> &'a mut W {
        self.variant(UNALIGNED_A::UNALIGNED_0)
    }
    #[doc = "the processor has made an unaligned memory access"]
    #[inline(always)]
    pub fn unaligned_1(self) -> &'a mut W {
        self.variant(UNALIGNED_A::UNALIGNED_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Divide by zero UsageFault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBYZERO_A {
    #[doc = "0: no divide by zero fault, or divide by zero trapping not enabled"]
    DIVBYZERO_0 = 0,
    #[doc = "1: the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    DIVBYZERO_1 = 1,
}
impl From<DIVBYZERO_A> for bool {
    #[inline(always)]
    fn from(variant: DIVBYZERO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIVBYZERO`"]
pub type DIVBYZERO_R = crate::R<bool, DIVBYZERO_A>;
impl DIVBYZERO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVBYZERO_A {
        match self.bits {
            false => DIVBYZERO_A::DIVBYZERO_0,
            true => DIVBYZERO_A::DIVBYZERO_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIVBYZERO_0`"]
    #[inline(always)]
    pub fn is_divbyzero_0(&self) -> bool {
        *self == DIVBYZERO_A::DIVBYZERO_0
    }
    #[doc = "Checks if the value of the field is `DIVBYZERO_1`"]
    #[inline(always)]
    pub fn is_divbyzero_1(&self) -> bool {
        *self == DIVBYZERO_A::DIVBYZERO_1
    }
}
#[doc = "Write proxy for field `DIVBYZERO`"]
pub struct DIVBYZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVBYZERO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVBYZERO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no divide by zero fault, or divide by zero trapping not enabled"]
    #[inline(always)]
    pub fn divbyzero_0(self) -> &'a mut W {
        self.variant(DIVBYZERO_A::DIVBYZERO_0)
    }
    #[doc = "the processor has executed an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub fn divbyzero_1(self) -> &'a mut W {
        self.variant(DIVBYZERO_A::DIVBYZERO_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Instruction access violation flag"]
    #[inline(always)]
    pub fn iaccviol(&self) -> IACCVIOL_R {
        IACCVIOL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data access violation flag"]
    #[inline(always)]
    pub fn daccviol(&self) -> DACCVIOL_R {
        DACCVIOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MemManage fault on unstacking for a return from exception"]
    #[inline(always)]
    pub fn munstkerr(&self) -> MUNSTKERR_R {
        MUNSTKERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MemManage fault on stacking for exception entry"]
    #[inline(always)]
    pub fn mstkerr(&self) -> MSTKERR_R {
        MSTKERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr(&self) -> MLSPERR_R {
        MLSPERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline(always)]
    pub fn mmarvalid(&self) -> MMARVALID_R {
        MMARVALID_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline(always)]
    pub fn ibuserr(&self) -> IBUSERR_R {
        IBUSERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline(always)]
    pub fn preciserr(&self) -> PRECISERR_R {
        PRECISERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline(always)]
    pub fn impreciserr(&self) -> IMPRECISERR_R {
        IMPRECISERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BusFault on unstacking for a return from exception"]
    #[inline(always)]
    pub fn unstkerr(&self) -> UNSTKERR_R {
        UNSTKERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline(always)]
    pub fn stkerr(&self) -> STKERR_R {
        STKERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn lsperr(&self) -> LSPERR_R {
        LSPERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - BusFault Address Register (BFAR) valid flag"]
    #[inline(always)]
    pub fn bfarvalid(&self) -> BFARVALID_R {
        BFARVALID_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline(always)]
    pub fn undefinstr(&self) -> UNDEFINSTR_R {
        UNDEFINSTR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline(always)]
    pub fn invstate(&self) -> INVSTATE_R {
        INVSTATE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN"]
    #[inline(always)]
    pub fn invpc(&self) -> INVPC_R {
        INVPC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline(always)]
    pub fn unaligned(&self) -> UNALIGNED_R {
        UNALIGNED_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline(always)]
    pub fn divbyzero(&self) -> DIVBYZERO_R {
        DIVBYZERO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Instruction access violation flag"]
    #[inline(always)]
    pub fn iaccviol(&mut self) -> IACCVIOL_W {
        IACCVIOL_W { w: self }
    }
    #[doc = "Bit 1 - Data access violation flag"]
    #[inline(always)]
    pub fn daccviol(&mut self) -> DACCVIOL_W {
        DACCVIOL_W { w: self }
    }
    #[doc = "Bit 3 - MemManage fault on unstacking for a return from exception"]
    #[inline(always)]
    pub fn munstkerr(&mut self) -> MUNSTKERR_W {
        MUNSTKERR_W { w: self }
    }
    #[doc = "Bit 4 - MemManage fault on stacking for exception entry"]
    #[inline(always)]
    pub fn mstkerr(&mut self) -> MSTKERR_W {
        MSTKERR_W { w: self }
    }
    #[doc = "Bit 5 - MemManage fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn mlsperr(&mut self) -> MLSPERR_W {
        MLSPERR_W { w: self }
    }
    #[doc = "Bit 7 - MemManage Fault Address Register (MMFAR) valid flag"]
    #[inline(always)]
    pub fn mmarvalid(&mut self) -> MMARVALID_W {
        MMARVALID_W { w: self }
    }
    #[doc = "Bit 8 - Instruction bus error"]
    #[inline(always)]
    pub fn ibuserr(&mut self) -> IBUSERR_W {
        IBUSERR_W { w: self }
    }
    #[doc = "Bit 9 - Precise data bus error"]
    #[inline(always)]
    pub fn preciserr(&mut self) -> PRECISERR_W {
        PRECISERR_W { w: self }
    }
    #[doc = "Bit 10 - Imprecise data bus error"]
    #[inline(always)]
    pub fn impreciserr(&mut self) -> IMPRECISERR_W {
        IMPRECISERR_W { w: self }
    }
    #[doc = "Bit 11 - BusFault on unstacking for a return from exception"]
    #[inline(always)]
    pub fn unstkerr(&mut self) -> UNSTKERR_W {
        UNSTKERR_W { w: self }
    }
    #[doc = "Bit 12 - BusFault on stacking for exception entry"]
    #[inline(always)]
    pub fn stkerr(&mut self) -> STKERR_W {
        STKERR_W { w: self }
    }
    #[doc = "Bit 13 - Bus fault occurred during floating-point lazy state preservation"]
    #[inline(always)]
    pub fn lsperr(&mut self) -> LSPERR_W {
        LSPERR_W { w: self }
    }
    #[doc = "Bit 15 - BusFault Address Register (BFAR) valid flag"]
    #[inline(always)]
    pub fn bfarvalid(&mut self) -> BFARVALID_W {
        BFARVALID_W { w: self }
    }
    #[doc = "Bit 16 - Undefined instruction UsageFault"]
    #[inline(always)]
    pub fn undefinstr(&mut self) -> UNDEFINSTR_W {
        UNDEFINSTR_W { w: self }
    }
    #[doc = "Bit 17 - Invalid state UsageFault"]
    #[inline(always)]
    pub fn invstate(&mut self) -> INVSTATE_W {
        INVSTATE_W { w: self }
    }
    #[doc = "Bit 18 - Invalid PC load UsageFault, caused by an invalid PC load by EXC_RETURN"]
    #[inline(always)]
    pub fn invpc(&mut self) -> INVPC_W {
        INVPC_W { w: self }
    }
    #[doc = "Bit 19 - No coprocessor UsageFault"]
    #[inline(always)]
    pub fn nocp(&mut self) -> NOCP_W {
        NOCP_W { w: self }
    }
    #[doc = "Bit 24 - Unaligned access UsageFault"]
    #[inline(always)]
    pub fn unaligned(&mut self) -> UNALIGNED_W {
        UNALIGNED_W { w: self }
    }
    #[doc = "Bit 25 - Divide by zero UsageFault"]
    #[inline(always)]
    pub fn divbyzero(&mut self) -> DIVBYZERO_W {
        DIVBYZERO_W { w: self }
    }
}
