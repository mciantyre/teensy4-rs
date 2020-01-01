#[doc = "Reader of register RCSR"]
pub type R = crate::R<u32, super::RCSR>;
#[doc = "Writer for register RCSR"]
pub type W = crate::W<u32, super::RCSR>;
#[doc = "Register RCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FIFO Request DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRDE_A {
    #[doc = "0: Disables the DMA request."]
    FRDE_0 = 0,
    #[doc = "1: Enables the DMA request."]
    FRDE_1 = 1,
}
impl From<FRDE_A> for bool {
    #[inline(always)]
    fn from(variant: FRDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRDE`"]
pub type FRDE_R = crate::R<bool, FRDE_A>;
impl FRDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDE_A {
        match self.bits {
            false => FRDE_A::FRDE_0,
            true => FRDE_A::FRDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRDE_0`"]
    #[inline(always)]
    pub fn is_frde_0(&self) -> bool {
        *self == FRDE_A::FRDE_0
    }
    #[doc = "Checks if the value of the field is `FRDE_1`"]
    #[inline(always)]
    pub fn is_frde_1(&self) -> bool {
        *self == FRDE_A::FRDE_1
    }
}
#[doc = "Write proxy for field `FRDE`"]
pub struct FRDE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the DMA request."]
    #[inline(always)]
    pub fn frde_0(self) -> &'a mut W {
        self.variant(FRDE_A::FRDE_0)
    }
    #[doc = "Enables the DMA request."]
    #[inline(always)]
    pub fn frde_1(self) -> &'a mut W {
        self.variant(FRDE_A::FRDE_1)
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
#[doc = "FIFO Warning DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWDE_A {
    #[doc = "0: Disables the DMA request."]
    FWDE_0 = 0,
    #[doc = "1: Enables the DMA request."]
    FWDE_1 = 1,
}
impl From<FWDE_A> for bool {
    #[inline(always)]
    fn from(variant: FWDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FWDE`"]
pub type FWDE_R = crate::R<bool, FWDE_A>;
impl FWDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWDE_A {
        match self.bits {
            false => FWDE_A::FWDE_0,
            true => FWDE_A::FWDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWDE_0`"]
    #[inline(always)]
    pub fn is_fwde_0(&self) -> bool {
        *self == FWDE_A::FWDE_0
    }
    #[doc = "Checks if the value of the field is `FWDE_1`"]
    #[inline(always)]
    pub fn is_fwde_1(&self) -> bool {
        *self == FWDE_A::FWDE_1
    }
}
#[doc = "Write proxy for field `FWDE`"]
pub struct FWDE_W<'a> {
    w: &'a mut W,
}
impl<'a> FWDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the DMA request."]
    #[inline(always)]
    pub fn fwde_0(self) -> &'a mut W {
        self.variant(FWDE_A::FWDE_0)
    }
    #[doc = "Enables the DMA request."]
    #[inline(always)]
    pub fn fwde_1(self) -> &'a mut W {
        self.variant(FWDE_A::FWDE_1)
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
#[doc = "FIFO Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRIE_A {
    #[doc = "0: Disables the interrupt."]
    FRIE_0 = 0,
    #[doc = "1: Enables the interrupt."]
    FRIE_1 = 1,
}
impl From<FRIE_A> for bool {
    #[inline(always)]
    fn from(variant: FRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRIE`"]
pub type FRIE_R = crate::R<bool, FRIE_A>;
impl FRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRIE_A {
        match self.bits {
            false => FRIE_A::FRIE_0,
            true => FRIE_A::FRIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRIE_0`"]
    #[inline(always)]
    pub fn is_frie_0(&self) -> bool {
        *self == FRIE_A::FRIE_0
    }
    #[doc = "Checks if the value of the field is `FRIE_1`"]
    #[inline(always)]
    pub fn is_frie_1(&self) -> bool {
        *self == FRIE_A::FRIE_1
    }
}
#[doc = "Write proxy for field `FRIE`"]
pub struct FRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn frie_0(self) -> &'a mut W {
        self.variant(FRIE_A::FRIE_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn frie_1(self) -> &'a mut W {
        self.variant(FRIE_A::FRIE_1)
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
#[doc = "FIFO Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWIE_A {
    #[doc = "0: Disables the interrupt."]
    FWIE_0 = 0,
    #[doc = "1: Enables the interrupt."]
    FWIE_1 = 1,
}
impl From<FWIE_A> for bool {
    #[inline(always)]
    fn from(variant: FWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FWIE`"]
pub type FWIE_R = crate::R<bool, FWIE_A>;
impl FWIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWIE_A {
        match self.bits {
            false => FWIE_A::FWIE_0,
            true => FWIE_A::FWIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWIE_0`"]
    #[inline(always)]
    pub fn is_fwie_0(&self) -> bool {
        *self == FWIE_A::FWIE_0
    }
    #[doc = "Checks if the value of the field is `FWIE_1`"]
    #[inline(always)]
    pub fn is_fwie_1(&self) -> bool {
        *self == FWIE_A::FWIE_1
    }
}
#[doc = "Write proxy for field `FWIE`"]
pub struct FWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FWIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn fwie_0(self) -> &'a mut W {
        self.variant(FWIE_A::FWIE_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn fwie_1(self) -> &'a mut W {
        self.variant(FWIE_A::FWIE_1)
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
#[doc = "FIFO Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    #[doc = "0: Disables the interrupt."]
    FEIE_0 = 0,
    #[doc = "1: Enables the interrupt."]
    FEIE_1 = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FEIE`"]
pub type FEIE_R = crate::R<bool, FEIE_A>;
impl FEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::FEIE_0,
            true => FEIE_A::FEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FEIE_0`"]
    #[inline(always)]
    pub fn is_feie_0(&self) -> bool {
        *self == FEIE_A::FEIE_0
    }
    #[doc = "Checks if the value of the field is `FEIE_1`"]
    #[inline(always)]
    pub fn is_feie_1(&self) -> bool {
        *self == FEIE_A::FEIE_1
    }
}
#[doc = "Write proxy for field `FEIE`"]
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn feie_0(self) -> &'a mut W {
        self.variant(FEIE_A::FEIE_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn feie_1(self) -> &'a mut W {
        self.variant(FEIE_A::FEIE_1)
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
#[doc = "Sync Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEIE_A {
    #[doc = "0: Disables interrupt."]
    SEIE_0 = 0,
    #[doc = "1: Enables interrupt."]
    SEIE_1 = 1,
}
impl From<SEIE_A> for bool {
    #[inline(always)]
    fn from(variant: SEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEIE`"]
pub type SEIE_R = crate::R<bool, SEIE_A>;
impl SEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEIE_A {
        match self.bits {
            false => SEIE_A::SEIE_0,
            true => SEIE_A::SEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEIE_0`"]
    #[inline(always)]
    pub fn is_seie_0(&self) -> bool {
        *self == SEIE_A::SEIE_0
    }
    #[doc = "Checks if the value of the field is `SEIE_1`"]
    #[inline(always)]
    pub fn is_seie_1(&self) -> bool {
        *self == SEIE_A::SEIE_1
    }
}
#[doc = "Write proxy for field `SEIE`"]
pub struct SEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables interrupt."]
    #[inline(always)]
    pub fn seie_0(self) -> &'a mut W {
        self.variant(SEIE_A::SEIE_0)
    }
    #[doc = "Enables interrupt."]
    #[inline(always)]
    pub fn seie_1(self) -> &'a mut W {
        self.variant(SEIE_A::SEIE_1)
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
#[doc = "Word Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSIE_A {
    #[doc = "0: Disables interrupt."]
    WSIE_0 = 0,
    #[doc = "1: Enables interrupt."]
    WSIE_1 = 1,
}
impl From<WSIE_A> for bool {
    #[inline(always)]
    fn from(variant: WSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WSIE`"]
pub type WSIE_R = crate::R<bool, WSIE_A>;
impl WSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSIE_A {
        match self.bits {
            false => WSIE_A::WSIE_0,
            true => WSIE_A::WSIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WSIE_0`"]
    #[inline(always)]
    pub fn is_wsie_0(&self) -> bool {
        *self == WSIE_A::WSIE_0
    }
    #[doc = "Checks if the value of the field is `WSIE_1`"]
    #[inline(always)]
    pub fn is_wsie_1(&self) -> bool {
        *self == WSIE_A::WSIE_1
    }
}
#[doc = "Write proxy for field `WSIE`"]
pub struct WSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables interrupt."]
    #[inline(always)]
    pub fn wsie_0(self) -> &'a mut W {
        self.variant(WSIE_A::WSIE_0)
    }
    #[doc = "Enables interrupt."]
    #[inline(always)]
    pub fn wsie_1(self) -> &'a mut W {
        self.variant(WSIE_A::WSIE_1)
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
#[doc = "FIFO Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRF_A {
    #[doc = "0: Receive FIFO watermark not reached."]
    FRF_0 = 0,
    #[doc = "1: Receive FIFO watermark has been reached."]
    FRF_1 = 1,
}
impl From<FRF_A> for bool {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRF`"]
pub type FRF_R = crate::R<bool, FRF_A>;
impl FRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            false => FRF_A::FRF_0,
            true => FRF_A::FRF_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRF_0`"]
    #[inline(always)]
    pub fn is_frf_0(&self) -> bool {
        *self == FRF_A::FRF_0
    }
    #[doc = "Checks if the value of the field is `FRF_1`"]
    #[inline(always)]
    pub fn is_frf_1(&self) -> bool {
        *self == FRF_A::FRF_1
    }
}
#[doc = "FIFO Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWF_A {
    #[doc = "0: No enabled receive FIFO is full."]
    FWF_0 = 0,
    #[doc = "1: Enabled receive FIFO is full."]
    FWF_1 = 1,
}
impl From<FWF_A> for bool {
    #[inline(always)]
    fn from(variant: FWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FWF`"]
pub type FWF_R = crate::R<bool, FWF_A>;
impl FWF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWF_A {
        match self.bits {
            false => FWF_A::FWF_0,
            true => FWF_A::FWF_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWF_0`"]
    #[inline(always)]
    pub fn is_fwf_0(&self) -> bool {
        *self == FWF_A::FWF_0
    }
    #[doc = "Checks if the value of the field is `FWF_1`"]
    #[inline(always)]
    pub fn is_fwf_1(&self) -> bool {
        *self == FWF_A::FWF_1
    }
}
#[doc = "FIFO Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEF_A {
    #[doc = "0: Receive overflow not detected."]
    FEF_0 = 0,
    #[doc = "1: Receive overflow detected."]
    FEF_1 = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FEF`"]
pub type FEF_R = crate::R<bool, FEF_A>;
impl FEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::FEF_0,
            true => FEF_A::FEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `FEF_0`"]
    #[inline(always)]
    pub fn is_fef_0(&self) -> bool {
        *self == FEF_A::FEF_0
    }
    #[doc = "Checks if the value of the field is `FEF_1`"]
    #[inline(always)]
    pub fn is_fef_1(&self) -> bool {
        *self == FEF_A::FEF_1
    }
}
#[doc = "Write proxy for field `FEF`"]
pub struct FEF_W<'a> {
    w: &'a mut W,
}
impl<'a> FEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive overflow not detected."]
    #[inline(always)]
    pub fn fef_0(self) -> &'a mut W {
        self.variant(FEF_A::FEF_0)
    }
    #[doc = "Receive overflow detected."]
    #[inline(always)]
    pub fn fef_1(self) -> &'a mut W {
        self.variant(FEF_A::FEF_1)
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
#[doc = "Sync Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEF_A {
    #[doc = "0: Sync error not detected."]
    SEF_0 = 0,
    #[doc = "1: Frame sync error detected."]
    SEF_1 = 1,
}
impl From<SEF_A> for bool {
    #[inline(always)]
    fn from(variant: SEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEF`"]
pub type SEF_R = crate::R<bool, SEF_A>;
impl SEF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEF_A {
        match self.bits {
            false => SEF_A::SEF_0,
            true => SEF_A::SEF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEF_0`"]
    #[inline(always)]
    pub fn is_sef_0(&self) -> bool {
        *self == SEF_A::SEF_0
    }
    #[doc = "Checks if the value of the field is `SEF_1`"]
    #[inline(always)]
    pub fn is_sef_1(&self) -> bool {
        *self == SEF_A::SEF_1
    }
}
#[doc = "Write proxy for field `SEF`"]
pub struct SEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sync error not detected."]
    #[inline(always)]
    pub fn sef_0(self) -> &'a mut W {
        self.variant(SEF_A::SEF_0)
    }
    #[doc = "Frame sync error detected."]
    #[inline(always)]
    pub fn sef_1(self) -> &'a mut W {
        self.variant(SEF_A::SEF_1)
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
#[doc = "Word Start Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSF_A {
    #[doc = "0: Start of word not detected."]
    WSF_0 = 0,
    #[doc = "1: Start of word detected."]
    WSF_1 = 1,
}
impl From<WSF_A> for bool {
    #[inline(always)]
    fn from(variant: WSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WSF`"]
pub type WSF_R = crate::R<bool, WSF_A>;
impl WSF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSF_A {
        match self.bits {
            false => WSF_A::WSF_0,
            true => WSF_A::WSF_1,
        }
    }
    #[doc = "Checks if the value of the field is `WSF_0`"]
    #[inline(always)]
    pub fn is_wsf_0(&self) -> bool {
        *self == WSF_A::WSF_0
    }
    #[doc = "Checks if the value of the field is `WSF_1`"]
    #[inline(always)]
    pub fn is_wsf_1(&self) -> bool {
        *self == WSF_A::WSF_1
    }
}
#[doc = "Write proxy for field `WSF`"]
pub struct WSF_W<'a> {
    w: &'a mut W,
}
impl<'a> WSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WSF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Start of word not detected."]
    #[inline(always)]
    pub fn wsf_0(self) -> &'a mut W {
        self.variant(WSF_A::WSF_0)
    }
    #[doc = "Start of word detected."]
    #[inline(always)]
    pub fn wsf_1(self) -> &'a mut W {
        self.variant(WSF_A::WSF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR_A {
    #[doc = "0: No effect."]
    SR_0 = 0,
    #[doc = "1: Software reset."]
    SR_1 = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SR`"]
pub type SR_R = crate::R<bool, SR_A>;
impl SR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR_A {
        match self.bits {
            false => SR_A::SR_0,
            true => SR_A::SR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SR_0`"]
    #[inline(always)]
    pub fn is_sr_0(&self) -> bool {
        *self == SR_A::SR_0
    }
    #[doc = "Checks if the value of the field is `SR_1`"]
    #[inline(always)]
    pub fn is_sr_1(&self) -> bool {
        *self == SR_A::SR_1
    }
}
#[doc = "Write proxy for field `SR`"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn sr_0(self) -> &'a mut W {
        self.variant(SR_A::SR_0)
    }
    #[doc = "Software reset."]
    #[inline(always)]
    pub fn sr_1(self) -> &'a mut W {
        self.variant(SR_A::SR_1)
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
#[doc = "FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FR_A {
    #[doc = "0: No effect."]
    FR_0 = 0,
    #[doc = "1: FIFO reset."]
    FR_1 = 1,
}
impl From<FR_A> for bool {
    #[inline(always)]
    fn from(variant: FR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FR`"]
pub type FR_R = crate::R<bool, FR_A>;
impl FR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FR_A {
        match self.bits {
            false => FR_A::FR_0,
            true => FR_A::FR_1,
        }
    }
    #[doc = "Checks if the value of the field is `FR_0`"]
    #[inline(always)]
    pub fn is_fr_0(&self) -> bool {
        *self == FR_A::FR_0
    }
    #[doc = "Checks if the value of the field is `FR_1`"]
    #[inline(always)]
    pub fn is_fr_1(&self) -> bool {
        *self == FR_A::FR_1
    }
}
#[doc = "Write proxy for field `FR`"]
pub struct FR_W<'a> {
    w: &'a mut W,
}
impl<'a> FR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn fr_0(self) -> &'a mut W {
        self.variant(FR_A::FR_0)
    }
    #[doc = "FIFO reset."]
    #[inline(always)]
    pub fn fr_1(self) -> &'a mut W {
        self.variant(FR_A::FR_1)
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
#[doc = "Bit Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCE_A {
    #[doc = "0: Receive bit clock is disabled."]
    BCE_0 = 0,
    #[doc = "1: Receive bit clock is enabled."]
    BCE_1 = 1,
}
impl From<BCE_A> for bool {
    #[inline(always)]
    fn from(variant: BCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BCE`"]
pub type BCE_R = crate::R<bool, BCE_A>;
impl BCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCE_A {
        match self.bits {
            false => BCE_A::BCE_0,
            true => BCE_A::BCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCE_0`"]
    #[inline(always)]
    pub fn is_bce_0(&self) -> bool {
        *self == BCE_A::BCE_0
    }
    #[doc = "Checks if the value of the field is `BCE_1`"]
    #[inline(always)]
    pub fn is_bce_1(&self) -> bool {
        *self == BCE_A::BCE_1
    }
}
#[doc = "Write proxy for field `BCE`"]
pub struct BCE_W<'a> {
    w: &'a mut W,
}
impl<'a> BCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive bit clock is disabled."]
    #[inline(always)]
    pub fn bce_0(self) -> &'a mut W {
        self.variant(BCE_A::BCE_0)
    }
    #[doc = "Receive bit clock is enabled."]
    #[inline(always)]
    pub fn bce_1(self) -> &'a mut W {
        self.variant(BCE_A::BCE_1)
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
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGE_A {
    #[doc = "0: Receiver is disabled in Debug mode, after completing the current frame."]
    DBGE_0 = 0,
    #[doc = "1: Receiver is enabled in Debug mode."]
    DBGE_1 = 1,
}
impl From<DBGE_A> for bool {
    #[inline(always)]
    fn from(variant: DBGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBGE`"]
pub type DBGE_R = crate::R<bool, DBGE_A>;
impl DBGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGE_A {
        match self.bits {
            false => DBGE_A::DBGE_0,
            true => DBGE_A::DBGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGE_0`"]
    #[inline(always)]
    pub fn is_dbge_0(&self) -> bool {
        *self == DBGE_A::DBGE_0
    }
    #[doc = "Checks if the value of the field is `DBGE_1`"]
    #[inline(always)]
    pub fn is_dbge_1(&self) -> bool {
        *self == DBGE_A::DBGE_1
    }
}
#[doc = "Write proxy for field `DBGE`"]
pub struct DBGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver is disabled in Debug mode, after completing the current frame."]
    #[inline(always)]
    pub fn dbge_0(self) -> &'a mut W {
        self.variant(DBGE_A::DBGE_0)
    }
    #[doc = "Receiver is enabled in Debug mode."]
    #[inline(always)]
    pub fn dbge_1(self) -> &'a mut W {
        self.variant(DBGE_A::DBGE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPE_A {
    #[doc = "0: Receiver disabled in Stop mode."]
    STOPE_0 = 0,
    #[doc = "1: Receiver enabled in Stop mode."]
    STOPE_1 = 1,
}
impl From<STOPE_A> for bool {
    #[inline(always)]
    fn from(variant: STOPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOPE`"]
pub type STOPE_R = crate::R<bool, STOPE_A>;
impl STOPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPE_A {
        match self.bits {
            false => STOPE_A::STOPE_0,
            true => STOPE_A::STOPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `STOPE_0`"]
    #[inline(always)]
    pub fn is_stope_0(&self) -> bool {
        *self == STOPE_A::STOPE_0
    }
    #[doc = "Checks if the value of the field is `STOPE_1`"]
    #[inline(always)]
    pub fn is_stope_1(&self) -> bool {
        *self == STOPE_A::STOPE_1
    }
}
#[doc = "Write proxy for field `STOPE`"]
pub struct STOPE_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver disabled in Stop mode."]
    #[inline(always)]
    pub fn stope_0(self) -> &'a mut W {
        self.variant(STOPE_A::STOPE_0)
    }
    #[doc = "Receiver enabled in Stop mode."]
    #[inline(always)]
    pub fn stope_1(self) -> &'a mut W {
        self.variant(STOPE_A::STOPE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    #[doc = "0: Receiver is disabled."]
    RE_0 = 0,
    #[doc = "1: Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
    RE_1 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RE`"]
pub type RE_R = crate::R<bool, RE_A>;
impl RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::RE_0,
            true => RE_A::RE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RE_0`"]
    #[inline(always)]
    pub fn is_re_0(&self) -> bool {
        *self == RE_A::RE_0
    }
    #[doc = "Checks if the value of the field is `RE_1`"]
    #[inline(always)]
    pub fn is_re_1(&self) -> bool {
        *self == RE_A::RE_1
    }
}
#[doc = "Write proxy for field `RE`"]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver is disabled."]
    #[inline(always)]
    pub fn re_0(self) -> &'a mut W {
        self.variant(RE_A::RE_0)
    }
    #[doc = "Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
    #[inline(always)]
    pub fn re_1(self) -> &'a mut W {
        self.variant(RE_A::RE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FIFO Request DMA Enable"]
    #[inline(always)]
    pub fn frde(&self) -> FRDE_R {
        FRDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline(always)]
    pub fn fwde(&self) -> FWDE_R {
        FWDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub fn frie(&self) -> FRIE_R {
        FRIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub fn fwie(&self) -> FWIE_R {
        FWIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline(always)]
    pub fn wsie(&self) -> WSIE_R {
        WSIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FIFO Request Flag"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FIFO Warning Flag"]
    #[inline(always)]
    pub fn fwf(&self) -> FWF_R {
        FWF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline(always)]
    pub fn wsf(&self) -> WSF_R {
        WSF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FIFO Reset"]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline(always)]
    pub fn bce(&self) -> BCE_R {
        BCE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline(always)]
    pub fn dbge(&self) -> DBGE_R {
        DBGE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Stop Enable"]
    #[inline(always)]
    pub fn stope(&self) -> STOPE_R {
        STOPE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Request DMA Enable"]
    #[inline(always)]
    pub fn frde(&mut self) -> FRDE_W {
        FRDE_W { w: self }
    }
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline(always)]
    pub fn fwde(&mut self) -> FWDE_W {
        FWDE_W { w: self }
    }
    #[doc = "Bit 8 - FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub fn frie(&mut self) -> FRIE_W {
        FRIE_W { w: self }
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub fn fwie(&mut self) -> FWIE_W {
        FWIE_W { w: self }
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&mut self) -> SEIE_W {
        SEIE_W { w: self }
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline(always)]
    pub fn wsie(&mut self) -> WSIE_W {
        WSIE_W { w: self }
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W {
        FEF_W { w: self }
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline(always)]
    pub fn sef(&mut self) -> SEF_W {
        SEF_W { w: self }
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline(always)]
    pub fn wsf(&mut self) -> WSF_W {
        WSF_W { w: self }
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 25 - FIFO Reset"]
    #[inline(always)]
    pub fn fr(&mut self) -> FR_W {
        FR_W { w: self }
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline(always)]
    pub fn bce(&mut self) -> BCE_W {
        BCE_W { w: self }
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline(always)]
    pub fn dbge(&mut self) -> DBGE_W {
        DBGE_W { w: self }
    }
    #[doc = "Bit 30 - Stop Enable"]
    #[inline(always)]
    pub fn stope(&mut self) -> STOPE_W {
        STOPE_W { w: self }
    }
    #[doc = "Bit 31 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
}
