#[doc = "Reader of register SMCAPTCTRLB"]
pub type R = crate::R<u16, super::SMCAPTCTRLB>;
#[doc = "Writer for register SMCAPTCTRLB"]
pub type W = crate::W<u16, super::SMCAPTCTRLB>;
#[doc = "Register SMCAPTCTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::SMCAPTCTRLB {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Arm B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARMB_A {
    #[doc = "0: Input capture operation is disabled."]
    ARMB_0 = 0,
    #[doc = "1: Input capture operation as specified by CAPTCTRLB\\[EDGBx\\]
is enabled."]
    ARMB_1 = 1,
}
impl From<ARMB_A> for bool {
    #[inline(always)]
    fn from(variant: ARMB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARMB`"]
pub type ARMB_R = crate::R<bool, ARMB_A>;
impl ARMB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARMB_A {
        match self.bits {
            false => ARMB_A::ARMB_0,
            true => ARMB_A::ARMB_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARMB_0`"]
    #[inline(always)]
    pub fn is_armb_0(&self) -> bool {
        *self == ARMB_A::ARMB_0
    }
    #[doc = "Checks if the value of the field is `ARMB_1`"]
    #[inline(always)]
    pub fn is_armb_1(&self) -> bool {
        *self == ARMB_A::ARMB_1
    }
}
#[doc = "Write proxy for field `ARMB`"]
pub struct ARMB_W<'a> {
    w: &'a mut W,
}
impl<'a> ARMB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARMB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input capture operation is disabled."]
    #[inline(always)]
    pub fn armb_0(self) -> &'a mut W {
        self.variant(ARMB_A::ARMB_0)
    }
    #[doc = "Input capture operation as specified by CAPTCTRLB\\[EDGBx\\]
is enabled."]
    #[inline(always)]
    pub fn armb_1(self) -> &'a mut W {
        self.variant(ARMB_A::ARMB_1)
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
#[doc = "One Shot Mode B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONESHOTB_A {
    #[doc = "0: Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLB\\[ARMB\\]
is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit."]
    ONESHOTB_0 = 0,
    #[doc = "1: One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLB\\[ARMB\\]
is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and CAPTCTRLB\\[ARMB\\]
is cleared. No further captures will be performed until CAPTCTRLB\\[ARMB\\]
is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and CAPTCTRLB\\[ARMB\\]
is then cleared."]
    ONESHOTB_1 = 1,
}
impl From<ONESHOTB_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONESHOTB`"]
pub type ONESHOTB_R = crate::R<bool, ONESHOTB_A>;
impl ONESHOTB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOTB_A {
        match self.bits {
            false => ONESHOTB_A::ONESHOTB_0,
            true => ONESHOTB_A::ONESHOTB_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONESHOTB_0`"]
    #[inline(always)]
    pub fn is_oneshotb_0(&self) -> bool {
        *self == ONESHOTB_A::ONESHOTB_0
    }
    #[doc = "Checks if the value of the field is `ONESHOTB_1`"]
    #[inline(always)]
    pub fn is_oneshotb_1(&self) -> bool {
        *self == ONESHOTB_A::ONESHOTB_1
    }
}
#[doc = "Write proxy for field `ONESHOTB`"]
pub struct ONESHOTB_W<'a> {
    w: &'a mut W,
}
impl<'a> ONESHOTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONESHOTB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLB\\[ARMB\\]
is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit."]
    #[inline(always)]
    pub fn oneshotb_0(self) -> &'a mut W {
        self.variant(ONESHOTB_A::ONESHOTB_0)
    }
    #[doc = "One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLB\\[ARMB\\]
