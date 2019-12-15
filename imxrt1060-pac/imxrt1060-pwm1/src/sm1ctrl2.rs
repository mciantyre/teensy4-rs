#[doc = "Reader of register SM1CTRL2"]
pub type R = crate::R<u16, super::SM1CTRL2>;
#[doc = "Writer for register SM1CTRL2"]
pub type W = crate::W<u16, super::SM1CTRL2>;
#[doc = "Register SM1CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SM1CTRL2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_SEL_A {
    #[doc = "0: The IPBus clock is used as the clock for the local prescaler and counter."]
    CLK_SEL_0,
    #[doc = "1: EXT_CLK is used as the clock for the local prescaler and counter."]
    CLK_SEL_1,
    #[doc = "2: Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it will force the clock to logic 0."]
    CLK_SEL_2,
}
impl From<CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SEL_A) -> Self {
        match variant {
            CLK_SEL_A::CLK_SEL_0 => 0,
            CLK_SEL_A::CLK_SEL_1 => 1,
            CLK_SEL_A::CLK_SEL_2 => 2,
        }
    }
}
#[doc = "Reader of field `CLK_SEL`"]
pub type CLK_SEL_R = crate::R<u8, CLK_SEL_A>;
impl CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLK_SEL_A::CLK_SEL_0),
            1 => Val(CLK_SEL_A::CLK_SEL_1),
            2 => Val(CLK_SEL_A::CLK_SEL_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_clk_sel_0(&self) -> bool {
        *self == CLK_SEL_A::CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_clk_sel_1(&self) -> bool {
        *self == CLK_SEL_A::CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `CLK_SEL_2`"]
    #[inline(always)]
    pub fn is_clk_sel_2(&self) -> bool {
        *self == CLK_SEL_A::CLK_SEL_2
    }
}
#[doc = "Write proxy for field `CLK_SEL`"]
pub struct CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    #[inline(always)]
    pub fn clk_sel_0(self) -> &'a mut W {
        self.variant(CLK_SEL_A::CLK_SEL_0)
    }
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    #[inline(always)]
    pub fn clk_sel_1(self) -> &'a mut W {
        self.variant(CLK_SEL_A::CLK_SEL_1)
    }
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it will force the clock to logic 0."]
    #[inline(always)]
    pub fn clk_sel_2(self) -> &'a mut W {
        self.variant(CLK_SEL_A::CLK_SEL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Reload Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RELOAD_SEL_A {
    #[doc = "0: The local RELOAD signal is used to reload registers."]
    RELOAD_SEL_0,
    #[doc = "1: The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it will force the RELOAD signal to logic 0."]
    RELOAD_SEL_1,
}
impl From<RELOAD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_SEL_A) -> Self {
        match variant {
            RELOAD_SEL_A::RELOAD_SEL_0 => false,
            RELOAD_SEL_A::RELOAD_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `RELOAD_SEL`"]
pub type RELOAD_SEL_R = crate::R<bool, RELOAD_SEL_A>;
impl RELOAD_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_SEL_A {
        match self.bits {
            false => RELOAD_SEL_A::RELOAD_SEL_0,
            true => RELOAD_SEL_A::RELOAD_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RELOAD_SEL_0`"]
    #[inline(always)]
    pub fn is_reload_sel_0(&self) -> bool {
        *self == RELOAD_SEL_A::RELOAD_SEL_0
    }
    #[doc = "Checks if the value of the field is `RELOAD_SEL_1`"]
    #[inline(always)]
    pub fn is_reload_sel_1(&self) -> bool {
        *self == RELOAD_SEL_A::RELOAD_SEL_1
    }
}
#[doc = "Write proxy for field `RELOAD_SEL`"]
pub struct RELOAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RELOAD_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The local RELOAD signal is used to reload registers."]
    #[inline(always)]
    pub fn reload_sel_0(self) -> &'a mut W {
        self.variant(RELOAD_SEL_A::RELOAD_SEL_0)
    }
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it will force the RELOAD signal to logic 0."]
    #[inline(always)]
    pub fn reload_sel_1(self) -> &'a mut W {
        self.variant(RELOAD_SEL_A::RELOAD_SEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "This read/write bit determines the source of the FORCE OUTPUT signal for this submodule.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_SEL_A {
    #[doc = "0: The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
    FORCE_SEL_0,
    #[doc = "1: The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it will hold the FORCE OUTPUT signal to logic 0."]
    FORCE_SEL_1,
    #[doc = "2: The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    FORCE_SEL_2,
    #[doc = "3: The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    FORCE_SEL_3,
    #[doc = "4: The local sync signal from this submodule is used to force updates."]
    FORCE_SEL_4,
    #[doc = "5: The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    FORCE_SEL_5,
    #[doc = "6: The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    FORCE_SEL_6,
    #[doc = "7: The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    FORCE_SEL_7,
}
impl From<FORCE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FORCE_SEL_A) -> Self {
        match variant {
            FORCE_SEL_A::FORCE_SEL_0 => 0,
            FORCE_SEL_A::FORCE_SEL_1 => 1,
            FORCE_SEL_A::FORCE_SEL_2 => 2,
            FORCE_SEL_A::FORCE_SEL_3 => 3,
            FORCE_SEL_A::FORCE_SEL_4 => 4,
            FORCE_SEL_A::FORCE_SEL_5 => 5,
            FORCE_SEL_A::FORCE_SEL_6 => 6,
            FORCE_SEL_A::FORCE_SEL_7 => 7,
        }
    }
}
#[doc = "Reader of field `FORCE_SEL`"]
pub type FORCE_SEL_R = crate::R<u8, FORCE_SEL_A>;
impl FORCE_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_SEL_A {
        match self.bits {
            0 => FORCE_SEL_A::FORCE_SEL_0,
            1 => FORCE_SEL_A::FORCE_SEL_1,
            2 => FORCE_SEL_A::FORCE_SEL_2,
            3 => FORCE_SEL_A::FORCE_SEL_3,
            4 => FORCE_SEL_A::FORCE_SEL_4,
            5 => FORCE_SEL_A::FORCE_SEL_5,
            6 => FORCE_SEL_A::FORCE_SEL_6,
            7 => FORCE_SEL_A::FORCE_SEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_0`"]
    #[inline(always)]
    pub fn is_force_sel_0(&self) -> bool {
        *self == FORCE_SEL_A::FORCE_SEL_0
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_1`"]
    #[inline(always)]
    pub fn is_force_sel_1(&self) -> bool {
        *self == FORCE_SEL_A::FORCE_SEL_1
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_2`"]
    #[inline(always)]
    pub fn is_force_sel_2(&self) -> bool {
        *self == FORCE_SEL_A::FORCE_SEL_2
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_3`"]
    #[inline(always)]
    pub fn is_force_sel_3(&self) -> bool {
        *self == FORCE_SEL_A::FORCE_SEL_3
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_4`"]
    #[inline(always)]
    pub fn is_force_sel_4(&self) -> bool {
        *self == FORCE_SEL_A::FORCE_SEL_4
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_5`"]
    #[inline(always)]
    pub fn is_force_sel_5(&self) -> bool {
        *self == FORCE_SEL_A::FORCE_SEL_5
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_6`"]
    #[inline(always)]
    pub fn is_force_sel_6(&self) -> bool {
        *self == FORCE_SEL_A::FORCE_SEL_6
    }
    #[doc = "Checks if the value of the field is `FORCE_SEL_7`"]
    #[inline(always)]
    pub fn is_force_sel_7(&self) -> bool {
        *self == FORCE_SEL_A::FORCE_SEL_7
    }
}
#[doc = "Write proxy for field `FORCE_SEL`"]
pub struct FORCE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCE_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
    #[inline(always)]
    pub fn force_sel_0(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::FORCE_SEL_0)
    }
    #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it will hold the FORCE OUTPUT signal to logic 0."]
    #[inline(always)]
    pub fn force_sel_1(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::FORCE_SEL_1)
    }
    #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    #[inline(always)]
    pub fn force_sel_2(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::FORCE_SEL_2)
    }
    #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    #[inline(always)]
    pub fn force_sel_3(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::FORCE_SEL_3)
    }
    #[doc = "The local sync signal from this submodule is used to force updates."]
    #[inline(always)]
    pub fn force_sel_4(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::FORCE_SEL_4)
    }
    #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it will hold the FORCE OUTPUT signal to logic 0."]
    #[inline(always)]
    pub fn force_sel_5(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::FORCE_SEL_5)
    }
    #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    #[inline(always)]
    pub fn force_sel_6(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::FORCE_SEL_6)
    }
    #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    #[inline(always)]
    pub fn force_sel_7(self) -> &'a mut W {
        self.variant(FORCE_SEL_A::FORCE_SEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u16) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `FORCE`"]
