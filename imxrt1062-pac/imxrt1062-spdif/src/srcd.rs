#[doc = "Reader of register SRCD"]
pub type R = crate::R<u32, super::SRCD>;
#[doc = "Writer for register SRCD"]
pub type W = crate::W<u32, super::SRCD>;
#[doc = "Register SRCD `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USYNCMODE_A {
    #[doc = "0: Non-CD data"]
    USYNCMODE_0 = 0,
    #[doc = "1: CD user channel subcode"]
    USYNCMODE_1 = 1,
}
impl From<USYNCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: USYNCMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USyncMode`"]
pub type USYNCMODE_R = crate::R<bool, USYNCMODE_A>;
impl USYNCMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USYNCMODE_A {
        match self.bits {
            false => USYNCMODE_A::USYNCMODE_0,
            true => USYNCMODE_A::USYNCMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `USYNCMODE_0`"]
    #[inline(always)]
    pub fn is_usync_mode_0(&self) -> bool {
        *self == USYNCMODE_A::USYNCMODE_0
    }
    #[doc = "Checks if the value of the field is `USYNCMODE_1`"]
    #[inline(always)]
    pub fn is_usync_mode_1(&self) -> bool {
        *self == USYNCMODE_A::USYNCMODE_1
    }
}
#[doc = "Write proxy for field `USyncMode`"]
pub struct USYNCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> USYNCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USYNCMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Non-CD data"]
    #[inline(always)]
    pub fn usync_mode_0(self) -> &'a mut W {
        self.variant(USYNCMODE_A::USYNCMODE_0)
    }
    #[doc = "CD user channel subcode"]
    #[inline(always)]
    pub fn usync_mode_1(self) -> &'a mut W {
        self.variant(USYNCMODE_A::USYNCMODE_1)
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
impl R {
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn usync_mode(&self) -> USYNCMODE_R {
        USYNCMODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn usync_mode(&mut self) -> USYNCMODE_W {
        USYNCMODE_W { w: self }
    }
}
