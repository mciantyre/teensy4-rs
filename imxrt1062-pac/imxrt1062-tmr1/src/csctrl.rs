#[doc = "Reader of register CSCTRL%s"]
pub type R = crate::R<u16, super::CSCTRL>;
#[doc = "Writer for register CSCTRL%s"]
pub type W = crate::W<u16, super::CSCTRL>;
#[doc = "Register CSCTRL%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Compare Load Control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CL1_A {
    #[doc = "0: Never preload"]
    CL1_0 = 0,
    #[doc = "1: Load upon successful compare with the value in COMP1"]
    CL1_1 = 1,
    #[doc = "2: Load upon successful compare with the value in COMP2"]
    CL1_2 = 2,
}
impl From<CL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CL1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CL1`"]
pub type CL1_R = crate::R<u8, CL1_A>;
impl CL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CL1_A::CL1_0),
            1 => Val(CL1_A::CL1_1),
            2 => Val(CL1_A::CL1_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL1_0`"]
    #[inline(always)]
    pub fn is_cl1_0(&self) -> bool {
        *self == CL1_A::CL1_0
    }
    #[doc = "Checks if the value of the field is `CL1_1`"]
    #[inline(always)]
    pub fn is_cl1_1(&self) -> bool {
        *self == CL1_A::CL1_1
    }
    #[doc = "Checks if the value of the field is `CL1_2`"]
    #[inline(always)]
    pub fn is_cl1_2(&self) -> bool {
        *self == CL1_A::CL1_2
    }
}
#[doc = "Write proxy for field `CL1`"]
pub struct CL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Never preload"]
    #[inline(always)]
    pub fn cl1_0(self) -> &'a mut W {
        self.variant(CL1_A::CL1_0)
    }
    #[doc = "Load upon successful compare with the value in COMP1"]
    #[inline(always)]
    pub fn cl1_1(self) -> &'a mut W {
        self.variant(CL1_A::CL1_1)
    }
    #[doc = "Load upon successful compare with the value in COMP2"]
    #[inline(always)]
    pub fn cl1_2(self) -> &'a mut W {
        self.variant(CL1_A::CL1_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Compare Load Control 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CL2_A {
    #[doc = "0: Never preload"]
    CL2_0 = 0,
    #[doc = "1: Load upon successful compare with the value in COMP1"]
    CL2_1 = 1,
    #[doc = "2: Load upon successful compare with the value in COMP2"]
    CL2_2 = 2,
}
impl From<CL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CL2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CL2`"]
pub type CL2_R = crate::R<u8, CL2_A>;
impl CL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CL2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CL2_A::CL2_0),
            1 => Val(CL2_A::CL2_1),
            2 => Val(CL2_A::CL2_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CL2_0`"]
    #[inline(always)]
    pub fn is_cl2_0(&self) -> bool {
        *self == CL2_A::CL2_0
    }
    #[doc = "Checks if the value of the field is `CL2_1`"]
    #[inline(always)]
    pub fn is_cl2_1(&self) -> bool {
        *self == CL2_A::CL2_1
    }
    #[doc = "Checks if the value of the field is `CL2_2`"]
    #[inline(always)]
    pub fn is_cl2_2(&self) -> bool {
        *self == CL2_A::CL2_2
    }
}
#[doc = "Write proxy for field `CL2`"]
pub struct CL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CL2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Never preload"]
    #[inline(always)]
    pub fn cl2_0(self) -> &'a mut W {
        self.variant(CL2_A::CL2_0)
    }
    #[doc = "Load upon successful compare with the value in COMP1"]
    #[inline(always)]
    pub fn cl2_1(self) -> &'a mut W {
        self.variant(CL2_A::CL2_1)
    }
    #[doc = "Load upon successful compare with the value in COMP2"]
    #[inline(always)]
    pub fn cl2_2(self) -> &'a mut W {
        self.variant(CL2_A::CL2_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TCF1`"]
pub type TCF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCF1`"]
pub struct TCF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF1_W<'a> {
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
#[doc = "Reader of field `TCF2`"]
pub type TCF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCF2`"]
pub struct TCF2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF2_W<'a> {
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
#[doc = "Reader of field `TCF1EN`"]
pub type TCF1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCF1EN`"]
pub struct TCF1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF1EN_W<'a> {
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
#[doc = "Reader of field `TCF2EN`"]
pub type TCF2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCF2EN`"]
pub struct TCF2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF2EN_W<'a> {
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
#[doc = "Counting Direction Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UP_A {
    #[doc = "0: The last count was in the DOWN direction."]
    UP_0 = 0,
    #[doc = "1: The last count was in the UP direction."]
    UP_1 = 1,
}
impl From<UP_A> for bool {
    #[inline(always)]
    fn from(variant: UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UP`"]
pub type UP_R = crate::R<bool, UP_A>;
impl UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UP_A {
        match self.bits {
            false => UP_A::UP_0,
            true => UP_A::UP_1,
        }
    }
    #[doc = "Checks if the value of the field is `UP_0`"]
    #[inline(always)]
    pub fn is_up_0(&self) -> bool {
        *self == UP_A::UP_0
    }
    #[doc = "Checks if the value of the field is `UP_1`"]
    #[inline(always)]
    pub fn is_up_1(&self) -> bool {
        *self == UP_A::UP_1
    }
}
#[doc = "Triggered Count Initialization Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCI_A {
    #[doc = "0: Stop counter upon receiving a second trigger event while still counting from the first trigger event."]
    TCI_0 = 0,
    #[doc = "1: Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
    TCI_1 = 1,
}
impl From<TCI_A> for bool {
    #[inline(always)]
    fn from(variant: TCI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCI`"]
pub type TCI_R = crate::R<bool, TCI_A>;
impl TCI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCI_A {
        match self.bits {
            false => TCI_A::TCI_0,
            true => TCI_A::TCI_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCI_0`"]
    #[inline(always)]
    pub fn is_tci_0(&self) -> bool {
        *self == TCI_A::TCI_0
    }
    #[doc = "Checks if the value of the field is `TCI_1`"]
    #[inline(always)]
    pub fn is_tci_1(&self) -> bool {
        *self == TCI_A::TCI_1
    }
}
#[doc = "Write proxy for field `TCI`"]
pub struct TCI_W<'a> {
    w: &'a mut W,
}
impl<'a> TCI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop counter upon receiving a second trigger event while still counting from the first trigger event."]
    #[inline(always)]
    pub fn tci_0(self) -> &'a mut W {
        self.variant(TCI_A::TCI_0)
    }
    #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
    #[inline(always)]
    pub fn tci_1(self) -> &'a mut W {
        self.variant(TCI_A::TCI_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reload on Capture\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROC_A {
    #[doc = "0: Do not reload the counter on a capture event."]
    ROC_0 = 0,
    #[doc = "1: Reload the counter on a capture event."]
    ROC_1 = 1,
}
impl From<ROC_A> for bool {
    #[inline(always)]
    fn from(variant: ROC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROC`"]
pub type ROC_R = crate::R<bool, ROC_A>;
impl ROC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROC_A {
        match self.bits {
            false => ROC_A::ROC_0,
            true => ROC_A::ROC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROC_0`"]
    #[inline(always)]
    pub fn is_roc_0(&self) -> bool {
        *self == ROC_A::ROC_0
    }
    #[doc = "Checks if the value of the field is `ROC_1`"]
    #[inline(always)]
    pub fn is_roc_1(&self) -> bool {
        *self == ROC_A::ROC_1
    }
}
#[doc = "Write proxy for field `ROC`"]
pub struct ROC_W<'a> {
    w: &'a mut W,
}
impl<'a> ROC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not reload the counter on a capture event."]
    #[inline(always)]
    pub fn roc_0(self) -> &'a mut W {
        self.variant(ROC_A::ROC_0)
    }
    #[doc = "Reload the counter on a capture event."]
    #[inline(always)]
    pub fn roc_1(self) -> &'a mut W {
        self.variant(ROC_A::ROC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Alternative Load Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALT_LOAD_A {
    #[doc = "0: Counter can be re-initialized only with the LOAD register."]
    ALT_LOAD_0 = 0,
    #[doc = "1: Counter can be re-initialized with the LOAD or CMPLD2 registers depending on count direction."]
    ALT_LOAD_1 = 1,
}
impl From<ALT_LOAD_A> for bool {
    #[inline(always)]
    fn from(variant: ALT_LOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALT_LOAD`"]
pub type ALT_LOAD_R = crate::R<bool, ALT_LOAD_A>;
impl ALT_LOAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALT_LOAD_A {
        match self.bits {
            false => ALT_LOAD_A::ALT_LOAD_0,
            true => ALT_LOAD_A::ALT_LOAD_1,
        }
    }
    #[doc = "Checks if the value of the field is `ALT_LOAD_0`"]
    #[inline(always)]
    pub fn is_alt_load_0(&self) -> bool {
        *self == ALT_LOAD_A::ALT_LOAD_0
    }
    #[doc = "Checks if the value of the field is `ALT_LOAD_1`"]
    #[inline(always)]
    pub fn is_alt_load_1(&self) -> bool {
        *self == ALT_LOAD_A::ALT_LOAD_1
    }
}
#[doc = "Write proxy for field `ALT_LOAD`"]
pub struct ALT_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> ALT_LOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALT_LOAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter can be re-initialized only with the LOAD register."]
    #[inline(always)]
    pub fn alt_load_0(self) -> &'a mut W {
        self.variant(ALT_LOAD_A::ALT_LOAD_0)
    }
    #[doc = "Counter can be re-initialized with the LOAD or CMPLD2 registers depending on count direction."]
    #[inline(always)]
    pub fn alt_load_1(self) -> &'a mut W {
        self.variant(ALT_LOAD_A::ALT_LOAD_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Fault Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT_A {
    #[doc = "0: Fault function disabled."]
    FAULT_0 = 0,
    #[doc = "1: Fault function enabled."]
    FAULT_1 = 1,
}
impl From<FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULT`"]
pub type FAULT_R = crate::R<bool, FAULT_A>;
impl FAULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT_A {
        match self.bits {
            false => FAULT_A::FAULT_0,
            true => FAULT_A::FAULT_1,
        }
    }
    #[doc = "Checks if the value of the field is `FAULT_0`"]
    #[inline(always)]
    pub fn is_fault_0(&self) -> bool {
        *self == FAULT_A::FAULT_0
    }
    #[doc = "Checks if the value of the field is `FAULT_1`"]
    #[inline(always)]
    pub fn is_fault_1(&self) -> bool {
        *self == FAULT_A::FAULT_1
    }
}
#[doc = "Write proxy for field `FAULT`"]
pub struct FAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault function disabled."]
    #[inline(always)]
    pub fn fault_0(self) -> &'a mut W {
        self.variant(FAULT_A::FAULT_0)
    }
    #[doc = "Fault function enabled."]
    #[inline(always)]
    pub fn fault_1(self) -> &'a mut W {
        self.variant(FAULT_A::FAULT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Debug Actions Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBG_EN_A {
    #[doc = "0: Continue with normal operation during debug mode. (default)"]
    DBG_EN_0 = 0,
    #[doc = "1: Halt TMR counter during debug mode."]
    DBG_EN_1 = 1,
    #[doc = "2: Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
    DBG_EN_2 = 2,
    #[doc = "3: Both halt counter and force output to 0 during debug mode."]
    DBG_EN_3 = 3,
}
impl From<DBG_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: DBG_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DBG_EN`"]
pub type DBG_EN_R = crate::R<u8, DBG_EN_A>;
impl DBG_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_EN_A {
        match self.bits {
            0 => DBG_EN_A::DBG_EN_0,
            1 => DBG_EN_A::DBG_EN_1,
            2 => DBG_EN_A::DBG_EN_2,
            3 => DBG_EN_A::DBG_EN_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DBG_EN_0`"]
    #[inline(always)]
    pub fn is_dbg_en_0(&self) -> bool {
        *self == DBG_EN_A::DBG_EN_0
    }
    #[doc = "Checks if the value of the field is `DBG_EN_1`"]
    #[inline(always)]
    pub fn is_dbg_en_1(&self) -> bool {
        *self == DBG_EN_A::DBG_EN_1
    }
    #[doc = "Checks if the value of the field is `DBG_EN_2`"]
    #[inline(always)]
    pub fn is_dbg_en_2(&self) -> bool {
        *self == DBG_EN_A::DBG_EN_2
    }
    #[doc = "Checks if the value of the field is `DBG_EN_3`"]
    #[inline(always)]
    pub fn is_dbg_en_3(&self) -> bool {
        *self == DBG_EN_A::DBG_EN_3
    }
}
#[doc = "Write proxy for field `DBG_EN`"]
pub struct DBG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_EN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Continue with normal operation during debug mode. (default)"]
    #[inline(always)]
    pub fn dbg_en_0(self) -> &'a mut W {
        self.variant(DBG_EN_A::DBG_EN_0)
    }
    #[doc = "Halt TMR counter during debug mode."]
    #[inline(always)]
    pub fn dbg_en_1(self) -> &'a mut W {
        self.variant(DBG_EN_A::DBG_EN_1)
    }
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
    #[inline(always)]
    pub fn dbg_en_2(self) -> &'a mut W {
        self.variant(DBG_EN_A::DBG_EN_2)
    }
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    #[inline(always)]
    pub fn dbg_en_3(self) -> &'a mut W {
        self.variant(DBG_EN_A::DBG_EN_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Compare Load Control 1"]
    #[inline(always)]
    pub fn cl1(&self) -> CL1_R {
        CL1_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Compare Load Control 2"]
    #[inline(always)]
    pub fn cl2(&self) -> CL2_R {
        CL2_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Timer Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn tcf1(&self) -> TCF1_R {
        TCF1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn tcf2(&self) -> TCF2_R {
        TCF2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub fn tcf1en(&self) -> TCF1EN_R {
        TCF1EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer Compare 2 Interrupt Enable"]
    #[inline(always)]
    pub fn tcf2en(&self) -> TCF2EN_R {
        TCF2EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counting Direction Indicator"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Triggered Count Initialization Control"]
    #[inline(always)]
    pub fn tci(&self) -> TCI_R {
        TCI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Reload on Capture"]
    #[inline(always)]
    pub fn roc(&self) -> ROC_R {
        ROC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Alternative Load Enable"]
    #[inline(always)]
    pub fn alt_load(&self) -> ALT_LOAD_R {
        ALT_LOAD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fault Enable"]
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Debug Actions Enable"]
    #[inline(always)]
    pub fn dbg_en(&self) -> DBG_EN_R {
        DBG_EN_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Compare Load Control 1"]
    #[inline(always)]
    pub fn cl1(&mut self) -> CL1_W {
        CL1_W { w: self }
    }
    #[doc = "Bits 2:3 - Compare Load Control 2"]
    #[inline(always)]
    pub fn cl2(&mut self) -> CL2_W {
        CL2_W { w: self }
    }
    #[doc = "Bit 4 - Timer Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn tcf1(&mut self) -> TCF1_W {
        TCF1_W { w: self }
    }
    #[doc = "Bit 5 - Timer Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn tcf2(&mut self) -> TCF2_W {
        TCF2_W { w: self }
    }
    #[doc = "Bit 6 - Timer Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub fn tcf1en(&mut self) -> TCF1EN_W {
        TCF1EN_W { w: self }
    }
    #[doc = "Bit 7 - Timer Compare 2 Interrupt Enable"]
    #[inline(always)]
    pub fn tcf2en(&mut self) -> TCF2EN_W {
        TCF2EN_W { w: self }
    }
    #[doc = "Bit 10 - Triggered Count Initialization Control"]
    #[inline(always)]
    pub fn tci(&mut self) -> TCI_W {
        TCI_W { w: self }
    }
    #[doc = "Bit 11 - Reload on Capture"]
    #[inline(always)]
    pub fn roc(&mut self) -> ROC_W {
        ROC_W { w: self }
    }
    #[doc = "Bit 12 - Alternative Load Enable"]
    #[inline(always)]
    pub fn alt_load(&mut self) -> ALT_LOAD_W {
        ALT_LOAD_W { w: self }
    }
    #[doc = "Bit 13 - Fault Enable"]
    #[inline(always)]
    pub fn fault(&mut self) -> FAULT_W {
        FAULT_W { w: self }
    }
    #[doc = "Bits 14:15 - Debug Actions Enable"]
    #[inline(always)]
    pub fn dbg_en(&mut self) -> DBG_EN_W {
        DBG_EN_W { w: self }
    }
}
