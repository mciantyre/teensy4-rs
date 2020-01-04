#[doc = "Reader of register SCR"]
pub type R = crate::R<u32, super::SCR>;
#[doc = "Writer for register SCR"]
pub type W = crate::W<u32, super::SCR>;
#[doc = "Register SCR `reset()`'s with value 0xa048_0520"]
impl crate::ResetValue for super::SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa048_0520
    }
}
#[doc = "Mask wdog_rst_b source\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MASK_WDOG_RST_A {
    #[doc = "5: wdog_rst_b is masked"]
    MASK_WDOG_RST_5 = 5,
    #[doc = "10: wdog_rst_b is not masked (default)"]
    MASK_WDOG_RST_10 = 10,
}
impl From<MASK_WDOG_RST_A> for u8 {
    #[inline(always)]
    fn from(variant: MASK_WDOG_RST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `mask_wdog_rst`"]
pub type MASK_WDOG_RST_R = crate::R<u8, MASK_WDOG_RST_A>;
impl MASK_WDOG_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MASK_WDOG_RST_A> {
        use crate::Variant::*;
        match self.bits {
            5 => Val(MASK_WDOG_RST_A::MASK_WDOG_RST_5),
            10 => Val(MASK_WDOG_RST_A::MASK_WDOG_RST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG_RST_5`"]
    #[inline(always)]
    pub fn is_mask_wdog_rst_5(&self) -> bool {
        *self == MASK_WDOG_RST_A::MASK_WDOG_RST_5
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG_RST_10`"]
    #[inline(always)]
    pub fn is_mask_wdog_rst_10(&self) -> bool {
        *self == MASK_WDOG_RST_A::MASK_WDOG_RST_10
    }
}
#[doc = "Write proxy for field `mask_wdog_rst`"]
pub struct MASK_WDOG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_WDOG_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_WDOG_RST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "wdog_rst_b is masked"]
    #[inline(always)]
    pub fn mask_wdog_rst_5(self) -> &'a mut W {
        self.variant(MASK_WDOG_RST_A::MASK_WDOG_RST_5)
    }
    #[doc = "wdog_rst_b is not masked (default)"]
    #[inline(always)]
    pub fn mask_wdog_rst_10(self) -> &'a mut W {
        self.variant(MASK_WDOG_RST_A::MASK_WDOG_RST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Software reset for core0 only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE0_RST_A {
    #[doc = "0: do not assert core0 reset"]
    CORE0_RST_0 = 0,
    #[doc = "1: assert core0 reset"]
    CORE0_RST_1 = 1,
}
impl From<CORE0_RST_A> for bool {
    #[inline(always)]
    fn from(variant: CORE0_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `core0_rst`"]
pub type CORE0_RST_R = crate::R<bool, CORE0_RST_A>;
impl CORE0_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CORE0_RST_A {
        match self.bits {
            false => CORE0_RST_A::CORE0_RST_0,
            true => CORE0_RST_A::CORE0_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CORE0_RST_0`"]
    #[inline(always)]
    pub fn is_core0_rst_0(&self) -> bool {
        *self == CORE0_RST_A::CORE0_RST_0
    }
    #[doc = "Checks if the value of the field is `CORE0_RST_1`"]
    #[inline(always)]
    pub fn is_core0_rst_1(&self) -> bool {
        *self == CORE0_RST_A::CORE0_RST_1
    }
}
#[doc = "Write proxy for field `core0_rst`"]
pub struct CORE0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE0_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CORE0_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "do not assert core0 reset"]
    #[inline(always)]
    pub fn core0_rst_0(self) -> &'a mut W {
        self.variant(CORE0_RST_A::CORE0_RST_0)
    }
    #[doc = "assert core0 reset"]
    #[inline(always)]
    pub fn core0_rst_1(self) -> &'a mut W {
        self.variant(CORE0_RST_A::CORE0_RST_1)
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
#[doc = "Software reset for core0 debug only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE0_DBG_RST_A {
    #[doc = "0: do not assert core0 debug reset"]
    CORE0_DBG_RST_0 = 0,
    #[doc = "1: assert core0 debug reset"]
    CORE0_DBG_RST_1 = 1,
}
impl From<CORE0_DBG_RST_A> for bool {
    #[inline(always)]
    fn from(variant: CORE0_DBG_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `core0_dbg_rst`"]
pub type CORE0_DBG_RST_R = crate::R<bool, CORE0_DBG_RST_A>;
impl CORE0_DBG_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CORE0_DBG_RST_A {
        match self.bits {
            false => CORE0_DBG_RST_A::CORE0_DBG_RST_0,
            true => CORE0_DBG_RST_A::CORE0_DBG_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CORE0_DBG_RST_0`"]
    #[inline(always)]
    pub fn is_core0_dbg_rst_0(&self) -> bool {
        *self == CORE0_DBG_RST_A::CORE0_DBG_RST_0
    }
    #[doc = "Checks if the value of the field is `CORE0_DBG_RST_1`"]
    #[inline(always)]
    pub fn is_core0_dbg_rst_1(&self) -> bool {
        *self == CORE0_DBG_RST_A::CORE0_DBG_RST_1
    }
}
#[doc = "Write proxy for field `core0_dbg_rst`"]
pub struct CORE0_DBG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE0_DBG_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CORE0_DBG_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "do not assert core0 debug reset"]
    #[inline(always)]
    pub fn core0_dbg_rst_0(self) -> &'a mut W {
        self.variant(CORE0_DBG_RST_A::CORE0_DBG_RST_0)
    }
    #[doc = "assert core0 debug reset"]
    #[inline(always)]
    pub fn core0_dbg_rst_1(self) -> &'a mut W {
        self.variant(CORE0_DBG_RST_A::CORE0_DBG_RST_1)
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
#[doc = "Do not assert debug resets after power gating event of core\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_RST_MSK_PG_A {
    #[doc = "0: do not mask core debug resets (debug resets will be asserted after power gating event)"]
    DBG_RST_MSK_PG_0 = 0,
    #[doc = "1: mask core debug resets (debug resets won't be asserted after power gating event)"]
    DBG_RST_MSK_PG_1 = 1,
}
impl From<DBG_RST_MSK_PG_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_RST_MSK_PG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `dbg_rst_msk_pg`"]
pub type DBG_RST_MSK_PG_R = crate::R<bool, DBG_RST_MSK_PG_A>;
impl DBG_RST_MSK_PG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_RST_MSK_PG_A {
        match self.bits {
            false => DBG_RST_MSK_PG_A::DBG_RST_MSK_PG_0,
            true => DBG_RST_MSK_PG_A::DBG_RST_MSK_PG_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBG_RST_MSK_PG_0`"]
    #[inline(always)]
    pub fn is_dbg_rst_msk_pg_0(&self) -> bool {
        *self == DBG_RST_MSK_PG_A::DBG_RST_MSK_PG_0
    }
    #[doc = "Checks if the value of the field is `DBG_RST_MSK_PG_1`"]
    #[inline(always)]
    pub fn is_dbg_rst_msk_pg_1(&self) -> bool {
        *self == DBG_RST_MSK_PG_A::DBG_RST_MSK_PG_1
    }
}
#[doc = "Write proxy for field `dbg_rst_msk_pg`"]
pub struct DBG_RST_MSK_PG_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_RST_MSK_PG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_RST_MSK_PG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "do not mask core debug resets (debug resets will be asserted after power gating event)"]
    #[inline(always)]
    pub fn dbg_rst_msk_pg_0(self) -> &'a mut W {
        self.variant(DBG_RST_MSK_PG_A::DBG_RST_MSK_PG_0)
    }
    #[doc = "mask core debug resets (debug resets won't be asserted after power gating event)"]
    #[inline(always)]
    pub fn dbg_rst_msk_pg_1(self) -> &'a mut W {
        self.variant(DBG_RST_MSK_PG_A::DBG_RST_MSK_PG_1)
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
#[doc = "Mask wdog3_rst_b source\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MASK_WDOG3_RST_A {
    #[doc = "5: wdog3_rst_b is masked"]
    MASK_WDOG3_RST_5 = 5,
    #[doc = "10: wdog3_rst_b is not masked"]
    MASK_WDOG3_RST_10 = 10,
}
impl From<MASK_WDOG3_RST_A> for u8 {
    #[inline(always)]
    fn from(variant: MASK_WDOG3_RST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `mask_wdog3_rst`"]
pub type MASK_WDOG3_RST_R = crate::R<u8, MASK_WDOG3_RST_A>;
impl MASK_WDOG3_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MASK_WDOG3_RST_A> {
        use crate::Variant::*;
        match self.bits {
            5 => Val(MASK_WDOG3_RST_A::MASK_WDOG3_RST_5),
            10 => Val(MASK_WDOG3_RST_A::MASK_WDOG3_RST_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG3_RST_5`"]
    #[inline(always)]
    pub fn is_mask_wdog3_rst_5(&self) -> bool {
        *self == MASK_WDOG3_RST_A::MASK_WDOG3_RST_5
    }
    #[doc = "Checks if the value of the field is `MASK_WDOG3_RST_10`"]
    #[inline(always)]
    pub fn is_mask_wdog3_rst_10(&self) -> bool {
        *self == MASK_WDOG3_RST_A::MASK_WDOG3_RST_10
    }
}
#[doc = "Write proxy for field `mask_wdog3_rst`"]
pub struct MASK_WDOG3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_WDOG3_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_WDOG3_RST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "wdog3_rst_b is masked"]
    #[inline(always)]
    pub fn mask_wdog3_rst_5(self) -> &'a mut W {
        self.variant(MASK_WDOG3_RST_A::MASK_WDOG3_RST_5)
    }
    #[doc = "wdog3_rst_b is not masked"]
    #[inline(always)]
    pub fn mask_wdog3_rst_10(self) -> &'a mut W {
        self.variant(MASK_WDOG3_RST_A::MASK_WDOG3_RST_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:10 - Mask wdog_rst_b source"]
    #[inline(always)]
    pub fn mask_wdog_rst(&self) -> MASK_WDOG_RST_R {
        MASK_WDOG_RST_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Software reset for core0 only"]
    #[inline(always)]
    pub fn core0_rst(&self) -> CORE0_RST_R {
        CORE0_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Software reset for core0 debug only"]
    #[inline(always)]
    pub fn core0_dbg_rst(&self) -> CORE0_DBG_RST_R {
        CORE0_DBG_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Do not assert debug resets after power gating event of core"]
    #[inline(always)]
    pub fn dbg_rst_msk_pg(&self) -> DBG_RST_MSK_PG_R {
        DBG_RST_MSK_PG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - Mask wdog3_rst_b source"]
    #[inline(always)]
    pub fn mask_wdog3_rst(&self) -> MASK_WDOG3_RST_R {
        MASK_WDOG3_RST_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:10 - Mask wdog_rst_b source"]
    #[inline(always)]
    pub fn mask_wdog_rst(&mut self) -> MASK_WDOG_RST_W {
        MASK_WDOG_RST_W { w: self }
    }
    #[doc = "Bit 13 - Software reset for core0 only"]
    #[inline(always)]
    pub fn core0_rst(&mut self) -> CORE0_RST_W {
        CORE0_RST_W { w: self }
    }
    #[doc = "Bit 17 - Software reset for core0 debug only"]
    #[inline(always)]
    pub fn core0_dbg_rst(&mut self) -> CORE0_DBG_RST_W {
        CORE0_DBG_RST_W { w: self }
    }
    #[doc = "Bit 25 - Do not assert debug resets after power gating event of core"]
    #[inline(always)]
    pub fn dbg_rst_msk_pg(&mut self) -> DBG_RST_MSK_PG_W {
        DBG_RST_MSK_PG_W { w: self }
    }
    #[doc = "Bits 28:31 - Mask wdog3_rst_b source"]
    #[inline(always)]
    pub fn mask_wdog3_rst(&mut self) -> MASK_WDOG3_RST_W {
        MASK_WDOG3_RST_W { w: self }
    }
}
