#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEN_A {
    #[doc = "0: Module is disabled"]
    MEN_0 = 0,
    #[doc = "1: Module is enabled"]
    MEN_1 = 1,
}
impl From<MEN_A> for bool {
    #[inline(always)]
    fn from(variant: MEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEN`"]
pub type MEN_R = crate::R<bool, MEN_A>;
impl MEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEN_A {
        match self.bits {
            false => MEN_A::MEN_0,
            true => MEN_A::MEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEN_0`"]
    #[inline(always)]
    pub fn is_men_0(&self) -> bool {
        *self == MEN_A::MEN_0
    }
    #[doc = "Checks if the value of the field is `MEN_1`"]
    #[inline(always)]
    pub fn is_men_1(&self) -> bool {
        *self == MEN_A::MEN_1
    }
}
#[doc = "Write proxy for field `MEN`"]
pub struct MEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Module is disabled"]
    #[inline(always)]
    pub fn men_0(self) -> &'a mut W {
        self.variant(MEN_A::MEN_0)
    }
    #[doc = "Module is enabled"]
    #[inline(always)]
    pub fn men_1(self) -> &'a mut W {
        self.variant(MEN_A::MEN_1)
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
    #[doc = "0: Module is not reset"]
    RST_0 = 0,
    #[doc = "1: Module is reset"]
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
    #[doc = "Module is not reset"]
    #[inline(always)]
    pub fn rst_0(self) -> &'a mut W {
        self.variant(RST_A::RST_0)
    }
    #[doc = "Module is reset"]
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
#[doc = "Doze mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEN_A {
    #[doc = "0: Module is enabled in Doze mode"]
    DOZEN_0 = 0,
    #[doc = "1: Module is disabled in Doze mode"]
    DOZEN_1 = 1,
}
impl From<DOZEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOZEN`"]
pub type DOZEN_R = crate::R<bool, DOZEN_A>;
impl DOZEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEN_A {
        match self.bits {
            false => DOZEN_A::DOZEN_0,
            true => DOZEN_A::DOZEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEN_0`"]
    #[inline(always)]
    pub fn is_dozen_0(&self) -> bool {
        *self == DOZEN_A::DOZEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEN_1`"]
    #[inline(always)]
    pub fn is_dozen_1(&self) -> bool {
        *self == DOZEN_A::DOZEN_1
    }
}
#[doc = "Write proxy for field `DOZEN`"]
pub struct DOZEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Module is enabled in Doze mode"]
    #[inline(always)]
    pub fn dozen_0(self) -> &'a mut W {
        self.variant(DOZEN_A::DOZEN_0)
    }
    #[doc = "Module is disabled in Doze mode"]
    #[inline(always)]
    pub fn dozen_1(self) -> &'a mut W {
        self.variant(DOZEN_A::DOZEN_1)
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
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGEN_A {
    #[doc = "0: Module is disabled in debug mode"]
    DBGEN_0 = 0,
    #[doc = "1: Module is enabled in debug mode"]
    DBGEN_1 = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBGEN`"]
pub type DBGEN_R = crate::R<bool, DBGEN_A>;
impl DBGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::DBGEN_0,
            true => DBGEN_A::DBGEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGEN_0`"]
    #[inline(always)]
    pub fn is_dbgen_0(&self) -> bool {
        *self == DBGEN_A::DBGEN_0
    }
    #[doc = "Checks if the value of the field is `DBGEN_1`"]
    #[inline(always)]
    pub fn is_dbgen_1(&self) -> bool {
        *self == DBGEN_A::DBGEN_1
    }
}
#[doc = "Write proxy for field `DBGEN`"]
pub struct DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Module is disabled in debug mode"]
    #[inline(always)]
    pub fn dbgen_0(self) -> &'a mut W {
        self.variant(DBGEN_A::DBGEN_0)
    }
    #[doc = "Module is enabled in debug mode"]
    #[inline(always)]
    pub fn dbgen_1(self) -> &'a mut W {
        self.variant(DBGEN_A::DBGEN_1)
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
#[doc = "Reset Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTF_AW {
    #[doc = "0: No effect"]
    RTF_0 = 0,
    #[doc = "1: Transmit FIFO is reset"]
    RTF_1 = 1,
}
impl From<RTF_AW> for bool {
    #[inline(always)]
    fn from(variant: RTF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RTF`"]
pub struct RTF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn rtf_0(self) -> &'a mut W {
        self.variant(RTF_AW::RTF_0)
    }
    #[doc = "Transmit FIFO is reset"]
    #[inline(always)]
    pub fn rtf_1(self) -> &'a mut W {
        self.variant(RTF_AW::RTF_1)
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
pub enum RRF_AW {
    #[doc = "0: No effect"]
    RRF_0 = 0,
    #[doc = "1: Receive FIFO is reset"]
    RRF_1 = 1,
}
impl From<RRF_AW> for bool {
    #[inline(always)]
    fn from(variant: RRF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RRF`"]
pub struct RRF_W<'a> {
    w: &'a mut W,
}
impl<'a> RRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn rrf_0(self) -> &'a mut W {
        self.variant(RRF_AW::RRF_0)
    }
    #[doc = "Receive FIFO is reset"]
    #[inline(always)]
    pub fn rrf_1(self) -> &'a mut W {
        self.variant(RRF_AW::RRF_1)
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
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn men(&self) -> MEN_R {
        MEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Doze mode enable"]
    #[inline(always)]
    pub fn dozen(&self) -> DOZEN_R {
        DOZEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn men(&mut self) -> MEN_W {
        MEN_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 2 - Doze mode enable"]
    #[inline(always)]
    pub fn dozen(&mut self) -> DOZEN_W {
        DOZEN_W { w: self }
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
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
