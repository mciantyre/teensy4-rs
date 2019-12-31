#[doc = "Reader of register IR"]
pub type R = crate::R<u32, super::IR>;
#[doc = "Writer for register IR"]
pub type W = crate::W<u32, super::IR>;
#[doc = "Register IR `reset()`'s with value 0"]
impl crate::ResetValue for super::IR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OF1IE`"]
pub type OF1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OF1IE`"]
pub struct OF1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OF1IE_W<'a> {
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
#[doc = "Reader of field `OF2IE`"]
pub type OF2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OF2IE`"]
pub struct OF2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OF2IE_W<'a> {
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
#[doc = "OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OF3IE_A {
    #[doc = "0: Output Compare Channel n interrupt is disabled."]
    OF3IE_0 = 0,
    #[doc = "1: Output Compare Channel n interrupt is enabled."]
    OF3IE_1 = 1,
}
impl From<OF3IE_A> for bool {
    #[inline(always)]
    fn from(variant: OF3IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OF3IE`"]
pub type OF3IE_R = crate::R<bool, OF3IE_A>;
impl OF3IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OF3IE_A {
        match self.bits {
            false => OF3IE_A::OF3IE_0,
            true => OF3IE_A::OF3IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `OF3IE_0`"]
    #[inline(always)]
    pub fn is_of3ie_0(&self) -> bool {
        *self == OF3IE_A::OF3IE_0
    }
    #[doc = "Checks if the value of the field is `OF3IE_1`"]
    #[inline(always)]
    pub fn is_of3ie_1(&self) -> bool {
        *self == OF3IE_A::OF3IE_1
    }
}
#[doc = "Write proxy for field `OF3IE`"]
pub struct OF3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OF3IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OF3IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Output Compare Channel n interrupt is disabled."]
    #[inline(always)]
    pub fn of3ie_0(self) -> &'a mut W {
        self.variant(OF3IE_A::OF3IE_0)
    }
    #[doc = "Output Compare Channel n interrupt is enabled."]
    #[inline(always)]
    pub fn of3ie_1(self) -> &'a mut W {
        self.variant(OF3IE_A::OF3IE_1)
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
#[doc = "Reader of field `IF1IE`"]
pub type IF1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF1IE`"]
pub struct IF1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IF1IE_W<'a> {
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
#[doc = "IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF2IE_A {
    #[doc = "0: IF2IE Input Capture n Interrupt Enable is disabled."]
    IF2IE_0 = 0,
    #[doc = "1: IF2IE Input Capture n Interrupt Enable is enabled."]
    IF2IE_1 = 1,
}
impl From<IF2IE_A> for bool {
    #[inline(always)]
    fn from(variant: IF2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IF2IE`"]
pub type IF2IE_R = crate::R<bool, IF2IE_A>;
impl IF2IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF2IE_A {
        match self.bits {
            false => IF2IE_A::IF2IE_0,
            true => IF2IE_A::IF2IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IF2IE_0`"]
    #[inline(always)]
    pub fn is_if2ie_0(&self) -> bool {
        *self == IF2IE_A::IF2IE_0
    }
    #[doc = "Checks if the value of the field is `IF2IE_1`"]
    #[inline(always)]
    pub fn is_if2ie_1(&self) -> bool {
        *self == IF2IE_A::IF2IE_1
    }
}
#[doc = "Write proxy for field `IF2IE`"]
pub struct IF2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IF2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IF2IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IF2IE Input Capture n Interrupt Enable is disabled."]
    #[inline(always)]
    pub fn if2ie_0(self) -> &'a mut W {
        self.variant(IF2IE_A::IF2IE_0)
    }
    #[doc = "IF2IE Input Capture n Interrupt Enable is enabled."]
    #[inline(always)]
    pub fn if2ie_1(self) -> &'a mut W {
        self.variant(IF2IE_A::IF2IE_1)
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
#[doc = "Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROVIE_A {
    #[doc = "0: Rollover interrupt is disabled."]
    ROVIE_0 = 0,
    #[doc = "1: Rollover interrupt enabled."]
    ROVIE_1 = 1,
}
impl From<ROVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ROVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROVIE`"]
pub type ROVIE_R = crate::R<bool, ROVIE_A>;
impl ROVIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVIE_A {
        match self.bits {
            false => ROVIE_A::ROVIE_0,
            true => ROVIE_A::ROVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROVIE_0`"]
    #[inline(always)]
    pub fn is_rovie_0(&self) -> bool {
        *self == ROVIE_A::ROVIE_0
    }
    #[doc = "Checks if the value of the field is `ROVIE_1`"]
    #[inline(always)]
    pub fn is_rovie_1(&self) -> bool {
        *self == ROVIE_A::ROVIE_1
    }
}
#[doc = "Write proxy for field `ROVIE`"]
pub struct ROVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROVIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rollover interrupt is disabled."]
    #[inline(always)]
    pub fn rovie_0(self) -> &'a mut W {
        self.variant(ROVIE_A::ROVIE_0)
    }
    #[doc = "Rollover interrupt enabled."]
    #[inline(always)]
    pub fn rovie_1(self) -> &'a mut W {
        self.variant(ROVIE_A::ROVIE_1)
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
impl R {
    #[doc = "Bit 0 - See OF3IE"]
    #[inline(always)]
    pub fn of1ie(&self) -> OF1IE_R {
        OF1IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - See OF3IE"]
    #[inline(always)]
    pub fn of2ie(&self) -> OF2IE_R {
        OF2IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
    #[inline(always)]
    pub fn of3ie(&self) -> OF3IE_R {
        OF3IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - See IF2IE"]
    #[inline(always)]
    pub fn if1ie(&self) -> IF1IE_R {
        IF1IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
    #[inline(always)]
    pub fn if2ie(&self) -> IF2IE_R {
        IF2IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
    #[inline(always)]
    pub fn rovie(&self) -> ROVIE_R {
        ROVIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See OF3IE"]
    #[inline(always)]
    pub fn of1ie(&mut self) -> OF1IE_W {
        OF1IE_W { w: self }
    }
    #[doc = "Bit 1 - See OF3IE"]
    #[inline(always)]
    pub fn of2ie(&mut self) -> OF2IE_W {
        OF2IE_W { w: self }
    }
    #[doc = "Bit 2 - OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
    #[inline(always)]
    pub fn of3ie(&mut self) -> OF3IE_W {
        OF3IE_W { w: self }
    }
    #[doc = "Bit 3 - See IF2IE"]
    #[inline(always)]
    pub fn if1ie(&mut self) -> IF1IE_W {
        IF1IE_W { w: self }
    }
    #[doc = "Bit 4 - IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
    #[inline(always)]
    pub fn if2ie(&mut self) -> IF2IE_W {
        IF2IE_W { w: self }
    }
    #[doc = "Bit 5 - Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
    #[inline(always)]
    pub fn rovie(&mut self) -> ROVIE_W {
        ROVIE_W { w: self }
    }
}