pub type FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE`"]
pub struct FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_W<'a> {
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
#[doc = "FRCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRCEN_A {
    #[doc = "0: Initialization from a FORCE_OUT is disabled."]
    FRCEN_0,
    #[doc = "1: Initialization from a FORCE_OUT is enabled."]
    FRCEN_1,
}
impl From<FRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: FRCEN_A) -> Self {
        match variant {
            FRCEN_A::FRCEN_0 => false,
            FRCEN_A::FRCEN_1 => true,
        }
    }
}
#[doc = "Reader of field `FRCEN`"]
pub type FRCEN_R = crate::R<bool, FRCEN_A>;
impl FRCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRCEN_A {
        match self.bits {
            false => FRCEN_A::FRCEN_0,
            true => FRCEN_A::FRCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FRCEN_0`"]
    #[inline(always)]
    pub fn is_frcen_0(&self) -> bool {
        *self == FRCEN_A::FRCEN_0
    }
    #[doc = "Checks if the value of the field is `FRCEN_1`"]
    #[inline(always)]
    pub fn is_frcen_1(&self) -> bool {
        *self == FRCEN_A::FRCEN_1
    }
}
#[doc = "Write proxy for field `FRCEN`"]
pub struct FRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Initialization from a FORCE_OUT is disabled."]
    #[inline(always)]
    pub fn frcen_0(self) -> &'a mut W {
        self.variant(FRCEN_A::FRCEN_0)
    }
    #[doc = "Initialization from a FORCE_OUT is enabled."]
    #[inline(always)]
    pub fn frcen_1(self) -> &'a mut W {
        self.variant(FRCEN_A::FRCEN_1)
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
#[doc = "Initialization Control Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_SEL_A {
    #[doc = "0: Local sync (PWM_X) causes initialization."]
    INIT_SEL_0,
    #[doc = "1: Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0. The submodule counter will only reinitialize when a master reload occurs."]
    INIT_SEL_1,
    #[doc = "2: Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0."]
    INIT_SEL_2,
    #[doc = "3: EXT_SYNC causes initialization."]
    INIT_SEL_3,
}
impl From<INIT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INIT_SEL_A) -> Self {
        match variant {
            INIT_SEL_A::INIT_SEL_0 => 0,
            INIT_SEL_A::INIT_SEL_1 => 1,
            INIT_SEL_A::INIT_SEL_2 => 2,
            INIT_SEL_A::INIT_SEL_3 => 3,
        }
    }
}
#[doc = "Reader of field `INIT_SEL`"]
pub type INIT_SEL_R = crate::R<u8, INIT_SEL_A>;
impl INIT_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_SEL_A {
        match self.bits {
            0 => INIT_SEL_A::INIT_SEL_0,
            1 => INIT_SEL_A::INIT_SEL_1,
            2 => INIT_SEL_A::INIT_SEL_2,
            3 => INIT_SEL_A::INIT_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INIT_SEL_0`"]
    #[inline(always)]
    pub fn is_init_sel_0(&self) -> bool {
        *self == INIT_SEL_A::INIT_SEL_0
    }
    #[doc = "Checks if the value of the field is `INIT_SEL_1`"]
    #[inline(always)]
    pub fn is_init_sel_1(&self) -> bool {
        *self == INIT_SEL_A::INIT_SEL_1
    }
    #[doc = "Checks if the value of the field is `INIT_SEL_2`"]
    #[inline(always)]
    pub fn is_init_sel_2(&self) -> bool {
        *self == INIT_SEL_A::INIT_SEL_2
    }
    #[doc = "Checks if the value of the field is `INIT_SEL_3`"]
    #[inline(always)]
    pub fn is_init_sel_3(&self) -> bool {
        *self == INIT_SEL_A::INIT_SEL_3
    }
}
#[doc = "Write proxy for field `INIT_SEL`"]
pub struct INIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Local sync (PWM_X) causes initialization."]
    #[inline(always)]
    pub fn init_sel_0(self) -> &'a mut W {
        self.variant(INIT_SEL_A::INIT_SEL_0)
    }
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0. The submodule counter will only reinitialize when a master reload occurs."]
    #[inline(always)]
    pub fn init_sel_1(self) -> &'a mut W {
        self.variant(INIT_SEL_A::INIT_SEL_1)
    }
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it will force the INIT signal to logic 0."]
    #[inline(always)]
    pub fn init_sel_2(self) -> &'a mut W {
        self.variant(INIT_SEL_A::INIT_SEL_2)
    }
    #[doc = "EXT_SYNC causes initialization."]
    #[inline(always)]
    pub fn init_sel_3(self) -> &'a mut W {
        self.variant(INIT_SEL_A::INIT_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `PWMX_INIT`"]
pub type PWMX_INIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWMX_INIT`"]
pub struct PWMX_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMX_INIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `PWM45_INIT`"]
pub type PWM45_INIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM45_INIT`"]
pub struct PWM45_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM45_INIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PWM23_INIT`"]
pub type PWM23_INIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM23_INIT`"]
pub struct PWM23_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM23_INIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Independent or Complementary Pair Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INDEP_A {
    #[doc = "0: PWM_A and PWM_B form a complementary PWM pair."]
    INDEP_0,
    #[doc = "1: PWM_A and PWM_B outputs are independent PWMs."]
    INDEP_1,
}
impl From<INDEP_A> for bool {
    #[inline(always)]
    fn from(variant: INDEP_A) -> Self {
        match variant {
            INDEP_A::INDEP_0 => false,
            INDEP_A::INDEP_1 => true,
        }
    }
}
#[doc = "Reader of field `INDEP`"]
pub type INDEP_R = crate::R<bool, INDEP_A>;
impl INDEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INDEP_A {
        match self.bits {
            false => INDEP_A::INDEP_0,
            true => INDEP_A::INDEP_1,
        }
    }
    #[doc = "Checks if the value of the field is `INDEP_0`"]
    #[inline(always)]
    pub fn is_indep_0(&self) -> bool {
        *self == INDEP_A::INDEP_0
    }
    #[doc = "Checks if the value of the field is `INDEP_1`"]
    #[inline(always)]
    pub fn is_indep_1(&self) -> bool {
        *self == INDEP_A::INDEP_1
    }
}
#[doc = "Write proxy for field `INDEP`"]
pub struct INDEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INDEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INDEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    #[inline(always)]
    pub fn indep_0(self) -> &'a mut W {
        self.variant(INDEP_A::INDEP_0)
    }
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    #[inline(always)]
    pub fn indep_1(self) -> &'a mut W {
        self.variant(INDEP_A::INDEP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `WAITEN`"]
pub type WAITEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAITEN`"]
pub struct WAITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DBGEN`"]
pub type DBGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGEN`"]
pub struct DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Reload Source Select"]
    #[inline(always)]
    pub fn reload_sel(&self) -> RELOAD_SEL_R {
        RELOAD_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - This read/write bit determines the source of the FORCE OUTPUT signal for this submodule."]
    #[inline(always)]
    pub fn force_sel(&self) -> FORCE_SEL_R {
        FORCE_SEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - Force Initialization"]
    #[inline(always)]
    pub fn force(&self) -> FORCE_R {
        FORCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FRCEN"]
    #[inline(always)]
    pub fn frcen(&self) -> FRCEN_R {
        FRCEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Initialization Control Select"]
    #[inline(always)]
    pub fn init_sel(&self) -> INIT_SEL_R {
        INIT_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - PWM_X Initial Value"]
    #[inline(always)]
    pub fn pwmx_init(&self) -> PWMX_INIT_R {
        PWMX_INIT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PWM45 Initial Value"]
    #[inline(always)]
    pub fn pwm45_init(&self) -> PWM45_INIT_R {
        PWM45_INIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PWM23 Initial Value"]
    #[inline(always)]
    pub fn pwm23_init(&self) -> PWM23_INIT_R {
        PWM23_INIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub fn indep(&self) -> INDEP_R {
        INDEP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - WAIT Enable"]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_sel(&mut self) -> CLK_SEL_W {
        CLK_SEL_W { w: self }
    }
    #[doc = "Bit 2 - Reload Source Select"]
    #[inline(always)]
    pub fn reload_sel(&mut self) -> RELOAD_SEL_W {
        RELOAD_SEL_W { w: self }
    }
    #[doc = "Bits 3:5 - This read/write bit determines the source of the FORCE OUTPUT signal for this submodule."]
    #[inline(always)]
    pub fn force_sel(&mut self) -> FORCE_SEL_W {
        FORCE_SEL_W { w: self }
    }
    #[doc = "Bit 6 - Force Initialization"]
    #[inline(always)]
    pub fn force(&mut self) -> FORCE_W {
        FORCE_W { w: self }
    }
    #[doc = "Bit 7 - FRCEN"]
    #[inline(always)]
    pub fn frcen(&mut self) -> FRCEN_W {
        FRCEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Initialization Control Select"]
    #[inline(always)]
    pub fn init_sel(&mut self) -> INIT_SEL_W {
        INIT_SEL_W { w: self }
    }
    #[doc = "Bit 10 - PWM_X Initial Value"]
    #[inline(always)]
    pub fn pwmx_init(&mut self) -> PWMX_INIT_W {
        PWMX_INIT_W { w: self }
    }
    #[doc = "Bit 11 - PWM45 Initial Value"]
    #[inline(always)]
    pub fn pwm45_init(&mut self) -> PWM45_INIT_W {
        PWM45_INIT_W { w: self }
    }
    #[doc = "Bit 12 - PWM23 Initial Value"]
    #[inline(always)]
    pub fn pwm23_init(&mut self) -> PWM23_INIT_W {
        PWM23_INIT_W { w: self }
    }
    #[doc = "Bit 13 - Independent or Complementary Pair Operation"]
    #[inline(always)]
    pub fn indep(&mut self) -> INDEP_W {
        INDEP_W { w: self }
    }
    #[doc = "Bit 14 - WAIT Enable"]
    #[inline(always)]
    pub fn waiten(&mut self) -> WAITEN_W {
        WAITEN_W { w: self }
    }
    #[doc = "Bit 15 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
}
