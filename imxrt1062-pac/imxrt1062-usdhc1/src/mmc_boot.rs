#[doc = "Reader of register MMC_BOOT"]
pub type R = crate::R<u32, super::MMC_BOOT>;
#[doc = "Writer for register MMC_BOOT"]
pub type W = crate::W<u32, super::MMC_BOOT>;
#[doc = "Register MMC_BOOT `reset()`'s with value 0"]
impl crate::ResetValue for super::MMC_BOOT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DTOCV_ACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTOCV_ACK_A {
    #[doc = "0: SDCLK x 2^14"]
    DTOCV_ACK_0 = 0,
    #[doc = "1: SDCLK x 2^15"]
    DTOCV_ACK_1 = 1,
    #[doc = "2: SDCLK x 2^16"]
    DTOCV_ACK_2 = 2,
    #[doc = "3: SDCLK x 2^17"]
    DTOCV_ACK_3 = 3,
    #[doc = "4: SDCLK x 2^18"]
    DTOCV_ACK_4 = 4,
    #[doc = "5: SDCLK x 2^19"]
    DTOCV_ACK_5 = 5,
    #[doc = "6: SDCLK x 2^20"]
    DTOCV_ACK_6 = 6,
    #[doc = "7: SDCLK x 2^21"]
    DTOCV_ACK_7 = 7,
    #[doc = "14: SDCLK x 2^28"]
    DTOCV_ACK_14 = 14,
    #[doc = "15: SDCLK x 2^29"]
    DTOCV_ACK_15 = 15,
}
impl From<DTOCV_ACK_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOCV_ACK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DTOCV_ACK`"]
pub type DTOCV_ACK_R = crate::R<u8, DTOCV_ACK_A>;
impl DTOCV_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTOCV_ACK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTOCV_ACK_A::DTOCV_ACK_0),
            1 => Val(DTOCV_ACK_A::DTOCV_ACK_1),
            2 => Val(DTOCV_ACK_A::DTOCV_ACK_2),
            3 => Val(DTOCV_ACK_A::DTOCV_ACK_3),
            4 => Val(DTOCV_ACK_A::DTOCV_ACK_4),
            5 => Val(DTOCV_ACK_A::DTOCV_ACK_5),
            6 => Val(DTOCV_ACK_A::DTOCV_ACK_6),
            7 => Val(DTOCV_ACK_A::DTOCV_ACK_7),
            14 => Val(DTOCV_ACK_A::DTOCV_ACK_14),
            15 => Val(DTOCV_ACK_A::DTOCV_ACK_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_0`"]
    #[inline(always)]
    pub fn is_dtocv_ack_0(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_0
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_1`"]
    #[inline(always)]
    pub fn is_dtocv_ack_1(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_1
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_2`"]
    #[inline(always)]
    pub fn is_dtocv_ack_2(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_2
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_3`"]
    #[inline(always)]
    pub fn is_dtocv_ack_3(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_3
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_4`"]
    #[inline(always)]
    pub fn is_dtocv_ack_4(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_4
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_5`"]
    #[inline(always)]
    pub fn is_dtocv_ack_5(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_5
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_6`"]
    #[inline(always)]
    pub fn is_dtocv_ack_6(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_6
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_7`"]
    #[inline(always)]
    pub fn is_dtocv_ack_7(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_7
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_14`"]
    #[inline(always)]
    pub fn is_dtocv_ack_14(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_14
    }
    #[doc = "Checks if the value of the field is `DTOCV_ACK_15`"]
    #[inline(always)]
    pub fn is_dtocv_ack_15(&self) -> bool {
        *self == DTOCV_ACK_A::DTOCV_ACK_15
    }
}
#[doc = "Write proxy for field `DTOCV_ACK`"]
pub struct DTOCV_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOCV_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTOCV_ACK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDCLK x 2^14"]
    #[inline(always)]
    pub fn dtocv_ack_0(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_0)
    }
    #[doc = "SDCLK x 2^15"]
    #[inline(always)]
    pub fn dtocv_ack_1(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_1)
    }
    #[doc = "SDCLK x 2^16"]
    #[inline(always)]
    pub fn dtocv_ack_2(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_2)
    }
    #[doc = "SDCLK x 2^17"]
    #[inline(always)]
    pub fn dtocv_ack_3(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_3)
    }
    #[doc = "SDCLK x 2^18"]
    #[inline(always)]
    pub fn dtocv_ack_4(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_4)
    }
    #[doc = "SDCLK x 2^19"]
    #[inline(always)]
    pub fn dtocv_ack_5(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_5)
    }
    #[doc = "SDCLK x 2^20"]
    #[inline(always)]
    pub fn dtocv_ack_6(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_6)
    }
    #[doc = "SDCLK x 2^21"]
    #[inline(always)]
    pub fn dtocv_ack_7(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_7)
    }
    #[doc = "SDCLK x 2^28"]
    #[inline(always)]
    pub fn dtocv_ack_14(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_14)
    }
    #[doc = "SDCLK x 2^29"]
    #[inline(always)]
    pub fn dtocv_ack_15(self) -> &'a mut W {
        self.variant(DTOCV_ACK_A::DTOCV_ACK_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "BOOT_ACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_ACK_A {
    #[doc = "0: No ack"]
    BOOT_ACK_0 = 0,
    #[doc = "1: Ack"]
    BOOT_ACK_1 = 1,
}
impl From<BOOT_ACK_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_ACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOOT_ACK`"]
pub type BOOT_ACK_R = crate::R<bool, BOOT_ACK_A>;
impl BOOT_ACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_ACK_A {
        match self.bits {
            false => BOOT_ACK_A::BOOT_ACK_0,
            true => BOOT_ACK_A::BOOT_ACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_ACK_0`"]
    #[inline(always)]
    pub fn is_boot_ack_0(&self) -> bool {
        *self == BOOT_ACK_A::BOOT_ACK_0
    }
    #[doc = "Checks if the value of the field is `BOOT_ACK_1`"]
    #[inline(always)]
    pub fn is_boot_ack_1(&self) -> bool {
        *self == BOOT_ACK_A::BOOT_ACK_1
    }
}
#[doc = "Write proxy for field `BOOT_ACK`"]
pub struct BOOT_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_ACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOT_ACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No ack"]
    #[inline(always)]
    pub fn boot_ack_0(self) -> &'a mut W {
        self.variant(BOOT_ACK_A::BOOT_ACK_0)
    }
    #[doc = "Ack"]
    #[inline(always)]
    pub fn boot_ack_1(self) -> &'a mut W {
        self.variant(BOOT_ACK_A::BOOT_ACK_1)
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
#[doc = "BOOT_MODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_MODE_A {
    #[doc = "0: Normal boot"]
    BOOT_MODE_0 = 0,
    #[doc = "1: Alternative boot"]
    BOOT_MODE_1 = 1,
}
impl From<BOOT_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOOT_MODE`"]
pub type BOOT_MODE_R = crate::R<bool, BOOT_MODE_A>;
impl BOOT_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_MODE_A {
        match self.bits {
            false => BOOT_MODE_A::BOOT_MODE_0,
            true => BOOT_MODE_A::BOOT_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_MODE_0`"]
    #[inline(always)]
    pub fn is_boot_mode_0(&self) -> bool {
        *self == BOOT_MODE_A::BOOT_MODE_0
    }
    #[doc = "Checks if the value of the field is `BOOT_MODE_1`"]
    #[inline(always)]
    pub fn is_boot_mode_1(&self) -> bool {
        *self == BOOT_MODE_A::BOOT_MODE_1
    }
}
#[doc = "Write proxy for field `BOOT_MODE`"]
pub struct BOOT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOT_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal boot"]
    #[inline(always)]
    pub fn boot_mode_0(self) -> &'a mut W {
        self.variant(BOOT_MODE_A::BOOT_MODE_0)
    }
    #[doc = "Alternative boot"]
    #[inline(always)]
    pub fn boot_mode_1(self) -> &'a mut W {
        self.variant(BOOT_MODE_A::BOOT_MODE_1)
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
#[doc = "BOOT_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_EN_A {
    #[doc = "0: Fast boot disable"]
    BOOT_EN_0 = 0,
    #[doc = "1: Fast boot enable"]
    BOOT_EN_1 = 1,
}
impl From<BOOT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOOT_EN`"]
pub type BOOT_EN_R = crate::R<bool, BOOT_EN_A>;
impl BOOT_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_EN_A {
        match self.bits {
            false => BOOT_EN_A::BOOT_EN_0,
            true => BOOT_EN_A::BOOT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOOT_EN_0`"]
    #[inline(always)]
    pub fn is_boot_en_0(&self) -> bool {
        *self == BOOT_EN_A::BOOT_EN_0
    }
    #[doc = "Checks if the value of the field is `BOOT_EN_1`"]
    #[inline(always)]
    pub fn is_boot_en_1(&self) -> bool {
        *self == BOOT_EN_A::BOOT_EN_1
    }
}
#[doc = "Write proxy for field `BOOT_EN`"]
pub struct BOOT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOT_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fast boot disable"]
    #[inline(always)]
    pub fn boot_en_0(self) -> &'a mut W {
        self.variant(BOOT_EN_A::BOOT_EN_0)
    }
    #[doc = "Fast boot enable"]
    #[inline(always)]
    pub fn boot_en_1(self) -> &'a mut W {
        self.variant(BOOT_EN_A::BOOT_EN_1)
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
#[doc = "Reader of field `AUTO_SABG_EN`"]
pub type AUTO_SABG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTO_SABG_EN`"]
pub struct AUTO_SABG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_SABG_EN_W<'a> {
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
#[doc = "Disable Time Out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_TIME_OUT_A {
    #[doc = "0: Enable time out"]
    DISABLE_TIME_OUT_0 = 0,
    #[doc = "1: Disable time out"]
    DISABLE_TIME_OUT_1 = 1,
}
impl From<DISABLE_TIME_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLE_TIME_OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISABLE_TIME_OUT`"]
pub type DISABLE_TIME_OUT_R = crate::R<bool, DISABLE_TIME_OUT_A>;
impl DISABLE_TIME_OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLE_TIME_OUT_A {
        match self.bits {
            false => DISABLE_TIME_OUT_A::DISABLE_TIME_OUT_0,
            true => DISABLE_TIME_OUT_A::DISABLE_TIME_OUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_TIME_OUT_0`"]
    #[inline(always)]
    pub fn is_disable_time_out_0(&self) -> bool {
        *self == DISABLE_TIME_OUT_A::DISABLE_TIME_OUT_0
    }
    #[doc = "Checks if the value of the field is `DISABLE_TIME_OUT_1`"]
    #[inline(always)]
    pub fn is_disable_time_out_1(&self) -> bool {
        *self == DISABLE_TIME_OUT_A::DISABLE_TIME_OUT_1
    }
}
#[doc = "Write proxy for field `DISABLE_TIME_OUT`"]
pub struct DISABLE_TIME_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_TIME_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLE_TIME_OUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable time out"]
    #[inline(always)]
    pub fn disable_time_out_0(self) -> &'a mut W {
        self.variant(DISABLE_TIME_OUT_A::DISABLE_TIME_OUT_0)
    }
    #[doc = "Disable time out"]
    #[inline(always)]
    pub fn disable_time_out_1(self) -> &'a mut W {
        self.variant(DISABLE_TIME_OUT_A::DISABLE_TIME_OUT_1)
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
#[doc = "Reader of field `BOOT_BLK_CNT`"]
pub type BOOT_BLK_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BOOT_BLK_CNT`"]
pub struct BOOT_BLK_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_BLK_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DTOCV_ACK"]
    #[inline(always)]
    pub fn dtocv_ack(&self) -> DTOCV_ACK_R {
        DTOCV_ACK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - BOOT_ACK"]
    #[inline(always)]
    pub fn boot_ack(&self) -> BOOT_ACK_R {
        BOOT_ACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BOOT_MODE"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BOOT_EN"]
    #[inline(always)]
    pub fn boot_en(&self) -> BOOT_EN_R {
        BOOT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AUTO_SABG_EN"]
    #[inline(always)]
    pub fn auto_sabg_en(&self) -> AUTO_SABG_EN_R {
        AUTO_SABG_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Disable Time Out"]
    #[inline(always)]
    pub fn disable_time_out(&self) -> DISABLE_TIME_OUT_R {
        DISABLE_TIME_OUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - BOOT_BLK_CNT"]
    #[inline(always)]
    pub fn boot_blk_cnt(&self) -> BOOT_BLK_CNT_R {
        BOOT_BLK_CNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DTOCV_ACK"]
    #[inline(always)]
    pub fn dtocv_ack(&mut self) -> DTOCV_ACK_W {
        DTOCV_ACK_W { w: self }
    }
    #[doc = "Bit 4 - BOOT_ACK"]
    #[inline(always)]
    pub fn boot_ack(&mut self) -> BOOT_ACK_W {
        BOOT_ACK_W { w: self }
    }
    #[doc = "Bit 5 - BOOT_MODE"]
    #[inline(always)]
    pub fn boot_mode(&mut self) -> BOOT_MODE_W {
        BOOT_MODE_W { w: self }
    }
    #[doc = "Bit 6 - BOOT_EN"]
    #[inline(always)]
    pub fn boot_en(&mut self) -> BOOT_EN_W {
        BOOT_EN_W { w: self }
    }
    #[doc = "Bit 7 - AUTO_SABG_EN"]
    #[inline(always)]
    pub fn auto_sabg_en(&mut self) -> AUTO_SABG_EN_W {
        AUTO_SABG_EN_W { w: self }
    }
    #[doc = "Bit 8 - Disable Time Out"]
    #[inline(always)]
    pub fn disable_time_out(&mut self) -> DISABLE_TIME_OUT_W {
        DISABLE_TIME_OUT_W { w: self }
    }
    #[doc = "Bits 16:31 - BOOT_BLK_CNT"]
    #[inline(always)]
    pub fn boot_blk_cnt(&mut self) -> BOOT_BLK_CNT_W {
        BOOT_BLK_CNT_W { w: self }
    }
}
