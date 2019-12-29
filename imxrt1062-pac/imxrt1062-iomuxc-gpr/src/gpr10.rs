#[doc = "Reader of register GPR10"]
pub type R = crate::R<u32, super::GPR10>;
#[doc = "Writer for register GPR10"]
pub type W = crate::W<u32, super::GPR10>;
#[doc = "Register GPR10 `reset()`'s with value 0x07"]
impl crate::ResetValue for super::GPR10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "ARM non-secure (non-invasive) debug enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NIDEN_A {
    #[doc = "0: Debug turned off."]
    NIDEN_0,
    #[doc = "1: Debug enabled (default)."]
    NIDEN_1,
}
impl From<NIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: NIDEN_A) -> Self {
        match variant {
            NIDEN_A::NIDEN_0 => false,
            NIDEN_A::NIDEN_1 => true,
        }
    }
}
#[doc = "Reader of field `NIDEN`"]
pub type NIDEN_R = crate::R<bool, NIDEN_A>;
impl NIDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NIDEN_A {
        match self.bits {
            false => NIDEN_A::NIDEN_0,
            true => NIDEN_A::NIDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `NIDEN_0`"]
    #[inline(always)]
    pub fn is_niden_0(&self) -> bool {
        *self == NIDEN_A::NIDEN_0
    }
    #[doc = "Checks if the value of the field is `NIDEN_1`"]
    #[inline(always)]
    pub fn is_niden_1(&self) -> bool {
        *self == NIDEN_A::NIDEN_1
    }
}
#[doc = "Write proxy for field `NIDEN`"]
pub struct NIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NIDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Debug turned off."]
    #[inline(always)]
    pub fn niden_0(self) -> &'a mut W {
        self.variant(NIDEN_A::NIDEN_0)
    }
    #[doc = "Debug enabled (default)."]
    #[inline(always)]
    pub fn niden_1(self) -> &'a mut W {
        self.variant(NIDEN_A::NIDEN_1)
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
#[doc = "ARM invasive debug enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_EN_A {
    #[doc = "0: Debug turned off."]
    DBG_EN_0,
    #[doc = "1: Debug enabled (default)."]
    DBG_EN_1,
}
impl From<DBG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_EN_A) -> Self {
        match variant {
            DBG_EN_A::DBG_EN_0 => false,
            DBG_EN_A::DBG_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `DBG_EN`"]
pub type DBG_EN_R = crate::R<bool, DBG_EN_A>;
impl DBG_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_EN_A {
        match self.bits {
            false => DBG_EN_A::DBG_EN_0,
            true => DBG_EN_A::DBG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBG_EN_0`"]
    #[inline(always)]
    pub fn is_dbg_en_0(&self) -> bool {
        *self == DBG_EN_A::DBG_EN_0
    }
    #[doc = "Checks if the value of the field is `DBG_EN_1`"]
    #[inline(always)]
    pub fn is_dbg_en_1(&self) -> bool {
        *self == DBG_EN_A::DBG_EN_1
    }
}
#[doc = "Write proxy for field `DBG_EN`"]
pub struct DBG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Debug turned off."]
    #[inline(always)]
    pub fn dbg_en_0(self) -> &'a mut W {
        self.variant(DBG_EN_A::DBG_EN_0)
    }
    #[doc = "Debug enabled (default)."]
    #[inline(always)]
    pub fn dbg_en_1(self) -> &'a mut W {
        self.variant(DBG_EN_A::DBG_EN_1)
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
#[doc = "Security error response enable for all security gaskets (on both AHB and AXI buses)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_ERR_RESP_A {
    #[doc = "0: OKEY response"]
    SEC_ERR_RESP_0,
    #[doc = "1: SLVError (default)"]
    SEC_ERR_RESP_1,
}
impl From<SEC_ERR_RESP_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_ERR_RESP_A) -> Self {
        match variant {
            SEC_ERR_RESP_A::SEC_ERR_RESP_0 => false,
            SEC_ERR_RESP_A::SEC_ERR_RESP_1 => true,
        }
    }
}
#[doc = "Reader of field `SEC_ERR_RESP`"]
pub type SEC_ERR_RESP_R = crate::R<bool, SEC_ERR_RESP_A>;
impl SEC_ERR_RESP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_ERR_RESP_A {
        match self.bits {
            false => SEC_ERR_RESP_A::SEC_ERR_RESP_0,
            true => SEC_ERR_RESP_A::SEC_ERR_RESP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEC_ERR_RESP_0`"]
    #[inline(always)]
    pub fn is_sec_err_resp_0(&self) -> bool {
        *self == SEC_ERR_RESP_A::SEC_ERR_RESP_0
    }
    #[doc = "Checks if the value of the field is `SEC_ERR_RESP_1`"]
    #[inline(always)]
    pub fn is_sec_err_resp_1(&self) -> bool {
        *self == SEC_ERR_RESP_A::SEC_ERR_RESP_1
    }
}
#[doc = "Write proxy for field `SEC_ERR_RESP`"]
pub struct SEC_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_ERR_RESP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEC_ERR_RESP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OKEY response"]
    #[inline(always)]
    pub fn sec_err_resp_0(self) -> &'a mut W {
        self.variant(SEC_ERR_RESP_A::SEC_ERR_RESP_0)
    }
    #[doc = "SLVError (default)"]
    #[inline(always)]
    pub fn sec_err_resp_1(self) -> &'a mut W {
        self.variant(SEC_ERR_RESP_A::SEC_ERR_RESP_1)
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
#[doc = "DCP Key selection bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCPKEY_OCOTP_OR_KEYMUX_A {
    #[doc = "0: Select key from Key MUX (SNVS/OTPMK)."]
    DCPKEY_OCOTP_OR_KEYMUX_0,
    #[doc = "1: Select key from OCOTP (SW_GP2)."]
    DCPKEY_OCOTP_OR_KEYMUX_1,
}
impl From<DCPKEY_OCOTP_OR_KEYMUX_A> for bool {
    #[inline(always)]
    fn from(variant: DCPKEY_OCOTP_OR_KEYMUX_A) -> Self {
        match variant {
            DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_0 => false,
            DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_1 => true,
        }
    }
}
#[doc = "Reader of field `DCPKEY_OCOTP_OR_KEYMUX`"]
pub type DCPKEY_OCOTP_OR_KEYMUX_R = crate::R<bool, DCPKEY_OCOTP_OR_KEYMUX_A>;
impl DCPKEY_OCOTP_OR_KEYMUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCPKEY_OCOTP_OR_KEYMUX_A {
        match self.bits {
            false => DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_0,
            true => DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCPKEY_OCOTP_OR_KEYMUX_0`"]
    #[inline(always)]
    pub fn is_dcpkey_ocotp_or_keymux_0(&self) -> bool {
        *self == DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_0
    }
    #[doc = "Checks if the value of the field is `DCPKEY_OCOTP_OR_KEYMUX_1`"]
    #[inline(always)]
    pub fn is_dcpkey_ocotp_or_keymux_1(&self) -> bool {
        *self == DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_1
    }
}
#[doc = "Write proxy for field `DCPKEY_OCOTP_OR_KEYMUX`"]
pub struct DCPKEY_OCOTP_OR_KEYMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> DCPKEY_OCOTP_OR_KEYMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCPKEY_OCOTP_OR_KEYMUX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Select key from Key MUX (SNVS/OTPMK)."]
    #[inline(always)]
    pub fn dcpkey_ocotp_or_keymux_0(self) -> &'a mut W {
        self.variant(DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_0)
    }
    #[doc = "Select key from OCOTP (SW_GP2)."]
    #[inline(always)]
    pub fn dcpkey_ocotp_or_keymux_1(self) -> &'a mut W {
        self.variant(DCPKEY_OCOTP_OR_KEYMUX_A::DCPKEY_OCOTP_OR_KEYMUX_1)
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
#[doc = "OCRAM TrustZone (TZ) enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCRAM_TZ_EN_A {
    #[doc = "0: The TrustZone feature is disabled. Entire OCRAM space is available for all access types (secure/non-secure/user/supervisor)."]
    OCRAM_TZ_EN_0,
    #[doc = "1: The TrustZone feature is enabled. Access to address in the range specified by \\[ENDADDR:STARTADDR\\] follows the execution mode access policy described in CSU chapter."]
    OCRAM_TZ_EN_1,
}
impl From<OCRAM_TZ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OCRAM_TZ_EN_A) -> Self {
        match variant {
            OCRAM_TZ_EN_A::OCRAM_TZ_EN_0 => false,
            OCRAM_TZ_EN_A::OCRAM_TZ_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `OCRAM_TZ_EN`"]
pub type OCRAM_TZ_EN_R = crate::R<bool, OCRAM_TZ_EN_A>;
impl OCRAM_TZ_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCRAM_TZ_EN_A {
        match self.bits {
            false => OCRAM_TZ_EN_A::OCRAM_TZ_EN_0,
            true => OCRAM_TZ_EN_A::OCRAM_TZ_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `OCRAM_TZ_EN_0`"]
    #[inline(always)]
    pub fn is_ocram_tz_en_0(&self) -> bool {
        *self == OCRAM_TZ_EN_A::OCRAM_TZ_EN_0
    }
    #[doc = "Checks if the value of the field is `OCRAM_TZ_EN_1`"]
    #[inline(always)]
    pub fn is_ocram_tz_en_1(&self) -> bool {
        *self == OCRAM_TZ_EN_A::OCRAM_TZ_EN_1
    }
}
#[doc = "Write proxy for field `OCRAM_TZ_EN`"]
pub struct OCRAM_TZ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM_TZ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCRAM_TZ_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The TrustZone feature is disabled. Entire OCRAM space is available for all access types (secure/non-secure/user/supervisor)."]
    #[inline(always)]
    pub fn ocram_tz_en_0(self) -> &'a mut W {
        self.variant(OCRAM_TZ_EN_A::OCRAM_TZ_EN_0)
    }
    #[doc = "The TrustZone feature is enabled. Access to address in the range specified by \\[ENDADDR:STARTADDR\\] follows the execution mode access policy described in CSU chapter."]
    #[inline(always)]
    pub fn ocram_tz_en_1(self) -> &'a mut W {
        self.variant(OCRAM_TZ_EN_A::OCRAM_TZ_EN_1)
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
#[doc = "Reader of field `OCRAM_TZ_ADDR`"]
pub type OCRAM_TZ_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OCRAM_TZ_ADDR`"]
pub struct OCRAM_TZ_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM_TZ_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Lock NIDEN field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_NIDEN_A {
    #[doc = "0: Field is not locked"]
    LOCK_NIDEN_0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_NIDEN_1,
}
impl From<LOCK_NIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_NIDEN_A) -> Self {
        match variant {
            LOCK_NIDEN_A::LOCK_NIDEN_0 => false,
            LOCK_NIDEN_A::LOCK_NIDEN_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCK_NIDEN`"]
pub type LOCK_NIDEN_R = crate::R<bool, LOCK_NIDEN_A>;
impl LOCK_NIDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_NIDEN_A {
        match self.bits {
            false => LOCK_NIDEN_A::LOCK_NIDEN_0,
            true => LOCK_NIDEN_A::LOCK_NIDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_NIDEN_0`"]
    #[inline(always)]
    pub fn is_lock_niden_0(&self) -> bool {
        *self == LOCK_NIDEN_A::LOCK_NIDEN_0
    }
    #[doc = "Checks if the value of the field is `LOCK_NIDEN_1`"]
    #[inline(always)]
    pub fn is_lock_niden_1(&self) -> bool {
        *self == LOCK_NIDEN_A::LOCK_NIDEN_1
    }
}
#[doc = "Write proxy for field `LOCK_NIDEN`"]
pub struct LOCK_NIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_NIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_NIDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_niden_0(self) -> &'a mut W {
        self.variant(LOCK_NIDEN_A::LOCK_NIDEN_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_niden_1(self) -> &'a mut W {
        self.variant(LOCK_NIDEN_A::LOCK_NIDEN_1)
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
#[doc = "Lock DBG_EN field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_DBG_EN_A {
    #[doc = "0: Field is not locked"]
    LOCK_DBG_EN_0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_DBG_EN_1,
}
impl From<LOCK_DBG_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_DBG_EN_A) -> Self {
        match variant {
            LOCK_DBG_EN_A::LOCK_DBG_EN_0 => false,
            LOCK_DBG_EN_A::LOCK_DBG_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCK_DBG_EN`"]
pub type LOCK_DBG_EN_R = crate::R<bool, LOCK_DBG_EN_A>;
impl LOCK_DBG_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_DBG_EN_A {
        match self.bits {
            false => LOCK_DBG_EN_A::LOCK_DBG_EN_0,
            true => LOCK_DBG_EN_A::LOCK_DBG_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_DBG_EN_0`"]
    #[inline(always)]
    pub fn is_lock_dbg_en_0(&self) -> bool {
        *self == LOCK_DBG_EN_A::LOCK_DBG_EN_0
    }
    #[doc = "Checks if the value of the field is `LOCK_DBG_EN_1`"]
    #[inline(always)]
    pub fn is_lock_dbg_en_1(&self) -> bool {
        *self == LOCK_DBG_EN_A::LOCK_DBG_EN_1
    }
}
#[doc = "Write proxy for field `LOCK_DBG_EN`"]
pub struct LOCK_DBG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_DBG_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_DBG_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_dbg_en_0(self) -> &'a mut W {
        self.variant(LOCK_DBG_EN_A::LOCK_DBG_EN_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_dbg_en_1(self) -> &'a mut W {
        self.variant(LOCK_DBG_EN_A::LOCK_DBG_EN_1)
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
#[doc = "Lock SEC_ERR_RESP field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_SEC_ERR_RESP_A {
    #[doc = "0: Field is not locked"]
    LOCK_SEC_ERR_RESP_0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_SEC_ERR_RESP_1,
}
impl From<LOCK_SEC_ERR_RESP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_SEC_ERR_RESP_A) -> Self {
        match variant {
            LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_0 => false,
            LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCK_SEC_ERR_RESP`"]
pub type LOCK_SEC_ERR_RESP_R = crate::R<bool, LOCK_SEC_ERR_RESP_A>;
impl LOCK_SEC_ERR_RESP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_SEC_ERR_RESP_A {
        match self.bits {
            false => LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_0,
            true => LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_SEC_ERR_RESP_0`"]
    #[inline(always)]
    pub fn is_lock_sec_err_resp_0(&self) -> bool {
        *self == LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_0
    }
    #[doc = "Checks if the value of the field is `LOCK_SEC_ERR_RESP_1`"]
    #[inline(always)]
    pub fn is_lock_sec_err_resp_1(&self) -> bool {
        *self == LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_1
    }
}
#[doc = "Write proxy for field `LOCK_SEC_ERR_RESP`"]
pub struct LOCK_SEC_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_SEC_ERR_RESP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_SEC_ERR_RESP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_sec_err_resp_0(self) -> &'a mut W {
        self.variant(LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_sec_err_resp_1(self) -> &'a mut W {
        self.variant(LOCK_SEC_ERR_RESP_A::LOCK_SEC_ERR_RESP_1)
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
#[doc = "Lock DCP Key OCOTP/Key MUX selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_DCPKEY_OCOTP_OR_KEYMUX_A {
    #[doc = "0: Field is not locked"]
    LOCK_DCPKEY_OCOTP_OR_KEYMUX_0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_DCPKEY_OCOTP_OR_KEYMUX_1,
}
impl From<LOCK_DCPKEY_OCOTP_OR_KEYMUX_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_DCPKEY_OCOTP_OR_KEYMUX_A) -> Self {
        match variant {
            LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_0 => false,
            LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCK_DCPKEY_OCOTP_OR_KEYMUX`"]
pub type LOCK_DCPKEY_OCOTP_OR_KEYMUX_R = crate::R<bool, LOCK_DCPKEY_OCOTP_OR_KEYMUX_A>;
impl LOCK_DCPKEY_OCOTP_OR_KEYMUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_DCPKEY_OCOTP_OR_KEYMUX_A {
        match self.bits {
            false => LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_0,
            true => LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_DCPKEY_OCOTP_OR_KEYMUX_0`"]
    #[inline(always)]
    pub fn is_lock_dcpkey_ocotp_or_keymux_0(&self) -> bool {
        *self == LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_0
    }
    #[doc = "Checks if the value of the field is `LOCK_DCPKEY_OCOTP_OR_KEYMUX_1`"]
    #[inline(always)]
    pub fn is_lock_dcpkey_ocotp_or_keymux_1(&self) -> bool {
        *self == LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_1
    }
}
#[doc = "Write proxy for field `LOCK_DCPKEY_OCOTP_OR_KEYMUX`"]
pub struct LOCK_DCPKEY_OCOTP_OR_KEYMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_DCPKEY_OCOTP_OR_KEYMUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_DCPKEY_OCOTP_OR_KEYMUX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_dcpkey_ocotp_or_keymux_0(self) -> &'a mut W {
        self.variant(LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_dcpkey_ocotp_or_keymux_1(self) -> &'a mut W {
        self.variant(LOCK_DCPKEY_OCOTP_OR_KEYMUX_A::LOCK_DCPKEY_OCOTP_OR_KEYMUX_1)
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
#[doc = "Lock OCRAM_TZ_EN field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_OCRAM_TZ_EN_A {
    #[doc = "0: Field is not locked"]
    LOCK_OCRAM_TZ_EN_0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_OCRAM_TZ_EN_1,
}
impl From<LOCK_OCRAM_TZ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_OCRAM_TZ_EN_A) -> Self {
        match variant {
            LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_0 => false,
            LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_1 => true,
        }
    }
}
#[doc = "Reader of field `LOCK_OCRAM_TZ_EN`"]
pub type LOCK_OCRAM_TZ_EN_R = crate::R<bool, LOCK_OCRAM_TZ_EN_A>;
impl LOCK_OCRAM_TZ_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_OCRAM_TZ_EN_A {
        match self.bits {
            false => LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_0,
            true => LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM_TZ_EN_0`"]
    #[inline(always)]
    pub fn is_lock_ocram_tz_en_0(&self) -> bool {
        *self == LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_0
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM_TZ_EN_1`"]
    #[inline(always)]
    pub fn is_lock_ocram_tz_en_1(&self) -> bool {
        *self == LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_1
    }
}
#[doc = "Write proxy for field `LOCK_OCRAM_TZ_EN`"]
pub struct LOCK_OCRAM_TZ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_OCRAM_TZ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_OCRAM_TZ_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_ocram_tz_en_0(self) -> &'a mut W {
        self.variant(LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_ocram_tz_en_1(self) -> &'a mut W {
        self.variant(LOCK_OCRAM_TZ_EN_A::LOCK_OCRAM_TZ_EN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Lock OCRAM_TZ_ADDR field for changes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_OCRAM_TZ_ADDR_A {
    #[doc = "0: Field is not locked"]
    LOCK_OCRAM_TZ_ADDR_0,
    #[doc = "1: Field is locked (read access only)"]
    LOCK_OCRAM_TZ_ADDR_1,
}
impl From<LOCK_OCRAM_TZ_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_OCRAM_TZ_ADDR_A) -> Self {
        match variant {
            LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_0 => 0,
            LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_1 => 1,
        }
    }
}
#[doc = "Reader of field `LOCK_OCRAM_TZ_ADDR`"]
pub type LOCK_OCRAM_TZ_ADDR_R = crate::R<u8, LOCK_OCRAM_TZ_ADDR_A>;
impl LOCK_OCRAM_TZ_ADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCK_OCRAM_TZ_ADDR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_0),
            1 => Val(LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM_TZ_ADDR_0`"]
    #[inline(always)]
    pub fn is_lock_ocram_tz_addr_0(&self) -> bool {
        *self == LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_0
    }
    #[doc = "Checks if the value of the field is `LOCK_OCRAM_TZ_ADDR_1`"]
    #[inline(always)]
    pub fn is_lock_ocram_tz_addr_1(&self) -> bool {
        *self == LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_1
    }
}
#[doc = "Write proxy for field `LOCK_OCRAM_TZ_ADDR`"]
pub struct LOCK_OCRAM_TZ_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_OCRAM_TZ_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_OCRAM_TZ_ADDR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Field is not locked"]
    #[inline(always)]
    pub fn lock_ocram_tz_addr_0(self) -> &'a mut W {
        self.variant(LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_0)
    }
    #[doc = "Field is locked (read access only)"]
    #[inline(always)]
    pub fn lock_ocram_tz_addr_1(self) -> &'a mut W {
        self.variant(LOCK_OCRAM_TZ_ADDR_A::LOCK_OCRAM_TZ_ADDR_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ARM non-secure (non-invasive) debug enable"]
    #[inline(always)]
    pub fn niden(&self) -> NIDEN_R {
        NIDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ARM invasive debug enable"]
    #[inline(always)]
    pub fn dbg_en(&self) -> DBG_EN_R {
        DBG_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Security error response enable for all security gaskets (on both AHB and AXI buses)"]
    #[inline(always)]
    pub fn sec_err_resp(&self) -> SEC_ERR_RESP_R {
        SEC_ERR_RESP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DCP Key selection bit."]
    #[inline(always)]
    pub fn dcpkey_ocotp_or_keymux(&self) -> DCPKEY_OCOTP_OR_KEYMUX_R {
        DCPKEY_OCOTP_OR_KEYMUX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OCRAM TrustZone (TZ) enable."]
    #[inline(always)]
    pub fn ocram_tz_en(&self) -> OCRAM_TZ_EN_R {
        OCRAM_TZ_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:15 - OCRAM TrustZone (TZ) start address"]
    #[inline(always)]
    pub fn ocram_tz_addr(&self) -> OCRAM_TZ_ADDR_R {
        OCRAM_TZ_ADDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Lock NIDEN field for changes"]
    #[inline(always)]
    pub fn lock_niden(&self) -> LOCK_NIDEN_R {
        LOCK_NIDEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Lock DBG_EN field for changes"]
    #[inline(always)]
    pub fn lock_dbg_en(&self) -> LOCK_DBG_EN_R {
        LOCK_DBG_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Lock SEC_ERR_RESP field for changes"]
    #[inline(always)]
    pub fn lock_sec_err_resp(&self) -> LOCK_SEC_ERR_RESP_R {
        LOCK_SEC_ERR_RESP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Lock DCP Key OCOTP/Key MUX selection bit"]
    #[inline(always)]
    pub fn lock_dcpkey_ocotp_or_keymux(&self) -> LOCK_DCPKEY_OCOTP_OR_KEYMUX_R {
        LOCK_DCPKEY_OCOTP_OR_KEYMUX_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Lock OCRAM_TZ_EN field for changes"]
    #[inline(always)]
    pub fn lock_ocram_tz_en(&self) -> LOCK_OCRAM_TZ_EN_R {
        LOCK_OCRAM_TZ_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:31 - Lock OCRAM_TZ_ADDR field for changes"]
    #[inline(always)]
    pub fn lock_ocram_tz_addr(&self) -> LOCK_OCRAM_TZ_ADDR_R {
        LOCK_OCRAM_TZ_ADDR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ARM non-secure (non-invasive) debug enable"]
    #[inline(always)]
    pub fn niden(&mut self) -> NIDEN_W {
        NIDEN_W { w: self }
    }
    #[doc = "Bit 1 - ARM invasive debug enable"]
    #[inline(always)]
    pub fn dbg_en(&mut self) -> DBG_EN_W {
        DBG_EN_W { w: self }
    }
    #[doc = "Bit 2 - Security error response enable for all security gaskets (on both AHB and AXI buses)"]
    #[inline(always)]
    pub fn sec_err_resp(&mut self) -> SEC_ERR_RESP_W {
        SEC_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 4 - DCP Key selection bit."]
    #[inline(always)]
    pub fn dcpkey_ocotp_or_keymux(&mut self) -> DCPKEY_OCOTP_OR_KEYMUX_W {
        DCPKEY_OCOTP_OR_KEYMUX_W { w: self }
    }
    #[doc = "Bit 8 - OCRAM TrustZone (TZ) enable."]
    #[inline(always)]
    pub fn ocram_tz_en(&mut self) -> OCRAM_TZ_EN_W {
        OCRAM_TZ_EN_W { w: self }
    }
    #[doc = "Bits 9:15 - OCRAM TrustZone (TZ) start address"]
    #[inline(always)]
    pub fn ocram_tz_addr(&mut self) -> OCRAM_TZ_ADDR_W {
        OCRAM_TZ_ADDR_W { w: self }
    }
    #[doc = "Bit 16 - Lock NIDEN field for changes"]
    #[inline(always)]
    pub fn lock_niden(&mut self) -> LOCK_NIDEN_W {
        LOCK_NIDEN_W { w: self }
    }
    #[doc = "Bit 17 - Lock DBG_EN field for changes"]
    #[inline(always)]
    pub fn lock_dbg_en(&mut self) -> LOCK_DBG_EN_W {
        LOCK_DBG_EN_W { w: self }
    }
    #[doc = "Bit 18 - Lock SEC_ERR_RESP field for changes"]
    #[inline(always)]
    pub fn lock_sec_err_resp(&mut self) -> LOCK_SEC_ERR_RESP_W {
        LOCK_SEC_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 20 - Lock DCP Key OCOTP/Key MUX selection bit"]
    #[inline(always)]
    pub fn lock_dcpkey_ocotp_or_keymux(&mut self) -> LOCK_DCPKEY_OCOTP_OR_KEYMUX_W {
        LOCK_DCPKEY_OCOTP_OR_KEYMUX_W { w: self }
    }
    #[doc = "Bit 24 - Lock OCRAM_TZ_EN field for changes"]
    #[inline(always)]
    pub fn lock_ocram_tz_en(&mut self) -> LOCK_OCRAM_TZ_EN_W {
        LOCK_OCRAM_TZ_EN_W { w: self }
    }
    #[doc = "Bits 25:31 - Lock OCRAM_TZ_ADDR field for changes"]
    #[inline(always)]
    pub fn lock_ocram_tz_addr(&mut self) -> LOCK_OCRAM_TZ_ADDR_W {
        LOCK_OCRAM_TZ_ADDR_W { w: self }
    }
}
