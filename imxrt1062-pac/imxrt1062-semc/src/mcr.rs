#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0x1000_0002"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000_0002
    }
}
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIS_A {
    #[doc = "0: Module enabled"]
    MDIS_0 = 0,
    #[doc = "1: Module disabled."]
    MDIS_1 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MDIS`"]
pub type MDIS_R = crate::R<bool, MDIS_A>;
impl MDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::MDIS_0,
            true => MDIS_A::MDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MDIS_0`"]
    #[inline(always)]
    pub fn is_mdis_0(&self) -> bool {
        *self == MDIS_A::MDIS_0
    }
    #[doc = "Checks if the value of the field is `MDIS_1`"]
    #[inline(always)]
    pub fn is_mdis_1(&self) -> bool {
        *self == MDIS_A::MDIS_1
    }
}
#[doc = "Write proxy for field `MDIS`"]
pub struct MDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Module enabled"]
    #[inline(always)]
    pub fn mdis_0(self) -> &'a mut W {
        self.variant(MDIS_A::MDIS_0)
    }
    #[doc = "Module disabled."]
    #[inline(always)]
    pub fn mdis_1(self) -> &'a mut W {
        self.variant(MDIS_A::MDIS_1)
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
#[doc = "DQS (read strobe) mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQSMD_A {
    #[doc = "0: Dummy read strobe loopbacked internally"]
    DQSMD_0 = 0,
    #[doc = "1: Dummy read strobe loopbacked from DQS pad or DLL delay chain. Details information at descriptions of DQSSEL bit."]
    DQSMD_1 = 1,
}
impl From<DQSMD_A> for bool {
    #[inline(always)]
    fn from(variant: DQSMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DQSMD`"]
pub type DQSMD_R = crate::R<bool, DQSMD_A>;
impl DQSMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQSMD_A {
        match self.bits {
            false => DQSMD_A::DQSMD_0,
            true => DQSMD_A::DQSMD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DQSMD_0`"]
    #[inline(always)]
    pub fn is_dqsmd_0(&self) -> bool {
        *self == DQSMD_A::DQSMD_0
    }
    #[doc = "Checks if the value of the field is `DQSMD_1`"]
    #[inline(always)]
    pub fn is_dqsmd_1(&self) -> bool {
        *self == DQSMD_A::DQSMD_1
    }
}
#[doc = "Write proxy for field `DQSMD`"]
pub struct DQSMD_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQSMD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Dummy read strobe loopbacked internally"]
    #[inline(always)]
    pub fn dqsmd_0(self) -> &'a mut W {
        self.variant(DQSMD_A::DQSMD_0)
    }
    #[doc = "Dummy read strobe loopbacked from DQS pad or DLL delay chain. Details information at descriptions of DQSSEL bit."]
    #[inline(always)]
    pub fn dqsmd_1(self) -> &'a mut W {
        self.variant(DQSMD_A::DQSMD_1)
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
#[doc = "WAIT/RDY# polarity for NOR/PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPOL0_A {
    #[doc = "0: Low active"]
    WPOL0_0 = 0,
    #[doc = "1: High active"]
    WPOL0_1 = 1,
}
impl From<WPOL0_A> for bool {
    #[inline(always)]
    fn from(variant: WPOL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WPOL0`"]
pub type WPOL0_R = crate::R<bool, WPOL0_A>;
impl WPOL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPOL0_A {
        match self.bits {
            false => WPOL0_A::WPOL0_0,
            true => WPOL0_A::WPOL0_1,
        }
    }
    #[doc = "Checks if the value of the field is `WPOL0_0`"]
    #[inline(always)]
    pub fn is_wpol0_0(&self) -> bool {
        *self == WPOL0_A::WPOL0_0
    }
    #[doc = "Checks if the value of the field is `WPOL0_1`"]
    #[inline(always)]
    pub fn is_wpol0_1(&self) -> bool {
        *self == WPOL0_A::WPOL0_1
    }
}
#[doc = "Write proxy for field `WPOL0`"]
pub struct WPOL0_W<'a> {
    w: &'a mut W,
}
impl<'a> WPOL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPOL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low active"]
    #[inline(always)]
    pub fn wpol0_0(self) -> &'a mut W {
        self.variant(WPOL0_A::WPOL0_0)
    }
    #[doc = "High active"]
    #[inline(always)]
    pub fn wpol0_1(self) -> &'a mut W {
        self.variant(WPOL0_A::WPOL0_1)
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
#[doc = "WAIT/RDY# polarity for NAND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPOL1_A {
    #[doc = "0: Low active"]
    WPOL1_0 = 0,
    #[doc = "1: High active"]
    WPOL1_1 = 1,
}
impl From<WPOL1_A> for bool {
    #[inline(always)]
    fn from(variant: WPOL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WPOL1`"]
pub type WPOL1_R = crate::R<bool, WPOL1_A>;
impl WPOL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPOL1_A {
        match self.bits {
            false => WPOL1_A::WPOL1_0,
            true => WPOL1_A::WPOL1_1,
        }
    }
    #[doc = "Checks if the value of the field is `WPOL1_0`"]
    #[inline(always)]
    pub fn is_wpol1_0(&self) -> bool {
        *self == WPOL1_A::WPOL1_0
    }
    #[doc = "Checks if the value of the field is `WPOL1_1`"]
    #[inline(always)]
    pub fn is_wpol1_1(&self) -> bool {
        *self == WPOL1_A::WPOL1_1
    }
}
#[doc = "Write proxy for field `WPOL1`"]
pub struct WPOL1_W<'a> {
    w: &'a mut W,
}
impl<'a> WPOL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPOL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low active"]
    #[inline(always)]
    pub fn wpol1_0(self) -> &'a mut W {
        self.variant(WPOL1_A::WPOL1_0)
    }
    #[doc = "High active"]
    #[inline(always)]
    pub fn wpol1_1(self) -> &'a mut W {
        self.variant(WPOL1_A::WPOL1_1)
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
#[doc = "Select DQS source when DQSMD and DLLSEL both set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DQSSEL_A {
    #[doc = "0: SDRAM/NOR/SRAM read clock source is from DQS pad in synchronous mode."]
    DQSSEL_0 = 0,
    #[doc = "1: SDRAM/NOR/SRAM read clock source is from DLL delay chain in synchronous mode."]
    DQSSEL_1 = 1,
}
impl From<DQSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DQSSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DQSSEL`"]
pub type DQSSEL_R = crate::R<bool, DQSSEL_A>;
impl DQSSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQSSEL_A {
        match self.bits {
            false => DQSSEL_A::DQSSEL_0,
            true => DQSSEL_A::DQSSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DQSSEL_0`"]
    #[inline(always)]
    pub fn is_dqssel_0(&self) -> bool {
        *self == DQSSEL_A::DQSSEL_0
    }
    #[doc = "Checks if the value of the field is `DQSSEL_1`"]
    #[inline(always)]
    pub fn is_dqssel_1(&self) -> bool {
        *self == DQSSEL_A::DQSSEL_1
    }
}
#[doc = "Write proxy for field `DQSSEL`"]
pub struct DQSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DQSSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SDRAM/NOR/SRAM read clock source is from DQS pad in synchronous mode."]
    #[inline(always)]
    pub fn dqssel_0(self) -> &'a mut W {
        self.variant(DQSSEL_A::DQSSEL_0)
    }
    #[doc = "SDRAM/NOR/SRAM read clock source is from DLL delay chain in synchronous mode."]
    #[inline(always)]
    pub fn dqssel_1(self) -> &'a mut W {
        self.variant(DQSSEL_A::DQSSEL_1)
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
#[doc = "Select DLL delay chain clock input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLLSEL_A {
    #[doc = "0: DLL delay chain clock input is from NAND device's DQS pad. For NAND synchronous mode only."]
    DLLSEL_0 = 0,
    #[doc = "1: DLL delay chain clock input is from internal clock. For SDRAM, NOR and SRAM synchronous mode only."]
    DLLSEL_1 = 1,
}
impl From<DLLSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DLLSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLLSEL`"]
pub type DLLSEL_R = crate::R<bool, DLLSEL_A>;
impl DLLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLLSEL_A {
        match self.bits {
            false => DLLSEL_A::DLLSEL_0,
            true => DLLSEL_A::DLLSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DLLSEL_0`"]
    #[inline(always)]
    pub fn is_dllsel_0(&self) -> bool {
        *self == DLLSEL_A::DLLSEL_0
    }
    #[doc = "Checks if the value of the field is `DLLSEL_1`"]
    #[inline(always)]
    pub fn is_dllsel_1(&self) -> bool {
        *self == DLLSEL_A::DLLSEL_1
    }
}
#[doc = "Write proxy for field `DLLSEL`"]
pub struct DLLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DLLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLLSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DLL delay chain clock input is from NAND device's DQS pad. For NAND synchronous mode only."]
    #[inline(always)]
    pub fn dllsel_0(self) -> &'a mut W {
        self.variant(DLLSEL_A::DLLSEL_0)
    }
    #[doc = "DLL delay chain clock input is from internal clock. For SDRAM, NOR and SRAM synchronous mode only."]
    #[inline(always)]
    pub fn dllsel_1(self) -> &'a mut W {
        self.variant(DLLSEL_A::DLLSEL_1)
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
#[doc = "Reader of field `CTO`"]
pub type CTO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTO`"]
pub struct CTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Bus timeout cycles\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BTO_A {
    #[doc = "0: 255*1"]
    BTO_0 = 0,
    #[doc = "1: 255*2 - 255*2^30"]
    BTO_1 = 1,
    #[doc = "2: 255*2 - 255*2^30"]
    BTO_2 = 2,
    #[doc = "3: 255*2 - 255*2^30"]
    BTO_3 = 3,
    #[doc = "4: 255*2 - 255*2^30"]
    BTO_4 = 4,
    #[doc = "5: 255*2 - 255*2^30"]
    BTO_5 = 5,
    #[doc = "6: 255*2 - 255*2^30"]
    BTO_6 = 6,
    #[doc = "7: 255*2 - 255*2^30"]
    BTO_7 = 7,
    #[doc = "8: 255*2 - 255*2^30"]
    BTO_8 = 8,
    #[doc = "9: 255*2 - 255*2^30"]
    BTO_9 = 9,
    #[doc = "31: 255*2^31"]
    BTO_31 = 31,
}
impl From<BTO_A> for u8 {
    #[inline(always)]
    fn from(variant: BTO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BTO`"]
