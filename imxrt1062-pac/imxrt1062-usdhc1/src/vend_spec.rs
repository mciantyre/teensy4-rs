#[doc = "Reader of register VEND_SPEC"]
pub type R = crate::R<u32, super::VEND_SPEC>;
#[doc = "Writer for register VEND_SPEC"]
pub type W = crate::W<u32, super::VEND_SPEC>;
#[doc = "Register VEND_SPEC `reset()`'s with value 0x2000_7809"]
impl crate::ResetValue for super::VEND_SPEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_7809
    }
}
#[doc = "Voltage Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSELECT_A {
    #[doc = "0: Change the voltage to high voltage range, around 3.0 V"]
    VSELECT_0 = 0,
    #[doc = "1: Change the voltage to low voltage range, around 1.8 V"]
    VSELECT_1 = 1,
}
impl From<VSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: VSELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VSELECT`"]
pub type VSELECT_R = crate::R<bool, VSELECT_A>;
impl VSELECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSELECT_A {
        match self.bits {
            false => VSELECT_A::VSELECT_0,
            true => VSELECT_A::VSELECT_1,
        }
    }
    #[doc = "Checks if the value of the field is `VSELECT_0`"]
    #[inline(always)]
    pub fn is_vselect_0(&self) -> bool {
        *self == VSELECT_A::VSELECT_0
    }
    #[doc = "Checks if the value of the field is `VSELECT_1`"]
    #[inline(always)]
    pub fn is_vselect_1(&self) -> bool {
        *self == VSELECT_A::VSELECT_1
    }
}
#[doc = "Write proxy for field `VSELECT`"]
pub struct VSELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> VSELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VSELECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Change the voltage to high voltage range, around 3.0 V"]
    #[inline(always)]
    pub fn vselect_0(self) -> &'a mut W {
        self.variant(VSELECT_A::VSELECT_0)
    }
    #[doc = "Change the voltage to low voltage range, around 1.8 V"]
    #[inline(always)]
    pub fn vselect_1(self) -> &'a mut W {
        self.variant(VSELECT_A::VSELECT_1)
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
#[doc = "Conflict check enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONFLICT_CHK_EN_A {
    #[doc = "0: Conflict check disable"]
    CONFLICT_CHK_EN_0 = 0,
    #[doc = "1: Conflict check enable"]
    CONFLICT_CHK_EN_1 = 1,
}
impl From<CONFLICT_CHK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CONFLICT_CHK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONFLICT_CHK_EN`"]
pub type CONFLICT_CHK_EN_R = crate::R<bool, CONFLICT_CHK_EN_A>;
impl CONFLICT_CHK_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONFLICT_CHK_EN_A {
        match self.bits {
            false => CONFLICT_CHK_EN_A::CONFLICT_CHK_EN_0,
            true => CONFLICT_CHK_EN_A::CONFLICT_CHK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CONFLICT_CHK_EN_0`"]
    #[inline(always)]
    pub fn is_conflict_chk_en_0(&self) -> bool {
        *self == CONFLICT_CHK_EN_A::CONFLICT_CHK_EN_0
    }
    #[doc = "Checks if the value of the field is `CONFLICT_CHK_EN_1`"]
    #[inline(always)]
    pub fn is_conflict_chk_en_1(&self) -> bool {
        *self == CONFLICT_CHK_EN_A::CONFLICT_CHK_EN_1
    }
}
#[doc = "Write proxy for field `CONFLICT_CHK_EN`"]
pub struct CONFLICT_CHK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFLICT_CHK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONFLICT_CHK_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Conflict check disable"]
    #[inline(always)]
    pub fn conflict_chk_en_0(self) -> &'a mut W {
        self.variant(CONFLICT_CHK_EN_A::CONFLICT_CHK_EN_0)
    }
    #[doc = "Conflict check enable"]
    #[inline(always)]
    pub fn conflict_chk_en_1(self) -> &'a mut W {
        self.variant(CONFLICT_CHK_EN_A::CONFLICT_CHK_EN_1)
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
#[doc = "AC12_WR_CHKBUSY_EN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12_WR_CHKBUSY_EN_A {
    #[doc = "0: Do not check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_0 = 0,
    #[doc = "1: Check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_1 = 1,
}
impl From<AC12_WR_CHKBUSY_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AC12_WR_CHKBUSY_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AC12_WR_CHKBUSY_EN`"]
pub type AC12_WR_CHKBUSY_EN_R = crate::R<bool, AC12_WR_CHKBUSY_EN_A>;
impl AC12_WR_CHKBUSY_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12_WR_CHKBUSY_EN_A {
        match self.bits {
            false => AC12_WR_CHKBUSY_EN_A::AC12_WR_CHKBUSY_EN_0,
            true => AC12_WR_CHKBUSY_EN_A::AC12_WR_CHKBUSY_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12_WR_CHKBUSY_EN_0`"]
    #[inline(always)]
    pub fn is_ac12_wr_chkbusy_en_0(&self) -> bool {
        *self == AC12_WR_CHKBUSY_EN_A::AC12_WR_CHKBUSY_EN_0
    }
    #[doc = "Checks if the value of the field is `AC12_WR_CHKBUSY_EN_1`"]
    #[inline(always)]
    pub fn is_ac12_wr_chkbusy_en_1(&self) -> bool {
        *self == AC12_WR_CHKBUSY_EN_A::AC12_WR_CHKBUSY_EN_1
    }
}
#[doc = "Write proxy for field `AC12_WR_CHKBUSY_EN`"]
pub struct AC12_WR_CHKBUSY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12_WR_CHKBUSY_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AC12_WR_CHKBUSY_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not check busy after auto CMD12 for write data packet"]
    #[inline(always)]
    pub fn ac12_wr_chkbusy_en_0(self) -> &'a mut W {
        self.variant(AC12_WR_CHKBUSY_EN_A::AC12_WR_CHKBUSY_EN_0)
    }
    #[doc = "Check busy after auto CMD12 for write data packet"]
    #[inline(always)]
    pub fn ac12_wr_chkbusy_en_1(self) -> &'a mut W {
        self.variant(AC12_WR_CHKBUSY_EN_A::AC12_WR_CHKBUSY_EN_1)
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
#[doc = "FRC_SDCLK_ON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRC_SDCLK_ON_A {
    #[doc = "0: CLK active or inactive is fully controlled by the hardware."]
    FRC_SDCLK_ON_0 = 0,
    #[doc = "1: Force CLK active."]
    FRC_SDCLK_ON_1 = 1,
}
impl From<FRC_SDCLK_ON_A> for bool {
    #[inline(always)]
    fn from(variant: FRC_SDCLK_ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRC_SDCLK_ON`"]
pub type FRC_SDCLK_ON_R = crate::R<bool, FRC_SDCLK_ON_A>;
impl FRC_SDCLK_ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRC_SDCLK_ON_A {
        match self.bits {
            false => FRC_SDCLK_ON_A::FRC_SDCLK_ON_0,
            true => FRC_SDCLK_ON_A::FRC_SDCLK_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRC_SDCLK_ON_0`"]
    #[inline(always)]
    pub fn is_frc_sdclk_on_0(&self) -> bool {
        *self == FRC_SDCLK_ON_A::FRC_SDCLK_ON_0
    }
    #[doc = "Checks if the value of the field is `FRC_SDCLK_ON_1`"]
    #[inline(always)]
    pub fn is_frc_sdclk_on_1(&self) -> bool {
        *self == FRC_SDCLK_ON_A::FRC_SDCLK_ON_1
    }
}
#[doc = "Write proxy for field `FRC_SDCLK_ON`"]
pub struct FRC_SDCLK_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> FRC_SDCLK_ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRC_SDCLK_ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    #[inline(always)]
    pub fn frc_sdclk_on_0(self) -> &'a mut W {
        self.variant(FRC_SDCLK_ON_A::FRC_SDCLK_ON_0)
    }
    #[doc = "Force CLK active."]
    #[inline(always)]
    pub fn frc_sdclk_on_1(self) -> &'a mut W {
        self.variant(FRC_SDCLK_ON_A::FRC_SDCLK_ON_1)
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
#[doc = "CRC Check Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_CHK_DIS_A {
    #[doc = "0: Check CRC16 for every read data packet and check CRC bits for every write data packet"]
    CRC_CHK_DIS_0 = 0,
    #[doc = "1: Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
    CRC_CHK_DIS_1 = 1,
}
impl From<CRC_CHK_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_CHK_DIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC_CHK_DIS`"]
pub type CRC_CHK_DIS_R = crate::R<bool, CRC_CHK_DIS_A>;
impl CRC_CHK_DIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_CHK_DIS_A {
        match self.bits {
            false => CRC_CHK_DIS_A::CRC_CHK_DIS_0,
            true => CRC_CHK_DIS_A::CRC_CHK_DIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_CHK_DIS_0`"]
    #[inline(always)]
    pub fn is_crc_chk_dis_0(&self) -> bool {
        *self == CRC_CHK_DIS_A::CRC_CHK_DIS_0
    }
    #[doc = "Checks if the value of the field is `CRC_CHK_DIS_1`"]
    #[inline(always)]
    pub fn is_crc_chk_dis_1(&self) -> bool {
        *self == CRC_CHK_DIS_A::CRC_CHK_DIS_1
    }
}
#[doc = "Write proxy for field `CRC_CHK_DIS`"]
pub struct CRC_CHK_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_CHK_DIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_CHK_DIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Check CRC16 for every read data packet and check CRC bits for every write data packet"]
    #[inline(always)]
    pub fn crc_chk_dis_0(self) -> &'a mut W {
        self.variant(CRC_CHK_DIS_A::CRC_CHK_DIS_0)
    }
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
    #[inline(always)]
    pub fn crc_chk_dis_1(self) -> &'a mut W {
        self.variant(CRC_CHK_DIS_A::CRC_CHK_DIS_1)
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
#[doc = "CMD_BYTE_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_BYTE_EN_A {
    #[doc = "0: Disable"]
    CMD_BYTE_EN_0 = 0,
    #[doc = "1: Enable"]
    CMD_BYTE_EN_1 = 1,
}
impl From<CMD_BYTE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_BYTE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMD_BYTE_EN`"]
pub type CMD_BYTE_EN_R = crate::R<bool, CMD_BYTE_EN_A>;
impl CMD_BYTE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_BYTE_EN_A {
        match self.bits {
            false => CMD_BYTE_EN_A::CMD_BYTE_EN_0,
            true => CMD_BYTE_EN_A::CMD_BYTE_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMD_BYTE_EN_0`"]
    #[inline(always)]
    pub fn is_cmd_byte_en_0(&self) -> bool {
        *self == CMD_BYTE_EN_A::CMD_BYTE_EN_0
    }
    #[doc = "Checks if the value of the field is `CMD_BYTE_EN_1`"]
    #[inline(always)]
    pub fn is_cmd_byte_en_1(&self) -> bool {
        *self == CMD_BYTE_EN_A::CMD_BYTE_EN_1
    }
}
#[doc = "Write proxy for field `CMD_BYTE_EN`"]
pub struct CMD_BYTE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_BYTE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_BYTE_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn cmd_byte_en_0(self) -> &'a mut W {
        self.variant(CMD_BYTE_EN_A::CMD_BYTE_EN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn cmd_byte_en_1(self) -> &'a mut W {
        self.variant(CMD_BYTE_EN_A::CMD_BYTE_EN_1)
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
    #[doc = "Bit 1 - Voltage Selection"]
    #[inline(always)]
    pub fn vselect(&self) -> VSELECT_R {
        VSELECT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Conflict check enable."]
    #[inline(always)]
    pub fn conflict_chk_en(&self) -> CONFLICT_CHK_EN_R {
        CONFLICT_CHK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AC12_WR_CHKBUSY_EN"]
    #[inline(always)]
    pub fn ac12_wr_chkbusy_en(&self) -> AC12_WR_CHKBUSY_EN_R {
        AC12_WR_CHKBUSY_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FRC_SDCLK_ON"]
    #[inline(always)]
    pub fn frc_sdclk_on(&self) -> FRC_SDCLK_ON_R {
        FRC_SDCLK_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CRC Check Disable"]
    #[inline(always)]
    pub fn crc_chk_dis(&self) -> CRC_CHK_DIS_R {
        CRC_CHK_DIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 31 - CMD_BYTE_EN"]
    #[inline(always)]
    pub fn cmd_byte_en(&self) -> CMD_BYTE_EN_R {
        CMD_BYTE_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Voltage Selection"]
    #[inline(always)]
    pub fn vselect(&mut self) -> VSELECT_W {
        VSELECT_W { w: self }
    }
    #[doc = "Bit 2 - Conflict check enable."]
    #[inline(always)]
    pub fn conflict_chk_en(&mut self) -> CONFLICT_CHK_EN_W {
        CONFLICT_CHK_EN_W { w: self }
    }
    #[doc = "Bit 3 - AC12_WR_CHKBUSY_EN"]
    #[inline(always)]
    pub fn ac12_wr_chkbusy_en(&mut self) -> AC12_WR_CHKBUSY_EN_W {
        AC12_WR_CHKBUSY_EN_W { w: self }
    }
    #[doc = "Bit 8 - FRC_SDCLK_ON"]
    #[inline(always)]
    pub fn frc_sdclk_on(&mut self) -> FRC_SDCLK_ON_W {
        FRC_SDCLK_ON_W { w: self }
    }
    #[doc = "Bit 15 - CRC Check Disable"]
    #[inline(always)]
    pub fn crc_chk_dis(&mut self) -> CRC_CHK_DIS_W {
        CRC_CHK_DIS_W { w: self }
    }
    #[doc = "Bit 31 - CMD_BYTE_EN"]
    #[inline(always)]
    pub fn cmd_byte_en(&mut self) -> CMD_BYTE_EN_W {
        CMD_BYTE_EN_W { w: self }
    }
}
