#[doc = "Reader of register ESR2"]
pub type R = crate::R<u32, super::ESR2>;
#[doc = "Inactive Mailbox\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMB_A {
    #[doc = "0: If CAN_ESR2\\[VPS\\]
is asserted, the CAN_ESR2\\[LPTM\\]
is not an inactive Mailbox."]
    IMB_0 = 0,
    #[doc = "1: If CAN_ESR2\\[VPS\\]
is asserted, there is at least one inactive Mailbox. LPTM content is the number of the first one."]
    IMB_1 = 1,
}
impl From<IMB_A> for bool {
    #[inline(always)]
    fn from(variant: IMB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IMB`"]
pub type IMB_R = crate::R<bool, IMB_A>;
impl IMB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMB_A {
        match self.bits {
            false => IMB_A::IMB_0,
            true => IMB_A::IMB_1,
        }
    }
    #[doc = "Checks if the value of the field is `IMB_0`"]
    #[inline(always)]
    pub fn is_imb_0(&self) -> bool {
        *self == IMB_A::IMB_0
    }
    #[doc = "Checks if the value of the field is `IMB_1`"]
    #[inline(always)]
    pub fn is_imb_1(&self) -> bool {
        *self == IMB_A::IMB_1
    }
}
#[doc = "Valid Priority Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VPS_A {
    #[doc = "0: Contents of IMB and LPTM are invalid."]
    VPS_0 = 0,
    #[doc = "1: Contents of IMB and LPTM are valid."]
    VPS_1 = 1,
}
impl From<VPS_A> for bool {
    #[inline(always)]
    fn from(variant: VPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VPS`"]
pub type VPS_R = crate::R<bool, VPS_A>;
impl VPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPS_A {
        match self.bits {
            false => VPS_A::VPS_0,
            true => VPS_A::VPS_1,
        }
    }
    #[doc = "Checks if the value of the field is `VPS_0`"]
    #[inline(always)]
    pub fn is_vps_0(&self) -> bool {
        *self == VPS_A::VPS_0
    }
    #[doc = "Checks if the value of the field is `VPS_1`"]
    #[inline(always)]
    pub fn is_vps_1(&self) -> bool {
        *self == VPS_A::VPS_1
    }
}
#[doc = "Reader of field `LPTM`"]
pub type LPTM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 13 - Inactive Mailbox"]
    #[inline(always)]
    pub fn imb(&self) -> IMB_R {
        IMB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Valid Priority Status"]
    #[inline(always)]
    pub fn vps(&self) -> VPS_R {
        VPS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Lowest Priority Tx Mailbox"]
    #[inline(always)]
    pub fn lptm(&self) -> LPTM_R {
        LPTM_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
