#[doc = "Reader of register STS0"]
pub type R = crate::R<u32, super::STS0>;
#[doc = "Reader of field `SEQIDLE`"]
pub type SEQIDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARBIDLE`"]
pub type ARBIDLE_R = crate::R<bool, bool>;
#[doc = "This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ARBCMDSRC_A {
    #[doc = "0: Triggered by AHB read command (triggered by AHB read)."]
    ARBCMDSRC_0 = 0,
    #[doc = "1: Triggered by AHB write command (triggered by AHB Write)."]
    ARBCMDSRC_1 = 1,
    #[doc = "2: Triggered by IP command (triggered by setting register bit IPCMD.TRG)."]
    ARBCMDSRC_2 = 2,
    #[doc = "3: Triggered by suspended command (resumed)."]
    ARBCMDSRC_3 = 3,
}
impl From<ARBCMDSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBCMDSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ARBCMDSRC`"]
pub type ARBCMDSRC_R = crate::R<u8, ARBCMDSRC_A>;
impl ARBCMDSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBCMDSRC_A {
        match self.bits {
            0 => ARBCMDSRC_A::ARBCMDSRC_0,
            1 => ARBCMDSRC_A::ARBCMDSRC_1,
            2 => ARBCMDSRC_A::ARBCMDSRC_2,
            3 => ARBCMDSRC_A::ARBCMDSRC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ARBCMDSRC_0`"]
    #[inline(always)]
    pub fn is_arbcmdsrc_0(&self) -> bool {
        *self == ARBCMDSRC_A::ARBCMDSRC_0
    }
    #[doc = "Checks if the value of the field is `ARBCMDSRC_1`"]
    #[inline(always)]
    pub fn is_arbcmdsrc_1(&self) -> bool {
        *self == ARBCMDSRC_A::ARBCMDSRC_1
    }
    #[doc = "Checks if the value of the field is `ARBCMDSRC_2`"]
    #[inline(always)]
    pub fn is_arbcmdsrc_2(&self) -> bool {
        *self == ARBCMDSRC_A::ARBCMDSRC_2
    }
    #[doc = "Checks if the value of the field is `ARBCMDSRC_3`"]
    #[inline(always)]
    pub fn is_arbcmdsrc_3(&self) -> bool {
        *self == ARBCMDSRC_A::ARBCMDSRC_3
    }
}
impl R {
    #[doc = "Bit 0 - This status bit indicates the state machine in SEQ_CTL is idle and there is command sequence executing on FlexSPI interface."]
    #[inline(always)]
    pub fn seqidle(&self) -> SEQIDLE_R {
        SEQIDLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This status bit indicates the state machine in ARB_CTL is busy and there is command sequence granted by arbitrator and not finished yet on FlexSPI interface. When ARB_CTL state (ARBIDLE=0x1) is idle, there will be no transaction on FlexSPI interface also (SEQIDLE=0x1). So this bit should be polled to wait for FlexSPI controller become idle instead of SEQIDLE."]
    #[inline(always)]
    pub fn arbidle(&self) -> ARBIDLE_R {
        ARBIDLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1)."]
    #[inline(always)]
    pub fn arbcmdsrc(&self) -> ARBCMDSRC_R {
        ARBCMDSRC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
