#[doc = "Reader of register HWHOST"]
pub type R = crate::R<u32, super::HWHOST>;
#[doc = "Host Capable. Indicating whether host operation mode is supported or not.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HC_A {
    #[doc = "0: Not supported"]
    HC_0 = 0,
    #[doc = "1: Supported"]
    HC_1 = 1,
}
impl From<HC_A> for bool {
    #[inline(always)]
    fn from(variant: HC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HC`"]
pub type HC_R = crate::R<bool, HC_A>;
impl HC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HC_A {
        match self.bits {
            false => HC_A::HC_0,
            true => HC_A::HC_1,
        }
    }
    #[doc = "Checks if the value of the field is `HC_0`"]
    #[inline(always)]
    pub fn is_hc_0(&self) -> bool {
        *self == HC_A::HC_0
    }
    #[doc = "Checks if the value of the field is `HC_1`"]
    #[inline(always)]
    pub fn is_hc_1(&self) -> bool {
        *self == HC_A::HC_1
    }
}
#[doc = "Reader of field `NPORT`"]
pub type NPORT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Host Capable. Indicating whether host operation mode is supported or not."]
    #[inline(always)]
    pub fn hc(&self) -> HC_R {
        HC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - The Nmber of downstream ports supported by the host controller is (NPORT+1)"]
    #[inline(always)]
    pub fn nport(&self) -> NPORT_R {
        NPORT_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
