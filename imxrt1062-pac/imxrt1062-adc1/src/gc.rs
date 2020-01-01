#[doc = "Reader of register GC"]
pub type R = crate::R<u32, super::GC>;
#[doc = "Writer for register GC"]
pub type W = crate::W<u32, super::GC>;
#[doc = "Register GC `reset()`'s with value 0"]
impl crate::ResetValue for super::GC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Asynchronous clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADACKEN_A {
    #[doc = "0: Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active."]
    ADACKEN_0 = 0,
    #[doc = "1: Asynchronous clock and clock output enabled regardless of the state of the ADC"]
    ADACKEN_1 = 1,
}
impl From<ADACKEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADACKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADACKEN`"]
pub type ADACKEN_R = crate::R<bool, ADACKEN_A>;
impl ADACKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACKEN_A {
        match self.bits {
            false => ADACKEN_A::ADACKEN_0,
            true => ADACKEN_A::ADACKEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADACKEN_0`"]
    #[inline(always)]
    pub fn is_adacken_0(&self) -> bool {
        *self == ADACKEN_A::ADACKEN_0
    }
    #[doc = "Checks if the value of the field is `ADACKEN_1`"]
    #[inline(always)]
    pub fn is_adacken_1(&self) -> bool {
        *self == ADACKEN_A::ADACKEN_1
    }
}
#[doc = "Write proxy for field `ADACKEN`"]
pub struct ADACKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADACKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADACKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active."]
    #[inline(always)]
    pub fn adacken_0(self) -> &'a mut W {
        self.variant(ADACKEN_A::ADACKEN_0)
    }
    #[doc = "Asynchronous clock and clock output enabled regardless of the state of the ADC"]
    #[inline(always)]
    pub fn adacken_1(self) -> &'a mut W {
        self.variant(ADACKEN_A::ADACKEN_1)
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
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA disabled (default)"]
    DMAEN_0 = 0,
    #[doc = "1: DMA enabled"]
    DMAEN_1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DMAEN_0,
            true => DMAEN_A::DMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAEN_0`"]
    #[inline(always)]
    pub fn is_dmaen_0(&self) -> bool {
        *self == DMAEN_A::DMAEN_0
    }
    #[doc = "Checks if the value of the field is `DMAEN_1`"]
    #[inline(always)]
    pub fn is_dmaen_1(&self) -> bool {
        *self == DMAEN_A::DMAEN_1
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA disabled (default)"]
    #[inline(always)]
    pub fn dmaen_0(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn dmaen_1(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_1)
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
#[doc = "Compare Function Range Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACREN_A {
    #[doc = "0: Range function disabled. Only the compare value 1 of ADC_CV register (CV1) is compared."]
    ACREN_0 = 0,
    #[doc = "1: Range function enabled. Both compare values of ADC_CV registers (CV1 and CV2) are compared."]
    ACREN_1 = 1,
}
impl From<ACREN_A> for bool {
    #[inline(always)]
    fn from(variant: ACREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACREN`"]
