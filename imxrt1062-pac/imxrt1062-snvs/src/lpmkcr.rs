#[doc = "Reader of register LPMKCR"]
pub type R = crate::R<u32, super::LPMKCR>;
#[doc = "Writer for register LPMKCR"]
pub type W = crate::W<u32, super::LPMKCR>;
#[doc = "Register LPMKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LPMKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MASTER_KEY_SEL_A {
    #[doc = "0: Select one time programmable master key."]
    MASTER_KEY_SEL_0 = 0,
    #[doc = "2: Select zeroizable master key when MKS_EN bit is set ."]
    MASTER_KEY_SEL_2 = 2,
    #[doc = "3: Select combined master key when MKS_EN bit is set ."]
    MASTER_KEY_SEL_3 = 3,
}
impl From<MASTER_KEY_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MASTER_KEY_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MASTER_KEY_SEL`"]
pub type MASTER_KEY_SEL_R = crate::R<u8, MASTER_KEY_SEL_A>;
impl MASTER_KEY_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MASTER_KEY_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MASTER_KEY_SEL_A::MASTER_KEY_SEL_0),
            2 => Val(MASTER_KEY_SEL_A::MASTER_KEY_SEL_2),
            3 => Val(MASTER_KEY_SEL_A::MASTER_KEY_SEL_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_KEY_SEL_0`"]
    #[inline(always)]
    pub fn is_master_key_sel_0(&self) -> bool {
        *self == MASTER_KEY_SEL_A::MASTER_KEY_SEL_0
    }
    #[doc = "Checks if the value of the field is `MASTER_KEY_SEL_2`"]
    #[inline(always)]
    pub fn is_master_key_sel_2(&self) -> bool {
        *self == MASTER_KEY_SEL_A::MASTER_KEY_SEL_2
    }
    #[doc = "Checks if the value of the field is `MASTER_KEY_SEL_3`"]
    #[inline(always)]
    pub fn is_master_key_sel_3(&self) -> bool {
        *self == MASTER_KEY_SEL_A::MASTER_KEY_SEL_3
    }
}
#[doc = "Write proxy for field `MASTER_KEY_SEL`"]
pub struct MASTER_KEY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_KEY_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASTER_KEY_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select one time programmable master key."]
    #[inline(always)]
    pub fn master_key_sel_0(self) -> &'a mut W {
        self.variant(MASTER_KEY_SEL_A::MASTER_KEY_SEL_0)
    }
    #[doc = "Select zeroizable master key when MKS_EN bit is set ."]
    #[inline(always)]
    pub fn master_key_sel_2(self) -> &'a mut W {
        self.variant(MASTER_KEY_SEL_A::MASTER_KEY_SEL_2)
    }
    #[doc = "Select combined master key when MKS_EN bit is set ."]
    #[inline(always)]
    pub fn master_key_sel_3(self) -> &'a mut W {
        self.variant(MASTER_KEY_SEL_A::MASTER_KEY_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_HWP_A {
    #[doc = "0: ZMK is in the software programming mode."]
    ZMK_HWP_0 = 0,
    #[doc = "1: ZMK is in the hardware programming mode."]
    ZMK_HWP_1 = 1,
}
impl From<ZMK_HWP_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_HWP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ZMK_HWP`"]
pub type ZMK_HWP_R = crate::R<bool, ZMK_HWP_A>;
impl ZMK_HWP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_HWP_A {
        match self.bits {
            false => ZMK_HWP_A::ZMK_HWP_0,
            true => ZMK_HWP_A::ZMK_HWP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_HWP_0`"]
    #[inline(always)]
    pub fn is_zmk_hwp_0(&self) -> bool {
        *self == ZMK_HWP_A::ZMK_HWP_0
    }
    #[doc = "Checks if the value of the field is `ZMK_HWP_1`"]
    #[inline(always)]
    pub fn is_zmk_hwp_1(&self) -> bool {
        *self == ZMK_HWP_A::ZMK_HWP_1
    }
}
#[doc = "Write proxy for field `ZMK_HWP`"]
pub struct ZMK_HWP_W<'a> {
    w: &'a mut W,
}
impl<'a> ZMK_HWP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZMK_HWP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ZMK is in the software programming mode."]
    #[inline(always)]
    pub fn zmk_hwp_0(self) -> &'a mut W {
        self.variant(ZMK_HWP_A::ZMK_HWP_0)
    }
    #[doc = "ZMK is in the hardware programming mode."]
    #[inline(always)]
    pub fn zmk_hwp_1(self) -> &'a mut W {
        self.variant(ZMK_HWP_A::ZMK_HWP_1)
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
#[doc = "Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_VAL_A {
    #[doc = "0: ZMK is not valid."]
    ZMK_VAL_0 = 0,
    #[doc = "1: ZMK is valid."]
    ZMK_VAL_1 = 1,
}
impl From<ZMK_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_VAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ZMK_VAL`"]
pub type ZMK_VAL_R = crate::R<bool, ZMK_VAL_A>;
impl ZMK_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_VAL_A {
        match self.bits {
            false => ZMK_VAL_A::ZMK_VAL_0,
            true => ZMK_VAL_A::ZMK_VAL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_VAL_0`"]
    #[inline(always)]
    pub fn is_zmk_val_0(&self) -> bool {
        *self == ZMK_VAL_A::ZMK_VAL_0
    }
    #[doc = "Checks if the value of the field is `ZMK_VAL_1`"]
    #[inline(always)]
    pub fn is_zmk_val_1(&self) -> bool {
        *self == ZMK_VAL_A::ZMK_VAL_1
    }
}
#[doc = "Write proxy for field `ZMK_VAL`"]
pub struct ZMK_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ZMK_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZMK_VAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ZMK is not valid."]
    #[inline(always)]
    pub fn zmk_val_0(self) -> &'a mut W {
        self.variant(ZMK_VAL_A::ZMK_VAL_0)
    }
    #[doc = "ZMK is valid."]
    #[inline(always)]
    pub fn zmk_val_1(self) -> &'a mut W {
        self.variant(ZMK_VAL_A::ZMK_VAL_1)
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
#[doc = "Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZMK_ECC_EN_A {
    #[doc = "0: ZMK ECC check is disabled."]
    ZMK_ECC_EN_0 = 0,
    #[doc = "1: ZMK ECC check is enabled."]
    ZMK_ECC_EN_1 = 1,
}
impl From<ZMK_ECC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ZMK_ECC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ZMK_ECC_EN`"]
pub type ZMK_ECC_EN_R = crate::R<bool, ZMK_ECC_EN_A>;
impl ZMK_ECC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZMK_ECC_EN_A {
        match self.bits {
            false => ZMK_ECC_EN_A::ZMK_ECC_EN_0,
            true => ZMK_ECC_EN_A::ZMK_ECC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ZMK_ECC_EN_0`"]
    #[inline(always)]
    pub fn is_zmk_ecc_en_0(&self) -> bool {
        *self == ZMK_ECC_EN_A::ZMK_ECC_EN_0
    }
    #[doc = "Checks if the value of the field is `ZMK_ECC_EN_1`"]
    #[inline(always)]
    pub fn is_zmk_ecc_en_1(&self) -> bool {
        *self == ZMK_ECC_EN_A::ZMK_ECC_EN_1
    }
}
#[doc = "Write proxy for field `ZMK_ECC_EN`"]
pub struct ZMK_ECC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZMK_ECC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZMK_ECC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ZMK ECC check is disabled."]
    #[inline(always)]
    pub fn zmk_ecc_en_0(self) -> &'a mut W {
        self.variant(ZMK_ECC_EN_A::ZMK_ECC_EN_0)
    }
    #[doc = "ZMK ECC check is enabled."]
    #[inline(always)]
    pub fn zmk_ecc_en_1(self) -> &'a mut W {
        self.variant(ZMK_ECC_EN_A::ZMK_ECC_EN_1)
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
#[doc = "Reader of field `ZMK_ECC_VALUE`"]
pub type ZMK_ECC_VALUE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:1 - Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
    #[inline(always)]
    pub fn master_key_sel(&self) -> MASTER_KEY_SEL_R {
        MASTER_KEY_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
    #[inline(always)]
    pub fn zmk_hwp(&self) -> ZMK_HWP_R {
        ZMK_HWP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
    #[inline(always)]
    pub fn zmk_val(&self) -> ZMK_VAL_R {
        ZMK_VAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
    #[inline(always)]
    pub fn zmk_ecc_en(&self) -> ZMK_ECC_EN_R {
        ZMK_ECC_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 7:15 - Zeroizable Master Key Error Correcting Code Value This field is automatically calculated and set when one is written into ZMK_ECC_EN bit of this register"]
    #[inline(always)]
    pub fn zmk_ecc_value(&self) -> ZMK_ECC_VALUE_R {
        ZMK_ECC_VALUE_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master Key Select These bits select the SNVS Master Key output when Master Key Select bits are enabled by MKS_EN bit in the HPCOMR"]
    #[inline(always)]
    pub fn master_key_sel(&mut self) -> MASTER_KEY_SEL_W {
        MASTER_KEY_SEL_W { w: self }
    }
    #[doc = "Bit 2 - Zeroizable Master Key hardware Programming mode When set, only the hardware key programming mechanism can set the ZMK and software cannot read it"]
    #[inline(always)]
    pub fn zmk_hwp(&mut self) -> ZMK_HWP_W {
        ZMK_HWP_W { w: self }
    }
    #[doc = "Bit 3 - Zeroizable Master Key Valid When set, the ZMK value can be selected by the master key control block for use by cryptographic modules"]
    #[inline(always)]
    pub fn zmk_val(&mut self) -> ZMK_VAL_W {
        ZMK_VAL_W { w: self }
    }
    #[doc = "Bit 4 - Zeroizable Master Key Error Correcting Code Check Enable Writing one to this field automatically calculates and sets the ZMK ECC value in the ZMK_ECC_VALUE field of this register"]
    #[inline(always)]
    pub fn zmk_ecc_en(&mut self) -> ZMK_ECC_EN_W {
        ZMK_ECC_EN_W { w: self }
    }
}
