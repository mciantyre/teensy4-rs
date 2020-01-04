#[doc = "Reader of register CCSIDR"]
pub type R = crate::R<u32, super::CCSIDR>;
#[doc = "(Log2(Number of words in cache line)) - 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINESIZE_A {
    #[doc = "0: The line length of 4 words."]
    LINESIZE_0 = 0,
    #[doc = "1: The line length of 8 words."]
    LINESIZE_1 = 1,
    #[doc = "2: The line length of 16 words."]
    LINESIZE_2 = 2,
    #[doc = "3: The line length of 32 words."]
    LINESIZE_3 = 3,
    #[doc = "4: The line length of 64 words."]
    LINESIZE_4 = 4,
    #[doc = "5: The line length of 128 words."]
    LINESIZE_5 = 5,
    #[doc = "6: The line length of 256 words."]
    LINESIZE_6 = 6,
    #[doc = "7: The line length of 512 words."]
    LINESIZE_7 = 7,
}
impl From<LINESIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: LINESIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LINESIZE`"]
pub type LINESIZE_R = crate::R<u8, LINESIZE_A>;
impl LINESIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINESIZE_A {
        match self.bits {
            0 => LINESIZE_A::LINESIZE_0,
            1 => LINESIZE_A::LINESIZE_1,
            2 => LINESIZE_A::LINESIZE_2,
            3 => LINESIZE_A::LINESIZE_3,
            4 => LINESIZE_A::LINESIZE_4,
            5 => LINESIZE_A::LINESIZE_5,
            6 => LINESIZE_A::LINESIZE_6,
            7 => LINESIZE_A::LINESIZE_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LINESIZE_0`"]
    #[inline(always)]
    pub fn is_linesize_0(&self) -> bool {
        *self == LINESIZE_A::LINESIZE_0
    }
    #[doc = "Checks if the value of the field is `LINESIZE_1`"]
    #[inline(always)]
    pub fn is_linesize_1(&self) -> bool {
        *self == LINESIZE_A::LINESIZE_1
    }
    #[doc = "Checks if the value of the field is `LINESIZE_2`"]
    #[inline(always)]
    pub fn is_linesize_2(&self) -> bool {
        *self == LINESIZE_A::LINESIZE_2
    }
    #[doc = "Checks if the value of the field is `LINESIZE_3`"]
    #[inline(always)]
    pub fn is_linesize_3(&self) -> bool {
        *self == LINESIZE_A::LINESIZE_3
    }
    #[doc = "Checks if the value of the field is `LINESIZE_4`"]
    #[inline(always)]
    pub fn is_linesize_4(&self) -> bool {
        *self == LINESIZE_A::LINESIZE_4
    }
    #[doc = "Checks if the value of the field is `LINESIZE_5`"]
    #[inline(always)]
    pub fn is_linesize_5(&self) -> bool {
        *self == LINESIZE_A::LINESIZE_5
    }
    #[doc = "Checks if the value of the field is `LINESIZE_6`"]
    #[inline(always)]
    pub fn is_linesize_6(&self) -> bool {
        *self == LINESIZE_A::LINESIZE_6
    }
    #[doc = "Checks if the value of the field is `LINESIZE_7`"]
    #[inline(always)]
    pub fn is_linesize_7(&self) -> bool {
        *self == LINESIZE_A::LINESIZE_7
    }
}
#[doc = "Reader of field `ASSOCIATIVITY`"]
pub type ASSOCIATIVITY_R = crate::R<u16, u16>;
#[doc = "Reader of field `NUMSETS`"]
pub type NUMSETS_R = crate::R<u16, u16>;
#[doc = "Indicates whether the cache level supports write-allocation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WA_A {
    #[doc = "0: Feature not supported"]
    WA_0 = 0,
    #[doc = "1: Feature supported"]
    WA_1 = 1,
}
impl From<WA_A> for bool {
    #[inline(always)]
    fn from(variant: WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WA`"]
pub type WA_R = crate::R<bool, WA_A>;
impl WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WA_A {
        match self.bits {
            false => WA_A::WA_0,
            true => WA_A::WA_1,
        }
    }
    #[doc = "Checks if the value of the field is `WA_0`"]
    #[inline(always)]
    pub fn is_wa_0(&self) -> bool {
        *self == WA_A::WA_0
    }
    #[doc = "Checks if the value of the field is `WA_1`"]
    #[inline(always)]
    pub fn is_wa_1(&self) -> bool {
        *self == WA_A::WA_1
    }
}
#[doc = "Indicates whether the cache level supports read-allocation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RA_A {
    #[doc = "0: Feature not supported"]
    RA_0 = 0,
    #[doc = "1: Feature supported"]
    RA_1 = 1,
}
impl From<RA_A> for bool {
    #[inline(always)]
    fn from(variant: RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RA`"]
pub type RA_R = crate::R<bool, RA_A>;
impl RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RA_A {
        match self.bits {
            false => RA_A::RA_0,
            true => RA_A::RA_1,
        }
    }
    #[doc = "Checks if the value of the field is `RA_0`"]
    #[inline(always)]
    pub fn is_ra_0(&self) -> bool {
        *self == RA_A::RA_0
    }
    #[doc = "Checks if the value of the field is `RA_1`"]
    #[inline(always)]
    pub fn is_ra_1(&self) -> bool {
        *self == RA_A::RA_1
    }
}
#[doc = "Indicates whether the cache level supports write-back\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WB_A {
    #[doc = "0: Feature not supported"]
    WB_0 = 0,
    #[doc = "1: Feature supported"]
    WB_1 = 1,
}
impl From<WB_A> for bool {
    #[inline(always)]
    fn from(variant: WB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WB`"]
pub type WB_R = crate::R<bool, WB_A>;
impl WB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WB_A {
        match self.bits {
            false => WB_A::WB_0,
            true => WB_A::WB_1,
        }
    }
    #[doc = "Checks if the value of the field is `WB_0`"]
    #[inline(always)]
    pub fn is_wb_0(&self) -> bool {
        *self == WB_A::WB_0
    }
    #[doc = "Checks if the value of the field is `WB_1`"]
    #[inline(always)]
    pub fn is_wb_1(&self) -> bool {
        *self == WB_A::WB_1
    }
}
#[doc = "Indicates whether the cache level supports write-through\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WT_A {
    #[doc = "0: Feature not supported"]
    WT_0 = 0,
    #[doc = "1: Feature supported"]
    WT_1 = 1,
}
impl From<WT_A> for bool {
    #[inline(always)]
    fn from(variant: WT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WT`"]
pub type WT_R = crate::R<bool, WT_A>;
impl WT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WT_A {
        match self.bits {
            false => WT_A::WT_0,
            true => WT_A::WT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WT_0`"]
    #[inline(always)]
    pub fn is_wt_0(&self) -> bool {
        *self == WT_A::WT_0
    }
    #[doc = "Checks if the value of the field is `WT_1`"]
    #[inline(always)]
    pub fn is_wt_1(&self) -> bool {
        *self == WT_A::WT_1
    }
}
impl R {
    #[doc = "Bits 0:2 - (Log2(Number of words in cache line)) - 2."]
    #[inline(always)]
    pub fn linesize(&self) -> LINESIZE_R {
        LINESIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:12 - (Associativity of cache) - 1, therefore a value of 0 indicates an associativity of 1. The associativity does not have to be a power of 2."]
    #[inline(always)]
    pub fn associativity(&self) -> ASSOCIATIVITY_R {
        ASSOCIATIVITY_R::new(((self.bits >> 3) & 0x03ff) as u16)
    }
    #[doc = "Bits 13:27 - (Number of sets in cache) - 1, therefore a value of 0 indicates 1 set in the cache. The number of sets does not have to be a power of 2."]
    #[inline(always)]
    pub fn numsets(&self) -> NUMSETS_R {
        NUMSETS_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    #[doc = "Bit 28 - Indicates whether the cache level supports write-allocation"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Indicates whether the cache level supports read-allocation"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Indicates whether the cache level supports write-back"]
    #[inline(always)]
    pub fn wb(&self) -> WB_R {
        WB_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Indicates whether the cache level supports write-through"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
