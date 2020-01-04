#[doc = "Reader of register ES"]
pub type R = crate::R<u32, super::ES>;
#[doc = "Destination Bus Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBE_A {
    #[doc = "0: No destination bus error"]
    DBE_0 = 0,
    #[doc = "1: The last recorded error was a bus error on a destination write"]
    DBE_1 = 1,
}
impl From<DBE_A> for bool {
    #[inline(always)]
    fn from(variant: DBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBE`"]
pub type DBE_R = crate::R<bool, DBE_A>;
impl DBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBE_A {
        match self.bits {
            false => DBE_A::DBE_0,
            true => DBE_A::DBE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBE_0`"]
    #[inline(always)]
    pub fn is_dbe_0(&self) -> bool {
        *self == DBE_A::DBE_0
    }
    #[doc = "Checks if the value of the field is `DBE_1`"]
    #[inline(always)]
    pub fn is_dbe_1(&self) -> bool {
        *self == DBE_A::DBE_1
    }
}
#[doc = "Source Bus Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBE_A {
    #[doc = "0: No source bus error"]
    SBE_0 = 0,
    #[doc = "1: The last recorded error was a bus error on a source read"]
    SBE_1 = 1,
}
impl From<SBE_A> for bool {
    #[inline(always)]
    fn from(variant: SBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBE`"]
pub type SBE_R = crate::R<bool, SBE_A>;
impl SBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBE_A {
        match self.bits {
            false => SBE_A::SBE_0,
            true => SBE_A::SBE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SBE_0`"]
    #[inline(always)]
    pub fn is_sbe_0(&self) -> bool {
        *self == SBE_A::SBE_0
    }
    #[doc = "Checks if the value of the field is `SBE_1`"]
    #[inline(always)]
    pub fn is_sbe_1(&self) -> bool {
        *self == SBE_A::SBE_1
    }
}
#[doc = "Scatter/Gather Configuration Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SGE_A {
    #[doc = "0: No scatter/gather configuration error"]
    SGE_0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_DLASTSGA field. This field is checked at the beginning of a scatter/gather operation after major loop completion if TCDn_CSR\\[ESG\\]
is enabled. TCDn_DLASTSGA is not on a 32 byte boundary."]
    SGE_1 = 1,
}
impl From<SGE_A> for bool {
    #[inline(always)]
    fn from(variant: SGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SGE`"]
pub type SGE_R = crate::R<bool, SGE_A>;
impl SGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGE_A {
        match self.bits {
            false => SGE_A::SGE_0,
            true => SGE_A::SGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SGE_0`"]
    #[inline(always)]
    pub fn is_sge_0(&self) -> bool {
        *self == SGE_A::SGE_0
    }
    #[doc = "Checks if the value of the field is `SGE_1`"]
    #[inline(always)]
    pub fn is_sge_1(&self) -> bool {
        *self == SGE_A::SGE_1
    }
}
#[doc = "NBYTES/CITER Configuration Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCE_A {
    #[doc = "0: No NBYTES/CITER configuration error"]
    NCE_0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields. TCDn_NBYTES is not a multiple of TCDn_ATTR\\[SSIZE\\]
and TCDn_ATTR\\[DSIZE\\], or TCDn_CITER\\[CITER\\]
is equal to zero, or TCDn_CITER\\[ELINK\\]
is not equal to TCDn_BITER\\[ELINK\\]"]
    NCE_1 = 1,
}
impl From<NCE_A> for bool {
    #[inline(always)]
    fn from(variant: NCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NCE`"]
pub type NCE_R = crate::R<bool, NCE_A>;
impl NCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCE_A {
        match self.bits {
            false => NCE_A::NCE_0,
            true => NCE_A::NCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `NCE_0`"]
    #[inline(always)]
    pub fn is_nce_0(&self) -> bool {
        *self == NCE_A::NCE_0
    }
    #[doc = "Checks if the value of the field is `NCE_1`"]
    #[inline(always)]
    pub fn is_nce_1(&self) -> bool {
        *self == NCE_A::NCE_1
    }
}
#[doc = "Destination Offset Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOE_A {
    #[doc = "0: No destination offset configuration error"]
    DOE_0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
    DOE_1 = 1,
}
impl From<DOE_A> for bool {
    #[inline(always)]
    fn from(variant: DOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOE`"]
pub type DOE_R = crate::R<bool, DOE_A>;
impl DOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOE_A {
        match self.bits {
            false => DOE_A::DOE_0,
            true => DOE_A::DOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOE_0`"]
    #[inline(always)]
    pub fn is_doe_0(&self) -> bool {
        *self == DOE_A::DOE_0
    }
    #[doc = "Checks if the value of the field is `DOE_1`"]
    #[inline(always)]
    pub fn is_doe_1(&self) -> bool {
        *self == DOE_A::DOE_1
    }
}
#[doc = "Destination Address Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAE_A {
    #[doc = "0: No destination address configuration error"]
    DAE_0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
    DAE_1 = 1,
}
impl From<DAE_A> for bool {
    #[inline(always)]
    fn from(variant: DAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DAE`"]
pub type DAE_R = crate::R<bool, DAE_A>;
impl DAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAE_A {
        match self.bits {
            false => DAE_A::DAE_0,
            true => DAE_A::DAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DAE_0`"]
    #[inline(always)]
    pub fn is_dae_0(&self) -> bool {
        *self == DAE_A::DAE_0
    }
    #[doc = "Checks if the value of the field is `DAE_1`"]
    #[inline(always)]
    pub fn is_dae_1(&self) -> bool {
        *self == DAE_A::DAE_1
    }
}
#[doc = "Source Offset Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOE_A {
    #[doc = "0: No source offset configuration error"]
    SOE_0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
    SOE_1 = 1,
}
impl From<SOE_A> for bool {
    #[inline(always)]
    fn from(variant: SOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOE`"]
pub type SOE_R = crate::R<bool, SOE_A>;
impl SOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOE_A {
        match self.bits {
            false => SOE_A::SOE_0,
            true => SOE_A::SOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOE_0`"]
    #[inline(always)]
    pub fn is_soe_0(&self) -> bool {
        *self == SOE_A::SOE_0
    }
    #[doc = "Checks if the value of the field is `SOE_1`"]
    #[inline(always)]
    pub fn is_soe_1(&self) -> bool {
        *self == SOE_A::SOE_1
    }
}
#[doc = "Source Address Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAE_A {
    #[doc = "0: No source address configuration error."]
    SAE_0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
    SAE_1 = 1,
}
impl From<SAE_A> for bool {
    #[inline(always)]
    fn from(variant: SAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAE`"]
pub type SAE_R = crate::R<bool, SAE_A>;
impl SAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAE_A {
        match self.bits {
            false => SAE_A::SAE_0,
            true => SAE_A::SAE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAE_0`"]
    #[inline(always)]
    pub fn is_sae_0(&self) -> bool {
        *self == SAE_A::SAE_0
    }
    #[doc = "Checks if the value of the field is `SAE_1`"]
    #[inline(always)]
    pub fn is_sae_1(&self) -> bool {
        *self == SAE_A::SAE_1
    }
}
#[doc = "Reader of field `ERRCHN`"]
pub type ERRCHN_R = crate::R<u8, u8>;
#[doc = "Channel Priority Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPE_A {
    #[doc = "0: No channel priority error"]
    CPE_0 = 0,
    #[doc = "1: The last recorded error was a configuration error in the channel priorities within a group. Channel priorities within a group are not unique."]
    CPE_1 = 1,
}
impl From<CPE_A> for bool {
    #[inline(always)]
    fn from(variant: CPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPE`"]
pub type CPE_R = crate::R<bool, CPE_A>;
impl CPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPE_A {
        match self.bits {
            false => CPE_A::CPE_0,
            true => CPE_A::CPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPE_0`"]
    #[inline(always)]
    pub fn is_cpe_0(&self) -> bool {
        *self == CPE_A::CPE_0
    }
    #[doc = "Checks if the value of the field is `CPE_1`"]
    #[inline(always)]
    pub fn is_cpe_1(&self) -> bool {
        *self == CPE_A::CPE_1
    }
}
#[doc = "Group Priority Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPE_A {
    #[doc = "0: No group priority error"]
    GPE_0 = 0,
    #[doc = "1: The last recorded error was a configuration error among the group priorities. All group priorities are not unique."]
    GPE_1 = 1,
}
impl From<GPE_A> for bool {
    #[inline(always)]
    fn from(variant: GPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPE`"]
pub type GPE_R = crate::R<bool, GPE_A>;
impl GPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPE_A {
        match self.bits {
            false => GPE_A::GPE_0,
            true => GPE_A::GPE_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPE_0`"]
    #[inline(always)]
    pub fn is_gpe_0(&self) -> bool {
        *self == GPE_A::GPE_0
    }
    #[doc = "Checks if the value of the field is `GPE_1`"]
    #[inline(always)]
    pub fn is_gpe_1(&self) -> bool {
        *self == GPE_A::GPE_1
    }
}
#[doc = "Transfer Canceled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECX_A {
    #[doc = "0: No canceled transfers"]
    ECX_0 = 0,
    #[doc = "1: The last recorded entry was a canceled transfer by the error cancel transfer input"]
    ECX_1 = 1,
}
impl From<ECX_A> for bool {
    #[inline(always)]
    fn from(variant: ECX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECX`"]
pub type ECX_R = crate::R<bool, ECX_A>;
impl ECX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECX_A {
        match self.bits {
            false => ECX_A::ECX_0,
            true => ECX_A::ECX_1,
        }
    }
    #[doc = "Checks if the value of the field is `ECX_0`"]
    #[inline(always)]
    pub fn is_ecx_0(&self) -> bool {
        *self == ECX_A::ECX_0
    }
    #[doc = "Checks if the value of the field is `ECX_1`"]
    #[inline(always)]
    pub fn is_ecx_1(&self) -> bool {
        *self == ECX_A::ECX_1
    }
}
#[doc = "VLD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLD_A {
    #[doc = "0: No ERR bits are set."]
    VLD_0 = 0,
    #[doc = "1: At least one ERR bit is set indicating a valid error exists that has not been cleared."]
    VLD_1 = 1,
}
impl From<VLD_A> for bool {
    #[inline(always)]
    fn from(variant: VLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VLD`"]
pub type VLD_R = crate::R<bool, VLD_A>;
impl VLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLD_A {
        match self.bits {
            false => VLD_A::VLD_0,
            true => VLD_A::VLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `VLD_0`"]
    #[inline(always)]
    pub fn is_vld_0(&self) -> bool {
        *self == VLD_A::VLD_0
    }
    #[doc = "Checks if the value of the field is `VLD_1`"]
    #[inline(always)]
    pub fn is_vld_1(&self) -> bool {
        *self == VLD_A::VLD_1
    }
}
impl R {
    #[doc = "Bit 0 - Destination Bus Error"]
    #[inline(always)]
    pub fn dbe(&self) -> DBE_R {
        DBE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Source Bus Error"]
    #[inline(always)]
    pub fn sbe(&self) -> SBE_R {
        SBE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Scatter/Gather Configuration Error"]
    #[inline(always)]
    pub fn sge(&self) -> SGE_R {
        SGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NBYTES/CITER Configuration Error"]
    #[inline(always)]
    pub fn nce(&self) -> NCE_R {
        NCE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Destination Offset Error"]
    #[inline(always)]
    pub fn doe(&self) -> DOE_R {
        DOE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Destination Address Error"]
    #[inline(always)]
    pub fn dae(&self) -> DAE_R {
        DAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Source Offset Error"]
    #[inline(always)]
    pub fn soe(&self) -> SOE_R {
        SOE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Source Address Error"]
    #[inline(always)]
    pub fn sae(&self) -> SAE_R {
        SAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Error Channel Number or Canceled Channel Number"]
    #[inline(always)]
    pub fn errchn(&self) -> ERRCHN_R {
        ERRCHN_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Channel Priority Error"]
    #[inline(always)]
    pub fn cpe(&self) -> CPE_R {
        CPE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Group Priority Error"]
    #[inline(always)]
    pub fn gpe(&self) -> GPE_R {
        GPE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transfer Canceled"]
    #[inline(always)]
    pub fn ecx(&self) -> ECX_R {
        ECX_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 31 - VLD"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
