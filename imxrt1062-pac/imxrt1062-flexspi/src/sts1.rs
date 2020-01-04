#[doc = "Reader of register STS1"]
pub type R = crate::R<u32, super::STS1>;
#[doc = "Reader of field `AHBCMDERRID`"]
pub type AHBCMDERRID_R = crate::R<u8, u8>;
#[doc = "Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AHBCMDERRCODE_A {
    #[doc = "0: No error."]
    AHBCMDERRCODE_0 = 0,
    #[doc = "2: AHB Write command with JMP_ON_CS instruction used in the sequence."]
    AHBCMDERRCODE_2 = 2,
    #[doc = "3: There is unknown instruction opcode in the sequence."]
    AHBCMDERRCODE_3 = 3,
    #[doc = "4: Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    AHBCMDERRCODE_4 = 4,
    #[doc = "5: Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    AHBCMDERRCODE_5 = 5,
    #[doc = "14: Sequence execution timeout."]
    AHBCMDERRCODE_14 = 14,
}
impl From<AHBCMDERRCODE_A> for u8 {
    #[inline(always)]
    fn from(variant: AHBCMDERRCODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AHBCMDERRCODE`"]
pub type AHBCMDERRCODE_R = crate::R<u8, AHBCMDERRCODE_A>;
impl AHBCMDERRCODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AHBCMDERRCODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AHBCMDERRCODE_A::AHBCMDERRCODE_0),
            2 => Val(AHBCMDERRCODE_A::AHBCMDERRCODE_2),
            3 => Val(AHBCMDERRCODE_A::AHBCMDERRCODE_3),
            4 => Val(AHBCMDERRCODE_A::AHBCMDERRCODE_4),
            5 => Val(AHBCMDERRCODE_A::AHBCMDERRCODE_5),
            14 => Val(AHBCMDERRCODE_A::AHBCMDERRCODE_14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AHBCMDERRCODE_0`"]
    #[inline(always)]
    pub fn is_ahbcmderrcode_0(&self) -> bool {
        *self == AHBCMDERRCODE_A::AHBCMDERRCODE_0
    }
    #[doc = "Checks if the value of the field is `AHBCMDERRCODE_2`"]
    #[inline(always)]
    pub fn is_ahbcmderrcode_2(&self) -> bool {
        *self == AHBCMDERRCODE_A::AHBCMDERRCODE_2
    }
    #[doc = "Checks if the value of the field is `AHBCMDERRCODE_3`"]
    #[inline(always)]
    pub fn is_ahbcmderrcode_3(&self) -> bool {
        *self == AHBCMDERRCODE_A::AHBCMDERRCODE_3
    }
    #[doc = "Checks if the value of the field is `AHBCMDERRCODE_4`"]
    #[inline(always)]
    pub fn is_ahbcmderrcode_4(&self) -> bool {
        *self == AHBCMDERRCODE_A::AHBCMDERRCODE_4
    }
    #[doc = "Checks if the value of the field is `AHBCMDERRCODE_5`"]
    #[inline(always)]
    pub fn is_ahbcmderrcode_5(&self) -> bool {
        *self == AHBCMDERRCODE_A::AHBCMDERRCODE_5
    }
    #[doc = "Checks if the value of the field is `AHBCMDERRCODE_14`"]
    #[inline(always)]
    pub fn is_ahbcmderrcode_14(&self) -> bool {
        *self == AHBCMDERRCODE_A::AHBCMDERRCODE_14
    }
}
#[doc = "Reader of field `IPCMDERRID`"]
pub type IPCMDERRID_R = crate::R<u8, u8>;
#[doc = "Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\]
is write-1-clear(w1c).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IPCMDERRCODE_A {
    #[doc = "0: No error."]
    IPCMDERRCODE_0 = 0,
    #[doc = "2: IP command with JMP_ON_CS instruction used in the sequence."]
    IPCMDERRCODE_2 = 2,
    #[doc = "3: There is unknown instruction opcode in the sequence."]
    IPCMDERRCODE_3 = 3,
    #[doc = "4: Instruction DUMMY_SDR/DUMMY_RWDS_SDR used in DDR sequence."]
    IPCMDERRCODE_4 = 4,
    #[doc = "5: Instruction DUMMY_DDR/DUMMY_RWDS_DDR used in SDR sequence."]
    IPCMDERRCODE_5 = 5,
    #[doc = "6: Flash access start address exceed the whole flash address range (A1/A2/B1/B2)."]
    IPCMDERRCODE_6 = 6,
    #[doc = "14: Sequence execution timeout."]
    IPCMDERRCODE_14 = 14,
    #[doc = "15: Flash boundary crossed."]
    IPCMDERRCODE_15 = 15,
}
impl From<IPCMDERRCODE_A> for u8 {
    #[inline(always)]
    fn from(variant: IPCMDERRCODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IPCMDERRCODE`"]
pub type IPCMDERRCODE_R = crate::R<u8, IPCMDERRCODE_A>;
impl IPCMDERRCODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IPCMDERRCODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IPCMDERRCODE_A::IPCMDERRCODE_0),
            2 => Val(IPCMDERRCODE_A::IPCMDERRCODE_2),
            3 => Val(IPCMDERRCODE_A::IPCMDERRCODE_3),
            4 => Val(IPCMDERRCODE_A::IPCMDERRCODE_4),
            5 => Val(IPCMDERRCODE_A::IPCMDERRCODE_5),
            6 => Val(IPCMDERRCODE_A::IPCMDERRCODE_6),
            14 => Val(IPCMDERRCODE_A::IPCMDERRCODE_14),
            15 => Val(IPCMDERRCODE_A::IPCMDERRCODE_15),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_0`"]
    #[inline(always)]
    pub fn is_ipcmderrcode_0(&self) -> bool {
        *self == IPCMDERRCODE_A::IPCMDERRCODE_0
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_2`"]
    #[inline(always)]
    pub fn is_ipcmderrcode_2(&self) -> bool {
        *self == IPCMDERRCODE_A::IPCMDERRCODE_2
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_3`"]
    #[inline(always)]
    pub fn is_ipcmderrcode_3(&self) -> bool {
        *self == IPCMDERRCODE_A::IPCMDERRCODE_3
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_4`"]
    #[inline(always)]
    pub fn is_ipcmderrcode_4(&self) -> bool {
        *self == IPCMDERRCODE_A::IPCMDERRCODE_4
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_5`"]
    #[inline(always)]
    pub fn is_ipcmderrcode_5(&self) -> bool {
        *self == IPCMDERRCODE_A::IPCMDERRCODE_5
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_6`"]
    #[inline(always)]
    pub fn is_ipcmderrcode_6(&self) -> bool {
        *self == IPCMDERRCODE_A::IPCMDERRCODE_6
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_14`"]
    #[inline(always)]
    pub fn is_ipcmderrcode_14(&self) -> bool {
        *self == IPCMDERRCODE_A::IPCMDERRCODE_14
    }
    #[doc = "Checks if the value of the field is `IPCMDERRCODE_15`"]
    #[inline(always)]
    pub fn is_ipcmderrcode_15(&self) -> bool {
        *self == IPCMDERRCODE_A::IPCMDERRCODE_15
    }
}
impl R {
    #[doc = "Bits 0:3 - Indicates the sequence index when an AHB command error is detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn ahbcmderrid(&self) -> AHBCMDERRID_R {
        AHBCMDERRID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\]
is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn ahbcmderrcode(&self) -> AHBCMDERRCODE_R {
        AHBCMDERRCODE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the sequence Index when IP command error detected. This field will be cleared when INTR\\[IPCMDERR\\]
is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn ipcmderrid(&self) -> IPCMDERRID_R {
        IPCMDERRID_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\]
is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn ipcmderrcode(&self) -> IPCMDERRCODE_R {
        IPCMDERRCODE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
