#[doc = "Reader of register STS2"]
pub type R = crate::R<u32, super::STS2>;
#[doc = "This field indicating whether there is pending AXI command (write) to NAND device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDWRPEND_A {
    #[doc = "0: No pending"]
    NDWRPEND_0 = 0,
    #[doc = "1: Pending"]
    NDWRPEND_1 = 1,
}
impl From<NDWRPEND_A> for bool {
    #[inline(always)]
    fn from(variant: NDWRPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NDWRPEND`"]
pub type NDWRPEND_R = crate::R<bool, NDWRPEND_A>;
impl NDWRPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDWRPEND_A {
        match self.bits {
            false => NDWRPEND_A::NDWRPEND_0,
            true => NDWRPEND_A::NDWRPEND_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDWRPEND_0`"]
    #[inline(always)]
    pub fn is_ndwrpend_0(&self) -> bool {
        *self == NDWRPEND_A::NDWRPEND_0
    }
    #[doc = "Checks if the value of the field is `NDWRPEND_1`"]
    #[inline(always)]
    pub fn is_ndwrpend_1(&self) -> bool {
        *self == NDWRPEND_A::NDWRPEND_1
    }
}
impl R {
    #[doc = "Bit 3 - This field indicating whether there is pending AXI command (write) to NAND device."]
    #[inline(always)]
    pub fn ndwrpend(&self) -> NDWRPEND_R {
        NDWRPEND_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
