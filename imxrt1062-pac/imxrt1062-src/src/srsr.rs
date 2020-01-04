#[doc = "Reader of register SRSR"]
pub type R = crate::R<u32, super::SRSR>;
#[doc = "Writer for register SRSR"]
pub type W = crate::W<u32, super::SRSR>;
#[doc = "Register SRSR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SRSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPP_RESET_B_A {
    #[doc = "0: Reset is not a result of ipp_reset_b pin."]
    IPP_RESET_B_0 = 0,
    #[doc = "1: Reset is a result of ipp_reset_b pin."]
    IPP_RESET_B_1 = 1,
}
impl From<IPP_RESET_B_A> for bool {
    #[inline(always)]
    fn from(variant: IPP_RESET_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ipp_reset_b`"]
pub type IPP_RESET_B_R = crate::R<bool, IPP_RESET_B_A>;
impl IPP_RESET_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPP_RESET_B_A {
        match self.bits {
            false => IPP_RESET_B_A::IPP_RESET_B_0,
            true => IPP_RESET_B_A::IPP_RESET_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPP_RESET_B_0`"]
    #[inline(always)]
    pub fn is_ipp_reset_b_0(&self) -> bool {
        *self == IPP_RESET_B_A::IPP_RESET_B_0
    }
    #[doc = "Checks if the value of the field is `IPP_RESET_B_1`"]
    #[inline(always)]
    pub fn is_ipp_reset_b_1(&self) -> bool {
        *self == IPP_RESET_B_A::IPP_RESET_B_1
    }
}
#[doc = "Write proxy for field `ipp_reset_b`"]
pub struct IPP_RESET_B_W<'a> {
    w: &'a mut W,
}
impl<'a> IPP_RESET_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPP_RESET_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset is not a result of ipp_reset_b pin."]
    #[inline(always)]
    pub fn ipp_reset_b_0(self) -> &'a mut W {
        self.variant(IPP_RESET_B_A::IPP_RESET_B_0)
    }
    #[doc = "Reset is a result of ipp_reset_b pin."]
    #[inline(always)]
    pub fn ipp_reset_b_1(self) -> &'a mut W {
        self.variant(IPP_RESET_B_A::IPP_RESET_B_1)
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
#[doc = "Indicates a reset has been caused by CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_SYSRESETREQ_A {
    #[doc = "0: Reset is not a result of the mentioned case."]
    LOCKUP_SYSRESETREQ_0 = 0,
    #[doc = "1: Reset is a result of the mentioned case."]
    LOCKUP_SYSRESETREQ_1 = 1,
}
impl From<LOCKUP_SYSRESETREQ_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_SYSRESETREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `lockup_sysresetreq`"]
pub type LOCKUP_SYSRESETREQ_R = crate::R<bool, LOCKUP_SYSRESETREQ_A>;
impl LOCKUP_SYSRESETREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_SYSRESETREQ_A {
        match self.bits {
            false => LOCKUP_SYSRESETREQ_A::LOCKUP_SYSRESETREQ_0,
            true => LOCKUP_SYSRESETREQ_A::LOCKUP_SYSRESETREQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKUP_SYSRESETREQ_0`"]
    #[inline(always)]
    pub fn is_lockup_sysresetreq_0(&self) -> bool {
        *self == LOCKUP_SYSRESETREQ_A::LOCKUP_SYSRESETREQ_0
    }
    #[doc = "Checks if the value of the field is `LOCKUP_SYSRESETREQ_1`"]
    #[inline(always)]
    pub fn is_lockup_sysresetreq_1(&self) -> bool {
        *self == LOCKUP_SYSRESETREQ_A::LOCKUP_SYSRESETREQ_1
    }
}
#[doc = "Write proxy for field `lockup_sysresetreq`"]
pub struct LOCKUP_SYSRESETREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_SYSRESETREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKUP_SYSRESETREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset is not a result of the mentioned case."]
    #[inline(always)]
    pub fn lockup_sysresetreq_0(self) -> &'a mut W {
        self.variant(LOCKUP_SYSRESETREQ_A::LOCKUP_SYSRESETREQ_0)
    }
    #[doc = "Reset is a result of the mentioned case."]
    #[inline(always)]
    pub fn lockup_sysresetreq_1(self) -> &'a mut W {
        self.variant(LOCKUP_SYSRESETREQ_A::LOCKUP_SYSRESETREQ_1)
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
#[doc = "Indicates whether the reset was the result of the csu_reset_b input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSU_RESET_B_A {
    #[doc = "0: Reset is not a result of the csu_reset_b event."]
    CSU_RESET_B_0 = 0,
    #[doc = "1: Reset is a result of the csu_reset_b event."]
    CSU_RESET_B_1 = 1,
}
impl From<CSU_RESET_B_A> for bool {
    #[inline(always)]
    fn from(variant: CSU_RESET_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `csu_reset_b`"]
pub type CSU_RESET_B_R = crate::R<bool, CSU_RESET_B_A>;
impl CSU_RESET_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSU_RESET_B_A {
        match self.bits {
            false => CSU_RESET_B_A::CSU_RESET_B_0,
            true => CSU_RESET_B_A::CSU_RESET_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `CSU_RESET_B_0`"]
    #[inline(always)]
    pub fn is_csu_reset_b_0(&self) -> bool {
        *self == CSU_RESET_B_A::CSU_RESET_B_0
    }
    #[doc = "Checks if the value of the field is `CSU_RESET_B_1`"]
    #[inline(always)]
    pub fn is_csu_reset_b_1(&self) -> bool {
        *self == CSU_RESET_B_A::CSU_RESET_B_1
    }
}
#[doc = "Write proxy for field `csu_reset_b`"]
pub struct CSU_RESET_B_W<'a> {
    w: &'a mut W,
}
impl<'a> CSU_RESET_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSU_RESET_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset is not a result of the csu_reset_b event."]
    #[inline(always)]
    pub fn csu_reset_b_0(self) -> &'a mut W {
        self.variant(CSU_RESET_B_A::CSU_RESET_B_0)
    }
    #[doc = "Reset is a result of the csu_reset_b event."]
    #[inline(always)]
    pub fn csu_reset_b_1(self) -> &'a mut W {
        self.variant(CSU_RESET_B_A::CSU_RESET_B_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Indicates whether the reset was the result of the ipp_user_reset_b qualified reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPP_USER_RESET_B_A {
    #[doc = "0: Reset is not a result of the ipp_user_reset_b qualified as COLD reset event."]
    IPP_USER_RESET_B_0 = 0,
    #[doc = "1: Reset is a result of the ipp_user_reset_b qualified as COLD reset event."]
    IPP_USER_RESET_B_1 = 1,
}
impl From<IPP_USER_RESET_B_A> for bool {
    #[inline(always)]
    fn from(variant: IPP_USER_RESET_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ipp_user_reset_b`"]
pub type IPP_USER_RESET_B_R = crate::R<bool, IPP_USER_RESET_B_A>;
impl IPP_USER_RESET_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPP_USER_RESET_B_A {
        match self.bits {
            false => IPP_USER_RESET_B_A::IPP_USER_RESET_B_0,
            true => IPP_USER_RESET_B_A::IPP_USER_RESET_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPP_USER_RESET_B_0`"]
    #[inline(always)]
    pub fn is_ipp_user_reset_b_0(&self) -> bool {
        *self == IPP_USER_RESET_B_A::IPP_USER_RESET_B_0
    }
    #[doc = "Checks if the value of the field is `IPP_USER_RESET_B_1`"]
    #[inline(always)]
    pub fn is_ipp_user_reset_b_1(&self) -> bool {
        *self == IPP_USER_RESET_B_A::IPP_USER_RESET_B_1
    }
}
#[doc = "Write proxy for field `ipp_user_reset_b`"]
pub struct IPP_USER_RESET_B_W<'a> {
    w: &'a mut W,
}
impl<'a> IPP_USER_RESET_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPP_USER_RESET_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset is not a result of the ipp_user_reset_b qualified as COLD reset event."]
    #[inline(always)]
    pub fn ipp_user_reset_b_0(self) -> &'a mut W {
        self.variant(IPP_USER_RESET_B_A::IPP_USER_RESET_B_0)
    }
    #[doc = "Reset is a result of the ipp_user_reset_b qualified as COLD reset event."]
    #[inline(always)]
    pub fn ipp_user_reset_b_1(self) -> &'a mut W {
        self.variant(IPP_USER_RESET_B_A::IPP_USER_RESET_B_1)
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
#[doc = "IC Watchdog Time-out reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_RST_B_A {
    #[doc = "0: Reset is not a result of the watchdog time-out event."]
    WDOG_RST_B_0 = 0,
    #[doc = "1: Reset is a result of the watchdog time-out event."]
    WDOG_RST_B_1 = 1,
}
impl From<WDOG_RST_B_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_RST_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `wdog_rst_b`"]
pub type WDOG_RST_B_R = crate::R<bool, WDOG_RST_B_A>;
impl WDOG_RST_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG_RST_B_A {
        match self.bits {
            false => WDOG_RST_B_A::WDOG_RST_B_0,
            true => WDOG_RST_B_A::WDOG_RST_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG_RST_B_0`"]
    #[inline(always)]
    pub fn is_wdog_rst_b_0(&self) -> bool {
        *self == WDOG_RST_B_A::WDOG_RST_B_0
    }
    #[doc = "Checks if the value of the field is `WDOG_RST_B_1`"]
    #[inline(always)]
    pub fn is_wdog_rst_b_1(&self) -> bool {
        *self == WDOG_RST_B_A::WDOG_RST_B_1
    }
}
#[doc = "Write proxy for field `wdog_rst_b`"]
pub struct WDOG_RST_B_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_RST_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG_RST_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset is not a result of the watchdog time-out event."]
    #[inline(always)]
    pub fn wdog_rst_b_0(self) -> &'a mut W {
        self.variant(WDOG_RST_B_A::WDOG_RST_B_0)
    }
    #[doc = "Reset is a result of the watchdog time-out event."]
    #[inline(always)]
    pub fn wdog_rst_b_1(self) -> &'a mut W {
        self.variant(WDOG_RST_B_A::WDOG_RST_B_1)
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
#[doc = "HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAG_RST_B_A {
    #[doc = "0: Reset is not a result of HIGH-Z reset from JTAG."]
    JTAG_RST_B_0 = 0,
    #[doc = "1: Reset is a result of HIGH-Z reset from JTAG."]
    JTAG_RST_B_1 = 1,
}
impl From<JTAG_RST_B_A> for bool {
    #[inline(always)]
    fn from(variant: JTAG_RST_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `jtag_rst_b`"]
pub type JTAG_RST_B_R = crate::R<bool, JTAG_RST_B_A>;
impl JTAG_RST_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTAG_RST_B_A {
        match self.bits {
            false => JTAG_RST_B_A::JTAG_RST_B_0,
            true => JTAG_RST_B_A::JTAG_RST_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `JTAG_RST_B_0`"]
    #[inline(always)]
    pub fn is_jtag_rst_b_0(&self) -> bool {
        *self == JTAG_RST_B_A::JTAG_RST_B_0
    }
    #[doc = "Checks if the value of the field is `JTAG_RST_B_1`"]
    #[inline(always)]
    pub fn is_jtag_rst_b_1(&self) -> bool {
        *self == JTAG_RST_B_A::JTAG_RST_B_1
    }
}
#[doc = "Write proxy for field `jtag_rst_b`"]
pub struct JTAG_RST_B_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_RST_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JTAG_RST_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset is not a result of HIGH-Z reset from JTAG."]
    #[inline(always)]
    pub fn jtag_rst_b_0(self) -> &'a mut W {
        self.variant(JTAG_RST_B_A::JTAG_RST_B_0)
    }
    #[doc = "Reset is a result of HIGH-Z reset from JTAG."]
    #[inline(always)]
    pub fn jtag_rst_b_1(self) -> &'a mut W {
        self.variant(JTAG_RST_B_A::JTAG_RST_B_1)
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
#[doc = "JTAG software reset. Indicates whether the reset was the result of software reset from JTAG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAG_SW_RST_A {
    #[doc = "0: Reset is not a result of software reset from JTAG."]
    JTAG_SW_RST_0 = 0,
    #[doc = "1: Reset is a result of software reset from JTAG."]
    JTAG_SW_RST_1 = 1,
}
impl From<JTAG_SW_RST_A> for bool {
    #[inline(always)]
    fn from(variant: JTAG_SW_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `jtag_sw_rst`"]
pub type JTAG_SW_RST_R = crate::R<bool, JTAG_SW_RST_A>;
impl JTAG_SW_RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTAG_SW_RST_A {
        match self.bits {
            false => JTAG_SW_RST_A::JTAG_SW_RST_0,
            true => JTAG_SW_RST_A::JTAG_SW_RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `JTAG_SW_RST_0`"]
    #[inline(always)]
    pub fn is_jtag_sw_rst_0(&self) -> bool {
        *self == JTAG_SW_RST_A::JTAG_SW_RST_0
    }
    #[doc = "Checks if the value of the field is `JTAG_SW_RST_1`"]
    #[inline(always)]
    pub fn is_jtag_sw_rst_1(&self) -> bool {
        *self == JTAG_SW_RST_A::JTAG_SW_RST_1
    }
}
#[doc = "Write proxy for field `jtag_sw_rst`"]
pub struct JTAG_SW_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_SW_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JTAG_SW_RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset is not a result of software reset from JTAG."]
    #[inline(always)]
    pub fn jtag_sw_rst_0(self) -> &'a mut W {
        self.variant(JTAG_SW_RST_A::JTAG_SW_RST_0)
    }
    #[doc = "Reset is a result of software reset from JTAG."]
    #[inline(always)]
    pub fn jtag_sw_rst_1(self) -> &'a mut W {
        self.variant(JTAG_SW_RST_A::JTAG_SW_RST_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "IC Watchdog3 Time-out reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG3_RST_B_A {
    #[doc = "0: Reset is not a result of the watchdog3 time-out event."]
    WDOG3_RST_B_0 = 0,
    #[doc = "1: Reset is a result of the watchdog3 time-out event."]
    WDOG3_RST_B_1 = 1,
}
impl From<WDOG3_RST_B_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG3_RST_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `wdog3_rst_b`"]
pub type WDOG3_RST_B_R = crate::R<bool, WDOG3_RST_B_A>;
impl WDOG3_RST_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG3_RST_B_A {
        match self.bits {
            false => WDOG3_RST_B_A::WDOG3_RST_B_0,
            true => WDOG3_RST_B_A::WDOG3_RST_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG3_RST_B_0`"]
    #[inline(always)]
    pub fn is_wdog3_rst_b_0(&self) -> bool {
        *self == WDOG3_RST_B_A::WDOG3_RST_B_0
    }
    #[doc = "Checks if the value of the field is `WDOG3_RST_B_1`"]
    #[inline(always)]
    pub fn is_wdog3_rst_b_1(&self) -> bool {
        *self == WDOG3_RST_B_A::WDOG3_RST_B_1
    }
}
#[doc = "Write proxy for field `wdog3_rst_b`"]
pub struct WDOG3_RST_B_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG3_RST_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG3_RST_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset is not a result of the watchdog3 time-out event."]
    #[inline(always)]
    pub fn wdog3_rst_b_0(self) -> &'a mut W {
        self.variant(WDOG3_RST_B_A::WDOG3_RST_B_0)
    }
    #[doc = "Reset is a result of the watchdog3 time-out event."]
    #[inline(always)]
    pub fn wdog3_rst_b_1(self) -> &'a mut W {
        self.variant(WDOG3_RST_B_A::WDOG3_RST_B_1)
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
#[doc = "Temper Sensor software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPSENSE_RST_B_A {
    #[doc = "0: Reset is not a result of software reset from Temperature Sensor."]
    TEMPSENSE_RST_B_0 = 0,
    #[doc = "1: Reset is a result of software reset from Temperature Sensor."]
    TEMPSENSE_RST_B_1 = 1,
}
impl From<TEMPSENSE_RST_B_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPSENSE_RST_B_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `tempsense_rst_b`"]
pub type TEMPSENSE_RST_B_R = crate::R<bool, TEMPSENSE_RST_B_A>;
impl TEMPSENSE_RST_B_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMPSENSE_RST_B_A {
        match self.bits {
            false => TEMPSENSE_RST_B_A::TEMPSENSE_RST_B_0,
            true => TEMPSENSE_RST_B_A::TEMPSENSE_RST_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEMPSENSE_RST_B_0`"]
    #[inline(always)]
    pub fn is_tempsense_rst_b_0(&self) -> bool {
        *self == TEMPSENSE_RST_B_A::TEMPSENSE_RST_B_0
    }
    #[doc = "Checks if the value of the field is `TEMPSENSE_RST_B_1`"]
    #[inline(always)]
    pub fn is_tempsense_rst_b_1(&self) -> bool {
        *self == TEMPSENSE_RST_B_A::TEMPSENSE_RST_B_1
    }
}
#[doc = "Write proxy for field `tempsense_rst_b`"]
pub struct TEMPSENSE_RST_B_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPSENSE_RST_B_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEMPSENSE_RST_B_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset is not a result of software reset from Temperature Sensor."]
    #[inline(always)]
    pub fn tempsense_rst_b_0(self) -> &'a mut W {
        self.variant(TEMPSENSE_RST_B_A::TEMPSENSE_RST_B_0)
    }
    #[doc = "Reset is a result of software reset from Temperature Sensor."]
    #[inline(always)]
    pub fn tempsense_rst_b_1(self) -> &'a mut W {
        self.variant(TEMPSENSE_RST_B_A::TEMPSENSE_RST_B_1)
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
impl R {
    #[doc = "Bit 0 - Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
    #[inline(always)]
    pub fn ipp_reset_b(&self) -> IPP_RESET_B_R {
        IPP_RESET_B_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates a reset has been caused by CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core"]
    #[inline(always)]
    pub fn lockup_sysresetreq(&self) -> LOCKUP_SYSRESETREQ_R {
        LOCKUP_SYSRESETREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates whether the reset was the result of the csu_reset_b input."]
    #[inline(always)]
    pub fn csu_reset_b(&self) -> CSU_RESET_B_R {
        CSU_RESET_B_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
    #[inline(always)]
    pub fn ipp_user_reset_b(&self) -> IPP_USER_RESET_B_R {
        IPP_USER_RESET_B_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IC Watchdog Time-out reset"]
    #[inline(always)]
    pub fn wdog_rst_b(&self) -> WDOG_RST_B_R {
        WDOG_RST_B_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
    #[inline(always)]
    pub fn jtag_rst_b(&self) -> JTAG_RST_B_R {
        JTAG_RST_B_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - JTAG software reset. Indicates whether the reset was the result of software reset from JTAG."]
    #[inline(always)]
    pub fn jtag_sw_rst(&self) -> JTAG_SW_RST_R {
        JTAG_SW_RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IC Watchdog3 Time-out reset"]
    #[inline(always)]
    pub fn wdog3_rst_b(&self) -> WDOG3_RST_B_R {
        WDOG3_RST_B_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Temper Sensor software reset"]
    #[inline(always)]
    pub fn tempsense_rst_b(&self) -> TEMPSENSE_RST_B_R {
        TEMPSENSE_RST_B_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
    #[inline(always)]
    pub fn ipp_reset_b(&mut self) -> IPP_RESET_B_W {
        IPP_RESET_B_W { w: self }
    }
    #[doc = "Bit 1 - Indicates a reset has been caused by CPU lockup or software setting of SYSRESETREQ bit in Application Interrupt and Reset Control Register of the ARM core"]
    #[inline(always)]
    pub fn lockup_sysresetreq(&mut self) -> LOCKUP_SYSRESETREQ_W {
        LOCKUP_SYSRESETREQ_W { w: self }
    }
    #[doc = "Bit 2 - Indicates whether the reset was the result of the csu_reset_b input."]
    #[inline(always)]
    pub fn csu_reset_b(&mut self) -> CSU_RESET_B_W {
        CSU_RESET_B_W { w: self }
    }
    #[doc = "Bit 3 - Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
    #[inline(always)]
    pub fn ipp_user_reset_b(&mut self) -> IPP_USER_RESET_B_W {
        IPP_USER_RESET_B_W { w: self }
    }
    #[doc = "Bit 4 - IC Watchdog Time-out reset"]
    #[inline(always)]
    pub fn wdog_rst_b(&mut self) -> WDOG_RST_B_W {
        WDOG_RST_B_W { w: self }
    }
    #[doc = "Bit 5 - HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
    #[inline(always)]
    pub fn jtag_rst_b(&mut self) -> JTAG_RST_B_W {
        JTAG_RST_B_W { w: self }
    }
    #[doc = "Bit 6 - JTAG software reset. Indicates whether the reset was the result of software reset from JTAG."]
    #[inline(always)]
    pub fn jtag_sw_rst(&mut self) -> JTAG_SW_RST_W {
        JTAG_SW_RST_W { w: self }
    }
    #[doc = "Bit 7 - IC Watchdog3 Time-out reset"]
    #[inline(always)]
    pub fn wdog3_rst_b(&mut self) -> WDOG3_RST_B_W {
        WDOG3_RST_B_W { w: self }
    }
    #[doc = "Bit 8 - Temper Sensor software reset"]
    #[inline(always)]
    pub fn tempsense_rst_b(&mut self) -> TEMPSENSE_RST_B_W {
        TEMPSENSE_RST_B_W { w: self }
    }
}
