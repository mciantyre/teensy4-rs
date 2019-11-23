#[doc = "Reader of register GPR5"]
pub type R = crate::R<u32, super::GPR5>;
#[doc = "Writer for register GPR5"]
pub type W = crate::W<u32, super::GPR5>;
#[doc = "Register GPR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WDOG1 Timeout Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG1_MASK_A {
    #[doc = "0: WDOG1 Timeout behaves normally"]
    WDOG1_MASK_0,
    #[doc = "1: WDOG1 Timeout is masked"]
    WDOG1_MASK_1,
}
impl From<WDOG1_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG1_MASK_A) -> Self {
        match variant {
            WDOG1_MASK_A::WDOG1_MASK_0 => false,
            WDOG1_MASK_A::WDOG1_MASK_1 => true,
        }
    }
}
#[doc = "Reader of field `WDOG1_MASK`"]
pub type WDOG1_MASK_R = crate::R<bool, WDOG1_MASK_A>;
impl WDOG1_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG1_MASK_A {
        match self.bits {
            false => WDOG1_MASK_A::WDOG1_MASK_0,
            true => WDOG1_MASK_A::WDOG1_MASK_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG1_MASK_0`"]
    #[inline(always)]
    pub fn is_wdog1_mask_0(&self) -> bool {
        *self == WDOG1_MASK_A::WDOG1_MASK_0
    }
    #[doc = "Checks if the value of the field is `WDOG1_MASK_1`"]
    #[inline(always)]
    pub fn is_wdog1_mask_1(&self) -> bool {
        *self == WDOG1_MASK_A::WDOG1_MASK_1
    }
}
#[doc = "Write proxy for field `WDOG1_MASK`"]
pub struct WDOG1_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG1_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG1_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "WDOG1 Timeout behaves normally"]
    #[inline(always)]
    pub fn wdog1_mask_0(self) -> &'a mut W {
        self.variant(WDOG1_MASK_A::WDOG1_MASK_0)
    }
    #[doc = "WDOG1 Timeout is masked"]
    #[inline(always)]
    pub fn wdog1_mask_1(self) -> &'a mut W {
        self.variant(WDOG1_MASK_A::WDOG1_MASK_1)
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
#[doc = "WDOG2 Timeout Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG2_MASK_A {
    #[doc = "0: WDOG2 Timeout behaves normally"]
    WDOG2_MASK_0,
    #[doc = "1: WDOG2 Timeout is masked"]
    WDOG2_MASK_1,
}
impl From<WDOG2_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG2_MASK_A) -> Self {
        match variant {
            WDOG2_MASK_A::WDOG2_MASK_0 => false,
            WDOG2_MASK_A::WDOG2_MASK_1 => true,
        }
    }
}
#[doc = "Reader of field `WDOG2_MASK`"]
pub type WDOG2_MASK_R = crate::R<bool, WDOG2_MASK_A>;
impl WDOG2_MASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG2_MASK_A {
        match self.bits {
            false => WDOG2_MASK_A::WDOG2_MASK_0,
            true => WDOG2_MASK_A::WDOG2_MASK_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG2_MASK_0`"]
    #[inline(always)]
    pub fn is_wdog2_mask_0(&self) -> bool {
        *self == WDOG2_MASK_A::WDOG2_MASK_0
    }
    #[doc = "Checks if the value of the field is `WDOG2_MASK_1`"]
    #[inline(always)]
    pub fn is_wdog2_mask_1(&self) -> bool {
        *self == WDOG2_MASK_A::WDOG2_MASK_1
    }
}
#[doc = "Write proxy for field `WDOG2_MASK`"]
pub struct WDOG2_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG2_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG2_MASK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "WDOG2 Timeout behaves normally"]
    #[inline(always)]
    pub fn wdog2_mask_0(self) -> &'a mut W {
        self.variant(WDOG2_MASK_A::WDOG2_MASK_0)
    }
    #[doc = "WDOG2 Timeout is masked"]
    #[inline(always)]
    pub fn wdog2_mask_1(self) -> &'a mut W {
        self.variant(WDOG2_MASK_A::WDOG2_MASK_1)
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
#[doc = "GPT2 input capture channel 1 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPT2_CAPIN1_SEL_A {
    #[doc = "0: source from GPT2_CAPTURE1"]
    GPT2_CAPIN1_SEL_0,
    #[doc = "1: source from ENET_1588_EVENT3_OUT (chnnal 3 of IEEE 1588 timer)"]
    GPT2_CAPIN1_SEL_1,
}
impl From<GPT2_CAPIN1_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: GPT2_CAPIN1_SEL_A) -> Self {
        match variant {
            GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_0 => false,
            GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `GPT2_CAPIN1_SEL`"]
pub type GPT2_CAPIN1_SEL_R = crate::R<bool, GPT2_CAPIN1_SEL_A>;
impl GPT2_CAPIN1_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPT2_CAPIN1_SEL_A {
        match self.bits {
            false => GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_0,
            true => GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPT2_CAPIN1_SEL_0`"]
    #[inline(always)]
    pub fn is_gpt2_capin1_sel_0(&self) -> bool {
        *self == GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_0
    }
    #[doc = "Checks if the value of the field is `GPT2_CAPIN1_SEL_1`"]
    #[inline(always)]
    pub fn is_gpt2_capin1_sel_1(&self) -> bool {
        *self == GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_1
    }
}
#[doc = "Write proxy for field `GPT2_CAPIN1_SEL`"]
pub struct GPT2_CAPIN1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT2_CAPIN1_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPT2_CAPIN1_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "source from GPT2_CAPTURE1"]
    #[inline(always)]
    pub fn gpt2_capin1_sel_0(self) -> &'a mut W {
        self.variant(GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_0)
    }
    #[doc = "source from ENET_1588_EVENT3_OUT (chnnal 3 of IEEE 1588 timer)"]
    #[inline(always)]
    pub fn gpt2_capin1_sel_1(self) -> &'a mut W {
        self.variant(GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_1)
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
#[doc = "GPT2 input capture channel 2 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPT2_CAPIN2_SEL_A {
    #[doc = "0: source from GPT2_CAPTURE2"]
    GPT2_CAPIN2_SEL_0,
    #[doc = "1: source from ENET2_1588_EVENT3_OUT (chnnal 3 of IEEE 1588 timer)"]
    GPT2_CAPIN2_SEL_1,
}
impl From<GPT2_CAPIN2_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: GPT2_CAPIN2_SEL_A) -> Self {
        match variant {
            GPT2_CAPIN2_SEL_A::GPT2_CAPIN2_SEL_0 => false,
            GPT2_CAPIN2_SEL_A::GPT2_CAPIN2_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `GPT2_CAPIN2_SEL`"]
pub type GPT2_CAPIN2_SEL_R = crate::R<bool, GPT2_CAPIN2_SEL_A>;
impl GPT2_CAPIN2_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPT2_CAPIN2_SEL_A {
        match self.bits {
            false => GPT2_CAPIN2_SEL_A::GPT2_CAPIN2_SEL_0,
            true => GPT2_CAPIN2_SEL_A::GPT2_CAPIN2_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPT2_CAPIN2_SEL_0`"]
    #[inline(always)]
    pub fn is_gpt2_capin2_sel_0(&self) -> bool {
        *self == GPT2_CAPIN2_SEL_A::GPT2_CAPIN2_SEL_0
    }
    #[doc = "Checks if the value of the field is `GPT2_CAPIN2_SEL_1`"]
    #[inline(always)]
    pub fn is_gpt2_capin2_sel_1(&self) -> bool {
        *self == GPT2_CAPIN2_SEL_A::GPT2_CAPIN2_SEL_1
    }
}
#[doc = "Write proxy for field `GPT2_CAPIN2_SEL`"]
pub struct GPT2_CAPIN2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPT2_CAPIN2_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPT2_CAPIN2_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "source from GPT2_CAPTURE2"]
    #[inline(always)]
    pub fn gpt2_capin2_sel_0(self) -> &'a mut W {
        self.variant(GPT2_CAPIN2_SEL_A::GPT2_CAPIN2_SEL_0)
    }
    #[doc = "source from ENET2_1588_EVENT3_OUT (chnnal 3 of IEEE 1588 timer)"]
    #[inline(always)]
    pub fn gpt2_capin2_sel_1(self) -> &'a mut W {
        self.variant(GPT2_CAPIN2_SEL_A::GPT2_CAPIN2_SEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "ENET input timer event3 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET_EVENT3IN_SEL_A {
    #[doc = "0: event3 source input from ENET_1588_EVENT3_IN"]
    ENET_EVENT3IN_SEL_0,
    #[doc = "1: event3 source input from GPT2.GPT_COMPARE1"]
    ENET_EVENT3IN_SEL_1,
}
impl From<ENET_EVENT3IN_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ENET_EVENT3IN_SEL_A) -> Self {
        match variant {
            ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_0 => false,
            ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `ENET_EVENT3IN_SEL`"]
pub type ENET_EVENT3IN_SEL_R = crate::R<bool, ENET_EVENT3IN_SEL_A>;
impl ENET_EVENT3IN_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET_EVENT3IN_SEL_A {
        match self.bits {
            false => ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_0,
            true => ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_EVENT3IN_SEL_0`"]
    #[inline(always)]
    pub fn is_enet_event3in_sel_0(&self) -> bool {
        *self == ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_0
    }
    #[doc = "Checks if the value of the field is `ENET_EVENT3IN_SEL_1`"]
    #[inline(always)]
    pub fn is_enet_event3in_sel_1(&self) -> bool {
        *self == ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_1
    }
}
#[doc = "Write proxy for field `ENET_EVENT3IN_SEL`"]
pub struct ENET_EVENT3IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET_EVENT3IN_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENET_EVENT3IN_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "event3 source input from ENET_1588_EVENT3_IN"]
    #[inline(always)]
    pub fn enet_event3in_sel_0(self) -> &'a mut W {
        self.variant(ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_0)
    }
    #[doc = "event3 source input from GPT2.GPT_COMPARE1"]
    #[inline(always)]
    pub fn enet_event3in_sel_1(self) -> &'a mut W {
        self.variant(ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "ENET2 input timer event3 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET2_EVENT3IN_SEL_A {
    #[doc = "0: event3 source input from ENET2_1588_EVENT3_IN"]
    ENET2_EVENT3IN_SEL_0,
    #[doc = "1: event3 source input from GPT2.GPT_COMPARE2"]
    ENET2_EVENT3IN_SEL_1,
}
impl From<ENET2_EVENT3IN_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ENET2_EVENT3IN_SEL_A) -> Self {
        match variant {
            ENET2_EVENT3IN_SEL_A::ENET2_EVENT3IN_SEL_0 => false,
            ENET2_EVENT3IN_SEL_A::ENET2_EVENT3IN_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `ENET2_EVENT3IN_SEL`"]
pub type ENET2_EVENT3IN_SEL_R = crate::R<bool, ENET2_EVENT3IN_SEL_A>;
impl ENET2_EVENT3IN_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET2_EVENT3IN_SEL_A {
        match self.bits {
            false => ENET2_EVENT3IN_SEL_A::ENET2_EVENT3IN_SEL_0,
            true => ENET2_EVENT3IN_SEL_A::ENET2_EVENT3IN_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET2_EVENT3IN_SEL_0`"]
    #[inline(always)]
    pub fn is_enet2_event3in_sel_0(&self) -> bool {
        *self == ENET2_EVENT3IN_SEL_A::ENET2_EVENT3IN_SEL_0
    }
    #[doc = "Checks if the value of the field is `ENET2_EVENT3IN_SEL_1`"]
    #[inline(always)]
    pub fn is_enet2_event3in_sel_1(&self) -> bool {
        *self == ENET2_EVENT3IN_SEL_A::ENET2_EVENT3IN_SEL_1
    }
}
#[doc = "Write proxy for field `ENET2_EVENT3IN_SEL`"]
pub struct ENET2_EVENT3IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET2_EVENT3IN_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENET2_EVENT3IN_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "event3 source input from ENET2_1588_EVENT3_IN"]
    #[inline(always)]
    pub fn enet2_event3in_sel_0(self) -> &'a mut W {
        self.variant(ENET2_EVENT3IN_SEL_A::ENET2_EVENT3IN_SEL_0)
    }
    #[doc = "event3 source input from GPT2.GPT_COMPARE2"]
    #[inline(always)]
    pub fn enet2_event3in_sel_1(self) -> &'a mut W {
        self.variant(ENET2_EVENT3IN_SEL_A::ENET2_EVENT3IN_SEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "GPT1 1 MHz clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREF_1M_CLK_GPT1_A {
    #[doc = "0: GPT1 ipg_clk_highfreq driven by IPG_PERCLK"]
    VREF_1M_CLK_GPT1_0,
    #[doc = "1: GPT1 ipg_clk_highfreq driven by anatop 1 MHz clock"]
    VREF_1M_CLK_GPT1_1,
}
impl From<VREF_1M_CLK_GPT1_A> for bool {
    #[inline(always)]
    fn from(variant: VREF_1M_CLK_GPT1_A) -> Self {
        match variant {
            VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_0 => false,
            VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_1 => true,
        }
    }
}
#[doc = "Reader of field `VREF_1M_CLK_GPT1`"]
pub type VREF_1M_CLK_GPT1_R = crate::R<bool, VREF_1M_CLK_GPT1_A>;
impl VREF_1M_CLK_GPT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREF_1M_CLK_GPT1_A {
        match self.bits {
            false => VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_0,
            true => VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `VREF_1M_CLK_GPT1_0`"]
    #[inline(always)]
    pub fn is_vref_1m_clk_gpt1_0(&self) -> bool {
        *self == VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_0
    }
    #[doc = "Checks if the value of the field is `VREF_1M_CLK_GPT1_1`"]
    #[inline(always)]
    pub fn is_vref_1m_clk_gpt1_1(&self) -> bool {
        *self == VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_1
    }
}
#[doc = "Write proxy for field `VREF_1M_CLK_GPT1`"]
pub struct VREF_1M_CLK_GPT1_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_1M_CLK_GPT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREF_1M_CLK_GPT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "GPT1 ipg_clk_highfreq driven by IPG_PERCLK"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt1_0(self) -> &'a mut W {
        self.variant(VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_0)
    }
    #[doc = "GPT1 ipg_clk_highfreq driven by anatop 1 MHz clock"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt1_1(self) -> &'a mut W {
        self.variant(VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "GPT2 1 MHz clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREF_1M_CLK_GPT2_A {
    #[doc = "0: GPT2 ipg_clk_highfreq driven by IPG_PERCLK"]
    VREF_1M_CLK_GPT2_0,
    #[doc = "1: GPT2 ipg_clk_highfreq driven by anatop 1 MHz clock"]
    VREF_1M_CLK_GPT2_1,
}
impl From<VREF_1M_CLK_GPT2_A> for bool {
    #[inline(always)]
    fn from(variant: VREF_1M_CLK_GPT2_A) -> Self {
        match variant {
            VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_0 => false,
            VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_1 => true,
        }
    }
}
#[doc = "Reader of field `VREF_1M_CLK_GPT2`"]
pub type VREF_1M_CLK_GPT2_R = crate::R<bool, VREF_1M_CLK_GPT2_A>;
impl VREF_1M_CLK_GPT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREF_1M_CLK_GPT2_A {
        match self.bits {
            false => VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_0,
            true => VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_1,
        }
    }
    #[doc = "Checks if the value of the field is `VREF_1M_CLK_GPT2_0`"]
    #[inline(always)]
    pub fn is_vref_1m_clk_gpt2_0(&self) -> bool {
        *self == VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_0
    }
    #[doc = "Checks if the value of the field is `VREF_1M_CLK_GPT2_1`"]
    #[inline(always)]
    pub fn is_vref_1m_clk_gpt2_1(&self) -> bool {
        *self == VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_1
    }
}
#[doc = "Write proxy for field `VREF_1M_CLK_GPT2`"]
pub struct VREF_1M_CLK_GPT2_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_1M_CLK_GPT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREF_1M_CLK_GPT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "GPT2 ipg_clk_highfreq driven by IPG_PERCLK"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt2_0(self) -> &'a mut W {
        self.variant(VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_0)
    }
    #[doc = "GPT2 ipg_clk_highfreq driven by anatop 1 MHz clock"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt2_1(self) -> &'a mut W {
        self.variant(VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - WDOG1 Timeout Mask"]
    #[inline(always)]
    pub fn wdog1_mask(&self) -> WDOG1_MASK_R {
        WDOG1_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - WDOG2 Timeout Mask"]
    #[inline(always)]
    pub fn wdog2_mask(&self) -> WDOG2_MASK_R {
        WDOG2_MASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPT2 input capture channel 1 source select"]
    #[inline(always)]
    pub fn gpt2_capin1_sel(&self) -> GPT2_CAPIN1_SEL_R {
        GPT2_CAPIN1_SEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GPT2 input capture channel 2 source select"]
    #[inline(always)]
    pub fn gpt2_capin2_sel(&self) -> GPT2_CAPIN2_SEL_R {
        GPT2_CAPIN2_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ENET input timer event3 source select"]
    #[inline(always)]
    pub fn enet_event3in_sel(&self) -> ENET_EVENT3IN_SEL_R {
        ENET_EVENT3IN_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ENET2 input timer event3 source select"]
    #[inline(always)]
    pub fn enet2_event3in_sel(&self) -> ENET2_EVENT3IN_SEL_R {
        ENET2_EVENT3IN_SEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - GPT1 1 MHz clock source select"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt1(&self) -> VREF_1M_CLK_GPT1_R {
        VREF_1M_CLK_GPT1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GPT2 1 MHz clock source select"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt2(&self) -> VREF_1M_CLK_GPT2_R {
        VREF_1M_CLK_GPT2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - WDOG1 Timeout Mask"]
    #[inline(always)]
    pub fn wdog1_mask(&mut self) -> WDOG1_MASK_W {
        WDOG1_MASK_W { w: self }
    }
    #[doc = "Bit 7 - WDOG2 Timeout Mask"]
    #[inline(always)]
    pub fn wdog2_mask(&mut self) -> WDOG2_MASK_W {
        WDOG2_MASK_W { w: self }
    }
    #[doc = "Bit 23 - GPT2 input capture channel 1 source select"]
    #[inline(always)]
    pub fn gpt2_capin1_sel(&mut self) -> GPT2_CAPIN1_SEL_W {
        GPT2_CAPIN1_SEL_W { w: self }
    }
    #[doc = "Bit 24 - GPT2 input capture channel 2 source select"]
    #[inline(always)]
    pub fn gpt2_capin2_sel(&mut self) -> GPT2_CAPIN2_SEL_W {
        GPT2_CAPIN2_SEL_W { w: self }
    }
    #[doc = "Bit 25 - ENET input timer event3 source select"]
    #[inline(always)]
    pub fn enet_event3in_sel(&mut self) -> ENET_EVENT3IN_SEL_W {
        ENET_EVENT3IN_SEL_W { w: self }
    }
    #[doc = "Bit 26 - ENET2 input timer event3 source select"]
    #[inline(always)]
    pub fn enet2_event3in_sel(&mut self) -> ENET2_EVENT3IN_SEL_W {
        ENET2_EVENT3IN_SEL_W { w: self }
    }
    #[doc = "Bit 28 - GPT1 1 MHz clock source select"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt1(&mut self) -> VREF_1M_CLK_GPT1_W {
        VREF_1M_CLK_GPT1_W { w: self }
    }
    #[doc = "Bit 29 - GPT2 1 MHz clock source select"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt2(&mut self) -> VREF_1M_CLK_GPT2_W {
        VREF_1M_CLK_GPT2_W { w: self }
    }
}