pub type ACREN_R = crate::R<bool, ACREN_A>;
impl ACREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACREN_A {
        match self.bits {
            false => ACREN_A::ACREN_0,
            true => ACREN_A::ACREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACREN_0`"]
    #[inline(always)]
    pub fn is_acren_0(&self) -> bool {
        *self == ACREN_A::ACREN_0
    }
    #[doc = "Checks if the value of the field is `ACREN_1`"]
    #[inline(always)]
    pub fn is_acren_1(&self) -> bool {
        *self == ACREN_A::ACREN_1
    }
}
#[doc = "Write proxy for field `ACREN`"]
pub struct ACREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Range function disabled. Only the compare value 1 of ADC_CV register (CV1) is compared."]
    #[inline(always)]
    pub fn acren_0(self) -> &'a mut W {
        self.variant(ACREN_A::ACREN_0)
    }
    #[doc = "Range function enabled. Both compare values of ADC_CV registers (CV1 and CV2) are compared."]
    #[inline(always)]
    pub fn acren_1(self) -> &'a mut W {
        self.variant(ACREN_A::ACREN_1)
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
#[doc = "Compare Function Greater Than Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACFGT_A {
    #[doc = "0: Configures \"Less Than Threshold, Outside Range Not Inclusive and Inside Range Not Inclusive\" functionality based on the values placed in the ADC_CV register."]
    ACFGT_0 = 0,
    #[doc = "1: Configures \"Greater Than Or Equal To Threshold, Outside Range Inclusive and Inside Range Inclusive\" functionality based on the values placed in the ADC_CV registers."]
    ACFGT_1 = 1,
}
impl From<ACFGT_A> for bool {
    #[inline(always)]
    fn from(variant: ACFGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACFGT`"]
pub type ACFGT_R = crate::R<bool, ACFGT_A>;
impl ACFGT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACFGT_A {
        match self.bits {
            false => ACFGT_A::ACFGT_0,
            true => ACFGT_A::ACFGT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACFGT_0`"]
    #[inline(always)]
    pub fn is_acfgt_0(&self) -> bool {
        *self == ACFGT_A::ACFGT_0
    }
    #[doc = "Checks if the value of the field is `ACFGT_1`"]
    #[inline(always)]
    pub fn is_acfgt_1(&self) -> bool {
        *self == ACFGT_A::ACFGT_1
    }
}
#[doc = "Write proxy for field `ACFGT`"]
pub struct ACFGT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACFGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACFGT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configures \"Less Than Threshold, Outside Range Not Inclusive and Inside Range Not Inclusive\" functionality based on the values placed in the ADC_CV register."]
    #[inline(always)]
    pub fn acfgt_0(self) -> &'a mut W {
        self.variant(ACFGT_A::ACFGT_0)
    }
    #[doc = "Configures \"Greater Than Or Equal To Threshold, Outside Range Inclusive and Inside Range Inclusive\" functionality based on the values placed in the ADC_CV registers."]
    #[inline(always)]
    pub fn acfgt_1(self) -> &'a mut W {
        self.variant(ACFGT_A::ACFGT_1)
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
#[doc = "Compare Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACFE_A {
    #[doc = "0: Compare function disabled"]
    ACFE_0 = 0,
    #[doc = "1: Compare function enabled"]
    ACFE_1 = 1,
}
impl From<ACFE_A> for bool {
    #[inline(always)]
    fn from(variant: ACFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACFE`"]
pub type ACFE_R = crate::R<bool, ACFE_A>;
impl ACFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACFE_A {
        match self.bits {
            false => ACFE_A::ACFE_0,
            true => ACFE_A::ACFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACFE_0`"]
    #[inline(always)]
    pub fn is_acfe_0(&self) -> bool {
        *self == ACFE_A::ACFE_0
    }
    #[doc = "Checks if the value of the field is `ACFE_1`"]
    #[inline(always)]
    pub fn is_acfe_1(&self) -> bool {
        *self == ACFE_A::ACFE_1
    }
}
#[doc = "Write proxy for field `ACFE`"]
pub struct ACFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare function disabled"]
    #[inline(always)]
    pub fn acfe_0(self) -> &'a mut W {
        self.variant(ACFE_A::ACFE_0)
    }
    #[doc = "Compare function enabled"]
    #[inline(always)]
    pub fn acfe_1(self) -> &'a mut W {
        self.variant(ACFE_A::ACFE_1)
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
#[doc = "Hardware average enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVGE_A {
    #[doc = "0: Hardware average function disabled"]
    AVGE_0 = 0,
    #[doc = "1: Hardware average function enabled"]
    AVGE_1 = 1,
}
impl From<AVGE_A> for bool {
    #[inline(always)]
    fn from(variant: AVGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AVGE`"]
pub type AVGE_R = crate::R<bool, AVGE_A>;
impl AVGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVGE_A {
        match self.bits {
            false => AVGE_A::AVGE_0,
            true => AVGE_A::AVGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AVGE_0`"]
    #[inline(always)]
    pub fn is_avge_0(&self) -> bool {
        *self == AVGE_A::AVGE_0
    }
    #[doc = "Checks if the value of the field is `AVGE_1`"]
    #[inline(always)]
    pub fn is_avge_1(&self) -> bool {
        *self == AVGE_A::AVGE_1
    }
}
#[doc = "Write proxy for field `AVGE`"]
pub struct AVGE_W<'a> {
    w: &'a mut W,
}
impl<'a> AVGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware average function disabled"]
    #[inline(always)]
    pub fn avge_0(self) -> &'a mut W {
        self.variant(AVGE_A::AVGE_0)
    }
    #[doc = "Hardware average function enabled"]
    #[inline(always)]
    pub fn avge_1(self) -> &'a mut W {
        self.variant(AVGE_A::AVGE_1)
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
#[doc = "Continuous Conversion Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCO_A {
    #[doc = "0: One conversion or one set of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    ADCO_0 = 0,
    #[doc = "1: Continuous conversions or sets of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    ADCO_1 = 1,
}
impl From<ADCO_A> for bool {
    #[inline(always)]
    fn from(variant: ADCO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCO`"]
pub type ADCO_R = crate::R<bool, ADCO_A>;
impl ADCO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCO_A {
        match self.bits {
            false => ADCO_A::ADCO_0,
            true => ADCO_A::ADCO_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCO_0`"]
    #[inline(always)]
    pub fn is_adco_0(&self) -> bool {
        *self == ADCO_A::ADCO_0
    }
    #[doc = "Checks if the value of the field is `ADCO_1`"]
    #[inline(always)]
    pub fn is_adco_1(&self) -> bool {
        *self == ADCO_A::ADCO_1
    }
}
#[doc = "Write proxy for field `ADCO`"]
pub struct ADCO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One conversion or one set of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    #[inline(always)]
    pub fn adco_0(self) -> &'a mut W {
        self.variant(ADCO_A::ADCO_0)
    }
    #[doc = "Continuous conversions or sets of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    #[inline(always)]
    pub fn adco_1(self) -> &'a mut W {
        self.variant(ADCO_A::ADCO_1)
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
#[doc = "Reader of field `CAL`"]
pub type CAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAL`"]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Asynchronous clock output enable"]
    #[inline(always)]
    pub fn adacken(&self) -> ADACKEN_R {
        ADACKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Compare Function Range Enable"]
    #[inline(always)]
    pub fn acren(&self) -> ACREN_R {
        ACREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Compare Function Greater Than Enable"]
    #[inline(always)]
    pub fn acfgt(&self) -> ACFGT_R {
        ACFGT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Compare Function Enable"]
    #[inline(always)]
    pub fn acfe(&self) -> ACFE_R {
        ACFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hardware average enable"]
    #[inline(always)]
    pub fn avge(&self) -> AVGE_R {
        AVGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Continuous Conversion Enable"]
    #[inline(always)]
    pub fn adco(&self) -> ADCO_R {
        ADCO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asynchronous clock output enable"]
    #[inline(always)]
    pub fn adacken(&mut self) -> ADACKEN_W {
        ADACKEN_W { w: self }
    }
    #[doc = "Bit 1 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 2 - Compare Function Range Enable"]
    #[inline(always)]
    pub fn acren(&mut self) -> ACREN_W {
        ACREN_W { w: self }
    }
    #[doc = "Bit 3 - Compare Function Greater Than Enable"]
    #[inline(always)]
    pub fn acfgt(&mut self) -> ACFGT_W {
        ACFGT_W { w: self }
    }
    #[doc = "Bit 4 - Compare Function Enable"]
    #[inline(always)]
    pub fn acfe(&mut self) -> ACFE_W {
        ACFE_W { w: self }
    }
    #[doc = "Bit 5 - Hardware average enable"]
    #[inline(always)]
    pub fn avge(&mut self) -> AVGE_W {
        AVGE_W { w: self }
    }
    #[doc = "Bit 6 - Continuous Conversion Enable"]
    #[inline(always)]
    pub fn adco(&mut self) -> ADCO_W {
        ADCO_W { w: self }
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
}
