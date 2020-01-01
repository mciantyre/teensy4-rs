#[doc = "Reader of register HPCR"]
pub type R = crate::R<u32, super::HPCR>;
#[doc = "Writer for register HPCR"]
pub type W = crate::W<u32, super::HPCR>;
#[doc = "Register HPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::HPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "HP Real Time Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_EN_A {
    #[doc = "0: RTC is disabled"]
    RTC_EN_0 = 0,
    #[doc = "1: RTC is enabled"]
    RTC_EN_1 = 1,
}
impl From<RTC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC_EN`"]
pub type RTC_EN_R = crate::R<bool, RTC_EN_A>;
impl RTC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_EN_A {
        match self.bits {
            false => RTC_EN_A::RTC_EN_0,
            true => RTC_EN_A::RTC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTC_EN_0`"]
    #[inline(always)]
    pub fn is_rtc_en_0(&self) -> bool {
        *self == RTC_EN_A::RTC_EN_0
    }
    #[doc = "Checks if the value of the field is `RTC_EN_1`"]
    #[inline(always)]
    pub fn is_rtc_en_1(&self) -> bool {
        *self == RTC_EN_A::RTC_EN_1
    }
}
#[doc = "Write proxy for field `RTC_EN`"]
pub struct RTC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC is disabled"]
    #[inline(always)]
    pub fn rtc_en_0(self) -> &'a mut W {
        self.variant(RTC_EN_A::RTC_EN_0)
    }
    #[doc = "RTC is enabled"]
    #[inline(always)]
    pub fn rtc_en_1(self) -> &'a mut W {
        self.variant(RTC_EN_A::RTC_EN_1)
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
#[doc = "HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPTA_EN_A {
    #[doc = "0: HP Time Alarm Interrupt is disabled"]
    HPTA_EN_0 = 0,
    #[doc = "1: HP Time Alarm Interrupt is enabled"]
    HPTA_EN_1 = 1,
}
impl From<HPTA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HPTA_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HPTA_EN`"]
pub type HPTA_EN_R = crate::R<bool, HPTA_EN_A>;
impl HPTA_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPTA_EN_A {
        match self.bits {
            false => HPTA_EN_A::HPTA_EN_0,
            true => HPTA_EN_A::HPTA_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPTA_EN_0`"]
    #[inline(always)]
    pub fn is_hpta_en_0(&self) -> bool {
        *self == HPTA_EN_A::HPTA_EN_0
    }
    #[doc = "Checks if the value of the field is `HPTA_EN_1`"]
    #[inline(always)]
    pub fn is_hpta_en_1(&self) -> bool {
        *self == HPTA_EN_A::HPTA_EN_1
    }
}
#[doc = "Write proxy for field `HPTA_EN`"]
pub struct HPTA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HPTA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPTA_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HP Time Alarm Interrupt is disabled"]
    #[inline(always)]
    pub fn hpta_en_0(self) -> &'a mut W {
        self.variant(HPTA_EN_A::HPTA_EN_0)
    }
    #[doc = "HP Time Alarm Interrupt is enabled"]
    #[inline(always)]
    pub fn hpta_en_1(self) -> &'a mut W {
        self.variant(HPTA_EN_A::HPTA_EN_1)
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
#[doc = "Disable periodic interrupt in the functional interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_PI_A {
    #[doc = "0: Periodic interrupt will trigger a functional interrupt"]
    DIS_PI_0 = 0,
    #[doc = "1: Disable periodic interrupt in the function interrupt"]
    DIS_PI_1 = 1,
}
impl From<DIS_PI_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_PI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIS_PI`"]
pub type DIS_PI_R = crate::R<bool, DIS_PI_A>;
impl DIS_PI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_PI_A {
        match self.bits {
            false => DIS_PI_A::DIS_PI_0,
            true => DIS_PI_A::DIS_PI_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_PI_0`"]
    #[inline(always)]
    pub fn is_dis_pi_0(&self) -> bool {
        *self == DIS_PI_A::DIS_PI_0
    }
    #[doc = "Checks if the value of the field is `DIS_PI_1`"]
    #[inline(always)]
    pub fn is_dis_pi_1(&self) -> bool {
        *self == DIS_PI_A::DIS_PI_1
    }
}
#[doc = "Write proxy for field `DIS_PI`"]
pub struct DIS_PI_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_PI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIS_PI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Periodic interrupt will trigger a functional interrupt"]
    #[inline(always)]
    pub fn dis_pi_0(self) -> &'a mut W {
        self.variant(DIS_PI_A::DIS_PI_0)
    }
    #[doc = "Disable periodic interrupt in the function interrupt"]
    #[inline(always)]
    pub fn dis_pi_1(self) -> &'a mut W {
        self.variant(DIS_PI_A::DIS_PI_1)
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
#[doc = "HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PI_EN_A {
    #[doc = "0: HP Periodic Interrupt is disabled"]
    PI_EN_0 = 0,
    #[doc = "1: HP Periodic Interrupt is enabled"]
    PI_EN_1 = 1,
}
impl From<PI_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PI_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PI_EN`"]
pub type PI_EN_R = crate::R<bool, PI_EN_A>;
impl PI_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PI_EN_A {
        match self.bits {
            false => PI_EN_A::PI_EN_0,
            true => PI_EN_A::PI_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PI_EN_0`"]
    #[inline(always)]
    pub fn is_pi_en_0(&self) -> bool {
        *self == PI_EN_A::PI_EN_0
    }
    #[doc = "Checks if the value of the field is `PI_EN_1`"]
    #[inline(always)]
    pub fn is_pi_en_1(&self) -> bool {
        *self == PI_EN_A::PI_EN_1
    }
}
#[doc = "Write proxy for field `PI_EN`"]
pub struct PI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PI_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PI_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HP Periodic Interrupt is disabled"]
    #[inline(always)]
    pub fn pi_en_0(self) -> &'a mut W {
        self.variant(PI_EN_A::PI_EN_0)
    }
    #[doc = "HP Periodic Interrupt is enabled"]
    #[inline(always)]
    pub fn pi_en_1(self) -> &'a mut W {
        self.variant(PI_EN_A::PI_EN_1)
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
#[doc = "Periodic Interrupt Frequency Defines frequency of the periodic interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PI_FREQ_A {
    #[doc = "0: - bit 0 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_0 = 0,
    #[doc = "1: - bit 1 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_1 = 1,
    #[doc = "2: - bit 2 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_2 = 2,
    #[doc = "3: - bit 3 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_3 = 3,
    #[doc = "4: - bit 4 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_4 = 4,
    #[doc = "5: - bit 5 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_5 = 5,
    #[doc = "6: - bit 6 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_6 = 6,
    #[doc = "7: - bit 7 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_7 = 7,
    #[doc = "8: - bit 8 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_8 = 8,
    #[doc = "9: - bit 9 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_9 = 9,
    #[doc = "10: - bit 10 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_10 = 10,
    #[doc = "11: - bit 11 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_11 = 11,
    #[doc = "12: - bit 12 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_12 = 12,
    #[doc = "13: - bit 13 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_13 = 13,
    #[doc = "14: - bit 14 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_14 = 14,
    #[doc = "15: - bit 15 of the HPRTCLR is selected as a source of the periodic interrupt"]
    PI_FREQ_15 = 15,
}
impl From<PI_FREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: PI_FREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PI_FREQ`"]
pub type PI_FREQ_R = crate::R<u8, PI_FREQ_A>;
impl PI_FREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PI_FREQ_A {
        match self.bits {
            0 => PI_FREQ_A::PI_FREQ_0,
            1 => PI_FREQ_A::PI_FREQ_1,
            2 => PI_FREQ_A::PI_FREQ_2,
            3 => PI_FREQ_A::PI_FREQ_3,
            4 => PI_FREQ_A::PI_FREQ_4,
            5 => PI_FREQ_A::PI_FREQ_5,
            6 => PI_FREQ_A::PI_FREQ_6,
            7 => PI_FREQ_A::PI_FREQ_7,
            8 => PI_FREQ_A::PI_FREQ_8,
            9 => PI_FREQ_A::PI_FREQ_9,
            10 => PI_FREQ_A::PI_FREQ_10,
            11 => PI_FREQ_A::PI_FREQ_11,
            12 => PI_FREQ_A::PI_FREQ_12,
            13 => PI_FREQ_A::PI_FREQ_13,
            14 => PI_FREQ_A::PI_FREQ_14,
            15 => PI_FREQ_A::PI_FREQ_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_0`"]
    #[inline(always)]
    pub fn is_pi_freq_0(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_0
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_1`"]
    #[inline(always)]
    pub fn is_pi_freq_1(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_1
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_2`"]
    #[inline(always)]
    pub fn is_pi_freq_2(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_2
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_3`"]
    #[inline(always)]
    pub fn is_pi_freq_3(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_3
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_4`"]
    #[inline(always)]
    pub fn is_pi_freq_4(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_4
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_5`"]
    #[inline(always)]
    pub fn is_pi_freq_5(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_5
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_6`"]
    #[inline(always)]
    pub fn is_pi_freq_6(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_6
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_7`"]
    #[inline(always)]
    pub fn is_pi_freq_7(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_7
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_8`"]
    #[inline(always)]
    pub fn is_pi_freq_8(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_8
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_9`"]
    #[inline(always)]
    pub fn is_pi_freq_9(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_9
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_10`"]
    #[inline(always)]
    pub fn is_pi_freq_10(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_10
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_11`"]
    #[inline(always)]
    pub fn is_pi_freq_11(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_11
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_12`"]
    #[inline(always)]
    pub fn is_pi_freq_12(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_12
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_13`"]
    #[inline(always)]
    pub fn is_pi_freq_13(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_13
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_14`"]
    #[inline(always)]
    pub fn is_pi_freq_14(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_14
    }
    #[doc = "Checks if the value of the field is `PI_FREQ_15`"]
    #[inline(always)]
    pub fn is_pi_freq_15(&self) -> bool {
        *self == PI_FREQ_A::PI_FREQ_15
    }
}
#[doc = "Write proxy for field `PI_FREQ`"]
pub struct PI_FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PI_FREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PI_FREQ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "- bit 0 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_0(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_0)
    }
    #[doc = "- bit 1 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_1(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_1)
    }
    #[doc = "- bit 2 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_2(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_2)
    }
    #[doc = "- bit 3 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_3(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_3)
    }
    #[doc = "- bit 4 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_4(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_4)
    }
    #[doc = "- bit 5 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_5(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_5)
    }
    #[doc = "- bit 6 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_6(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_6)
    }
    #[doc = "- bit 7 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_7(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_7)
    }
    #[doc = "- bit 8 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_8(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_8)
    }
    #[doc = "- bit 9 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_9(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_9)
    }
    #[doc = "- bit 10 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_10(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_10)
    }
    #[doc = "- bit 11 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_11(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_11)
    }
    #[doc = "- bit 12 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_12(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_12)
    }
    #[doc = "- bit 13 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_13(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_13)
    }
    #[doc = "- bit 14 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_14(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_14)
    }
    #[doc = "- bit 15 of the HPRTCLR is selected as a source of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq_15(self) -> &'a mut W {
        self.variant(PI_FREQ_A::PI_FREQ_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPCALB_EN_A {
    #[doc = "0: HP Timer calibration disabled"]
    HPCALB_EN_0 = 0,
    #[doc = "1: HP Timer calibration enabled"]
    HPCALB_EN_1 = 1,
}
impl From<HPCALB_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HPCALB_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HPCALB_EN`"]
pub type HPCALB_EN_R = crate::R<bool, HPCALB_EN_A>;
impl HPCALB_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPCALB_EN_A {
        match self.bits {
            false => HPCALB_EN_A::HPCALB_EN_0,
            true => HPCALB_EN_A::HPCALB_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPCALB_EN_0`"]
    #[inline(always)]
    pub fn is_hpcalb_en_0(&self) -> bool {
        *self == HPCALB_EN_A::HPCALB_EN_0
    }
    #[doc = "Checks if the value of the field is `HPCALB_EN_1`"]
    #[inline(always)]
    pub fn is_hpcalb_en_1(&self) -> bool {
        *self == HPCALB_EN_A::HPCALB_EN_1
    }
}
#[doc = "Write proxy for field `HPCALB_EN`"]
pub struct HPCALB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HPCALB_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPCALB_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HP Timer calibration disabled"]
    #[inline(always)]
    pub fn hpcalb_en_0(self) -> &'a mut W {
        self.variant(HPCALB_EN_A::HPCALB_EN_0)
    }
    #[doc = "HP Timer calibration enabled"]
    #[inline(always)]
    pub fn hpcalb_en_1(self) -> &'a mut W {
        self.variant(HPCALB_EN_A::HPCALB_EN_1)
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
#[doc = "HP Calibration Value Defines signed calibration value for the HP Real Time Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HPCALB_VAL_A {
    #[doc = "0: +0 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_0 = 0,
    #[doc = "1: +1 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_1 = 1,
    #[doc = "2: +2 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_2 = 2,
    #[doc = "15: +15 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_15 = 15,
    #[doc = "16: -16 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_16 = 16,
    #[doc = "17: -15 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_17 = 17,
    #[doc = "30: -2 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_30 = 30,
    #[doc = "31: -1 counts per each 32768 ticks of the counter"]
    HPCALB_VAL_31 = 31,
}
impl From<HPCALB_VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: HPCALB_VAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HPCALB_VAL`"]
pub type HPCALB_VAL_R = crate::R<u8, HPCALB_VAL_A>;
impl HPCALB_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HPCALB_VAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HPCALB_VAL_A::HPCALB_VAL_0),
            1 => Val(HPCALB_VAL_A::HPCALB_VAL_1),
            2 => Val(HPCALB_VAL_A::HPCALB_VAL_2),
            15 => Val(HPCALB_VAL_A::HPCALB_VAL_15),
            16 => Val(HPCALB_VAL_A::HPCALB_VAL_16),
            17 => Val(HPCALB_VAL_A::HPCALB_VAL_17),
            30 => Val(HPCALB_VAL_A::HPCALB_VAL_30),
            31 => Val(HPCALB_VAL_A::HPCALB_VAL_31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_0`"]
    #[inline(always)]
    pub fn is_hpcalb_val_0(&self) -> bool {
        *self == HPCALB_VAL_A::HPCALB_VAL_0
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_1`"]
    #[inline(always)]
    pub fn is_hpcalb_val_1(&self) -> bool {
        *self == HPCALB_VAL_A::HPCALB_VAL_1
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_2`"]
    #[inline(always)]
    pub fn is_hpcalb_val_2(&self) -> bool {
        *self == HPCALB_VAL_A::HPCALB_VAL_2
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_15`"]
    #[inline(always)]
    pub fn is_hpcalb_val_15(&self) -> bool {
        *self == HPCALB_VAL_A::HPCALB_VAL_15
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_16`"]
    #[inline(always)]
    pub fn is_hpcalb_val_16(&self) -> bool {
        *self == HPCALB_VAL_A::HPCALB_VAL_16
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_17`"]
    #[inline(always)]
    pub fn is_hpcalb_val_17(&self) -> bool {
        *self == HPCALB_VAL_A::HPCALB_VAL_17
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_30`"]
    #[inline(always)]
    pub fn is_hpcalb_val_30(&self) -> bool {
        *self == HPCALB_VAL_A::HPCALB_VAL_30
    }
    #[doc = "Checks if the value of the field is `HPCALB_VAL_31`"]
    #[inline(always)]
    pub fn is_hpcalb_val_31(&self) -> bool {
        *self == HPCALB_VAL_A::HPCALB_VAL_31
    }
}
#[doc = "Write proxy for field `HPCALB_VAL`"]
pub struct HPCALB_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> HPCALB_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPCALB_VAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "+0 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn hpcalb_val_0(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::HPCALB_VAL_0)
    }
    #[doc = "+1 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn hpcalb_val_1(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::HPCALB_VAL_1)
    }
    #[doc = "+2 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn hpcalb_val_2(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::HPCALB_VAL_2)
    }
    #[doc = "+15 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn hpcalb_val_15(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::HPCALB_VAL_15)
    }
    #[doc = "-16 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn hpcalb_val_16(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::HPCALB_VAL_16)
    }
    #[doc = "-15 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn hpcalb_val_17(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::HPCALB_VAL_17)
    }
    #[doc = "-2 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn hpcalb_val_30(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::HPCALB_VAL_30)
    }
    #[doc = "-1 counts per each 32768 ticks of the counter"]
    #[inline(always)]
    pub fn hpcalb_val_31(self) -> &'a mut W {
        self.variant(HPCALB_VAL_A::HPCALB_VAL_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "HP Time Synchronize\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HP_TS_A {
    #[doc = "0: No Action"]
    HP_TS_0 = 0,
    #[doc = "1: Synchronize the HP Time Counter to the LP Time Counter"]
    HP_TS_1 = 1,
}
impl From<HP_TS_A> for bool {
    #[inline(always)]
    fn from(variant: HP_TS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HP_TS`"]
pub type HP_TS_R = crate::R<bool, HP_TS_A>;
impl HP_TS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HP_TS_A {
        match self.bits {
            false => HP_TS_A::HP_TS_0,
            true => HP_TS_A::HP_TS_1,
        }
    }
    #[doc = "Checks if the value of the field is `HP_TS_0`"]
    #[inline(always)]
    pub fn is_hp_ts_0(&self) -> bool {
        *self == HP_TS_A::HP_TS_0
    }
    #[doc = "Checks if the value of the field is `HP_TS_1`"]
    #[inline(always)]
    pub fn is_hp_ts_1(&self) -> bool {
        *self == HP_TS_A::HP_TS_1
    }
}
#[doc = "Write proxy for field `HP_TS`"]
pub struct HP_TS_W<'a> {
    w: &'a mut W,
}
impl<'a> HP_TS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HP_TS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn hp_ts_0(self) -> &'a mut W {
        self.variant(HP_TS_A::HP_TS_0)
    }
    #[doc = "Synchronize the HP Time Counter to the LP Time Counter"]
    #[inline(always)]
    pub fn hp_ts_1(self) -> &'a mut W {
        self.variant(HP_TS_A::HP_TS_1)
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
#[doc = "Reader of field `BTN_CONFIG`"]
pub type BTN_CONFIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BTN_CONFIG`"]
pub struct BTN_CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> BTN_CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `BTN_MASK`"]
pub type BTN_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BTN_MASK`"]
pub struct BTN_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> BTN_MASK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - HP Real Time Counter Enable"]
    #[inline(always)]
    pub fn rtc_en(&self) -> RTC_EN_R {
        RTC_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
    #[inline(always)]
    pub fn hpta_en(&self) -> HPTA_EN_R {
        HPTA_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable periodic interrupt in the functional interrupt"]
    #[inline(always)]
    pub fn dis_pi(&self) -> DIS_PI_R {
        DIS_PI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
    #[inline(always)]
    pub fn pi_en(&self) -> PI_EN_R {
        PI_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq(&self) -> PI_FREQ_R {
        PI_FREQ_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
    #[inline(always)]
    pub fn hpcalb_en(&self) -> HPCALB_EN_R {
        HPCALB_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:14 - HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
    #[inline(always)]
    pub fn hpcalb_val(&self) -> HPCALB_VAL_R {
        HPCALB_VAL_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - HP Time Synchronize"]
    #[inline(always)]
    pub fn hp_ts(&self) -> HP_TS_R {
        HP_TS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Button Configuration"]
    #[inline(always)]
    pub fn btn_config(&self) -> BTN_CONFIG_R {
        BTN_CONFIG_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Button interrupt mask"]
    #[inline(always)]
    pub fn btn_mask(&self) -> BTN_MASK_R {
        BTN_MASK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HP Real Time Counter Enable"]
    #[inline(always)]
    pub fn rtc_en(&mut self) -> RTC_EN_W {
        RTC_EN_W { w: self }
    }
    #[doc = "Bit 1 - HP Time Alarm Enable When set, the time alarm interrupt is generated if the value in the HP Time Alarm Registers is equal to the value of the HP Real Time Counter"]
    #[inline(always)]
    pub fn hpta_en(&mut self) -> HPTA_EN_W {
        HPTA_EN_W { w: self }
    }
    #[doc = "Bit 2 - Disable periodic interrupt in the functional interrupt"]
    #[inline(always)]
    pub fn dis_pi(&mut self) -> DIS_PI_W {
        DIS_PI_W { w: self }
    }
    #[doc = "Bit 3 - HP Periodic Interrupt Enable The periodic interrupt can be generated only if the HP Real Time Counter is enabled"]
    #[inline(always)]
    pub fn pi_en(&mut self) -> PI_EN_W {
        PI_EN_W { w: self }
    }
    #[doc = "Bits 4:7 - Periodic Interrupt Frequency Defines frequency of the periodic interrupt"]
    #[inline(always)]
    pub fn pi_freq(&mut self) -> PI_FREQ_W {
        PI_FREQ_W { w: self }
    }
    #[doc = "Bit 8 - HP Real Time Counter Calibration Enabled Indicates that the time calibration mechanism is enabled."]
    #[inline(always)]
    pub fn hpcalb_en(&mut self) -> HPCALB_EN_W {
        HPCALB_EN_W { w: self }
    }
    #[doc = "Bits 10:14 - HP Calibration Value Defines signed calibration value for the HP Real Time Counter"]
    #[inline(always)]
    pub fn hpcalb_val(&mut self) -> HPCALB_VAL_W {
        HPCALB_VAL_W { w: self }
    }
    #[doc = "Bit 16 - HP Time Synchronize"]
    #[inline(always)]
    pub fn hp_ts(&mut self) -> HP_TS_W {
        HP_TS_W { w: self }
    }
    #[doc = "Bits 24:26 - Button Configuration"]
    #[inline(always)]
    pub fn btn_config(&mut self) -> BTN_CONFIG_W {
        BTN_CONFIG_W { w: self }
    }
    #[doc = "Bit 27 - Button interrupt mask"]
    #[inline(always)]
    pub fn btn_mask(&mut self) -> BTN_MASK_W {
        BTN_MASK_W { w: self }
    }
}
