#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x7700"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7700
    }
}
#[doc = "BEE enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEE_ENABLE_A {
    #[doc = "0: Disable BEE"]
    BEE_ENABLE_0 = 0,
    #[doc = "1: Enable BEE"]
    BEE_ENABLE_1 = 1,
}
impl From<BEE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BEE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BEE_ENABLE`"]
pub type BEE_ENABLE_R = crate::R<bool, BEE_ENABLE_A>;
impl BEE_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEE_ENABLE_A {
        match self.bits {
            false => BEE_ENABLE_A::BEE_ENABLE_0,
            true => BEE_ENABLE_A::BEE_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BEE_ENABLE_0`"]
    #[inline(always)]
    pub fn is_bee_enable_0(&self) -> bool {
        *self == BEE_ENABLE_A::BEE_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `BEE_ENABLE_1`"]
    #[inline(always)]
    pub fn is_bee_enable_1(&self) -> bool {
        *self == BEE_ENABLE_A::BEE_ENABLE_1
    }
}
#[doc = "Write proxy for field `BEE_ENABLE`"]
pub struct BEE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BEE_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BEE_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable BEE"]
    #[inline(always)]
    pub fn bee_enable_0(self) -> &'a mut W {
        self.variant(BEE_ENABLE_A::BEE_ENABLE_0)
    }
    #[doc = "Enable BEE"]
    #[inline(always)]
    pub fn bee_enable_1(self) -> &'a mut W {
        self.variant(BEE_ENABLE_A::BEE_ENABLE_1)
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
#[doc = "Reader of field `CTRL_CLK_EN`"]
pub type CTRL_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTRL_CLK_EN`"]
pub struct CTRL_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_CLK_EN_W<'a> {
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
#[doc = "Reader of field `CTRL_SFTRST_N`"]
pub type CTRL_SFTRST_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTRL_SFTRST_N`"]
pub struct CTRL_SFTRST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_SFTRST_N_W<'a> {
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
#[doc = "Reader of field `KEY_VALID`"]
pub type KEY_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEY_VALID`"]
pub struct KEY_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_VALID_W<'a> {
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
#[doc = "AES key region select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_REGION_SEL_A {
    #[doc = "0: Load AES key for region0"]
    KEY_REGION_SEL_0 = 0,
    #[doc = "1: Load AES key for region1"]
    KEY_REGION_SEL_1 = 1,
}
impl From<KEY_REGION_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: KEY_REGION_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `KEY_REGION_SEL`"]
pub type KEY_REGION_SEL_R = crate::R<bool, KEY_REGION_SEL_A>;
impl KEY_REGION_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEY_REGION_SEL_A {
        match self.bits {
            false => KEY_REGION_SEL_A::KEY_REGION_SEL_0,
            true => KEY_REGION_SEL_A::KEY_REGION_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `KEY_REGION_SEL_0`"]
    #[inline(always)]
    pub fn is_key_region_sel_0(&self) -> bool {
        *self == KEY_REGION_SEL_A::KEY_REGION_SEL_0
    }
    #[doc = "Checks if the value of the field is `KEY_REGION_SEL_1`"]
    #[inline(always)]
    pub fn is_key_region_sel_1(&self) -> bool {
        *self == KEY_REGION_SEL_A::KEY_REGION_SEL_1
    }
}
#[doc = "Write proxy for field `KEY_REGION_SEL`"]
pub struct KEY_REGION_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_REGION_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_REGION_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Load AES key for region0"]
    #[inline(always)]
    pub fn key_region_sel_0(self) -> &'a mut W {
        self.variant(KEY_REGION_SEL_A::KEY_REGION_SEL_0)
    }
    #[doc = "Load AES key for region1"]
    #[inline(always)]
    pub fn key_region_sel_1(self) -> &'a mut W {
        self.variant(KEY_REGION_SEL_A::KEY_REGION_SEL_1)
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
#[doc = "Reader of field `AC_PROT_EN`"]
pub type AC_PROT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC_PROT_EN`"]
pub struct AC_PROT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AC_PROT_EN_W<'a> {
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
#[doc = "Endian swap control for the 16 bytes input and output data of AES core.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LITTLE_ENDIAN_A {
    #[doc = "0: The input and output data of the AES core is swapped as below: {B15,B14,B13,B12,B11,B10,B9,B8, B7,B6,B5,B4,B3,B2,B1,B0} swap to {B0,B1,B2,B3,B4,B5,B6,B7, B8,B9,B10,B11,B12,B13,B14,B15}, where B0~B15 refers to Byte0 to Byte15."]
    LITTLE_ENDIAN_0 = 0,
    #[doc = "1: The input and output data of AES core is not swapped."]
    LITTLE_ENDIAN_1 = 1,
}
impl From<LITTLE_ENDIAN_A> for bool {
    #[inline(always)]
    fn from(variant: LITTLE_ENDIAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LITTLE_ENDIAN`"]
pub type LITTLE_ENDIAN_R = crate::R<bool, LITTLE_ENDIAN_A>;
impl LITTLE_ENDIAN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LITTLE_ENDIAN_A {
        match self.bits {
            false => LITTLE_ENDIAN_A::LITTLE_ENDIAN_0,
            true => LITTLE_ENDIAN_A::LITTLE_ENDIAN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN_0`"]
    #[inline(always)]
    pub fn is_little_endian_0(&self) -> bool {
        *self == LITTLE_ENDIAN_A::LITTLE_ENDIAN_0
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN_1`"]
    #[inline(always)]
    pub fn is_little_endian_1(&self) -> bool {
        *self == LITTLE_ENDIAN_A::LITTLE_ENDIAN_1
    }
}
#[doc = "Write proxy for field `LITTLE_ENDIAN`"]
pub struct LITTLE_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> LITTLE_ENDIAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LITTLE_ENDIAN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The input and output data of the AES core is swapped as below: {B15,B14,B13,B12,B11,B10,B9,B8, B7,B6,B5,B4,B3,B2,B1,B0} swap to {B0,B1,B2,B3,B4,B5,B6,B7, B8,B9,B10,B11,B12,B13,B14,B15}, where B0~B15 refers to Byte0 to Byte15."]
    #[inline(always)]
    pub fn little_endian_0(self) -> &'a mut W {
        self.variant(LITTLE_ENDIAN_A::LITTLE_ENDIAN_0)
    }
    #[doc = "The input and output data of AES core is not swapped."]
    #[inline(always)]
    pub fn little_endian_1(self) -> &'a mut W {
        self.variant(LITTLE_ENDIAN_A::LITTLE_ENDIAN_1)
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
#[doc = "Reader of field `SECURITY_LEVEL_R0`"]
pub type SECURITY_LEVEL_R0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECURITY_LEVEL_R0`"]
pub struct SECURITY_LEVEL_R0_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURITY_LEVEL_R0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "AES mode of region0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_AES_MODE_R0_A {
    #[doc = "0: ECB"]
    CTRL_AES_MODE_R0_0 = 0,
    #[doc = "1: CTR"]
    CTRL_AES_MODE_R0_1 = 1,
}
impl From<CTRL_AES_MODE_R0_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_AES_MODE_R0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTRL_AES_MODE_R0`"]
pub type CTRL_AES_MODE_R0_R = crate::R<bool, CTRL_AES_MODE_R0_A>;
impl CTRL_AES_MODE_R0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRL_AES_MODE_R0_A {
        match self.bits {
            false => CTRL_AES_MODE_R0_A::CTRL_AES_MODE_R0_0,
            true => CTRL_AES_MODE_R0_A::CTRL_AES_MODE_R0_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_AES_MODE_R0_0`"]
    #[inline(always)]
    pub fn is_ctrl_aes_mode_r0_0(&self) -> bool {
        *self == CTRL_AES_MODE_R0_A::CTRL_AES_MODE_R0_0
    }
    #[doc = "Checks if the value of the field is `CTRL_AES_MODE_R0_1`"]
    #[inline(always)]
    pub fn is_ctrl_aes_mode_r0_1(&self) -> bool {
        *self == CTRL_AES_MODE_R0_A::CTRL_AES_MODE_R0_1
    }
}
#[doc = "Write proxy for field `CTRL_AES_MODE_R0`"]
pub struct CTRL_AES_MODE_R0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_AES_MODE_R0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_AES_MODE_R0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ECB"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r0_0(self) -> &'a mut W {
        self.variant(CTRL_AES_MODE_R0_A::CTRL_AES_MODE_R0_0)
    }
    #[doc = "CTR"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r0_1(self) -> &'a mut W {
        self.variant(CTRL_AES_MODE_R0_A::CTRL_AES_MODE_R0_1)
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
#[doc = "Reader of field `SECURITY_LEVEL_R1`"]
pub type SECURITY_LEVEL_R1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECURITY_LEVEL_R1`"]
pub struct SECURITY_LEVEL_R1_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURITY_LEVEL_R1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "AES mode of region1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_AES_MODE_R1_A {
    #[doc = "0: ECB"]
    CTRL_AES_MODE_R1_0 = 0,
    #[doc = "1: CTR"]
    CTRL_AES_MODE_R1_1 = 1,
}
impl From<CTRL_AES_MODE_R1_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_AES_MODE_R1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTRL_AES_MODE_R1`"]
pub type CTRL_AES_MODE_R1_R = crate::R<bool, CTRL_AES_MODE_R1_A>;
impl CTRL_AES_MODE_R1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRL_AES_MODE_R1_A {
        match self.bits {
            false => CTRL_AES_MODE_R1_A::CTRL_AES_MODE_R1_0,
            true => CTRL_AES_MODE_R1_A::CTRL_AES_MODE_R1_1,
        }
    }
    #[doc = "Checks if the value of the field is `CTRL_AES_MODE_R1_0`"]
    #[inline(always)]
    pub fn is_ctrl_aes_mode_r1_0(&self) -> bool {
        *self == CTRL_AES_MODE_R1_A::CTRL_AES_MODE_R1_0
    }
    #[doc = "Checks if the value of the field is `CTRL_AES_MODE_R1_1`"]
    #[inline(always)]
    pub fn is_ctrl_aes_mode_r1_1(&self) -> bool {
        *self == CTRL_AES_MODE_R1_A::CTRL_AES_MODE_R1_1
    }
}
#[doc = "Write proxy for field `CTRL_AES_MODE_R1`"]
pub struct CTRL_AES_MODE_R1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_AES_MODE_R1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_AES_MODE_R1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ECB"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r1_0(self) -> &'a mut W {
        self.variant(CTRL_AES_MODE_R1_A::CTRL_AES_MODE_R1_0)
    }
    #[doc = "CTR"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r1_1(self) -> &'a mut W {
        self.variant(CTRL_AES_MODE_R1_A::CTRL_AES_MODE_R1_1)
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
#[doc = "Reader of field `BEE_ENABLE_LOCK`"]
pub type BEE_ENABLE_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEE_ENABLE_LOCK`"]
pub struct BEE_ENABLE_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BEE_ENABLE_LOCK_W<'a> {
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
#[doc = "Reader of field `CTRL_CLK_EN_LOCK`"]
pub type CTRL_CLK_EN_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTRL_CLK_EN_LOCK`"]
pub struct CTRL_CLK_EN_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_CLK_EN_LOCK_W<'a> {
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
#[doc = "Reader of field `CTRL_SFTRST_N_LOCK`"]
pub type CTRL_SFTRST_N_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTRL_SFTRST_N_LOCK`"]
pub struct CTRL_SFTRST_N_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_SFTRST_N_LOCK_W<'a> {
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
#[doc = "Reader of field `REGION1_ADDR_LOCK`"]
pub type REGION1_ADDR_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGION1_ADDR_LOCK`"]
pub struct REGION1_ADDR_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION1_ADDR_LOCK_W<'a> {
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
#[doc = "Reader of field `KEY_VALID_LOCK`"]
pub type KEY_VALID_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEY_VALID_LOCK`"]
pub struct KEY_VALID_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_VALID_LOCK_W<'a> {
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
#[doc = "Reader of field `KEY_REGION_SEL_LOCK`"]
pub type KEY_REGION_SEL_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEY_REGION_SEL_LOCK`"]
pub struct KEY_REGION_SEL_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_REGION_SEL_LOCK_W<'a> {
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
#[doc = "Reader of field `AC_PROT_EN_LOCK`"]
pub type AC_PROT_EN_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC_PROT_EN_LOCK`"]
pub struct AC_PROT_EN_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> AC_PROT_EN_LOCK_W<'a> {
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
#[doc = "Reader of field `LITTLE_ENDIAN_LOCK`"]
pub type LITTLE_ENDIAN_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LITTLE_ENDIAN_LOCK`"]
pub struct LITTLE_ENDIAN_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LITTLE_ENDIAN_LOCK_W<'a> {
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
#[doc = "Reader of field `SECURITY_LEVEL_R0_LOCK`"]
pub type SECURITY_LEVEL_R0_LOCK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECURITY_LEVEL_R0_LOCK`"]
pub struct SECURITY_LEVEL_R0_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURITY_LEVEL_R0_LOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `CTRL_AES_MODE_R0_LOCK`"]
pub type CTRL_AES_MODE_R0_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTRL_AES_MODE_R0_LOCK`"]
pub struct CTRL_AES_MODE_R0_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_AES_MODE_R0_LOCK_W<'a> {
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
#[doc = "Reader of field `REGION0_KEY_LOCK`"]
pub type REGION0_KEY_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGION0_KEY_LOCK`"]
pub struct REGION0_KEY_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION0_KEY_LOCK_W<'a> {
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
#[doc = "Reader of field `SECURITY_LEVEL_R1_LOCK`"]
pub type SECURITY_LEVEL_R1_LOCK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECURITY_LEVEL_R1_LOCK`"]
pub struct SECURITY_LEVEL_R1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURITY_LEVEL_R1_LOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `CTRL_AES_MODE_R1_LOCK`"]
pub type CTRL_AES_MODE_R1_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTRL_AES_MODE_R1_LOCK`"]
pub struct CTRL_AES_MODE_R1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_AES_MODE_R1_LOCK_W<'a> {
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
#[doc = "Reader of field `REGION1_KEY_LOCK`"]
pub type REGION1_KEY_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REGION1_KEY_LOCK`"]
pub struct REGION1_KEY_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION1_KEY_LOCK_W<'a> {
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
    #[doc = "Bit 0 - BEE enable bit"]
    #[inline(always)]
    pub fn bee_enable(&self) -> BEE_ENABLE_R {
        BEE_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock enable input, low inactive"]
    #[inline(always)]
    pub fn ctrl_clk_en(&self) -> CTRL_CLK_EN_R {
        CTRL_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Soft reset input, low active"]
    #[inline(always)]
    pub fn ctrl_sftrst_n(&self) -> CTRL_SFTRST_N_R {
        CTRL_SFTRST_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AES-128 key is ready"]
    #[inline(always)]
    pub fn key_valid(&self) -> KEY_VALID_R {
        KEY_VALID_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AES key region select"]
    #[inline(always)]
    pub fn key_region_sel(&self) -> KEY_REGION_SEL_R {
        KEY_REGION_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable access permission control When AC_PROT_EN is asserted, all encrypted regions are limited to be ARM core access only"]
    #[inline(always)]
    pub fn ac_prot_en(&self) -> AC_PROT_EN_R {
        AC_PROT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endian swap control for the 16 bytes input and output data of AES core."]
    #[inline(always)]
    pub fn little_endian(&self) -> LITTLE_ENDIAN_R {
        LITTLE_ENDIAN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Security level of the allowed access for memory region0"]
    #[inline(always)]
    pub fn security_level_r0(&self) -> SECURITY_LEVEL_R0_R {
        SECURITY_LEVEL_R0_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - AES mode of region0"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r0(&self) -> CTRL_AES_MODE_R0_R {
        CTRL_AES_MODE_R0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Security level of the allowed access for memory region1"]
    #[inline(always)]
    pub fn security_level_r1(&self) -> SECURITY_LEVEL_R1_R {
        SECURITY_LEVEL_R1_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - AES mode of region1"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r1(&self) -> CTRL_AES_MODE_R1_R {
        CTRL_AES_MODE_R1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Lock bit for bee_enable"]
    #[inline(always)]
    pub fn bee_enable_lock(&self) -> BEE_ENABLE_LOCK_R {
        BEE_ENABLE_LOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Lock bit for ctrl_clk_en"]
    #[inline(always)]
    pub fn ctrl_clk_en_lock(&self) -> CTRL_CLK_EN_LOCK_R {
        CTRL_CLK_EN_LOCK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Lock bit for ctrl_sftrst"]
    #[inline(always)]
    pub fn ctrl_sftrst_n_lock(&self) -> CTRL_SFTRST_N_LOCK_R {
        CTRL_SFTRST_N_LOCK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Lock bit for region1 address boundary"]
    #[inline(always)]
    pub fn region1_addr_lock(&self) -> REGION1_ADDR_LOCK_R {
        REGION1_ADDR_LOCK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Lock bit for key_valid"]
    #[inline(always)]
    pub fn key_valid_lock(&self) -> KEY_VALID_LOCK_R {
        KEY_VALID_LOCK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Lock bit for key_region_sel"]
    #[inline(always)]
    pub fn key_region_sel_lock(&self) -> KEY_REGION_SEL_LOCK_R {
        KEY_REGION_SEL_LOCK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Lock bit for ac_prot"]
    #[inline(always)]
    pub fn ac_prot_en_lock(&self) -> AC_PROT_EN_LOCK_R {
        AC_PROT_EN_LOCK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock bit for little_endian"]
    #[inline(always)]
    pub fn little_endian_lock(&self) -> LITTLE_ENDIAN_LOCK_R {
        LITTLE_ENDIAN_LOCK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Lock bits for security_level_r0"]
    #[inline(always)]
    pub fn security_level_r0_lock(&self) -> SECURITY_LEVEL_R0_LOCK_R {
        SECURITY_LEVEL_R0_LOCK_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Lock bit for region0 ctrl_aes_mode"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r0_lock(&self) -> CTRL_AES_MODE_R0_LOCK_R {
        CTRL_AES_MODE_R0_LOCK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Lock bit for region0 AES key"]
    #[inline(always)]
    pub fn region0_key_lock(&self) -> REGION0_KEY_LOCK_R {
        REGION0_KEY_LOCK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Lock bits for security_level_r1"]
    #[inline(always)]
    pub fn security_level_r1_lock(&self) -> SECURITY_LEVEL_R1_LOCK_R {
        SECURITY_LEVEL_R1_LOCK_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Lock bit for region1 ctrl_aes_mode"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r1_lock(&self) -> CTRL_AES_MODE_R1_LOCK_R {
        CTRL_AES_MODE_R1_LOCK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock bit for region1 AES key"]
    #[inline(always)]
    pub fn region1_key_lock(&self) -> REGION1_KEY_LOCK_R {
        REGION1_KEY_LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BEE enable bit"]
    #[inline(always)]
    pub fn bee_enable(&mut self) -> BEE_ENABLE_W {
        BEE_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Clock enable input, low inactive"]
    #[inline(always)]
    pub fn ctrl_clk_en(&mut self) -> CTRL_CLK_EN_W {
        CTRL_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2 - Soft reset input, low active"]
    #[inline(always)]
    pub fn ctrl_sftrst_n(&mut self) -> CTRL_SFTRST_N_W {
        CTRL_SFTRST_N_W { w: self }
    }
    #[doc = "Bit 4 - AES-128 key is ready"]
    #[inline(always)]
    pub fn key_valid(&mut self) -> KEY_VALID_W {
        KEY_VALID_W { w: self }
    }
    #[doc = "Bit 5 - AES key region select"]
    #[inline(always)]
    pub fn key_region_sel(&mut self) -> KEY_REGION_SEL_W {
        KEY_REGION_SEL_W { w: self }
    }
    #[doc = "Bit 6 - Enable access permission control When AC_PROT_EN is asserted, all encrypted regions are limited to be ARM core access only"]
    #[inline(always)]
    pub fn ac_prot_en(&mut self) -> AC_PROT_EN_W {
        AC_PROT_EN_W { w: self }
    }
    #[doc = "Bit 7 - Endian swap control for the 16 bytes input and output data of AES core."]
    #[inline(always)]
    pub fn little_endian(&mut self) -> LITTLE_ENDIAN_W {
        LITTLE_ENDIAN_W { w: self }
    }
    #[doc = "Bits 8:9 - Security level of the allowed access for memory region0"]
    #[inline(always)]
    pub fn security_level_r0(&mut self) -> SECURITY_LEVEL_R0_W {
        SECURITY_LEVEL_R0_W { w: self }
    }
    #[doc = "Bit 10 - AES mode of region0"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r0(&mut self) -> CTRL_AES_MODE_R0_W {
        CTRL_AES_MODE_R0_W { w: self }
    }
    #[doc = "Bits 12:13 - Security level of the allowed access for memory region1"]
    #[inline(always)]
    pub fn security_level_r1(&mut self) -> SECURITY_LEVEL_R1_W {
        SECURITY_LEVEL_R1_W { w: self }
    }
    #[doc = "Bit 14 - AES mode of region1"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r1(&mut self) -> CTRL_AES_MODE_R1_W {
        CTRL_AES_MODE_R1_W { w: self }
    }
    #[doc = "Bit 16 - Lock bit for bee_enable"]
    #[inline(always)]
    pub fn bee_enable_lock(&mut self) -> BEE_ENABLE_LOCK_W {
        BEE_ENABLE_LOCK_W { w: self }
    }
    #[doc = "Bit 17 - Lock bit for ctrl_clk_en"]
    #[inline(always)]
    pub fn ctrl_clk_en_lock(&mut self) -> CTRL_CLK_EN_LOCK_W {
        CTRL_CLK_EN_LOCK_W { w: self }
    }
    #[doc = "Bit 18 - Lock bit for ctrl_sftrst"]
    #[inline(always)]
    pub fn ctrl_sftrst_n_lock(&mut self) -> CTRL_SFTRST_N_LOCK_W {
        CTRL_SFTRST_N_LOCK_W { w: self }
    }
    #[doc = "Bit 19 - Lock bit for region1 address boundary"]
    #[inline(always)]
    pub fn region1_addr_lock(&mut self) -> REGION1_ADDR_LOCK_W {
        REGION1_ADDR_LOCK_W { w: self }
    }
    #[doc = "Bit 20 - Lock bit for key_valid"]
    #[inline(always)]
    pub fn key_valid_lock(&mut self) -> KEY_VALID_LOCK_W {
        KEY_VALID_LOCK_W { w: self }
    }
    #[doc = "Bit 21 - Lock bit for key_region_sel"]
    #[inline(always)]
    pub fn key_region_sel_lock(&mut self) -> KEY_REGION_SEL_LOCK_W {
        KEY_REGION_SEL_LOCK_W { w: self }
    }
    #[doc = "Bit 22 - Lock bit for ac_prot"]
    #[inline(always)]
    pub fn ac_prot_en_lock(&mut self) -> AC_PROT_EN_LOCK_W {
        AC_PROT_EN_LOCK_W { w: self }
    }
    #[doc = "Bit 23 - Lock bit for little_endian"]
    #[inline(always)]
    pub fn little_endian_lock(&mut self) -> LITTLE_ENDIAN_LOCK_W {
        LITTLE_ENDIAN_LOCK_W { w: self }
    }
    #[doc = "Bits 24:25 - Lock bits for security_level_r0"]
    #[inline(always)]
    pub fn security_level_r0_lock(&mut self) -> SECURITY_LEVEL_R0_LOCK_W {
        SECURITY_LEVEL_R0_LOCK_W { w: self }
    }
    #[doc = "Bit 26 - Lock bit for region0 ctrl_aes_mode"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r0_lock(&mut self) -> CTRL_AES_MODE_R0_LOCK_W {
        CTRL_AES_MODE_R0_LOCK_W { w: self }
    }
    #[doc = "Bit 27 - Lock bit for region0 AES key"]
    #[inline(always)]
    pub fn region0_key_lock(&mut self) -> REGION0_KEY_LOCK_W {
        REGION0_KEY_LOCK_W { w: self }
    }
    #[doc = "Bits 28:29 - Lock bits for security_level_r1"]
    #[inline(always)]
    pub fn security_level_r1_lock(&mut self) -> SECURITY_LEVEL_R1_LOCK_W {
        SECURITY_LEVEL_R1_LOCK_W { w: self }
    }
    #[doc = "Bit 30 - Lock bit for region1 ctrl_aes_mode"]
    #[inline(always)]
    pub fn ctrl_aes_mode_r1_lock(&mut self) -> CTRL_AES_MODE_R1_LOCK_W {
        CTRL_AES_MODE_R1_LOCK_W { w: self }
    }
    #[doc = "Bit 31 - Lock bit for region1 AES key"]
    #[inline(always)]
    pub fn region1_key_lock(&mut self) -> REGION1_KEY_LOCK_W {
        REGION1_KEY_LOCK_W { w: self }
    }
}
