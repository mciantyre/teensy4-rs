#[doc = "Reader of register HPSR"]
pub type R = crate::R<u32, super::HPSR>;
#[doc = "Writer for register HPSR"]
pub type W = crate::W<u32, super::HPSR>;
#[doc = "Register HPSR `reset()`'s with value 0x8000_3000"]
impl crate::ResetValue for super::HPSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_3000
    }
}
#[doc = "HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPTA_A {
    #[doc = "0: No time alarm interrupt occurred."]
    HPTA_0 = 0,
    #[doc = "1: A time alarm interrupt occurred."]
    HPTA_1 = 1,
}
impl From<HPTA_A> for bool {
    #[inline(always)]
    fn from(variant: HPTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HPTA`"]
pub type HPTA_R = crate::R<bool, HPTA_A>;
impl HPTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPTA_A {
        match self.bits {
            false => HPTA_A::HPTA_0,
            true => HPTA_A::HPTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPTA_0`"]
    #[inline(always)]
    pub fn is_hpta_0(&self) -> bool {
        *self == HPTA_A::HPTA_0
    }
    #[doc = "Checks if the value of the field is `HPTA_1`"]
    #[inline(always)]
    pub fn is_hpta_1(&self) -> bool {
        *self == HPTA_A::HPTA_1
    }
}
#[doc = "Write proxy for field `HPTA`"]
pub struct HPTA_W<'a> {
    w: &'a mut W,
}
impl<'a> HPTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPTA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No time alarm interrupt occurred."]
    #[inline(always)]
    pub fn hpta_0(self) -> &'a mut W {
        self.variant(HPTA_A::HPTA_0)
    }
    #[doc = "A time alarm interrupt occurred."]
    #[inline(always)]
    pub fn hpta_1(self) -> &'a mut W {
        self.variant(HPTA_A::HPTA_1)
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
#[doc = "Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PI_A {
    #[doc = "0: No periodic interrupt occurred."]
    PI_0 = 0,
    #[doc = "1: A periodic interrupt occurred."]
    PI_1 = 1,
}
impl From<PI_A> for bool {
    #[inline(always)]
    fn from(variant: PI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PI`"]
pub type PI_R = crate::R<bool, PI_A>;
impl PI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PI_A {
        match self.bits {
            false => PI_A::PI_0,
            true => PI_A::PI_1,
        }
    }
    #[doc = "Checks if the value of the field is `PI_0`"]
    #[inline(always)]
    pub fn is_pi_0(&self) -> bool {
        *self == PI_A::PI_0
    }
    #[doc = "Checks if the value of the field is `PI_1`"]
    #[inline(always)]
    pub fn is_pi_1(&self) -> bool {
        *self == PI_A::PI_1
    }
}
#[doc = "Write proxy for field `PI`"]
pub struct PI_W<'a> {
    w: &'a mut W,
}
impl<'a> PI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No periodic interrupt occurred."]
    #[inline(always)]
    pub fn pi_0(self) -> &'a mut W {
        self.variant(PI_A::PI_0)
    }
    #[doc = "A periodic interrupt occurred."]
    #[inline(always)]
    pub fn pi_1(self) -> &'a mut W {
        self.variant(PI_A::PI_1)
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
#[doc = "Reader of field `LPDIS`"]
pub type LPDIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTN`"]
pub type BTN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BI`"]
pub type BI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BI`"]
pub struct BI_W<'a> {
    w: &'a mut W,
}
impl<'a> BI_W<'a> {
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
#[doc = "System Security Monitor State This field contains the encoded state of the SSM's state machine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSM_STATE_A {
    #[doc = "0: Init"]
    SSM_STATE_0 = 0,
    #[doc = "1: Hard Fail"]
    SSM_STATE_1 = 1,
    #[doc = "3: Soft Fail"]
    SSM_STATE_3 = 3,
    #[doc = "8: Init Intermediate (transition state between Init and Check - SSM stays in this state only one clock cycle)"]
    SSM_STATE_8 = 8,
    #[doc = "9: Check"]
    SSM_STATE_9 = 9,
    #[doc = "11: Non-Secure"]
    SSM_STATE_11 = 11,
    #[doc = "13: Trusted"]
    SSM_STATE_13 = 13,
    #[doc = "15: Secure"]
    SSM_STATE_15 = 15,
}
impl From<SSM_STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSM_STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSM_STATE`"]
pub type SSM_STATE_R = crate::R<u8, SSM_STATE_A>;
impl SSM_STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSM_STATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SSM_STATE_A::SSM_STATE_0),
            1 => Val(SSM_STATE_A::SSM_STATE_1),
            3 => Val(SSM_STATE_A::SSM_STATE_3),
            8 => Val(SSM_STATE_A::SSM_STATE_8),
            9 => Val(SSM_STATE_A::SSM_STATE_9),
            11 => Val(SSM_STATE_A::SSM_STATE_11),
            13 => Val(SSM_STATE_A::SSM_STATE_13),
            15 => Val(SSM_STATE_A::SSM_STATE_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_0`"]
    #[inline(always)]
    pub fn is_ssm_state_0(&self) -> bool {
        *self == SSM_STATE_A::SSM_STATE_0
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_1`"]
    #[inline(always)]
    pub fn is_ssm_state_1(&self) -> bool {
        *self == SSM_STATE_A::SSM_STATE_1
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_3`"]
    #[inline(always)]
    pub fn is_ssm_state_3(&self) -> bool {
        *self == SSM_STATE_A::SSM_STATE_3
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_8`"]
    #[inline(always)]
    pub fn is_ssm_state_8(&self) -> bool {
        *self == SSM_STATE_A::SSM_STATE_8
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_9`"]
    #[inline(always)]
    pub fn is_ssm_state_9(&self) -> bool {
        *self == SSM_STATE_A::SSM_STATE_9
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_11`"]
    #[inline(always)]
    pub fn is_ssm_state_11(&self) -> bool {
        *self == SSM_STATE_A::SSM_STATE_11
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_13`"]
    #[inline(always)]
    pub fn is_ssm_state_13(&self) -> bool {
        *self == SSM_STATE_A::SSM_STATE_13
    }
    #[doc = "Checks if the value of the field is `SSM_STATE_15`"]
    #[inline(always)]
    pub fn is_ssm_state_15(&self) -> bool {
        *self == SSM_STATE_A::SSM_STATE_15
    }
}
#[doc = "Security Configuration This field reflects the settings of the sys_secure_boot input and the three security configuration inputs to SNVS\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SECURITY_CONFIG_A {
    #[doc = "0: FAB configuration 1"]
    FAB_CONFIG_1 = 0,
    #[doc = "1: OPEN configuration 1"]
    OPEN_CONFIG_1 = 1,
    #[doc = "2: OPEN configuration 2"]
    OPEN_CONFIG_2 = 2,
    #[doc = "3: OPEN configuration_3"]
    OPEN_CONFIG_3 = 3,
    #[doc = "4: FIELD RETURN configuration"]
    FIELD_RETURN_CONFIG = 4,
    #[doc = "8: FAB configuration 2"]
    FAB_CONFIG_2 = 8,
    #[doc = "9: CLOSED configuration 1"]
    CLOSED_CONFIG_1 = 9,
    #[doc = "10: CLOSED configuration 2"]
    CLOSED_CONFIG_2 = 10,
    #[doc = "11: CLOSED configuration 3"]
    CLOSED_CONFIG_3 = 11,
}
impl From<SECURITY_CONFIG_A> for u8 {
    #[inline(always)]
    fn from(variant: SECURITY_CONFIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SECURITY_CONFIG`"]
pub type SECURITY_CONFIG_R = crate::R<u8, SECURITY_CONFIG_A>;
impl SECURITY_CONFIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SECURITY_CONFIG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SECURITY_CONFIG_A::FAB_CONFIG_1),
            1 => Val(SECURITY_CONFIG_A::OPEN_CONFIG_1),
            2 => Val(SECURITY_CONFIG_A::OPEN_CONFIG_2),
            3 => Val(SECURITY_CONFIG_A::OPEN_CONFIG_3),
            4 => Val(SECURITY_CONFIG_A::FIELD_RETURN_CONFIG),
            8 => Val(SECURITY_CONFIG_A::FAB_CONFIG_2),
            9 => Val(SECURITY_CONFIG_A::CLOSED_CONFIG_1),
            10 => Val(SECURITY_CONFIG_A::CLOSED_CONFIG_2),
            11 => Val(SECURITY_CONFIG_A::CLOSED_CONFIG_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FAB_CONFIG_1`"]
    #[inline(always)]
    pub fn is_fab_config_1(&self) -> bool {
        *self == SECURITY_CONFIG_A::FAB_CONFIG_1
    }
    #[doc = "Checks if the value of the field is `OPEN_CONFIG_1`"]
    #[inline(always)]
    pub fn is_open_config_1(&self) -> bool {
        *self == SECURITY_CONFIG_A::OPEN_CONFIG_1
    }
    #[doc = "Checks if the value of the field is `OPEN_CONFIG_2`"]
    #[inline(always)]
    pub fn is_open_config_2(&self) -> bool {
        *self == SECURITY_CONFIG_A::OPEN_CONFIG_2
    }
    #[doc = "Checks if the value of the field is `OPEN_CONFIG_3`"]
    #[inline(always)]
    pub fn is_open_config_3(&self) -> bool {
        *self == SECURITY_CONFIG_A::OPEN_CONFIG_3
    }
    #[doc = "Checks if the value of the field is `FIELD_RETURN_CONFIG`"]
    #[inline(always)]
    pub fn is_field_return_config(&self) -> bool {
        *self == SECURITY_CONFIG_A::FIELD_RETURN_CONFIG
    }
    #[doc = "Checks if the value of the field is `FAB_CONFIG_2`"]
    #[inline(always)]
    pub fn is_fab_config_2(&self) -> bool {
        *self == SECURITY_CONFIG_A::FAB_CONFIG_2
    }
    #[doc = "Checks if the value of the field is `CLOSED_CONFIG_1`"]
    #[inline(always)]
    pub fn is_closed_config_1(&self) -> bool {
        *self == SECURITY_CONFIG_A::CLOSED_CONFIG_1
    }
    #[doc = "Checks if the value of the field is `CLOSED_CONFIG_2`"]
    #[inline(always)]
    pub fn is_closed_config_2(&self) -> bool {
        *self == SECURITY_CONFIG_A::CLOSED_CONFIG_2
    }
    #[doc = "Checks if the value of the field is `CLOSED_CONFIG_3`"]
    #[inline(always)]
    pub fn is_closed_config_3(&self) -> bool {
        *self == SECURITY_CONFIG_A::CLOSED_CONFIG_3
    }
}
#[doc = "Reader of field `OTPMK_SYNDROME`"]
pub type OTPMK_SYNDROME_R = crate::R<u16, u16>;
#[doc = "One Time Programmable Master Key is Equal to Zero\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTPMK_ZERO_A {
    #[doc = "0: The OTPMK is not zero."]
    OTPMK_ZERO_0 = 0,
    #[doc = "1: The OTPMK is zero."]
    OTPMK_ZERO_1 = 1,
}
impl From<OTPMK_ZERO_A> for bool {
    #[inline(always)]
    fn from(variant: OTPMK_ZERO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OTPMK_ZERO`"]
pub type OTPMK_ZERO_R = crate::R<bool, OTPMK_ZERO_A>;
impl OTPMK_ZERO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTPMK_ZERO_A {
        match self.bits {
            false => OTPMK_ZERO_A::OTPMK_ZERO_0,
            true => OTPMK_ZERO_A::OTPMK_ZERO_1,
        }
    }
    #[doc = "Checks if the value of the field is `OTPMK_ZERO_0`"]
    #[inline(always)]
    pub fn is_otpmk_zero_0(&self) -> bool {
        *self == OTPMK_ZERO_A::OTPMK_ZERO_0
    }
    #[doc = "Checks if the value of the field is `OTPMK_ZERO_1`"]
    #[inline(always)]
    pub fn is_otpmk_zero_1(&self) -> bool {
        *self == OTPMK_ZERO_A::OTPMK_ZERO_1
    }
}
#[doc = "Zeroizable Master Key is Equal to Zero\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_ZERO_A {
    #[doc = "0: The ZMK is not zero."]
    ZMK_ZERO_0 = 0,
    #[doc = "1: The ZMK is zero."]
    ZMK_ZERO_1 = 1,
}
impl From<ZMK_ZERO_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_ZERO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ZMK_ZERO`"]
pub type ZMK_ZERO_R = crate::R<bool, ZMK_ZERO_A>;
impl ZMK_ZERO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_ZERO_A {
        match self.bits {
            false => ZMK_ZERO_A::ZMK_ZERO_0,
            true => ZMK_ZERO_A::ZMK_ZERO_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_ZERO_0`"]
    #[inline(always)]
    pub fn is_zmk_zero_0(&self) -> bool {
        *self == ZMK_ZERO_A::ZMK_ZERO_0
    }
    #[doc = "Checks if the value of the field is `ZMK_ZERO_1`"]
    #[inline(always)]
    pub fn is_zmk_zero_1(&self) -> bool {
        *self == ZMK_ZERO_A::ZMK_ZERO_1
    }
}
impl R {
    #[doc = "Bit 0 - HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
    #[inline(always)]
    pub fn hpta(&self) -> HPTA_R {
        HPTA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Low Power Disable If 1, the low power section has been disabled by means of an input signal to SNVS"]
    #[inline(always)]
    pub fn lpdis(&self) -> LPDIS_R {
        LPDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Button Value of the BTN input"]
    #[inline(always)]
    pub fn btn(&self) -> BTN_R {
        BTN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
    #[inline(always)]
    pub fn bi(&self) -> BI_R {
        BI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - System Security Monitor State This field contains the encoded state of the SSM's state machine"]
    #[inline(always)]
    pub fn ssm_state(&self) -> SSM_STATE_R {
        SSM_STATE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Security Configuration This field reflects the settings of the sys_secure_boot input and the three security configuration inputs to SNVS"]
    #[inline(always)]
    pub fn security_config(&self) -> SECURITY_CONFIG_R {
        SECURITY_CONFIG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - One Time Programmable Master Key Syndrome In the case of a single-bit error, the eight lower bits of this value indicate the bit number of error location"]
    #[inline(always)]
    pub fn otpmk_syndrome(&self) -> OTPMK_SYNDROME_R {
        OTPMK_SYNDROME_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - One Time Programmable Master Key is Equal to Zero"]
    #[inline(always)]
    pub fn otpmk_zero(&self) -> OTPMK_ZERO_R {
        OTPMK_ZERO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Zeroizable Master Key is Equal to Zero"]
    #[inline(always)]
    pub fn zmk_zero(&self) -> ZMK_ZERO_R {
        ZMK_ZERO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HP Time Alarm Indicates that the HP Time Alarm has occurred since this bit was last cleared."]
    #[inline(always)]
    pub fn hpta(&mut self) -> HPTA_W {
        HPTA_W { w: self }
    }
    #[doc = "Bit 1 - Periodic Interrupt Indicates that periodic interrupt has occurred since this bit was last cleared."]
    #[inline(always)]
    pub fn pi(&mut self) -> PI_W {
        PI_W { w: self }
    }
    #[doc = "Bit 7 - Button Interrupt Signal ipi_snvs_btn_int_b was asserted."]
    #[inline(always)]
    pub fn bi(&mut self) -> BI_W {
        BI_W { w: self }
    }
}
