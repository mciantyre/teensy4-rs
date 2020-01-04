#[doc = "Reader of register ACTLR"]
pub type R = crate::R<u32, super::ACTLR>;
#[doc = "Writer for register ACTLR"]
pub type W = crate::W<u32, super::ACTLR>;
#[doc = "Register ACTLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ACTLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Disables folding of IT instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISFOLD_A {
    #[doc = "0: Normal operation."]
    DISFOLD_0 = 0,
}
impl From<DISFOLD_A> for bool {
    #[inline(always)]
    fn from(variant: DISFOLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISFOLD`"]
pub type DISFOLD_R = crate::R<bool, DISFOLD_A>;
impl DISFOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DISFOLD_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DISFOLD_A::DISFOLD_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISFOLD_0`"]
    #[inline(always)]
    pub fn is_disfold_0(&self) -> bool {
        *self == DISFOLD_A::DISFOLD_0
    }
}
#[doc = "Write proxy for field `DISFOLD`"]
pub struct DISFOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISFOLD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disfold_0(self) -> &'a mut W {
        self.variant(DISFOLD_A::DISFOLD_0)
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
#[doc = "Disables FPU exception outputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPEXCODIS_A {
    #[doc = "0: Normal operation."]
    FPEXCODIS_0 = 0,
    #[doc = "1: FPU exception outputs are disabled."]
    FPEXCODIS_1 = 1,
}
impl From<FPEXCODIS_A> for bool {
    #[inline(always)]
    fn from(variant: FPEXCODIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FPEXCODIS`"]
pub type FPEXCODIS_R = crate::R<bool, FPEXCODIS_A>;
impl FPEXCODIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPEXCODIS_A {
        match self.bits {
            false => FPEXCODIS_A::FPEXCODIS_0,
            true => FPEXCODIS_A::FPEXCODIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `FPEXCODIS_0`"]
    #[inline(always)]
    pub fn is_fpexcodis_0(&self) -> bool {
        *self == FPEXCODIS_A::FPEXCODIS_0
    }
    #[doc = "Checks if the value of the field is `FPEXCODIS_1`"]
    #[inline(always)]
    pub fn is_fpexcodis_1(&self) -> bool {
        *self == FPEXCODIS_A::FPEXCODIS_1
    }
}
#[doc = "Write proxy for field `FPEXCODIS`"]
pub struct FPEXCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FPEXCODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPEXCODIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn fpexcodis_0(self) -> &'a mut W {
        self.variant(FPEXCODIS_A::FPEXCODIS_0)
    }
    #[doc = "FPU exception outputs are disabled."]
    #[inline(always)]
    pub fn fpexcodis_1(self) -> &'a mut W {
        self.variant(FPEXCODIS_A::FPEXCODIS_1)
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
#[doc = "Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISRAMODE_A {
    #[doc = "0: Normal operation."]
    DISRAMODE_0 = 0,
    #[doc = "1: Dynamic disabled."]
    DISRAMODE_1 = 1,
}
impl From<DISRAMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DISRAMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISRAMODE`"]
pub type DISRAMODE_R = crate::R<bool, DISRAMODE_A>;
impl DISRAMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISRAMODE_A {
        match self.bits {
            false => DISRAMODE_A::DISRAMODE_0,
            true => DISRAMODE_A::DISRAMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISRAMODE_0`"]
    #[inline(always)]
    pub fn is_disramode_0(&self) -> bool {
        *self == DISRAMODE_A::DISRAMODE_0
    }
    #[doc = "Checks if the value of the field is `DISRAMODE_1`"]
    #[inline(always)]
    pub fn is_disramode_1(&self) -> bool {
        *self == DISRAMODE_A::DISRAMODE_1
    }
}
#[doc = "Write proxy for field `DISRAMODE`"]
pub struct DISRAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISRAMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISRAMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disramode_0(self) -> &'a mut W {
        self.variant(DISRAMODE_A::DISRAMODE_0)
    }
    #[doc = "Dynamic disabled."]
    #[inline(always)]
    pub fn disramode_1(self) -> &'a mut W {
        self.variant(DISRAMODE_A::DISRAMODE_1)
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
#[doc = "Disables ITM and DWT ATB flush.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISITMATBFLUSH_A {
    #[doc = "1: ITM and DWT ATB flush disabled, this bit is always 1."]
    DISITMATBFLUSH_1 = 1,
}
impl From<DISITMATBFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: DISITMATBFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISITMATBFLUSH`"]
pub type DISITMATBFLUSH_R = crate::R<bool, DISITMATBFLUSH_A>;
impl DISITMATBFLUSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DISITMATBFLUSH_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(DISITMATBFLUSH_A::DISITMATBFLUSH_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISITMATBFLUSH_1`"]
    #[inline(always)]
    pub fn is_disitmatbflush_1(&self) -> bool {
        *self == DISITMATBFLUSH_A::DISITMATBFLUSH_1
    }
}
#[doc = "Write proxy for field `DISITMATBFLUSH`"]
pub struct DISITMATBFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> DISITMATBFLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISITMATBFLUSH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ITM and DWT ATB flush disabled, this bit is always 1."]
    #[inline(always)]
    pub fn disitmatbflush_1(self) -> &'a mut W {
        self.variant(DISITMATBFLUSH_A::DISITMATBFLUSH_1)
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
#[doc = "Disables BTAC read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISBTACREAD_A {
    #[doc = "0: Normal operation."]
    DISBTACREAD_0 = 0,
    #[doc = "1: BTAC is not used and only static branch prediction can occur."]
    DISBTACREAD_1 = 1,
}
impl From<DISBTACREAD_A> for bool {
    #[inline(always)]
    fn from(variant: DISBTACREAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISBTACREAD`"]
pub type DISBTACREAD_R = crate::R<bool, DISBTACREAD_A>;
impl DISBTACREAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISBTACREAD_A {
        match self.bits {
            false => DISBTACREAD_A::DISBTACREAD_0,
            true => DISBTACREAD_A::DISBTACREAD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISBTACREAD_0`"]
    #[inline(always)]
    pub fn is_disbtacread_0(&self) -> bool {
        *self == DISBTACREAD_A::DISBTACREAD_0
    }
    #[doc = "Checks if the value of the field is `DISBTACREAD_1`"]
    #[inline(always)]
    pub fn is_disbtacread_1(&self) -> bool {
        *self == DISBTACREAD_A::DISBTACREAD_1
    }
}
#[doc = "Write proxy for field `DISBTACREAD`"]
pub struct DISBTACREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISBTACREAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISBTACREAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disbtacread_0(self) -> &'a mut W {
        self.variant(DISBTACREAD_A::DISBTACREAD_0)
    }
    #[doc = "BTAC is not used and only static branch prediction can occur."]
    #[inline(always)]
    pub fn disbtacread_1(self) -> &'a mut W {
        self.variant(DISBTACREAD_A::DISBTACREAD_1)
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
#[doc = "Disables BTAC allocate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISBTACALLOC_A {
    #[doc = "0: Normal operation."]
    DISBTACALLOC_0 = 0,
    #[doc = "1: No new entries are allocated in Branch Target Address Cache (BTAC), but existing entries can be updated."]
    DISBTACALLOC_1 = 1,
}
impl From<DISBTACALLOC_A> for bool {
    #[inline(always)]
    fn from(variant: DISBTACALLOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISBTACALLOC`"]
pub type DISBTACALLOC_R = crate::R<bool, DISBTACALLOC_A>;
impl DISBTACALLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISBTACALLOC_A {
        match self.bits {
            false => DISBTACALLOC_A::DISBTACALLOC_0,
            true => DISBTACALLOC_A::DISBTACALLOC_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISBTACALLOC_0`"]
    #[inline(always)]
    pub fn is_disbtacalloc_0(&self) -> bool {
        *self == DISBTACALLOC_A::DISBTACALLOC_0
    }
    #[doc = "Checks if the value of the field is `DISBTACALLOC_1`"]
    #[inline(always)]
    pub fn is_disbtacalloc_1(&self) -> bool {
        *self == DISBTACALLOC_A::DISBTACALLOC_1
    }
}
#[doc = "Write proxy for field `DISBTACALLOC`"]
pub struct DISBTACALLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> DISBTACALLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISBTACALLOC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disbtacalloc_0(self) -> &'a mut W {
        self.variant(DISBTACALLOC_A::DISBTACALLOC_0)
    }
    #[doc = "No new entries are allocated in Branch Target Address Cache (BTAC), but existing entries can be updated."]
    #[inline(always)]
    pub fn disbtacalloc_1(self) -> &'a mut W {
        self.variant(DISBTACALLOC_A::DISBTACALLOC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Disables critical AXI Read-Under-Read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCRITAXIRUR_A {
    #[doc = "0: Normal operation."]
    DISCRITAXIRUR_0 = 0,
    #[doc = "1: An AXI read to Strongly-Ordered or Device memory, or an LDREX to Shareable memory, is not put on AXI if there are any outstanding reads on AXI. Transactions on AXI cannot be interrupted. This bit might reduce the time that these transactions are in progress and might improve worst case interrupt latency. Performance is decreased when this bit is set."]
    DISCRITAXIRUR_1 = 1,
}
impl From<DISCRITAXIRUR_A> for bool {
    #[inline(always)]
    fn from(variant: DISCRITAXIRUR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISCRITAXIRUR`"]
pub type DISCRITAXIRUR_R = crate::R<bool, DISCRITAXIRUR_A>;
impl DISCRITAXIRUR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCRITAXIRUR_A {
        match self.bits {
            false => DISCRITAXIRUR_A::DISCRITAXIRUR_0,
            true => DISCRITAXIRUR_A::DISCRITAXIRUR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISCRITAXIRUR_0`"]
    #[inline(always)]
    pub fn is_discritaxirur_0(&self) -> bool {
        *self == DISCRITAXIRUR_A::DISCRITAXIRUR_0
    }
    #[doc = "Checks if the value of the field is `DISCRITAXIRUR_1`"]
    #[inline(always)]
    pub fn is_discritaxirur_1(&self) -> bool {
        *self == DISCRITAXIRUR_A::DISCRITAXIRUR_1
    }
}
#[doc = "Write proxy for field `DISCRITAXIRUR`"]
pub struct DISCRITAXIRUR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCRITAXIRUR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISCRITAXIRUR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn discritaxirur_0(self) -> &'a mut W {
        self.variant(DISCRITAXIRUR_A::DISCRITAXIRUR_0)
    }
    #[doc = "An AXI read to Strongly-Ordered or Device memory, or an LDREX to Shareable memory, is not put on AXI if there are any outstanding reads on AXI. Transactions on AXI cannot be interrupted. This bit might reduce the time that these transactions are in progress and might improve worst case interrupt latency. Performance is decreased when this bit is set."]
    #[inline(always)]
    pub fn discritaxirur_1(self) -> &'a mut W {
        self.variant(DISCRITAXIRUR_A::DISCRITAXIRUR_1)
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
#[doc = "Disables dual-issued.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DISDI_A {
    #[doc = "0: Normal operation."]
    DISDI_0 = 0,
    #[doc = "1: Nothing can be dual-issued when this instruction type is in channel 0."]
    DISDI_1 = 1,
}
impl From<DISDI_A> for u8 {
    #[inline(always)]
    fn from(variant: DISDI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DISDI`"]
pub type DISDI_R = crate::R<u8, DISDI_A>;
impl DISDI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DISDI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DISDI_A::DISDI_0),
            1 => Val(DISDI_A::DISDI_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISDI_0`"]
    #[inline(always)]
    pub fn is_disdi_0(&self) -> bool {
        *self == DISDI_A::DISDI_0
    }
    #[doc = "Checks if the value of the field is `DISDI_1`"]
    #[inline(always)]
    pub fn is_disdi_1(&self) -> bool {
        *self == DISDI_A::DISDI_1
    }
}
#[doc = "Write proxy for field `DISDI`"]
pub struct DISDI_W<'a> {
    w: &'a mut W,
}
impl<'a> DISDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISDI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disdi_0(self) -> &'a mut W {
        self.variant(DISDI_A::DISDI_0)
    }
    #[doc = "Nothing can be dual-issued when this instruction type is in channel 0."]
    #[inline(always)]
    pub fn disdi_1(self) -> &'a mut W {
        self.variant(DISDI_A::DISDI_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Disables dual-issued.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DISISSCH1_A {
    #[doc = "0: Normal operation."]
    DISISSCH1_0 = 0,
    #[doc = "1: Nothing can be dual-issued when this instruction type is in channel 1."]
    DISISSCH1_1 = 1,
}
impl From<DISISSCH1_A> for u8 {
    #[inline(always)]
    fn from(variant: DISISSCH1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DISISSCH1`"]
pub type DISISSCH1_R = crate::R<u8, DISISSCH1_A>;
impl DISISSCH1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DISISSCH1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DISISSCH1_A::DISISSCH1_0),
            1 => Val(DISISSCH1_A::DISISSCH1_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISISSCH1_0`"]
    #[inline(always)]
    pub fn is_disissch1_0(&self) -> bool {
        *self == DISISSCH1_A::DISISSCH1_0
    }
    #[doc = "Checks if the value of the field is `DISISSCH1_1`"]
    #[inline(always)]
    pub fn is_disissch1_1(&self) -> bool {
        *self == DISISSCH1_A::DISISSCH1_1
    }
}
#[doc = "Write proxy for field `DISISSCH1`"]
pub struct DISISSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DISISSCH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISISSCH1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disissch1_0(self) -> &'a mut W {
        self.variant(DISISSCH1_A::DISISSCH1_0)
    }
    #[doc = "Nothing can be dual-issued when this instruction type is in channel 1."]
    #[inline(always)]
    pub fn disissch1_1(self) -> &'a mut W {
        self.variant(DISISSCH1_A::DISISSCH1_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | (((value as u32) & 0x1f) << 21);
        self.w
    }
}
#[doc = "Disables dynamic allocation of ADD and SUB instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISDYNADD_A {
    #[doc = "0: Normal operation. Some ADD and SUB instrctions are resolved in EX1."]
    DISDYNADD_0 = 0,
    #[doc = "1: All ADD and SUB instructions are resolved in EX2."]
    DISDYNADD_1 = 1,
}
impl From<DISDYNADD_A> for bool {
    #[inline(always)]
    fn from(variant: DISDYNADD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISDYNADD`"]
pub type DISDYNADD_R = crate::R<bool, DISDYNADD_A>;
impl DISDYNADD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISDYNADD_A {
        match self.bits {
            false => DISDYNADD_A::DISDYNADD_0,
            true => DISDYNADD_A::DISDYNADD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISDYNADD_0`"]
    #[inline(always)]
    pub fn is_disdynadd_0(&self) -> bool {
        *self == DISDYNADD_A::DISDYNADD_0
    }
    #[doc = "Checks if the value of the field is `DISDYNADD_1`"]
    #[inline(always)]
    pub fn is_disdynadd_1(&self) -> bool {
        *self == DISDYNADD_A::DISDYNADD_1
    }
}
#[doc = "Write proxy for field `DISDYNADD`"]
pub struct DISDYNADD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISDYNADD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISDYNADD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation. Some ADD and SUB instrctions are resolved in EX1."]
    #[inline(always)]
    pub fn disdynadd_0(self) -> &'a mut W {
        self.variant(DISDYNADD_A::DISDYNADD_0)
    }
    #[doc = "All ADD and SUB instructions are resolved in EX2."]
    #[inline(always)]
    pub fn disdynadd_1(self) -> &'a mut W {
        self.variant(DISDYNADD_A::DISDYNADD_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Disables critical AXI read-under-write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCRITAXIRUW_A {
    #[doc = "0: Normal operation. This is backwards compatible with r0."]
    DISCRITAXIRUW_0 = 0,
    #[doc = "1: AXI reads to DEV/SO memory. Exclusive reads to Shareable memory are not initiated on the AXIM AR channel until all outstanding stores on AXI are complete."]
    DISCRITAXIRUW_1 = 1,
}
impl From<DISCRITAXIRUW_A> for bool {
    #[inline(always)]
    fn from(variant: DISCRITAXIRUW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISCRITAXIRUW`"]
pub type DISCRITAXIRUW_R = crate::R<bool, DISCRITAXIRUW_A>;
impl DISCRITAXIRUW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCRITAXIRUW_A {
        match self.bits {
            false => DISCRITAXIRUW_A::DISCRITAXIRUW_0,
            true => DISCRITAXIRUW_A::DISCRITAXIRUW_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISCRITAXIRUW_0`"]
    #[inline(always)]
    pub fn is_discritaxiruw_0(&self) -> bool {
        *self == DISCRITAXIRUW_A::DISCRITAXIRUW_0
    }
    #[doc = "Checks if the value of the field is `DISCRITAXIRUW_1`"]
    #[inline(always)]
    pub fn is_discritaxiruw_1(&self) -> bool {
        *self == DISCRITAXIRUW_A::DISCRITAXIRUW_1
    }
}
#[doc = "Write proxy for field `DISCRITAXIRUW`"]
pub struct DISCRITAXIRUW_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCRITAXIRUW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISCRITAXIRUW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation. This is backwards compatible with r0."]
    #[inline(always)]
    pub fn discritaxiruw_0(self) -> &'a mut W {
        self.variant(DISCRITAXIRUW_A::DISCRITAXIRUW_0)
    }
    #[doc = "AXI reads to DEV/SO memory. Exclusive reads to Shareable memory are not initiated on the AXIM AR channel until all outstanding stores on AXI are complete."]
    #[inline(always)]
    pub fn discritaxiruw_1(self) -> &'a mut W {
        self.variant(DISCRITAXIRUW_A::DISCRITAXIRUW_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Disables critical AXI read-under-write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISFPUISSOPT_A {
    #[doc = "0: Normal operation."]
    DISFPUISSOPT_0 = 0,
}
impl From<DISFPUISSOPT_A> for bool {
    #[inline(always)]
    fn from(variant: DISFPUISSOPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISFPUISSOPT`"]
pub type DISFPUISSOPT_R = crate::R<bool, DISFPUISSOPT_A>;
impl DISFPUISSOPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DISFPUISSOPT_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DISFPUISSOPT_A::DISFPUISSOPT_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISFPUISSOPT_0`"]
    #[inline(always)]
    pub fn is_disfpuissopt_0(&self) -> bool {
        *self == DISFPUISSOPT_A::DISFPUISSOPT_0
    }
}
#[doc = "Write proxy for field `DISFPUISSOPT`"]
pub struct DISFPUISSOPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFPUISSOPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISFPUISSOPT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn disfpuissopt_0(self) -> &'a mut W {
        self.variant(DISFPUISSOPT_A::DISFPUISSOPT_0)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Disables folding of IT instructions."]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Disables FPU exception outputs."]
    #[inline(always)]
    pub fn fpexcodis(&self) -> FPEXCODIS_R {
        FPEXCODIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions."]
    #[inline(always)]
    pub fn disramode(&self) -> DISRAMODE_R {
        DISRAMODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Disables ITM and DWT ATB flush."]
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DISITMATBFLUSH_R {
        DISITMATBFLUSH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Disables BTAC read."]
    #[inline(always)]
    pub fn disbtacread(&self) -> DISBTACREAD_R {
        DISBTACREAD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Disables BTAC allocate."]
    #[inline(always)]
    pub fn disbtacalloc(&self) -> DISBTACALLOC_R {
        DISBTACALLOC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Disables critical AXI Read-Under-Read."]
    #[inline(always)]
    pub fn discritaxirur(&self) -> DISCRITAXIRUR_R {
        DISCRITAXIRUR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Disables dual-issued."]
    #[inline(always)]
    pub fn disdi(&self) -> DISDI_R {
        DISDI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Disables dual-issued."]
    #[inline(always)]
    pub fn disissch1(&self) -> DISISSCH1_R {
        DISISSCH1_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disdynadd(&self) -> DISDYNADD_R {
        DISDYNADD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Disables critical AXI read-under-write"]
    #[inline(always)]
    pub fn discritaxiruw(&self) -> DISCRITAXIRUW_R {
        DISCRITAXIRUW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Disables critical AXI read-under-write"]
    #[inline(always)]
    pub fn disfpuissopt(&self) -> DISFPUISSOPT_R {
        DISFPUISSOPT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Disables folding of IT instructions."]
    #[inline(always)]
    pub fn disfold(&mut self) -> DISFOLD_W {
        DISFOLD_W { w: self }
    }
    #[doc = "Bit 10 - Disables FPU exception outputs."]
    #[inline(always)]
    pub fn fpexcodis(&mut self) -> FPEXCODIS_W {
        FPEXCODIS_W { w: self }
    }
    #[doc = "Bit 11 - Disables dynamic read allocate mode for Write-Back Write-Allocate memory regions."]
    #[inline(always)]
    pub fn disramode(&mut self) -> DISRAMODE_W {
        DISRAMODE_W { w: self }
    }
    #[doc = "Bit 12 - Disables ITM and DWT ATB flush."]
    #[inline(always)]
    pub fn disitmatbflush(&mut self) -> DISITMATBFLUSH_W {
        DISITMATBFLUSH_W { w: self }
    }
    #[doc = "Bit 13 - Disables BTAC read."]
    #[inline(always)]
    pub fn disbtacread(&mut self) -> DISBTACREAD_W {
        DISBTACREAD_W { w: self }
    }
    #[doc = "Bit 14 - Disables BTAC allocate."]
    #[inline(always)]
    pub fn disbtacalloc(&mut self) -> DISBTACALLOC_W {
        DISBTACALLOC_W { w: self }
    }
    #[doc = "Bit 15 - Disables critical AXI Read-Under-Read."]
    #[inline(always)]
    pub fn discritaxirur(&mut self) -> DISCRITAXIRUR_W {
        DISCRITAXIRUR_W { w: self }
    }
    #[doc = "Bits 16:20 - Disables dual-issued."]
    #[inline(always)]
    pub fn disdi(&mut self) -> DISDI_W {
        DISDI_W { w: self }
    }
    #[doc = "Bits 21:25 - Disables dual-issued."]
    #[inline(always)]
    pub fn disissch1(&mut self) -> DISISSCH1_W {
        DISISSCH1_W { w: self }
    }
    #[doc = "Bit 26 - Disables dynamic allocation of ADD and SUB instructions"]
    #[inline(always)]
    pub fn disdynadd(&mut self) -> DISDYNADD_W {
        DISDYNADD_W { w: self }
    }
    #[doc = "Bit 27 - Disables critical AXI read-under-write"]
    #[inline(always)]
    pub fn discritaxiruw(&mut self) -> DISCRITAXIRUW_W {
        DISCRITAXIRUW_W { w: self }
    }
    #[doc = "Bit 28 - Disables critical AXI read-under-write"]
    #[inline(always)]
    pub fn disfpuissopt(&mut self) -> DISFPUISSOPT_W {
        DISFPUISSOPT_W { w: self }
    }
}
