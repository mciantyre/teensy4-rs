#[doc = "Reader of register CM7_DTCMCR"]
pub type R = crate::R<u32, super::CM7_DTCMCR>;
#[doc = "Writer for register CM7_DTCMCR"]
pub type W = crate::W<u32, super::CM7_DTCMCR>;
#[doc = "Register CM7_DTCMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CM7_DTCMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TCM enable. When a TCM is disabled all accesses are made to the AXIM interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: TCM disabled."]
    EN_0 = 0,
    #[doc = "1: TCM enabled."]
    EN_1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, EN_A>;
impl EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::EN_0,
            true => EN_A::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline(always)]
    pub fn is_en_0(&self) -> bool {
        *self == EN_A::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline(always)]
    pub fn is_en_1(&self) -> bool {
        *self == EN_A::EN_1
    }
}
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TCM disabled."]
    #[inline(always)]
    pub fn en_0(self) -> &'a mut W {
        self.variant(EN_A::EN_0)
    }
    #[doc = "TCM enabled."]
    #[inline(always)]
    pub fn en_1(self) -> &'a mut W {
        self.variant(EN_A::EN_1)
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
#[doc = "Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMW_A {
    #[doc = "0: RMW disabled."]
    RMW_0 = 0,
    #[doc = "1: RMW enabled."]
    RMW_1 = 1,
}
impl From<RMW_A> for bool {
    #[inline(always)]
    fn from(variant: RMW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RMW`"]
pub type RMW_R = crate::R<bool, RMW_A>;
impl RMW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMW_A {
        match self.bits {
            false => RMW_A::RMW_0,
            true => RMW_A::RMW_1,
        }
    }
    #[doc = "Checks if the value of the field is `RMW_0`"]
    #[inline(always)]
    pub fn is_rmw_0(&self) -> bool {
        *self == RMW_A::RMW_0
    }
    #[doc = "Checks if the value of the field is `RMW_1`"]
    #[inline(always)]
    pub fn is_rmw_1(&self) -> bool {
        *self == RMW_A::RMW_1
    }
}
#[doc = "Write proxy for field `RMW`"]
pub struct RMW_W<'a> {
    w: &'a mut W,
}
impl<'a> RMW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RMW disabled."]
    #[inline(always)]
    pub fn rmw_0(self) -> &'a mut W {
        self.variant(RMW_A::RMW_0)
    }
    #[doc = "RMW enabled."]
    #[inline(always)]
    pub fn rmw_1(self) -> &'a mut W {
        self.variant(RMW_A::RMW_1)
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
#[doc = "Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETEN_A {
    #[doc = "0: Retry phase disabled."]
    RETEN_0 = 0,
    #[doc = "1: Retry phase enabled."]
    RETEN_1 = 1,
}
impl From<RETEN_A> for bool {
    #[inline(always)]
    fn from(variant: RETEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RETEN`"]
pub type RETEN_R = crate::R<bool, RETEN_A>;
impl RETEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETEN_A {
        match self.bits {
            false => RETEN_A::RETEN_0,
            true => RETEN_A::RETEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RETEN_0`"]
    #[inline(always)]
    pub fn is_reten_0(&self) -> bool {
        *self == RETEN_A::RETEN_0
    }
    #[doc = "Checks if the value of the field is `RETEN_1`"]
    #[inline(always)]
    pub fn is_reten_1(&self) -> bool {
        *self == RETEN_A::RETEN_1
    }
}
#[doc = "Write proxy for field `RETEN`"]
pub struct RETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Retry phase disabled."]
    #[inline(always)]
    pub fn reten_0(self) -> &'a mut W {
        self.variant(RETEN_A::RETEN_0)
    }
    #[doc = "Retry phase enabled."]
    #[inline(always)]
    pub fn reten_1(self) -> &'a mut W {
        self.variant(RETEN_A::RETEN_1)
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
#[doc = "TCM size. Indicates the size of the relevant TCM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SZ_A {
    #[doc = "0: No TCM implemented."]
    SZ_0 = 0,
    #[doc = "3: 4KB."]
    SZ_3 = 3,
    #[doc = "4: 8KB."]
    SZ_4 = 4,
    #[doc = "5: 16KB."]
    SZ_5 = 5,
    #[doc = "6: 32KB."]
    SZ_6 = 6,
    #[doc = "7: 64KB."]
    SZ_7 = 7,
    #[doc = "8: 128KB."]
    SZ_8 = 8,
    #[doc = "9: 256KB."]
    SZ_9 = 9,
    #[doc = "10: 512KB."]
    SZ_10 = 10,
    #[doc = "11: 1MB."]
    SZ_11 = 11,
    #[doc = "12: 2MB."]
    SZ_12 = 12,
    #[doc = "13: 4MB."]
    SZ_13 = 13,
    #[doc = "14: 8MB."]
    SZ_14 = 14,
    #[doc = "15: 16MB."]
    SZ_15 = 15,
}
impl From<SZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SZ`"]
pub type SZ_R = crate::R<u8, SZ_A>;
impl SZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SZ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SZ_A::SZ_0),
            3 => Val(SZ_A::SZ_3),
            4 => Val(SZ_A::SZ_4),
            5 => Val(SZ_A::SZ_5),
            6 => Val(SZ_A::SZ_6),
            7 => Val(SZ_A::SZ_7),
            8 => Val(SZ_A::SZ_8),
            9 => Val(SZ_A::SZ_9),
            10 => Val(SZ_A::SZ_10),
            11 => Val(SZ_A::SZ_11),
            12 => Val(SZ_A::SZ_12),
            13 => Val(SZ_A::SZ_13),
            14 => Val(SZ_A::SZ_14),
            15 => Val(SZ_A::SZ_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SZ_0`"]
    #[inline(always)]
    pub fn is_sz_0(&self) -> bool {
        *self == SZ_A::SZ_0
    }
    #[doc = "Checks if the value of the field is `SZ_3`"]
    #[inline(always)]
    pub fn is_sz_3(&self) -> bool {
        *self == SZ_A::SZ_3
    }
    #[doc = "Checks if the value of the field is `SZ_4`"]
    #[inline(always)]
    pub fn is_sz_4(&self) -> bool {
        *self == SZ_A::SZ_4
    }
    #[doc = "Checks if the value of the field is `SZ_5`"]
    #[inline(always)]
    pub fn is_sz_5(&self) -> bool {
        *self == SZ_A::SZ_5
    }
    #[doc = "Checks if the value of the field is `SZ_6`"]
    #[inline(always)]
    pub fn is_sz_6(&self) -> bool {
        *self == SZ_A::SZ_6
    }
    #[doc = "Checks if the value of the field is `SZ_7`"]
    #[inline(always)]
    pub fn is_sz_7(&self) -> bool {
        *self == SZ_A::SZ_7
    }
    #[doc = "Checks if the value of the field is `SZ_8`"]
    #[inline(always)]
    pub fn is_sz_8(&self) -> bool {
        *self == SZ_A::SZ_8
    }
    #[doc = "Checks if the value of the field is `SZ_9`"]
    #[inline(always)]
    pub fn is_sz_9(&self) -> bool {
        *self == SZ_A::SZ_9
    }
    #[doc = "Checks if the value of the field is `SZ_10`"]
    #[inline(always)]
    pub fn is_sz_10(&self) -> bool {
        *self == SZ_A::SZ_10
    }
    #[doc = "Checks if the value of the field is `SZ_11`"]
    #[inline(always)]
    pub fn is_sz_11(&self) -> bool {
        *self == SZ_A::SZ_11
    }
    #[doc = "Checks if the value of the field is `SZ_12`"]
    #[inline(always)]
    pub fn is_sz_12(&self) -> bool {
        *self == SZ_A::SZ_12
    }
    #[doc = "Checks if the value of the field is `SZ_13`"]
    #[inline(always)]
    pub fn is_sz_13(&self) -> bool {
        *self == SZ_A::SZ_13
    }
    #[doc = "Checks if the value of the field is `SZ_14`"]
    #[inline(always)]
    pub fn is_sz_14(&self) -> bool {
        *self == SZ_A::SZ_14
    }
    #[doc = "Checks if the value of the field is `SZ_15`"]
    #[inline(always)]
    pub fn is_sz_15(&self) -> bool {
        *self == SZ_A::SZ_15
    }
}
impl R {
    #[doc = "Bit 0 - TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
    #[inline(always)]
    pub fn rmw(&self) -> RMW_R {
        RMW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
    #[inline(always)]
    pub fn reten(&self) -> RETEN_R {
        RETEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - TCM size. Indicates the size of the relevant TCM."]
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TCM enable. When a TCM is disabled all accesses are made to the AXIM interface."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Read-Modify-Write (RMW) enable. Indicates that all writes to TCM, that are not the full width of the TCM RAM, use a RMW sequence."]
    #[inline(always)]
    pub fn rmw(&mut self) -> RMW_W {
        RMW_W { w: self }
    }
    #[doc = "Bit 2 - Retry phase enable. When enabled the processor guarantees to honor the retry output on the corresponding TCM interface, re-executing the instruction which carried out the TCM access."]
    #[inline(always)]
    pub fn reten(&mut self) -> RETEN_W {
        RETEN_W { w: self }
    }
}
