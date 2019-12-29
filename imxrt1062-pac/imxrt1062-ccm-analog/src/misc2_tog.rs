#[doc = "Reader of register MISC2_TOG"]
pub type R = crate::R<u32, super::MISC2_TOG>;
#[doc = "Writer for register MISC2_TOG"]
pub type W = crate::W<u32, super::MISC2_TOG>;
#[doc = "Register MISC2_TOG `reset()`'s with value 0x0027_2727"]
impl crate::ResetValue for super::MISC2_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0027_2727
    }
}
#[doc = "This field defines the brown out voltage offset for the CORE power domain\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG0_BO_OFFSET_A {
    #[doc = "4: Brownout offset = 0.100V"]
    REG0_BO_OFFSET_4,
    #[doc = "7: Brownout offset = 0.175V"]
    REG0_BO_OFFSET_7,
}
impl From<REG0_BO_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: REG0_BO_OFFSET_A) -> Self {
        match variant {
            REG0_BO_OFFSET_A::REG0_BO_OFFSET_4 => 4,
            REG0_BO_OFFSET_A::REG0_BO_OFFSET_7 => 7,
        }
    }
}
#[doc = "Reader of field `REG0_BO_OFFSET`"]
pub type REG0_BO_OFFSET_R = crate::R<u8, REG0_BO_OFFSET_A>;
impl REG0_BO_OFFSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REG0_BO_OFFSET_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(REG0_BO_OFFSET_A::REG0_BO_OFFSET_4),
            7 => Val(REG0_BO_OFFSET_A::REG0_BO_OFFSET_7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG0_BO_OFFSET_4`"]
    #[inline(always)]
    pub fn is_reg0_bo_offset_4(&self) -> bool {
        *self == REG0_BO_OFFSET_A::REG0_BO_OFFSET_4
    }
    #[doc = "Checks if the value of the field is `REG0_BO_OFFSET_7`"]
    #[inline(always)]
    pub fn is_reg0_bo_offset_7(&self) -> bool {
        *self == REG0_BO_OFFSET_A::REG0_BO_OFFSET_7
    }
}
#[doc = "Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG0_BO_STATUS_A {
    #[doc = "1: Brownout, supply is below target minus brownout offset."]
    REG0_BO_STATUS_1,
}
impl From<REG0_BO_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG0_BO_STATUS_A) -> Self {
        match variant {
            REG0_BO_STATUS_A::REG0_BO_STATUS_1 => true,
        }
    }
}
#[doc = "Reader of field `REG0_BO_STATUS`"]
pub type REG0_BO_STATUS_R = crate::R<bool, REG0_BO_STATUS_A>;
impl REG0_BO_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, REG0_BO_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(REG0_BO_STATUS_A::REG0_BO_STATUS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG0_BO_STATUS_1`"]
    #[inline(always)]
    pub fn is_reg0_bo_status_1(&self) -> bool {
        *self == REG0_BO_STATUS_A::REG0_BO_STATUS_1
    }
}
#[doc = "Reader of field `REG0_ENABLE_BO`"]
pub type REG0_ENABLE_BO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REG0_ENABLE_BO`"]
pub struct REG0_ENABLE_BO_W<'a> {
    w: &'a mut W,
}
impl<'a> REG0_ENABLE_BO_W<'a> {
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
#[doc = "Reader of field `REG0_OK`"]
pub type REG0_OK_R = crate::R<bool, bool>;
#[doc = "When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL3_DISABLE_A {
    #[doc = "0: PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
    PLL3_DISABLE_0,
    #[doc = "1: PLL3 can be disabled when the SoC is not in any low power mode"]
    PLL3_DISABLE_1,
}
impl From<PLL3_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PLL3_DISABLE_A) -> Self {
        match variant {
            PLL3_DISABLE_A::PLL3_DISABLE_0 => false,
            PLL3_DISABLE_A::PLL3_DISABLE_1 => true,
        }
    }
}
#[doc = "Reader of field `PLL3_DISABLE`"]
pub type PLL3_DISABLE_R = crate::R<bool, PLL3_DISABLE_A>;
impl PLL3_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL3_DISABLE_A {
        match self.bits {
            false => PLL3_DISABLE_A::PLL3_DISABLE_0,
            true => PLL3_DISABLE_A::PLL3_DISABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLL3_DISABLE_0`"]
    #[inline(always)]
    pub fn is_pll3_disable_0(&self) -> bool {
        *self == PLL3_DISABLE_A::PLL3_DISABLE_0
    }
    #[doc = "Checks if the value of the field is `PLL3_DISABLE_1`"]
    #[inline(always)]
    pub fn is_pll3_disable_1(&self) -> bool {
        *self == PLL3_DISABLE_A::PLL3_DISABLE_1
    }
}
#[doc = "Write proxy for field `PLL3_DISABLE`"]
pub struct PLL3_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL3_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL3 is being used by peripherals and is enabled when SoC is not in any low power mode"]
    #[inline(always)]
    pub fn pll3_disable_0(self) -> &'a mut W {
        self.variant(PLL3_DISABLE_A::PLL3_DISABLE_0)
    }
    #[doc = "PLL3 can be disabled when the SoC is not in any low power mode"]
    #[inline(always)]
    pub fn pll3_disable_1(self) -> &'a mut W {
        self.variant(PLL3_DISABLE_A::PLL3_DISABLE_1)
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
#[doc = "This field defines the brown out voltage offset for the xPU power domain\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG1_BO_OFFSET_A {
    #[doc = "4: Brownout offset = 0.100V"]
    REG1_BO_OFFSET_4,
    #[doc = "7: Brownout offset = 0.175V"]
    REG1_BO_OFFSET_7,
}
impl From<REG1_BO_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: REG1_BO_OFFSET_A) -> Self {
        match variant {
            REG1_BO_OFFSET_A::REG1_BO_OFFSET_4 => 4,
            REG1_BO_OFFSET_A::REG1_BO_OFFSET_7 => 7,
        }
    }
}
#[doc = "Reader of field `REG1_BO_OFFSET`"]
pub type REG1_BO_OFFSET_R = crate::R<u8, REG1_BO_OFFSET_A>;
impl REG1_BO_OFFSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REG1_BO_OFFSET_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(REG1_BO_OFFSET_A::REG1_BO_OFFSET_4),
            7 => Val(REG1_BO_OFFSET_A::REG1_BO_OFFSET_7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG1_BO_OFFSET_4`"]
    #[inline(always)]
    pub fn is_reg1_bo_offset_4(&self) -> bool {
        *self == REG1_BO_OFFSET_A::REG1_BO_OFFSET_4
    }
    #[doc = "Checks if the value of the field is `REG1_BO_OFFSET_7`"]
    #[inline(always)]
    pub fn is_reg1_bo_offset_7(&self) -> bool {
        *self == REG1_BO_OFFSET_A::REG1_BO_OFFSET_7
    }
}
#[doc = "Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG1_BO_STATUS_A {
    #[doc = "1: Brownout, supply is below target minus brownout offset."]
    REG1_BO_STATUS_1,
}
impl From<REG1_BO_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG1_BO_STATUS_A) -> Self {
        match variant {
            REG1_BO_STATUS_A::REG1_BO_STATUS_1 => true,
        }
    }
}
#[doc = "Reader of field `REG1_BO_STATUS`"]
pub type REG1_BO_STATUS_R = crate::R<bool, REG1_BO_STATUS_A>;
impl REG1_BO_STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, REG1_BO_STATUS_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(REG1_BO_STATUS_A::REG1_BO_STATUS_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG1_BO_STATUS_1`"]
    #[inline(always)]
    pub fn is_reg1_bo_status_1(&self) -> bool {
        *self == REG1_BO_STATUS_A::REG1_BO_STATUS_1
    }
}
#[doc = "Reader of field `REG1_ENABLE_BO`"]
pub type REG1_ENABLE_BO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REG1_ENABLE_BO`"]
pub struct REG1_ENABLE_BO_W<'a> {
    w: &'a mut W,
}
impl<'a> REG1_ENABLE_BO_W<'a> {
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
#[doc = "Reader of field `REG1_OK`"]
pub type REG1_OK_R = crate::R<bool, bool>;
#[doc = "LSB of Post-divider for Audio PLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIO_DIV_LSB_A {
    #[doc = "0: divide by 1 (Default)"]
    AUDIO_DIV_LSB_0,
    #[doc = "1: divide by 2"]
    AUDIO_DIV_LSB_1,
}
impl From<AUDIO_DIV_LSB_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIO_DIV_LSB_A) -> Self {
        match variant {
            AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_0 => false,
            AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_1 => true,
        }
    }
}
#[doc = "Reader of field `AUDIO_DIV_LSB`"]
pub type AUDIO_DIV_LSB_R = crate::R<bool, AUDIO_DIV_LSB_A>;
impl AUDIO_DIV_LSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIO_DIV_LSB_A {
        match self.bits {
            false => AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_0,
            true => AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIO_DIV_LSB_0`"]
    #[inline(always)]
    pub fn is_audio_div_lsb_0(&self) -> bool {
        *self == AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_0
    }
    #[doc = "Checks if the value of the field is `AUDIO_DIV_LSB_1`"]
    #[inline(always)]
    pub fn is_audio_div_lsb_1(&self) -> bool {
        *self == AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_1
    }
}
#[doc = "Write proxy for field `AUDIO_DIV_LSB`"]
pub struct AUDIO_DIV_LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_DIV_LSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_DIV_LSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "divide by 1 (Default)"]
    #[inline(always)]
    pub fn audio_div_lsb_0(self) -> &'a mut W {
        self.variant(AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn audio_div_lsb_1(self) -> &'a mut W {
        self.variant(AUDIO_DIV_LSB_A::AUDIO_DIV_LSB_1)
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
#[doc = "This field defines the brown out voltage offset for the xPU power domain\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG2_BO_OFFSET_A {
    #[doc = "4: Brownout offset = 0.100V"]
    REG2_BO_OFFSET_4,
    #[doc = "7: Brownout offset = 0.175V"]
    REG2_BO_OFFSET_7,
}
impl From<REG2_BO_OFFSET_A> for u8 {
    #[inline(always)]
    fn from(variant: REG2_BO_OFFSET_A) -> Self {
        match variant {
            REG2_BO_OFFSET_A::REG2_BO_OFFSET_4 => 4,
            REG2_BO_OFFSET_A::REG2_BO_OFFSET_7 => 7,
        }
    }
}
#[doc = "Reader of field `REG2_BO_OFFSET`"]
pub type REG2_BO_OFFSET_R = crate::R<u8, REG2_BO_OFFSET_A>;
impl REG2_BO_OFFSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REG2_BO_OFFSET_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(REG2_BO_OFFSET_A::REG2_BO_OFFSET_4),
            7 => Val(REG2_BO_OFFSET_A::REG2_BO_OFFSET_7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REG2_BO_OFFSET_4`"]
    #[inline(always)]
    pub fn is_reg2_bo_offset_4(&self) -> bool {
        *self == REG2_BO_OFFSET_A::REG2_BO_OFFSET_4
    }
    #[doc = "Checks if the value of the field is `REG2_BO_OFFSET_7`"]
    #[inline(always)]
    pub fn is_reg2_bo_offset_7(&self) -> bool {
        *self == REG2_BO_OFFSET_A::REG2_BO_OFFSET_7
    }
}
#[doc = "Reader of field `REG2_BO_STATUS`"]
pub type REG2_BO_STATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `REG2_ENABLE_BO`"]
pub type REG2_ENABLE_BO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REG2_ENABLE_BO`"]
pub struct REG2_ENABLE_BO_W<'a> {
    w: &'a mut W,
}
impl<'a> REG2_ENABLE_BO_W<'a> {
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
#[doc = "Reader of field `REG2_OK`"]
pub type REG2_OK_R = crate::R<bool, bool>;
#[doc = "MSB of Post-divider for Audio PLL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUDIO_DIV_MSB_A {
    #[doc = "0: divide by 1 (Default)"]
    AUDIO_DIV_MSB_0,
    #[doc = "1: divide by 2"]
    AUDIO_DIV_MSB_1,
}
impl From<AUDIO_DIV_MSB_A> for bool {
    #[inline(always)]
    fn from(variant: AUDIO_DIV_MSB_A) -> Self {
        match variant {
            AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_0 => false,
            AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_1 => true,
        }
    }
}
#[doc = "Reader of field `AUDIO_DIV_MSB`"]
pub type AUDIO_DIV_MSB_R = crate::R<bool, AUDIO_DIV_MSB_A>;
impl AUDIO_DIV_MSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUDIO_DIV_MSB_A {
        match self.bits {
            false => AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_0,
            true => AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUDIO_DIV_MSB_0`"]
    #[inline(always)]
    pub fn is_audio_div_msb_0(&self) -> bool {
        *self == AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_0
    }
    #[doc = "Checks if the value of the field is `AUDIO_DIV_MSB_1`"]
    #[inline(always)]
    pub fn is_audio_div_msb_1(&self) -> bool {
        *self == AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_1
    }
}
#[doc = "Write proxy for field `AUDIO_DIV_MSB`"]
pub struct AUDIO_DIV_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_DIV_MSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUDIO_DIV_MSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "divide by 1 (Default)"]
    #[inline(always)]
    pub fn audio_div_msb_0(self) -> &'a mut W {
        self.variant(AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn audio_div_msb_1(self) -> &'a mut W {
        self.variant(AUDIO_DIV_MSB_A::AUDIO_DIV_MSB_1)
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
#[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG0_STEP_TIME_A {
    #[doc = "0: 64"]
    _64_CLOCKS,
    #[doc = "1: 128"]
    _128_CLOCKS,
    #[doc = "2: 256"]
    _256_CLOCKS,
    #[doc = "3: 512"]
    _512_CLOCKS,
}
impl From<REG0_STEP_TIME_A> for u8 {
    #[inline(always)]
    fn from(variant: REG0_STEP_TIME_A) -> Self {
        match variant {
            REG0_STEP_TIME_A::_64_CLOCKS => 0,
            REG0_STEP_TIME_A::_128_CLOCKS => 1,
            REG0_STEP_TIME_A::_256_CLOCKS => 2,
            REG0_STEP_TIME_A::_512_CLOCKS => 3,
        }
    }
}
#[doc = "Reader of field `REG0_STEP_TIME`"]
pub type REG0_STEP_TIME_R = crate::R<u8, REG0_STEP_TIME_A>;
impl REG0_STEP_TIME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG0_STEP_TIME_A {
        match self.bits {
            0 => REG0_STEP_TIME_A::_64_CLOCKS,
            1 => REG0_STEP_TIME_A::_128_CLOCKS,
            2 => REG0_STEP_TIME_A::_256_CLOCKS,
            3 => REG0_STEP_TIME_A::_512_CLOCKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64_CLOCKS`"]
    #[inline(always)]
    pub fn is_64_clocks(&self) -> bool {
        *self == REG0_STEP_TIME_A::_64_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_128_CLOCKS`"]
    #[inline(always)]
    pub fn is_128_clocks(&self) -> bool {
        *self == REG0_STEP_TIME_A::_128_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_256_CLOCKS`"]
    #[inline(always)]
    pub fn is_256_clocks(&self) -> bool {
        *self == REG0_STEP_TIME_A::_256_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_512_CLOCKS`"]
    #[inline(always)]
    pub fn is_512_clocks(&self) -> bool {
        *self == REG0_STEP_TIME_A::_512_CLOCKS
    }
}
#[doc = "Write proxy for field `REG0_STEP_TIME`"]
pub struct REG0_STEP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> REG0_STEP_TIME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG0_STEP_TIME_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn _64_clocks(self) -> &'a mut W {
        self.variant(REG0_STEP_TIME_A::_64_CLOCKS)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn _128_clocks(self) -> &'a mut W {
        self.variant(REG0_STEP_TIME_A::_128_CLOCKS)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn _256_clocks(self) -> &'a mut W {
        self.variant(REG0_STEP_TIME_A::_256_CLOCKS)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn _512_clocks(self) -> &'a mut W {
        self.variant(REG0_STEP_TIME_A::_512_CLOCKS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG1_STEP_TIME_A {
    #[doc = "0: 64"]
    _64_CLOCKS,
    #[doc = "1: 128"]
    _128_CLOCKS,
    #[doc = "2: 256"]
    _256_CLOCKS,
    #[doc = "3: 512"]
    _512_CLOCKS,
}
impl From<REG1_STEP_TIME_A> for u8 {
    #[inline(always)]
    fn from(variant: REG1_STEP_TIME_A) -> Self {
        match variant {
            REG1_STEP_TIME_A::_64_CLOCKS => 0,
            REG1_STEP_TIME_A::_128_CLOCKS => 1,
            REG1_STEP_TIME_A::_256_CLOCKS => 2,
            REG1_STEP_TIME_A::_512_CLOCKS => 3,
        }
    }
}
#[doc = "Reader of field `REG1_STEP_TIME`"]
pub type REG1_STEP_TIME_R = crate::R<u8, REG1_STEP_TIME_A>;
impl REG1_STEP_TIME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG1_STEP_TIME_A {
        match self.bits {
            0 => REG1_STEP_TIME_A::_64_CLOCKS,
            1 => REG1_STEP_TIME_A::_128_CLOCKS,
            2 => REG1_STEP_TIME_A::_256_CLOCKS,
            3 => REG1_STEP_TIME_A::_512_CLOCKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64_CLOCKS`"]
    #[inline(always)]
    pub fn is_64_clocks(&self) -> bool {
        *self == REG1_STEP_TIME_A::_64_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_128_CLOCKS`"]
    #[inline(always)]
    pub fn is_128_clocks(&self) -> bool {
        *self == REG1_STEP_TIME_A::_128_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_256_CLOCKS`"]
    #[inline(always)]
    pub fn is_256_clocks(&self) -> bool {
        *self == REG1_STEP_TIME_A::_256_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_512_CLOCKS`"]
    #[inline(always)]
    pub fn is_512_clocks(&self) -> bool {
        *self == REG1_STEP_TIME_A::_512_CLOCKS
    }
}
#[doc = "Write proxy for field `REG1_STEP_TIME`"]
pub struct REG1_STEP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> REG1_STEP_TIME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG1_STEP_TIME_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn _64_clocks(self) -> &'a mut W {
        self.variant(REG1_STEP_TIME_A::_64_CLOCKS)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn _128_clocks(self) -> &'a mut W {
        self.variant(REG1_STEP_TIME_A::_128_CLOCKS)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn _256_clocks(self) -> &'a mut W {
        self.variant(REG1_STEP_TIME_A::_256_CLOCKS)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn _512_clocks(self) -> &'a mut W {
        self.variant(REG1_STEP_TIME_A::_512_CLOCKS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG2_STEP_TIME_A {
    #[doc = "0: 64"]
    _64_CLOCKS,
    #[doc = "1: 128"]
    _128_CLOCKS,
    #[doc = "2: 256"]
    _256_CLOCKS,
    #[doc = "3: 512"]
    _512_CLOCKS,
}
impl From<REG2_STEP_TIME_A> for u8 {
    #[inline(always)]
    fn from(variant: REG2_STEP_TIME_A) -> Self {
        match variant {
            REG2_STEP_TIME_A::_64_CLOCKS => 0,
            REG2_STEP_TIME_A::_128_CLOCKS => 1,
            REG2_STEP_TIME_A::_256_CLOCKS => 2,
            REG2_STEP_TIME_A::_512_CLOCKS => 3,
        }
    }
}
#[doc = "Reader of field `REG2_STEP_TIME`"]
pub type REG2_STEP_TIME_R = crate::R<u8, REG2_STEP_TIME_A>;
impl REG2_STEP_TIME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG2_STEP_TIME_A {
        match self.bits {
            0 => REG2_STEP_TIME_A::_64_CLOCKS,
            1 => REG2_STEP_TIME_A::_128_CLOCKS,
            2 => REG2_STEP_TIME_A::_256_CLOCKS,
            3 => REG2_STEP_TIME_A::_512_CLOCKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_64_CLOCKS`"]
    #[inline(always)]
    pub fn is_64_clocks(&self) -> bool {
        *self == REG2_STEP_TIME_A::_64_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_128_CLOCKS`"]
    #[inline(always)]
    pub fn is_128_clocks(&self) -> bool {
        *self == REG2_STEP_TIME_A::_128_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_256_CLOCKS`"]
    #[inline(always)]
    pub fn is_256_clocks(&self) -> bool {
        *self == REG2_STEP_TIME_A::_256_CLOCKS
    }
    #[doc = "Checks if the value of the field is `_512_CLOCKS`"]
    #[inline(always)]
    pub fn is_512_clocks(&self) -> bool {
        *self == REG2_STEP_TIME_A::_512_CLOCKS
    }
}
#[doc = "Write proxy for field `REG2_STEP_TIME`"]
pub struct REG2_STEP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> REG2_STEP_TIME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG2_STEP_TIME_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn _64_clocks(self) -> &'a mut W {
        self.variant(REG2_STEP_TIME_A::_64_CLOCKS)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn _128_clocks(self) -> &'a mut W {
        self.variant(REG2_STEP_TIME_A::_128_CLOCKS)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn _256_clocks(self) -> &'a mut W {
        self.variant(REG2_STEP_TIME_A::_256_CLOCKS)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn _512_clocks(self) -> &'a mut W {
        self.variant(REG2_STEP_TIME_A::_512_CLOCKS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Post-divider for video\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIDEO_DIV_A {
    #[doc = "0: divide by 1 (Default)"]
    VIDEO_DIV_0,
    #[doc = "1: divide by 2"]
    VIDEO_DIV_1,
    #[doc = "2: divide by 1"]
    VIDEO_DIV_2,
    #[doc = "3: divide by 4"]
    VIDEO_DIV_3,
}
impl From<VIDEO_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: VIDEO_DIV_A) -> Self {
        match variant {
            VIDEO_DIV_A::VIDEO_DIV_0 => 0,
            VIDEO_DIV_A::VIDEO_DIV_1 => 1,
            VIDEO_DIV_A::VIDEO_DIV_2 => 2,
            VIDEO_DIV_A::VIDEO_DIV_3 => 3,
        }
    }
}
#[doc = "Reader of field `VIDEO_DIV`"]
pub type VIDEO_DIV_R = crate::R<u8, VIDEO_DIV_A>;
impl VIDEO_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIDEO_DIV_A {
        match self.bits {
            0 => VIDEO_DIV_A::VIDEO_DIV_0,
            1 => VIDEO_DIV_A::VIDEO_DIV_1,
            2 => VIDEO_DIV_A::VIDEO_DIV_2,
            3 => VIDEO_DIV_A::VIDEO_DIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VIDEO_DIV_0`"]
    #[inline(always)]
    pub fn is_video_div_0(&self) -> bool {
        *self == VIDEO_DIV_A::VIDEO_DIV_0
    }
    #[doc = "Checks if the value of the field is `VIDEO_DIV_1`"]
    #[inline(always)]
    pub fn is_video_div_1(&self) -> bool {
        *self == VIDEO_DIV_A::VIDEO_DIV_1
    }
    #[doc = "Checks if the value of the field is `VIDEO_DIV_2`"]
    #[inline(always)]
    pub fn is_video_div_2(&self) -> bool {
        *self == VIDEO_DIV_A::VIDEO_DIV_2
    }
    #[doc = "Checks if the value of the field is `VIDEO_DIV_3`"]
    #[inline(always)]
    pub fn is_video_div_3(&self) -> bool {
        *self == VIDEO_DIV_A::VIDEO_DIV_3
    }
}
#[doc = "Write proxy for field `VIDEO_DIV`"]
pub struct VIDEO_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> VIDEO_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIDEO_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1 (Default)"]
    #[inline(always)]
    pub fn video_div_0(self) -> &'a mut W {
        self.variant(VIDEO_DIV_A::VIDEO_DIV_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn video_div_1(self) -> &'a mut W {
        self.variant(VIDEO_DIV_A::VIDEO_DIV_1)
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn video_div_2(self) -> &'a mut W {
        self.variant(VIDEO_DIV_A::VIDEO_DIV_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn video_div_3(self) -> &'a mut W {
        self.variant(VIDEO_DIV_A::VIDEO_DIV_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - This field defines the brown out voltage offset for the CORE power domain"]
    #[inline(always)]
    pub fn reg0_bo_offset(&self) -> REG0_BO_OFFSET_R {
        REG0_BO_OFFSET_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Reg0 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg0_bo_status(&self) -> REG0_BO_STATUS_R {
        REG0_BO_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg0_enable_bo(&self) -> REG0_ENABLE_BO_R {
        REG0_ENABLE_BO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ARM supply Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg0_ok(&self) -> REG0_OK_R {
        REG0_OK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    #[inline(always)]
    pub fn pll3_disable(&self) -> PLL3_DISABLE_R {
        PLL3_DISABLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub fn reg1_bo_offset(&self) -> REG1_BO_OFFSET_R {
        REG1_BO_OFFSET_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Reg1 brownout status bit. Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg1_bo_status(&self) -> REG1_BO_STATUS_R {
        REG1_BO_STATUS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg1_enable_bo(&self) -> REG1_ENABLE_BO_R {
        REG1_ENABLE_BO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPU/VPU supply Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg1_ok(&self) -> REG1_OK_R {
        REG1_OK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub fn audio_div_lsb(&self) -> AUDIO_DIV_LSB_R {
        AUDIO_DIV_LSB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - This field defines the brown out voltage offset for the xPU power domain"]
    #[inline(always)]
    pub fn reg2_bo_offset(&self) -> REG2_BO_OFFSET_R {
        REG2_BO_OFFSET_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - Reg2 brownout status bit.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg2_bo_status(&self) -> REG2_BO_STATUS_R {
        REG2_BO_STATUS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg2_enable_bo(&self) -> REG2_ENABLE_BO_R {
        REG2_ENABLE_BO_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Signals that the voltage is above the brownout level for the SOC supply"]
    #[inline(always)]
    pub fn reg2_ok(&self) -> REG2_OK_R {
        REG2_OK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub fn audio_div_msb(&self) -> AUDIO_DIV_MSB_R {
        AUDIO_DIV_MSB_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg0_step_time(&self) -> REG0_STEP_TIME_R {
        REG0_STEP_TIME_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg1_step_time(&self) -> REG1_STEP_TIME_R {
        REG1_STEP_TIME_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg2_step_time(&self) -> REG2_STEP_TIME_R {
        REG2_STEP_TIME_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Post-divider for video"]
    #[inline(always)]
    pub fn video_div(&self) -> VIDEO_DIV_R {
        VIDEO_DIV_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg0_enable_bo(&mut self) -> REG0_ENABLE_BO_W {
        REG0_ENABLE_BO_W { w: self }
    }
    #[doc = "Bit 7 - When USB is in low power suspend mode this Control bit is used to indicate if other system peripherals require the USB PLL3 clock when the SoC is not in low power mode"]
    #[inline(always)]
    pub fn pll3_disable(&mut self) -> PLL3_DISABLE_W {
        PLL3_DISABLE_W { w: self }
    }
    #[doc = "Bit 13 - Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg1_enable_bo(&mut self) -> REG1_ENABLE_BO_W {
        REG1_ENABLE_BO_W { w: self }
    }
    #[doc = "Bit 15 - LSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub fn audio_div_lsb(&mut self) -> AUDIO_DIV_LSB_W {
        AUDIO_DIV_LSB_W { w: self }
    }
    #[doc = "Bit 21 - Enables the brownout detection.Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg2_enable_bo(&mut self) -> REG2_ENABLE_BO_W {
        REG2_ENABLE_BO_W { w: self }
    }
    #[doc = "Bit 23 - MSB of Post-divider for Audio PLL"]
    #[inline(always)]
    pub fn audio_div_msb(&mut self) -> AUDIO_DIV_MSB_W {
        AUDIO_DIV_MSB_W { w: self }
    }
    #[doc = "Bits 24:25 - Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg0_step_time(&mut self) -> REG0_STEP_TIME_W {
        REG0_STEP_TIME_W { w: self }
    }
    #[doc = "Bits 26:27 - Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg1_step_time(&mut self) -> REG1_STEP_TIME_W {
        REG1_STEP_TIME_W { w: self }
    }
    #[doc = "Bits 28:29 - Number of clock periods (24MHz clock).Not related to CCM. See Power Management Unit (PMU)"]
    #[inline(always)]
    pub fn reg2_step_time(&mut self) -> REG2_STEP_TIME_W {
        REG2_STEP_TIME_W { w: self }
    }
    #[doc = "Bits 30:31 - Post-divider for video"]
    #[inline(always)]
    pub fn video_div(&mut self) -> VIDEO_DIV_W {
        VIDEO_DIV_W { w: self }
    }
}
