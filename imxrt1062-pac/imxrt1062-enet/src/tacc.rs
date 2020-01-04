#[doc = "Reader of register TACC"]
pub type R = crate::R<u32, super::TACC>;
#[doc = "Writer for register TACC"]
pub type W = crate::W<u32, super::TACC>;
#[doc = "Register TACC `reset()`'s with value 0"]
impl crate::ResetValue for super::TACC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TX FIFO Shift-16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHIFT16_A {
    #[doc = "0: Disabled."]
    SHIFT16_0 = 0,
    #[doc = "1: Indicates to the transmit data FIFO that the written frames contain two additional octets before the frame data. This means the actual frame begins at bit 16 of the first word written into the FIFO. This function allows putting the frame payload on a 32-bit boundary in memory, as the 14-byte Ethernet header is extended to a 16-byte header."]
    SHIFT16_1 = 1,
}
impl From<SHIFT16_A> for bool {
    #[inline(always)]
    fn from(variant: SHIFT16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SHIFT16`"]
pub type SHIFT16_R = crate::R<bool, SHIFT16_A>;
impl SHIFT16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHIFT16_A {
        match self.bits {
            false => SHIFT16_A::SHIFT16_0,
            true => SHIFT16_A::SHIFT16_1,
        }
    }
    #[doc = "Checks if the value of the field is `SHIFT16_0`"]
    #[inline(always)]
    pub fn is_shift16_0(&self) -> bool {
        *self == SHIFT16_A::SHIFT16_0
    }
    #[doc = "Checks if the value of the field is `SHIFT16_1`"]
    #[inline(always)]
    pub fn is_shift16_1(&self) -> bool {
        *self == SHIFT16_A::SHIFT16_1
    }
}
#[doc = "Write proxy for field `SHIFT16`"]
pub struct SHIFT16_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHIFT16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn shift16_0(self) -> &'a mut W {
        self.variant(SHIFT16_A::SHIFT16_0)
    }
    #[doc = "Indicates to the transmit data FIFO that the written frames contain two additional octets before the frame data. This means the actual frame begins at bit 16 of the first word written into the FIFO. This function allows putting the frame payload on a 32-bit boundary in memory, as the 14-byte Ethernet header is extended to a 16-byte header."]
    #[inline(always)]
    pub fn shift16_1(self) -> &'a mut W {
        self.variant(SHIFT16_A::SHIFT16_1)
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
#[doc = "Enables insertion of IP header checksum.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPCHK_A {
    #[doc = "0: Checksum is not inserted."]
    IPCHK_0 = 0,
    #[doc = "1: If an IP frame is transmitted, the checksum is inserted automatically. The IP header checksum field must be cleared. If a non-IP frame is transmitted the frame is not modified."]
    IPCHK_1 = 1,
}
impl From<IPCHK_A> for bool {
    #[inline(always)]
    fn from(variant: IPCHK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IPCHK`"]
pub type IPCHK_R = crate::R<bool, IPCHK_A>;
impl IPCHK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPCHK_A {
        match self.bits {
            false => IPCHK_A::IPCHK_0,
            true => IPCHK_A::IPCHK_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPCHK_0`"]
    #[inline(always)]
    pub fn is_ipchk_0(&self) -> bool {
        *self == IPCHK_A::IPCHK_0
    }
    #[doc = "Checks if the value of the field is `IPCHK_1`"]
    #[inline(always)]
    pub fn is_ipchk_1(&self) -> bool {
        *self == IPCHK_A::IPCHK_1
    }
}
#[doc = "Write proxy for field `IPCHK`"]
pub struct IPCHK_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCHK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPCHK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Checksum is not inserted."]
    #[inline(always)]
    pub fn ipchk_0(self) -> &'a mut W {
        self.variant(IPCHK_A::IPCHK_0)
    }
    #[doc = "If an IP frame is transmitted, the checksum is inserted automatically. The IP header checksum field must be cleared. If a non-IP frame is transmitted the frame is not modified."]
    #[inline(always)]
    pub fn ipchk_1(self) -> &'a mut W {
        self.variant(IPCHK_A::IPCHK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Enables insertion of protocol checksum.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROCHK_A {
    #[doc = "0: Checksum not inserted."]
    PROCHK_0 = 0,
    #[doc = "1: If an IP frame with a known protocol is transmitted, the checksum is inserted automatically into the frame. The checksum field must be cleared. The other frames are not modified."]
    PROCHK_1 = 1,
}
impl From<PROCHK_A> for bool {
    #[inline(always)]
    fn from(variant: PROCHK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PROCHK`"]
pub type PROCHK_R = crate::R<bool, PROCHK_A>;
impl PROCHK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROCHK_A {
        match self.bits {
            false => PROCHK_A::PROCHK_0,
            true => PROCHK_A::PROCHK_1,
        }
    }
    #[doc = "Checks if the value of the field is `PROCHK_0`"]
    #[inline(always)]
    pub fn is_prochk_0(&self) -> bool {
        *self == PROCHK_A::PROCHK_0
    }
    #[doc = "Checks if the value of the field is `PROCHK_1`"]
    #[inline(always)]
    pub fn is_prochk_1(&self) -> bool {
        *self == PROCHK_A::PROCHK_1
    }
}
#[doc = "Write proxy for field `PROCHK`"]
pub struct PROCHK_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCHK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROCHK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Checksum not inserted."]
    #[inline(always)]
    pub fn prochk_0(self) -> &'a mut W {
        self.variant(PROCHK_A::PROCHK_0)
    }
    #[doc = "If an IP frame with a known protocol is transmitted, the checksum is inserted automatically into the frame. The checksum field must be cleared. The other frames are not modified."]
    #[inline(always)]
    pub fn prochk_1(self) -> &'a mut W {
        self.variant(PROCHK_A::PROCHK_1)
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
impl R {
    #[doc = "Bit 0 - TX FIFO Shift-16"]
    #[inline(always)]
    pub fn shift16(&self) -> SHIFT16_R {
        SHIFT16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables insertion of IP header checksum."]
    #[inline(always)]
    pub fn ipchk(&self) -> IPCHK_R {
        IPCHK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables insertion of protocol checksum."]
    #[inline(always)]
    pub fn prochk(&self) -> PROCHK_R {
        PROCHK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Shift-16"]
    #[inline(always)]
    pub fn shift16(&mut self) -> SHIFT16_W {
        SHIFT16_W { w: self }
    }
    #[doc = "Bit 3 - Enables insertion of IP header checksum."]
    #[inline(always)]
    pub fn ipchk(&mut self) -> IPCHK_W {
        IPCHK_W { w: self }
    }
    #[doc = "Bit 4 - Enables insertion of protocol checksum."]
    #[inline(always)]
    pub fn prochk(&mut self) -> PROCHK_W {
        PROCHK_W { w: self }
    }
}
