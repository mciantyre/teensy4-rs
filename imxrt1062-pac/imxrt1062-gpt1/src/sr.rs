#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OF1`"]
pub type OF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OF1`"]
pub struct OF1_W<'a> {
    w: &'a mut W,
}
impl<'a> OF1_W<'a> {
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
#[doc = "Reader of field `OF2`"]
pub type OF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OF2`"]
pub struct OF2_W<'a> {
    w: &'a mut W,
}
impl<'a> OF2_W<'a> {
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
#[doc = "OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OF3_A {
    #[doc = "0: Compare event has not occurred."]
    OF3_0,
    #[doc = "1: Compare event has occurred."]
    OF3_1,
}
impl From<OF3_A> for bool {
    #[inline(always)]
    fn from(variant: OF3_A) -> Self {
        match variant {
            OF3_A::OF3_0 => false,
            OF3_A::OF3_1 => true,
        }
    }
}
#[doc = "Reader of field `OF3`"]
pub type OF3_R = crate::R<bool, OF3_A>;
impl OF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OF3_A {
        match self.bits {
            false => OF3_A::OF3_0,
            true => OF3_A::OF3_1,
        }
    }
    #[doc = "Checks if the value of the field is `OF3_0`"]
    #[inline(always)]
    pub fn is_of3_0(&self) -> bool {
        *self == OF3_A::OF3_0
    }
    #[doc = "Checks if the value of the field is `OF3_1`"]
    #[inline(always)]
    pub fn is_of3_1(&self) -> bool {
        *self == OF3_A::OF3_1
    }
}
#[doc = "Write proxy for field `OF3`"]
pub struct OF3_W<'a> {
    w: &'a mut W,
}
impl<'a> OF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OF3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare event has not occurred."]
    #[inline(always)]
    pub fn of3_0(self) -> &'a mut W {
        self.variant(OF3_A::OF3_0)
    }
    #[doc = "Compare event has occurred."]
    #[inline(always)]
    pub fn of3_1(self) -> &'a mut W {
        self.variant(OF3_A::OF3_1)
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
#[doc = "Reader of field `IF1`"]
pub type IF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF1`"]
pub struct IF1_W<'a> {
    w: &'a mut W,
}
impl<'a> IF1_W<'a> {
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
#[doc = "IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF2_A {
    #[doc = "0: Capture event has not occurred."]
    IF2_0,
    #[doc = "1: Capture event has occurred."]
    IF2_1,
}
impl From<IF2_A> for bool {
    #[inline(always)]
    fn from(variant: IF2_A) -> Self {
        match variant {
            IF2_A::IF2_0 => false,
            IF2_A::IF2_1 => true,
        }
    }
}
#[doc = "Reader of field `IF2`"]
pub type IF2_R = crate::R<bool, IF2_A>;
impl IF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF2_A {
        match self.bits {
            false => IF2_A::IF2_0,
            true => IF2_A::IF2_1,
        }
    }
    #[doc = "Checks if the value of the field is `IF2_0`"]
    #[inline(always)]
    pub fn is_if2_0(&self) -> bool {
        *self == IF2_A::IF2_0
    }
    #[doc = "Checks if the value of the field is `IF2_1`"]
    #[inline(always)]
    pub fn is_if2_1(&self) -> bool {
        *self == IF2_A::IF2_1
    }
}
#[doc = "Write proxy for field `IF2`"]
pub struct IF2_W<'a> {
    w: &'a mut W,
}
impl<'a> IF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IF2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture event has not occurred."]
    #[inline(always)]
    pub fn if2_0(self) -> &'a mut W {
        self.variant(IF2_A::IF2_0)
    }
    #[doc = "Capture event has occurred."]
    #[inline(always)]
    pub fn if2_1(self) -> &'a mut W {
        self.variant(IF2_A::IF2_1)
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
#[doc = "Rollover Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROV_A {
    #[doc = "0: Rollover has not occurred."]
    ROV_0,
    #[doc = "1: Rollover has occurred."]
    ROV_1,
}
impl From<ROV_A> for bool {
    #[inline(always)]
    fn from(variant: ROV_A) -> Self {
        match variant {
            ROV_A::ROV_0 => false,
            ROV_A::ROV_1 => true,
        }
    }
}
#[doc = "Reader of field `ROV`"]
pub type ROV_R = crate::R<bool, ROV_A>;
impl ROV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROV_A {
        match self.bits {
            false => ROV_A::ROV_0,
            true => ROV_A::ROV_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROV_0`"]
    #[inline(always)]
    pub fn is_rov_0(&self) -> bool {
        *self == ROV_A::ROV_0
    }
    #[doc = "Checks if the value of the field is `ROV_1`"]
    #[inline(always)]
    pub fn is_rov_1(&self) -> bool {
        *self == ROV_A::ROV_1
    }
}
#[doc = "Write proxy for field `ROV`"]
pub struct ROV_W<'a> {
    w: &'a mut W,
}
impl<'a> ROV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rollover has not occurred."]
    #[inline(always)]
    pub fn rov_0(self) -> &'a mut W {
        self.variant(ROV_A::ROV_0)
    }
    #[doc = "Rollover has occurred."]
    #[inline(always)]
    pub fn rov_1(self) -> &'a mut W {
        self.variant(ROV_A::ROV_1)
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
    #[doc = "Bit 0 - See OF3"]
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - See OF3"]
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - See IF2"]
    #[inline(always)]
    pub fn if1(&self) -> IF1_R {
        IF1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
    #[inline(always)]
    pub fn if2(&self) -> IF2_R {
        IF2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rollover Flag"]
    #[inline(always)]
    pub fn rov(&self) -> ROV_R {
        ROV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See OF3"]
    #[inline(always)]
    pub fn of1(&mut self) -> OF1_W {
        OF1_W { w: self }
    }
    #[doc = "Bit 1 - See OF3"]
    #[inline(always)]
    pub fn of2(&mut self) -> OF2_W {
        OF2_W { w: self }
    }
    #[doc = "Bit 2 - OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
    #[inline(always)]
    pub fn of3(&mut self) -> OF3_W {
        OF3_W { w: self }
    }
    #[doc = "Bit 3 - See IF2"]
    #[inline(always)]
    pub fn if1(&mut self) -> IF1_W {
        IF1_W { w: self }
    }
    #[doc = "Bit 4 - IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
    #[inline(always)]
    pub fn if2(&mut self) -> IF2_W {
        IF2_W { w: self }
    }
    #[doc = "Bit 5 - Rollover Flag"]
    #[inline(always)]
    pub fn rov(&mut self) -> ROV_W {
        ROV_W { w: self }
    }
}
