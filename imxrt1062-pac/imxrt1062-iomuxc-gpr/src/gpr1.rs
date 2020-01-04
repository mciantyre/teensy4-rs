#[doc = "Reader of register GPR1"]
pub type R = crate::R<u32, super::GPR1>;
#[doc = "Writer for register GPR1"]
pub type W = crate::W<u32, super::GPR1>;
#[doc = "Register GPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SAI1 MCLK1 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI1_MCLK1_SEL_A {
    #[doc = "0: ccm.ssi1_clk_root"]
    SAI1_MCLK1_SEL_0 = 0,
    #[doc = "1: ccm.ssi2_clk_root"]
    SAI1_MCLK1_SEL_1 = 1,
    #[doc = "2: ccm.ssi3_clk_root"]
    SAI1_MCLK1_SEL_2 = 2,
    #[doc = "3: iomux.sai1_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_3 = 3,
    #[doc = "4: iomux.sai2_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_4 = 4,
    #[doc = "5: iomux.sai3_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_5 = 5,
}
impl From<SAI1_MCLK1_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1_MCLK1_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI1_MCLK1_SEL`"]
pub type SAI1_MCLK1_SEL_R = crate::R<u8, SAI1_MCLK1_SEL_A>;
impl SAI1_MCLK1_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAI1_MCLK1_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_0),
            1 => Val(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_1),
            2 => Val(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_2),
            3 => Val(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_3),
            4 => Val(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_4),
            5 => Val(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_0`"]
    #[inline(always)]
    pub fn is_sai1_mclk1_sel_0(&self) -> bool {
        *self == SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_1`"]
    #[inline(always)]
    pub fn is_sai1_mclk1_sel_1(&self) -> bool {
        *self == SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_2`"]
    #[inline(always)]
    pub fn is_sai1_mclk1_sel_2(&self) -> bool {
        *self == SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_3`"]
    #[inline(always)]
    pub fn is_sai1_mclk1_sel_3(&self) -> bool {
        *self == SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_3
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_4`"]
    #[inline(always)]
    pub fn is_sai1_mclk1_sel_4(&self) -> bool {
        *self == SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_4
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_5`"]
    #[inline(always)]
    pub fn is_sai1_mclk1_sel_5(&self) -> bool {
        *self == SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_5
    }
}
#[doc = "Write proxy for field `SAI1_MCLK1_SEL`"]
pub struct SAI1_MCLK1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1_MCLK1_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1_MCLK1_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ccm.ssi1_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk1_sel_0(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_0)
    }
    #[doc = "ccm.ssi2_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk1_sel_1(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_1)
    }
    #[doc = "ccm.ssi3_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk1_sel_2(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_2)
    }
    #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
    #[inline(always)]
    pub fn sai1_mclk1_sel_3(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_3)
    }
    #[doc = "iomux.sai2_ipg_clk_sai_mclk"]
    #[inline(always)]
    pub fn sai1_mclk1_sel_4(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_4)
    }
    #[doc = "iomux.sai3_ipg_clk_sai_mclk"]
    #[inline(always)]
    pub fn sai1_mclk1_sel_5(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "SAI1 MCLK2 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI1_MCLK2_SEL_A {
    #[doc = "0: ccm.ssi1_clk_root"]
    SAI1_MCLK2_SEL_0 = 0,
    #[doc = "1: ccm.ssi2_clk_root"]
    SAI1_MCLK2_SEL_1 = 1,
    #[doc = "2: ccm.ssi3_clk_root"]
    SAI1_MCLK2_SEL_2 = 2,
    #[doc = "3: iomux.sai1_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_3 = 3,
    #[doc = "4: iomux.sai2_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_4 = 4,
    #[doc = "5: iomux.sai3_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_5 = 5,
}
impl From<SAI1_MCLK2_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1_MCLK2_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI1_MCLK2_SEL`"]
pub type SAI1_MCLK2_SEL_R = crate::R<u8, SAI1_MCLK2_SEL_A>;
impl SAI1_MCLK2_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAI1_MCLK2_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_0),
            1 => Val(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_1),
            2 => Val(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_2),
            3 => Val(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_3),
            4 => Val(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_4),
            5 => Val(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_0`"]
    #[inline(always)]
    pub fn is_sai1_mclk2_sel_0(&self) -> bool {
        *self == SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_1`"]
    #[inline(always)]
    pub fn is_sai1_mclk2_sel_1(&self) -> bool {
        *self == SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_2`"]
    #[inline(always)]
    pub fn is_sai1_mclk2_sel_2(&self) -> bool {
        *self == SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_3`"]
    #[inline(always)]
    pub fn is_sai1_mclk2_sel_3(&self) -> bool {
        *self == SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_3
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_4`"]
    #[inline(always)]
    pub fn is_sai1_mclk2_sel_4(&self) -> bool {
        *self == SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_4
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_5`"]
    #[inline(always)]
    pub fn is_sai1_mclk2_sel_5(&self) -> bool {
        *self == SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_5
    }
}
#[doc = "Write proxy for field `SAI1_MCLK2_SEL`"]
pub struct SAI1_MCLK2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1_MCLK2_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1_MCLK2_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ccm.ssi1_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk2_sel_0(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_0)
    }
    #[doc = "ccm.ssi2_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk2_sel_1(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_1)
    }
    #[doc = "ccm.ssi3_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk2_sel_2(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_2)
    }
    #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
    #[inline(always)]
    pub fn sai1_mclk2_sel_3(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_3)
    }
    #[doc = "iomux.sai2_ipg_clk_sai_mclk"]
    #[inline(always)]
    pub fn sai1_mclk2_sel_4(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_4)
    }
    #[doc = "iomux.sai3_ipg_clk_sai_mclk"]
    #[inline(always)]
    pub fn sai1_mclk2_sel_5(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "SAI1 MCLK3 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI1_MCLK3_SEL_A {
    #[doc = "0: ccm.spdif0_clk_root"]
    SAI1_MCLK3_SEL_0 = 0,
    #[doc = "1: iomux.spdif_tx_clk2"]
    SAI1_MCLK3_SEL_1 = 1,
    #[doc = "2: spdif.spdif_srclk"]
    SAI1_MCLK3_SEL_2 = 2,
    #[doc = "3: spdif.spdif_outclock"]
    SAI1_MCLK3_SEL_3 = 3,
}
impl From<SAI1_MCLK3_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1_MCLK3_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI1_MCLK3_SEL`"]
pub type SAI1_MCLK3_SEL_R = crate::R<u8, SAI1_MCLK3_SEL_A>;
impl SAI1_MCLK3_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1_MCLK3_SEL_A {
        match self.bits {
            0 => SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_0,
            1 => SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_1,
            2 => SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_2,
            3 => SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK3_SEL_0`"]
    #[inline(always)]
    pub fn is_sai1_mclk3_sel_0(&self) -> bool {
        *self == SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK3_SEL_1`"]
    #[inline(always)]
    pub fn is_sai1_mclk3_sel_1(&self) -> bool {
        *self == SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK3_SEL_2`"]
    #[inline(always)]
    pub fn is_sai1_mclk3_sel_2(&self) -> bool {
        *self == SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK3_SEL_3`"]
    #[inline(always)]
    pub fn is_sai1_mclk3_sel_3(&self) -> bool {
        *self == SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_3
    }
}
#[doc = "Write proxy for field `SAI1_MCLK3_SEL`"]
pub struct SAI1_MCLK3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1_MCLK3_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1_MCLK3_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ccm.spdif0_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk3_sel_0(self) -> &'a mut W {
        self.variant(SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_0)
    }
    #[doc = "iomux.spdif_tx_clk2"]
    #[inline(always)]
    pub fn sai1_mclk3_sel_1(self) -> &'a mut W {
        self.variant(SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_1)
    }
    #[doc = "spdif.spdif_srclk"]
    #[inline(always)]
    pub fn sai1_mclk3_sel_2(self) -> &'a mut W {
        self.variant(SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_2)
    }
    #[doc = "spdif.spdif_outclock"]
    #[inline(always)]
    pub fn sai1_mclk3_sel_3(self) -> &'a mut W {
        self.variant(SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "SAI2 MCLK3 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI2_MCLK3_SEL_A {
    #[doc = "0: ccm.spdif0_clk_root"]
    SAI2_MCLK3_SEL_0 = 0,
    #[doc = "1: iomux.spdif_tx_clk2"]
    SAI2_MCLK3_SEL_1 = 1,
    #[doc = "2: spdif.spdif_srclk"]
    SAI2_MCLK3_SEL_2 = 2,
    #[doc = "3: spdif.spdif_outclock"]
    SAI2_MCLK3_SEL_3 = 3,
}
impl From<SAI2_MCLK3_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI2_MCLK3_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI2_MCLK3_SEL`"]
pub type SAI2_MCLK3_SEL_R = crate::R<u8, SAI2_MCLK3_SEL_A>;
impl SAI2_MCLK3_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2_MCLK3_SEL_A {
        match self.bits {
            0 => SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_0,
            1 => SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_1,
            2 => SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_2,
            3 => SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK3_SEL_0`"]
    #[inline(always)]
    pub fn is_sai2_mclk3_sel_0(&self) -> bool {
        *self == SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK3_SEL_1`"]
    #[inline(always)]
    pub fn is_sai2_mclk3_sel_1(&self) -> bool {
        *self == SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK3_SEL_2`"]
    #[inline(always)]
    pub fn is_sai2_mclk3_sel_2(&self) -> bool {
        *self == SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK3_SEL_3`"]
    #[inline(always)]
    pub fn is_sai2_mclk3_sel_3(&self) -> bool {
        *self == SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_3
    }
}
#[doc = "Write proxy for field `SAI2_MCLK3_SEL`"]
pub struct SAI2_MCLK3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2_MCLK3_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI2_MCLK3_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ccm.spdif0_clk_root"]
    #[inline(always)]
    pub fn sai2_mclk3_sel_0(self) -> &'a mut W {
        self.variant(SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_0)
    }
    #[doc = "iomux.spdif_tx_clk2"]
    #[inline(always)]
    pub fn sai2_mclk3_sel_1(self) -> &'a mut W {
        self.variant(SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_1)
    }
    #[doc = "spdif.spdif_srclk"]
    #[inline(always)]
    pub fn sai2_mclk3_sel_2(self) -> &'a mut W {
        self.variant(SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_2)
    }
    #[doc = "spdif.spdif_outclock"]
    #[inline(always)]
    pub fn sai2_mclk3_sel_3(self) -> &'a mut W {
        self.variant(SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "SAI3 MCLK3 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI3_MCLK3_SEL_A {
    #[doc = "0: ccm.spdif0_clk_root"]
    SAI3_MCLK3_SEL_0 = 0,
    #[doc = "1: iomux.spdif_tx_clk2"]
    SAI3_MCLK3_SEL_1 = 1,
    #[doc = "2: spdif.spdif_srclk"]
    SAI3_MCLK3_SEL_2 = 2,
    #[doc = "3: spdif.spdif_outclock"]
    SAI3_MCLK3_SEL_3 = 3,
}
impl From<SAI3_MCLK3_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI3_MCLK3_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI3_MCLK3_SEL`"]
pub type SAI3_MCLK3_SEL_R = crate::R<u8, SAI3_MCLK3_SEL_A>;
impl SAI3_MCLK3_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI3_MCLK3_SEL_A {
        match self.bits {
            0 => SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_0,
            1 => SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_1,
            2 => SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_2,
            3 => SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK3_SEL_0`"]
    #[inline(always)]
    pub fn is_sai3_mclk3_sel_0(&self) -> bool {
        *self == SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK3_SEL_1`"]
    #[inline(always)]
    pub fn is_sai3_mclk3_sel_1(&self) -> bool {
        *self == SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK3_SEL_2`"]
    #[inline(always)]
    pub fn is_sai3_mclk3_sel_2(&self) -> bool {
        *self == SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK3_SEL_3`"]
    #[inline(always)]
    pub fn is_sai3_mclk3_sel_3(&self) -> bool {
        *self == SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_3
    }
}
#[doc = "Write proxy for field `SAI3_MCLK3_SEL`"]
pub struct SAI3_MCLK3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI3_MCLK3_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI3_MCLK3_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ccm.spdif0_clk_root"]
    #[inline(always)]
    pub fn sai3_mclk3_sel_0(self) -> &'a mut W {
        self.variant(SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_0)
    }
    #[doc = "iomux.spdif_tx_clk2"]
    #[inline(always)]
    pub fn sai3_mclk3_sel_1(self) -> &'a mut W {
        self.variant(SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_1)
    }
    #[doc = "spdif.spdif_srclk"]
    #[inline(always)]
    pub fn sai3_mclk3_sel_2(self) -> &'a mut W {
        self.variant(SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_2)
    }
    #[doc = "spdif.spdif_outclock"]
    #[inline(always)]
    pub fn sai3_mclk3_sel_3(self) -> &'a mut W {
        self.variant(SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Global interrupt \"0\" bit (connected to ARM M7 IRQ#0 and GPC)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GINT_A {
    #[doc = "0: Global interrupt request is not asserted."]
    GINT_0 = 0,
    #[doc = "1: Global interrupt request is asserted."]
    GINT_1 = 1,
}
impl From<GINT_A> for bool {
    #[inline(always)]
    fn from(variant: GINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GINT`"]
pub type GINT_R = crate::R<bool, GINT_A>;
impl GINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GINT_A {
        match self.bits {
            false => GINT_A::GINT_0,
            true => GINT_A::GINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `GINT_0`"]
    #[inline(always)]
    pub fn is_gint_0(&self) -> bool {
        *self == GINT_A::GINT_0
    }
    #[doc = "Checks if the value of the field is `GINT_1`"]
    #[inline(always)]
    pub fn is_gint_1(&self) -> bool {
        *self == GINT_A::GINT_1
    }
}
#[doc = "Write proxy for field `GINT`"]
pub struct GINT_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Global interrupt request is not asserted."]
    #[inline(always)]
    pub fn gint_0(self) -> &'a mut W {
        self.variant(GINT_A::GINT_0)
    }
    #[doc = "Global interrupt request is asserted."]
    #[inline(always)]
    pub fn gint_1(self) -> &'a mut W {
        self.variant(GINT_A::GINT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "ENET1 reference clock mode select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET1_CLK_SEL_A {
    #[doc = "0: ENET1 TX reference clock driven by ref_enetpll. This clock is also output to pins via the IOMUX. ENET_REF_CLK1 function."]
    ENET1_CLK_SEL_0 = 0,
    #[doc = "1: Gets ENET1 TX reference clock from the ENET1_TX_CLK pin. In this use case, an external OSC provides the clock for both the external PHY and the internal controller."]
    ENET1_CLK_SEL_1 = 1,
}
impl From<ENET1_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ENET1_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENET1_CLK_SEL`"]
pub type ENET1_CLK_SEL_R = crate::R<bool, ENET1_CLK_SEL_A>;
impl ENET1_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET1_CLK_SEL_A {
        match self.bits {
            false => ENET1_CLK_SEL_A::ENET1_CLK_SEL_0,
            true => ENET1_CLK_SEL_A::ENET1_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET1_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_enet1_clk_sel_0(&self) -> bool {
        *self == ENET1_CLK_SEL_A::ENET1_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `ENET1_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_enet1_clk_sel_1(&self) -> bool {
        *self == ENET1_CLK_SEL_A::ENET1_CLK_SEL_1
    }
}
#[doc = "Write proxy for field `ENET1_CLK_SEL`"]
pub struct ENET1_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET1_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENET1_CLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ENET1 TX reference clock driven by ref_enetpll. This clock is also output to pins via the IOMUX. ENET_REF_CLK1 function."]
    #[inline(always)]
    pub fn enet1_clk_sel_0(self) -> &'a mut W {
        self.variant(ENET1_CLK_SEL_A::ENET1_CLK_SEL_0)
    }
    #[doc = "Gets ENET1 TX reference clock from the ENET1_TX_CLK pin. In this use case, an external OSC provides the clock for both the external PHY and the internal controller."]
    #[inline(always)]
    pub fn enet1_clk_sel_1(self) -> &'a mut W {
        self.variant(ENET1_CLK_SEL_A::ENET1_CLK_SEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "ENET2 reference clock mode select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET2_CLK_SEL_A {
    #[doc = "0: ENET2 TX reference clock driven by ref_enetpll. This clock is also output to pins via the IOMUX. ENET2_REF_CLK function."]
    ENET2_CLK_SEL_0 = 0,
    #[doc = "1: Gets ENET2 TX reference clock from the ENET2_TX_CLK pin. In this use case, an external OSC provides the clock for both the external PHY and the internal controller."]
    ENET2_CLK_SEL_1 = 1,
}
impl From<ENET2_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ENET2_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENET2_CLK_SEL`"]
pub type ENET2_CLK_SEL_R = crate::R<bool, ENET2_CLK_SEL_A>;
impl ENET2_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET2_CLK_SEL_A {
        match self.bits {
            false => ENET2_CLK_SEL_A::ENET2_CLK_SEL_0,
            true => ENET2_CLK_SEL_A::ENET2_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET2_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_enet2_clk_sel_0(&self) -> bool {
        *self == ENET2_CLK_SEL_A::ENET2_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `ENET2_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_enet2_clk_sel_1(&self) -> bool {
        *self == ENET2_CLK_SEL_A::ENET2_CLK_SEL_1
    }
}
#[doc = "Write proxy for field `ENET2_CLK_SEL`"]
pub struct ENET2_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET2_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENET2_CLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ENET2 TX reference clock driven by ref_enetpll. This clock is also output to pins via the IOMUX. ENET2_REF_CLK function."]
    #[inline(always)]
    pub fn enet2_clk_sel_0(self) -> &'a mut W {
        self.variant(ENET2_CLK_SEL_A::ENET2_CLK_SEL_0)
    }
    #[doc = "Gets ENET2 TX reference clock from the ENET2_TX_CLK pin. In this use case, an external OSC provides the clock for both the external PHY and the internal controller."]
    #[inline(always)]
    pub fn enet2_clk_sel_1(self) -> &'a mut W {
        self.variant(ENET2_CLK_SEL_A::ENET2_CLK_SEL_1)
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
#[doc = "USB Exposure mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_EXP_MODE_A {
    #[doc = "0: Exposure mode is disabled."]
    USB_EXP_MODE_0 = 0,
    #[doc = "1: Exposure mode is enabled."]
    USB_EXP_MODE_1 = 1,
}
impl From<USB_EXP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: USB_EXP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USB_EXP_MODE`"]
pub type USB_EXP_MODE_R = crate::R<bool, USB_EXP_MODE_A>;
impl USB_EXP_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_EXP_MODE_A {
        match self.bits {
            false => USB_EXP_MODE_A::USB_EXP_MODE_0,
            true => USB_EXP_MODE_A::USB_EXP_MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `USB_EXP_MODE_0`"]
    #[inline(always)]
    pub fn is_usb_exp_mode_0(&self) -> bool {
        *self == USB_EXP_MODE_A::USB_EXP_MODE_0
    }
    #[doc = "Checks if the value of the field is `USB_EXP_MODE_1`"]
    #[inline(always)]
    pub fn is_usb_exp_mode_1(&self) -> bool {
        *self == USB_EXP_MODE_A::USB_EXP_MODE_1
    }
}
#[doc = "Write proxy for field `USB_EXP_MODE`"]
pub struct USB_EXP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_EXP_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_EXP_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Exposure mode is disabled."]
    #[inline(always)]
    pub fn usb_exp_mode_0(self) -> &'a mut W {
        self.variant(USB_EXP_MODE_A::USB_EXP_MODE_0)
    }
    #[doc = "Exposure mode is enabled."]
    #[inline(always)]
    pub fn usb_exp_mode_1(self) -> &'a mut W {
        self.variant(USB_EXP_MODE_A::USB_EXP_MODE_1)
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
#[doc = "ENET1_TX_CLK data direction control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET1_TX_CLK_DIR_A {
    #[doc = "0: ENET1_TX_CLK output driver is disabled"]
    ENET1_TX_CLK_DIR_0 = 0,
    #[doc = "1: ENET1_TX_CLK output driver is enabled"]
    ENET1_TX_CLK_DIR_1 = 1,
}
impl From<ENET1_TX_CLK_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: ENET1_TX_CLK_DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENET1_TX_CLK_DIR`"]
pub type ENET1_TX_CLK_DIR_R = crate::R<bool, ENET1_TX_CLK_DIR_A>;
impl ENET1_TX_CLK_DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET1_TX_CLK_DIR_A {
        match self.bits {
            false => ENET1_TX_CLK_DIR_A::ENET1_TX_CLK_DIR_0,
            true => ENET1_TX_CLK_DIR_A::ENET1_TX_CLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET1_TX_CLK_DIR_0`"]
    #[inline(always)]
    pub fn is_enet1_tx_clk_dir_0(&self) -> bool {
        *self == ENET1_TX_CLK_DIR_A::ENET1_TX_CLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `ENET1_TX_CLK_DIR_1`"]
    #[inline(always)]
    pub fn is_enet1_tx_clk_dir_1(&self) -> bool {
        *self == ENET1_TX_CLK_DIR_A::ENET1_TX_CLK_DIR_1
    }
}
#[doc = "Write proxy for field `ENET1_TX_CLK_DIR`"]
pub struct ENET1_TX_CLK_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET1_TX_CLK_DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENET1_TX_CLK_DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ENET1_TX_CLK output driver is disabled"]
    #[inline(always)]
    pub fn enet1_tx_clk_dir_0(self) -> &'a mut W {
        self.variant(ENET1_TX_CLK_DIR_A::ENET1_TX_CLK_DIR_0)
    }
    #[doc = "ENET1_TX_CLK output driver is enabled"]
    #[inline(always)]
    pub fn enet1_tx_clk_dir_1(self) -> &'a mut W {
        self.variant(ENET1_TX_CLK_DIR_A::ENET1_TX_CLK_DIR_1)
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
#[doc = "ENET2_TX_CLK data direction control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET2_TX_CLK_DIR_A {
    #[doc = "0: ENET2_TX_CLK output driver is disabled"]
    ENET2_TX_CLK_DIR_0 = 0,
    #[doc = "1: ENET2_TX_CLK output driver is enabled"]
    ENET2_TX_CLK_DIR_1 = 1,
}
impl From<ENET2_TX_CLK_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: ENET2_TX_CLK_DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENET2_TX_CLK_DIR`"]
pub type ENET2_TX_CLK_DIR_R = crate::R<bool, ENET2_TX_CLK_DIR_A>;
impl ENET2_TX_CLK_DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET2_TX_CLK_DIR_A {
        match self.bits {
            false => ENET2_TX_CLK_DIR_A::ENET2_TX_CLK_DIR_0,
            true => ENET2_TX_CLK_DIR_A::ENET2_TX_CLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET2_TX_CLK_DIR_0`"]
    #[inline(always)]
    pub fn is_enet2_tx_clk_dir_0(&self) -> bool {
        *self == ENET2_TX_CLK_DIR_A::ENET2_TX_CLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `ENET2_TX_CLK_DIR_1`"]
    #[inline(always)]
    pub fn is_enet2_tx_clk_dir_1(&self) -> bool {
        *self == ENET2_TX_CLK_DIR_A::ENET2_TX_CLK_DIR_1
    }
}
#[doc = "Write proxy for field `ENET2_TX_CLK_DIR`"]
pub struct ENET2_TX_CLK_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET2_TX_CLK_DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENET2_TX_CLK_DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ENET2_TX_CLK output driver is disabled"]
    #[inline(always)]
    pub fn enet2_tx_clk_dir_0(self) -> &'a mut W {
        self.variant(ENET2_TX_CLK_DIR_A::ENET2_TX_CLK_DIR_0)
    }
    #[doc = "ENET2_TX_CLK output driver is enabled"]
    #[inline(always)]
    pub fn enet2_tx_clk_dir_1(self) -> &'a mut W {
        self.variant(ENET2_TX_CLK_DIR_A::ENET2_TX_CLK_DIR_1)
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
#[doc = "sai1.MCLK signal direction control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1_MCLK_DIR_A {
    #[doc = "0: sai1.MCLK is input signal"]
    SAI1_MCLK_DIR_0 = 0,
    #[doc = "1: sai1.MCLK is output signal"]
    SAI1_MCLK_DIR_1 = 1,
}
impl From<SAI1_MCLK_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1_MCLK_DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI1_MCLK_DIR`"]
pub type SAI1_MCLK_DIR_R = crate::R<bool, SAI1_MCLK_DIR_A>;
impl SAI1_MCLK_DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1_MCLK_DIR_A {
        match self.bits {
            false => SAI1_MCLK_DIR_A::SAI1_MCLK_DIR_0,
            true => SAI1_MCLK_DIR_A::SAI1_MCLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK_DIR_0`"]
    #[inline(always)]
    pub fn is_sai1_mclk_dir_0(&self) -> bool {
        *self == SAI1_MCLK_DIR_A::SAI1_MCLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK_DIR_1`"]
    #[inline(always)]
    pub fn is_sai1_mclk_dir_1(&self) -> bool {
        *self == SAI1_MCLK_DIR_A::SAI1_MCLK_DIR_1
    }
}
#[doc = "Write proxy for field `SAI1_MCLK_DIR`"]
pub struct SAI1_MCLK_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1_MCLK_DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1_MCLK_DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "sai1.MCLK is input signal"]
    #[inline(always)]
    pub fn sai1_mclk_dir_0(self) -> &'a mut W {
        self.variant(SAI1_MCLK_DIR_A::SAI1_MCLK_DIR_0)
    }
    #[doc = "sai1.MCLK is output signal"]
    #[inline(always)]
    pub fn sai1_mclk_dir_1(self) -> &'a mut W {
        self.variant(SAI1_MCLK_DIR_A::SAI1_MCLK_DIR_1)
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
#[doc = "sai2.MCLK signal direction control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2_MCLK_DIR_A {
    #[doc = "0: sai2.MCLK is input signal"]
    SAI2_MCLK_DIR_0 = 0,
    #[doc = "1: sai2.MCLK is output signal"]
    SAI2_MCLK_DIR_1 = 1,
}
impl From<SAI2_MCLK_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: SAI2_MCLK_DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI2_MCLK_DIR`"]
pub type SAI2_MCLK_DIR_R = crate::R<bool, SAI2_MCLK_DIR_A>;
impl SAI2_MCLK_DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2_MCLK_DIR_A {
        match self.bits {
            false => SAI2_MCLK_DIR_A::SAI2_MCLK_DIR_0,
            true => SAI2_MCLK_DIR_A::SAI2_MCLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK_DIR_0`"]
    #[inline(always)]
    pub fn is_sai2_mclk_dir_0(&self) -> bool {
        *self == SAI2_MCLK_DIR_A::SAI2_MCLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK_DIR_1`"]
    #[inline(always)]
    pub fn is_sai2_mclk_dir_1(&self) -> bool {
        *self == SAI2_MCLK_DIR_A::SAI2_MCLK_DIR_1
    }
}
#[doc = "Write proxy for field `SAI2_MCLK_DIR`"]
pub struct SAI2_MCLK_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2_MCLK_DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI2_MCLK_DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "sai2.MCLK is input signal"]
    #[inline(always)]
    pub fn sai2_mclk_dir_0(self) -> &'a mut W {
        self.variant(SAI2_MCLK_DIR_A::SAI2_MCLK_DIR_0)
    }
    #[doc = "sai2.MCLK is output signal"]
    #[inline(always)]
    pub fn sai2_mclk_dir_1(self) -> &'a mut W {
        self.variant(SAI2_MCLK_DIR_A::SAI2_MCLK_DIR_1)
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
#[doc = "sai3.MCLK signal direction control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3_MCLK_DIR_A {
    #[doc = "0: sai3.MCLK is input signal"]
    SAI3_MCLK_DIR_0 = 0,
    #[doc = "1: sai3.MCLK is output signal"]
    SAI3_MCLK_DIR_1 = 1,
}
impl From<SAI3_MCLK_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: SAI3_MCLK_DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI3_MCLK_DIR`"]
pub type SAI3_MCLK_DIR_R = crate::R<bool, SAI3_MCLK_DIR_A>;
impl SAI3_MCLK_DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI3_MCLK_DIR_A {
        match self.bits {
            false => SAI3_MCLK_DIR_A::SAI3_MCLK_DIR_0,
            true => SAI3_MCLK_DIR_A::SAI3_MCLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK_DIR_0`"]
    #[inline(always)]
    pub fn is_sai3_mclk_dir_0(&self) -> bool {
        *self == SAI3_MCLK_DIR_A::SAI3_MCLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK_DIR_1`"]
    #[inline(always)]
    pub fn is_sai3_mclk_dir_1(&self) -> bool {
        *self == SAI3_MCLK_DIR_A::SAI3_MCLK_DIR_1
    }
}
#[doc = "Write proxy for field `SAI3_MCLK_DIR`"]
pub struct SAI3_MCLK_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI3_MCLK_DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI3_MCLK_DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "sai3.MCLK is input signal"]
    #[inline(always)]
    pub fn sai3_mclk_dir_0(self) -> &'a mut W {
        self.variant(SAI3_MCLK_DIR_A::SAI3_MCLK_DIR_0)
    }
    #[doc = "sai3.MCLK is output signal"]
    #[inline(always)]
    pub fn sai3_mclk_dir_1(self) -> &'a mut W {
        self.variant(SAI3_MCLK_DIR_A::SAI3_MCLK_DIR_1)
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
#[doc = "Exclusive monitor response select of illegal command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXC_MON_A {
    #[doc = "0: OKAY response"]
    EXC_MON_0 = 0,
    #[doc = "1: SLVError response (default)"]
    EXC_MON_1 = 1,
}
impl From<EXC_MON_A> for bool {
    #[inline(always)]
    fn from(variant: EXC_MON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXC_MON`"]
pub type EXC_MON_R = crate::R<bool, EXC_MON_A>;
impl EXC_MON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXC_MON_A {
        match self.bits {
            false => EXC_MON_A::EXC_MON_0,
            true => EXC_MON_A::EXC_MON_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXC_MON_0`"]
    #[inline(always)]
    pub fn is_exc_mon_0(&self) -> bool {
        *self == EXC_MON_A::EXC_MON_0
    }
    #[doc = "Checks if the value of the field is `EXC_MON_1`"]
    #[inline(always)]
    pub fn is_exc_mon_1(&self) -> bool {
        *self == EXC_MON_A::EXC_MON_1
    }
}
#[doc = "Write proxy for field `EXC_MON`"]
pub struct EXC_MON_W<'a> {
    w: &'a mut W,
}
impl<'a> EXC_MON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXC_MON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OKAY response"]
    #[inline(always)]
    pub fn exc_mon_0(self) -> &'a mut W {
        self.variant(EXC_MON_A::EXC_MON_0)
    }
    #[doc = "SLVError response (default)"]
    #[inline(always)]
    pub fn exc_mon_1(self) -> &'a mut W {
        self.variant(EXC_MON_A::EXC_MON_1)
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
#[doc = "ENET and ENET2 ipg_clk_s clock gating enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENET_IPG_CLK_S_EN_A {
    #[doc = "0: ipg_clk_s is gated when there is no IPS access"]
    ENET_IPG_CLK_S_EN_0 = 0,
    #[doc = "1: ipg_clk_s is always on"]
    ENET_IPG_CLK_S_EN_1 = 1,
}
impl From<ENET_IPG_CLK_S_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ENET_IPG_CLK_S_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENET_IPG_CLK_S_EN`"]
pub type ENET_IPG_CLK_S_EN_R = crate::R<bool, ENET_IPG_CLK_S_EN_A>;
impl ENET_IPG_CLK_S_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET_IPG_CLK_S_EN_A {
        match self.bits {
            false => ENET_IPG_CLK_S_EN_A::ENET_IPG_CLK_S_EN_0,
            true => ENET_IPG_CLK_S_EN_A::ENET_IPG_CLK_S_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_IPG_CLK_S_EN_0`"]
    #[inline(always)]
    pub fn is_enet_ipg_clk_s_en_0(&self) -> bool {
        *self == ENET_IPG_CLK_S_EN_A::ENET_IPG_CLK_S_EN_0
    }
    #[doc = "Checks if the value of the field is `ENET_IPG_CLK_S_EN_1`"]
    #[inline(always)]
    pub fn is_enet_ipg_clk_s_en_1(&self) -> bool {
        *self == ENET_IPG_CLK_S_EN_A::ENET_IPG_CLK_S_EN_1
    }
}
#[doc = "Write proxy for field `ENET_IPG_CLK_S_EN`"]
pub struct ENET_IPG_CLK_S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENET_IPG_CLK_S_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENET_IPG_CLK_S_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ipg_clk_s is gated when there is no IPS access"]
    #[inline(always)]
    pub fn enet_ipg_clk_s_en_0(self) -> &'a mut W {
        self.variant(ENET_IPG_CLK_S_EN_A::ENET_IPG_CLK_S_EN_0)
    }
    #[doc = "ipg_clk_s is always on"]
    #[inline(always)]
    pub fn enet_ipg_clk_s_en_1(self) -> &'a mut W {
        self.variant(ENET_IPG_CLK_S_EN_A::ENET_IPG_CLK_S_EN_1)
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
#[doc = "ARM CM7 platform AHB clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM7_FORCE_HCLK_EN_A {
    #[doc = "0: AHB clock is not running (gated)"]
    CM7_FORCE_HCLK_EN_0 = 0,
    #[doc = "1: AHB clock is running (enabled)"]
    CM7_FORCE_HCLK_EN_1 = 1,
}
impl From<CM7_FORCE_HCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CM7_FORCE_HCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CM7_FORCE_HCLK_EN`"]
pub type CM7_FORCE_HCLK_EN_R = crate::R<bool, CM7_FORCE_HCLK_EN_A>;
impl CM7_FORCE_HCLK_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM7_FORCE_HCLK_EN_A {
        match self.bits {
            false => CM7_FORCE_HCLK_EN_A::CM7_FORCE_HCLK_EN_0,
            true => CM7_FORCE_HCLK_EN_A::CM7_FORCE_HCLK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CM7_FORCE_HCLK_EN_0`"]
    #[inline(always)]
    pub fn is_cm7_force_hclk_en_0(&self) -> bool {
        *self == CM7_FORCE_HCLK_EN_A::CM7_FORCE_HCLK_EN_0
    }
    #[doc = "Checks if the value of the field is `CM7_FORCE_HCLK_EN_1`"]
    #[inline(always)]
    pub fn is_cm7_force_hclk_en_1(&self) -> bool {
        *self == CM7_FORCE_HCLK_EN_A::CM7_FORCE_HCLK_EN_1
    }
}
#[doc = "Write proxy for field `CM7_FORCE_HCLK_EN`"]
pub struct CM7_FORCE_HCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CM7_FORCE_HCLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM7_FORCE_HCLK_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AHB clock is not running (gated)"]
    #[inline(always)]
    pub fn cm7_force_hclk_en_0(self) -> &'a mut W {
        self.variant(CM7_FORCE_HCLK_EN_A::CM7_FORCE_HCLK_EN_0)
    }
    #[doc = "AHB clock is running (enabled)"]
    #[inline(always)]
    pub fn cm7_force_hclk_en_1(self) -> &'a mut W {
        self.variant(CM7_FORCE_HCLK_EN_A::CM7_FORCE_HCLK_EN_1)
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
    #[doc = "Bits 0:2 - SAI1 MCLK1 source select"]
    #[inline(always)]
    pub fn sai1_mclk1_sel(&self) -> SAI1_MCLK1_SEL_R {
        SAI1_MCLK1_SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - SAI1 MCLK2 source select"]
    #[inline(always)]
    pub fn sai1_mclk2_sel(&self) -> SAI1_MCLK2_SEL_R {
        SAI1_MCLK2_SEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - SAI1 MCLK3 source select"]
    #[inline(always)]
    pub fn sai1_mclk3_sel(&self) -> SAI1_MCLK3_SEL_R {
        SAI1_MCLK3_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - SAI2 MCLK3 source select"]
    #[inline(always)]
    pub fn sai2_mclk3_sel(&self) -> SAI2_MCLK3_SEL_R {
        SAI2_MCLK3_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - SAI3 MCLK3 source select"]
    #[inline(always)]
    pub fn sai3_mclk3_sel(&self) -> SAI3_MCLK3_SEL_R {
        SAI3_MCLK3_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Global interrupt \"0\" bit (connected to ARM M7 IRQ#0 and GPC)"]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ENET1 reference clock mode select."]
    #[inline(always)]
    pub fn enet1_clk_sel(&self) -> ENET1_CLK_SEL_R {
        ENET1_CLK_SEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ENET2 reference clock mode select."]
    #[inline(always)]
    pub fn enet2_clk_sel(&self) -> ENET2_CLK_SEL_R {
        ENET2_CLK_SEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USB Exposure mode"]
    #[inline(always)]
    pub fn usb_exp_mode(&self) -> USB_EXP_MODE_R {
        USB_EXP_MODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ENET1_TX_CLK data direction control"]
    #[inline(always)]
    pub fn enet1_tx_clk_dir(&self) -> ENET1_TX_CLK_DIR_R {
        ENET1_TX_CLK_DIR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ENET2_TX_CLK data direction control"]
    #[inline(always)]
    pub fn enet2_tx_clk_dir(&self) -> ENET2_TX_CLK_DIR_R {
        ENET2_TX_CLK_DIR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - sai1.MCLK signal direction control"]
    #[inline(always)]
    pub fn sai1_mclk_dir(&self) -> SAI1_MCLK_DIR_R {
        SAI1_MCLK_DIR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - sai2.MCLK signal direction control"]
    #[inline(always)]
    pub fn sai2_mclk_dir(&self) -> SAI2_MCLK_DIR_R {
        SAI2_MCLK_DIR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - sai3.MCLK signal direction control"]
    #[inline(always)]
    pub fn sai3_mclk_dir(&self) -> SAI3_MCLK_DIR_R {
        SAI3_MCLK_DIR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Exclusive monitor response select of illegal command"]
    #[inline(always)]
    pub fn exc_mon(&self) -> EXC_MON_R {
        EXC_MON_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ENET and ENET2 ipg_clk_s clock gating enable"]
    #[inline(always)]
    pub fn enet_ipg_clk_s_en(&self) -> ENET_IPG_CLK_S_EN_R {
        ENET_IPG_CLK_S_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ARM CM7 platform AHB clock enable"]
    #[inline(always)]
    pub fn cm7_force_hclk_en(&self) -> CM7_FORCE_HCLK_EN_R {
        CM7_FORCE_HCLK_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAI1 MCLK1 source select"]
    #[inline(always)]
    pub fn sai1_mclk1_sel(&mut self) -> SAI1_MCLK1_SEL_W {
        SAI1_MCLK1_SEL_W { w: self }
    }
    #[doc = "Bits 3:5 - SAI1 MCLK2 source select"]
    #[inline(always)]
    pub fn sai1_mclk2_sel(&mut self) -> SAI1_MCLK2_SEL_W {
        SAI1_MCLK2_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - SAI1 MCLK3 source select"]
    #[inline(always)]
    pub fn sai1_mclk3_sel(&mut self) -> SAI1_MCLK3_SEL_W {
        SAI1_MCLK3_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - SAI2 MCLK3 source select"]
    #[inline(always)]
    pub fn sai2_mclk3_sel(&mut self) -> SAI2_MCLK3_SEL_W {
        SAI2_MCLK3_SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - SAI3 MCLK3 source select"]
    #[inline(always)]
    pub fn sai3_mclk3_sel(&mut self) -> SAI3_MCLK3_SEL_W {
        SAI3_MCLK3_SEL_W { w: self }
    }
    #[doc = "Bit 12 - Global interrupt \"0\" bit (connected to ARM M7 IRQ#0 and GPC)"]
    #[inline(always)]
    pub fn gint(&mut self) -> GINT_W {
        GINT_W { w: self }
    }
    #[doc = "Bit 13 - ENET1 reference clock mode select."]
    #[inline(always)]
    pub fn enet1_clk_sel(&mut self) -> ENET1_CLK_SEL_W {
        ENET1_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 14 - ENET2 reference clock mode select."]
    #[inline(always)]
    pub fn enet2_clk_sel(&mut self) -> ENET2_CLK_SEL_W {
        ENET2_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 15 - USB Exposure mode"]
    #[inline(always)]
    pub fn usb_exp_mode(&mut self) -> USB_EXP_MODE_W {
        USB_EXP_MODE_W { w: self }
    }
    #[doc = "Bit 17 - ENET1_TX_CLK data direction control"]
    #[inline(always)]
    pub fn enet1_tx_clk_dir(&mut self) -> ENET1_TX_CLK_DIR_W {
        ENET1_TX_CLK_DIR_W { w: self }
    }
    #[doc = "Bit 18 - ENET2_TX_CLK data direction control"]
    #[inline(always)]
    pub fn enet2_tx_clk_dir(&mut self) -> ENET2_TX_CLK_DIR_W {
        ENET2_TX_CLK_DIR_W { w: self }
    }
    #[doc = "Bit 19 - sai1.MCLK signal direction control"]
    #[inline(always)]
    pub fn sai1_mclk_dir(&mut self) -> SAI1_MCLK_DIR_W {
        SAI1_MCLK_DIR_W { w: self }
    }
    #[doc = "Bit 20 - sai2.MCLK signal direction control"]
    #[inline(always)]
    pub fn sai2_mclk_dir(&mut self) -> SAI2_MCLK_DIR_W {
        SAI2_MCLK_DIR_W { w: self }
    }
    #[doc = "Bit 21 - sai3.MCLK signal direction control"]
    #[inline(always)]
    pub fn sai3_mclk_dir(&mut self) -> SAI3_MCLK_DIR_W {
        SAI3_MCLK_DIR_W { w: self }
    }
    #[doc = "Bit 22 - Exclusive monitor response select of illegal command"]
    #[inline(always)]
    pub fn exc_mon(&mut self) -> EXC_MON_W {
        EXC_MON_W { w: self }
    }
    #[doc = "Bit 23 - ENET and ENET2 ipg_clk_s clock gating enable"]
    #[inline(always)]
    pub fn enet_ipg_clk_s_en(&mut self) -> ENET_IPG_CLK_S_EN_W {
        ENET_IPG_CLK_S_EN_W { w: self }
    }
    #[doc = "Bit 31 - ARM CM7 platform AHB clock enable"]
    #[inline(always)]
    pub fn cm7_force_hclk_en(&mut self) -> CM7_FORCE_HCLK_EN_W {
        CM7_FORCE_HCLK_EN_W { w: self }
    }
}
