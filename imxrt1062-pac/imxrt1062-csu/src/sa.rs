#[doc = "Reader of register SA"]
pub type R = crate::R<u32, super::SA>;
#[doc = "Writer for register SA"]
pub type W = crate::W<u32, super::SA>;
#[doc = "Register SA `reset()`'s with value 0"]
impl crate::ResetValue for super::SA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Non-secure access policy indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_DMA_A {
    #[doc = "0: Secure access for the corresponding type-1 master"]
    NSA_DMA_0 = 0,
    #[doc = "1: Non-secure access for the corresponding type-1 master"]
    NSA_DMA_1 = 1,
}
impl From<NSA_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: NSA_DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSA_DMA`"]
pub type NSA_DMA_R = crate::R<bool, NSA_DMA_A>;
impl NSA_DMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSA_DMA_A {
        match self.bits {
            false => NSA_DMA_A::NSA_DMA_0,
            true => NSA_DMA_A::NSA_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_DMA_0`"]
    #[inline(always)]
    pub fn is_nsa_dma_0(&self) -> bool {
        *self == NSA_DMA_A::NSA_DMA_0
    }
    #[doc = "Checks if the value of the field is `NSA_DMA_1`"]
    #[inline(always)]
    pub fn is_nsa_dma_1(&self) -> bool {
        *self == NSA_DMA_A::NSA_DMA_1
    }
}
#[doc = "Write proxy for field `NSA_DMA`"]
pub struct NSA_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> NSA_DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSA_DMA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_dma_0(self) -> &'a mut W {
        self.variant(NSA_DMA_A::NSA_DMA_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_dma_1(self) -> &'a mut W {
        self.variant(NSA_DMA_A::NSA_DMA_1)
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
#[doc = "Lock bit set by the TZ software for the eDMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_DMA_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_DMA_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DMA_1 = 1,
}
impl From<L_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: L_DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L_DMA`"]
pub type L_DMA_R = crate::R<bool, L_DMA_A>;
impl L_DMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_DMA_A {
        match self.bits {
            false => L_DMA_A::L_DMA_0,
            true => L_DMA_A::L_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_DMA_0`"]
    #[inline(always)]
    pub fn is_l_dma_0(&self) -> bool {
        *self == L_DMA_A::L_DMA_0
    }
    #[doc = "Checks if the value of the field is `L_DMA_1`"]
    #[inline(always)]
    pub fn is_l_dma_1(&self) -> bool {
        *self == L_DMA_A::L_DMA_1
    }
}
#[doc = "Write proxy for field `L_DMA`"]
pub struct L_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> L_DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L_DMA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_dma_0(self) -> &'a mut W {
        self.variant(L_DMA_A::L_DMA_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_dma_1(self) -> &'a mut W {
        self.variant(L_DMA_A::L_DMA_1)
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
#[doc = "Non-secure access policy indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_LCDIF_A {
    #[doc = "0: Secure access for the corresponding type-1 master"]
    NSA_LCDIF_0 = 0,
    #[doc = "1: Non-secure access for the corresponding type-1 master"]
    NSA_LCDIF_1 = 1,
}
impl From<NSA_LCDIF_A> for bool {
    #[inline(always)]
    fn from(variant: NSA_LCDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSA_LCDIF`"]
pub type NSA_LCDIF_R = crate::R<bool, NSA_LCDIF_A>;
impl NSA_LCDIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSA_LCDIF_A {
        match self.bits {
            false => NSA_LCDIF_A::NSA_LCDIF_0,
            true => NSA_LCDIF_A::NSA_LCDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_LCDIF_0`"]
    #[inline(always)]
    pub fn is_nsa_lcdif_0(&self) -> bool {
        *self == NSA_LCDIF_A::NSA_LCDIF_0
    }
    #[doc = "Checks if the value of the field is `NSA_LCDIF_1`"]
    #[inline(always)]
    pub fn is_nsa_lcdif_1(&self) -> bool {
        *self == NSA_LCDIF_A::NSA_LCDIF_1
    }
}
#[doc = "Write proxy for field `NSA_LCDIF`"]
pub struct NSA_LCDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> NSA_LCDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSA_LCDIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_lcdif_0(self) -> &'a mut W {
        self.variant(NSA_LCDIF_A::NSA_LCDIF_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_lcdif_1(self) -> &'a mut W {
        self.variant(NSA_LCDIF_A::NSA_LCDIF_1)
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
#[doc = "Lock bit set by the TZ software for the LCDIF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_LCDIF_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_LCDIF_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_LCDIF_1 = 1,
}
impl From<L_LCDIF_A> for bool {
    #[inline(always)]
    fn from(variant: L_LCDIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L_LCDIF`"]
pub type L_LCDIF_R = crate::R<bool, L_LCDIF_A>;
impl L_LCDIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_LCDIF_A {
        match self.bits {
            false => L_LCDIF_A::L_LCDIF_0,
            true => L_LCDIF_A::L_LCDIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_LCDIF_0`"]
    #[inline(always)]
    pub fn is_l_lcdif_0(&self) -> bool {
        *self == L_LCDIF_A::L_LCDIF_0
    }
    #[doc = "Checks if the value of the field is `L_LCDIF_1`"]
    #[inline(always)]
    pub fn is_l_lcdif_1(&self) -> bool {
        *self == L_LCDIF_A::L_LCDIF_1
    }
}
#[doc = "Write proxy for field `L_LCDIF`"]
pub struct L_LCDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> L_LCDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L_LCDIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_lcdif_0(self) -> &'a mut W {
        self.variant(L_LCDIF_A::L_LCDIF_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_lcdif_1(self) -> &'a mut W {
        self.variant(L_LCDIF_A::L_LCDIF_1)
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
#[doc = "Non-secure access policy indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_CSI_A {
    #[doc = "0: Secure access for the corresponding type-1 master"]
    NSA_CSI_0 = 0,
    #[doc = "1: Non-secure access for the corresponding type-1 master"]
    NSA_CSI_1 = 1,
}
impl From<NSA_CSI_A> for bool {
    #[inline(always)]
    fn from(variant: NSA_CSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSA_CSI`"]
pub type NSA_CSI_R = crate::R<bool, NSA_CSI_A>;
impl NSA_CSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSA_CSI_A {
        match self.bits {
            false => NSA_CSI_A::NSA_CSI_0,
            true => NSA_CSI_A::NSA_CSI_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_CSI_0`"]
    #[inline(always)]
    pub fn is_nsa_csi_0(&self) -> bool {
        *self == NSA_CSI_A::NSA_CSI_0
    }
    #[doc = "Checks if the value of the field is `NSA_CSI_1`"]
    #[inline(always)]
    pub fn is_nsa_csi_1(&self) -> bool {
        *self == NSA_CSI_A::NSA_CSI_1
    }
}
#[doc = "Write proxy for field `NSA_CSI`"]
pub struct NSA_CSI_W<'a> {
    w: &'a mut W,
}
impl<'a> NSA_CSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSA_CSI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_csi_0(self) -> &'a mut W {
        self.variant(NSA_CSI_A::NSA_CSI_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_csi_1(self) -> &'a mut W {
        self.variant(NSA_CSI_A::NSA_CSI_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Lock bit set by the TZ software for the CSI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_CSI_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_CSI_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_CSI_1 = 1,
}
impl From<L_CSI_A> for bool {
    #[inline(always)]
    fn from(variant: L_CSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L_CSI`"]
pub type L_CSI_R = crate::R<bool, L_CSI_A>;
impl L_CSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_CSI_A {
        match self.bits {
            false => L_CSI_A::L_CSI_0,
            true => L_CSI_A::L_CSI_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_CSI_0`"]
    #[inline(always)]
    pub fn is_l_csi_0(&self) -> bool {
        *self == L_CSI_A::L_CSI_0
    }
    #[doc = "Checks if the value of the field is `L_CSI_1`"]
    #[inline(always)]
    pub fn is_l_csi_1(&self) -> bool {
        *self == L_CSI_A::L_CSI_1
    }
}
#[doc = "Write proxy for field `L_CSI`"]
pub struct L_CSI_W<'a> {
    w: &'a mut W,
}
impl<'a> L_CSI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L_CSI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_csi_0(self) -> &'a mut W {
        self.variant(L_CSI_A::L_CSI_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_csi_1(self) -> &'a mut W {
        self.variant(L_CSI_A::L_CSI_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Non-Secure Access Policy indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_PXP_A {
    #[doc = "0: Secure access for the corresponding type-1 master"]
    NSA_PXP_0 = 0,
    #[doc = "1: Non-secure access for the corresponding type-1 master"]
    NSA_PXP_1 = 1,
}
impl From<NSA_PXP_A> for bool {
    #[inline(always)]
    fn from(variant: NSA_PXP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSA_PXP`"]
pub type NSA_PXP_R = crate::R<bool, NSA_PXP_A>;
impl NSA_PXP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSA_PXP_A {
        match self.bits {
            false => NSA_PXP_A::NSA_PXP_0,
            true => NSA_PXP_A::NSA_PXP_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_PXP_0`"]
    #[inline(always)]
    pub fn is_nsa_pxp_0(&self) -> bool {
        *self == NSA_PXP_A::NSA_PXP_0
    }
    #[doc = "Checks if the value of the field is `NSA_PXP_1`"]
    #[inline(always)]
    pub fn is_nsa_pxp_1(&self) -> bool {
        *self == NSA_PXP_A::NSA_PXP_1
    }
}
#[doc = "Write proxy for field `NSA_PXP`"]
pub struct NSA_PXP_W<'a> {
    w: &'a mut W,
}
impl<'a> NSA_PXP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSA_PXP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_pxp_0(self) -> &'a mut W {
        self.variant(NSA_PXP_A::NSA_PXP_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_pxp_1(self) -> &'a mut W {
        self.variant(NSA_PXP_A::NSA_PXP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Lock bit set by the TZ software for the PXP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_PXP_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_PXP_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_PXP_1 = 1,
}
impl From<L_PXP_A> for bool {
    #[inline(always)]
    fn from(variant: L_PXP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L_PXP`"]
pub type L_PXP_R = crate::R<bool, L_PXP_A>;
impl L_PXP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_PXP_A {
        match self.bits {
            false => L_PXP_A::L_PXP_0,
            true => L_PXP_A::L_PXP_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_PXP_0`"]
    #[inline(always)]
    pub fn is_l_pxp_0(&self) -> bool {
        *self == L_PXP_A::L_PXP_0
    }
    #[doc = "Checks if the value of the field is `L_PXP_1`"]
    #[inline(always)]
    pub fn is_l_pxp_1(&self) -> bool {
        *self == L_PXP_A::L_PXP_1
    }
}
#[doc = "Write proxy for field `L_PXP`"]
pub struct L_PXP_W<'a> {
    w: &'a mut W,
}
impl<'a> L_PXP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L_PXP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_pxp_0(self) -> &'a mut W {
        self.variant(L_PXP_A::L_PXP_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_pxp_1(self) -> &'a mut W {
        self.variant(L_PXP_A::L_PXP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Non-secure access policy indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_DCP_A {
    #[doc = "0: Secure access for the corresponding type-1 master"]
    NSA_DCP_0 = 0,
    #[doc = "1: Non-secure access for the corresponding type-1 master"]
    NSA_DCP_1 = 1,
}
impl From<NSA_DCP_A> for bool {
    #[inline(always)]
    fn from(variant: NSA_DCP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSA_DCP`"]
pub type NSA_DCP_R = crate::R<bool, NSA_DCP_A>;
impl NSA_DCP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSA_DCP_A {
        match self.bits {
            false => NSA_DCP_A::NSA_DCP_0,
            true => NSA_DCP_A::NSA_DCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_DCP_0`"]
    #[inline(always)]
    pub fn is_nsa_dcp_0(&self) -> bool {
        *self == NSA_DCP_A::NSA_DCP_0
    }
    #[doc = "Checks if the value of the field is `NSA_DCP_1`"]
    #[inline(always)]
    pub fn is_nsa_dcp_1(&self) -> bool {
        *self == NSA_DCP_A::NSA_DCP_1
    }
}
#[doc = "Write proxy for field `NSA_DCP`"]
pub struct NSA_DCP_W<'a> {
    w: &'a mut W,
}
impl<'a> NSA_DCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSA_DCP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_dcp_0(self) -> &'a mut W {
        self.variant(NSA_DCP_A::NSA_DCP_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_dcp_1(self) -> &'a mut W {
        self.variant(NSA_DCP_A::NSA_DCP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Lock bit set by the TZ software for the DCP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_DCP_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_DCP_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_DCP_1 = 1,
}
impl From<L_DCP_A> for bool {
    #[inline(always)]
    fn from(variant: L_DCP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L_DCP`"]
pub type L_DCP_R = crate::R<bool, L_DCP_A>;
impl L_DCP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_DCP_A {
        match self.bits {
            false => L_DCP_A::L_DCP_0,
            true => L_DCP_A::L_DCP_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_DCP_0`"]
    #[inline(always)]
    pub fn is_l_dcp_0(&self) -> bool {
        *self == L_DCP_A::L_DCP_0
    }
    #[doc = "Checks if the value of the field is `L_DCP_1`"]
    #[inline(always)]
    pub fn is_l_dcp_1(&self) -> bool {
        *self == L_DCP_A::L_DCP_1
    }
}
#[doc = "Write proxy for field `L_DCP`"]
pub struct L_DCP_W<'a> {
    w: &'a mut W,
}
impl<'a> L_DCP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L_DCP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_dcp_0(self) -> &'a mut W {
        self.variant(L_DCP_A::L_DCP_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_dcp_1(self) -> &'a mut W {
        self.variant(L_DCP_A::L_DCP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Non-secure access policy indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_ENET_A {
    #[doc = "0: Secure access for the corresponding type-1 master"]
    NSA_ENET_0 = 0,
    #[doc = "1: Non-secure access for the corresponding type-1 master"]
    NSA_ENET_1 = 1,
}
impl From<NSA_ENET_A> for bool {
    #[inline(always)]
    fn from(variant: NSA_ENET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSA_ENET`"]
pub type NSA_ENET_R = crate::R<bool, NSA_ENET_A>;
impl NSA_ENET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSA_ENET_A {
        match self.bits {
            false => NSA_ENET_A::NSA_ENET_0,
            true => NSA_ENET_A::NSA_ENET_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_ENET_0`"]
    #[inline(always)]
    pub fn is_nsa_enet_0(&self) -> bool {
        *self == NSA_ENET_A::NSA_ENET_0
    }
    #[doc = "Checks if the value of the field is `NSA_ENET_1`"]
    #[inline(always)]
    pub fn is_nsa_enet_1(&self) -> bool {
        *self == NSA_ENET_A::NSA_ENET_1
    }
}
#[doc = "Write proxy for field `NSA_ENET`"]
pub struct NSA_ENET_W<'a> {
    w: &'a mut W,
}
impl<'a> NSA_ENET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSA_ENET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_enet_0(self) -> &'a mut W {
        self.variant(NSA_ENET_A::NSA_ENET_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_enet_1(self) -> &'a mut W {
        self.variant(NSA_ENET_A::NSA_ENET_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Lock bit set by the TZ software for the ENET1 and ENET2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_ENET_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET_1 = 1,
}
impl From<L_ENET_A> for bool {
    #[inline(always)]
    fn from(variant: L_ENET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L_ENET`"]
pub type L_ENET_R = crate::R<bool, L_ENET_A>;
impl L_ENET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_ENET_A {
        match self.bits {
            false => L_ENET_A::L_ENET_0,
            true => L_ENET_A::L_ENET_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_ENET_0`"]
    #[inline(always)]
    pub fn is_l_enet_0(&self) -> bool {
        *self == L_ENET_A::L_ENET_0
    }
    #[doc = "Checks if the value of the field is `L_ENET_1`"]
    #[inline(always)]
    pub fn is_l_enet_1(&self) -> bool {
        *self == L_ENET_A::L_ENET_1
    }
}
#[doc = "Write proxy for field `L_ENET`"]
pub struct L_ENET_W<'a> {
    w: &'a mut W,
}
impl<'a> L_ENET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L_ENET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_enet_0(self) -> &'a mut W {
        self.variant(L_ENET_A::L_ENET_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_enet_1(self) -> &'a mut W {
        self.variant(L_ENET_A::L_ENET_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Non-secure access policy indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_USDHC1_A {
    #[doc = "0: Secure access for the corresponding type-1 master"]
    NSA_USDHC1_0 = 0,
    #[doc = "1: Non-secure access for the corresponding type-1 master"]
    NSA_USDHC1_1 = 1,
}
impl From<NSA_USDHC1_A> for bool {
    #[inline(always)]
    fn from(variant: NSA_USDHC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSA_USDHC1`"]
pub type NSA_USDHC1_R = crate::R<bool, NSA_USDHC1_A>;
impl NSA_USDHC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSA_USDHC1_A {
        match self.bits {
            false => NSA_USDHC1_A::NSA_USDHC1_0,
            true => NSA_USDHC1_A::NSA_USDHC1_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_USDHC1_0`"]
    #[inline(always)]
    pub fn is_nsa_usdhc1_0(&self) -> bool {
        *self == NSA_USDHC1_A::NSA_USDHC1_0
    }
    #[doc = "Checks if the value of the field is `NSA_USDHC1_1`"]
    #[inline(always)]
    pub fn is_nsa_usdhc1_1(&self) -> bool {
        *self == NSA_USDHC1_A::NSA_USDHC1_1
    }
}
#[doc = "Write proxy for field `NSA_USDHC1`"]
pub struct NSA_USDHC1_W<'a> {
    w: &'a mut W,
}
impl<'a> NSA_USDHC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSA_USDHC1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_usdhc1_0(self) -> &'a mut W {
        self.variant(NSA_USDHC1_A::NSA_USDHC1_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_usdhc1_1(self) -> &'a mut W {
        self.variant(NSA_USDHC1_A::NSA_USDHC1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Lock bit set by the TZ software for the USDHC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_USDHC1_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC1_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC1_1 = 1,
}
impl From<L_USDHC1_A> for bool {
    #[inline(always)]
    fn from(variant: L_USDHC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L_USDHC1`"]
pub type L_USDHC1_R = crate::R<bool, L_USDHC1_A>;
impl L_USDHC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_USDHC1_A {
        match self.bits {
            false => L_USDHC1_A::L_USDHC1_0,
            true => L_USDHC1_A::L_USDHC1_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_USDHC1_0`"]
    #[inline(always)]
    pub fn is_l_usdhc1_0(&self) -> bool {
        *self == L_USDHC1_A::L_USDHC1_0
    }
    #[doc = "Checks if the value of the field is `L_USDHC1_1`"]
    #[inline(always)]
    pub fn is_l_usdhc1_1(&self) -> bool {
        *self == L_USDHC1_A::L_USDHC1_1
    }
}
#[doc = "Write proxy for field `L_USDHC1`"]
pub struct L_USDHC1_W<'a> {
    w: &'a mut W,
}
impl<'a> L_USDHC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L_USDHC1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_usdhc1_0(self) -> &'a mut W {
        self.variant(L_USDHC1_A::L_USDHC1_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_usdhc1_1(self) -> &'a mut W {
        self.variant(L_USDHC1_A::L_USDHC1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Non-secure access policy indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_USDHC2_A {
    #[doc = "0: Secure access for the corresponding type-1 master"]
    NSA_USDHC2_0 = 0,
    #[doc = "1: Non-secure access for the corresponding type-1 master"]
    NSA_USDHC2_1 = 1,
}
impl From<NSA_USDHC2_A> for bool {
    #[inline(always)]
    fn from(variant: NSA_USDHC2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSA_USDHC2`"]
pub type NSA_USDHC2_R = crate::R<bool, NSA_USDHC2_A>;
impl NSA_USDHC2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSA_USDHC2_A {
        match self.bits {
            false => NSA_USDHC2_A::NSA_USDHC2_0,
            true => NSA_USDHC2_A::NSA_USDHC2_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_USDHC2_0`"]
    #[inline(always)]
    pub fn is_nsa_usdhc2_0(&self) -> bool {
        *self == NSA_USDHC2_A::NSA_USDHC2_0
    }
    #[doc = "Checks if the value of the field is `NSA_USDHC2_1`"]
    #[inline(always)]
    pub fn is_nsa_usdhc2_1(&self) -> bool {
        *self == NSA_USDHC2_A::NSA_USDHC2_1
    }
}
#[doc = "Write proxy for field `NSA_USDHC2`"]
pub struct NSA_USDHC2_W<'a> {
    w: &'a mut W,
}
impl<'a> NSA_USDHC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSA_USDHC2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_usdhc2_0(self) -> &'a mut W {
        self.variant(NSA_USDHC2_A::NSA_USDHC2_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_usdhc2_1(self) -> &'a mut W {
        self.variant(NSA_USDHC2_A::NSA_USDHC2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Lock bit set by the TZ software for the USDHC2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_USDHC2_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_USDHC2_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USDHC2_1 = 1,
}
impl From<L_USDHC2_A> for bool {
    #[inline(always)]
    fn from(variant: L_USDHC2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L_USDHC2`"]
pub type L_USDHC2_R = crate::R<bool, L_USDHC2_A>;
impl L_USDHC2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_USDHC2_A {
        match self.bits {
            false => L_USDHC2_A::L_USDHC2_0,
            true => L_USDHC2_A::L_USDHC2_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_USDHC2_0`"]
    #[inline(always)]
    pub fn is_l_usdhc2_0(&self) -> bool {
        *self == L_USDHC2_A::L_USDHC2_0
    }
    #[doc = "Checks if the value of the field is `L_USDHC2_1`"]
    #[inline(always)]
    pub fn is_l_usdhc2_1(&self) -> bool {
        *self == L_USDHC2_A::L_USDHC2_1
    }
}
#[doc = "Write proxy for field `L_USDHC2`"]
pub struct L_USDHC2_W<'a> {
    w: &'a mut W,
}
impl<'a> L_USDHC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L_USDHC2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_usdhc2_0(self) -> &'a mut W {
        self.variant(L_USDHC2_A::L_USDHC2_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_usdhc2_1(self) -> &'a mut W {
        self.variant(L_USDHC2_A::L_USDHC2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Non-secure access policy indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_TPSMP_A {
    #[doc = "0: Secure access for the corresponding type-1 master"]
    NSA_TPSMP_0 = 0,
    #[doc = "1: Non-secure access for the corresponding type-1 master"]
    NSA_TPSMP_1 = 1,
}
impl From<NSA_TPSMP_A> for bool {
    #[inline(always)]
    fn from(variant: NSA_TPSMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSA_TPSMP`"]
pub type NSA_TPSMP_R = crate::R<bool, NSA_TPSMP_A>;
impl NSA_TPSMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSA_TPSMP_A {
        match self.bits {
            false => NSA_TPSMP_A::NSA_TPSMP_0,
            true => NSA_TPSMP_A::NSA_TPSMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_TPSMP_0`"]
    #[inline(always)]
    pub fn is_nsa_tpsmp_0(&self) -> bool {
        *self == NSA_TPSMP_A::NSA_TPSMP_0
    }
    #[doc = "Checks if the value of the field is `NSA_TPSMP_1`"]
    #[inline(always)]
    pub fn is_nsa_tpsmp_1(&self) -> bool {
        *self == NSA_TPSMP_A::NSA_TPSMP_1
    }
}
#[doc = "Write proxy for field `NSA_TPSMP`"]
pub struct NSA_TPSMP_W<'a> {
    w: &'a mut W,
}
impl<'a> NSA_TPSMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSA_TPSMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_tpsmp_0(self) -> &'a mut W {
        self.variant(NSA_TPSMP_A::NSA_TPSMP_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_tpsmp_1(self) -> &'a mut W {
        self.variant(NSA_TPSMP_A::NSA_TPSMP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Lock bit set by the TZ software for the TPSMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_TPSMP_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_TPSMP_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_TPSMP_1 = 1,
}
impl From<L_TPSMP_A> for bool {
    #[inline(always)]
    fn from(variant: L_TPSMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L_TPSMP`"]
pub type L_TPSMP_R = crate::R<bool, L_TPSMP_A>;
impl L_TPSMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_TPSMP_A {
        match self.bits {
            false => L_TPSMP_A::L_TPSMP_0,
            true => L_TPSMP_A::L_TPSMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_TPSMP_0`"]
    #[inline(always)]
    pub fn is_l_tpsmp_0(&self) -> bool {
        *self == L_TPSMP_A::L_TPSMP_0
    }
    #[doc = "Checks if the value of the field is `L_TPSMP_1`"]
    #[inline(always)]
    pub fn is_l_tpsmp_1(&self) -> bool {
        *self == L_TPSMP_A::L_TPSMP_1
    }
}
#[doc = "Write proxy for field `L_TPSMP`"]
pub struct L_TPSMP_W<'a> {
    w: &'a mut W,
}
impl<'a> L_TPSMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L_TPSMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_tpsmp_0(self) -> &'a mut W {
        self.variant(L_TPSMP_A::L_TPSMP_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_tpsmp_1(self) -> &'a mut W {
        self.variant(L_TPSMP_A::L_TPSMP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Non-secure access policy indicator bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSA_USB_A {
    #[doc = "0: Secure access for the corresponding type-1 master"]
    NSA_USB_0 = 0,
    #[doc = "1: Non-secure access for the corresponding type-1 master"]
    NSA_USB_1 = 1,
}
impl From<NSA_USB_A> for bool {
    #[inline(always)]
    fn from(variant: NSA_USB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSA_USB`"]
pub type NSA_USB_R = crate::R<bool, NSA_USB_A>;
impl NSA_USB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSA_USB_A {
        match self.bits {
            false => NSA_USB_A::NSA_USB_0,
            true => NSA_USB_A::NSA_USB_1,
        }
    }
    #[doc = "Checks if the value of the field is `NSA_USB_0`"]
    #[inline(always)]
    pub fn is_nsa_usb_0(&self) -> bool {
        *self == NSA_USB_A::NSA_USB_0
    }
    #[doc = "Checks if the value of the field is `NSA_USB_1`"]
    #[inline(always)]
    pub fn is_nsa_usb_1(&self) -> bool {
        *self == NSA_USB_A::NSA_USB_1
    }
}
#[doc = "Write proxy for field `NSA_USB`"]
pub struct NSA_USB_W<'a> {
    w: &'a mut W,
}
impl<'a> NSA_USB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSA_USB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_usb_0(self) -> &'a mut W {
        self.variant(NSA_USB_A::NSA_USB_0)
    }
    #[doc = "Non-secure access for the corresponding type-1 master"]
    #[inline(always)]
    pub fn nsa_usb_1(self) -> &'a mut W {
        self.variant(NSA_USB_A::NSA_USB_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Lock bit set by the TZ software for the USB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L_USB_A {
    #[doc = "0: No lock-the adjacent (next lower) bit can be written by the software."]
    L_USB_0 = 0,
    #[doc = "1: Lock-the adjacent (next lower) bit can't be written by the software."]
    L_USB_1 = 1,
}
impl From<L_USB_A> for bool {
    #[inline(always)]
    fn from(variant: L_USB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `L_USB`"]
pub type L_USB_R = crate::R<bool, L_USB_A>;
impl L_USB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> L_USB_A {
        match self.bits {
            false => L_USB_A::L_USB_0,
            true => L_USB_A::L_USB_1,
        }
    }
    #[doc = "Checks if the value of the field is `L_USB_0`"]
    #[inline(always)]
    pub fn is_l_usb_0(&self) -> bool {
        *self == L_USB_A::L_USB_0
    }
    #[doc = "Checks if the value of the field is `L_USB_1`"]
    #[inline(always)]
    pub fn is_l_usb_1(&self) -> bool {
        *self == L_USB_A::L_USB_1
    }
}
#[doc = "Write proxy for field `L_USB`"]
pub struct L_USB_W<'a> {
    w: &'a mut W,
}
impl<'a> L_USB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: L_USB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    #[inline(always)]
    pub fn l_usb_0(self) -> &'a mut W {
        self.variant(L_USB_A::L_USB_0)
    }
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    #[inline(always)]
    pub fn l_usb_1(self) -> &'a mut W {
        self.variant(L_USB_A::L_USB_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_dma(&self) -> NSA_DMA_R {
        NSA_DMA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub fn l_dma(&self) -> L_DMA_R {
        L_DMA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_lcdif(&self) -> NSA_LCDIF_R {
        NSA_LCDIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub fn l_lcdif(&self) -> L_LCDIF_R {
        L_LCDIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_csi(&self) -> NSA_CSI_R {
        NSA_CSI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub fn l_csi(&self) -> L_CSI_R {
        L_CSI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Non-Secure Access Policy indicator bit"]
    #[inline(always)]
    pub fn nsa_pxp(&self) -> NSA_PXP_R {
        NSA_PXP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub fn l_pxp(&self) -> L_PXP_R {
        L_PXP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_dcp(&self) -> NSA_DCP_R {
        NSA_DCP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub fn l_dcp(&self) -> L_DCP_R {
        L_DCP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_enet(&self) -> NSA_ENET_R {
        NSA_ENET_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Lock bit set by the TZ software for the ENET1 and ENET2"]
    #[inline(always)]
    pub fn l_enet(&self) -> L_ENET_R {
        L_ENET_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_usdhc1(&self) -> NSA_USDHC1_R {
        NSA_USDHC1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub fn l_usdhc1(&self) -> L_USDHC1_R {
        L_USDHC1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_usdhc2(&self) -> NSA_USDHC2_R {
        NSA_USDHC2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Lock bit set by the TZ software for the USDHC2"]
    #[inline(always)]
    pub fn l_usdhc2(&self) -> L_USDHC2_R {
        L_USDHC2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_tpsmp(&self) -> NSA_TPSMP_R {
        NSA_TPSMP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Lock bit set by the TZ software for the TPSMP"]
    #[inline(always)]
    pub fn l_tpsmp(&self) -> L_TPSMP_R {
        L_TPSMP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_usb(&self) -> NSA_USB_R {
        NSA_USB_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock bit set by the TZ software for the USB"]
    #[inline(always)]
    pub fn l_usb(&self) -> L_USB_R {
        L_USB_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_dma(&mut self) -> NSA_DMA_W {
        NSA_DMA_W { w: self }
    }
    #[doc = "Bit 3 - Lock bit set by the TZ software for the eDMA"]
    #[inline(always)]
    pub fn l_dma(&mut self) -> L_DMA_W {
        L_DMA_W { w: self }
    }
    #[doc = "Bit 4 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_lcdif(&mut self) -> NSA_LCDIF_W {
        NSA_LCDIF_W { w: self }
    }
    #[doc = "Bit 5 - Lock bit set by the TZ software for the LCDIF"]
    #[inline(always)]
    pub fn l_lcdif(&mut self) -> L_LCDIF_W {
        L_LCDIF_W { w: self }
    }
    #[doc = "Bit 6 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_csi(&mut self) -> NSA_CSI_W {
        NSA_CSI_W { w: self }
    }
    #[doc = "Bit 7 - Lock bit set by the TZ software for the CSI"]
    #[inline(always)]
    pub fn l_csi(&mut self) -> L_CSI_W {
        L_CSI_W { w: self }
    }
    #[doc = "Bit 8 - Non-Secure Access Policy indicator bit"]
    #[inline(always)]
    pub fn nsa_pxp(&mut self) -> NSA_PXP_W {
        NSA_PXP_W { w: self }
    }
    #[doc = "Bit 9 - Lock bit set by the TZ software for the PXP"]
    #[inline(always)]
    pub fn l_pxp(&mut self) -> L_PXP_W {
        L_PXP_W { w: self }
    }
    #[doc = "Bit 10 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_dcp(&mut self) -> NSA_DCP_W {
        NSA_DCP_W { w: self }
    }
    #[doc = "Bit 11 - Lock bit set by the TZ software for the DCP"]
    #[inline(always)]
    pub fn l_dcp(&mut self) -> L_DCP_W {
        L_DCP_W { w: self }
    }
    #[doc = "Bit 14 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_enet(&mut self) -> NSA_ENET_W {
        NSA_ENET_W { w: self }
    }
    #[doc = "Bit 15 - Lock bit set by the TZ software for the ENET1 and ENET2"]
    #[inline(always)]
    pub fn l_enet(&mut self) -> L_ENET_W {
        L_ENET_W { w: self }
    }
    #[doc = "Bit 16 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_usdhc1(&mut self) -> NSA_USDHC1_W {
        NSA_USDHC1_W { w: self }
    }
    #[doc = "Bit 17 - Lock bit set by the TZ software for the USDHC1"]
    #[inline(always)]
    pub fn l_usdhc1(&mut self) -> L_USDHC1_W {
        L_USDHC1_W { w: self }
    }
    #[doc = "Bit 18 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_usdhc2(&mut self) -> NSA_USDHC2_W {
        NSA_USDHC2_W { w: self }
    }
    #[doc = "Bit 19 - Lock bit set by the TZ software for the USDHC2"]
    #[inline(always)]
    pub fn l_usdhc2(&mut self) -> L_USDHC2_W {
        L_USDHC2_W { w: self }
    }
    #[doc = "Bit 20 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_tpsmp(&mut self) -> NSA_TPSMP_W {
        NSA_TPSMP_W { w: self }
    }
    #[doc = "Bit 21 - Lock bit set by the TZ software for the TPSMP"]
    #[inline(always)]
    pub fn l_tpsmp(&mut self) -> L_TPSMP_W {
        L_TPSMP_W { w: self }
    }
    #[doc = "Bit 22 - Non-secure access policy indicator bit"]
    #[inline(always)]
    pub fn nsa_usb(&mut self) -> NSA_USB_W {
        NSA_USB_W { w: self }
    }
    #[doc = "Bit 23 - Lock bit set by the TZ software for the USB"]
    #[inline(always)]
    pub fn l_usb(&mut self) -> L_USB_W {
        L_USB_W { w: self }
    }
}
