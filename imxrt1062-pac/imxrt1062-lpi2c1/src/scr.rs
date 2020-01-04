#[doc = "Reader of register SCR"]
pub type R = crate::R<u32, super::SCR>;
#[doc = "Writer for register SCR"]
pub type W = crate::W<u32, super::SCR>;
#[doc = "Register SCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Slave Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN_A {
    #[doc = "0: I2C Slave mode is disabled"]
    SEN_0 = 0,
    #[doc = "1: I2C Slave mode is enabled"]
    SEN_1 = 1,
}
impl From<SEN_A> for bool {
    #[inline(always)]
    fn from(variant: SEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEN`"]
pub type SEN_R = crate::R<bool, SEN_A>;
impl SEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEN_A {
        match self.bits {
            false => SEN_A::SEN_0,
            true => SEN_A::SEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEN_0`"]
    #[inline(always)]
    pub fn is_sen_0(&self) -> bool {
        *self == SEN_A::SEN_0
    }
    #[doc = "Checks if the value of the field is `SEN_1`"]
    #[inline(always)]
    pub fn is_sen_1(&self) -> bool {
        *self == SEN_A::SEN_1
    }
}
#[doc = "Write proxy for field `SEN`"]
pub struct SEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C Slave mode is disabled"]
    #[inline(always)]
    pub fn sen_0(self) -> &'a mut W {
        self.variant(SEN_A::SEN_0)
    }
    #[doc = "I2C Slave mode is enabled"]
    #[inline(always)]
    pub fn sen_1(self) -> &'a mut W {
        self.variant(SEN_A::SEN_1)
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
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
    #[doc = "0: Slave mode logic is not reset"]
    RST_0 = 0,
    #[doc = "1: Slave mode logic is reset"]
    RST_1 = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<bool, RST_A>;
