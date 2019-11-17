#[doc = "Reader of register CDHIPR"]
pub type R = crate::R<u32, super::CDHIPR>;
#[doc = "Busy indicator for semc_podf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEMC_PODF_BUSY_A {
    #[doc = "0: divider is not busy and its value represents the actual division."]
    SEMC_PODF_BUSY_0,
    #[doc = "1: divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the semc_podf will be applied."]
    SEMC_PODF_BUSY_1,
}
impl From<SEMC_PODF_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: SEMC_PODF_BUSY_A) -> Self {
        match variant {
            SEMC_PODF_BUSY_A::SEMC_PODF_BUSY_0 => false,
            SEMC_PODF_BUSY_A::SEMC_PODF_BUSY_1 => true,
        }
    }
}
#[doc = "Reader of field `SEMC_PODF_BUSY`"]
pub type SEMC_PODF_BUSY_R = crate::R<bool, SEMC_PODF_BUSY_A>;
impl SEMC_PODF_BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEMC_PODF_BUSY_A {
        match self.bits {
            false => SEMC_PODF_BUSY_A::SEMC_PODF_BUSY_0,
            true => SEMC_PODF_BUSY_A::SEMC_PODF_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_BUSY_0`"]
    #[inline(always)]
    pub fn is_semc_podf_busy_0(&self) -> bool {
        *self == SEMC_PODF_BUSY_A::SEMC_PODF_BUSY_0
    }
    #[doc = "Checks if the value of the field is `SEMC_PODF_BUSY_1`"]
    #[inline(always)]
    pub fn is_semc_podf_busy_1(&self) -> bool {
        *self == SEMC_PODF_BUSY_A::SEMC_PODF_BUSY_1
    }
}
#[doc = "Busy indicator for ahb_podf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_PODF_BUSY_A {
    #[doc = "0: divider is not busy and its value represents the actual division."]
    AHB_PODF_BUSY_0,
    #[doc = "1: divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the ahb_podf will be applied."]
    AHB_PODF_BUSY_1,
}
impl From<AHB_PODF_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_PODF_BUSY_A) -> Self {
        match variant {
            AHB_PODF_BUSY_A::AHB_PODF_BUSY_0 => false,
            AHB_PODF_BUSY_A::AHB_PODF_BUSY_1 => true,
        }
    }
}
#[doc = "Reader of field `AHB_PODF_BUSY`"]
pub type AHB_PODF_BUSY_R = crate::R<bool, AHB_PODF_BUSY_A>;
impl AHB_PODF_BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_PODF_BUSY_A {
        match self.bits {
            false => AHB_PODF_BUSY_A::AHB_PODF_BUSY_0,
            true => AHB_PODF_BUSY_A::AHB_PODF_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_BUSY_0`"]
    #[inline(always)]
    pub fn is_ahb_podf_busy_0(&self) -> bool {
        *self == AHB_PODF_BUSY_A::AHB_PODF_BUSY_0
    }
    #[doc = "Checks if the value of the field is `AHB_PODF_BUSY_1`"]
    #[inline(always)]
    pub fn is_ahb_podf_busy_1(&self) -> bool {
        *self == AHB_PODF_BUSY_A::AHB_PODF_BUSY_1
    }
}
#[doc = "Busy indicator for periph2_clk_sel mux control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPH2_CLK_SEL_BUSY_A {
    #[doc = "0: mux is not busy and its value represents the actual division."]
    PERIPH2_CLK_SEL_BUSY_0,
    #[doc = "1: mux is busy with handshake process with module. The value read in the periph2_clk_sel represents the previous value of select, and after the handshake periph2_clk_sel value will be applied."]
    PERIPH2_CLK_SEL_BUSY_1,
}
impl From<PERIPH2_CLK_SEL_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPH2_CLK_SEL_BUSY_A) -> Self {
        match variant {
            PERIPH2_CLK_SEL_BUSY_A::PERIPH2_CLK_SEL_BUSY_0 => false,
            PERIPH2_CLK_SEL_BUSY_A::PERIPH2_CLK_SEL_BUSY_1 => true,
        }
    }
}
#[doc = "Reader of field `PERIPH2_CLK_SEL_BUSY`"]
pub type PERIPH2_CLK_SEL_BUSY_R = crate::R<bool, PERIPH2_CLK_SEL_BUSY_A>;
impl PERIPH2_CLK_SEL_BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPH2_CLK_SEL_BUSY_A {
        match self.bits {
            false => PERIPH2_CLK_SEL_BUSY_A::PERIPH2_CLK_SEL_BUSY_0,
            true => PERIPH2_CLK_SEL_BUSY_A::PERIPH2_CLK_SEL_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH2_CLK_SEL_BUSY_0`"]
    #[inline(always)]
    pub fn is_periph2_clk_sel_busy_0(&self) -> bool {
        *self == PERIPH2_CLK_SEL_BUSY_A::PERIPH2_CLK_SEL_BUSY_0
    }
    #[doc = "Checks if the value of the field is `PERIPH2_CLK_SEL_BUSY_1`"]
    #[inline(always)]
    pub fn is_periph2_clk_sel_busy_1(&self) -> bool {
        *self == PERIPH2_CLK_SEL_BUSY_A::PERIPH2_CLK_SEL_BUSY_1
    }
}
#[doc = "Busy indicator for periph_clk_sel mux control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPH_CLK_SEL_BUSY_A {
    #[doc = "0: mux is not busy and its value represents the actual division."]
    PERIPH_CLK_SEL_BUSY_0,
    #[doc = "1: mux is busy with handshake process with module. The value read in the periph_clk_sel represents the previous value of select, and after the handshake periph_clk_sel value will be applied."]
    PERIPH_CLK_SEL_BUSY_1,
}
impl From<PERIPH_CLK_SEL_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPH_CLK_SEL_BUSY_A) -> Self {
        match variant {
            PERIPH_CLK_SEL_BUSY_A::PERIPH_CLK_SEL_BUSY_0 => false,
            PERIPH_CLK_SEL_BUSY_A::PERIPH_CLK_SEL_BUSY_1 => true,
        }
    }
}
#[doc = "Reader of field `PERIPH_CLK_SEL_BUSY`"]
pub type PERIPH_CLK_SEL_BUSY_R = crate::R<bool, PERIPH_CLK_SEL_BUSY_A>;
impl PERIPH_CLK_SEL_BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPH_CLK_SEL_BUSY_A {
        match self.bits {
            false => PERIPH_CLK_SEL_BUSY_A::PERIPH_CLK_SEL_BUSY_0,
            true => PERIPH_CLK_SEL_BUSY_A::PERIPH_CLK_SEL_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_BUSY_0`"]
    #[inline(always)]
    pub fn is_periph_clk_sel_busy_0(&self) -> bool {
        *self == PERIPH_CLK_SEL_BUSY_A::PERIPH_CLK_SEL_BUSY_0
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK_SEL_BUSY_1`"]
    #[inline(always)]
    pub fn is_periph_clk_sel_busy_1(&self) -> bool {
        *self == PERIPH_CLK_SEL_BUSY_A::PERIPH_CLK_SEL_BUSY_1
    }
}
#[doc = "Busy indicator for arm_podf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARM_PODF_BUSY_A {
    #[doc = "0: divider is not busy and its value represents the actual division."]
    ARM_PODF_BUSY_0,
    #[doc = "1: divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the arm_podf will be applied."]
    ARM_PODF_BUSY_1,
}
impl From<ARM_PODF_BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: ARM_PODF_BUSY_A) -> Self {
        match variant {
            ARM_PODF_BUSY_A::ARM_PODF_BUSY_0 => false,
            ARM_PODF_BUSY_A::ARM_PODF_BUSY_1 => true,
        }
    }
}
#[doc = "Reader of field `ARM_PODF_BUSY`"]
pub type ARM_PODF_BUSY_R = crate::R<bool, ARM_PODF_BUSY_A>;
impl ARM_PODF_BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARM_PODF_BUSY_A {
        match self.bits {
            false => ARM_PODF_BUSY_A::ARM_PODF_BUSY_0,
            true => ARM_PODF_BUSY_A::ARM_PODF_BUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_BUSY_0`"]
    #[inline(always)]
    pub fn is_arm_podf_busy_0(&self) -> bool {
        *self == ARM_PODF_BUSY_A::ARM_PODF_BUSY_0
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_BUSY_1`"]
    #[inline(always)]
    pub fn is_arm_podf_busy_1(&self) -> bool {
        *self == ARM_PODF_BUSY_A::ARM_PODF_BUSY_1
    }
}
impl R {
    #[doc = "Bit 0 - Busy indicator for semc_podf."]
    #[inline(always)]
    pub fn semc_podf_busy(&self) -> SEMC_PODF_BUSY_R {
        SEMC_PODF_BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Busy indicator for ahb_podf."]
    #[inline(always)]
    pub fn ahb_podf_busy(&self) -> AHB_PODF_BUSY_R {
        AHB_PODF_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Busy indicator for periph2_clk_sel mux control."]
    #[inline(always)]
    pub fn periph2_clk_sel_busy(&self) -> PERIPH2_CLK_SEL_BUSY_R {
        PERIPH2_CLK_SEL_BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Busy indicator for periph_clk_sel mux control."]
    #[inline(always)]
    pub fn periph_clk_sel_busy(&self) -> PERIPH_CLK_SEL_BUSY_R {
        PERIPH_CLK_SEL_BUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Busy indicator for arm_podf."]
    #[inline(always)]
    pub fn arm_podf_busy(&self) -> ARM_PODF_BUSY_R {
        ARM_PODF_BUSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
