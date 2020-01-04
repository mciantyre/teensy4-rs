#[doc = "Reader of register CONFIGFLAG"]
pub type R = crate::R<u32, super::CONFIGFLAG>;
#[doc = "Configure Flag Host software sets this bit as the last action in its process of configuring the Host Controller\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CF_A {
    #[doc = "0: Port routing control logic default-routes each port to an implementation dependent classic host controller."]
    CF_0 = 0,
    #[doc = "1: Port routing control logic default-routes all ports to this host controller."]
    CF_1 = 1,
}
impl From<CF_A> for bool {
    #[inline(always)]
    fn from(variant: CF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CF`"]
pub type CF_R = crate::R<bool, CF_A>;
impl CF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CF_A {
        match self.bits {
            false => CF_A::CF_0,
            true => CF_A::CF_1,
        }
    }
    #[doc = "Checks if the value of the field is `CF_0`"]
    #[inline(always)]
    pub fn is_cf_0(&self) -> bool {
        *self == CF_A::CF_0
    }
    #[doc = "Checks if the value of the field is `CF_1`"]
    #[inline(always)]
    pub fn is_cf_1(&self) -> bool {
        *self == CF_A::CF_1
    }
}
impl R {
    #[doc = "Bit 0 - Configure Flag Host software sets this bit as the last action in its process of configuring the Host Controller"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 0x01) != 0)
    }
}
