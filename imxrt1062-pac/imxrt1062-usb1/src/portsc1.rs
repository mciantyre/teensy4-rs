#[doc = "Reader of register PORTSC1"]
pub type R = crate::R<u32, super::PORTSC1>;
#[doc = "Writer for register PORTSC1"]
pub type W = crate::W<u32, super::PORTSC1>;
#[doc = "Register PORTSC1 `reset()`'s with value 0x1000_0000"]
impl crate::ResetValue for super::PORTSC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000_0000
    }
}
#[doc = "Reader of field `CCS`"]
pub type CCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSC`"]
pub type CSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSC`"]
pub struct CSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSC_W<'a> {
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
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
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
#[doc = "Reader of field `PEC`"]
pub type PEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEC`"]
pub struct PEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEC_W<'a> {
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
#[doc = "Over-current Active-Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCA_A {
    #[doc = "0: This port does not have an over-current condition."]
    OCA_0 = 0,
    #[doc = "1: This port currently has an over-current condition"]
    OCA_1 = 1,
}
impl From<OCA_A> for bool {
    #[inline(always)]
    fn from(variant: OCA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OCA`"]
pub type OCA_R = crate::R<bool, OCA_A>;
impl OCA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCA_A {
        match self.bits {
            false => OCA_A::OCA_0,
            true => OCA_A::OCA_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCA_0`"]
    #[inline(always)]
    pub fn is_oca_0(&self) -> bool {
        *self == OCA_A::OCA_0
    }
    #[doc = "Checks if the value of the field is `OCA_1`"]
    #[inline(always)]
    pub fn is_oca_1(&self) -> bool {
        *self == OCA_A::OCA_1
    }
}
#[doc = "Reader of field `OCC`"]
pub type OCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCC`"]
pub struct OCC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCC_W<'a> {
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
#[doc = "Reader of field `FPR`"]
pub type FPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPR`"]
pub struct FPR_W<'a> {
    w: &'a mut W,
}
impl<'a> FPR_W<'a> {
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
#[doc = "Reader of field `SUSP`"]
pub type SUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSP`"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
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
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PR`"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
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
#[doc = "Reader of field `HSP`"]
pub type HSP_R = crate::R<bool, bool>;
#[doc = "Line Status-Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LS_A {
    #[doc = "0: SE0"]
    LS_0 = 0,
    #[doc = "1: K-state"]
    LS_1 = 1,
    #[doc = "2: J-state"]
    LS_2 = 2,
    #[doc = "3: Undefined"]
    LS_3 = 3,
}
impl From<LS_A> for u8 {
    #[inline(always)]
    fn from(variant: LS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LS`"]
pub type LS_R = crate::R<u8, LS_A>;
impl LS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LS_A {
        match self.bits {
            0 => LS_A::LS_0,
            1 => LS_A::LS_1,
            2 => LS_A::LS_2,
            3 => LS_A::LS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LS_0`"]
    #[inline(always)]
    pub fn is_ls_0(&self) -> bool {
        *self == LS_A::LS_0
    }
    #[doc = "Checks if the value of the field is `LS_1`"]
    #[inline(always)]
    pub fn is_ls_1(&self) -> bool {
        *self == LS_A::LS_1
    }
    #[doc = "Checks if the value of the field is `LS_2`"]
    #[inline(always)]
    pub fn is_ls_2(&self) -> bool {
        *self == LS_A::LS_2
    }
    #[doc = "Checks if the value of the field is `LS_3`"]
    #[inline(always)]
    pub fn is_ls_3(&self) -> bool {
        *self == LS_A::LS_3
    }
}
#[doc = "Write proxy for field `LS`"]
pub struct LS_W<'a> {
    w: &'a mut W,
}
impl<'a> LS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SE0"]
    #[inline(always)]
    pub fn ls_0(self) -> &'a mut W {
        self.variant(LS_A::LS_0)
    }
    #[doc = "K-state"]
    #[inline(always)]
    pub fn ls_1(self) -> &'a mut W {
        self.variant(LS_A::LS_1)
    }
    #[doc = "J-state"]
    #[inline(always)]
    pub fn ls_2(self) -> &'a mut W {
        self.variant(LS_A::LS_2)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn ls_3(self) -> &'a mut W {
        self.variant(LS_A::LS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `PP`"]
pub type PP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PP`"]
pub struct PP_W<'a> {
    w: &'a mut W,
}
impl<'a> PP_W<'a> {
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
#[doc = "Reader of field `PO`"]
pub type PO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PO`"]
pub struct PO_W<'a> {
    w: &'a mut W,
}
impl<'a> PO_W<'a> {
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
#[doc = "Port Indicator Control - Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PIC_A {
    #[doc = "0: Port indicators are off"]
    PIC_0 = 0,
    #[doc = "1: Amber"]
    PIC_1 = 1,
    #[doc = "2: Green"]
    PIC_2 = 2,
    #[doc = "3: Undefined"]
    PIC_3 = 3,
}
impl From<PIC_A> for u8 {
    #[inline(always)]
    fn from(variant: PIC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PIC`"]
pub type PIC_R = crate::R<u8, PIC_A>;
impl PIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIC_A {
        match self.bits {
            0 => PIC_A::PIC_0,
            1 => PIC_A::PIC_1,
            2 => PIC_A::PIC_2,
            3 => PIC_A::PIC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIC_0`"]
    #[inline(always)]
    pub fn is_pic_0(&self) -> bool {
        *self == PIC_A::PIC_0
    }
    #[doc = "Checks if the value of the field is `PIC_1`"]
    #[inline(always)]
    pub fn is_pic_1(&self) -> bool {
        *self == PIC_A::PIC_1
    }
    #[doc = "Checks if the value of the field is `PIC_2`"]
    #[inline(always)]
    pub fn is_pic_2(&self) -> bool {
        *self == PIC_A::PIC_2
    }
    #[doc = "Checks if the value of the field is `PIC_3`"]
    #[inline(always)]
    pub fn is_pic_3(&self) -> bool {
        *self == PIC_A::PIC_3
    }
}
#[doc = "Write proxy for field `PIC`"]
pub struct PIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PIC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Port indicators are off"]
    #[inline(always)]
    pub fn pic_0(self) -> &'a mut W {
        self.variant(PIC_A::PIC_0)
    }
    #[doc = "Amber"]
    #[inline(always)]
    pub fn pic_1(self) -> &'a mut W {
        self.variant(PIC_A::PIC_1)
    }
    #[doc = "Green"]
    #[inline(always)]
    pub fn pic_2(self) -> &'a mut W {
        self.variant(PIC_A::PIC_2)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn pic_3(self) -> &'a mut W {
        self.variant(PIC_A::PIC_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Port Test Control - Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PTC_A {
    #[doc = "0: TEST_MODE_DISABLE"]
    PTC_0 = 0,
    #[doc = "1: J_STATE"]
    PTC_1 = 1,
    #[doc = "2: K_STATE"]
    PTC_2 = 2,
    #[doc = "3: SE0 (host) / NAK (device)"]
    PTC_3 = 3,
    #[doc = "4: Packet"]
    PTC_4 = 4,
    #[doc = "5: FORCE_ENABLE_HS"]
    PTC_5 = 5,
    #[doc = "6: FORCE_ENABLE_FS"]
    PTC_6 = 6,
    #[doc = "7: FORCE_ENABLE_LS"]
    PTC_7 = 7,
}
impl From<PTC_A> for u8 {
    #[inline(always)]
    fn from(variant: PTC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PTC`"]
pub type PTC_R = crate::R<u8, PTC_A>;
impl PTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PTC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PTC_A::PTC_0),
            1 => Val(PTC_A::PTC_1),
            2 => Val(PTC_A::PTC_2),
            3 => Val(PTC_A::PTC_3),
            4 => Val(PTC_A::PTC_4),
            5 => Val(PTC_A::PTC_5),
            6 => Val(PTC_A::PTC_6),
            7 => Val(PTC_A::PTC_7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PTC_0`"]
    #[inline(always)]
    pub fn is_ptc_0(&self) -> bool {
        *self == PTC_A::PTC_0
    }
    #[doc = "Checks if the value of the field is `PTC_1`"]
    #[inline(always)]
    pub fn is_ptc_1(&self) -> bool {
        *self == PTC_A::PTC_1
    }
    #[doc = "Checks if the value of the field is `PTC_2`"]
    #[inline(always)]
    pub fn is_ptc_2(&self) -> bool {
        *self == PTC_A::PTC_2
    }
    #[doc = "Checks if the value of the field is `PTC_3`"]
    #[inline(always)]
    pub fn is_ptc_3(&self) -> bool {
        *self == PTC_A::PTC_3
    }
    #[doc = "Checks if the value of the field is `PTC_4`"]
    #[inline(always)]
    pub fn is_ptc_4(&self) -> bool {
        *self == PTC_A::PTC_4
    }
    #[doc = "Checks if the value of the field is `PTC_5`"]
    #[inline(always)]
    pub fn is_ptc_5(&self) -> bool {
        *self == PTC_A::PTC_5
    }
    #[doc = "Checks if the value of the field is `PTC_6`"]
    #[inline(always)]
    pub fn is_ptc_6(&self) -> bool {
        *self == PTC_A::PTC_6
    }
    #[doc = "Checks if the value of the field is `PTC_7`"]
    #[inline(always)]
    pub fn is_ptc_7(&self) -> bool {
        *self == PTC_A::PTC_7
    }
}
#[doc = "Write proxy for field `PTC`"]
pub struct PTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TEST_MODE_DISABLE"]
    #[inline(always)]
    pub fn ptc_0(self) -> &'a mut W {
        self.variant(PTC_A::PTC_0)
    }
    #[doc = "J_STATE"]
    #[inline(always)]
    pub fn ptc_1(self) -> &'a mut W {
        self.variant(PTC_A::PTC_1)
    }
    #[doc = "K_STATE"]
    #[inline(always)]
    pub fn ptc_2(self) -> &'a mut W {
        self.variant(PTC_A::PTC_2)
    }
    #[doc = "SE0 (host) / NAK (device)"]
    #[inline(always)]
    pub fn ptc_3(self) -> &'a mut W {
        self.variant(PTC_A::PTC_3)
    }
    #[doc = "Packet"]
    #[inline(always)]
    pub fn ptc_4(self) -> &'a mut W {
        self.variant(PTC_A::PTC_4)
    }
    #[doc = "FORCE_ENABLE_HS"]
    #[inline(always)]
    pub fn ptc_5(self) -> &'a mut W {
        self.variant(PTC_A::PTC_5)
    }
    #[doc = "FORCE_ENABLE_FS"]
    #[inline(always)]
    pub fn ptc_6(self) -> &'a mut W {
        self.variant(PTC_A::PTC_6)
    }
    #[doc = "FORCE_ENABLE_LS"]
    #[inline(always)]
    pub fn ptc_7(self) -> &'a mut W {
        self.variant(PTC_A::PTC_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `WKCN`"]
pub type WKCN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKCN`"]
pub struct WKCN_W<'a> {
    w: &'a mut W,
}
impl<'a> WKCN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `WKDC`"]
pub type WKDC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKDC`"]
pub struct WKDC_W<'a> {
    w: &'a mut W,
}
impl<'a> WKDC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `WKOC`"]
pub type WKOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKOC`"]
pub struct WKOC_W<'a> {
    w: &'a mut W,
}
impl<'a> WKOC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHCD_A {
    #[doc = "0: Enable PHY clock"]
    PHCD_0 = 0,
    #[doc = "1: Disable PHY clock"]
    PHCD_1 = 1,
}
impl From<PHCD_A> for bool {
    #[inline(always)]
    fn from(variant: PHCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHCD`"]
pub type PHCD_R = crate::R<bool, PHCD_A>;
impl PHCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHCD_A {
        match self.bits {
            false => PHCD_A::PHCD_0,
            true => PHCD_A::PHCD_1,
        }
    }
    #[doc = "Checks if the value of the field is `PHCD_0`"]
    #[inline(always)]
    pub fn is_phcd_0(&self) -> bool {
        *self == PHCD_A::PHCD_0
    }
    #[doc = "Checks if the value of the field is `PHCD_1`"]
    #[inline(always)]
    pub fn is_phcd_1(&self) -> bool {
        *self == PHCD_A::PHCD_1
    }
}
#[doc = "Write proxy for field `PHCD`"]
pub struct PHCD_W<'a> {
    w: &'a mut W,
}
impl<'a> PHCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHCD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable PHY clock"]
    #[inline(always)]
    pub fn phcd_0(self) -> &'a mut W {
        self.variant(PHCD_A::PHCD_0)
    }
    #[doc = "Disable PHY clock"]
    #[inline(always)]
    pub fn phcd_1(self) -> &'a mut W {
        self.variant(PHCD_A::PHCD_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Port Force Full Speed Connect - Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFSC_A {
    #[doc = "0: Normal operation"]
    PFSC_0 = 0,
    #[doc = "1: Forced to full speed"]
    PFSC_1 = 1,
}
impl From<PFSC_A> for bool {
    #[inline(always)]
    fn from(variant: PFSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PFSC`"]
pub type PFSC_R = crate::R<bool, PFSC_A>;
impl PFSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFSC_A {
        match self.bits {
            false => PFSC_A::PFSC_0,
            true => PFSC_A::PFSC_1,
        }
    }
    #[doc = "Checks if the value of the field is `PFSC_0`"]
    #[inline(always)]
    pub fn is_pfsc_0(&self) -> bool {
        *self == PFSC_A::PFSC_0
    }
    #[doc = "Checks if the value of the field is `PFSC_1`"]
    #[inline(always)]
    pub fn is_pfsc_1(&self) -> bool {
        *self == PFSC_A::PFSC_1
    }
}
#[doc = "Write proxy for field `PFSC`"]
pub struct PFSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PFSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PFSC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn pfsc_0(self) -> &'a mut W {
        self.variant(PFSC_A::PFSC_0)
    }
    #[doc = "Forced to full speed"]
    #[inline(always)]
    pub fn pfsc_1(self) -> &'a mut W {
        self.variant(PFSC_A::PFSC_1)
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
#[doc = "Reader of field `PTS_2`"]
pub type PTS_2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTS_2`"]
pub struct PTS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PTS_2_W<'a> {
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
#[doc = "Port Speed - Read Only. This register field indicates the speed at which the port is operating.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSPD_A {
    #[doc = "0: Full Speed"]
    PSPD_0 = 0,
    #[doc = "1: Low Speed"]
    PSPD_1 = 1,
    #[doc = "2: High Speed"]
    PSPD_2 = 2,
    #[doc = "3: Undefined"]
    PSPD_3 = 3,
}
impl From<PSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: PSPD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSPD`"]
pub type PSPD_R = crate::R<u8, PSPD_A>;
impl PSPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSPD_A {
        match self.bits {
            0 => PSPD_A::PSPD_0,
            1 => PSPD_A::PSPD_1,
            2 => PSPD_A::PSPD_2,
            3 => PSPD_A::PSPD_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSPD_0`"]
    #[inline(always)]
    pub fn is_pspd_0(&self) -> bool {
        *self == PSPD_A::PSPD_0
    }
    #[doc = "Checks if the value of the field is `PSPD_1`"]
    #[inline(always)]
    pub fn is_pspd_1(&self) -> bool {
        *self == PSPD_A::PSPD_1
    }
    #[doc = "Checks if the value of the field is `PSPD_2`"]
    #[inline(always)]
    pub fn is_pspd_2(&self) -> bool {
        *self == PSPD_A::PSPD_2
    }
    #[doc = "Checks if the value of the field is `PSPD_3`"]
    #[inline(always)]
    pub fn is_pspd_3(&self) -> bool {
        *self == PSPD_A::PSPD_3
    }
}
#[doc = "Write proxy for field `PSPD`"]
pub struct PSPD_W<'a> {
    w: &'a mut W,
}
impl<'a> PSPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSPD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Full Speed"]
    #[inline(always)]
    pub fn pspd_0(self) -> &'a mut W {
        self.variant(PSPD_A::PSPD_0)
    }
    #[doc = "Low Speed"]
    #[inline(always)]
    pub fn pspd_1(self) -> &'a mut W {
        self.variant(PSPD_A::PSPD_1)
    }
    #[doc = "High Speed"]
    #[inline(always)]
    pub fn pspd_2(self) -> &'a mut W {
        self.variant(PSPD_A::PSPD_2)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn pspd_3(self) -> &'a mut W {
        self.variant(PSPD_A::PSPD_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Parallel Transceiver Width This bit has no effect if serial interface engine is used\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTW_A {
    #[doc = "0: Select the 8-bit UTMI interface \\[60MHz\\]"]
    PTW_0 = 0,
    #[doc = "1: Select the 16-bit UTMI interface \\[30MHz\\]"]
    PTW_1 = 1,
}
impl From<PTW_A> for bool {
    #[inline(always)]
    fn from(variant: PTW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTW`"]
pub type PTW_R = crate::R<bool, PTW_A>;
impl PTW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTW_A {
        match self.bits {
            false => PTW_A::PTW_0,
            true => PTW_A::PTW_1,
        }
    }
    #[doc = "Checks if the value of the field is `PTW_0`"]
    #[inline(always)]
    pub fn is_ptw_0(&self) -> bool {
        *self == PTW_A::PTW_0
    }
    #[doc = "Checks if the value of the field is `PTW_1`"]
    #[inline(always)]
    pub fn is_ptw_1(&self) -> bool {
        *self == PTW_A::PTW_1
    }
}
#[doc = "Write proxy for field `PTW`"]
pub struct PTW_W<'a> {
    w: &'a mut W,
}
impl<'a> PTW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Select the 8-bit UTMI interface \\[60MHz\\]"]
    #[inline(always)]
    pub fn ptw_0(self) -> &'a mut W {
        self.variant(PTW_A::PTW_0)
    }
    #[doc = "Select the 16-bit UTMI interface \\[30MHz\\]"]
    #[inline(always)]
    pub fn ptw_1(self) -> &'a mut W {
        self.variant(PTW_A::PTW_1)
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
#[doc = "Reader of field `STS`"]
pub type STS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STS`"]
pub struct STS_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `PTS_1`"]
pub type PTS_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PTS_1`"]
pub struct PTS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PTS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Current Connect Status-Read Only"]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Connect Status Change-R/WC"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled-Read/Write"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change-R/WC"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Over-current Active-Read Only"]
    #[inline(always)]
    pub fn oca(&self) -> OCA_R {
        OCA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Over-current Change-R/WC"]
    #[inline(always)]
    pub fn occ(&self) -> OCC_R {
        OCC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Force Port Resume -Read/Write"]
    #[inline(always)]
    pub fn fpr(&self) -> FPR_R {
        FPR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Suspend - Read/Write or Read Only"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Reset - Read/Write or Read Only"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - High-Speed Port - Read Only"]
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Line Status-Read Only"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Port Power (PP)-Read/Write or Read Only"]
    #[inline(always)]
    pub fn pp(&self) -> PP_R {
        PP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Owner-Read/Write"]
    #[inline(always)]
    pub fn po(&self) -> PO_R {
        PO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Port Indicator Control - Read/Write"]
    #[inline(always)]
    pub fn pic(&self) -> PIC_R {
        PIC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Port Test Control - Read/Write"]
    #[inline(always)]
    pub fn ptc(&self) -> PTC_R {
        PTC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Wake on Connect Enable (WKCNNT_E) - Read/Write"]
    #[inline(always)]
    pub fn wkcn(&self) -> WKCN_R {
        WKCN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write"]
    #[inline(always)]
    pub fn wkdc(&self) -> WKDC_R {
        WKDC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Wake on Over-current Enable (WKOC_E) - Read/Write"]
    #[inline(always)]
    pub fn wkoc(&self) -> WKOC_R {
        WKOC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write"]
    #[inline(always)]
    pub fn phcd(&self) -> PHCD_R {
        PHCD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Force Full Speed Connect - Read/Write"]
    #[inline(always)]
    pub fn pfsc(&self) -> PFSC_R {
        PFSC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - See description at bits 31-30"]
    #[inline(always)]
    pub fn pts_2(&self) -> PTS_2_R {
        PTS_2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - Port Speed - Read Only. This register field indicates the speed at which the port is operating."]
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Parallel Transceiver Width This bit has no effect if serial interface engine is used"]
    #[inline(always)]
    pub fn ptw(&self) -> PTW_R {
        PTW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - All USB port interface modes are listed in this field description, but not all are supported"]
    #[inline(always)]
    pub fn pts_1(&self) -> PTS_1_R {
        PTS_1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Connect Status Change-R/WC"]
    #[inline(always)]
    pub fn csc(&mut self) -> CSC_W {
        CSC_W { w: self }
    }
    #[doc = "Bit 2 - Port Enabled/Disabled-Read/Write"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bit 3 - Port Enable/Disable Change-R/WC"]
    #[inline(always)]
    pub fn pec(&mut self) -> PEC_W {
        PEC_W { w: self }
    }
    #[doc = "Bit 5 - Over-current Change-R/WC"]
    #[inline(always)]
    pub fn occ(&mut self) -> OCC_W {
        OCC_W { w: self }
    }
    #[doc = "Bit 6 - Force Port Resume -Read/Write"]
    #[inline(always)]
    pub fn fpr(&mut self) -> FPR_W {
        FPR_W { w: self }
    }
    #[doc = "Bit 7 - Suspend - Read/Write or Read Only"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Bit 8 - Port Reset - Read/Write or Read Only"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
    #[doc = "Bits 10:11 - Line Status-Read Only"]
    #[inline(always)]
    pub fn ls(&mut self) -> LS_W {
        LS_W { w: self }
    }
    #[doc = "Bit 12 - Port Power (PP)-Read/Write or Read Only"]
    #[inline(always)]
    pub fn pp(&mut self) -> PP_W {
        PP_W { w: self }
    }
    #[doc = "Bit 13 - Port Owner-Read/Write"]
    #[inline(always)]
    pub fn po(&mut self) -> PO_W {
        PO_W { w: self }
    }
    #[doc = "Bits 14:15 - Port Indicator Control - Read/Write"]
    #[inline(always)]
    pub fn pic(&mut self) -> PIC_W {
        PIC_W { w: self }
    }
    #[doc = "Bits 16:19 - Port Test Control - Read/Write"]
    #[inline(always)]
    pub fn ptc(&mut self) -> PTC_W {
        PTC_W { w: self }
    }
    #[doc = "Bit 20 - Wake on Connect Enable (WKCNNT_E) - Read/Write"]
    #[inline(always)]
    pub fn wkcn(&mut self) -> WKCN_W {
        WKCN_W { w: self }
    }
    #[doc = "Bit 21 - Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write"]
    #[inline(always)]
    pub fn wkdc(&mut self) -> WKDC_W {
        WKDC_W { w: self }
    }
    #[doc = "Bit 22 - Wake on Over-current Enable (WKOC_E) - Read/Write"]
    #[inline(always)]
    pub fn wkoc(&mut self) -> WKOC_W {
        WKOC_W { w: self }
    }
    #[doc = "Bit 23 - PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write"]
    #[inline(always)]
    pub fn phcd(&mut self) -> PHCD_W {
        PHCD_W { w: self }
    }
    #[doc = "Bit 24 - Port Force Full Speed Connect - Read/Write"]
    #[inline(always)]
    pub fn pfsc(&mut self) -> PFSC_W {
        PFSC_W { w: self }
    }
    #[doc = "Bit 25 - See description at bits 31-30"]
    #[inline(always)]
    pub fn pts_2(&mut self) -> PTS_2_W {
        PTS_2_W { w: self }
    }
    #[doc = "Bits 26:27 - Port Speed - Read Only. This register field indicates the speed at which the port is operating."]
    #[inline(always)]
    pub fn pspd(&mut self) -> PSPD_W {
        PSPD_W { w: self }
    }
    #[doc = "Bit 28 - Parallel Transceiver Width This bit has no effect if serial interface engine is used"]
    #[inline(always)]
    pub fn ptw(&mut self) -> PTW_W {
        PTW_W { w: self }
    }
    #[doc = "Bit 29 - Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals"]
    #[inline(always)]
    pub fn sts(&mut self) -> STS_W {
        STS_W { w: self }
    }
    #[doc = "Bits 30:31 - All USB port interface modes are listed in this field description, but not all are supported"]
    #[inline(always)]
    pub fn pts_1(&mut self) -> PTS_1_W {
        PTS_1_W { w: self }
    }
}
