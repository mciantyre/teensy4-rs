#[doc = "Reader of register IPCR1"]
pub type R = crate::R<u32, super::IPCR1>;
#[doc = "Writer for register IPCR1"]
pub type W = crate::W<u32, super::IPCR1>;
#[doc = "Register IPCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IPCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDATSZ`"]
pub type IDATSZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IDATSZ`"]
pub struct IDATSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> IDATSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ISEQID`"]
pub type ISEQID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ISEQID`"]
pub struct ISEQID_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEQID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ISEQNUM`"]
pub type ISEQNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ISEQNUM`"]
pub struct ISEQNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEQNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Parallel mode Enabled for IP command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPAREN_A {
    #[doc = "0: Flash will be accessed in Individual mode."]
    IPAREN_0 = 0,
    #[doc = "1: Flash will be accessed in Parallel mode."]
    IPAREN_1 = 1,
}
impl From<IPAREN_A> for bool {
    #[inline(always)]
    fn from(variant: IPAREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IPAREN`"]
pub type IPAREN_R = crate::R<bool, IPAREN_A>;
impl IPAREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPAREN_A {
        match self.bits {
            false => IPAREN_A::IPAREN_0,
            true => IPAREN_A::IPAREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPAREN_0`"]
    #[inline(always)]
    pub fn is_iparen_0(&self) -> bool {
        *self == IPAREN_A::IPAREN_0
    }
    #[doc = "Checks if the value of the field is `IPAREN_1`"]
    #[inline(always)]
    pub fn is_iparen_1(&self) -> bool {
        *self == IPAREN_A::IPAREN_1
    }
}
#[doc = "Write proxy for field `IPAREN`"]
pub struct IPAREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPAREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPAREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash will be accessed in Individual mode."]
    #[inline(always)]
    pub fn iparen_0(self) -> &'a mut W {
        self.variant(IPAREN_A::IPAREN_0)
    }
    #[doc = "Flash will be accessed in Parallel mode."]
    #[inline(always)]
    pub fn iparen_1(self) -> &'a mut W {
        self.variant(IPAREN_A::IPAREN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Flash Read/Program Data Size (in Bytes) for IP command."]
    #[inline(always)]
    pub fn idatsz(&self) -> IDATSZ_R {
        IDATSZ_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Sequence Index in LUT for IP command."]
    #[inline(always)]
    pub fn iseqid(&self) -> ISEQID_R {
        ISEQID_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Sequence Number for IP command: ISEQNUM+1."]
    #[inline(always)]
    pub fn iseqnum(&self) -> ISEQNUM_R {
        ISEQNUM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Parallel mode Enabled for IP command."]
    #[inline(always)]
    pub fn iparen(&self) -> IPAREN_R {
        IPAREN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Read/Program Data Size (in Bytes) for IP command."]
    #[inline(always)]
    pub fn idatsz(&mut self) -> IDATSZ_W {
        IDATSZ_W { w: self }
    }
    #[doc = "Bits 16:19 - Sequence Index in LUT for IP command."]
    #[inline(always)]
    pub fn iseqid(&mut self) -> ISEQID_W {
        ISEQID_W { w: self }
    }
    #[doc = "Bits 24:26 - Sequence Number for IP command: ISEQNUM+1."]
    #[inline(always)]
    pub fn iseqnum(&mut self) -> ISEQNUM_W {
        ISEQNUM_W { w: self }
    }
    #[doc = "Bit 31 - Parallel mode Enabled for IP command."]
    #[inline(always)]
    pub fn iparen(&mut self) -> IPAREN_W {
        IPAREN_W { w: self }
    }
}
