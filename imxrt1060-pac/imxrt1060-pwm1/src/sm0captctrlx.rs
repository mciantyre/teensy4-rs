#[doc = "Reader of register SM0CAPTCTRLX"]
pub type R = crate::R<u16, super::SM0CAPTCTRLX>;
#[doc = "Writer for register SM0CAPTCTRLX"]
pub type W = crate::W<u16, super::SM0CAPTCTRLX>;
#[doc = "Register SM0CAPTCTRLX `reset()`'s with value 0"]
impl crate::ResetValue for super::SM0CAPTCTRLX {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Arm X\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARMX_A {
    #[doc = "0: Input capture operation is disabled."]
    ARMX_0,
    #[doc = "1: Input capture operation as specified by CAPTCTRLX\\[EDGXx\\] is enabled."]
    ARMX_1,
}
impl From<ARMX_A> for bool {
    #[inline(always)]
    fn from(variant: ARMX_A) -> Self {
        match variant {
            ARMX_A::ARMX_0 => false,
            ARMX_A::ARMX_1 => true,
        }
    }
}
#[doc = "Reader of field `ARMX`"]
pub type ARMX_R = crate::R<bool, ARMX_A>;
impl ARMX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARMX_A {
        match self.bits {
            false => ARMX_A::ARMX_0,
            true => ARMX_A::ARMX_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARMX_0`"]
    #[inline(always)]
    pub fn is_armx_0(&self) -> bool {
        *self == ARMX_A::ARMX_0
    }
    #[doc = "Checks if the value of the field is `ARMX_1`"]
    #[inline(always)]
    pub fn is_armx_1(&self) -> bool {
        *self == ARMX_A::ARMX_1
    }
}
#[doc = "Write proxy for field `ARMX`"]
pub struct ARMX_W<'a> {
    w: &'a mut W,
}
impl<'a> ARMX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARMX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input capture operation is disabled."]
    #[inline(always)]
    pub fn armx_0(self) -> &'a mut W {
        self.variant(ARMX_A::ARMX_0)
    }
    #[doc = "Input capture operation as specified by CAPTCTRLX\\[EDGXx\\] is enabled."]
    #[inline(always)]
    pub fn armx_1(self) -> &'a mut W {
        self.variant(ARMX_A::ARMX_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "One Shot Mode Aux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONESHOTX_A {
    #[doc = "0: Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after the ARMX bit is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit."]
    ONESHOTX_0,
    #[doc = "1: One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after the ARMX bit is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and the ARMX bit is cleared. No further captures will be performed until the ARMX bit is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and the ARMX bit is then cleared."]
    ONESHOTX_1,
}
impl From<ONESHOTX_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOTX_A) -> Self {
        match variant {
            ONESHOTX_A::ONESHOTX_0 => false,
            ONESHOTX_A::ONESHOTX_1 => true,
        }
    }
}
#[doc = "Reader of field `ONESHOTX`"]
pub type ONESHOTX_R = crate::R<bool, ONESHOTX_A>;
impl ONESHOTX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOTX_A {
        match self.bits {
            false => ONESHOTX_A::ONESHOTX_0,
            true => ONESHOTX_A::ONESHOTX_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONESHOTX_0`"]
    #[inline(always)]
    pub fn is_oneshotx_0(&self) -> bool {
        *self == ONESHOTX_A::ONESHOTX_0
    }
    #[doc = "Checks if the value of the field is `ONESHOTX_1`"]
    #[inline(always)]
    pub fn is_oneshotx_1(&self) -> bool {
        *self == ONESHOTX_A::ONESHOTX_1
    }
}
#[doc = "Write proxy for field `ONESHOTX`"]
pub struct ONESHOTX_W<'a> {
    w: &'a mut W,
}
impl<'a> ONESHOTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONESHOTX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after the ARMX bit is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit."]
    #[inline(always)]
    pub fn oneshotx_0(self) -> &'a mut W {
        self.variant(ONESHOTX_A::ONESHOTX_0)
    }
    #[doc = "One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after the ARMX bit is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and the ARMX bit is cleared. No further captures will be performed until the ARMX bit is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and the ARMX bit is then cleared."]
    #[inline(always)]
    pub fn oneshotx_1(self) -> &'a mut W {
        self.variant(ONESHOTX_A::ONESHOTX_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Edge X 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGX0_A {
    #[doc = "0: Disabled"]
    EDGX0_0,
    #[doc = "1: Capture falling edges"]
    EDGX0_1,
    #[doc = "2: Capture rising edges"]
    EDGX0_2,
    #[doc = "3: Capture any edge"]
    EDGX0_3,
}
impl From<EDGX0_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGX0_A) -> Self {
        match variant {
            EDGX0_A::EDGX0_0 => 0,
            EDGX0_A::EDGX0_1 => 1,
            EDGX0_A::EDGX0_2 => 2,
            EDGX0_A::EDGX0_3 => 3,
        }
    }
}
#[doc = "Reader of field `EDGX0`"]
pub type EDGX0_R = crate::R<u8, EDGX0_A>;
impl EDGX0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGX0_A {
        match self.bits {
            0 => EDGX0_A::EDGX0_0,
            1 => EDGX0_A::EDGX0_1,
            2 => EDGX0_A::EDGX0_2,
            3 => EDGX0_A::EDGX0_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGX0_0`"]
    #[inline(always)]
    pub fn is_edgx0_0(&self) -> bool {
        *self == EDGX0_A::EDGX0_0
    }
    #[doc = "Checks if the value of the field is `EDGX0_1`"]
    #[inline(always)]
    pub fn is_edgx0_1(&self) -> bool {
        *self == EDGX0_A::EDGX0_1
    }
    #[doc = "Checks if the value of the field is `EDGX0_2`"]
    #[inline(always)]
    pub fn is_edgx0_2(&self) -> bool {
        *self == EDGX0_A::EDGX0_2
    }
    #[doc = "Checks if the value of the field is `EDGX0_3`"]
    #[inline(always)]
    pub fn is_edgx0_3(&self) -> bool {
        *self == EDGX0_A::EDGX0_3
    }
}
#[doc = "Write proxy for field `EDGX0`"]
pub struct EDGX0_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGX0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGX0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn edgx0_0(self) -> &'a mut W {
        self.variant(EDGX0_A::EDGX0_0)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn edgx0_1(self) -> &'a mut W {
        self.variant(EDGX0_A::EDGX0_1)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn edgx0_2(self) -> &'a mut W {
        self.variant(EDGX0_A::EDGX0_2)
    }
    #[doc = "Capture any edge"]
    #[inline(always)]
    pub fn edgx0_3(self) -> &'a mut W {
        self.variant(EDGX0_A::EDGX0_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Edge X 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGX1_A {
    #[doc = "0: Disabled"]
    EDGX1_0,
    #[doc = "1: Capture falling edges"]
    EDGX1_1,
    #[doc = "2: Capture rising edges"]
    EDGX1_2,
    #[doc = "3: Capture any edge"]
    EDGX1_3,
}
impl From<EDGX1_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGX1_A) -> Self {
        match variant {
            EDGX1_A::EDGX1_0 => 0,
            EDGX1_A::EDGX1_1 => 1,
            EDGX1_A::EDGX1_2 => 2,
            EDGX1_A::EDGX1_3 => 3,
        }
    }
}
#[doc = "Reader of field `EDGX1`"]
pub type EDGX1_R = crate::R<u8, EDGX1_A>;
impl EDGX1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGX1_A {
        match self.bits {
            0 => EDGX1_A::EDGX1_0,
            1 => EDGX1_A::EDGX1_1,
            2 => EDGX1_A::EDGX1_2,
            3 => EDGX1_A::EDGX1_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGX1_0`"]
    #[inline(always)]
    pub fn is_edgx1_0(&self) -> bool {
        *self == EDGX1_A::EDGX1_0
    }
    #[doc = "Checks if the value of the field is `EDGX1_1`"]
    #[inline(always)]
    pub fn is_edgx1_1(&self) -> bool {
        *self == EDGX1_A::EDGX1_1
    }
    #[doc = "Checks if the value of the field is `EDGX1_2`"]
    #[inline(always)]
    pub fn is_edgx1_2(&self) -> bool {
        *self == EDGX1_A::EDGX1_2
    }
    #[doc = "Checks if the value of the field is `EDGX1_3`"]
    #[inline(always)]
    pub fn is_edgx1_3(&self) -> bool {
        *self == EDGX1_A::EDGX1_3
    }
}
#[doc = "Write proxy for field `EDGX1`"]
pub struct EDGX1_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGX1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGX1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn edgx1_0(self) -> &'a mut W {
        self.variant(EDGX1_A::EDGX1_0)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn edgx1_1(self) -> &'a mut W {
        self.variant(EDGX1_A::EDGX1_1)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn edgx1_2(self) -> &'a mut W {
        self.variant(EDGX1_A::EDGX1_2)
    }
    #[doc = "Capture any edge"]
    #[inline(always)]
    pub fn edgx1_3(self) -> &'a mut W {
        self.variant(EDGX1_A::EDGX1_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Input Select X\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INP_SELX_A {
    #[doc = "0: Raw PWM_X input signal selected as source."]
    INP_SELX_0,
    #[doc = "1: Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLX\\[EDGX0\\] and CAPTCTRLX\\[EDGX1\\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRX\\[EDGX0\\] and/or CAPTCTRLX\\[EDGX1\\] fields in order to enable one or both of the capture registers."]
    INP_SELX_1,
}
impl From<INP_SELX_A> for bool {
    #[inline(always)]
    fn from(variant: INP_SELX_A) -> Self {
        match variant {
            INP_SELX_A::INP_SELX_0 => false,
            INP_SELX_A::INP_SELX_1 => true,
        }
    }
}
#[doc = "Reader of field `INP_SELX`"]
pub type INP_SELX_R = crate::R<bool, INP_SELX_A>;
impl INP_SELX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INP_SELX_A {
        match self.bits {
            false => INP_SELX_A::INP_SELX_0,
            true => INP_SELX_A::INP_SELX_1,
        }
    }
    #[doc = "Checks if the value of the field is `INP_SELX_0`"]
    #[inline(always)]
    pub fn is_inp_selx_0(&self) -> bool {
        *self == INP_SELX_A::INP_SELX_0
    }
    #[doc = "Checks if the value of the field is `INP_SELX_1`"]
    #[inline(always)]
    pub fn is_inp_selx_1(&self) -> bool {
        *self == INP_SELX_A::INP_SELX_1
    }
}
#[doc = "Write proxy for field `INP_SELX`"]
pub struct INP_SELX_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_SELX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INP_SELX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Raw PWM_X input signal selected as source."]
    #[inline(always)]
    pub fn inp_selx_0(self) -> &'a mut W {
        self.variant(INP_SELX_A::INP_SELX_0)
    }
    #[doc = "Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLX\\[EDGX0\\] and CAPTCTRLX\\[EDGX1\\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRX\\[EDGX0\\] and/or CAPTCTRLX\\[EDGX1\\] fields in order to enable one or both of the capture registers."]
    #[inline(always)]
    pub fn inp_selx_1(self) -> &'a mut W {
        self.variant(INP_SELX_A::INP_SELX_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Edge Counter X Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGCNTX_EN_A {
    #[doc = "0: Edge counter disabled and held in reset"]
    EDGCNTX_EN_0,
    #[doc = "1: Edge counter enabled"]
    EDGCNTX_EN_1,
}
impl From<EDGCNTX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EDGCNTX_EN_A) -> Self {
        match variant {
            EDGCNTX_EN_A::EDGCNTX_EN_0 => false,
            EDGCNTX_EN_A::EDGCNTX_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `EDGCNTX_EN`"]
pub type EDGCNTX_EN_R = crate::R<bool, EDGCNTX_EN_A>;
impl EDGCNTX_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGCNTX_EN_A {
        match self.bits {
            false => EDGCNTX_EN_A::EDGCNTX_EN_0,
            true => EDGCNTX_EN_A::EDGCNTX_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDGCNTX_EN_0`"]
    #[inline(always)]
    pub fn is_edgcntx_en_0(&self) -> bool {
        *self == EDGCNTX_EN_A::EDGCNTX_EN_0
    }
    #[doc = "Checks if the value of the field is `EDGCNTX_EN_1`"]
    #[inline(always)]
    pub fn is_edgcntx_en_1(&self) -> bool {
        *self == EDGCNTX_EN_A::EDGCNTX_EN_1
    }
}
#[doc = "Write proxy for field `EDGCNTX_EN`"]
pub struct EDGCNTX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGCNTX_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGCNTX_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Edge counter disabled and held in reset"]
    #[inline(always)]
    pub fn edgcntx_en_0(self) -> &'a mut W {
        self.variant(EDGCNTX_EN_A::EDGCNTX_EN_0)
    }
    #[doc = "Edge counter enabled"]
    #[inline(always)]
    pub fn edgcntx_en_1(self) -> &'a mut W {
        self.variant(EDGCNTX_EN_A::EDGCNTX_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CFXWM`"]
pub type CFXWM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFXWM`"]
pub struct CFXWM_W<'a> {
    w: &'a mut W,
}
impl<'a> CFXWM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CX0CNT`"]
pub type CX0CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `CX1CNT`"]
pub type CX1CNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Arm X"]
    #[inline(always)]
    pub fn armx(&self) -> ARMX_R {
        ARMX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - One Shot Mode Aux"]
    #[inline(always)]
    pub fn oneshotx(&self) -> ONESHOTX_R {
        ONESHOTX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Edge X 0"]
    #[inline(always)]
    pub fn edgx0(&self) -> EDGX0_R {
        EDGX0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Edge X 1"]
    #[inline(always)]
    pub fn edgx1(&self) -> EDGX1_R {
        EDGX1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Input Select X"]
    #[inline(always)]
    pub fn inp_selx(&self) -> INP_SELX_R {
        INP_SELX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Edge Counter X Enable"]
    #[inline(always)]
    pub fn edgcntx_en(&self) -> EDGCNTX_EN_R {
        EDGCNTX_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub fn cfxwm(&self) -> CFXWM_R {
        CFXWM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12 - Capture X0 FIFO Word Count"]
    #[inline(always)]
    pub fn cx0cnt(&self) -> CX0CNT_R {
        CX0CNT_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 13:15 - Capture X1 FIFO Word Count"]
    #[inline(always)]
    pub fn cx1cnt(&self) -> CX1CNT_R {
        CX1CNT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Arm X"]
    #[inline(always)]
    pub fn armx(&mut self) -> ARMX_W {
        ARMX_W { w: self }
    }
    #[doc = "Bit 1 - One Shot Mode Aux"]
    #[inline(always)]
    pub fn oneshotx(&mut self) -> ONESHOTX_W {
        ONESHOTX_W { w: self }
    }
    #[doc = "Bits 2:3 - Edge X 0"]
    #[inline(always)]
    pub fn edgx0(&mut self) -> EDGX0_W {
        EDGX0_W { w: self }
    }
    #[doc = "Bits 4:5 - Edge X 1"]
    #[inline(always)]
    pub fn edgx1(&mut self) -> EDGX1_W {
        EDGX1_W { w: self }
    }
    #[doc = "Bit 6 - Input Select X"]
    #[inline(always)]
    pub fn inp_selx(&mut self) -> INP_SELX_W {
        INP_SELX_W { w: self }
    }
    #[doc = "Bit 7 - Edge Counter X Enable"]
    #[inline(always)]
    pub fn edgcntx_en(&mut self) -> EDGCNTX_EN_W {
        EDGCNTX_EN_W { w: self }
    }
    #[doc = "Bits 8:9 - Capture X FIFOs Water Mark"]
    #[inline(always)]
    pub fn cfxwm(&mut self) -> CFXWM_W {
        CFXWM_W { w: self }
    }
}
