#[doc = "Reader of register GPR34"]
pub type R = crate::R<u32, super::GPR34>;
#[doc = "Writer for register GPR34"]
pub type W = crate::W<u32, super::GPR34>;
#[doc = "Register GPR34 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR34 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIP_TEST_MUX_BOOT_PIN_SEL`"]
pub type SIP_TEST_MUX_BOOT_PIN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIP_TEST_MUX_BOOT_PIN_SEL`"]
pub struct SIP_TEST_MUX_BOOT_PIN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIP_TEST_MUX_BOOT_PIN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Enable SIP_TEST_MUX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIP_TEST_MUX_QSPI_SIP_EN_A {
    #[doc = "0: SIP_TEST_MUX is disabled"]
    SIP_TEST_MUX_QSPI_SIP_EN_0,
    #[doc = "1: SIP_TEST_MUX is enabled"]
    SIP_TEST_MUX_QSPI_SIP_EN_1,
}
impl From<SIP_TEST_MUX_QSPI_SIP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SIP_TEST_MUX_QSPI_SIP_EN_A) -> Self {
        match variant {
            SIP_TEST_MUX_QSPI_SIP_EN_A::SIP_TEST_MUX_QSPI_SIP_EN_0 => false,
            SIP_TEST_MUX_QSPI_SIP_EN_A::SIP_TEST_MUX_QSPI_SIP_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `SIP_TEST_MUX_QSPI_SIP_EN`"]
pub type SIP_TEST_MUX_QSPI_SIP_EN_R = crate::R<bool, SIP_TEST_MUX_QSPI_SIP_EN_A>;
impl SIP_TEST_MUX_QSPI_SIP_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIP_TEST_MUX_QSPI_SIP_EN_A {
        match self.bits {
            false => SIP_TEST_MUX_QSPI_SIP_EN_A::SIP_TEST_MUX_QSPI_SIP_EN_0,
            true => SIP_TEST_MUX_QSPI_SIP_EN_A::SIP_TEST_MUX_QSPI_SIP_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SIP_TEST_MUX_QSPI_SIP_EN_0`"]
    #[inline(always)]
    pub fn is_sip_test_mux_qspi_sip_en_0(&self) -> bool {
        *self == SIP_TEST_MUX_QSPI_SIP_EN_A::SIP_TEST_MUX_QSPI_SIP_EN_0
    }
    #[doc = "Checks if the value of the field is `SIP_TEST_MUX_QSPI_SIP_EN_1`"]
    #[inline(always)]
    pub fn is_sip_test_mux_qspi_sip_en_1(&self) -> bool {
        *self == SIP_TEST_MUX_QSPI_SIP_EN_A::SIP_TEST_MUX_QSPI_SIP_EN_1
    }
}
#[doc = "Write proxy for field `SIP_TEST_MUX_QSPI_SIP_EN`"]
pub struct SIP_TEST_MUX_QSPI_SIP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIP_TEST_MUX_QSPI_SIP_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIP_TEST_MUX_QSPI_SIP_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SIP_TEST_MUX is disabled"]
    #[inline(always)]
    pub fn sip_test_mux_qspi_sip_en_0(self) -> &'a mut W {
        self.variant(SIP_TEST_MUX_QSPI_SIP_EN_A::SIP_TEST_MUX_QSPI_SIP_EN_0)
    }
    #[doc = "SIP_TEST_MUX is enabled"]
    #[inline(always)]
    pub fn sip_test_mux_qspi_sip_en_1(self) -> &'a mut W {
        self.variant(SIP_TEST_MUX_QSPI_SIP_EN_A::SIP_TEST_MUX_QSPI_SIP_EN_1)
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
impl R {
    #[doc = "Bits 0:7 - Boot Pin select in SIP_TEST_MUX"]
    #[inline(always)]
    pub fn sip_test_mux_boot_pin_sel(&self) -> SIP_TEST_MUX_BOOT_PIN_SEL_R {
        SIP_TEST_MUX_BOOT_PIN_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Enable SIP_TEST_MUX"]
    #[inline(always)]
    pub fn sip_test_mux_qspi_sip_en(&self) -> SIP_TEST_MUX_QSPI_SIP_EN_R {
        SIP_TEST_MUX_QSPI_SIP_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Boot Pin select in SIP_TEST_MUX"]
    #[inline(always)]
    pub fn sip_test_mux_boot_pin_sel(&mut self) -> SIP_TEST_MUX_BOOT_PIN_SEL_W {
        SIP_TEST_MUX_BOOT_PIN_SEL_W { w: self }
    }
    #[doc = "Bit 8 - Enable SIP_TEST_MUX"]
    #[inline(always)]
    pub fn sip_test_mux_qspi_sip_en(&mut self) -> SIP_TEST_MUX_QSPI_SIP_EN_W {
        SIP_TEST_MUX_QSPI_SIP_EN_W { w: self }
    }
}
