#[doc = "Reader of register DEBUG1_SET"]
pub type R = crate::R<u32, super::DEBUG1_SET>;
#[doc = "Writer for register DEBUG1_SET"]
pub type W = crate::W<u32, super::DEBUG1_SET>;
#[doc = "Register DEBUG1_SET `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::DEBUG1_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RSVD0`"]
pub struct RSVD0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Reader of field `ENTAILADJVD`"]
pub type ENTAILADJVD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ENTAILADJVD`"]
pub struct ENTAILADJVD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTAILADJVD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:12 - Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub fn entailadjvd(&self) -> ENTAILADJVD_R {
        ENTAILADJVD_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 15:31 - Reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 15) & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:12 - Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub fn rsvd0(&mut self) -> RSVD0_W {
        RSVD0_W { w: self }
    }
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub fn entailadjvd(&mut self) -> ENTAILADJVD_W {
        ENTAILADJVD_W { w: self }
    }
}
