#[doc = "Reader of register MEASEURE_VALUE"]
pub type R = crate::R<u32, super::MEASEURE_VALUE>;
#[doc = "Reader of field `Y_VALUE`"]
pub type Y_VALUE_R = crate::R<u16, u16>;
#[doc = "Reader of field `X_VALUE`"]
pub type X_VALUE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Y Value"]
    #[inline(always)]
    pub fn y_value(&self) -> Y_VALUE_R {
        Y_VALUE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - X Value"]
    #[inline(always)]
    pub fn x_value(&self) -> X_VALUE_R {
        X_VALUE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
