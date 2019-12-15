#[doc = "Reader of register SM3CAPTCTRLA"]
pub type R = crate::R<u16, super::SM3CAPTCTRLA>;
#[doc = "Writer for register SM3CAPTCTRLA"]
pub type W = crate::W<u16, super::SM3CAPTCTRLA>;
#[doc = "Register SM3CAPTCTRLA `reset()`'s with value 0"]
impl crate::ResetValue for super::SM3CAPTCTRLA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Arm A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARMA_A {
    #[doc = "0: Input capture operation is disabled."]
    ARMA_0,
    #[doc = "1: Input capture operation as specified by CAPTCTRLA\\[EDGAx\\] is enabled."]
    ARMA_1,
}
impl From<ARMA_A> for bool {
    #[inline(always)]
    fn from(variant: ARMA_A) -> Self {
        match variant {
            ARMA_A::ARMA_0 => false,
            ARMA_A::ARMA_1 => true,
        }
    }
}
#[doc = "Reader of field `ARMA`"]
pub type ARMA_R = crate::R<bool, ARMA_A>;
impl ARMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARMA_A {
        match self.bits {
            false => ARMA_A::ARMA_0,
            true => ARMA_A::ARMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARMA_0`"]
    #[inline(always)]
    pub fn is_arma_0(&self) -> bool {
        *self == ARMA_A::ARMA_0
    }
    #[doc = "Checks if the value of the field is `ARMA_1`"]
    #[inline(always)]
    pub fn is_arma_1(&self) -> bool {
        *self == ARMA_A::ARMA_1
    }
}
#[doc = "Write proxy for field `ARMA`"]
pub struct ARMA_W<'a> {
    w: &'a mut W,
}
impl<'a> ARMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARMA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input capture operation is disabled."]
    #[inline(always)]
    pub fn arma_0(self) -> &'a mut W {
        self.variant(ARMA_A::ARMA_0)
    }
    #[doc = "Input capture operation as specified by CAPTCTRLA\\[EDGAx\\] is enabled."]
    #[inline(always)]
    pub fn arma_1(self) -> &'a mut W {
        self.variant(ARMA_A::ARMA_1)
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
#[doc = "One Shot Mode A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONESHOTA_A {
    #[doc = "0: Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLA\\[ARMA\\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit."]
    ONESHOTA_0,
    #[doc = "1: One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLA\\[ARMA\\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and CAPTCTRLA\\[ARMA\\] is cleared. No further captures will be performed until CAPTCTRLA\\[ARMA\\] is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and CAPTCTRLA\\[ARMA\\] is then cleared."]
    ONESHOTA_1,
}
impl From<ONESHOTA_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOTA_A) -> Self {
        match variant {
            ONESHOTA_A::ONESHOTA_0 => false,
            ONESHOTA_A::ONESHOTA_1 => true,
        }
    }
}
#[doc = "Reader of field `ONESHOTA`"]
pub type ONESHOTA_R = crate::R<bool, ONESHOTA_A>;
impl ONESHOTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOTA_A {
        match self.bits {
            false => ONESHOTA_A::ONESHOTA_0,
            true => ONESHOTA_A::ONESHOTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONESHOTA_0`"]
    #[inline(always)]
    pub fn is_oneshota_0(&self) -> bool {
        *self == ONESHOTA_A::ONESHOTA_0
    }
    #[doc = "Checks if the value of the field is `ONESHOTA_1`"]
    #[inline(always)]
    pub fn is_oneshota_1(&self) -> bool {
        *self == ONESHOTA_A::ONESHOTA_1
    }
}
#[doc = "Write proxy for field `ONESHOTA`"]
pub struct ONESHOTA_W<'a> {
    w: &'a mut W,
}
impl<'a> ONESHOTA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONESHOTA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Free running mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLA\\[ARMA\\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and capture circuit 0 is re-armed. The process continues indefinitely.If only one of the capture circuits is enabled, then captures continue indefinitely on the enabled capture circuit."]
    #[inline(always)]
    pub fn oneshota_0(self) -> &'a mut W {
        self.variant(ONESHOTA_A::ONESHOTA_0)
    }
    #[doc = "One shot mode is selected. If both capture circuits are enabled, then capture circuit 0 is armed first after CAPTCTRLA\\[ARMA\\] is set. Once a capture occurs, capture circuit 0 is disarmed and capture circuit 1 is armed. After capture circuit 1 performs a capture, it is disarmed and CAPTCTRLA\\[ARMA\\] is cleared. No further captures will be performed until CAPTCTRLA\\[ARMA\\] is set again.If only one of the capture circuits is enabled, then a single capture will occur on the enabled capture circuit and CAPTCTRLA\\[ARMA\\] is then cleared."]
    #[inline(always)]
    pub fn oneshota_1(self) -> &'a mut W {
        self.variant(ONESHOTA_A::ONESHOTA_1)
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
#[doc = "Edge A 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGA0_A {
    #[doc = "0: Disabled"]
    EDGA0_0,
    #[doc = "1: Capture falling edges"]
    EDGA0_1,
    #[doc = "2: Capture rising edges"]
    EDGA0_2,
    #[doc = "3: Capture any edge"]
    EDGA0_3,
}
impl From<EDGA0_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGA0_A) -> Self {
        match variant {
            EDGA0_A::EDGA0_0 => 0,
            EDGA0_A::EDGA0_1 => 1,
            EDGA0_A::EDGA0_2 => 2,
            EDGA0_A::EDGA0_3 => 3,
        }
    }
}
#[doc = "Reader of field `EDGA0`"]
pub type EDGA0_R = crate::R<u8, EDGA0_A>;
impl EDGA0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGA0_A {
        match self.bits {
            0 => EDGA0_A::EDGA0_0,
            1 => EDGA0_A::EDGA0_1,
            2 => EDGA0_A::EDGA0_2,
            3 => EDGA0_A::EDGA0_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGA0_0`"]
    #[inline(always)]
    pub fn is_edga0_0(&self) -> bool {
        *self == EDGA0_A::EDGA0_0
    }
    #[doc = "Checks if the value of the field is `EDGA0_1`"]
    #[inline(always)]
    pub fn is_edga0_1(&self) -> bool {
        *self == EDGA0_A::EDGA0_1
    }
    #[doc = "Checks if the value of the field is `EDGA0_2`"]
    #[inline(always)]
    pub fn is_edga0_2(&self) -> bool {
        *self == EDGA0_A::EDGA0_2
    }
    #[doc = "Checks if the value of the field is `EDGA0_3`"]
    #[inline(always)]
    pub fn is_edga0_3(&self) -> bool {
        *self == EDGA0_A::EDGA0_3
    }
}
#[doc = "Write proxy for field `EDGA0`"]
pub struct EDGA0_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGA0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn edga0_0(self) -> &'a mut W {
        self.variant(EDGA0_A::EDGA0_0)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn edga0_1(self) -> &'a mut W {
        self.variant(EDGA0_A::EDGA0_1)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn edga0_2(self) -> &'a mut W {
        self.variant(EDGA0_A::EDGA0_2)
    }
    #[doc = "Capture any edge"]
    #[inline(always)]
    pub fn edga0_3(self) -> &'a mut W {
        self.variant(EDGA0_A::EDGA0_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Edge A 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGA1_A {
    #[doc = "0: Disabled"]
    EDGA1_0,
    #[doc = "1: Capture falling edges"]
    EDGA1_1,
    #[doc = "2: Capture rising edges"]
    EDGA1_2,
    #[doc = "3: Capture any edge"]
    EDGA1_3,
}
impl From<EDGA1_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGA1_A) -> Self {
        match variant {
            EDGA1_A::EDGA1_0 => 0,
            EDGA1_A::EDGA1_1 => 1,
            EDGA1_A::EDGA1_2 => 2,
            EDGA1_A::EDGA1_3 => 3,
        }
    }
}
#[doc = "Reader of field `EDGA1`"]
pub type EDGA1_R = crate::R<u8, EDGA1_A>;
impl EDGA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGA1_A {
        match self.bits {
            0 => EDGA1_A::EDGA1_0,
            1 => EDGA1_A::EDGA1_1,
            2 => EDGA1_A::EDGA1_2,
            3 => EDGA1_A::EDGA1_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGA1_0`"]
    #[inline(always)]
    pub fn is_edga1_0(&self) -> bool {
        *self == EDGA1_A::EDGA1_0
    }
    #[doc = "Checks if the value of the field is `EDGA1_1`"]
    #[inline(always)]
    pub fn is_edga1_1(&self) -> bool {
        *self == EDGA1_A::EDGA1_1
    }
    #[doc = "Checks if the value of the field is `EDGA1_2`"]
    #[inline(always)]
    pub fn is_edga1_2(&self) -> bool {
        *self == EDGA1_A::EDGA1_2
    }
    #[doc = "Checks if the value of the field is `EDGA1_3`"]
    #[inline(always)]
    pub fn is_edga1_3(&self) -> bool {
        *self == EDGA1_A::EDGA1_3
    }
}
#[doc = "Write proxy for field `EDGA1`"]
pub struct EDGA1_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGA1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn edga1_0(self) -> &'a mut W {
        self.variant(EDGA1_A::EDGA1_0)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn edga1_1(self) -> &'a mut W {
        self.variant(EDGA1_A::EDGA1_1)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn edga1_2(self) -> &'a mut W {
        self.variant(EDGA1_A::EDGA1_2)
    }
    #[doc = "Capture any edge"]
    #[inline(always)]
    pub fn edga1_3(self) -> &'a mut W {
        self.variant(EDGA1_A::EDGA1_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u16) & 0x03) << 4);
        self.w
    }
}
#[doc = "Input Select A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INP_SELA_A {
    #[doc = "0: Raw PWM_A input signal selected as source."]
    INP_SELA_0,
    #[doc = "1: Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLA\\[EDGA0\\] and CAPTCTRLA\\[EDGA1\\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRA\\[EDGA0\\] and/or CAPTCTRLA\\[EDGA1\\] fields in order to enable one or both of the capture registers."]
    INP_SELA_1,
}
impl From<INP_SELA_A> for bool {
    #[inline(always)]
    fn from(variant: INP_SELA_A) -> Self {
        match variant {
            INP_SELA_A::INP_SELA_0 => false,
            INP_SELA_A::INP_SELA_1 => true,
        }
    }
}
#[doc = "Reader of field `INP_SELA`"]
pub type INP_SELA_R = crate::R<bool, INP_SELA_A>;
impl INP_SELA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INP_SELA_A {
        match self.bits {
            false => INP_SELA_A::INP_SELA_0,
            true => INP_SELA_A::INP_SELA_1,
        }
    }
    #[doc = "Checks if the value of the field is `INP_SELA_0`"]
    #[inline(always)]
    pub fn is_inp_sela_0(&self) -> bool {
        *self == INP_SELA_A::INP_SELA_0
    }
    #[doc = "Checks if the value of the field is `INP_SELA_1`"]
    #[inline(always)]
    pub fn is_inp_sela_1(&self) -> bool {
        *self == INP_SELA_A::INP_SELA_1
    }
}
#[doc = "Write proxy for field `INP_SELA`"]
pub struct INP_SELA_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_SELA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INP_SELA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Raw PWM_A input signal selected as source."]
    #[inline(always)]
    pub fn inp_sela_0(self) -> &'a mut W {
        self.variant(INP_SELA_A::INP_SELA_0)
    }
    #[doc = "Output of edge counter/compare selected as source. Note that when this bitfield is set to 1, the internal edge counter is enabled and the rising and/or falling edges specified by the CAPTCTRLA\\[EDGA0\\] and CAPTCTRLA\\[EDGA1\\] fields are ignored. The software must still place a value other than 00 in either or both of the CAPTCTLRA\\[EDGA0\\] and/or CAPTCTRLA\\[EDGA1\\] fields in order to enable one or both of the capture registers."]
    #[inline(always)]
    pub fn inp_sela_1(self) -> &'a mut W {
        self.variant(INP_SELA_A::INP_SELA_1)
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
#[doc = "Edge Counter A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGCNTA_EN_A {
    #[doc = "0: Edge counter disabled and held in reset"]
    EDGCNTA_EN_0,
    #[doc = "1: Edge counter enabled"]
    EDGCNTA_EN_1,
}
impl From<EDGCNTA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EDGCNTA_EN_A) -> Self {
        match variant {
            EDGCNTA_EN_A::EDGCNTA_EN_0 => false,
            EDGCNTA_EN_A::EDGCNTA_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `EDGCNTA_EN`"]
pub type EDGCNTA_EN_R = crate::R<bool, EDGCNTA_EN_A>;
impl EDGCNTA_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGCNTA_EN_A {
        match self.bits {
            false => EDGCNTA_EN_A::EDGCNTA_EN_0,
            true => EDGCNTA_EN_A::EDGCNTA_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EDGCNTA_EN_0`"]
    #[inline(always)]
    pub fn is_edgcnta_en_0(&self) -> bool {
        *self == EDGCNTA_EN_A::EDGCNTA_EN_0
    }
    #[doc = "Checks if the value of the field is `EDGCNTA_EN_1`"]
    #[inline(always)]
    pub fn is_edgcnta_en_1(&self) -> bool {
        *self == EDGCNTA_EN_A::EDGCNTA_EN_1
    }
}
#[doc = "Write proxy for field `EDGCNTA_EN`"]
pub struct EDGCNTA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGCNTA_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGCNTA_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Edge counter disabled and held in reset"]
    #[inline(always)]
    pub fn edgcnta_en_0(self) -> &'a mut W {
        self.variant(EDGCNTA_EN_A::EDGCNTA_EN_0)
    }
    #[doc = "Edge counter enabled"]
    #[inline(always)]
    pub fn edgcnta_en_1(self) -> &'a mut W {
        self.variant(EDGCNTA_EN_A::EDGCNTA_EN_1)
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
#[doc = "Reader of field `CFAWM`"]
pub type CFAWM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFAWM`"]
pub struct CFAWM_W<'a> {
    w: &'a mut W,
}
impl<'a> CFAWM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CA0CNT`"]
pub type CA0CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `CA1CNT`"]
pub type CA1CNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Arm A"]
    #[inline(always)]
    pub fn arma(&self) -> ARMA_R {
        ARMA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - One Shot Mode A"]
    #[inline(always)]
    pub fn oneshota(&self) -> ONESHOTA_R {
        ONESHOTA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Edge A 0"]
    #[inline(always)]
    pub fn edga0(&self) -> EDGA0_R {
        EDGA0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Edge A 1"]
    #[inline(always)]
    pub fn edga1(&self) -> EDGA1_R {
        EDGA1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Input Select A"]
    #[inline(always)]
    pub fn inp_sela(&self) -> INP_SELA_R {
        INP_SELA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Edge Counter A Enable"]
    #[inline(always)]
    pub fn edgcnta_en(&self) -> EDGCNTA_EN_R {
        EDGCNTA_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub fn cfawm(&self) -> CFAWM_R {
        CFAWM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12 - Capture A0 FIFO Word Count"]
    #[inline(always)]
    pub fn ca0cnt(&self) -> CA0CNT_R {
        CA0CNT_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 13:15 - Capture A1 FIFO Word Count"]
    #[inline(always)]
    pub fn ca1cnt(&self) -> CA1CNT_R {
        CA1CNT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Arm A"]
    #[inline(always)]
    pub fn arma(&mut self) -> ARMA_W {
        ARMA_W { w: self }
    }
    #[doc = "Bit 1 - One Shot Mode A"]
    #[inline(always)]
    pub fn oneshota(&mut self) -> ONESHOTA_W {
        ONESHOTA_W { w: self }
    }
    #[doc = "Bits 2:3 - Edge A 0"]
    #[inline(always)]
    pub fn edga0(&mut self) -> EDGA0_W {
        EDGA0_W { w: self }
    }
    #[doc = "Bits 4:5 - Edge A 1"]
    #[inline(always)]
    pub fn edga1(&mut self) -> EDGA1_W {
        EDGA1_W { w: self }
    }
    #[doc = "Bit 6 - Input Select A"]
    #[inline(always)]
    pub fn inp_sela(&mut self) -> INP_SELA_W {
        INP_SELA_W { w: self }
    }
    #[doc = "Bit 7 - Edge Counter A Enable"]
    #[inline(always)]
    pub fn edgcnta_en(&mut self) -> EDGCNTA_EN_W {
        EDGCNTA_EN_W { w: self }
    }
    #[doc = "Bits 8:9 - Capture A FIFOs Water Mark"]
    #[inline(always)]
    pub fn cfawm(&mut self) -> CFAWM_W {
        CFAWM_W { w: self }
    }
}