pub type BTO_R = crate::R<u8, BTO_A>;
impl BTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BTO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BTO_A::BTO_0),
            1 => Val(BTO_A::BTO_1),
            2 => Val(BTO_A::BTO_2),
            3 => Val(BTO_A::BTO_3),
            4 => Val(BTO_A::BTO_4),
            5 => Val(BTO_A::BTO_5),
            6 => Val(BTO_A::BTO_6),
            7 => Val(BTO_A::BTO_7),
            8 => Val(BTO_A::BTO_8),
            9 => Val(BTO_A::BTO_9),
            31 => Val(BTO_A::BTO_31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BTO_0`"]
    #[inline(always)]
    pub fn is_bto_0(&self) -> bool {
        *self == BTO_A::BTO_0
    }
    #[doc = "Checks if the value of the field is `BTO_1`"]
    #[inline(always)]
    pub fn is_bto_1(&self) -> bool {
        *self == BTO_A::BTO_1
    }
    #[doc = "Checks if the value of the field is `BTO_2`"]
    #[inline(always)]
    pub fn is_bto_2(&self) -> bool {
        *self == BTO_A::BTO_2
    }
    #[doc = "Checks if the value of the field is `BTO_3`"]
    #[inline(always)]
    pub fn is_bto_3(&self) -> bool {
        *self == BTO_A::BTO_3
    }
    #[doc = "Checks if the value of the field is `BTO_4`"]
    #[inline(always)]
    pub fn is_bto_4(&self) -> bool {
        *self == BTO_A::BTO_4
    }
    #[doc = "Checks if the value of the field is `BTO_5`"]
    #[inline(always)]
    pub fn is_bto_5(&self) -> bool {
        *self == BTO_A::BTO_5
    }
    #[doc = "Checks if the value of the field is `BTO_6`"]
    #[inline(always)]
    pub fn is_bto_6(&self) -> bool {
        *self == BTO_A::BTO_6
    }
    #[doc = "Checks if the value of the field is `BTO_7`"]
    #[inline(always)]
    pub fn is_bto_7(&self) -> bool {
        *self == BTO_A::BTO_7
    }
    #[doc = "Checks if the value of the field is `BTO_8`"]
    #[inline(always)]
    pub fn is_bto_8(&self) -> bool {
        *self == BTO_A::BTO_8
    }
    #[doc = "Checks if the value of the field is `BTO_9`"]
    #[inline(always)]
    pub fn is_bto_9(&self) -> bool {
        *self == BTO_A::BTO_9
    }
    #[doc = "Checks if the value of the field is `BTO_31`"]
    #[inline(always)]
    pub fn is_bto_31(&self) -> bool {
        *self == BTO_A::BTO_31
    }
}
#[doc = "Write proxy for field `BTO`"]
pub struct BTO_W<'a> {
    w: &'a mut W,
}
impl<'a> BTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BTO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "255*1"]
    #[inline(always)]
    pub fn bto_0(self) -> &'a mut W {
        self.variant(BTO_A::BTO_0)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline(always)]
    pub fn bto_1(self) -> &'a mut W {
        self.variant(BTO_A::BTO_1)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline(always)]
    pub fn bto_2(self) -> &'a mut W {
        self.variant(BTO_A::BTO_2)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline(always)]
    pub fn bto_3(self) -> &'a mut W {
        self.variant(BTO_A::BTO_3)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline(always)]
    pub fn bto_4(self) -> &'a mut W {
        self.variant(BTO_A::BTO_4)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline(always)]
    pub fn bto_5(self) -> &'a mut W {
        self.variant(BTO_A::BTO_5)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline(always)]
    pub fn bto_6(self) -> &'a mut W {
        self.variant(BTO_A::BTO_6)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline(always)]
    pub fn bto_7(self) -> &'a mut W {
        self.variant(BTO_A::BTO_7)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline(always)]
    pub fn bto_8(self) -> &'a mut W {
        self.variant(BTO_A::BTO_8)
    }
    #[doc = "255*2 - 255*2^30"]
    #[inline(always)]
    pub fn bto_9(self) -> &'a mut W {
        self.variant(BTO_A::BTO_9)
    }
    #[doc = "255*2^31"]
    #[inline(always)]
    pub fn bto_31(self) -> &'a mut W {
        self.variant(BTO_A::BTO_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DQS (read strobe) mode"]
    #[inline(always)]
    pub fn dqsmd(&self) -> DQSMD_R {
        DQSMD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WAIT/RDY# polarity for NOR/PSRAM"]
    #[inline(always)]
    pub fn wpol0(&self) -> WPOL0_R {
        WPOL0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - WAIT/RDY# polarity for NAND"]
    #[inline(always)]
    pub fn wpol1(&self) -> WPOL1_R {
        WPOL1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Select DQS source when DQSMD and DLLSEL both set."]
    #[inline(always)]
    pub fn dqssel(&self) -> DQSSEL_R {
        DQSSEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Select DLL delay chain clock input."]
    #[inline(always)]
    pub fn dllsel(&self) -> DLLSEL_R {
        DLLSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Command Execution timeout cycles"]
    #[inline(always)]
    pub fn cto(&self) -> CTO_R {
        CTO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Bus timeout cycles"]
    #[inline(always)]
    pub fn bto(&self) -> BTO_R {
        BTO_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&mut self) -> MDIS_W {
        MDIS_W { w: self }
    }
    #[doc = "Bit 2 - DQS (read strobe) mode"]
    #[inline(always)]
    pub fn dqsmd(&mut self) -> DQSMD_W {
        DQSMD_W { w: self }
    }
    #[doc = "Bit 6 - WAIT/RDY# polarity for NOR/PSRAM"]
    #[inline(always)]
    pub fn wpol0(&mut self) -> WPOL0_W {
        WPOL0_W { w: self }
    }
    #[doc = "Bit 7 - WAIT/RDY# polarity for NAND"]
    #[inline(always)]
    pub fn wpol1(&mut self) -> WPOL1_W {
        WPOL1_W { w: self }
    }
    #[doc = "Bit 10 - Select DQS source when DQSMD and DLLSEL both set."]
    #[inline(always)]
    pub fn dqssel(&mut self) -> DQSSEL_W {
        DQSSEL_W { w: self }
    }
    #[doc = "Bit 11 - Select DLL delay chain clock input."]
    #[inline(always)]
    pub fn dllsel(&mut self) -> DLLSEL_W {
        DLLSEL_W { w: self }
    }
    #[doc = "Bits 16:23 - Command Execution timeout cycles"]
    #[inline(always)]
    pub fn cto(&mut self) -> CTO_W {
        CTO_W { w: self }
    }
    #[doc = "Bits 24:28 - Bus timeout cycles"]
    #[inline(always)]
    pub fn bto(&mut self) -> BTO_W {
        BTO_W { w: self }
    }
}
