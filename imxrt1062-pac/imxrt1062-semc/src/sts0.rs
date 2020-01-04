#[doc = "Reader of register STS0"]
pub type R = crate::R<u32, super::STS0>;
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, bool>;
#[doc = "Indicating NAND device Ready/WAIT# pin level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NARDY_A {
    #[doc = "0: NAND device is not ready"]
    NARDY_0 = 0,
    #[doc = "1: NAND device is ready"]
    NARDY_1 = 1,
}
impl From<NARDY_A> for bool {
    #[inline(always)]
    fn from(variant: NARDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NARDY`"]
pub type NARDY_R = crate::R<bool, NARDY_A>;
impl NARDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NARDY_A {
        match self.bits {
            false => NARDY_A::NARDY_0,
            true => NARDY_A::NARDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `NARDY_0`"]
    #[inline(always)]
    pub fn is_nardy_0(&self) -> bool {
        *self == NARDY_A::NARDY_0
    }
    #[doc = "Checks if the value of the field is `NARDY_1`"]
    #[inline(always)]
    pub fn is_nardy_1(&self) -> bool {
        *self == NARDY_A::NARDY_1
    }
}
impl R {
    #[doc = "Bit 0 - Indicating whether SEMC is in IDLE state."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicating NAND device Ready/WAIT# pin level."]
    #[inline(always)]
    pub fn nardy(&self) -> NARDY_R {
        NARDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
