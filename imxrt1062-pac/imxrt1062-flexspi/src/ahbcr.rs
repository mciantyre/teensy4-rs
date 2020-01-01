#[doc = "Reader of register AHBCR"]
pub type R = crate::R<u32, super::AHBCR>;
#[doc = "Writer for register AHBCR"]
pub type W = crate::W<u32, super::AHBCR>;
#[doc = "Register AHBCR `reset()`'s with value 0x18"]
impl crate::ResetValue for super::AHBCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x18
    }
}
#[doc = "Parallel mode enabled for AHB triggered Command (both read and write) .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APAREN_A {
    #[doc = "0: Flash will be accessed in Individual mode."]
    APAREN_0 = 0,
    #[doc = "1: Flash will be accessed in Parallel mode."]
    APAREN_1 = 1,
}
impl From<APAREN_A> for bool {
    #[inline(always)]
    fn from(variant: APAREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `APAREN`"]
pub type APAREN_R = crate::R<bool, APAREN_A>;
impl APAREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APAREN_A {
        match self.bits {
            false => APAREN_A::APAREN_0,
            true => APAREN_A::APAREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `APAREN_0`"]
    #[inline(always)]
    pub fn is_aparen_0(&self) -> bool {
        *self == APAREN_A::APAREN_0
    }
    #[doc = "Checks if the value of the field is `APAREN_1`"]
    #[inline(always)]
    pub fn is_aparen_1(&self) -> bool {
        *self == APAREN_A::APAREN_1
    }
}
#[doc = "Write proxy for field `APAREN`"]
pub struct APAREN_W<'a> {
    w: &'a mut W,
}
impl<'a> APAREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APAREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flash will be accessed in Individual mode."]
    #[inline(always)]
    pub fn aparen_0(self) -> &'a mut W {
        self.variant(APAREN_A::APAREN_0)
    }
    #[doc = "Flash will be accessed in Parallel mode."]
    #[inline(always)]
    pub fn aparen_1(self) -> &'a mut W {
        self.variant(APAREN_A::APAREN_1)
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
#[doc = "Enable AHB bus cachable read access support.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHABLEEN_A {
    #[doc = "0: Disabled. When there is AHB bus cachable read access, FlexSPI will not check whether it hit AHB TX Buffer."]
    CACHABLEEN_0 = 0,
    #[doc = "1: Enabled. When there is AHB bus cachable read access, FlexSPI will check whether it hit AHB TX Buffer first."]
    CACHABLEEN_1 = 1,
}
impl From<CACHABLEEN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHABLEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHABLEEN`"]
pub type CACHABLEEN_R = crate::R<bool, CACHABLEEN_A>;
impl CACHABLEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHABLEEN_A {
        match self.bits {
            false => CACHABLEEN_A::CACHABLEEN_0,
            true => CACHABLEEN_A::CACHABLEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CACHABLEEN_0`"]
    #[inline(always)]
    pub fn is_cachableen_0(&self) -> bool {
        *self == CACHABLEEN_A::CACHABLEEN_0
    }
    #[doc = "Checks if the value of the field is `CACHABLEEN_1`"]
    #[inline(always)]
    pub fn is_cachableen_1(&self) -> bool {
        *self == CACHABLEEN_A::CACHABLEEN_1
    }
}
#[doc = "Write proxy for field `CACHABLEEN`"]
pub struct CACHABLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHABLEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHABLEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. When there is AHB bus cachable read access, FlexSPI will not check whether it hit AHB TX Buffer."]
    #[inline(always)]
    pub fn cachableen_0(self) -> &'a mut W {
        self.variant(CACHABLEEN_A::CACHABLEEN_0)
    }
    #[doc = "Enabled. When there is AHB bus cachable read access, FlexSPI will check whether it hit AHB TX Buffer first."]
    #[inline(always)]
    pub fn cachableen_1(self) -> &'a mut W {
        self.variant(CACHABLEEN_A::CACHABLEEN_1)
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
#[doc = "Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFFERABLEEN_A {
    #[doc = "0: Disabled. For all AHB write access (no matter bufferable or non-bufferable ), FlexSPI will return AHB Bus ready after all data is transmitted to External device and AHB command finished."]
    BUFFERABLEEN_0 = 0,
    #[doc = "1: Enabled. For AHB bufferable write access, FlexSPI will return AHB Bus ready when the AHB command is granted by arbitrator and will not wait for AHB command finished."]
    BUFFERABLEEN_1 = 1,
}
impl From<BUFFERABLEEN_A> for bool {
    #[inline(always)]
    fn from(variant: BUFFERABLEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BUFFERABLEEN`"]
pub type BUFFERABLEEN_R = crate::R<bool, BUFFERABLEEN_A>;
impl BUFFERABLEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFFERABLEEN_A {
        match self.bits {
            false => BUFFERABLEEN_A::BUFFERABLEEN_0,
            true => BUFFERABLEEN_A::BUFFERABLEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUFFERABLEEN_0`"]
    #[inline(always)]
    pub fn is_bufferableen_0(&self) -> bool {
        *self == BUFFERABLEEN_A::BUFFERABLEEN_0
    }
    #[doc = "Checks if the value of the field is `BUFFERABLEEN_1`"]
    #[inline(always)]
    pub fn is_bufferableen_1(&self) -> bool {
        *self == BUFFERABLEEN_A::BUFFERABLEEN_1
    }
}
#[doc = "Write proxy for field `BUFFERABLEEN`"]
pub struct BUFFERABLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFERABLEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFFERABLEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. For all AHB write access (no matter bufferable or non-bufferable ), FlexSPI will return AHB Bus ready after all data is transmitted to External device and AHB command finished."]
    #[inline(always)]
    pub fn bufferableen_0(self) -> &'a mut W {
        self.variant(BUFFERABLEEN_A::BUFFERABLEEN_0)
    }
    #[doc = "Enabled. For AHB bufferable write access, FlexSPI will return AHB Bus ready when the AHB command is granted by arbitrator and will not wait for AHB command finished."]
    #[inline(always)]
    pub fn bufferableen_1(self) -> &'a mut W {
        self.variant(BUFFERABLEEN_A::BUFFERABLEEN_1)
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
#[doc = "Reader of field `PREFETCHEN`"]
pub type PREFETCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREFETCHEN`"]
pub struct PREFETCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFETCHEN_W<'a> {
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
#[doc = "AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READADDROPT_A {
    #[doc = "0: There is AHB read burst start address alignment limitation when flash is accessed in parallel mode or flash is wordaddressable."]
    READADDROPT_0 = 0,
    #[doc = "1: There is no AHB read burst start address alignment limitation. FlexSPI will fetch more datas than AHB burst required to meet the alignment requirement."]
    READADDROPT_1 = 1,
}
impl From<READADDROPT_A> for bool {
    #[inline(always)]
    fn from(variant: READADDROPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READADDROPT`"]
