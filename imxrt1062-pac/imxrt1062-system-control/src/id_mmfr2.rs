#[doc = "Reader of register ID_MMFR2"]
pub type R = crate::R<u32, super::ID_MMFR2>;
#[doc = "Indicates the support for Wait For Interrupt (WFI) stalling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WFI_STALL_A {
    #[doc = "0: Not supported"]
    WFI_STALL_0 = 0,
    #[doc = "1: Support for WFI stalling"]
    WFI_STALL_1 = 1,
}
impl From<WFI_STALL_A> for u8 {
    #[inline(always)]
    fn from(variant: WFI_STALL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WFI_STALL`"]
pub type WFI_STALL_R = crate::R<u8, WFI_STALL_A>;
impl WFI_STALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WFI_STALL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WFI_STALL_A::WFI_STALL_0),
            1 => Val(WFI_STALL_A::WFI_STALL_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WFI_STALL_0`"]
    #[inline(always)]
    pub fn is_wfi_stall_0(&self) -> bool {
        *self == WFI_STALL_A::WFI_STALL_0
    }
    #[doc = "Checks if the value of the field is `WFI_STALL_1`"]
    #[inline(always)]
    pub fn is_wfi_stall_1(&self) -> bool {
        *self == WFI_STALL_A::WFI_STALL_1
    }
}
impl R {
    #[doc = "Bits 24:27 - Indicates the support for Wait For Interrupt (WFI) stalling"]
    #[inline(always)]
    pub fn wfi_stall(&self) -> WFI_STALL_R {
        WFI_STALL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
