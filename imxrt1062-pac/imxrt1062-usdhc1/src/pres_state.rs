#[doc = "Reader of register PRES_STATE"]
pub type R = crate::R<u32, super::PRES_STATE>;
#[doc = "Command Inhibit (CMD)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIHB_A {
    #[doc = "0: Can issue command using only CMD line"]
    CIHB_0 = 0,
    #[doc = "1: Cannot issue command"]
    CIHB_1 = 1,
}
impl From<CIHB_A> for bool {
    #[inline(always)]
    fn from(variant: CIHB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CIHB`"]
pub type CIHB_R = crate::R<bool, CIHB_A>;
impl CIHB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIHB_A {
        match self.bits {
            false => CIHB_A::CIHB_0,
            true => CIHB_A::CIHB_1,
        }
    }
    #[doc = "Checks if the value of the field is `CIHB_0`"]
    #[inline(always)]
    pub fn is_cihb_0(&self) -> bool {
        *self == CIHB_A::CIHB_0
    }
    #[doc = "Checks if the value of the field is `CIHB_1`"]
    #[inline(always)]
    pub fn is_cihb_1(&self) -> bool {
        *self == CIHB_A::CIHB_1
    }
}
#[doc = "Command Inhibit (DATA)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDIHB_A {
    #[doc = "0: Can issue command which uses the DATA line"]
    CDIHB_0 = 0,
    #[doc = "1: Cannot issue command which uses the DATA line"]
    CDIHB_1 = 1,
}
impl From<CDIHB_A> for bool {
    #[inline(always)]
    fn from(variant: CDIHB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CDIHB`"]
pub type CDIHB_R = crate::R<bool, CDIHB_A>;
impl CDIHB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDIHB_A {
        match self.bits {
            false => CDIHB_A::CDIHB_0,
            true => CDIHB_A::CDIHB_1,
        }
    }
    #[doc = "Checks if the value of the field is `CDIHB_0`"]
    #[inline(always)]
    pub fn is_cdihb_0(&self) -> bool {
        *self == CDIHB_A::CDIHB_0
    }
    #[doc = "Checks if the value of the field is `CDIHB_1`"]
    #[inline(always)]
    pub fn is_cdihb_1(&self) -> bool {
        *self == CDIHB_A::CDIHB_1
    }
}
#[doc = "Data Line Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLA_A {
    #[doc = "0: DATA Line Inactive"]
    DLA_0 = 0,
    #[doc = "1: DATA Line Active"]
    DLA_1 = 1,
}
impl From<DLA_A> for bool {
    #[inline(always)]
    fn from(variant: DLA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLA`"]
pub type DLA_R = crate::R<bool, DLA_A>;
impl DLA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLA_A {
        match self.bits {
            false => DLA_A::DLA_0,
            true => DLA_A::DLA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DLA_0`"]
    #[inline(always)]
    pub fn is_dla_0(&self) -> bool {
        *self == DLA_A::DLA_0
    }
    #[doc = "Checks if the value of the field is `DLA_1`"]
    #[inline(always)]
    pub fn is_dla_1(&self) -> bool {
        *self == DLA_A::DLA_1
    }
}
#[doc = "SD Clock Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDSTB_A {
    #[doc = "0: Clock is changing frequency and not stable."]
    SDSTB_0 = 0,
    #[doc = "1: Clock is stable."]
    SDSTB_1 = 1,
}
impl From<SDSTB_A> for bool {
    #[inline(always)]
    fn from(variant: SDSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDSTB`"]
pub type SDSTB_R = crate::R<bool, SDSTB_A>;
impl SDSTB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDSTB_A {
        match self.bits {
            false => SDSTB_A::SDSTB_0,
            true => SDSTB_A::SDSTB_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDSTB_0`"]
    #[inline(always)]
    pub fn is_sdstb_0(&self) -> bool {
        *self == SDSTB_A::SDSTB_0
    }
    #[doc = "Checks if the value of the field is `SDSTB_1`"]
    #[inline(always)]
    pub fn is_sdstb_1(&self) -> bool {
        *self == SDSTB_A::SDSTB_1
    }
}
#[doc = "IPG_CLK Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPGOFF_A {
    #[doc = "0: IPG_CLK is active."]
    IPGOFF_0 = 0,
    #[doc = "1: IPG_CLK is gated off."]
    IPGOFF_1 = 1,
}
impl From<IPGOFF_A> for bool {
    #[inline(always)]
    fn from(variant: IPGOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IPGOFF`"]
pub type IPGOFF_R = crate::R<bool, IPGOFF_A>;
impl IPGOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPGOFF_A {
        match self.bits {
            false => IPGOFF_A::IPGOFF_0,
            true => IPGOFF_A::IPGOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPGOFF_0`"]
    #[inline(always)]
    pub fn is_ipgoff_0(&self) -> bool {
        *self == IPGOFF_A::IPGOFF_0
    }
    #[doc = "Checks if the value of the field is `IPGOFF_1`"]
    #[inline(always)]
    pub fn is_ipgoff_1(&self) -> bool {
        *self == IPGOFF_A::IPGOFF_1
    }
}
#[doc = "HCLK Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCKOFF_A {
    #[doc = "0: HCLK is active."]
    HCKOFF_0 = 0,
    #[doc = "1: HCLK is gated off."]
    HCKOFF_1 = 1,
}
impl From<HCKOFF_A> for bool {
    #[inline(always)]
    fn from(variant: HCKOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HCKOFF`"]
pub type HCKOFF_R = crate::R<bool, HCKOFF_A>;
impl HCKOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCKOFF_A {
        match self.bits {
            false => HCKOFF_A::HCKOFF_0,
            true => HCKOFF_A::HCKOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `HCKOFF_0`"]
    #[inline(always)]
    pub fn is_hckoff_0(&self) -> bool {
        *self == HCKOFF_A::HCKOFF_0
    }
    #[doc = "Checks if the value of the field is `HCKOFF_1`"]
    #[inline(always)]
    pub fn is_hckoff_1(&self) -> bool {
        *self == HCKOFF_A::HCKOFF_1
    }
}
#[doc = "IPG_PERCLK Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEROFF_A {
    #[doc = "0: IPG_PERCLK is active."]
    PEROFF_0 = 0,
    #[doc = "1: IPG_PERCLK is gated off."]
    PEROFF_1 = 1,
}
impl From<PEROFF_A> for bool {
    #[inline(always)]
    fn from(variant: PEROFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEROFF`"]
pub type PEROFF_R = crate::R<bool, PEROFF_A>;
impl PEROFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEROFF_A {
        match self.bits {
            false => PEROFF_A::PEROFF_0,
            true => PEROFF_A::PEROFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `PEROFF_0`"]
    #[inline(always)]
    pub fn is_peroff_0(&self) -> bool {
        *self == PEROFF_A::PEROFF_0
    }
    #[doc = "Checks if the value of the field is `PEROFF_1`"]
    #[inline(always)]
    pub fn is_peroff_1(&self) -> bool {
        *self == PEROFF_A::PEROFF_1
    }
}
#[doc = "SD Clock Gated Off Internally\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDOFF_A {
    #[doc = "0: SD Clock is active."]
    SDOFF_0 = 0,
    #[doc = "1: SD Clock is gated off."]
    SDOFF_1 = 1,
}
impl From<SDOFF_A> for bool {
    #[inline(always)]
    fn from(variant: SDOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDOFF`"]
pub type SDOFF_R = crate::R<bool, SDOFF_A>;
impl SDOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDOFF_A {
        match self.bits {
            false => SDOFF_A::SDOFF_0,
            true => SDOFF_A::SDOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDOFF_0`"]
    #[inline(always)]
    pub fn is_sdoff_0(&self) -> bool {
        *self == SDOFF_A::SDOFF_0
    }
    #[doc = "Checks if the value of the field is `SDOFF_1`"]
    #[inline(always)]
    pub fn is_sdoff_1(&self) -> bool {
        *self == SDOFF_A::SDOFF_1
    }
}
#[doc = "Write Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTA_A {
    #[doc = "0: No valid data"]
    WTA_0 = 0,
    #[doc = "1: Transferring data"]
    WTA_1 = 1,
}
impl From<WTA_A> for bool {
    #[inline(always)]
    fn from(variant: WTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WTA`"]
pub type WTA_R = crate::R<bool, WTA_A>;
impl WTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTA_A {
        match self.bits {
            false => WTA_A::WTA_0,
            true => WTA_A::WTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `WTA_0`"]
    #[inline(always)]
    pub fn is_wta_0(&self) -> bool {
        *self == WTA_A::WTA_0
    }
    #[doc = "Checks if the value of the field is `WTA_1`"]
    #[inline(always)]
    pub fn is_wta_1(&self) -> bool {
        *self == WTA_A::WTA_1
    }
}
#[doc = "Read Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTA_A {
    #[doc = "0: No valid data"]
    RTA_0 = 0,
    #[doc = "1: Transferring data"]
    RTA_1 = 1,
}
impl From<RTA_A> for bool {
    #[inline(always)]
    fn from(variant: RTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTA`"]
pub type RTA_R = crate::R<bool, RTA_A>;
impl RTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTA_A {
        match self.bits {
            false => RTA_A::RTA_0,
            true => RTA_A::RTA_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTA_0`"]
    #[inline(always)]
    pub fn is_rta_0(&self) -> bool {
        *self == RTA_A::RTA_0
    }
    #[doc = "Checks if the value of the field is `RTA_1`"]
    #[inline(always)]
    pub fn is_rta_1(&self) -> bool {
        *self == RTA_A::RTA_1
    }
}
#[doc = "Buffer Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWEN_A {
    #[doc = "0: Write disable"]
    BWEN_0 = 0,
    #[doc = "1: Write enable"]
    BWEN_1 = 1,
}
impl From<BWEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BWEN`"]
pub type BWEN_R = crate::R<bool, BWEN_A>;
impl BWEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWEN_A {
        match self.bits {
            false => BWEN_A::BWEN_0,
            true => BWEN_A::BWEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BWEN_0`"]
    #[inline(always)]
    pub fn is_bwen_0(&self) -> bool {
        *self == BWEN_A::BWEN_0
    }
    #[doc = "Checks if the value of the field is `BWEN_1`"]
    #[inline(always)]
    pub fn is_bwen_1(&self) -> bool {
        *self == BWEN_A::BWEN_1
    }
}
#[doc = "Buffer Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREN_A {
    #[doc = "0: Read disable"]
    BREN_0 = 0,
    #[doc = "1: Read enable"]
    BREN_1 = 1,
}
impl From<BREN_A> for bool {
    #[inline(always)]
    fn from(variant: BREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BREN`"]
pub type BREN_R = crate::R<bool, BREN_A>;
impl BREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREN_A {
        match self.bits {
            false => BREN_A::BREN_0,
            true => BREN_A::BREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BREN_0`"]
    #[inline(always)]
    pub fn is_bren_0(&self) -> bool {
        *self == BREN_A::BREN_0
    }
    #[doc = "Checks if the value of the field is `BREN_1`"]
    #[inline(always)]
    pub fn is_bren_1(&self) -> bool {
        *self == BREN_A::BREN_1
    }
}
#[doc = "Re-Tuning Request (only for SD3.0 SDR104 mode and EMMC HS200 mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTR_A {
    #[doc = "0: Fixed or well tuned sampling clock"]
    RTR_0 = 0,
    #[doc = "1: Sampling clock needs re-tuning"]
    RTR_1 = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTR`"]
pub type RTR_R = crate::R<bool, RTR_A>;
impl RTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::RTR_0,
            true => RTR_A::RTR_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTR_0`"]
    #[inline(always)]
    pub fn is_rtr_0(&self) -> bool {
        *self == RTR_A::RTR_0
    }
    #[doc = "Checks if the value of the field is `RTR_1`"]
    #[inline(always)]
    pub fn is_rtr_1(&self) -> bool {
        *self == RTR_A::RTR_1
    }
}
#[doc = "Tape Select Change Done\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSCD_A {
    #[doc = "0: Delay cell select change is not finished."]
    TSCD_0 = 0,
    #[doc = "1: Delay cell select change is finished."]
    TSCD_1 = 1,
}
impl From<TSCD_A> for bool {
    #[inline(always)]
    fn from(variant: TSCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSCD`"]
pub type TSCD_R = crate::R<bool, TSCD_A>;
impl TSCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSCD_A {
        match self.bits {
            false => TSCD_A::TSCD_0,
            true => TSCD_A::TSCD_1,
        }
    }
    #[doc = "Checks if the value of the field is `TSCD_0`"]
    #[inline(always)]
    pub fn is_tscd_0(&self) -> bool {
        *self == TSCD_A::TSCD_0
    }
    #[doc = "Checks if the value of the field is `TSCD_1`"]
    #[inline(always)]
    pub fn is_tscd_1(&self) -> bool {
        *self == TSCD_A::TSCD_1
    }
}
#[doc = "Card Inserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINST_A {
    #[doc = "0: Power on Reset or No Card"]
    CINST_0 = 0,
    #[doc = "1: Card Inserted"]
    CINST_1 = 1,
}
impl From<CINST_A> for bool {
    #[inline(always)]
    fn from(variant: CINST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CINST`"]
pub type CINST_R = crate::R<bool, CINST_A>;
impl CINST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINST_A {
        match self.bits {
            false => CINST_A::CINST_0,
            true => CINST_A::CINST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CINST_0`"]
    #[inline(always)]
    pub fn is_cinst_0(&self) -> bool {
        *self == CINST_A::CINST_0
    }
    #[doc = "Checks if the value of the field is `CINST_1`"]
    #[inline(always)]
    pub fn is_cinst_1(&self) -> bool {
        *self == CINST_A::CINST_1
    }
}
#[doc = "Card Detect Pin Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDPL_A {
    #[doc = "0: No card present (CD_B = 1)"]
    CDPL_0 = 0,
    #[doc = "1: Card present (CD_B = 0)"]
    CDPL_1 = 1,
}
impl From<CDPL_A> for bool {
    #[inline(always)]
    fn from(variant: CDPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CDPL`"]
pub type CDPL_R = crate::R<bool, CDPL_A>;
impl CDPL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDPL_A {
        match self.bits {
            false => CDPL_A::CDPL_0,
            true => CDPL_A::CDPL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CDPL_0`"]
    #[inline(always)]
    pub fn is_cdpl_0(&self) -> bool {
        *self == CDPL_A::CDPL_0
    }
    #[doc = "Checks if the value of the field is `CDPL_1`"]
    #[inline(always)]
    pub fn is_cdpl_1(&self) -> bool {
        *self == CDPL_A::CDPL_1
    }
}
#[doc = "Write Protect Switch Pin Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPSPL_A {
    #[doc = "0: Write protected (WP = 1)"]
    WPSPL_0 = 0,
    #[doc = "1: Write enabled (WP = 0)"]
    WPSPL_1 = 1,
}
impl From<WPSPL_A> for bool {
    #[inline(always)]
    fn from(variant: WPSPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WPSPL`"]
pub type WPSPL_R = crate::R<bool, WPSPL_A>;
impl WPSPL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPSPL_A {
        match self.bits {
            false => WPSPL_A::WPSPL_0,
            true => WPSPL_A::WPSPL_1,
        }
    }
    #[doc = "Checks if the value of the field is `WPSPL_0`"]
    #[inline(always)]
    pub fn is_wpspl_0(&self) -> bool {
        *self == WPSPL_A::WPSPL_0
    }
    #[doc = "Checks if the value of the field is `WPSPL_1`"]
    #[inline(always)]
    pub fn is_wpspl_1(&self) -> bool {
        *self == WPSPL_A::WPSPL_1
    }
}
#[doc = "Reader of field `CLSL`"]
pub type CLSL_R = crate::R<bool, bool>;
#[doc = "DATA\\[7:0\\]
Line Signal Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DLSL_A {
    #[doc = "0: Data 0 line signal level"]
    DATA0 = 0,
    #[doc = "1: Data 1 line signal level"]
    DATA1 = 1,
    #[doc = "2: Data 2 line signal level"]
    DATA2 = 2,
    #[doc = "3: Data 3 line signal level"]
    DATA3 = 3,
    #[doc = "4: Data 4 line signal level"]
    DATA4 = 4,
    #[doc = "5: Data 5 line signal level"]
    DATA5 = 5,
    #[doc = "6: Data 6 line signal level"]
    DATA6 = 6,
    #[doc = "7: Data 7 line signal level"]
    DATA7 = 7,
}
impl From<DLSL_A> for u8 {
    #[inline(always)]
    fn from(variant: DLSL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DLSL`"]
pub type DLSL_R = crate::R<u8, DLSL_A>;
impl DLSL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DLSL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DLSL_A::DATA0),
            1 => Val(DLSL_A::DATA1),
            2 => Val(DLSL_A::DATA2),
            3 => Val(DLSL_A::DATA3),
            4 => Val(DLSL_A::DATA4),
            5 => Val(DLSL_A::DATA5),
            6 => Val(DLSL_A::DATA6),
            7 => Val(DLSL_A::DATA7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DLSL_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DLSL_A::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == DLSL_A::DATA2
    }
    #[doc = "Checks if the value of the field is `DATA3`"]
    #[inline(always)]
    pub fn is_data3(&self) -> bool {
        *self == DLSL_A::DATA3
    }
    #[doc = "Checks if the value of the field is `DATA4`"]
    #[inline(always)]
    pub fn is_data4(&self) -> bool {
        *self == DLSL_A::DATA4
    }
    #[doc = "Checks if the value of the field is `DATA5`"]
    #[inline(always)]
    pub fn is_data5(&self) -> bool {
        *self == DLSL_A::DATA5
    }
    #[doc = "Checks if the value of the field is `DATA6`"]
    #[inline(always)]
    pub fn is_data6(&self) -> bool {
        *self == DLSL_A::DATA6
    }
    #[doc = "Checks if the value of the field is `DATA7`"]
    #[inline(always)]
    pub fn is_data7(&self) -> bool {
        *self == DLSL_A::DATA7
    }
}
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn cihb(&self) -> CIHB_R {
        CIHB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DATA)"]
    #[inline(always)]
    pub fn cdihb(&self) -> CDIHB_R {
        CDIHB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Data Line Active"]
    #[inline(always)]
    pub fn dla(&self) -> DLA_R {
        DLA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SD Clock Stable"]
    #[inline(always)]
    pub fn sdstb(&self) -> SDSTB_R {
        SDSTB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IPG_CLK Gated Off Internally"]
    #[inline(always)]
    pub fn ipgoff(&self) -> IPGOFF_R {
        IPGOFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HCLK Gated Off Internally"]
    #[inline(always)]
    pub fn hckoff(&self) -> HCKOFF_R {
        HCKOFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IPG_PERCLK Gated Off Internally"]
    #[inline(always)]
    pub fn peroff(&self) -> PEROFF_R {
        PEROFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SD Clock Gated Off Internally"]
    #[inline(always)]
    pub fn sdoff(&self) -> SDOFF_R {
        SDOFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline(always)]
    pub fn wta(&self) -> WTA_R {
        WTA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline(always)]
    pub fn rta(&self) -> RTA_R {
        RTA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline(always)]
    pub fn bwen(&self) -> BWEN_R {
        BWEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Request (only for SD3.0 SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tape Select Change Done"]
    #[inline(always)]
    pub fn tscd(&self) -> TSCD_R {
        TSCD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline(always)]
    pub fn cinst(&self) -> CINST_R {
        CINST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline(always)]
    pub fn cdpl(&self) -> CDPL_R {
        CDPL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level"]
    #[inline(always)]
    pub fn wpspl(&self) -> WPSPL_R {
        WPSPL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CMD Line Signal Level"]
    #[inline(always)]
    pub fn clsl(&self) -> CLSL_R {
        CLSL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - DATA\\[7:0\\]
Line Signal Level"]
    #[inline(always)]
    pub fn dlsl(&self) -> DLSL_R {
        DLSL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