is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and CAPTCTRLB\\[ARMB\\]
is cleared. No further captures will be performed until CAPTCTRLB\\[ARMB\\]
is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and CAPTCTRLB\\[ARMB\\]
is then cleared."]
    #[inline(always)]
    pub fn oneshotb_1(self) -> &'a mut W {
        self.variant(ONESHOTB_A::ONESHOTB_1)
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
#[doc = "Edge B 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGB0_A {
    #[doc = "0: Disabled"]
    EDGB0_0 = 0,
    #[doc = "1: Capture falling edges"]
    EDGB0_1 = 1,
    #[doc = "2: Capture rising edges"]
    EDGB0_2 = 2,
    #[doc = "3: Capture any edge"]
    EDGB0_3 = 3,
}
impl From<EDGB0_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGB0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EDGB0`"]
pub type EDGB0_R = crate::R<u8, EDGB0_A>;
impl EDGB0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGB0_A {
        match self.bits {
            0 => EDGB0_A::EDGB0_0,
            1 => EDGB0_A::EDGB0_1,
            2 => EDGB0_A::EDGB0_2,
            3 => EDGB0_A::EDGB0_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGB0_0`"]
    #[inline(always)]
    pub fn is_edgb0_0(&self) -> bool {
        *self == EDGB0_A::EDGB0_0
    }
    #[doc = "Checks if the value of the field is `EDGB0_1`"]
    #[inline(always)]
    pub fn is_edgb0_1(&self) -> bool {
        *self == EDGB0_A::EDGB0_1
    }
    #[doc = "Checks if the value of the field is `EDGB0_2`"]
    #[inline(always)]
    pub fn is_edgb0_2(&self) -> bool {
        *self == EDGB0_A::EDGB0_2
    }
    #[doc = "Checks if the value of the field is `EDGB0_3`"]
    #[inline(always)]
    pub fn is_edgb0_3(&self) -> bool {
        *self == EDGB0_A::EDGB0_3
    }
}
#[doc = "Write proxy for field `EDGB0`"]
pub struct EDGB0_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGB0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGB0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn edgb0_0(self) -> &'a mut W {
        self.variant(EDGB0_A::EDGB0_0)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn edgb0_1(self) -> &'a mut W {
        self.variant(EDGB0_A::EDGB0_1)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn edgb0_2(self) -> &'a mut W {
        self.variant(EDGB0_A::EDGB0_2)
    }
    #[doc = "Capture any edge"]
    #[inline(always)]
    pub fn edgb0_3(self) -> &'a mut W {
        self.variant(EDGB0_A::EDGB0_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Edge B 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDGB1_A {
    #[doc = "0: Disabled"]
    EDGB1_0 = 0,
    #[doc = "1: Capture falling edges"]
    EDGB1_1 = 1,
    #[doc = "2: Capture rising edges"]
    EDGB1_2 = 2,
    #[doc = "3: Capture any edge"]
    EDGB1_3 = 3,
}
impl From<EDGB1_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGB1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EDGB1`"]
pub type EDGB1_R = crate::R<u8, EDGB1_A>;
impl EDGB1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGB1_A {
        match self.bits {
            0 => EDGB1_A::EDGB1_0,
            1 => EDGB1_A::EDGB1_1,
            2 => EDGB1_A::EDGB1_2,
            3 => EDGB1_A::EDGB1_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGB1_0`"]
    #[inline(always)]
    pub fn is_edgb1_0(&self) -> bool {
        *self == EDGB1_A::EDGB1_0
    }
    #[doc = "Checks if the value of the field is `EDGB1_1`"]
    #[inline(always)]
    pub fn is_edgb1_1(&self) -> bool {
        *self == EDGB1_A::EDGB1_1
    }
    #[doc = "Checks if the value of the field is `EDGB1_2`"]
    #[inline(always)]
    pub fn is_edgb1_2(&self) -> bool {
        *self == EDGB1_A::EDGB1_2
    }
    #[doc = "Checks if the value of the field is `EDGB1_3`"]
    #[inline(always)]
    pub fn is_edgb1_3(&self) -> bool {
        *self == EDGB1_A::EDGB1_3
    }
}
#[doc = "Write proxy for field `EDGB1`"]
pub struct EDGB1_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGB1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGB1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn edgb1_0(self) -> &'a mut W {
        self.variant(EDGB1_A::EDGB1_0)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn edgb1_1(self) -> &'a mut W {
        self.variant(EDGB1_A::EDGB1_1)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn edgb1_2(self) -> &'a mut W {
        self.variant(EDGB1_A::EDGB1_2)
    }
    #[doc = "Capture any edge"]
    #[inline(always)]
    pub fn edgb1_3(self) -> &'a mut W {
        self.variant(EDGB1_A::EDGB1_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Input Select B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INP_SELB_A {
    #[doc = "0: Raw PWM_B input signal selected as source."]
    INP_SELB_0 = 0,
    #[doc = "1: Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLB\\[EDGB0\\]
and CAPTCTRLB\\[EDGB1\\]
fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRB\\[EDGB0\\]
and/or CAPTCTRLB\\[EDGB1\\]
fields in order to enable one or both of the capture registers."]
    INP_SELB_1 = 1,
}
impl From<INP_SELB_A> for bool {
    #[inline(always)]
    fn from(variant: INP_SELB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INP_SELB`"]
pub type INP_SELB_R = crate::R<bool, INP_SELB_A>;
impl INP_SELB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INP_SELB_A {
        match self.bits {
            false => INP_SELB_A::INP_SELB_0,
            true => INP_SELB_A::INP_SELB_1,
        }
    }
    #[doc = "Checks if the value of the field is `INP_SELB_0`"]
    #[inline(always)]
    pub fn is_inp_selb_0(&self) -> bool {
        *self == INP_SELB_A::INP_SELB_0
    }
    #[doc = "Checks if the value of the field is `INP_SELB_1`"]
    #[inline(always)]
    pub fn is_inp_selb_1(&self) -> bool {
        *self == INP_SELB_A::INP_SELB_1
    }
}
#[doc = "Write proxy for field `INP_SELB`"]
pub struct INP_SELB_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_SELB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INP_SELB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Raw PWM_B input signal selected as source."]
    #[inline(always)]
    pub fn inp_selb_0(self) -> &'a mut W {
        self.variant(INP_SELB_A::INP_SELB_0)
    }
    #[doc = "Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLB\\[EDGB0\\]
and CAPTCTRLB\\[EDGB1\\]
fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRB\\[EDGB0\\]
and/or CAPTCTRLB\\[EDGB1\\]
fields in order to enable one or both of the capture registers."]
    #[inline(always)]
    pub fn inp_selb_1(self) -> &'a mut W {
        self.variant(INP_SELB_A::INP_SELB_1)
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
#[doc = "Edge Counter B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGCNTB_EN_A {
    #[doc = "0: Edge counter disabled and held in reset"]
    EDGCNTB_EN_0 = 0,
    #[doc = "1: Edge counter enabled"]
    EDGCNTB_EN_1 = 1,
}
impl From<EDGCNTB_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EDGCNTB_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDGCNTB_EN`"]
pub type EDGCNTB_EN_R = crate::R<bool, EDGCNTB_EN_A>;
impl EDGCNTB_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGCNTB_EN_A {
        match self.bits {
            false => EDGCNTB_EN_A::EDGCNTB_EN_0,
            true => EDGCNTB_EN_A::EDGCNTB_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDGCNTB_EN_0`"]
    #[inline(always)]
    pub fn is_edgcntb_en_0(&self) -> bool {
        *self == EDGCNTB_EN_A::EDGCNTB_EN_0
    }
    #[doc = "Checks if the value of the field is `EDGCNTB_EN_1`"]
    #[inline(always)]
    pub fn is_edgcntb_en_1(&self) -> bool {
        *self == EDGCNTB_EN_A::EDGCNTB_EN_1
    }
}
#[doc = "Write proxy for field `EDGCNTB_EN`"]
pub struct EDGCNTB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGCNTB_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGCNTB_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Edge counter disabled and held in reset"]
    #[inline(always)]
    pub fn edgcntb_en_0(self) -> &'a mut W {
        self.variant(EDGCNTB_EN_A::EDGCNTB_EN_0)
    }
    #[doc = "Edge counter enabled"]
    #[inline(always)]
    pub fn edgcntb_en_1(self) -> &'a mut W {
        self.variant(EDGCNTB_EN_A::EDGCNTB_EN_1)
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
#[doc = "Reader of field `CFBWM`"]
pub type CFBWM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFBWM`"]
pub struct CFBWM_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBWM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CB0CNT`"]
pub type CB0CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `CB1CNT`"]
pub type CB1CNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Arm B"]
    #[inline(always)]
    pub fn armb(&self) -> ARMB_R {
        ARMB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - One Shot Mode B"]
    #[inline(always)]
    pub fn oneshotb(&self) -> ONESHOTB_R {
        ONESHOTB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Edge B 0"]
    #[inline(always)]
    pub fn edgb0(&self) -> EDGB0_R {
        EDGB0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Edge B 1"]
    #[inline(always)]
    pub fn edgb1(&self) -> EDGB1_R {
        EDGB1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Input Select B"]
    #[inline(always)]
    pub fn inp_selb(&self) -> INP_SELB_R {
        INP_SELB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Edge Counter B Enable"]
    #[inline(always)]
    pub fn edgcntb_en(&self) -> EDGCNTB_EN_R {
        EDGCNTB_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub fn cfbwm(&self) -> CFBWM_R {
        CFBWM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12 - Capture B0 FIFO Word Count"]
    #[inline(always)]
    pub fn cb0cnt(&self) -> CB0CNT_R {
        CB0CNT_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 13:15 - Capture B1 FIFO Word Count"]
    #[inline(always)]
    pub fn cb1cnt(&self) -> CB1CNT_R {
        CB1CNT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Arm B"]
    #[inline(always)]
    pub fn armb(&mut self) -> ARMB_W {
        ARMB_W { w: self }
    }
    #[doc = "Bit 1 - One Shot Mode B"]
    #[inline(always)]
    pub fn oneshotb(&mut self) -> ONESHOTB_W {
        ONESHOTB_W { w: self }
    }
    #[doc = "Bits 2:3 - Edge B 0"]
    #[inline(always)]
    pub fn edgb0(&mut self) -> EDGB0_W {
        EDGB0_W { w: self }
    }
    #[doc = "Bits 4:5 - Edge B 1"]
    #[inline(always)]
    pub fn edgb1(&mut self) -> EDGB1_W {
        EDGB1_W { w: self }
    }
    #[doc = "Bit 6 - Input Select B"]
    #[inline(always)]
    pub fn inp_selb(&mut self) -> INP_SELB_W {
        INP_SELB_W { w: self }
    }
    #[doc = "Bit 7 - Edge Counter B Enable"]
    #[inline(always)]
    pub fn edgcntb_en(&mut self) -> EDGCNTB_EN_W {
        EDGCNTB_EN_W { w: self }
    }
    #[doc = "Bits 8:9 - Capture B FIFOs Water Mark"]
    #[inline(always)]
    pub fn cfbwm(&mut self) -> CFBWM_W {
        CFBWM_W { w: self }
    }
}
