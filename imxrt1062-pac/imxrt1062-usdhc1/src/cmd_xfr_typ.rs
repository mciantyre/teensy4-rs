#[doc = "Reader of register CMD_XFR_TYP"]
pub type R = crate::R<u32, super::CMD_XFR_TYP>;
#[doc = "Writer for register CMD_XFR_TYP"]
pub type W = crate::W<u32, super::CMD_XFR_TYP>;
#[doc = "Register CMD_XFR_TYP `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD_XFR_TYP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSPTYP_A {
    #[doc = "0: No Response"]
    RSPTYP_0 = 0,
    #[doc = "1: Response Length 136"]
    RSPTYP_1 = 1,
    #[doc = "2: Response Length 48"]
    RSPTYP_2 = 2,
    #[doc = "3: Response Length 48, check Busy after response"]
    RSPTYP_3 = 3,
}
impl From<RSPTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: RSPTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSPTYP`"]
pub type RSPTYP_R = crate::R<u8, RSPTYP_A>;
impl RSPTYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPTYP_A {
        match self.bits {
            0 => RSPTYP_A::RSPTYP_0,
            1 => RSPTYP_A::RSPTYP_1,
            2 => RSPTYP_A::RSPTYP_2,
            3 => RSPTYP_A::RSPTYP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RSPTYP_0`"]
    #[inline(always)]
    pub fn is_rsptyp_0(&self) -> bool {
        *self == RSPTYP_A::RSPTYP_0
    }
    #[doc = "Checks if the value of the field is `RSPTYP_1`"]
    #[inline(always)]
    pub fn is_rsptyp_1(&self) -> bool {
        *self == RSPTYP_A::RSPTYP_1
    }
    #[doc = "Checks if the value of the field is `RSPTYP_2`"]
    #[inline(always)]
    pub fn is_rsptyp_2(&self) -> bool {
        *self == RSPTYP_A::RSPTYP_2
    }
    #[doc = "Checks if the value of the field is `RSPTYP_3`"]
    #[inline(always)]
    pub fn is_rsptyp_3(&self) -> bool {
        *self == RSPTYP_A::RSPTYP_3
    }
}
#[doc = "Write proxy for field `RSPTYP`"]
pub struct RSPTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> RSPTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSPTYP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No Response"]
    #[inline(always)]
    pub fn rsptyp_0(self) -> &'a mut W {
        self.variant(RSPTYP_A::RSPTYP_0)
    }
    #[doc = "Response Length 136"]
    #[inline(always)]
    pub fn rsptyp_1(self) -> &'a mut W {
        self.variant(RSPTYP_A::RSPTYP_1)
    }
    #[doc = "Response Length 48"]
    #[inline(always)]
    pub fn rsptyp_2(self) -> &'a mut W {
        self.variant(RSPTYP_A::RSPTYP_2)
    }
    #[doc = "Response Length 48, check Busy after response"]
    #[inline(always)]
    pub fn rsptyp_3(self) -> &'a mut W {
        self.variant(RSPTYP_A::RSPTYP_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Command CRC Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCCEN_A {
    #[doc = "0: Disable"]
    CCCEN_0 = 0,
    #[doc = "1: Enable"]
    CCCEN_1 = 1,
}
impl From<CCCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCCEN`"]
pub type CCCEN_R = crate::R<bool, CCCEN_A>;
impl CCCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCCEN_A {
        match self.bits {
            false => CCCEN_A::CCCEN_0,
            true => CCCEN_A::CCCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCCEN_0`"]
    #[inline(always)]
    pub fn is_cccen_0(&self) -> bool {
        *self == CCCEN_A::CCCEN_0
    }
    #[doc = "Checks if the value of the field is `CCCEN_1`"]
    #[inline(always)]
    pub fn is_cccen_1(&self) -> bool {
        *self == CCCEN_A::CCCEN_1
    }
}
#[doc = "Write proxy for field `CCCEN`"]
pub struct CCCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn cccen_0(self) -> &'a mut W {
        self.variant(CCCEN_A::CCCEN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn cccen_1(self) -> &'a mut W {
        self.variant(CCCEN_A::CCCEN_1)
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
#[doc = "Command Index Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CICEN_A {
    #[doc = "0: Disable"]
    CICEN_0 = 0,
    #[doc = "1: Enable"]
    CICEN_1 = 1,
}
impl From<CICEN_A> for bool {
    #[inline(always)]
    fn from(variant: CICEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CICEN`"]
pub type CICEN_R = crate::R<bool, CICEN_A>;
impl CICEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CICEN_A {
        match self.bits {
            false => CICEN_A::CICEN_0,
            true => CICEN_A::CICEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CICEN_0`"]
    #[inline(always)]
    pub fn is_cicen_0(&self) -> bool {
        *self == CICEN_A::CICEN_0
    }
    #[doc = "Checks if the value of the field is `CICEN_1`"]
    #[inline(always)]
    pub fn is_cicen_1(&self) -> bool {
        *self == CICEN_A::CICEN_1
    }
}
#[doc = "Write proxy for field `CICEN`"]
pub struct CICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CICEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CICEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn cicen_0(self) -> &'a mut W {
        self.variant(CICEN_A::CICEN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn cicen_1(self) -> &'a mut W {
        self.variant(CICEN_A::CICEN_1)
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
#[doc = "Data Present Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPSEL_A {
    #[doc = "0: No Data Present"]
    DPSEL_0 = 0,
    #[doc = "1: Data Present"]
    DPSEL_1 = 1,
}
impl From<DPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DPSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPSEL`"]
pub type DPSEL_R = crate::R<bool, DPSEL_A>;
impl DPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPSEL_A {
        match self.bits {
            false => DPSEL_A::DPSEL_0,
            true => DPSEL_A::DPSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DPSEL_0`"]
    #[inline(always)]
    pub fn is_dpsel_0(&self) -> bool {
        *self == DPSEL_A::DPSEL_0
    }
    #[doc = "Checks if the value of the field is `DPSEL_1`"]
    #[inline(always)]
    pub fn is_dpsel_1(&self) -> bool {
        *self == DPSEL_A::DPSEL_1
    }
}
#[doc = "Write proxy for field `DPSEL`"]
pub struct DPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Data Present"]
    #[inline(always)]
    pub fn dpsel_0(self) -> &'a mut W {
        self.variant(DPSEL_A::DPSEL_0)
    }
    #[doc = "Data Present"]
    #[inline(always)]
    pub fn dpsel_1(self) -> &'a mut W {
        self.variant(DPSEL_A::DPSEL_1)
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
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMDTYP_A {
    #[doc = "0: Normal Other commands"]
    CMDTYP_0 = 0,
    #[doc = "1: Suspend CMD52 for writing Bus Suspend in CCCR"]
    CMDTYP_1 = 1,
    #[doc = "2: Resume CMD52 for writing Function Select in CCCR"]
    CMDTYP_2 = 2,
    #[doc = "3: Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
    CMDTYP_3 = 3,
}
impl From<CMDTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMDTYP`"]
pub type CMDTYP_R = crate::R<u8, CMDTYP_A>;
impl CMDTYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTYP_A {
        match self.bits {
            0 => CMDTYP_A::CMDTYP_0,
            1 => CMDTYP_A::CMDTYP_1,
            2 => CMDTYP_A::CMDTYP_2,
            3 => CMDTYP_A::CMDTYP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CMDTYP_0`"]
    #[inline(always)]
    pub fn is_cmdtyp_0(&self) -> bool {
        *self == CMDTYP_A::CMDTYP_0
    }
    #[doc = "Checks if the value of the field is `CMDTYP_1`"]
    #[inline(always)]
    pub fn is_cmdtyp_1(&self) -> bool {
        *self == CMDTYP_A::CMDTYP_1
    }
    #[doc = "Checks if the value of the field is `CMDTYP_2`"]
    #[inline(always)]
    pub fn is_cmdtyp_2(&self) -> bool {
        *self == CMDTYP_A::CMDTYP_2
    }
    #[doc = "Checks if the value of the field is `CMDTYP_3`"]
    #[inline(always)]
    pub fn is_cmdtyp_3(&self) -> bool {
        *self == CMDTYP_A::CMDTYP_3
    }
}
#[doc = "Write proxy for field `CMDTYP`"]
pub struct CMDTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDTYP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Normal Other commands"]
    #[inline(always)]
    pub fn cmdtyp_0(self) -> &'a mut W {
        self.variant(CMDTYP_A::CMDTYP_0)
    }
    #[doc = "Suspend CMD52 for writing Bus Suspend in CCCR"]
    #[inline(always)]
    pub fn cmdtyp_1(self) -> &'a mut W {
        self.variant(CMDTYP_A::CMDTYP_1)
    }
    #[doc = "Resume CMD52 for writing Function Select in CCCR"]
    #[inline(always)]
    pub fn cmdtyp_2(self) -> &'a mut W {
        self.variant(CMDTYP_A::CMDTYP_2)
    }
    #[doc = "Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
    #[inline(always)]
    pub fn cmdtyp_3(self) -> &'a mut W {
        self.variant(CMDTYP_A::CMDTYP_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `CMDINX`"]
pub type CMDINX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDINX`"]
pub struct CMDINX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDINX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn rsptyp(&self) -> RSPTYP_R {
        RSPTYP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cccen(&self) -> CCCEN_R {
        CCCEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cicen(&self) -> CICEN_R {
        CICEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn dpsel(&self) -> DPSEL_R {
        DPSEL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CMDTYP_R {
        CMDTYP_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdinx(&self) -> CMDINX_R {
        CMDINX_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn rsptyp(&mut self) -> RSPTYP_W {
        RSPTYP_W { w: self }
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cccen(&mut self) -> CCCEN_W {
        CCCEN_W { w: self }
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cicen(&mut self) -> CICEN_W {
        CICEN_W { w: self }
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn dpsel(&mut self) -> DPSEL_W {
        DPSEL_W { w: self }
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&mut self) -> CMDTYP_W {
        CMDTYP_W { w: self }
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdinx(&mut self) -> CMDINX_W {
        CMDINX_W { w: self }
    }
}
