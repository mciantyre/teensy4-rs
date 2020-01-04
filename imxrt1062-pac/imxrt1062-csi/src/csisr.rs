#[doc = "Reader of register CSISR"]
pub type R = crate::R<u32, super::CSISR>;
#[doc = "Writer for register CSISR"]
pub type W = crate::W<u32, super::CSISR>;
#[doc = "Register CSISR `reset()`'s with value 0x8000_4000"]
impl crate::ResetValue for super::CSISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_4000
    }
}
#[doc = "RXFIFO Data Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRDY_A {
    #[doc = "0: No data (word) is ready"]
    DRDY_0 = 0,
    #[doc = "1: At least 1 datum (word) is ready in RXFIFO."]
    DRDY_1 = 1,
}
impl From<DRDY_A> for bool {
    #[inline(always)]
    fn from(variant: DRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRDY`"]
pub type DRDY_R = crate::R<bool, DRDY_A>;
impl DRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRDY_A {
        match self.bits {
            false => DRDY_A::DRDY_0,
            true => DRDY_A::DRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `DRDY_0`"]
    #[inline(always)]
    pub fn is_drdy_0(&self) -> bool {
        *self == DRDY_A::DRDY_0
    }
    #[doc = "Checks if the value of the field is `DRDY_1`"]
    #[inline(always)]
    pub fn is_drdy_1(&self) -> bool {
        *self == DRDY_A::DRDY_1
    }
}
#[doc = "Write proxy for field `DRDY`"]
pub struct DRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No data (word) is ready"]
    #[inline(always)]
    pub fn drdy_0(self) -> &'a mut W {
        self.variant(DRDY_A::DRDY_0)
    }
    #[doc = "At least 1 datum (word) is ready in RXFIFO."]
    #[inline(always)]
    pub fn drdy_1(self) -> &'a mut W {
        self.variant(DRDY_A::DRDY_1)
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
#[doc = "CCIR Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC_INT_A {
    #[doc = "0: No error detected"]
    ECC_INT_0 = 0,
    #[doc = "1: Error is detected in CCIR coding"]
    ECC_INT_1 = 1,
}
impl From<ECC_INT_A> for bool {
    #[inline(always)]
    fn from(variant: ECC_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECC_INT`"]
pub type ECC_INT_R = crate::R<bool, ECC_INT_A>;
impl ECC_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECC_INT_A {
        match self.bits {
            false => ECC_INT_A::ECC_INT_0,
            true => ECC_INT_A::ECC_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECC_INT_0`"]
    #[inline(always)]
    pub fn is_ecc_int_0(&self) -> bool {
        *self == ECC_INT_A::ECC_INT_0
    }
    #[doc = "Checks if the value of the field is `ECC_INT_1`"]
    #[inline(always)]
    pub fn is_ecc_int_1(&self) -> bool {
        *self == ECC_INT_A::ECC_INT_1
    }
}
#[doc = "Write proxy for field `ECC_INT`"]
pub struct ECC_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECC_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No error detected"]
    #[inline(always)]
    pub fn ecc_int_0(self) -> &'a mut W {
        self.variant(ECC_INT_A::ECC_INT_0)
    }
    #[doc = "Error is detected in CCIR coding"]
    #[inline(always)]
    pub fn ecc_int_1(self) -> &'a mut W {
        self.variant(ECC_INT_A::ECC_INT_1)
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
#[doc = "Hresponse Error Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRESP_ERR_INT_A {
    #[doc = "0: No hresponse error."]
    HRESP_ERR_INT_0 = 0,
    #[doc = "1: Hresponse error is detected."]
    HRESP_ERR_INT_1 = 1,
}
impl From<HRESP_ERR_INT_A> for bool {
    #[inline(always)]
    fn from(variant: HRESP_ERR_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRESP_ERR_INT`"]
pub type HRESP_ERR_INT_R = crate::R<bool, HRESP_ERR_INT_A>;
impl HRESP_ERR_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRESP_ERR_INT_A {
        match self.bits {
            false => HRESP_ERR_INT_A::HRESP_ERR_INT_0,
            true => HRESP_ERR_INT_A::HRESP_ERR_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRESP_ERR_INT_0`"]
    #[inline(always)]
    pub fn is_hresp_err_int_0(&self) -> bool {
        *self == HRESP_ERR_INT_A::HRESP_ERR_INT_0
    }
    #[doc = "Checks if the value of the field is `HRESP_ERR_INT_1`"]
    #[inline(always)]
    pub fn is_hresp_err_int_1(&self) -> bool {
        *self == HRESP_ERR_INT_A::HRESP_ERR_INT_1
    }
}
#[doc = "Write proxy for field `HRESP_ERR_INT`"]
pub struct HRESP_ERR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> HRESP_ERR_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRESP_ERR_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No hresponse error."]
    #[inline(always)]
    pub fn hresp_err_int_0(self) -> &'a mut W {
        self.variant(HRESP_ERR_INT_A::HRESP_ERR_INT_0)
    }
    #[doc = "Hresponse error is detected."]
    #[inline(always)]
    pub fn hresp_err_int_1(self) -> &'a mut W {
        self.variant(HRESP_ERR_INT_A::HRESP_ERR_INT_1)
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
#[doc = "Change Of Field Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COF_INT_A {
    #[doc = "0: Video field has no change."]
    COF_INT_0 = 0,
    #[doc = "1: Change of video field is detected."]
    COF_INT_1 = 1,
}
impl From<COF_INT_A> for bool {
    #[inline(always)]
    fn from(variant: COF_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COF_INT`"]
pub type COF_INT_R = crate::R<bool, COF_INT_A>;
impl COF_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COF_INT_A {
        match self.bits {
            false => COF_INT_A::COF_INT_0,
            true => COF_INT_A::COF_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `COF_INT_0`"]
    #[inline(always)]
    pub fn is_cof_int_0(&self) -> bool {
        *self == COF_INT_A::COF_INT_0
    }
    #[doc = "Checks if the value of the field is `COF_INT_1`"]
    #[inline(always)]
    pub fn is_cof_int_1(&self) -> bool {
        *self == COF_INT_A::COF_INT_1
    }
}
#[doc = "Write proxy for field `COF_INT`"]
pub struct COF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> COF_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COF_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Video field has no change."]
    #[inline(always)]
    pub fn cof_int_0(self) -> &'a mut W {
        self.variant(COF_INT_A::COF_INT_0)
    }
    #[doc = "Change of video field is detected."]
    #[inline(always)]
    pub fn cof_int_1(self) -> &'a mut W {
        self.variant(COF_INT_A::COF_INT_1)
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
#[doc = "CCIR Field 1 Interrupt Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F1_INT_A {
    #[doc = "0: Field 1 of video is not detected."]
    F1_INT_0 = 0,
    #[doc = "1: Field 1 of video is about to start."]
    F1_INT_1 = 1,
}
impl From<F1_INT_A> for bool {
    #[inline(always)]
    fn from(variant: F1_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `F1_INT`"]
pub type F1_INT_R = crate::R<bool, F1_INT_A>;
impl F1_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F1_INT_A {
        match self.bits {
            false => F1_INT_A::F1_INT_0,
            true => F1_INT_A::F1_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `F1_INT_0`"]
    #[inline(always)]
    pub fn is_f1_int_0(&self) -> bool {
        *self == F1_INT_A::F1_INT_0
    }
    #[doc = "Checks if the value of the field is `F1_INT_1`"]
    #[inline(always)]
    pub fn is_f1_int_1(&self) -> bool {
        *self == F1_INT_A::F1_INT_1
    }
}
#[doc = "Write proxy for field `F1_INT`"]
pub struct F1_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> F1_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: F1_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Field 1 of video is not detected."]
    #[inline(always)]
    pub fn f1_int_0(self) -> &'a mut W {
        self.variant(F1_INT_A::F1_INT_0)
    }
    #[doc = "Field 1 of video is about to start."]
    #[inline(always)]
    pub fn f1_int_1(self) -> &'a mut W {
        self.variant(F1_INT_A::F1_INT_1)
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
#[doc = "CCIR Field 2 Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F2_INT_A {
    #[doc = "0: Field 2 of video is not detected"]
    F2_INT_0 = 0,
    #[doc = "1: Field 2 of video is about to start"]
    F2_INT_1 = 1,
}
impl From<F2_INT_A> for bool {
    #[inline(always)]
    fn from(variant: F2_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `F2_INT`"]
pub type F2_INT_R = crate::R<bool, F2_INT_A>;
impl F2_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F2_INT_A {
        match self.bits {
            false => F2_INT_A::F2_INT_0,
            true => F2_INT_A::F2_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `F2_INT_0`"]
    #[inline(always)]
    pub fn is_f2_int_0(&self) -> bool {
        *self == F2_INT_A::F2_INT_0
    }
    #[doc = "Checks if the value of the field is `F2_INT_1`"]
    #[inline(always)]
    pub fn is_f2_int_1(&self) -> bool {
        *self == F2_INT_A::F2_INT_1
    }
}
#[doc = "Write proxy for field `F2_INT`"]
pub struct F2_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> F2_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: F2_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Field 2 of video is not detected"]
    #[inline(always)]
    pub fn f2_int_0(self) -> &'a mut W {
        self.variant(F2_INT_A::F2_INT_0)
    }
    #[doc = "Field 2 of video is about to start"]
    #[inline(always)]
    pub fn f2_int_1(self) -> &'a mut W {
        self.variant(F2_INT_A::F2_INT_1)
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
#[doc = "Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_INT_A {
    #[doc = "0: SOF is not detected."]
    SOF_INT_0 = 0,
    #[doc = "1: SOF is detected."]
    SOF_INT_1 = 1,
}
impl From<SOF_INT_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOF_INT`"]
pub type SOF_INT_R = crate::R<bool, SOF_INT_A>;
impl SOF_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_INT_A {
        match self.bits {
            false => SOF_INT_A::SOF_INT_0,
            true => SOF_INT_A::SOF_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOF_INT_0`"]
    #[inline(always)]
    pub fn is_sof_int_0(&self) -> bool {
        *self == SOF_INT_A::SOF_INT_0
    }
    #[doc = "Checks if the value of the field is `SOF_INT_1`"]
    #[inline(always)]
    pub fn is_sof_int_1(&self) -> bool {
        *self == SOF_INT_A::SOF_INT_1
    }
}
#[doc = "Write proxy for field `SOF_INT`"]
pub struct SOF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOF_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SOF is not detected."]
    #[inline(always)]
    pub fn sof_int_0(self) -> &'a mut W {
        self.variant(SOF_INT_A::SOF_INT_0)
    }
    #[doc = "SOF is detected."]
    #[inline(always)]
    pub fn sof_int_1(self) -> &'a mut W {
        self.variant(SOF_INT_A::SOF_INT_1)
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
#[doc = "End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOF_INT_A {
    #[doc = "0: EOF is not detected."]
    EOF_INT_0 = 0,
    #[doc = "1: EOF is detected."]
    EOF_INT_1 = 1,
}
impl From<EOF_INT_A> for bool {
    #[inline(always)]
    fn from(variant: EOF_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOF_INT`"]
pub type EOF_INT_R = crate::R<bool, EOF_INT_A>;
impl EOF_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOF_INT_A {
        match self.bits {
            false => EOF_INT_A::EOF_INT_0,
            true => EOF_INT_A::EOF_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `EOF_INT_0`"]
    #[inline(always)]
    pub fn is_eof_int_0(&self) -> bool {
        *self == EOF_INT_A::EOF_INT_0
    }
    #[doc = "Checks if the value of the field is `EOF_INT_1`"]
    #[inline(always)]
    pub fn is_eof_int_1(&self) -> bool {
        *self == EOF_INT_A::EOF_INT_1
    }
}
#[doc = "Write proxy for field `EOF_INT`"]
pub struct EOF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOF_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOF_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EOF is not detected."]
    #[inline(always)]
    pub fn eof_int_0(self) -> &'a mut W {
        self.variant(EOF_INT_A::EOF_INT_0)
    }
    #[doc = "EOF is detected."]
    #[inline(always)]
    pub fn eof_int_1(self) -> &'a mut W {
        self.variant(EOF_INT_A::EOF_INT_1)
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
#[doc = "RXFIFO Full Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFF_INT_A {
    #[doc = "0: RxFIFO is not full."]
    RXFF_INT_0 = 0,
    #[doc = "1: RxFIFO is full."]
    RXFF_INT_1 = 1,
}
impl From<RXFF_INT_A> for bool {
    #[inline(always)]
    fn from(variant: RXFF_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RxFF_INT`"]
pub type RXFF_INT_R = crate::R<bool, RXFF_INT_A>;
impl RXFF_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFF_INT_A {
        match self.bits {
            false => RXFF_INT_A::RXFF_INT_0,
            true => RXFF_INT_A::RXFF_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXFF_INT_0`"]
    #[inline(always)]
    pub fn is_rx_ff_int_0(&self) -> bool {
        *self == RXFF_INT_A::RXFF_INT_0
    }
    #[doc = "Checks if the value of the field is `RXFF_INT_1`"]
    #[inline(always)]
    pub fn is_rx_ff_int_1(&self) -> bool {
        *self == RXFF_INT_A::RXFF_INT_1
    }
}
#[doc = "Write proxy for field `RxFF_INT`"]
pub struct RXFF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFF_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFF_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RxFIFO is not full."]
    #[inline(always)]
    pub fn rx_ff_int_0(self) -> &'a mut W {
        self.variant(RXFF_INT_A::RXFF_INT_0)
    }
    #[doc = "RxFIFO is full."]
    #[inline(always)]
    pub fn rx_ff_int_1(self) -> &'a mut W {
        self.variant(RXFF_INT_A::RXFF_INT_1)
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
#[doc = "DMA Transfer Done in Frame Buffer1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_TSF_DONE_FB1_A {
    #[doc = "0: DMA transfer is not completed."]
    DMA_TSF_DONE_FB1_0 = 0,
    #[doc = "1: DMA transfer is completed."]
    DMA_TSF_DONE_FB1_1 = 1,
}
impl From<DMA_TSF_DONE_FB1_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_TSF_DONE_FB1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_TSF_DONE_FB1`"]
pub type DMA_TSF_DONE_FB1_R = crate::R<bool, DMA_TSF_DONE_FB1_A>;
impl DMA_TSF_DONE_FB1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_TSF_DONE_FB1_A {
        match self.bits {
            false => DMA_TSF_DONE_FB1_A::DMA_TSF_DONE_FB1_0,
            true => DMA_TSF_DONE_FB1_A::DMA_TSF_DONE_FB1_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_TSF_DONE_FB1_0`"]
    #[inline(always)]
    pub fn is_dma_tsf_done_fb1_0(&self) -> bool {
        *self == DMA_TSF_DONE_FB1_A::DMA_TSF_DONE_FB1_0
    }
    #[doc = "Checks if the value of the field is `DMA_TSF_DONE_FB1_1`"]
    #[inline(always)]
    pub fn is_dma_tsf_done_fb1_1(&self) -> bool {
        *self == DMA_TSF_DONE_FB1_A::DMA_TSF_DONE_FB1_1
    }
}
#[doc = "Write proxy for field `DMA_TSF_DONE_FB1`"]
pub struct DMA_TSF_DONE_FB1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TSF_DONE_FB1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_TSF_DONE_FB1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA transfer is not completed."]
    #[inline(always)]
    pub fn dma_tsf_done_fb1_0(self) -> &'a mut W {
        self.variant(DMA_TSF_DONE_FB1_A::DMA_TSF_DONE_FB1_0)
    }
    #[doc = "DMA transfer is completed."]
    #[inline(always)]
    pub fn dma_tsf_done_fb1_1(self) -> &'a mut W {
        self.variant(DMA_TSF_DONE_FB1_A::DMA_TSF_DONE_FB1_1)
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
#[doc = "DMA Transfer Done in Frame Buffer2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_TSF_DONE_FB2_A {
    #[doc = "0: DMA transfer is not completed."]
    DMA_TSF_DONE_FB2_0 = 0,
    #[doc = "1: DMA transfer is completed."]
    DMA_TSF_DONE_FB2_1 = 1,
}
impl From<DMA_TSF_DONE_FB2_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_TSF_DONE_FB2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_TSF_DONE_FB2`"]
pub type DMA_TSF_DONE_FB2_R = crate::R<bool, DMA_TSF_DONE_FB2_A>;
impl DMA_TSF_DONE_FB2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_TSF_DONE_FB2_A {
        match self.bits {
            false => DMA_TSF_DONE_FB2_A::DMA_TSF_DONE_FB2_0,
            true => DMA_TSF_DONE_FB2_A::DMA_TSF_DONE_FB2_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_TSF_DONE_FB2_0`"]
    #[inline(always)]
    pub fn is_dma_tsf_done_fb2_0(&self) -> bool {
        *self == DMA_TSF_DONE_FB2_A::DMA_TSF_DONE_FB2_0
    }
    #[doc = "Checks if the value of the field is `DMA_TSF_DONE_FB2_1`"]
    #[inline(always)]
    pub fn is_dma_tsf_done_fb2_1(&self) -> bool {
        *self == DMA_TSF_DONE_FB2_A::DMA_TSF_DONE_FB2_1
    }
}
#[doc = "Write proxy for field `DMA_TSF_DONE_FB2`"]
pub struct DMA_TSF_DONE_FB2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TSF_DONE_FB2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_TSF_DONE_FB2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA transfer is not completed."]
    #[inline(always)]
    pub fn dma_tsf_done_fb2_0(self) -> &'a mut W {
        self.variant(DMA_TSF_DONE_FB2_A::DMA_TSF_DONE_FB2_0)
    }
    #[doc = "DMA transfer is completed."]
    #[inline(always)]
    pub fn dma_tsf_done_fb2_1(self) -> &'a mut W {
        self.variant(DMA_TSF_DONE_FB2_A::DMA_TSF_DONE_FB2_1)
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
#[doc = "STATFIFO Full Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATFF_INT_A {
    #[doc = "0: STATFIFO is not full."]
    STATFF_INT_0 = 0,
    #[doc = "1: STATFIFO is full."]
    STATFF_INT_1 = 1,
}
impl From<STATFF_INT_A> for bool {
    #[inline(always)]
    fn from(variant: STATFF_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STATFF_INT`"]
pub type STATFF_INT_R = crate::R<bool, STATFF_INT_A>;
impl STATFF_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATFF_INT_A {
        match self.bits {
            false => STATFF_INT_A::STATFF_INT_0,
            true => STATFF_INT_A::STATFF_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `STATFF_INT_0`"]
    #[inline(always)]
    pub fn is_statff_int_0(&self) -> bool {
        *self == STATFF_INT_A::STATFF_INT_0
    }
    #[doc = "Checks if the value of the field is `STATFF_INT_1`"]
    #[inline(always)]
    pub fn is_statff_int_1(&self) -> bool {
        *self == STATFF_INT_A::STATFF_INT_1
    }
}
#[doc = "Write proxy for field `STATFF_INT`"]
pub struct STATFF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> STATFF_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATFF_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "STATFIFO is not full."]
    #[inline(always)]
    pub fn statff_int_0(self) -> &'a mut W {
        self.variant(STATFF_INT_A::STATFF_INT_0)
    }
    #[doc = "STATFIFO is full."]
    #[inline(always)]
    pub fn statff_int_1(self) -> &'a mut W {
        self.variant(STATFF_INT_A::STATFF_INT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "DMA Transfer Done from StatFIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_TSF_DONE_SFF_A {
    #[doc = "0: DMA transfer is not completed."]
    DMA_TSF_DONE_SFF_0 = 0,
    #[doc = "1: DMA transfer is completed."]
    DMA_TSF_DONE_SFF_1 = 1,
}
impl From<DMA_TSF_DONE_SFF_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_TSF_DONE_SFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_TSF_DONE_SFF`"]
pub type DMA_TSF_DONE_SFF_R = crate::R<bool, DMA_TSF_DONE_SFF_A>;
impl DMA_TSF_DONE_SFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_TSF_DONE_SFF_A {
        match self.bits {
            false => DMA_TSF_DONE_SFF_A::DMA_TSF_DONE_SFF_0,
            true => DMA_TSF_DONE_SFF_A::DMA_TSF_DONE_SFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_TSF_DONE_SFF_0`"]
    #[inline(always)]
    pub fn is_dma_tsf_done_sff_0(&self) -> bool {
        *self == DMA_TSF_DONE_SFF_A::DMA_TSF_DONE_SFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_TSF_DONE_SFF_1`"]
    #[inline(always)]
    pub fn is_dma_tsf_done_sff_1(&self) -> bool {
        *self == DMA_TSF_DONE_SFF_A::DMA_TSF_DONE_SFF_1
    }
}
#[doc = "Write proxy for field `DMA_TSF_DONE_SFF`"]
pub struct DMA_TSF_DONE_SFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TSF_DONE_SFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_TSF_DONE_SFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA transfer is not completed."]
    #[inline(always)]
    pub fn dma_tsf_done_sff_0(self) -> &'a mut W {
        self.variant(DMA_TSF_DONE_SFF_A::DMA_TSF_DONE_SFF_0)
    }
    #[doc = "DMA transfer is completed."]
    #[inline(always)]
    pub fn dma_tsf_done_sff_1(self) -> &'a mut W {
        self.variant(DMA_TSF_DONE_SFF_A::DMA_TSF_DONE_SFF_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "RxFIFO Overrun Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_OR_INT_A {
    #[doc = "0: RXFIFO has not overflowed."]
    RF_OR_INT_0 = 0,
    #[doc = "1: RXFIFO has overflowed."]
    RF_OR_INT_1 = 1,
}
impl From<RF_OR_INT_A> for bool {
    #[inline(always)]
    fn from(variant: RF_OR_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF_OR_INT`"]
pub type RF_OR_INT_R = crate::R<bool, RF_OR_INT_A>;
impl RF_OR_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_OR_INT_A {
        match self.bits {
            false => RF_OR_INT_A::RF_OR_INT_0,
            true => RF_OR_INT_A::RF_OR_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `RF_OR_INT_0`"]
    #[inline(always)]
    pub fn is_rf_or_int_0(&self) -> bool {
        *self == RF_OR_INT_A::RF_OR_INT_0
    }
    #[doc = "Checks if the value of the field is `RF_OR_INT_1`"]
    #[inline(always)]
    pub fn is_rf_or_int_1(&self) -> bool {
        *self == RF_OR_INT_A::RF_OR_INT_1
    }
}
#[doc = "Write proxy for field `RF_OR_INT`"]
pub struct RF_OR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_OR_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_OR_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RXFIFO has not overflowed."]
    #[inline(always)]
    pub fn rf_or_int_0(self) -> &'a mut W {
        self.variant(RF_OR_INT_A::RF_OR_INT_0)
    }
    #[doc = "RXFIFO has overflowed."]
    #[inline(always)]
    pub fn rf_or_int_1(self) -> &'a mut W {
        self.variant(RF_OR_INT_A::RF_OR_INT_1)
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
#[doc = "STATFIFO Overrun Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SF_OR_INT_A {
    #[doc = "0: STATFIFO has not overflowed."]
    SF_OR_INT_0 = 0,
    #[doc = "1: STATFIFO has overflowed."]
    SF_OR_INT_1 = 1,
}
impl From<SF_OR_INT_A> for bool {
    #[inline(always)]
    fn from(variant: SF_OR_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SF_OR_INT`"]
pub type SF_OR_INT_R = crate::R<bool, SF_OR_INT_A>;
impl SF_OR_INT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SF_OR_INT_A {
        match self.bits {
            false => SF_OR_INT_A::SF_OR_INT_0,
            true => SF_OR_INT_A::SF_OR_INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SF_OR_INT_0`"]
    #[inline(always)]
    pub fn is_sf_or_int_0(&self) -> bool {
        *self == SF_OR_INT_A::SF_OR_INT_0
    }
    #[doc = "Checks if the value of the field is `SF_OR_INT_1`"]
    #[inline(always)]
    pub fn is_sf_or_int_1(&self) -> bool {
        *self == SF_OR_INT_A::SF_OR_INT_1
    }
}
#[doc = "Write proxy for field `SF_OR_INT`"]
pub struct SF_OR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_OR_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SF_OR_INT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "STATFIFO has not overflowed."]
    #[inline(always)]
    pub fn sf_or_int_0(self) -> &'a mut W {
        self.variant(SF_OR_INT_A::SF_OR_INT_0)
    }
    #[doc = "STATFIFO has overflowed."]
    #[inline(always)]
    pub fn sf_or_int_1(self) -> &'a mut W {
        self.variant(SF_OR_INT_A::SF_OR_INT_1)
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
#[doc = "Reader of field `DMA_FIELD1_DONE`"]
pub type DMA_FIELD1_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_FIELD1_DONE`"]
pub struct DMA_FIELD1_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_FIELD1_DONE_W<'a> {
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
#[doc = "Reader of field `DMA_FIELD0_DONE`"]
pub type DMA_FIELD0_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_FIELD0_DONE`"]
pub struct DMA_FIELD0_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_FIELD0_DONE_W<'a> {
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
#[doc = "Reader of field `BASEADDR_CHHANGE_ERROR`"]
pub type BASEADDR_CHHANGE_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BASEADDR_CHHANGE_ERROR`"]
pub struct BASEADDR_CHHANGE_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASEADDR_CHHANGE_ERROR_W<'a> {
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
    #[doc = "Bit 0 - RXFIFO Data Ready"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CCIR Error Interrupt"]
    #[inline(always)]
    pub fn ecc_int(&self) -> ECC_INT_R {
        ECC_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Hresponse Error Interrupt Status"]
    #[inline(always)]
    pub fn hresp_err_int(&self) -> HRESP_ERR_INT_R {
        HRESP_ERR_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Change Of Field Interrupt Status"]
    #[inline(always)]
    pub fn cof_int(&self) -> COF_INT_R {
        COF_INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CCIR Field 1 Interrupt Status"]
    #[inline(always)]
    pub fn f1_int(&self) -> F1_INT_R {
        F1_INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CCIR Field 2 Interrupt Status"]
    #[inline(always)]
    pub fn f2_int(&self) -> F2_INT_R {
        F2_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1)"]
    #[inline(always)]
    pub fn sof_int(&self) -> SOF_INT_R {
        SOF_INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1)"]
    #[inline(always)]
    pub fn eof_int(&self) -> EOF_INT_R {
        EOF_INT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RXFIFO Full Interrupt Status"]
    #[inline(always)]
    pub fn rx_ff_int(&self) -> RXFF_INT_R {
        RXFF_INT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DMA Transfer Done in Frame Buffer1"]
    #[inline(always)]
    pub fn dma_tsf_done_fb1(&self) -> DMA_TSF_DONE_FB1_R {
        DMA_TSF_DONE_FB1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DMA Transfer Done in Frame Buffer2"]
    #[inline(always)]
    pub fn dma_tsf_done_fb2(&self) -> DMA_TSF_DONE_FB2_R {
        DMA_TSF_DONE_FB2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - STATFIFO Full Interrupt Status"]
    #[inline(always)]
    pub fn statff_int(&self) -> STATFF_INT_R {
        STATFF_INT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DMA Transfer Done from StatFIFO"]
    #[inline(always)]
    pub fn dma_tsf_done_sff(&self) -> DMA_TSF_DONE_SFF_R {
        DMA_TSF_DONE_SFF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RxFIFO Overrun Interrupt Status"]
    #[inline(always)]
    pub fn rf_or_int(&self) -> RF_OR_INT_R {
        RF_OR_INT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - STATFIFO Overrun Interrupt Status"]
    #[inline(always)]
    pub fn sf_or_int(&self) -> SF_OR_INT_R {
        SF_OR_INT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - When DMA field 0 is complete, this bit will be set to 1(clear by writing 1)."]
    #[inline(always)]
    pub fn dma_field1_done(&self) -> DMA_FIELD1_DONE_R {
        DMA_FIELD1_DONE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - When DMA field 0 is complete, this bit will be set to 1(clear by writing 1)."]
    #[inline(always)]
    pub fn dma_field0_done(&self) -> DMA_FIELD0_DONE_R {
        DMA_FIELD0_DONE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - When using base address switching enable, this bit will be 1 when switching occur before DMA complete"]
    #[inline(always)]
    pub fn baseaddr_chhange_error(&self) -> BASEADDR_CHHANGE_ERROR_R {
        BASEADDR_CHHANGE_ERROR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXFIFO Data Ready"]
    #[inline(always)]
    pub fn drdy(&mut self) -> DRDY_W {
        DRDY_W { w: self }
    }
    #[doc = "Bit 1 - CCIR Error Interrupt"]
    #[inline(always)]
    pub fn ecc_int(&mut self) -> ECC_INT_W {
        ECC_INT_W { w: self }
    }
    #[doc = "Bit 7 - Hresponse Error Interrupt Status"]
    #[inline(always)]
    pub fn hresp_err_int(&mut self) -> HRESP_ERR_INT_W {
        HRESP_ERR_INT_W { w: self }
    }
    #[doc = "Bit 13 - Change Of Field Interrupt Status"]
    #[inline(always)]
    pub fn cof_int(&mut self) -> COF_INT_W {
        COF_INT_W { w: self }
    }
    #[doc = "Bit 14 - CCIR Field 1 Interrupt Status"]
    #[inline(always)]
    pub fn f1_int(&mut self) -> F1_INT_W {
        F1_INT_W { w: self }
    }
    #[doc = "Bit 15 - CCIR Field 2 Interrupt Status"]
    #[inline(always)]
    pub fn f2_int(&mut self) -> F2_INT_W {
        F2_INT_W { w: self }
    }
    #[doc = "Bit 16 - Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1)"]
    #[inline(always)]
    pub fn sof_int(&mut self) -> SOF_INT_W {
        SOF_INT_W { w: self }
    }
    #[doc = "Bit 17 - End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1)"]
    #[inline(always)]
    pub fn eof_int(&mut self) -> EOF_INT_W {
        EOF_INT_W { w: self }
    }
    #[doc = "Bit 18 - RXFIFO Full Interrupt Status"]
    #[inline(always)]
    pub fn rx_ff_int(&mut self) -> RXFF_INT_W {
        RXFF_INT_W { w: self }
    }
    #[doc = "Bit 19 - DMA Transfer Done in Frame Buffer1"]
    #[inline(always)]
    pub fn dma_tsf_done_fb1(&mut self) -> DMA_TSF_DONE_FB1_W {
        DMA_TSF_DONE_FB1_W { w: self }
    }
    #[doc = "Bit 20 - DMA Transfer Done in Frame Buffer2"]
    #[inline(always)]
    pub fn dma_tsf_done_fb2(&mut self) -> DMA_TSF_DONE_FB2_W {
        DMA_TSF_DONE_FB2_W { w: self }
    }
    #[doc = "Bit 21 - STATFIFO Full Interrupt Status"]
    #[inline(always)]
    pub fn statff_int(&mut self) -> STATFF_INT_W {
        STATFF_INT_W { w: self }
    }
    #[doc = "Bit 22 - DMA Transfer Done from StatFIFO"]
    #[inline(always)]
    pub fn dma_tsf_done_sff(&mut self) -> DMA_TSF_DONE_SFF_W {
        DMA_TSF_DONE_SFF_W { w: self }
    }
    #[doc = "Bit 24 - RxFIFO Overrun Interrupt Status"]
    #[inline(always)]
    pub fn rf_or_int(&mut self) -> RF_OR_INT_W {
        RF_OR_INT_W { w: self }
    }
    #[doc = "Bit 25 - STATFIFO Overrun Interrupt Status"]
    #[inline(always)]
    pub fn sf_or_int(&mut self) -> SF_OR_INT_W {
        SF_OR_INT_W { w: self }
    }
    #[doc = "Bit 26 - When DMA field 0 is complete, this bit will be set to 1(clear by writing 1)."]
    #[inline(always)]
    pub fn dma_field1_done(&mut self) -> DMA_FIELD1_DONE_W {
        DMA_FIELD1_DONE_W { w: self }
    }
    #[doc = "Bit 27 - When DMA field 0 is complete, this bit will be set to 1(clear by writing 1)."]
    #[inline(always)]
    pub fn dma_field0_done(&mut self) -> DMA_FIELD0_DONE_W {
        DMA_FIELD0_DONE_W { w: self }
    }
    #[doc = "Bit 28 - When using base address switching enable, this bit will be 1 when switching occur before DMA complete"]
    #[inline(always)]
    pub fn baseaddr_chhange_error(&mut self) -> BASEADDR_CHHANGE_ERROR_W {
        BASEADDR_CHHANGE_ERROR_W { w: self }
    }
}
