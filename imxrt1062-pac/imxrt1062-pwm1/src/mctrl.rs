#[doc = "Reader of register MCTRL"]
pub type R = crate::R<u16, super::MCTRL>;
#[doc = "Writer for register MCTRL"]
pub type W = crate::W<u16, super::MCTRL>;
#[doc = "Register MCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MCTRL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Load Okay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LDOK_A {
    #[doc = "0: Do not load new values."]
    LDOK_0 = 0,
    #[doc = "1: Load prescaler, modulus, and PWM values of the corresponding submodule."]
    LDOK_1 = 1,
}
impl From<LDOK_A> for u8 {
    #[inline(always)]
    fn from(variant: LDOK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LDOK`"]
pub type LDOK_R = crate::R<u8, LDOK_A>;
impl LDOK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LDOK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LDOK_A::LDOK_0),
            1 => Val(LDOK_A::LDOK_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LDOK_0`"]
    #[inline(always)]
    pub fn is_ldok_0(&self) -> bool {
        *self == LDOK_A::LDOK_0
    }
    #[doc = "Checks if the value of the field is `LDOK_1`"]
    #[inline(always)]
    pub fn is_ldok_1(&self) -> bool {
        *self == LDOK_A::LDOK_1
    }
}
#[doc = "Write proxy for field `LDOK`"]
pub struct LDOK_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDOK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Do not load new values."]
    #[inline(always)]
    pub fn ldok_0(self) -> &'a mut W {
        self.variant(LDOK_A::LDOK_0)
    }
    #[doc = "Load prescaler, modulus, and PWM values of the corresponding submodule."]
    #[inline(always)]
    pub fn ldok_1(self) -> &'a mut W {
        self.variant(LDOK_A::LDOK_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CLDOK`"]
pub type CLDOK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLDOK`"]
pub struct CLDOK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLDOK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RUN_A {
    #[doc = "0: PWM generator is disabled in the corresponding submodule."]
    RUN_0 = 0,
    #[doc = "1: PWM generator is enabled in the corresponding submodule."]
    RUN_1 = 1,
}
impl From<RUN_A> for u8 {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RUN`"]
pub type RUN_R = crate::R<u8, RUN_A>;
impl RUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RUN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RUN_A::RUN_0),
            1 => Val(RUN_A::RUN_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RUN_0`"]
    #[inline(always)]
    pub fn is_run_0(&self) -> bool {
        *self == RUN_A::RUN_0
    }
    #[doc = "Checks if the value of the field is `RUN_1`"]
    #[inline(always)]
    pub fn is_run_1(&self) -> bool {
        *self == RUN_A::RUN_1
    }
}
#[doc = "Write proxy for field `RUN`"]
pub struct RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RUN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM generator is disabled in the corresponding submodule."]
    #[inline(always)]
    pub fn run_0(self) -> &'a mut W {
        self.variant(RUN_A::RUN_0)
    }
    #[doc = "PWM generator is enabled in the corresponding submodule."]
    #[inline(always)]
    pub fn run_1(self) -> &'a mut W {
        self.variant(RUN_A::RUN_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Current Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IPOL_A {
    #[doc = "0: PWM23 is used to generate complementary PWM pair in the corresponding submodule."]
    IPOL_0 = 0,
    #[doc = "1: PWM45 is used to generate complementary PWM pair in the corresponding submodule."]
    IPOL_1 = 1,
}
impl From<IPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: IPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IPOL`"]
pub type IPOL_R = crate::R<u8, IPOL_A>;
impl IPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IPOL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IPOL_A::IPOL_0),
            1 => Val(IPOL_A::IPOL_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IPOL_0`"]
    #[inline(always)]
    pub fn is_ipol_0(&self) -> bool {
        *self == IPOL_A::IPOL_0
    }
    #[doc = "Checks if the value of the field is `IPOL_1`"]
    #[inline(always)]
    pub fn is_ipol_1(&self) -> bool {
        *self == IPOL_A::IPOL_1
    }
}
#[doc = "Write proxy for field `IPOL`"]
pub struct IPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> IPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPOL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM23 is used to generate complementary PWM pair in the corresponding submodule."]
    #[inline(always)]
    pub fn ipol_0(self) -> &'a mut W {
        self.variant(IPOL_A::IPOL_0)
    }
    #[doc = "PWM45 is used to generate complementary PWM pair in the corresponding submodule."]
    #[inline(always)]
    pub fn ipol_1(self) -> &'a mut W {
        self.variant(IPOL_A::IPOL_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Load Okay"]
    #[inline(always)]
    pub fn ldok(&self) -> LDOK_R {
        LDOK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Clear Load Okay"]
    #[inline(always)]
    pub fn cldok(&self) -> CLDOK_R {
        CLDOK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Run"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Current Polarity"]
    #[inline(always)]
    pub fn ipol(&self) -> IPOL_R {
        IPOL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Load Okay"]
    #[inline(always)]
    pub fn ldok(&mut self) -> LDOK_W {
        LDOK_W { w: self }
    }
    #[doc = "Bits 4:7 - Clear Load Okay"]
    #[inline(always)]
    pub fn cldok(&mut self) -> CLDOK_W {
        CLDOK_W { w: self }
    }
    #[doc = "Bits 8:11 - Run"]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W {
        RUN_W { w: self }
    }
    #[doc = "Bits 12:15 - Current Polarity"]
    #[inline(always)]
    pub fn ipol(&mut self) -> IPOL_W {
        IPOL_W { w: self }
    }
}
