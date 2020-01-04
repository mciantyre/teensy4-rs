#[doc = "Reader of register WCR"]
pub type R = crate::R<u16, super::WCR>;
#[doc = "Writer for register WCR"]
pub type W = crate::W<u16, super::WCR>;
#[doc = "Register WCR `reset()`'s with value 0x30"]
impl crate::ResetValue for super::WCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30
    }
}
#[doc = "WDZST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDZST_A {
    #[doc = "0: Continue timer operation (Default)."]
    WDZST_0 = 0,
    #[doc = "1: Suspend the watchdog timer."]
    WDZST_1 = 1,
}
impl From<WDZST_A> for bool {
    #[inline(always)]
    fn from(variant: WDZST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDZST`"]
pub type WDZST_R = crate::R<bool, WDZST_A>;
impl WDZST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDZST_A {
        match self.bits {
            false => WDZST_A::WDZST_0,
            true => WDZST_A::WDZST_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDZST_0`"]
    #[inline(always)]
    pub fn is_wdzst_0(&self) -> bool {
        *self == WDZST_A::WDZST_0
    }
    #[doc = "Checks if the value of the field is `WDZST_1`"]
    #[inline(always)]
    pub fn is_wdzst_1(&self) -> bool {
        *self == WDZST_A::WDZST_1
    }
}
#[doc = "Write proxy for field `WDZST`"]
pub struct WDZST_W<'a> {
    w: &'a mut W,
}
impl<'a> WDZST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDZST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Continue timer operation (Default)."]
    #[inline(always)]
    pub fn wdzst_0(self) -> &'a mut W {
        self.variant(WDZST_A::WDZST_0)
    }
    #[doc = "Suspend the watchdog timer."]
    #[inline(always)]
    pub fn wdzst_1(self) -> &'a mut W {
        self.variant(WDZST_A::WDZST_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "WDBG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDBG_A {
    #[doc = "0: Continue WDOG timer operation (Default)."]
    WDBG_0 = 0,
    #[doc = "1: Suspend the watchdog timer."]
    WDBG_1 = 1,
}
impl From<WDBG_A> for bool {
    #[inline(always)]
    fn from(variant: WDBG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDBG`"]
pub type WDBG_R = crate::R<bool, WDBG_A>;
impl WDBG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDBG_A {
        match self.bits {
            false => WDBG_A::WDBG_0,
            true => WDBG_A::WDBG_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDBG_0`"]
    #[inline(always)]
    pub fn is_wdbg_0(&self) -> bool {
        *self == WDBG_A::WDBG_0
    }
    #[doc = "Checks if the value of the field is `WDBG_1`"]
    #[inline(always)]
    pub fn is_wdbg_1(&self) -> bool {
        *self == WDBG_A::WDBG_1
    }
}
#[doc = "Write proxy for field `WDBG`"]
pub struct WDBG_W<'a> {
    w: &'a mut W,
}
impl<'a> WDBG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDBG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Continue WDOG timer operation (Default)."]
    #[inline(always)]
    pub fn wdbg_0(self) -> &'a mut W {
        self.variant(WDBG_A::WDBG_0)
    }
    #[doc = "Suspend the watchdog timer."]
    #[inline(always)]
    pub fn wdbg_1(self) -> &'a mut W {
        self.variant(WDBG_A::WDBG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "WDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDE_A {
    #[doc = "0: Disable the Watchdog (Default)."]
    WDE_0 = 0,
    #[doc = "1: Enable the Watchdog."]
    WDE_1 = 1,
}
impl From<WDE_A> for bool {
    #[inline(always)]
    fn from(variant: WDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDE`"]
pub type WDE_R = crate::R<bool, WDE_A>;
impl WDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDE_A {
        match self.bits {
            false => WDE_A::WDE_0,
            true => WDE_A::WDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDE_0`"]
    #[inline(always)]
    pub fn is_wde_0(&self) -> bool {
        *self == WDE_A::WDE_0
    }
    #[doc = "Checks if the value of the field is `WDE_1`"]
    #[inline(always)]
    pub fn is_wde_1(&self) -> bool {
        *self == WDE_A::WDE_1
    }
}
#[doc = "Write proxy for field `WDE`"]
pub struct WDE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the Watchdog (Default)."]
    #[inline(always)]
    pub fn wde_0(self) -> &'a mut W {
        self.variant(WDE_A::WDE_0)
    }
    #[doc = "Enable the Watchdog."]
    #[inline(always)]
    pub fn wde_1(self) -> &'a mut W {
        self.variant(WDE_A::WDE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "WDT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_A {
    #[doc = "0: No effect on WDOG_B (Default)."]
    WDT_0 = 0,
    #[doc = "1: Assert WDOG_B upon a Watchdog Time-out event."]
    WDT_1 = 1,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDT`"]
pub type WDT_R = crate::R<bool, WDT_A>;
impl WDT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::WDT_0,
            true => WDT_A::WDT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDT_0`"]
    #[inline(always)]
    pub fn is_wdt_0(&self) -> bool {
        *self == WDT_A::WDT_0
    }
    #[doc = "Checks if the value of the field is `WDT_1`"]
    #[inline(always)]
    pub fn is_wdt_1(&self) -> bool {
        *self == WDT_A::WDT_1
    }
}
#[doc = "Write proxy for field `WDT`"]
pub struct WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect on WDOG_B (Default)."]
    #[inline(always)]
    pub fn wdt_0(self) -> &'a mut W {
        self.variant(WDT_A::WDT_0)
    }
    #[doc = "Assert WDOG_B upon a Watchdog Time-out event."]
    #[inline(always)]
    pub fn wdt_1(self) -> &'a mut W {
        self.variant(WDT_A::WDT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "SRS\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRS_A {
    #[doc = "0: Assert system reset signal."]
    SRS_0 = 0,
    #[doc = "1: No effect on the system (Default)."]
    SRS_1 = 1,
}
impl From<SRS_A> for bool {
    #[inline(always)]
    fn from(variant: SRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRS`"]
pub type SRS_R = crate::R<bool, SRS_A>;
impl SRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRS_A {
        match self.bits {
            false => SRS_A::SRS_0,
            true => SRS_A::SRS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRS_0`"]
    #[inline(always)]
    pub fn is_srs_0(&self) -> bool {
        *self == SRS_A::SRS_0
    }
    #[doc = "Checks if the value of the field is `SRS_1`"]
    #[inline(always)]
    pub fn is_srs_1(&self) -> bool {
        *self == SRS_A::SRS_1
    }
}
#[doc = "Write proxy for field `SRS`"]
pub struct SRS_W<'a> {
    w: &'a mut W,
}
impl<'a> SRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert system reset signal."]
    #[inline(always)]
    pub fn srs_0(self) -> &'a mut W {
        self.variant(SRS_A::SRS_0)
    }
    #[doc = "No effect on the system (Default)."]
    #[inline(always)]
    pub fn srs_1(self) -> &'a mut W {
        self.variant(SRS_A::SRS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "WDA\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDA_A {
    #[doc = "0: Assert WDOG_B output."]
    WDA_0 = 0,
    #[doc = "1: No effect on system (Default)."]
    WDA_1 = 1,
}
impl From<WDA_A> for bool {
    #[inline(always)]
    fn from(variant: WDA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDA`"]
pub type WDA_R = crate::R<bool, WDA_A>;
impl WDA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDA_A {
        match self.bits {
            false => WDA_A::WDA_0,
            true => WDA_A::WDA_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDA_0`"]
    #[inline(always)]
    pub fn is_wda_0(&self) -> bool {
        *self == WDA_A::WDA_0
    }
    #[doc = "Checks if the value of the field is `WDA_1`"]
    #[inline(always)]
    pub fn is_wda_1(&self) -> bool {
        *self == WDA_A::WDA_1
    }
}
#[doc = "Write proxy for field `WDA`"]
pub struct WDA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert WDOG_B output."]
    #[inline(always)]
    pub fn wda_0(self) -> &'a mut W {
        self.variant(WDA_A::WDA_0)
    }
    #[doc = "No effect on system (Default)."]
    #[inline(always)]
    pub fn wda_1(self) -> &'a mut W {
        self.variant(WDA_A::WDA_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "software reset extension, an option way to generate software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRE_A {
    #[doc = "0: using original way to generate software reset (default)"]
    SRE_0 = 0,
    #[doc = "1: using new way to generate software reset."]
    SRE_1 = 1,
}
impl From<SRE_A> for bool {
    #[inline(always)]
    fn from(variant: SRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRE`"]
pub type SRE_R = crate::R<bool, SRE_A>;
impl SRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRE_A {
        match self.bits {
            false => SRE_A::SRE_0,
            true => SRE_A::SRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRE_0`"]
    #[inline(always)]
    pub fn is_sre_0(&self) -> bool {
        *self == SRE_A::SRE_0
    }
    #[doc = "Checks if the value of the field is `SRE_1`"]
    #[inline(always)]
    pub fn is_sre_1(&self) -> bool {
        *self == SRE_A::SRE_1
    }
}
#[doc = "Write proxy for field `SRE`"]
pub struct SRE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "using original way to generate software reset (default)"]
    #[inline(always)]
    pub fn sre_0(self) -> &'a mut W {
        self.variant(SRE_A::SRE_0)
    }
    #[doc = "using new way to generate software reset."]
    #[inline(always)]
    pub fn sre_1(self) -> &'a mut W {
        self.variant(SRE_A::SRE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "WDW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDW_A {
    #[doc = "0: Continue WDOG timer operation (Default)."]
    WDW_0 = 0,
    #[doc = "1: Suspend WDOG timer operation."]
    WDW_1 = 1,
}
impl From<WDW_A> for bool {
    #[inline(always)]
    fn from(variant: WDW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDW`"]
pub type WDW_R = crate::R<bool, WDW_A>;
impl WDW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDW_A {
        match self.bits {
            false => WDW_A::WDW_0,
            true => WDW_A::WDW_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDW_0`"]
    #[inline(always)]
    pub fn is_wdw_0(&self) -> bool {
        *self == WDW_A::WDW_0
    }
    #[doc = "Checks if the value of the field is `WDW_1`"]
    #[inline(always)]
    pub fn is_wdw_1(&self) -> bool {
        *self == WDW_A::WDW_1
    }
}
#[doc = "Write proxy for field `WDW`"]
pub struct WDW_W<'a> {
    w: &'a mut W,
}
impl<'a> WDW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Continue WDOG timer operation (Default)."]
    #[inline(always)]
    pub fn wdw_0(self) -> &'a mut W {
        self.variant(WDW_A::WDW_0)
    }
    #[doc = "Suspend WDOG timer operation."]
    #[inline(always)]
    pub fn wdw_1(self) -> &'a mut W {
        self.variant(WDW_A::WDW_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "WT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WT_A {
    #[doc = "0: - 0.5 Seconds (Default)."]
    WT_0 = 0,
    #[doc = "1: - 1.0 Seconds."]
    WT_1 = 1,
    #[doc = "2: - 1.5 Seconds."]
    WT_2 = 2,
    #[doc = "3: - 2.0 Seconds."]
    WT_3 = 3,
    #[doc = "255: - 128 Seconds."]
    WT_255 = 255,
}
impl From<WT_A> for u8 {
    #[inline(always)]
    fn from(variant: WT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WT`"]
pub type WT_R = crate::R<u8, WT_A>;
impl WT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WT_A::WT_0),
            1 => Val(WT_A::WT_1),
            2 => Val(WT_A::WT_2),
            3 => Val(WT_A::WT_3),
            255 => Val(WT_A::WT_255),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WT_0`"]
    #[inline(always)]
    pub fn is_wt_0(&self) -> bool {
        *self == WT_A::WT_0
    }
    #[doc = "Checks if the value of the field is `WT_1`"]
    #[inline(always)]
    pub fn is_wt_1(&self) -> bool {
        *self == WT_A::WT_1
    }
    #[doc = "Checks if the value of the field is `WT_2`"]
    #[inline(always)]
    pub fn is_wt_2(&self) -> bool {
        *self == WT_A::WT_2
    }
    #[doc = "Checks if the value of the field is `WT_3`"]
    #[inline(always)]
    pub fn is_wt_3(&self) -> bool {
        *self == WT_A::WT_3
    }
    #[doc = "Checks if the value of the field is `WT_255`"]
    #[inline(always)]
    pub fn is_wt_255(&self) -> bool {
        *self == WT_A::WT_255
    }
}
#[doc = "Write proxy for field `WT`"]
pub struct WT_W<'a> {
    w: &'a mut W,
}
impl<'a> WT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "- 0.5 Seconds (Default)."]
    #[inline(always)]
    pub fn wt_0(self) -> &'a mut W {
        self.variant(WT_A::WT_0)
    }
    #[doc = "- 1.0 Seconds."]
    #[inline(always)]
    pub fn wt_1(self) -> &'a mut W {
        self.variant(WT_A::WT_1)
    }
    #[doc = "- 1.5 Seconds."]
    #[inline(always)]
    pub fn wt_2(self) -> &'a mut W {
        self.variant(WT_A::WT_2)
    }
    #[doc = "- 2.0 Seconds."]
    #[inline(always)]
    pub fn wt_3(self) -> &'a mut W {
        self.variant(WT_A::WT_3)
    }
    #[doc = "- 128 Seconds."]
    #[inline(always)]
    pub fn wt_255(self) -> &'a mut W {
        self.variant(WT_A::WT_255)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - WDZST"]
    #[inline(always)]
    pub fn wdzst(&self) -> WDZST_R {
        WDZST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WDBG"]
    #[inline(always)]
    pub fn wdbg(&self) -> WDBG_R {
        WDBG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WDE"]
    #[inline(always)]
    pub fn wde(&self) -> WDE_R {
        WDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WDT"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SRS"]
    #[inline(always)]
    pub fn srs(&self) -> SRS_R {
        SRS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WDA"]
    #[inline(always)]
    pub fn wda(&self) -> WDA_R {
        WDA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - software reset extension, an option way to generate software reset"]
    #[inline(always)]
    pub fn sre(&self) -> SRE_R {
        SRE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - WDW"]
    #[inline(always)]
    pub fn wdw(&self) -> WDW_R {
        WDW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - WT"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - WDZST"]
    #[inline(always)]
    pub fn wdzst(&mut self) -> WDZST_W {
        WDZST_W { w: self }
    }
    #[doc = "Bit 1 - WDBG"]
    #[inline(always)]
    pub fn wdbg(&mut self) -> WDBG_W {
        WDBG_W { w: self }
    }
    #[doc = "Bit 2 - WDE"]
    #[inline(always)]
    pub fn wde(&mut self) -> WDE_W {
        WDE_W { w: self }
    }
    #[doc = "Bit 3 - WDT"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W {
        WDT_W { w: self }
    }
    #[doc = "Bit 4 - SRS"]
    #[inline(always)]
    pub fn srs(&mut self) -> SRS_W {
        SRS_W { w: self }
    }
    #[doc = "Bit 5 - WDA"]
    #[inline(always)]
    pub fn wda(&mut self) -> WDA_W {
        WDA_W { w: self }
    }
    #[doc = "Bit 6 - software reset extension, an option way to generate software reset"]
    #[inline(always)]
    pub fn sre(&mut self) -> SRE_W {
        SRE_W { w: self }
    }
    #[doc = "Bit 7 - WDW"]
    #[inline(always)]
    pub fn wdw(&mut self) -> WDW_W {
        WDW_W { w: self }
    }
    #[doc = "Bits 8:15 - WT"]
    #[inline(always)]
    pub fn wt(&mut self) -> WT_W {
        WT_W { w: self }
    }
}