pub type READADDROPT_R = crate::R<bool, READADDROPT_A>;
impl READADDROPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READADDROPT_A {
        match self.bits {
            false => READADDROPT_A::READADDROPT_0,
            true => READADDROPT_A::READADDROPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `READADDROPT_0`"]
    #[inline(always)]
    pub fn is_readaddropt_0(&self) -> bool {
        *self == READADDROPT_A::READADDROPT_0
    }
    #[doc = "Checks if the value of the field is `READADDROPT_1`"]
    #[inline(always)]
    pub fn is_readaddropt_1(&self) -> bool {
        *self == READADDROPT_A::READADDROPT_1
    }
}
#[doc = "Write proxy for field `READADDROPT`"]
pub struct READADDROPT_W<'a> {
    w: &'a mut W,
}
impl<'a> READADDROPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READADDROPT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "There is AHB read burst start address alignment limitation when flash is accessed in parallel mode or flash is wordaddressable."]
    #[inline(always)]
    pub fn readaddropt_0(self) -> &'a mut W {
        self.variant(READADDROPT_A::READADDROPT_0)
    }
    #[doc = "There is no AHB read burst start address alignment limitation. FlexSPI will fetch more datas than AHB burst required to meet the alignment requirement."]
    #[inline(always)]
    pub fn readaddropt_1(self) -> &'a mut W {
        self.variant(READADDROPT_A::READADDROPT_1)
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
impl R {
    #[doc = "Bit 0 - Parallel mode enabled for AHB triggered Command (both read and write) ."]
    #[inline(always)]
    pub fn aparen(&self) -> APAREN_R {
        APAREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable AHB bus cachable read access support."]
    #[inline(always)]
    pub fn cachableen(&self) -> CACHABLEEN_R {
        CACHABLEEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
    #[inline(always)]
    pub fn bufferableen(&self) -> BUFFERABLEEN_R {
        BUFFERABLEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AHB Read Prefetch Enable."]
    #[inline(always)]
    pub fn prefetchen(&self) -> PREFETCHEN_R {
        PREFETCHEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
    #[inline(always)]
    pub fn readaddropt(&self) -> READADDROPT_R {
        READADDROPT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parallel mode enabled for AHB triggered Command (both read and write) ."]
    #[inline(always)]
    pub fn aparen(&mut self) -> APAREN_W {
        APAREN_W { w: self }
    }
    #[doc = "Bit 3 - Enable AHB bus cachable read access support."]
    #[inline(always)]
    pub fn cachableen(&mut self) -> CACHABLEEN_W {
        CACHABLEEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
    #[inline(always)]
    pub fn bufferableen(&mut self) -> BUFFERABLEEN_W {
        BUFFERABLEEN_W { w: self }
    }
    #[doc = "Bit 5 - AHB Read Prefetch Enable."]
    #[inline(always)]
    pub fn prefetchen(&mut self) -> PREFETCHEN_W {
        PREFETCHEN_W { w: self }
    }
    #[doc = "Bit 6 - AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
    #[inline(always)]
    pub fn readaddropt(&mut self) -> READADDROPT_W {
        READADDROPT_W { w: self }
    }
}
