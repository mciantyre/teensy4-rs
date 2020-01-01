#[doc = "Reader of register BAUD"]
pub type R = crate::R<u32, super::BAUD>;
#[doc = "Writer for register BAUD"]
pub type W = crate::W<u32, super::BAUD>;
#[doc = "Register BAUD `reset()`'s with value 0x0f00_0004"]
impl crate::ResetValue for super::BAUD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f00_0004
    }
}
#[doc = "Reader of field `SBR`"]
pub type SBR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SBR`"]
pub struct SBR_W<'a> {
    w: &'a mut W,
}
impl<'a> SBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Stop Bit Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBNS_A {
    #[doc = "0: One stop bit."]
    SBNS_0 = 0,
    #[doc = "1: Two stop bits."]
    SBNS_1 = 1,
}
impl From<SBNS_A> for bool {
    #[inline(always)]
    fn from(variant: SBNS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBNS`"]
pub type SBNS_R = crate::R<bool, SBNS_A>;
impl SBNS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBNS_A {
        match self.bits {
            false => SBNS_A::SBNS_0,
            true => SBNS_A::SBNS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SBNS_0`"]
    #[inline(always)]
    pub fn is_sbns_0(&self) -> bool {
        *self == SBNS_A::SBNS_0
    }
    #[doc = "Checks if the value of the field is `SBNS_1`"]
    #[inline(always)]
    pub fn is_sbns_1(&self) -> bool {
        *self == SBNS_A::SBNS_1
    }
}
#[doc = "Write proxy for field `SBNS`"]
pub struct SBNS_W<'a> {
    w: &'a mut W,
}
impl<'a> SBNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBNS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One stop bit."]
    #[inline(always)]
    pub fn sbns_0(self) -> &'a mut W {
        self.variant(SBNS_A::SBNS_0)
    }
    #[doc = "Two stop bits."]
    #[inline(always)]
    pub fn sbns_1(self) -> &'a mut W {
        self.variant(SBNS_A::SBNS_1)
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
#[doc = "RX Input Active Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIE_A {
    #[doc = "0: Hardware interrupts from STAT\\[RXEDGIF\\]
are disabled."]
    RXEDGIE_0 = 0,
    #[doc = "1: Hardware interrupt is requested when STAT\\[RXEDGIF\\]
flag is 1."]
    RXEDGIE_1 = 1,
}
impl From<RXEDGIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXEDGIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXEDGIE`"]
pub type RXEDGIE_R = crate::R<bool, RXEDGIE_A>;
impl RXEDGIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEDGIE_A {
        match self.bits {
            false => RXEDGIE_A::RXEDGIE_0,
            true => RXEDGIE_A::RXEDGIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXEDGIE_0`"]
    #[inline(always)]
    pub fn is_rxedgie_0(&self) -> bool {
        *self == RXEDGIE_A::RXEDGIE_0
    }
    #[doc = "Checks if the value of the field is `RXEDGIE_1`"]
    #[inline(always)]
    pub fn is_rxedgie_1(&self) -> bool {
        *self == RXEDGIE_A::RXEDGIE_1
    }
}
#[doc = "Write proxy for field `RXEDGIE`"]
pub struct RXEDGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEDGIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXEDGIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupts from STAT\\[RXEDGIF\\]
are disabled."]
    #[inline(always)]
    pub fn rxedgie_0(self) -> &'a mut W {
        self.variant(RXEDGIE_A::RXEDGIE_0)
    }
    #[doc = "Hardware interrupt is requested when STAT\\[RXEDGIF\\]
flag is 1."]
    #[inline(always)]
    pub fn rxedgie_1(self) -> &'a mut W {
        self.variant(RXEDGIE_A::RXEDGIE_1)
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
#[doc = "LIN Break Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIE_A {
    #[doc = "0: Hardware interrupts from STAT\\[LBKDIF\\]
flag are disabled (use polling)."]
    LBKDIE_0 = 0,
    #[doc = "1: Hardware interrupt requested when STAT\\[LBKDIF\\]
flag is 1."]
    LBKDIE_1 = 1,
}
impl From<LBKDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LBKDIE`"]
pub type LBKDIE_R = crate::R<bool, LBKDIE_A>;
impl LBKDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDIE_A {
        match self.bits {
            false => LBKDIE_A::LBKDIE_0,
            true => LBKDIE_A::LBKDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LBKDIE_0`"]
    #[inline(always)]
    pub fn is_lbkdie_0(&self) -> bool {
        *self == LBKDIE_A::LBKDIE_0
    }
    #[doc = "Checks if the value of the field is `LBKDIE_1`"]
    #[inline(always)]
    pub fn is_lbkdie_1(&self) -> bool {
        *self == LBKDIE_A::LBKDIE_1
    }
}
#[doc = "Write proxy for field `LBKDIE`"]
pub struct LBKDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBKDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupts from STAT\\[LBKDIF\\]
flag are disabled (use polling)."]
    #[inline(always)]
    pub fn lbkdie_0(self) -> &'a mut W {
        self.variant(LBKDIE_A::LBKDIE_0)
    }
    #[doc = "Hardware interrupt requested when STAT\\[LBKDIF\\]
flag is 1."]
    #[inline(always)]
    pub fn lbkdie_1(self) -> &'a mut W {
        self.variant(LBKDIE_A::LBKDIE_1)
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
#[doc = "Resynchronization Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESYNCDIS_A {
    #[doc = "0: Resynchronization during received data word is supported"]
    RESYNCDIS_0 = 0,
    #[doc = "1: Resynchronization during received data word is disabled"]
    RESYNCDIS_1 = 1,
}
impl From<RESYNCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RESYNCDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESYNCDIS`"]
pub type RESYNCDIS_R = crate::R<bool, RESYNCDIS_A>;
impl RESYNCDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESYNCDIS_A {
        match self.bits {
            false => RESYNCDIS_A::RESYNCDIS_0,
            true => RESYNCDIS_A::RESYNCDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RESYNCDIS_0`"]
    #[inline(always)]
    pub fn is_resyncdis_0(&self) -> bool {
        *self == RESYNCDIS_A::RESYNCDIS_0
    }
    #[doc = "Checks if the value of the field is `RESYNCDIS_1`"]
    #[inline(always)]
    pub fn is_resyncdis_1(&self) -> bool {
        *self == RESYNCDIS_A::RESYNCDIS_1
    }
}
#[doc = "Write proxy for field `RESYNCDIS`"]
pub struct RESYNCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RESYNCDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESYNCDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Resynchronization during received data word is supported"]
    #[inline(always)]
    pub fn resyncdis_0(self) -> &'a mut W {
        self.variant(RESYNCDIS_A::RESYNCDIS_0)
    }
    #[doc = "Resynchronization during received data word is disabled"]
    #[inline(always)]
    pub fn resyncdis_1(self) -> &'a mut W {
        self.variant(RESYNCDIS_A::RESYNCDIS_1)
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
#[doc = "Both Edge Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOTHEDGE_A {
    #[doc = "0: Receiver samples input data using the rising edge of the baud rate clock."]
    BOTHEDGE_0 = 0,
    #[doc = "1: Receiver samples input data using the rising and falling edge of the baud rate clock."]
    BOTHEDGE_1 = 1,
}
impl From<BOTHEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: BOTHEDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOTHEDGE`"]
pub type BOTHEDGE_R = crate::R<bool, BOTHEDGE_A>;
impl BOTHEDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOTHEDGE_A {
        match self.bits {
            false => BOTHEDGE_A::BOTHEDGE_0,
            true => BOTHEDGE_A::BOTHEDGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE_0`"]
    #[inline(always)]
    pub fn is_bothedge_0(&self) -> bool {
        *self == BOTHEDGE_A::BOTHEDGE_0
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE_1`"]
    #[inline(always)]
    pub fn is_bothedge_1(&self) -> bool {
        *self == BOTHEDGE_A::BOTHEDGE_1
    }
}
#[doc = "Write proxy for field `BOTHEDGE`"]
pub struct BOTHEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOTHEDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOTHEDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    #[inline(always)]
    pub fn bothedge_0(self) -> &'a mut W {
        self.variant(BOTHEDGE_A::BOTHEDGE_0)
    }
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    #[inline(always)]
    pub fn bothedge_1(self) -> &'a mut W {
        self.variant(BOTHEDGE_A::BOTHEDGE_1)
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
#[doc = "Match Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MATCFG_A {
    #[doc = "0: Address Match Wakeup"]
    MATCFG_0 = 0,
    #[doc = "1: Idle Match Wakeup"]
    MATCFG_1 = 1,
    #[doc = "2: Match On and Match Off"]
    MATCFG_2 = 2,
    #[doc = "3: Enables RWU on Data Match and Match On/Off for transmitter CTS input"]
    MATCFG_3 = 3,
}
impl From<MATCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MATCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MATCFG`"]
pub type MATCFG_R = crate::R<u8, MATCFG_A>;
impl MATCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MATCFG_A {
        match self.bits {
            0 => MATCFG_A::MATCFG_0,
            1 => MATCFG_A::MATCFG_1,
            2 => MATCFG_A::MATCFG_2,
            3 => MATCFG_A::MATCFG_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MATCFG_0`"]
    #[inline(always)]
    pub fn is_matcfg_0(&self) -> bool {
        *self == MATCFG_A::MATCFG_0
    }
    #[doc = "Checks if the value of the field is `MATCFG_1`"]
    #[inline(always)]
    pub fn is_matcfg_1(&self) -> bool {
        *self == MATCFG_A::MATCFG_1
    }
    #[doc = "Checks if the value of the field is `MATCFG_2`"]
    #[inline(always)]
    pub fn is_matcfg_2(&self) -> bool {
        *self == MATCFG_A::MATCFG_2
    }
    #[doc = "Checks if the value of the field is `MATCFG_3`"]
    #[inline(always)]
    pub fn is_matcfg_3(&self) -> bool {
        *self == MATCFG_A::MATCFG_3
    }
}
#[doc = "Write proxy for field `MATCFG`"]
pub struct MATCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MATCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Address Match Wakeup"]
    #[inline(always)]
    pub fn matcfg_0(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_0)
    }
    #[doc = "Idle Match Wakeup"]
    #[inline(always)]
    pub fn matcfg_1(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_1)
    }
    #[doc = "Match On and Match Off"]
    #[inline(always)]
    pub fn matcfg_2(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_2)
    }
    #[doc = "Enables RWU on Data Match and Match On/Off for transmitter CTS input"]
    #[inline(always)]
    pub fn matcfg_3(self) -> &'a mut W {
        self.variant(MATCFG_A::MATCFG_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Receiver Idle DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIDMAE_A {
    #[doc = "0: DMA request disabled."]
    RIDMAE_0 = 0,
    #[doc = "1: DMA request enabled."]
    RIDMAE_1 = 1,
}
impl From<RIDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: RIDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RIDMAE`"]
pub type RIDMAE_R = crate::R<bool, RIDMAE_A>;
impl RIDMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIDMAE_A {
        match self.bits {
            false => RIDMAE_A::RIDMAE_0,
            true => RIDMAE_A::RIDMAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RIDMAE_0`"]
    #[inline(always)]
    pub fn is_ridmae_0(&self) -> bool {
        *self == RIDMAE_A::RIDMAE_0
    }
    #[doc = "Checks if the value of the field is `RIDMAE_1`"]
    #[inline(always)]
    pub fn is_ridmae_1(&self) -> bool {
        *self == RIDMAE_A::RIDMAE_1
    }
}
#[doc = "Write proxy for field `RIDMAE`"]
pub struct RIDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIDMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn ridmae_0(self) -> &'a mut W {
        self.variant(RIDMAE_A::RIDMAE_0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn ridmae_1(self) -> &'a mut W {
        self.variant(RIDMAE_A::RIDMAE_1)
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
#[doc = "Receiver Full DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMAE_A {
    #[doc = "0: DMA request disabled."]
    RDMAE_0 = 0,
    #[doc = "1: DMA request enabled."]
    RDMAE_1 = 1,
}
impl From<RDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: RDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDMAE`"]
pub type RDMAE_R = crate::R<bool, RDMAE_A>;
impl RDMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMAE_A {
        match self.bits {
            false => RDMAE_A::RDMAE_0,
            true => RDMAE_A::RDMAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDMAE_0`"]
    #[inline(always)]
    pub fn is_rdmae_0(&self) -> bool {
        *self == RDMAE_A::RDMAE_0
    }
    #[doc = "Checks if the value of the field is `RDMAE_1`"]
    #[inline(always)]
    pub fn is_rdmae_1(&self) -> bool {
        *self == RDMAE_A::RDMAE_1
    }
}
#[doc = "Write proxy for field `RDMAE`"]
pub struct RDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn rdmae_0(self) -> &'a mut W {
        self.variant(RDMAE_A::RDMAE_0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn rdmae_1(self) -> &'a mut W {
        self.variant(RDMAE_A::RDMAE_1)
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
#[doc = "Transmitter DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMAE_A {
    #[doc = "0: DMA request disabled."]
    TDMAE_0 = 0,
    #[doc = "1: DMA request enabled."]
    TDMAE_1 = 1,
}
impl From<TDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: TDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDMAE`"]
pub type TDMAE_R = crate::R<bool, TDMAE_A>;
impl TDMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDMAE_A {
        match self.bits {
            false => TDMAE_A::TDMAE_0,
            true => TDMAE_A::TDMAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDMAE_0`"]
    #[inline(always)]
    pub fn is_tdmae_0(&self) -> bool {
        *self == TDMAE_A::TDMAE_0
    }
    #[doc = "Checks if the value of the field is `TDMAE_1`"]
    #[inline(always)]
    pub fn is_tdmae_1(&self) -> bool {
        *self == TDMAE_A::TDMAE_1
    }
}
#[doc = "Write proxy for field `TDMAE`"]
pub struct TDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn tdmae_0(self) -> &'a mut W {
        self.variant(TDMAE_A::TDMAE_0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn tdmae_1(self) -> &'a mut W {
        self.variant(TDMAE_A::TDMAE_1)
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
#[doc = "Oversampling Ratio\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSR_A {
    #[doc = "0: Writing 0 to this field will result in an oversampling ratio of 16"]
    OSR_0 = 0,
    #[doc = "3: Oversampling ratio of 4, requires BOTHEDGE to be set."]
    OSR_3 = 3,
    #[doc = "4: Oversampling ratio of 5, requires BOTHEDGE to be set."]
    OSR_4 = 4,
    #[doc = "5: Oversampling ratio of 6, requires BOTHEDGE to be set."]
    OSR_5 = 5,
    #[doc = "6: Oversampling ratio of 7, requires BOTHEDGE to be set."]
    OSR_6 = 6,
    #[doc = "7: Oversampling ratio of 8."]
    OSR_7 = 7,
    #[doc = "8: Oversampling ratio of 9."]
    OSR_8 = 8,
    #[doc = "9: Oversampling ratio of 10."]
    OSR_9 = 9,
    #[doc = "10: Oversampling ratio of 11."]
    OSR_10 = 10,
    #[doc = "11: Oversampling ratio of 12."]
    OSR_11 = 11,
    #[doc = "12: Oversampling ratio of 13."]
    OSR_12 = 12,
    #[doc = "13: Oversampling ratio of 14."]
    OSR_13 = 13,
    #[doc = "14: Oversampling ratio of 15."]
    OSR_14 = 14,
    #[doc = "15: Oversampling ratio of 16."]
    OSR_15 = 15,
    #[doc = "16: Oversampling ratio of 17."]
    OSR_16 = 16,
    #[doc = "17: Oversampling ratio of 18."]
    OSR_17 = 17,
    #[doc = "18: Oversampling ratio of 19."]
    OSR_18 = 18,
    #[doc = "19: Oversampling ratio of 20."]
    OSR_19 = 19,
    #[doc = "20: Oversampling ratio of 21."]
    OSR_20 = 20,
    #[doc = "21: Oversampling ratio of 22."]
    OSR_21 = 21,
    #[doc = "22: Oversampling ratio of 23."]
    OSR_22 = 22,
    #[doc = "23: Oversampling ratio of 24."]
    OSR_23 = 23,
    #[doc = "24: Oversampling ratio of 25."]
    OSR_24 = 24,
    #[doc = "25: Oversampling ratio of 26."]
    OSR_25 = 25,
    #[doc = "26: Oversampling ratio of 27."]
    OSR_26 = 26,
    #[doc = "27: Oversampling ratio of 28."]
    OSR_27 = 27,
    #[doc = "28: Oversampling ratio of 29."]
    OSR_28 = 28,
    #[doc = "29: Oversampling ratio of 30."]
    OSR_29 = 29,
    #[doc = "30: Oversampling ratio of 31."]
    OSR_30 = 30,
    #[doc = "31: Oversampling ratio of 32."]
    OSR_31 = 31,
}
impl From<OSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OSR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSR`"]
pub type OSR_R = crate::R<u8, OSR_A>;
impl OSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OSR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OSR_A::OSR_0),
            3 => Val(OSR_A::OSR_3),
            4 => Val(OSR_A::OSR_4),
            5 => Val(OSR_A::OSR_5),
            6 => Val(OSR_A::OSR_6),
            7 => Val(OSR_A::OSR_7),
            8 => Val(OSR_A::OSR_8),
            9 => Val(OSR_A::OSR_9),
            10 => Val(OSR_A::OSR_10),
            11 => Val(OSR_A::OSR_11),
            12 => Val(OSR_A::OSR_12),
            13 => Val(OSR_A::OSR_13),
            14 => Val(OSR_A::OSR_14),
            15 => Val(OSR_A::OSR_15),
            16 => Val(OSR_A::OSR_16),
            17 => Val(OSR_A::OSR_17),
            18 => Val(OSR_A::OSR_18),
            19 => Val(OSR_A::OSR_19),
            20 => Val(OSR_A::OSR_20),
            21 => Val(OSR_A::OSR_21),
            22 => Val(OSR_A::OSR_22),
            23 => Val(OSR_A::OSR_23),
            24 => Val(OSR_A::OSR_24),
            25 => Val(OSR_A::OSR_25),
            26 => Val(OSR_A::OSR_26),
            27 => Val(OSR_A::OSR_27),
            28 => Val(OSR_A::OSR_28),
            29 => Val(OSR_A::OSR_29),
            30 => Val(OSR_A::OSR_30),
            31 => Val(OSR_A::OSR_31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OSR_0`"]
    #[inline(always)]
    pub fn is_osr_0(&self) -> bool {
        *self == OSR_A::OSR_0
    }
    #[doc = "Checks if the value of the field is `OSR_3`"]
    #[inline(always)]
    pub fn is_osr_3(&self) -> bool {
        *self == OSR_A::OSR_3
    }
    #[doc = "Checks if the value of the field is `OSR_4`"]
    #[inline(always)]
    pub fn is_osr_4(&self) -> bool {
        *self == OSR_A::OSR_4
    }
    #[doc = "Checks if the value of the field is `OSR_5`"]
    #[inline(always)]
    pub fn is_osr_5(&self) -> bool {
        *self == OSR_A::OSR_5
    }
    #[doc = "Checks if the value of the field is `OSR_6`"]
    #[inline(always)]
    pub fn is_osr_6(&self) -> bool {
        *self == OSR_A::OSR_6
    }
    #[doc = "Checks if the value of the field is `OSR_7`"]
    #[inline(always)]
    pub fn is_osr_7(&self) -> bool {
        *self == OSR_A::OSR_7
    }
    #[doc = "Checks if the value of the field is `OSR_8`"]
    #[inline(always)]
    pub fn is_osr_8(&self) -> bool {
        *self == OSR_A::OSR_8
    }
    #[doc = "Checks if the value of the field is `OSR_9`"]
    #[inline(always)]
    pub fn is_osr_9(&self) -> bool {
        *self == OSR_A::OSR_9
    }
    #[doc = "Checks if the value of the field is `OSR_10`"]
    #[inline(always)]
    pub fn is_osr_10(&self) -> bool {
        *self == OSR_A::OSR_10
    }
    #[doc = "Checks if the value of the field is `OSR_11`"]
    #[inline(always)]
    pub fn is_osr_11(&self) -> bool {
        *self == OSR_A::OSR_11
    }
    #[doc = "Checks if the value of the field is `OSR_12`"]
    #[inline(always)]
    pub fn is_osr_12(&self) -> bool {
        *self == OSR_A::OSR_12
    }
    #[doc = "Checks if the value of the field is `OSR_13`"]
    #[inline(always)]
    pub fn is_osr_13(&self) -> bool {
        *self == OSR_A::OSR_13
    }
    #[doc = "Checks if the value of the field is `OSR_14`"]
    #[inline(always)]
    pub fn is_osr_14(&self) -> bool {
        *self == OSR_A::OSR_14
    }
    #[doc = "Checks if the value of the field is `OSR_15`"]
    #[inline(always)]
    pub fn is_osr_15(&self) -> bool {
        *self == OSR_A::OSR_15
    }
    #[doc = "Checks if the value of the field is `OSR_16`"]
    #[inline(always)]
    pub fn is_osr_16(&self) -> bool {
        *self == OSR_A::OSR_16
    }
    #[doc = "Checks if the value of the field is `OSR_17`"]
    #[inline(always)]
    pub fn is_osr_17(&self) -> bool {
        *self == OSR_A::OSR_17
    }
    #[doc = "Checks if the value of the field is `OSR_18`"]
    #[inline(always)]
    pub fn is_osr_18(&self) -> bool {
        *self == OSR_A::OSR_18
    }
    #[doc = "Checks if the value of the field is `OSR_19`"]
    #[inline(always)]
    pub fn is_osr_19(&self) -> bool {
        *self == OSR_A::OSR_19
    }
    #[doc = "Checks if the value of the field is `OSR_20`"]
    #[inline(always)]
    pub fn is_osr_20(&self) -> bool {
        *self == OSR_A::OSR_20
    }
    #[doc = "Checks if the value of the field is `OSR_21`"]
    #[inline(always)]
    pub fn is_osr_21(&self) -> bool {
        *self == OSR_A::OSR_21
    }
    #[doc = "Checks if the value of the field is `OSR_22`"]
    #[inline(always)]
    pub fn is_osr_22(&self) -> bool {
        *self == OSR_A::OSR_22
    }
    #[doc = "Checks if the value of the field is `OSR_23`"]
    #[inline(always)]
    pub fn is_osr_23(&self) -> bool {
        *self == OSR_A::OSR_23
    }
    #[doc = "Checks if the value of the field is `OSR_24`"]
    #[inline(always)]
    pub fn is_osr_24(&self) -> bool {
        *self == OSR_A::OSR_24
    }
    #[doc = "Checks if the value of the field is `OSR_25`"]
    #[inline(always)]
    pub fn is_osr_25(&self) -> bool {
        *self == OSR_A::OSR_25
    }
    #[doc = "Checks if the value of the field is `OSR_26`"]
    #[inline(always)]
    pub fn is_osr_26(&self) -> bool {
        *self == OSR_A::OSR_26
    }
    #[doc = "Checks if the value of the field is `OSR_27`"]
    #[inline(always)]
    pub fn is_osr_27(&self) -> bool {
        *self == OSR_A::OSR_27
    }
    #[doc = "Checks if the value of the field is `OSR_28`"]
    #[inline(always)]
    pub fn is_osr_28(&self) -> bool {
        *self == OSR_A::OSR_28
    }
    #[doc = "Checks if the value of the field is `OSR_29`"]
    #[inline(always)]
    pub fn is_osr_29(&self) -> bool {
        *self == OSR_A::OSR_29
    }
    #[doc = "Checks if the value of the field is `OSR_30`"]
    #[inline(always)]
    pub fn is_osr_30(&self) -> bool {
        *self == OSR_A::OSR_30
    }
    #[doc = "Checks if the value of the field is `OSR_31`"]
    #[inline(always)]
    pub fn is_osr_31(&self) -> bool {
        *self == OSR_A::OSR_31
    }
}
#[doc = "Write proxy for field `OSR`"]
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing 0 to this field will result in an oversampling ratio of 16"]
    #[inline(always)]
    pub fn osr_0(self) -> &'a mut W {
        self.variant(OSR_A::OSR_0)
    }
    #[doc = "Oversampling ratio of 4, requires BOTHEDGE to be set."]
    #[inline(always)]
    pub fn osr_3(self) -> &'a mut W {
        self.variant(OSR_A::OSR_3)
    }
    #[doc = "Oversampling ratio of 5, requires BOTHEDGE to be set."]
    #[inline(always)]
    pub fn osr_4(self) -> &'a mut W {
        self.variant(OSR_A::OSR_4)
    }
    #[doc = "Oversampling ratio of 6, requires BOTHEDGE to be set."]
    #[inline(always)]
    pub fn osr_5(self) -> &'a mut W {
        self.variant(OSR_A::OSR_5)
    }
    #[doc = "Oversampling ratio of 7, requires BOTHEDGE to be set."]
    #[inline(always)]
    pub fn osr_6(self) -> &'a mut W {
        self.variant(OSR_A::OSR_6)
    }
    #[doc = "Oversampling ratio of 8."]
    #[inline(always)]
    pub fn osr_7(self) -> &'a mut W {
        self.variant(OSR_A::OSR_7)
    }
    #[doc = "Oversampling ratio of 9."]
    #[inline(always)]
    pub fn osr_8(self) -> &'a mut W {
        self.variant(OSR_A::OSR_8)
    }
    #[doc = "Oversampling ratio of 10."]
    #[inline(always)]
    pub fn osr_9(self) -> &'a mut W {
        self.variant(OSR_A::OSR_9)
    }
    #[doc = "Oversampling ratio of 11."]
    #[inline(always)]
    pub fn osr_10(self) -> &'a mut W {
        self.variant(OSR_A::OSR_10)
    }
    #[doc = "Oversampling ratio of 12."]
    #[inline(always)]
    pub fn osr_11(self) -> &'a mut W {
        self.variant(OSR_A::OSR_11)
    }
    #[doc = "Oversampling ratio of 13."]
    #[inline(always)]
    pub fn osr_12(self) -> &'a mut W {
        self.variant(OSR_A::OSR_12)
    }
    #[doc = "Oversampling ratio of 14."]
    #[inline(always)]
    pub fn osr_13(self) -> &'a mut W {
        self.variant(OSR_A::OSR_13)
    }
    #[doc = "Oversampling ratio of 15."]
    #[inline(always)]
    pub fn osr_14(self) -> &'a mut W {
        self.variant(OSR_A::OSR_14)
    }
    #[doc = "Oversampling ratio of 16."]
    #[inline(always)]
    pub fn osr_15(self) -> &'a mut W {
        self.variant(OSR_A::OSR_15)
    }
    #[doc = "Oversampling ratio of 17."]
    #[inline(always)]
    pub fn osr_16(self) -> &'a mut W {
        self.variant(OSR_A::OSR_16)
    }
    #[doc = "Oversampling ratio of 18."]
    #[inline(always)]
    pub fn osr_17(self) -> &'a mut W {
        self.variant(OSR_A::OSR_17)
    }
    #[doc = "Oversampling ratio of 19."]
    #[inline(always)]
    pub fn osr_18(self) -> &'a mut W {
        self.variant(OSR_A::OSR_18)
    }
    #[doc = "Oversampling ratio of 20."]
    #[inline(always)]
    pub fn osr_19(self) -> &'a mut W {
        self.variant(OSR_A::OSR_19)
    }
    #[doc = "Oversampling ratio of 21."]
    #[inline(always)]
    pub fn osr_20(self) -> &'a mut W {
        self.variant(OSR_A::OSR_20)
    }
    #[doc = "Oversampling ratio of 22."]
    #[inline(always)]
    pub fn osr_21(self) -> &'a mut W {
        self.variant(OSR_A::OSR_21)
    }
    #[doc = "Oversampling ratio of 23."]
    #[inline(always)]
    pub fn osr_22(self) -> &'a mut W {
        self.variant(OSR_A::OSR_22)
    }
    #[doc = "Oversampling ratio of 24."]
    #[inline(always)]
    pub fn osr_23(self) -> &'a mut W {
        self.variant(OSR_A::OSR_23)
    }
    #[doc = "Oversampling ratio of 25."]
    #[inline(always)]
    pub fn osr_24(self) -> &'a mut W {
        self.variant(OSR_A::OSR_24)
    }
    #[doc = "Oversampling ratio of 26."]
    #[inline(always)]
    pub fn osr_25(self) -> &'a mut W {
        self.variant(OSR_A::OSR_25)
    }
    #[doc = "Oversampling ratio of 27."]
    #[inline(always)]
    pub fn osr_26(self) -> &'a mut W {
        self.variant(OSR_A::OSR_26)
    }
    #[doc = "Oversampling ratio of 28."]
    #[inline(always)]
    pub fn osr_27(self) -> &'a mut W {
        self.variant(OSR_A::OSR_27)
    }
    #[doc = "Oversampling ratio of 29."]
    #[inline(always)]
    pub fn osr_28(self) -> &'a mut W {
        self.variant(OSR_A::OSR_28)
    }
    #[doc = "Oversampling ratio of 30."]
    #[inline(always)]
    pub fn osr_29(self) -> &'a mut W {
        self.variant(OSR_A::OSR_29)
    }
    #[doc = "Oversampling ratio of 31."]
    #[inline(always)]
    pub fn osr_30(self) -> &'a mut W {
        self.variant(OSR_A::OSR_30)
    }
    #[doc = "Oversampling ratio of 32."]
    #[inline(always)]
    pub fn osr_31(self) -> &'a mut W {
        self.variant(OSR_A::OSR_31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "10-bit Mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M10_A {
    #[doc = "0: Receiver and transmitter use 7-bit to 9-bit data characters."]
    M10_0 = 0,
    #[doc = "1: Receiver and transmitter use 10-bit data characters."]
    M10_1 = 1,
}
impl From<M10_A> for bool {
    #[inline(always)]
    fn from(variant: M10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `M10`"]
pub type M10_R = crate::R<bool, M10_A>;
impl M10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M10_A {
        match self.bits {
            false => M10_A::M10_0,
            true => M10_A::M10_1,
        }
    }
    #[doc = "Checks if the value of the field is `M10_0`"]
    #[inline(always)]
    pub fn is_m10_0(&self) -> bool {
        *self == M10_A::M10_0
    }
    #[doc = "Checks if the value of the field is `M10_1`"]
    #[inline(always)]
    pub fn is_m10_1(&self) -> bool {
        *self == M10_A::M10_1
    }
}
#[doc = "Write proxy for field `M10`"]
pub struct M10_W<'a> {
    w: &'a mut W,
}
impl<'a> M10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters."]
    #[inline(always)]
    pub fn m10_0(self) -> &'a mut W {
        self.variant(M10_A::M10_0)
    }
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    #[inline(always)]
    pub fn m10_1(self) -> &'a mut W {
        self.variant(M10_A::M10_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Match Address Mode Enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN2_A {
    #[doc = "0: Normal operation."]
    MAEN2_0 = 0,
    #[doc = "1: Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
    MAEN2_1 = 1,
}
impl From<MAEN2_A> for bool {
    #[inline(always)]
    fn from(variant: MAEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAEN2`"]
pub type MAEN2_R = crate::R<bool, MAEN2_A>;
impl MAEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAEN2_A {
        match self.bits {
            false => MAEN2_A::MAEN2_0,
            true => MAEN2_A::MAEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `MAEN2_0`"]
    #[inline(always)]
    pub fn is_maen2_0(&self) -> bool {
        *self == MAEN2_A::MAEN2_0
    }
    #[doc = "Checks if the value of the field is `MAEN2_1`"]
    #[inline(always)]
    pub fn is_maen2_1(&self) -> bool {
        *self == MAEN2_A::MAEN2_1
    }
}
#[doc = "Write proxy for field `MAEN2`"]
pub struct MAEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> MAEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn maen2_0(self) -> &'a mut W {
        self.variant(MAEN2_A::MAEN2_0)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
    #[inline(always)]
    pub fn maen2_1(self) -> &'a mut W {
        self.variant(MAEN2_A::MAEN2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Match Address Mode Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN1_A {
    #[doc = "0: Normal operation."]
    MAEN1_0 = 0,
    #[doc = "1: Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
    MAEN1_1 = 1,
}
impl From<MAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: MAEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAEN1`"]
pub type MAEN1_R = crate::R<bool, MAEN1_A>;
impl MAEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAEN1_A {
        match self.bits {
            false => MAEN1_A::MAEN1_0,
            true => MAEN1_A::MAEN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `MAEN1_0`"]
    #[inline(always)]
    pub fn is_maen1_0(&self) -> bool {
        *self == MAEN1_A::MAEN1_0
    }
    #[doc = "Checks if the value of the field is `MAEN1_1`"]
    #[inline(always)]
    pub fn is_maen1_1(&self) -> bool {
        *self == MAEN1_A::MAEN1_1
    }
}
#[doc = "Write proxy for field `MAEN1`"]
pub struct MAEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn maen1_0(self) -> &'a mut W {
        self.variant(MAEN1_A::MAEN1_0)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
    #[inline(always)]
    pub fn maen1_1(self) -> &'a mut W {
        self.variant(MAEN1_A::MAEN1_1)
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
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline(always)]
    pub fn sbr(&self) -> SBR_R {
        SBR_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline(always)]
    pub fn sbns(&self) -> SBNS_R {
        SBNS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub fn rxedgie(&self) -> RXEDGIE_R {
        RXEDGIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lbkdie(&self) -> LBKDIE_R {
        LBKDIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline(always)]
    pub fn resyncdis(&self) -> RESYNCDIS_R {
        RESYNCDIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline(always)]
    pub fn bothedge(&self) -> BOTHEDGE_R {
        BOTHEDGE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&self) -> MATCFG_R {
        MATCFG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Receiver Idle DMA Enable"]
    #[inline(always)]
    pub fn ridmae(&self) -> RIDMAE_R {
        RIDMAE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline(always)]
    pub fn rdmae(&self) -> RDMAE_R {
        RDMAE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline(always)]
    pub fn tdmae(&self) -> TDMAE_R {
        TDMAE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Oversampling Ratio"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&self) -> M10_R {
        M10_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&self) -> MAEN2_R {
        MAEN2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&self) -> MAEN1_R {
        MAEN1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline(always)]
    pub fn sbr(&mut self) -> SBR_W {
        SBR_W { w: self }
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline(always)]
    pub fn sbns(&mut self) -> SBNS_W {
        SBNS_W { w: self }
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub fn rxedgie(&mut self) -> RXEDGIE_W {
        RXEDGIE_W { w: self }
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lbkdie(&mut self) -> LBKDIE_W {
        LBKDIE_W { w: self }
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline(always)]
    pub fn resyncdis(&mut self) -> RESYNCDIS_W {
        RESYNCDIS_W { w: self }
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline(always)]
    pub fn bothedge(&mut self) -> BOTHEDGE_W {
        BOTHEDGE_W { w: self }
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&mut self) -> MATCFG_W {
        MATCFG_W { w: self }
    }
    #[doc = "Bit 20 - Receiver Idle DMA Enable"]
    #[inline(always)]
    pub fn ridmae(&mut self) -> RIDMAE_W {
        RIDMAE_W { w: self }
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline(always)]
    pub fn rdmae(&mut self) -> RDMAE_W {
        RDMAE_W { w: self }
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline(always)]
    pub fn tdmae(&mut self) -> TDMAE_W {
        TDMAE_W { w: self }
    }
    #[doc = "Bits 24:28 - Oversampling Ratio"]
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W {
        OSR_W { w: self }
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&mut self) -> M10_W {
        M10_W { w: self }
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&mut self) -> MAEN2_W {
        MAEN2_W { w: self }
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&mut self) -> MAEN1_W {
        MAEN1_W { w: self }
    }
}
