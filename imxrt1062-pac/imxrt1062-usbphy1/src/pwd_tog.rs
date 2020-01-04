#[doc = "Reader of register PWD_TOG"]
pub type R = crate::R<u32, super::PWD_TOG>;
#[doc = "Writer for register PWD_TOG"]
pub type W = crate::W<u32, super::PWD_TOG>;
#[doc = "Register PWD_TOG `reset()`'s with value 0x001e_1c00"]
impl crate::ResetValue for super::PWD_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x001e_1c00
    }
}
#[doc = "Reader of field `RSVD0`"]
pub type RSVD0_R = crate::R<u16, u16>;
#[doc = "Reader of field `TXPWDFS`"]
pub type TXPWDFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPWDFS`"]
pub struct TXPWDFS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPWDFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TXPWDIBIAS`"]
pub type TXPWDIBIAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPWDIBIAS`"]
pub struct TXPWDIBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPWDIBIAS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TXPWDV2I`"]
pub type TXPWDV2I_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPWDV2I`"]
pub struct TXPWDV2I_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPWDV2I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `RSVD1`"]
pub type RSVD1_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXPWDENV`"]
pub type RXPWDENV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPWDENV`"]
pub struct RXPWDENV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPWDENV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RXPWD1PT1`"]
pub type RXPWD1PT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPWD1PT1`"]
pub struct RXPWD1PT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPWD1PT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `RXPWDDIFF`"]
pub type RXPWDDIFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPWDDIFF`"]
pub struct RXPWDDIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPWDDIFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RXPWDRX`"]
pub type RXPWDRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPWDRX`"]
pub struct RXPWDRX_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPWDRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RSVD2`"]
pub type RSVD2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - 0 = Normal operation"]
    #[inline(always)]
    pub fn txpwdfs(&self) -> TXPWDFS_R {
        TXPWDFS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 0 = Normal operation"]
    #[inline(always)]
    pub fn txpwdibias(&self) -> TXPWDIBIAS_R {
        TXPWDIBIAS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 0 = Normal operation"]
    #[inline(always)]
    pub fn txpwdv2i(&self) -> TXPWDV2I_R {
        TXPWDV2I_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:16 - Reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxpwdenv(&self) -> RXPWDENV_R {
        RXPWDENV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxpwd1pt1(&self) -> RXPWD1PT1_R {
        RXPWD1PT1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxpwddiff(&self) -> RXPWDDIFF_R {
        RXPWDDIFF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxpwdrx(&self) -> RXPWDRX_R {
        RXPWDRX_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:31 - Reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 10 - 0 = Normal operation"]
    #[inline(always)]
    pub fn txpwdfs(&mut self) -> TXPWDFS_W {
        TXPWDFS_W { w: self }
    }
    #[doc = "Bit 11 - 0 = Normal operation"]
    #[inline(always)]
    pub fn txpwdibias(&mut self) -> TXPWDIBIAS_W {
        TXPWDIBIAS_W { w: self }
    }
    #[doc = "Bit 12 - 0 = Normal operation"]
    #[inline(always)]
    pub fn txpwdv2i(&mut self) -> TXPWDV2I_W {
        TXPWDV2I_W { w: self }
    }
    #[doc = "Bit 17 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxpwdenv(&mut self) -> RXPWDENV_W {
        RXPWDENV_W { w: self }
    }
    #[doc = "Bit 18 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxpwd1pt1(&mut self) -> RXPWD1PT1_W {
        RXPWD1PT1_W { w: self }
    }
    #[doc = "Bit 19 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxpwddiff(&mut self) -> RXPWDDIFF_W {
        RXPWDDIFF_W { w: self }
    }
    #[doc = "Bit 20 - 0 = Normal operation"]
    #[inline(always)]
    pub fn rxpwdrx(&mut self) -> RXPWDRX_W {
        RXPWDRX_W { w: self }
    }
}
