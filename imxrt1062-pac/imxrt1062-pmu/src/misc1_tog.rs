#[doc = "Reader of register MISC1_TOG"]
pub type R = crate::R<u32, super::MISC1_TOG>;
#[doc = "Writer for register MISC1_TOG"]
pub type W = crate::W<u32, super::MISC1_TOG>;
#[doc = "Register MISC1_TOG `reset()`'s with value 0"]
impl crate::ResetValue for super::MISC1_TOG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "This field selects the clk to be routed to anaclk1/1b.Not related to PMU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVDS1_CLK_SEL_A {
    #[doc = "0: Arm PLL"]
    ARM_PLL = 0,
    #[doc = "1: System PLL"]
    SYS_PLL = 1,
    #[doc = "2: ref_pfd4_clk == pll2_pfd0_clk"]
    PFD4 = 2,
    #[doc = "3: ref_pfd5_clk == pll2_pfd1_clk"]
    PFD5 = 3,
    #[doc = "4: ref_pfd6_clk == pll2_pfd2_clk"]
    PFD6 = 4,
    #[doc = "5: ref_pfd7_clk == pll2_pfd3_clk"]
    PFD7 = 5,
    #[doc = "6: Audio PLL"]
    AUDIO_PLL = 6,
    #[doc = "7: Video PLL"]
    VIDEO_PLL = 7,
    #[doc = "9: ethernet ref clock (ENET_PLL)"]
    ETHERNET_REF = 9,
    #[doc = "12: USB1 PLL clock"]
    USB1_PLL = 12,
    #[doc = "13: USB2 PLL clock"]
    USB2_PLL = 13,
    #[doc = "14: ref_pfd0_clk == pll3_pfd0_clk"]
    PFD0 = 14,
    #[doc = "15: ref_pfd1_clk == pll3_pfd1_clk"]
    PFD1 = 15,
    #[doc = "16: ref_pfd2_clk == pll3_pfd2_clk"]
    PFD2 = 16,
    #[doc = "17: ref_pfd3_clk == pll3_pfd3_clk"]
    PFD3 = 17,
    #[doc = "18: xtal (24M)"]
    XTAL = 18,
}
impl From<LVDS1_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVDS1_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LVDS1_CLK_SEL`"]
pub type LVDS1_CLK_SEL_R = crate::R<u8, LVDS1_CLK_SEL_A>;
impl LVDS1_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LVDS1_CLK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LVDS1_CLK_SEL_A::ARM_PLL),
            1 => Val(LVDS1_CLK_SEL_A::SYS_PLL),
            2 => Val(LVDS1_CLK_SEL_A::PFD4),
            3 => Val(LVDS1_CLK_SEL_A::PFD5),
            4 => Val(LVDS1_CLK_SEL_A::PFD6),
            5 => Val(LVDS1_CLK_SEL_A::PFD7),
            6 => Val(LVDS1_CLK_SEL_A::AUDIO_PLL),
            7 => Val(LVDS1_CLK_SEL_A::VIDEO_PLL),
            9 => Val(LVDS1_CLK_SEL_A::ETHERNET_REF),
            12 => Val(LVDS1_CLK_SEL_A::USB1_PLL),
            13 => Val(LVDS1_CLK_SEL_A::USB2_PLL),
            14 => Val(LVDS1_CLK_SEL_A::PFD0),
            15 => Val(LVDS1_CLK_SEL_A::PFD1),
            16 => Val(LVDS1_CLK_SEL_A::PFD2),
            17 => Val(LVDS1_CLK_SEL_A::PFD3),
            18 => Val(LVDS1_CLK_SEL_A::XTAL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ARM_PLL`"]
    #[inline(always)]
    pub fn is_arm_pll(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::ARM_PLL
    }
    #[doc = "Checks if the value of the field is `SYS_PLL`"]
    #[inline(always)]
    pub fn is_sys_pll(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::SYS_PLL
    }
    #[doc = "Checks if the value of the field is `PFD4`"]
    #[inline(always)]
    pub fn is_pfd4(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::PFD4
    }
    #[doc = "Checks if the value of the field is `PFD5`"]
    #[inline(always)]
    pub fn is_pfd5(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::PFD5
    }
    #[doc = "Checks if the value of the field is `PFD6`"]
    #[inline(always)]
    pub fn is_pfd6(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::PFD6
    }
    #[doc = "Checks if the value of the field is `PFD7`"]
    #[inline(always)]
    pub fn is_pfd7(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::PFD7
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL`"]
    #[inline(always)]
    pub fn is_audio_pll(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::AUDIO_PLL
    }
    #[doc = "Checks if the value of the field is `VIDEO_PLL`"]
    #[inline(always)]
    pub fn is_video_pll(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::VIDEO_PLL
    }
    #[doc = "Checks if the value of the field is `ETHERNET_REF`"]
    #[inline(always)]
    pub fn is_ethernet_ref(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::ETHERNET_REF
    }
    #[doc = "Checks if the value of the field is `USB1_PLL`"]
    #[inline(always)]
    pub fn is_usb1_pll(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::USB1_PLL
    }
    #[doc = "Checks if the value of the field is `USB2_PLL`"]
    #[inline(always)]
    pub fn is_usb2_pll(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::USB2_PLL
    }
    #[doc = "Checks if the value of the field is `PFD0`"]
    #[inline(always)]
    pub fn is_pfd0(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::PFD0
    }
    #[doc = "Checks if the value of the field is `PFD1`"]
    #[inline(always)]
    pub fn is_pfd1(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::PFD1
    }
    #[doc = "Checks if the value of the field is `PFD2`"]
    #[inline(always)]
    pub fn is_pfd2(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::PFD2
    }
    #[doc = "Checks if the value of the field is `PFD3`"]
    #[inline(always)]
    pub fn is_pfd3(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::PFD3
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == LVDS1_CLK_SEL_A::XTAL
    }
}
#[doc = "Write proxy for field `LVDS1_CLK_SEL`"]
pub struct LVDS1_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDS1_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDS1_CLK_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Arm PLL"]
    #[inline(always)]
    pub fn arm_pll(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::ARM_PLL)
    }
    #[doc = "System PLL"]
    #[inline(always)]
    pub fn sys_pll(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::SYS_PLL)
    }
    #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
    #[inline(always)]
    pub fn pfd4(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::PFD4)
    }
    #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
    #[inline(always)]
    pub fn pfd5(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::PFD5)
    }
    #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
    #[inline(always)]
    pub fn pfd6(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::PFD6)
    }
    #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
    #[inline(always)]
    pub fn pfd7(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::PFD7)
    }
    #[doc = "Audio PLL"]
    #[inline(always)]
    pub fn audio_pll(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::AUDIO_PLL)
    }
    #[doc = "Video PLL"]
    #[inline(always)]
    pub fn video_pll(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::VIDEO_PLL)
    }
    #[doc = "ethernet ref clock (ENET_PLL)"]
    #[inline(always)]
    pub fn ethernet_ref(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::ETHERNET_REF)
    }
    #[doc = "USB1 PLL clock"]
    #[inline(always)]
    pub fn usb1_pll(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::USB1_PLL)
    }
    #[doc = "USB2 PLL clock"]
    #[inline(always)]
    pub fn usb2_pll(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::USB2_PLL)
    }
    #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
    #[inline(always)]
    pub fn pfd0(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::PFD0)
    }
    #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
    #[inline(always)]
    pub fn pfd1(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::PFD1)
    }
    #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
    #[inline(always)]
    pub fn pfd2(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::PFD2)
    }
    #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
    #[inline(always)]
    pub fn pfd3(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::PFD3)
    }
    #[doc = "xtal (24M)"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(LVDS1_CLK_SEL_A::XTAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "This field selects the clk to be routed to anaclk2/2b.Not related to PMU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVDS2_CLK_SEL_A {
    #[doc = "0: Arm PLL"]
    ARM_PLL = 0,
    #[doc = "1: System PLL"]
    SYS_PLL = 1,
    #[doc = "2: ref_pfd4_clk == pll2_pfd0_clk"]
    PFD4 = 2,
    #[doc = "3: ref_pfd5_clk == pll2_pfd1_clk"]
    PFD5 = 3,
    #[doc = "4: ref_pfd6_clk == pll2_pfd2_clk"]
    PFD6 = 4,
    #[doc = "5: ref_pfd7_clk == pll2_pfd3_clk"]
    PFD7 = 5,
    #[doc = "6: Audio PLL"]
    AUDIO_PLL = 6,
    #[doc = "7: Video PLL"]
    VIDEO_PLL = 7,
    #[doc = "8: MLB PLL"]
    MLB_PLL = 8,
    #[doc = "9: ethernet ref clock (ENET_PLL)"]
    ETHERNET_REF = 9,
    #[doc = "10: PCIe ref clock (125M)"]
    PCIE_REF = 10,
    #[doc = "11: SATA ref clock (100M)"]
    SATA_REF = 11,
    #[doc = "12: USB1 PLL clock"]
    USB1_PLL = 12,
    #[doc = "13: USB2 PLL clock"]
    USB2_PLL = 13,
    #[doc = "14: ref_pfd0_clk == pll3_pfd0_clk"]
    PFD0 = 14,
    #[doc = "15: ref_pfd1_clk == pll3_pfd1_clk"]
    PFD1 = 15,
    #[doc = "16: ref_pfd2_clk == pll3_pfd2_clk"]
    PFD2 = 16,
    #[doc = "17: ref_pfd3_clk == pll3_pfd3_clk"]
    PFD3 = 17,
    #[doc = "18: xtal (24M)"]
    XTAL = 18,
    #[doc = "19: LVDS1 (loopback)"]
    LVDS1 = 19,
    #[doc = "20: LVDS2 (not useful)"]
    LVDS2 = 20,
}
impl From<LVDS2_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVDS2_CLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LVDS2_CLK_SEL`"]
pub type LVDS2_CLK_SEL_R = crate::R<u8, LVDS2_CLK_SEL_A>;
impl LVDS2_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LVDS2_CLK_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LVDS2_CLK_SEL_A::ARM_PLL),
            1 => Val(LVDS2_CLK_SEL_A::SYS_PLL),
            2 => Val(LVDS2_CLK_SEL_A::PFD4),
            3 => Val(LVDS2_CLK_SEL_A::PFD5),
            4 => Val(LVDS2_CLK_SEL_A::PFD6),
            5 => Val(LVDS2_CLK_SEL_A::PFD7),
            6 => Val(LVDS2_CLK_SEL_A::AUDIO_PLL),
            7 => Val(LVDS2_CLK_SEL_A::VIDEO_PLL),
            8 => Val(LVDS2_CLK_SEL_A::MLB_PLL),
            9 => Val(LVDS2_CLK_SEL_A::ETHERNET_REF),
            10 => Val(LVDS2_CLK_SEL_A::PCIE_REF),
            11 => Val(LVDS2_CLK_SEL_A::SATA_REF),
            12 => Val(LVDS2_CLK_SEL_A::USB1_PLL),
            13 => Val(LVDS2_CLK_SEL_A::USB2_PLL),
            14 => Val(LVDS2_CLK_SEL_A::PFD0),
            15 => Val(LVDS2_CLK_SEL_A::PFD1),
            16 => Val(LVDS2_CLK_SEL_A::PFD2),
            17 => Val(LVDS2_CLK_SEL_A::PFD3),
            18 => Val(LVDS2_CLK_SEL_A::XTAL),
            19 => Val(LVDS2_CLK_SEL_A::LVDS1),
            20 => Val(LVDS2_CLK_SEL_A::LVDS2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ARM_PLL`"]
    #[inline(always)]
    pub fn is_arm_pll(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::ARM_PLL
    }
    #[doc = "Checks if the value of the field is `SYS_PLL`"]
    #[inline(always)]
    pub fn is_sys_pll(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::SYS_PLL
    }
    #[doc = "Checks if the value of the field is `PFD4`"]
    #[inline(always)]
    pub fn is_pfd4(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::PFD4
    }
    #[doc = "Checks if the value of the field is `PFD5`"]
    #[inline(always)]
    pub fn is_pfd5(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::PFD5
    }
    #[doc = "Checks if the value of the field is `PFD6`"]
    #[inline(always)]
    pub fn is_pfd6(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::PFD6
    }
    #[doc = "Checks if the value of the field is `PFD7`"]
    #[inline(always)]
    pub fn is_pfd7(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::PFD7
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL`"]
    #[inline(always)]
    pub fn is_audio_pll(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::AUDIO_PLL
    }
    #[doc = "Checks if the value of the field is `VIDEO_PLL`"]
    #[inline(always)]
    pub fn is_video_pll(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::VIDEO_PLL
    }
    #[doc = "Checks if the value of the field is `MLB_PLL`"]
    #[inline(always)]
    pub fn is_mlb_pll(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::MLB_PLL
    }
    #[doc = "Checks if the value of the field is `ETHERNET_REF`"]
    #[inline(always)]
    pub fn is_ethernet_ref(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::ETHERNET_REF
    }
    #[doc = "Checks if the value of the field is `PCIE_REF`"]
    #[inline(always)]
    pub fn is_pcie_ref(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::PCIE_REF
    }
    #[doc = "Checks if the value of the field is `SATA_REF`"]
    #[inline(always)]
    pub fn is_sata_ref(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::SATA_REF
    }
    #[doc = "Checks if the value of the field is `USB1_PLL`"]
    #[inline(always)]
    pub fn is_usb1_pll(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::USB1_PLL
    }
    #[doc = "Checks if the value of the field is `USB2_PLL`"]
    #[inline(always)]
    pub fn is_usb2_pll(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::USB2_PLL
    }
    #[doc = "Checks if the value of the field is `PFD0`"]
    #[inline(always)]
    pub fn is_pfd0(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::PFD0
    }
    #[doc = "Checks if the value of the field is `PFD1`"]
    #[inline(always)]
    pub fn is_pfd1(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::PFD1
    }
    #[doc = "Checks if the value of the field is `PFD2`"]
    #[inline(always)]
    pub fn is_pfd2(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::PFD2
    }
    #[doc = "Checks if the value of the field is `PFD3`"]
    #[inline(always)]
    pub fn is_pfd3(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::PFD3
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::XTAL
    }
    #[doc = "Checks if the value of the field is `LVDS1`"]
    #[inline(always)]
    pub fn is_lvds1(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::LVDS1
    }
    #[doc = "Checks if the value of the field is `LVDS2`"]
    #[inline(always)]
    pub fn is_lvds2(&self) -> bool {
        *self == LVDS2_CLK_SEL_A::LVDS2
    }
}
#[doc = "Write proxy for field `LVDS2_CLK_SEL`"]
pub struct LVDS2_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDS2_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDS2_CLK_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Arm PLL"]
    #[inline(always)]
    pub fn arm_pll(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::ARM_PLL)
    }
    #[doc = "System PLL"]
    #[inline(always)]
    pub fn sys_pll(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::SYS_PLL)
    }
    #[doc = "ref_pfd4_clk == pll2_pfd0_clk"]
    #[inline(always)]
    pub fn pfd4(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::PFD4)
    }
    #[doc = "ref_pfd5_clk == pll2_pfd1_clk"]
    #[inline(always)]
    pub fn pfd5(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::PFD5)
    }
    #[doc = "ref_pfd6_clk == pll2_pfd2_clk"]
    #[inline(always)]
    pub fn pfd6(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::PFD6)
    }
    #[doc = "ref_pfd7_clk == pll2_pfd3_clk"]
    #[inline(always)]
    pub fn pfd7(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::PFD7)
    }
    #[doc = "Audio PLL"]
    #[inline(always)]
    pub fn audio_pll(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::AUDIO_PLL)
    }
    #[doc = "Video PLL"]
    #[inline(always)]
    pub fn video_pll(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::VIDEO_PLL)
    }
    #[doc = "MLB PLL"]
    #[inline(always)]
    pub fn mlb_pll(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::MLB_PLL)
    }
    #[doc = "ethernet ref clock (ENET_PLL)"]
    #[inline(always)]
    pub fn ethernet_ref(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::ETHERNET_REF)
    }
    #[doc = "PCIe ref clock (125M)"]
    #[inline(always)]
    pub fn pcie_ref(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::PCIE_REF)
    }
    #[doc = "SATA ref clock (100M)"]
    #[inline(always)]
    pub fn sata_ref(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::SATA_REF)
    }
    #[doc = "USB1 PLL clock"]
    #[inline(always)]
    pub fn usb1_pll(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::USB1_PLL)
    }
    #[doc = "USB2 PLL clock"]
    #[inline(always)]
    pub fn usb2_pll(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::USB2_PLL)
    }
    #[doc = "ref_pfd0_clk == pll3_pfd0_clk"]
    #[inline(always)]
    pub fn pfd0(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::PFD0)
    }
    #[doc = "ref_pfd1_clk == pll3_pfd1_clk"]
    #[inline(always)]
    pub fn pfd1(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::PFD1)
    }
    #[doc = "ref_pfd2_clk == pll3_pfd2_clk"]
    #[inline(always)]
    pub fn pfd2(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::PFD2)
    }
    #[doc = "ref_pfd3_clk == pll3_pfd3_clk"]
    #[inline(always)]
    pub fn pfd3(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::PFD3)
    }
    #[doc = "xtal (24M)"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::XTAL)
    }
    #[doc = "LVDS1 (loopback)"]
    #[inline(always)]
    pub fn lvds1(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::LVDS1)
    }
    #[doc = "LVDS2 (not useful)"]
    #[inline(always)]
    pub fn lvds2(self) -> &'a mut W {
        self.variant(LVDS2_CLK_SEL_A::LVDS2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `LVDSCLK1_OBEN`"]
pub type LVDSCLK1_OBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVDSCLK1_OBEN`"]
pub struct LVDSCLK1_OBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDSCLK1_OBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `LVDSCLK2_OBEN`"]
pub type LVDSCLK2_OBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVDSCLK2_OBEN`"]
pub struct LVDSCLK2_OBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDSCLK2_OBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `LVDSCLK1_IBEN`"]
pub type LVDSCLK1_IBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVDSCLK1_IBEN`"]
pub struct LVDSCLK1_IBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDSCLK1_IBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `LVDSCLK2_IBEN`"]
pub type LVDSCLK2_IBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LVDSCLK2_IBEN`"]
pub struct LVDSCLK2_IBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDSCLK2_IBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PFD_480_AUTOGATE_EN`"]
pub type PFD_480_AUTOGATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFD_480_AUTOGATE_EN`"]
pub struct PFD_480_AUTOGATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD_480_AUTOGATE_EN_W<'a> {
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
#[doc = "Reader of field `PFD_528_AUTOGATE_EN`"]
pub type PFD_528_AUTOGATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PFD_528_AUTOGATE_EN`"]
pub struct PFD_528_AUTOGATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PFD_528_AUTOGATE_EN_W<'a> {
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
#[doc = "Reader of field `IRQ_TEMPPANIC`"]
pub type IRQ_TEMPPANIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ_TEMPPANIC`"]
pub struct IRQ_TEMPPANIC_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_TEMPPANIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `IRQ_TEMPLOW`"]
pub type IRQ_TEMPLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ_TEMPLOW`"]
pub struct IRQ_TEMPLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_TEMPLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `IRQ_TEMPHIGH`"]
pub type IRQ_TEMPHIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ_TEMPHIGH`"]
pub struct IRQ_TEMPHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_TEMPHIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `IRQ_ANA_BO`"]
pub type IRQ_ANA_BO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ_ANA_BO`"]
pub struct IRQ_ANA_BO_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_ANA_BO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `IRQ_DIG_BO`"]
pub type IRQ_DIG_BO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ_DIG_BO`"]
pub struct IRQ_DIG_BO_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_DIG_BO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
    #[inline(always)]
    pub fn lvds1_clk_sel(&self) -> LVDS1_CLK_SEL_R {
        LVDS1_CLK_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    #[inline(always)]
    pub fn lvds2_clk_sel(&self) -> LVDS2_CLK_SEL_R {
        LVDS2_CLK_SEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - This enables the LVDS output buffer for anaclk1/1b"]
    #[inline(always)]
    pub fn lvdsclk1_oben(&self) -> LVDSCLK1_OBEN_R {
        LVDSCLK1_OBEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This enables the LVDS output buffer for anaclk2/2b"]
    #[inline(always)]
    pub fn lvdsclk2_oben(&self) -> LVDSCLK2_OBEN_R {
        LVDSCLK2_OBEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This enables the LVDS input buffer for anaclk1/1b"]
    #[inline(always)]
    pub fn lvdsclk1_iben(&self) -> LVDSCLK1_IBEN_R {
        LVDSCLK1_IBEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This enables the LVDS input buffer for anaclk2/2b"]
    #[inline(always)]
    pub fn lvdsclk2_iben(&self) -> LVDSCLK2_IBEN_R {
        LVDSCLK2_IBEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub fn pfd_480_autogate_en(&self) -> PFD_480_AUTOGATE_EN_R {
        PFD_480_AUTOGATE_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub fn pfd_528_autogate_en(&self) -> PFD_528_AUTOGATE_EN_R {
        PFD_528_AUTOGATE_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 27 - This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub fn irq_temppanic(&self) -> IRQ_TEMPPANIC_R {
        IRQ_TEMPPANIC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub fn irq_templow(&self) -> IRQ_TEMPLOW_R {
        IRQ_TEMPLOW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub fn irq_temphigh(&self) -> IRQ_TEMPHIGH_R {
        IRQ_TEMPHIGH_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub fn irq_ana_bo(&self) -> IRQ_ANA_BO_R {
        IRQ_ANA_BO_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub fn irq_dig_bo(&self) -> IRQ_DIG_BO_R {
        IRQ_DIG_BO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - This field selects the clk to be routed to anaclk1/1b.Not related to PMU."]
    #[inline(always)]
    pub fn lvds1_clk_sel(&mut self) -> LVDS1_CLK_SEL_W {
        LVDS1_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 5:9 - This field selects the clk to be routed to anaclk2/2b.Not related to PMU."]
    #[inline(always)]
    pub fn lvds2_clk_sel(&mut self) -> LVDS2_CLK_SEL_W {
        LVDS2_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 10 - This enables the LVDS output buffer for anaclk1/1b"]
    #[inline(always)]
    pub fn lvdsclk1_oben(&mut self) -> LVDSCLK1_OBEN_W {
        LVDSCLK1_OBEN_W { w: self }
    }
    #[doc = "Bit 11 - This enables the LVDS output buffer for anaclk2/2b"]
    #[inline(always)]
    pub fn lvdsclk2_oben(&mut self) -> LVDSCLK2_OBEN_W {
        LVDSCLK2_OBEN_W { w: self }
    }
    #[doc = "Bit 12 - This enables the LVDS input buffer for anaclk1/1b"]
    #[inline(always)]
    pub fn lvdsclk1_iben(&mut self) -> LVDSCLK1_IBEN_W {
        LVDSCLK1_IBEN_W { w: self }
    }
    #[doc = "Bit 13 - This enables the LVDS input buffer for anaclk2/2b"]
    #[inline(always)]
    pub fn lvdsclk2_iben(&mut self) -> LVDSCLK2_IBEN_W {
        LVDSCLK2_IBEN_W { w: self }
    }
    #[doc = "Bit 16 - This enables a feature that will clkgate (reset) all PFD_480 clocks anytime the USB1_PLL_480 is unlocked or powered off"]
    #[inline(always)]
    pub fn pfd_480_autogate_en(&mut self) -> PFD_480_AUTOGATE_EN_W {
        PFD_480_AUTOGATE_EN_W { w: self }
    }
    #[doc = "Bit 17 - This enables a feature that will clkgate (reset) all PFD_528 clocks anytime the PLL_528 is unlocked or powered off"]
    #[inline(always)]
    pub fn pfd_528_autogate_en(&mut self) -> PFD_528_AUTOGATE_EN_W {
        PFD_528_AUTOGATE_EN_W { w: self }
    }
    #[doc = "Bit 27 - This status bit is set to one when the temperature sensor panic interrupt asserts for a panic high temperature"]
    #[inline(always)]
    pub fn irq_temppanic(&mut self) -> IRQ_TEMPPANIC_W {
        IRQ_TEMPPANIC_W { w: self }
    }
    #[doc = "Bit 28 - This status bit is set to one when the temperature sensor low interrupt asserts for low temperature"]
    #[inline(always)]
    pub fn irq_templow(&mut self) -> IRQ_TEMPLOW_W {
        IRQ_TEMPLOW_W { w: self }
    }
    #[doc = "Bit 29 - This status bit is set to one when the temperature sensor high interrupt asserts for high temperature"]
    #[inline(always)]
    pub fn irq_temphigh(&mut self) -> IRQ_TEMPHIGH_W {
        IRQ_TEMPHIGH_W { w: self }
    }
    #[doc = "Bit 30 - This status bit is set to one when when any of the analog regulator brownout interrupts assert"]
    #[inline(always)]
    pub fn irq_ana_bo(&mut self) -> IRQ_ANA_BO_W {
        IRQ_ANA_BO_W { w: self }
    }
    #[doc = "Bit 31 - This status bit is set to one when when any of the digital regulator brownout interrupts assert"]
    #[inline(always)]
    pub fn irq_dig_bo(&mut self) -> IRQ_DIG_BO_W {
        IRQ_DIG_BO_W { w: self }
    }
}
