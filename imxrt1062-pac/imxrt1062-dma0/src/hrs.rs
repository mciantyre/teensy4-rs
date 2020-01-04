#[doc = "Reader of register HRS"]
pub type R = crate::R<u32, super::HRS>;
#[doc = "Hardware Request Status Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS0_A {
    #[doc = "0: A hardware service request for channel 0 is not present"]
    HRS0_0 = 0,
    #[doc = "1: A hardware service request for channel 0 is present"]
    HRS0_1 = 1,
}
impl From<HRS0_A> for bool {
    #[inline(always)]
    fn from(variant: HRS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS0`"]
pub type HRS0_R = crate::R<bool, HRS0_A>;
impl HRS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS0_A {
        match self.bits {
            false => HRS0_A::HRS0_0,
            true => HRS0_A::HRS0_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS0_0`"]
    #[inline(always)]
    pub fn is_hrs0_0(&self) -> bool {
        *self == HRS0_A::HRS0_0
    }
    #[doc = "Checks if the value of the field is `HRS0_1`"]
    #[inline(always)]
    pub fn is_hrs0_1(&self) -> bool {
        *self == HRS0_A::HRS0_1
    }
}
#[doc = "Hardware Request Status Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS1_A {
    #[doc = "0: A hardware service request for channel 1 is not present"]
    HRS1_0 = 0,
    #[doc = "1: A hardware service request for channel 1 is present"]
    HRS1_1 = 1,
}
impl From<HRS1_A> for bool {
    #[inline(always)]
    fn from(variant: HRS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS1`"]
pub type HRS1_R = crate::R<bool, HRS1_A>;
impl HRS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS1_A {
        match self.bits {
            false => HRS1_A::HRS1_0,
            true => HRS1_A::HRS1_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS1_0`"]
    #[inline(always)]
    pub fn is_hrs1_0(&self) -> bool {
        *self == HRS1_A::HRS1_0
    }
    #[doc = "Checks if the value of the field is `HRS1_1`"]
    #[inline(always)]
    pub fn is_hrs1_1(&self) -> bool {
        *self == HRS1_A::HRS1_1
    }
}
#[doc = "Hardware Request Status Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS2_A {
    #[doc = "0: A hardware service request for channel 2 is not present"]
    HRS2_0 = 0,
    #[doc = "1: A hardware service request for channel 2 is present"]
    HRS2_1 = 1,
}
impl From<HRS2_A> for bool {
    #[inline(always)]
    fn from(variant: HRS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS2`"]
pub type HRS2_R = crate::R<bool, HRS2_A>;
impl HRS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS2_A {
        match self.bits {
            false => HRS2_A::HRS2_0,
            true => HRS2_A::HRS2_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS2_0`"]
    #[inline(always)]
    pub fn is_hrs2_0(&self) -> bool {
        *self == HRS2_A::HRS2_0
    }
    #[doc = "Checks if the value of the field is `HRS2_1`"]
    #[inline(always)]
    pub fn is_hrs2_1(&self) -> bool {
        *self == HRS2_A::HRS2_1
    }
}
#[doc = "Hardware Request Status Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS3_A {
    #[doc = "0: A hardware service request for channel 3 is not present"]
    HRS3_0 = 0,
    #[doc = "1: A hardware service request for channel 3 is present"]
    HRS3_1 = 1,
}
impl From<HRS3_A> for bool {
    #[inline(always)]
    fn from(variant: HRS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS3`"]
pub type HRS3_R = crate::R<bool, HRS3_A>;
impl HRS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS3_A {
        match self.bits {
            false => HRS3_A::HRS3_0,
            true => HRS3_A::HRS3_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS3_0`"]
    #[inline(always)]
    pub fn is_hrs3_0(&self) -> bool {
        *self == HRS3_A::HRS3_0
    }
    #[doc = "Checks if the value of the field is `HRS3_1`"]
    #[inline(always)]
    pub fn is_hrs3_1(&self) -> bool {
        *self == HRS3_A::HRS3_1
    }
}
#[doc = "Hardware Request Status Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS4_A {
    #[doc = "0: A hardware service request for channel 4 is not present"]
    HRS4_0 = 0,
    #[doc = "1: A hardware service request for channel 4 is present"]
    HRS4_1 = 1,
}
impl From<HRS4_A> for bool {
    #[inline(always)]
    fn from(variant: HRS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS4`"]
pub type HRS4_R = crate::R<bool, HRS4_A>;
impl HRS4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS4_A {
        match self.bits {
            false => HRS4_A::HRS4_0,
            true => HRS4_A::HRS4_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS4_0`"]
    #[inline(always)]
    pub fn is_hrs4_0(&self) -> bool {
        *self == HRS4_A::HRS4_0
    }
    #[doc = "Checks if the value of the field is `HRS4_1`"]
    #[inline(always)]
    pub fn is_hrs4_1(&self) -> bool {
        *self == HRS4_A::HRS4_1
    }
}
#[doc = "Hardware Request Status Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS5_A {
    #[doc = "0: A hardware service request for channel 5 is not present"]
    HRS5_0 = 0,
    #[doc = "1: A hardware service request for channel 5 is present"]
    HRS5_1 = 1,
}
impl From<HRS5_A> for bool {
    #[inline(always)]
    fn from(variant: HRS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS5`"]
pub type HRS5_R = crate::R<bool, HRS5_A>;
impl HRS5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS5_A {
        match self.bits {
            false => HRS5_A::HRS5_0,
            true => HRS5_A::HRS5_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS5_0`"]
    #[inline(always)]
    pub fn is_hrs5_0(&self) -> bool {
        *self == HRS5_A::HRS5_0
    }
    #[doc = "Checks if the value of the field is `HRS5_1`"]
    #[inline(always)]
    pub fn is_hrs5_1(&self) -> bool {
        *self == HRS5_A::HRS5_1
    }
}
#[doc = "Hardware Request Status Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS6_A {
    #[doc = "0: A hardware service request for channel 6 is not present"]
    HRS6_0 = 0,
    #[doc = "1: A hardware service request for channel 6 is present"]
    HRS6_1 = 1,
}
impl From<HRS6_A> for bool {
    #[inline(always)]
    fn from(variant: HRS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS6`"]
pub type HRS6_R = crate::R<bool, HRS6_A>;
impl HRS6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS6_A {
        match self.bits {
            false => HRS6_A::HRS6_0,
            true => HRS6_A::HRS6_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS6_0`"]
    #[inline(always)]
    pub fn is_hrs6_0(&self) -> bool {
        *self == HRS6_A::HRS6_0
    }
    #[doc = "Checks if the value of the field is `HRS6_1`"]
    #[inline(always)]
    pub fn is_hrs6_1(&self) -> bool {
        *self == HRS6_A::HRS6_1
    }
}
#[doc = "Hardware Request Status Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS7_A {
    #[doc = "0: A hardware service request for channel 7 is not present"]
    HRS7_0 = 0,
    #[doc = "1: A hardware service request for channel 7 is present"]
    HRS7_1 = 1,
}
impl From<HRS7_A> for bool {
    #[inline(always)]
    fn from(variant: HRS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS7`"]
pub type HRS7_R = crate::R<bool, HRS7_A>;
impl HRS7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS7_A {
        match self.bits {
            false => HRS7_A::HRS7_0,
            true => HRS7_A::HRS7_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS7_0`"]
    #[inline(always)]
    pub fn is_hrs7_0(&self) -> bool {
        *self == HRS7_A::HRS7_0
    }
    #[doc = "Checks if the value of the field is `HRS7_1`"]
    #[inline(always)]
    pub fn is_hrs7_1(&self) -> bool {
        *self == HRS7_A::HRS7_1
    }
}
#[doc = "Hardware Request Status Channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS8_A {
    #[doc = "0: A hardware service request for channel 8 is not present"]
    HRS8_0 = 0,
    #[doc = "1: A hardware service request for channel 8 is present"]
    HRS8_1 = 1,
}
impl From<HRS8_A> for bool {
    #[inline(always)]
    fn from(variant: HRS8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS8`"]
pub type HRS8_R = crate::R<bool, HRS8_A>;
impl HRS8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS8_A {
        match self.bits {
            false => HRS8_A::HRS8_0,
            true => HRS8_A::HRS8_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS8_0`"]
    #[inline(always)]
    pub fn is_hrs8_0(&self) -> bool {
        *self == HRS8_A::HRS8_0
    }
    #[doc = "Checks if the value of the field is `HRS8_1`"]
    #[inline(always)]
    pub fn is_hrs8_1(&self) -> bool {
        *self == HRS8_A::HRS8_1
    }
}
#[doc = "Hardware Request Status Channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS9_A {
    #[doc = "0: A hardware service request for channel 9 is not present"]
    HRS9_0 = 0,
    #[doc = "1: A hardware service request for channel 9 is present"]
    HRS9_1 = 1,
}
impl From<HRS9_A> for bool {
    #[inline(always)]
    fn from(variant: HRS9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS9`"]
pub type HRS9_R = crate::R<bool, HRS9_A>;
impl HRS9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS9_A {
        match self.bits {
            false => HRS9_A::HRS9_0,
            true => HRS9_A::HRS9_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS9_0`"]
    #[inline(always)]
    pub fn is_hrs9_0(&self) -> bool {
        *self == HRS9_A::HRS9_0
    }
    #[doc = "Checks if the value of the field is `HRS9_1`"]
    #[inline(always)]
    pub fn is_hrs9_1(&self) -> bool {
        *self == HRS9_A::HRS9_1
    }
}
#[doc = "Hardware Request Status Channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS10_A {
    #[doc = "0: A hardware service request for channel 10 is not present"]
    HRS10_0 = 0,
    #[doc = "1: A hardware service request for channel 10 is present"]
    HRS10_1 = 1,
}
impl From<HRS10_A> for bool {
    #[inline(always)]
    fn from(variant: HRS10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS10`"]
pub type HRS10_R = crate::R<bool, HRS10_A>;
impl HRS10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS10_A {
        match self.bits {
            false => HRS10_A::HRS10_0,
            true => HRS10_A::HRS10_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS10_0`"]
    #[inline(always)]
    pub fn is_hrs10_0(&self) -> bool {
        *self == HRS10_A::HRS10_0
    }
    #[doc = "Checks if the value of the field is `HRS10_1`"]
    #[inline(always)]
    pub fn is_hrs10_1(&self) -> bool {
        *self == HRS10_A::HRS10_1
    }
}
#[doc = "Hardware Request Status Channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS11_A {
    #[doc = "0: A hardware service request for channel 11 is not present"]
    HRS11_0 = 0,
    #[doc = "1: A hardware service request for channel 11 is present"]
    HRS11_1 = 1,
}
impl From<HRS11_A> for bool {
    #[inline(always)]
    fn from(variant: HRS11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS11`"]
pub type HRS11_R = crate::R<bool, HRS11_A>;
impl HRS11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS11_A {
        match self.bits {
            false => HRS11_A::HRS11_0,
            true => HRS11_A::HRS11_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS11_0`"]
    #[inline(always)]
    pub fn is_hrs11_0(&self) -> bool {
        *self == HRS11_A::HRS11_0
    }
    #[doc = "Checks if the value of the field is `HRS11_1`"]
    #[inline(always)]
    pub fn is_hrs11_1(&self) -> bool {
        *self == HRS11_A::HRS11_1
    }
}
#[doc = "Hardware Request Status Channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS12_A {
    #[doc = "0: A hardware service request for channel 12 is not present"]
    HRS12_0 = 0,
    #[doc = "1: A hardware service request for channel 12 is present"]
    HRS12_1 = 1,
}
impl From<HRS12_A> for bool {
    #[inline(always)]
    fn from(variant: HRS12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS12`"]
pub type HRS12_R = crate::R<bool, HRS12_A>;
impl HRS12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS12_A {
        match self.bits {
            false => HRS12_A::HRS12_0,
            true => HRS12_A::HRS12_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS12_0`"]
    #[inline(always)]
    pub fn is_hrs12_0(&self) -> bool {
        *self == HRS12_A::HRS12_0
    }
    #[doc = "Checks if the value of the field is `HRS12_1`"]
    #[inline(always)]
    pub fn is_hrs12_1(&self) -> bool {
        *self == HRS12_A::HRS12_1
    }
}
#[doc = "Hardware Request Status Channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS13_A {
    #[doc = "0: A hardware service request for channel 13 is not present"]
    HRS13_0 = 0,
    #[doc = "1: A hardware service request for channel 13 is present"]
    HRS13_1 = 1,
}
impl From<HRS13_A> for bool {
    #[inline(always)]
    fn from(variant: HRS13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS13`"]
pub type HRS13_R = crate::R<bool, HRS13_A>;
impl HRS13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS13_A {
        match self.bits {
            false => HRS13_A::HRS13_0,
            true => HRS13_A::HRS13_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS13_0`"]
    #[inline(always)]
    pub fn is_hrs13_0(&self) -> bool {
        *self == HRS13_A::HRS13_0
    }
    #[doc = "Checks if the value of the field is `HRS13_1`"]
    #[inline(always)]
    pub fn is_hrs13_1(&self) -> bool {
        *self == HRS13_A::HRS13_1
    }
}
#[doc = "Hardware Request Status Channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS14_A {
    #[doc = "0: A hardware service request for channel 14 is not present"]
    HRS14_0 = 0,
    #[doc = "1: A hardware service request for channel 14 is present"]
    HRS14_1 = 1,
}
impl From<HRS14_A> for bool {
    #[inline(always)]
    fn from(variant: HRS14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS14`"]
pub type HRS14_R = crate::R<bool, HRS14_A>;
impl HRS14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS14_A {
        match self.bits {
            false => HRS14_A::HRS14_0,
            true => HRS14_A::HRS14_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS14_0`"]
    #[inline(always)]
    pub fn is_hrs14_0(&self) -> bool {
        *self == HRS14_A::HRS14_0
    }
    #[doc = "Checks if the value of the field is `HRS14_1`"]
    #[inline(always)]
    pub fn is_hrs14_1(&self) -> bool {
        *self == HRS14_A::HRS14_1
    }
}
#[doc = "Hardware Request Status Channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS15_A {
    #[doc = "0: A hardware service request for channel 15 is not present"]
    HRS15_0 = 0,
    #[doc = "1: A hardware service request for channel 15 is present"]
    HRS15_1 = 1,
}
impl From<HRS15_A> for bool {
    #[inline(always)]
    fn from(variant: HRS15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS15`"]
pub type HRS15_R = crate::R<bool, HRS15_A>;
impl HRS15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS15_A {
        match self.bits {
            false => HRS15_A::HRS15_0,
            true => HRS15_A::HRS15_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS15_0`"]
    #[inline(always)]
    pub fn is_hrs15_0(&self) -> bool {
        *self == HRS15_A::HRS15_0
    }
    #[doc = "Checks if the value of the field is `HRS15_1`"]
    #[inline(always)]
    pub fn is_hrs15_1(&self) -> bool {
        *self == HRS15_A::HRS15_1
    }
}
#[doc = "Hardware Request Status Channel 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS16_A {
    #[doc = "0: A hardware service request for channel 16 is not present"]
    HRS16_0 = 0,
    #[doc = "1: A hardware service request for channel 16 is present"]
    HRS16_1 = 1,
}
impl From<HRS16_A> for bool {
    #[inline(always)]
    fn from(variant: HRS16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS16`"]
pub type HRS16_R = crate::R<bool, HRS16_A>;
impl HRS16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS16_A {
        match self.bits {
            false => HRS16_A::HRS16_0,
            true => HRS16_A::HRS16_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS16_0`"]
    #[inline(always)]
    pub fn is_hrs16_0(&self) -> bool {
        *self == HRS16_A::HRS16_0
    }
    #[doc = "Checks if the value of the field is `HRS16_1`"]
    #[inline(always)]
    pub fn is_hrs16_1(&self) -> bool {
        *self == HRS16_A::HRS16_1
    }
}
#[doc = "Hardware Request Status Channel 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS17_A {
    #[doc = "0: A hardware service request for channel 17 is not present"]
    HRS17_0 = 0,
    #[doc = "1: A hardware service request for channel 17 is present"]
    HRS17_1 = 1,
}
impl From<HRS17_A> for bool {
    #[inline(always)]
    fn from(variant: HRS17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS17`"]
pub type HRS17_R = crate::R<bool, HRS17_A>;
impl HRS17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS17_A {
        match self.bits {
            false => HRS17_A::HRS17_0,
            true => HRS17_A::HRS17_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS17_0`"]
    #[inline(always)]
    pub fn is_hrs17_0(&self) -> bool {
        *self == HRS17_A::HRS17_0
    }
    #[doc = "Checks if the value of the field is `HRS17_1`"]
    #[inline(always)]
    pub fn is_hrs17_1(&self) -> bool {
        *self == HRS17_A::HRS17_1
    }
}
#[doc = "Hardware Request Status Channel 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS18_A {
    #[doc = "0: A hardware service request for channel 18 is not present"]
    HRS18_0 = 0,
    #[doc = "1: A hardware service request for channel 18 is present"]
    HRS18_1 = 1,
}
impl From<HRS18_A> for bool {
    #[inline(always)]
    fn from(variant: HRS18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS18`"]
pub type HRS18_R = crate::R<bool, HRS18_A>;
impl HRS18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS18_A {
        match self.bits {
            false => HRS18_A::HRS18_0,
            true => HRS18_A::HRS18_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS18_0`"]
    #[inline(always)]
    pub fn is_hrs18_0(&self) -> bool {
        *self == HRS18_A::HRS18_0
    }
    #[doc = "Checks if the value of the field is `HRS18_1`"]
    #[inline(always)]
    pub fn is_hrs18_1(&self) -> bool {
        *self == HRS18_A::HRS18_1
    }
}
#[doc = "Hardware Request Status Channel 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS19_A {
    #[doc = "0: A hardware service request for channel 19 is not present"]
    HRS19_0 = 0,
    #[doc = "1: A hardware service request for channel 19 is present"]
    HRS19_1 = 1,
}
impl From<HRS19_A> for bool {
    #[inline(always)]
    fn from(variant: HRS19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS19`"]
pub type HRS19_R = crate::R<bool, HRS19_A>;
impl HRS19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS19_A {
        match self.bits {
            false => HRS19_A::HRS19_0,
            true => HRS19_A::HRS19_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS19_0`"]
    #[inline(always)]
    pub fn is_hrs19_0(&self) -> bool {
        *self == HRS19_A::HRS19_0
    }
    #[doc = "Checks if the value of the field is `HRS19_1`"]
    #[inline(always)]
    pub fn is_hrs19_1(&self) -> bool {
        *self == HRS19_A::HRS19_1
    }
}
#[doc = "Hardware Request Status Channel 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS20_A {
    #[doc = "0: A hardware service request for channel 20 is not present"]
    HRS20_0 = 0,
    #[doc = "1: A hardware service request for channel 20 is present"]
    HRS20_1 = 1,
}
impl From<HRS20_A> for bool {
    #[inline(always)]
    fn from(variant: HRS20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS20`"]
pub type HRS20_R = crate::R<bool, HRS20_A>;
impl HRS20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS20_A {
        match self.bits {
            false => HRS20_A::HRS20_0,
            true => HRS20_A::HRS20_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS20_0`"]
    #[inline(always)]
    pub fn is_hrs20_0(&self) -> bool {
        *self == HRS20_A::HRS20_0
    }
    #[doc = "Checks if the value of the field is `HRS20_1`"]
    #[inline(always)]
    pub fn is_hrs20_1(&self) -> bool {
        *self == HRS20_A::HRS20_1
    }
}
#[doc = "Hardware Request Status Channel 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS21_A {
    #[doc = "0: A hardware service request for channel 21 is not present"]
    HRS21_0 = 0,
    #[doc = "1: A hardware service request for channel 21 is present"]
    HRS21_1 = 1,
}
impl From<HRS21_A> for bool {
    #[inline(always)]
    fn from(variant: HRS21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS21`"]
pub type HRS21_R = crate::R<bool, HRS21_A>;
impl HRS21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS21_A {
        match self.bits {
            false => HRS21_A::HRS21_0,
            true => HRS21_A::HRS21_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS21_0`"]
    #[inline(always)]
    pub fn is_hrs21_0(&self) -> bool {
        *self == HRS21_A::HRS21_0
    }
    #[doc = "Checks if the value of the field is `HRS21_1`"]
    #[inline(always)]
    pub fn is_hrs21_1(&self) -> bool {
        *self == HRS21_A::HRS21_1
    }
}
#[doc = "Hardware Request Status Channel 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS22_A {
    #[doc = "0: A hardware service request for channel 22 is not present"]
    HRS22_0 = 0,
    #[doc = "1: A hardware service request for channel 22 is present"]
    HRS22_1 = 1,
}
impl From<HRS22_A> for bool {
    #[inline(always)]
    fn from(variant: HRS22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS22`"]
pub type HRS22_R = crate::R<bool, HRS22_A>;
impl HRS22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS22_A {
        match self.bits {
            false => HRS22_A::HRS22_0,
            true => HRS22_A::HRS22_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS22_0`"]
    #[inline(always)]
    pub fn is_hrs22_0(&self) -> bool {
        *self == HRS22_A::HRS22_0
    }
    #[doc = "Checks if the value of the field is `HRS22_1`"]
    #[inline(always)]
    pub fn is_hrs22_1(&self) -> bool {
        *self == HRS22_A::HRS22_1
    }
}
#[doc = "Hardware Request Status Channel 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS23_A {
    #[doc = "0: A hardware service request for channel 23 is not present"]
    HRS23_0 = 0,
    #[doc = "1: A hardware service request for channel 23 is present"]
    HRS23_1 = 1,
}
impl From<HRS23_A> for bool {
    #[inline(always)]
    fn from(variant: HRS23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS23`"]
pub type HRS23_R = crate::R<bool, HRS23_A>;
impl HRS23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS23_A {
        match self.bits {
            false => HRS23_A::HRS23_0,
            true => HRS23_A::HRS23_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS23_0`"]
    #[inline(always)]
    pub fn is_hrs23_0(&self) -> bool {
        *self == HRS23_A::HRS23_0
    }
    #[doc = "Checks if the value of the field is `HRS23_1`"]
    #[inline(always)]
    pub fn is_hrs23_1(&self) -> bool {
        *self == HRS23_A::HRS23_1
    }
}
#[doc = "Hardware Request Status Channel 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS24_A {
    #[doc = "0: A hardware service request for channel 24 is not present"]
    HRS24_0 = 0,
    #[doc = "1: A hardware service request for channel 24 is present"]
    HRS24_1 = 1,
}
impl From<HRS24_A> for bool {
    #[inline(always)]
    fn from(variant: HRS24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS24`"]
pub type HRS24_R = crate::R<bool, HRS24_A>;
impl HRS24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS24_A {
        match self.bits {
            false => HRS24_A::HRS24_0,
            true => HRS24_A::HRS24_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS24_0`"]
    #[inline(always)]
    pub fn is_hrs24_0(&self) -> bool {
        *self == HRS24_A::HRS24_0
    }
    #[doc = "Checks if the value of the field is `HRS24_1`"]
    #[inline(always)]
    pub fn is_hrs24_1(&self) -> bool {
        *self == HRS24_A::HRS24_1
    }
}
#[doc = "Hardware Request Status Channel 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS25_A {
    #[doc = "0: A hardware service request for channel 25 is not present"]
    HRS25_0 = 0,
    #[doc = "1: A hardware service request for channel 25 is present"]
    HRS25_1 = 1,
}
impl From<HRS25_A> for bool {
    #[inline(always)]
    fn from(variant: HRS25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS25`"]
pub type HRS25_R = crate::R<bool, HRS25_A>;
impl HRS25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS25_A {
        match self.bits {
            false => HRS25_A::HRS25_0,
            true => HRS25_A::HRS25_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS25_0`"]
    #[inline(always)]
    pub fn is_hrs25_0(&self) -> bool {
        *self == HRS25_A::HRS25_0
    }
    #[doc = "Checks if the value of the field is `HRS25_1`"]
    #[inline(always)]
    pub fn is_hrs25_1(&self) -> bool {
        *self == HRS25_A::HRS25_1
    }
}
#[doc = "Hardware Request Status Channel 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS26_A {
    #[doc = "0: A hardware service request for channel 26 is not present"]
    HRS26_0 = 0,
    #[doc = "1: A hardware service request for channel 26 is present"]
    HRS26_1 = 1,
}
impl From<HRS26_A> for bool {
    #[inline(always)]
    fn from(variant: HRS26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS26`"]
pub type HRS26_R = crate::R<bool, HRS26_A>;
impl HRS26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS26_A {
        match self.bits {
            false => HRS26_A::HRS26_0,
            true => HRS26_A::HRS26_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS26_0`"]
    #[inline(always)]
    pub fn is_hrs26_0(&self) -> bool {
        *self == HRS26_A::HRS26_0
    }
    #[doc = "Checks if the value of the field is `HRS26_1`"]
    #[inline(always)]
    pub fn is_hrs26_1(&self) -> bool {
        *self == HRS26_A::HRS26_1
    }
}
#[doc = "Hardware Request Status Channel 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS27_A {
    #[doc = "0: A hardware service request for channel 27 is not present"]
    HRS27_0 = 0,
    #[doc = "1: A hardware service request for channel 27 is present"]
    HRS27_1 = 1,
}
impl From<HRS27_A> for bool {
    #[inline(always)]
    fn from(variant: HRS27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS27`"]
pub type HRS27_R = crate::R<bool, HRS27_A>;
impl HRS27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS27_A {
        match self.bits {
            false => HRS27_A::HRS27_0,
            true => HRS27_A::HRS27_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS27_0`"]
    #[inline(always)]
    pub fn is_hrs27_0(&self) -> bool {
        *self == HRS27_A::HRS27_0
    }
    #[doc = "Checks if the value of the field is `HRS27_1`"]
    #[inline(always)]
    pub fn is_hrs27_1(&self) -> bool {
        *self == HRS27_A::HRS27_1
    }
}
#[doc = "Hardware Request Status Channel 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS28_A {
    #[doc = "0: A hardware service request for channel 28 is not present"]
    HRS28_0 = 0,
    #[doc = "1: A hardware service request for channel 28 is present"]
    HRS28_1 = 1,
}
impl From<HRS28_A> for bool {
    #[inline(always)]
    fn from(variant: HRS28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS28`"]
pub type HRS28_R = crate::R<bool, HRS28_A>;
impl HRS28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS28_A {
        match self.bits {
            false => HRS28_A::HRS28_0,
            true => HRS28_A::HRS28_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS28_0`"]
    #[inline(always)]
    pub fn is_hrs28_0(&self) -> bool {
        *self == HRS28_A::HRS28_0
    }
    #[doc = "Checks if the value of the field is `HRS28_1`"]
    #[inline(always)]
    pub fn is_hrs28_1(&self) -> bool {
        *self == HRS28_A::HRS28_1
    }
}
#[doc = "Hardware Request Status Channel 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS29_A {
    #[doc = "0: A hardware service request for channel 29 is not preset"]
    HRS29_0 = 0,
    #[doc = "1: A hardware service request for channel 29 is present"]
    HRS29_1 = 1,
}
impl From<HRS29_A> for bool {
    #[inline(always)]
    fn from(variant: HRS29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS29`"]
pub type HRS29_R = crate::R<bool, HRS29_A>;
impl HRS29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS29_A {
        match self.bits {
            false => HRS29_A::HRS29_0,
            true => HRS29_A::HRS29_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS29_0`"]
    #[inline(always)]
    pub fn is_hrs29_0(&self) -> bool {
        *self == HRS29_A::HRS29_0
    }
    #[doc = "Checks if the value of the field is `HRS29_1`"]
    #[inline(always)]
    pub fn is_hrs29_1(&self) -> bool {
        *self == HRS29_A::HRS29_1
    }
}
#[doc = "Hardware Request Status Channel 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS30_A {
    #[doc = "0: A hardware service request for channel 30 is not present"]
    HRS30_0 = 0,
    #[doc = "1: A hardware service request for channel 30 is present"]
    HRS30_1 = 1,
}
impl From<HRS30_A> for bool {
    #[inline(always)]
    fn from(variant: HRS30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS30`"]
pub type HRS30_R = crate::R<bool, HRS30_A>;
impl HRS30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS30_A {
        match self.bits {
            false => HRS30_A::HRS30_0,
            true => HRS30_A::HRS30_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS30_0`"]
    #[inline(always)]
    pub fn is_hrs30_0(&self) -> bool {
        *self == HRS30_A::HRS30_0
    }
    #[doc = "Checks if the value of the field is `HRS30_1`"]
    #[inline(always)]
    pub fn is_hrs30_1(&self) -> bool {
        *self == HRS30_A::HRS30_1
    }
}
#[doc = "Hardware Request Status Channel 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS31_A {
    #[doc = "0: A hardware service request for channel 31 is not present"]
    HRS31_0 = 0,
    #[doc = "1: A hardware service request for channel 31 is present"]
    HRS31_1 = 1,
}
impl From<HRS31_A> for bool {
    #[inline(always)]
    fn from(variant: HRS31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS31`"]
pub type HRS31_R = crate::R<bool, HRS31_A>;
impl HRS31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS31_A {
        match self.bits {
            false => HRS31_A::HRS31_0,
            true => HRS31_A::HRS31_1,
        }
    }
    #[doc = "Checks if the value of the field is `HRS31_0`"]
    #[inline(always)]
    pub fn is_hrs31_0(&self) -> bool {
        *self == HRS31_A::HRS31_0
    }
    #[doc = "Checks if the value of the field is `HRS31_1`"]
    #[inline(always)]
    pub fn is_hrs31_1(&self) -> bool {
        *self == HRS31_A::HRS31_1
    }
}
impl R {
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline(always)]
    pub fn hrs0(&self) -> HRS0_R {
        HRS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline(always)]
    pub fn hrs1(&self) -> HRS1_R {
        HRS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline(always)]
    pub fn hrs2(&self) -> HRS2_R {
        HRS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline(always)]
    pub fn hrs3(&self) -> HRS3_R {
        HRS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hardware Request Status Channel 4"]
    #[inline(always)]
    pub fn hrs4(&self) -> HRS4_R {
        HRS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hardware Request Status Channel 5"]
    #[inline(always)]
    pub fn hrs5(&self) -> HRS5_R {
        HRS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Hardware Request Status Channel 6"]
    #[inline(always)]
    pub fn hrs6(&self) -> HRS6_R {
        HRS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Hardware Request Status Channel 7"]
    #[inline(always)]
    pub fn hrs7(&self) -> HRS7_R {
        HRS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Hardware Request Status Channel 8"]
    #[inline(always)]
    pub fn hrs8(&self) -> HRS8_R {
        HRS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Hardware Request Status Channel 9"]
    #[inline(always)]
    pub fn hrs9(&self) -> HRS9_R {
        HRS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Hardware Request Status Channel 10"]
    #[inline(always)]
    pub fn hrs10(&self) -> HRS10_R {
        HRS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Hardware Request Status Channel 11"]
    #[inline(always)]
    pub fn hrs11(&self) -> HRS11_R {
        HRS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Hardware Request Status Channel 12"]
    #[inline(always)]
    pub fn hrs12(&self) -> HRS12_R {
        HRS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Hardware Request Status Channel 13"]
    #[inline(always)]
    pub fn hrs13(&self) -> HRS13_R {
        HRS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Hardware Request Status Channel 14"]
    #[inline(always)]
    pub fn hrs14(&self) -> HRS14_R {
        HRS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Hardware Request Status Channel 15"]
    #[inline(always)]
    pub fn hrs15(&self) -> HRS15_R {
        HRS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Hardware Request Status Channel 16"]
    #[inline(always)]
    pub fn hrs16(&self) -> HRS16_R {
        HRS16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Hardware Request Status Channel 17"]
    #[inline(always)]
    pub fn hrs17(&self) -> HRS17_R {
        HRS17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Hardware Request Status Channel 18"]
    #[inline(always)]
    pub fn hrs18(&self) -> HRS18_R {
        HRS18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Hardware Request Status Channel 19"]
    #[inline(always)]
    pub fn hrs19(&self) -> HRS19_R {
        HRS19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Hardware Request Status Channel 20"]
    #[inline(always)]
    pub fn hrs20(&self) -> HRS20_R {
        HRS20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Hardware Request Status Channel 21"]
    #[inline(always)]
    pub fn hrs21(&self) -> HRS21_R {
        HRS21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Hardware Request Status Channel 22"]
    #[inline(always)]
    pub fn hrs22(&self) -> HRS22_R {
        HRS22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Hardware Request Status Channel 23"]
    #[inline(always)]
    pub fn hrs23(&self) -> HRS23_R {
        HRS23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Hardware Request Status Channel 24"]
    #[inline(always)]
    pub fn hrs24(&self) -> HRS24_R {
        HRS24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Hardware Request Status Channel 25"]
    #[inline(always)]
    pub fn hrs25(&self) -> HRS25_R {
        HRS25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Hardware Request Status Channel 26"]
    #[inline(always)]
    pub fn hrs26(&self) -> HRS26_R {
        HRS26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Hardware Request Status Channel 27"]
    #[inline(always)]
    pub fn hrs27(&self) -> HRS27_R {
        HRS27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Hardware Request Status Channel 28"]
    #[inline(always)]
    pub fn hrs28(&self) -> HRS28_R {
        HRS28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Hardware Request Status Channel 29"]
    #[inline(always)]
    pub fn hrs29(&self) -> HRS29_R {
        HRS29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Hardware Request Status Channel 30"]
    #[inline(always)]
    pub fn hrs30(&self) -> HRS30_R {
        HRS30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Hardware Request Status Channel 31"]
    #[inline(always)]
    pub fn hrs31(&self) -> HRS31_R {
        HRS31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
