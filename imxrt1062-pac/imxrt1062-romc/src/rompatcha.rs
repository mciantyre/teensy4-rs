#[doc = "Reader of register ROMPATCH%sA"]
pub type R = crate::R<u32, super::ROMPATCHA>;
#[doc = "Writer for register ROMPATCH%sA"]
pub type W = crate::W<u32, super::ROMPATCHA>;
#[doc = "Register ROMPATCH%sA `reset()`'s with value 0"]
impl crate::ResetValue for super::ROMPATCHA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "THUMB Comparator Select - Indicates that this address will trigger a THUMB opcode patch or an Arm opcode patch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THUMBX_A {
    #[doc = "0: Arm patch"]
    THUMBX_0 = 0,
    #[doc = "1: THUMB patch (ignore if data fix)"]
    THUMBX_1 = 1,
}
impl From<THUMBX_A> for bool {
    #[inline(always)]
    fn from(variant: THUMBX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `THUMBX`"]
pub type THUMBX_R = crate::R<bool, THUMBX_A>;
impl THUMBX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THUMBX_A {
        match self.bits {
            false => THUMBX_A::THUMBX_0,
            true => THUMBX_A::THUMBX_1,
        }
    }
    #[doc = "Checks if the value of the field is `THUMBX_0`"]
    #[inline(always)]
    pub fn is_thumbx_0(&self) -> bool {
        *self == THUMBX_A::THUMBX_0
    }
    #[doc = "Checks if the value of the field is `THUMBX_1`"]
    #[inline(always)]
    pub fn is_thumbx_1(&self) -> bool {
        *self == THUMBX_A::THUMBX_1
    }
}
#[doc = "Write proxy for field `THUMBX`"]
pub struct THUMBX_W<'a> {
    w: &'a mut W,
}
impl<'a> THUMBX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THUMBX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Arm patch"]
    #[inline(always)]
    pub fn thumbx_0(self) -> &'a mut W {
        self.variant(THUMBX_A::THUMBX_0)
    }
    #[doc = "THUMB patch (ignore if data fix)"]
    #[inline(always)]
    pub fn thumbx_1(self) -> &'a mut W {
        self.variant(THUMBX_A::THUMBX_1)
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
#[doc = "Reader of field `ADDRX`"]
pub type ADDRX_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDRX`"]
pub struct ADDRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 1)) | (((value as u32) & 0x003f_ffff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - THUMB Comparator Select - Indicates that this address will trigger a THUMB opcode patch or an Arm opcode patch"]
    #[inline(always)]
    pub fn thumbx(&self) -> THUMBX_R {
        THUMBX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:22 - Address Comparator Registers - Indicates the memory address to be watched"]
    #[inline(always)]
    pub fn addrx(&self) -> ADDRX_R {
        ADDRX_R::new(((self.bits >> 1) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - THUMB Comparator Select - Indicates that this address will trigger a THUMB opcode patch or an Arm opcode patch"]
    #[inline(always)]
    pub fn thumbx(&mut self) -> THUMBX_W {
        THUMBX_W { w: self }
    }
    #[doc = "Bits 1:22 - Address Comparator Registers - Indicates the memory address to be watched"]
    #[inline(always)]
    pub fn addrx(&mut self) -> ADDRX_W {
        ADDRX_W { w: self }
    }
}