impl RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::RST_0,
            true => RST_A::RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `RST_0`"]
    #[inline(always)]
    pub fn is_rst_0(&self) -> bool {
        *self == RST_A::RST_0
    }
    #[doc = "Checks if the value of the field is `RST_1`"]
    #[inline(always)]
    pub fn is_rst_1(&self) -> bool {
        *self == RST_A::RST_1
    }
}
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave mode logic is not reset"]
    #[inline(always)]
    pub fn rst_0(self) -> &'a mut W {
        self.variant(RST_A::RST_0)
    }
    #[doc = "Slave mode logic is reset"]
    #[inline(always)]
    pub fn rst_1(self) -> &'a mut W {
        self.variant(RST_A::RST_1)
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
#[doc = "Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTEN_A {
    #[doc = "0: Disable digital filter and output delay counter for slave mode"]
    FILTEN_0 = 0,
    #[doc = "1: Enable digital filter and output delay counter for slave mode"]
    FILTEN_1 = 1,
}
impl From<FILTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FILTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FILTEN`"]
pub type FILTEN_R = crate::R<bool, FILTEN_A>;
impl FILTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTEN_A {
        match self.bits {
            false => FILTEN_A::FILTEN_0,
            true => FILTEN_A::FILTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FILTEN_0`"]
    #[inline(always)]
    pub fn is_filten_0(&self) -> bool {
        *self == FILTEN_A::FILTEN_0
    }
    #[doc = "Checks if the value of the field is `FILTEN_1`"]
    #[inline(always)]
    pub fn is_filten_1(&self) -> bool {
        *self == FILTEN_A::FILTEN_1
    }
}
#[doc = "Write proxy for field `FILTEN`"]
pub struct FILTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable digital filter and output delay counter for slave mode"]
    #[inline(always)]
    pub fn filten_0(self) -> &'a mut W {
        self.variant(FILTEN_A::FILTEN_0)
    }
    #[doc = "Enable digital filter and output delay counter for slave mode"]
    #[inline(always)]
    pub fn filten_1(self) -> &'a mut W {
        self.variant(FILTEN_A::FILTEN_1)
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
#[doc = "Filter Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTDZ_A {
    #[doc = "0: Filter remains enabled in Doze mode"]
    FILTDZ_0 = 0,
    #[doc = "1: Filter is disabled in Doze mode"]
    FILTDZ_1 = 1,
}
impl From<FILTDZ_A> for bool {
    #[inline(always)]
    fn from(variant: FILTDZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FILTDZ`"]
pub type FILTDZ_R = crate::R<bool, FILTDZ_A>;
impl FILTDZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTDZ_A {
        match self.bits {
            false => FILTDZ_A::FILTDZ_0,
            true => FILTDZ_A::FILTDZ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FILTDZ_0`"]
    #[inline(always)]
    pub fn is_filtdz_0(&self) -> bool {
        *self == FILTDZ_A::FILTDZ_0
    }
    #[doc = "Checks if the value of the field is `FILTDZ_1`"]
    #[inline(always)]
    pub fn is_filtdz_1(&self) -> bool {
        *self == FILTDZ_A::FILTDZ_1
    }
}
#[doc = "Write proxy for field `FILTDZ`"]
pub struct FILTDZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTDZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTDZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Filter remains enabled in Doze mode"]
    #[inline(always)]
    pub fn filtdz_0(self) -> &'a mut W {
        self.variant(FILTDZ_A::FILTDZ_0)
    }
    #[doc = "Filter is disabled in Doze mode"]
    #[inline(always)]
    pub fn filtdz_1(self) -> &'a mut W {
        self.variant(FILTDZ_A::FILTDZ_1)
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
#[doc = "Reset Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTF_A {
    #[doc = "0: No effect"]
    RTF_0 = 0,
    #[doc = "1: Transmit Data Register is now empty"]
    RTF_1 = 1,
}
impl From<RTF_A> for bool {
    #[inline(always)]
    fn from(variant: RTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTF`"]
pub type RTF_R = crate::R<bool, RTF_A>;
impl RTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTF_A {
        match self.bits {
            false => RTF_A::RTF_0,
            true => RTF_A::RTF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTF_0`"]
    #[inline(always)]
    pub fn is_rtf_0(&self) -> bool {
        *self == RTF_A::RTF_0
    }
    #[doc = "Checks if the value of the field is `RTF_1`"]
    #[inline(always)]
    pub fn is_rtf_1(&self) -> bool {
        *self == RTF_A::RTF_1
    }
}
#[doc = "Write proxy for field `RTF`"]
pub struct RTF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn rtf_0(self) -> &'a mut W {
        self.variant(RTF_A::RTF_0)
    }
    #[doc = "Transmit Data Register is now empty"]
    #[inline(always)]
    pub fn rtf_1(self) -> &'a mut W {
        self.variant(RTF_A::RTF_1)
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
#[doc = "Reset Receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRF_A {
    #[doc = "0: No effect"]
    RRF_0 = 0,
    #[doc = "1: Receive Data Register is now empty"]
    RRF_1 = 1,
}
impl From<RRF_A> for bool {
    #[inline(always)]
    fn from(variant: RRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RRF`"]
pub type RRF_R = crate::R<bool, RRF_A>;
impl RRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRF_A {
        match self.bits {
            false => RRF_A::RRF_0,
            true => RRF_A::RRF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RRF_0`"]
    #[inline(always)]
    pub fn is_rrf_0(&self) -> bool {
        *self == RRF_A::RRF_0
    }
    #[doc = "Checks if the value of the field is `RRF_1`"]
    #[inline(always)]
    pub fn is_rrf_1(&self) -> bool {
        *self == RRF_A::RRF_1
    }
}
#[doc = "Write proxy for field `RRF`"]
pub struct RRF_W<'a> {
    w: &'a mut W,
}
impl<'a> RRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn rrf_0(self) -> &'a mut W {
        self.variant(RRF_A::RRF_0)
    }
    #[doc = "Receive Data Register is now empty"]
    #[inline(always)]
    pub fn rrf_1(self) -> &'a mut W {
        self.variant(RRF_A::RRF_1)
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
impl R {
    #[doc = "Bit 0 - Slave Enable"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter Enable"]
    #[inline(always)]
    pub fn filten(&self) -> FILTEN_R {
        FILTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter Doze Enable"]
    #[inline(always)]
    pub fn filtdz(&self) -> FILTDZ_R {
        FILTDZ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reset Transmit FIFO"]
    #[inline(always)]
    pub fn rtf(&self) -> RTF_R {
        RTF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reset Receive FIFO"]
    #[inline(always)]
    pub fn rrf(&self) -> RRF_R {
        RRF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Enable"]
    #[inline(always)]
    pub fn sen(&mut self) -> SEN_W {
        SEN_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 4 - Filter Enable"]
    #[inline(always)]
    pub fn filten(&mut self) -> FILTEN_W {
        FILTEN_W { w: self }
    }
    #[doc = "Bit 5 - Filter Doze Enable"]
    #[inline(always)]
    pub fn filtdz(&mut self) -> FILTDZ_W {
        FILTDZ_W { w: self }
    }
    #[doc = "Bit 8 - Reset Transmit FIFO"]
    #[inline(always)]
    pub fn rtf(&mut self) -> RTF_W {
        RTF_W { w: self }
    }
    #[doc = "Bit 9 - Reset Receive FIFO"]
    #[inline(always)]
    pub fn rrf(&mut self) -> RRF_W {
        RRF_W { w: self }
    }
}
