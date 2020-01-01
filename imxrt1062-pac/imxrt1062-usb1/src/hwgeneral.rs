#[doc = "Reader of register HWGENERAL"]
pub type R = crate::R<u32, super::HWGENERAL>;
#[doc = "Data width of the transciever connected to the controller core. PHYW bit reset value is\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PHYW_A {
    #[doc = "0: 8 bit wide data bus Software non-programmable"]
    PHYW_0 = 0,
    #[doc = "1: 16 bit wide data bus Software non-programmable"]
    PHYW_1 = 1,
    #[doc = "2: Reset to 8 bit wide data bus Software programmable"]
    PHYW_2 = 2,
    #[doc = "3: Reset to 16 bit wide data bus Software programmable"]
    PHYW_3 = 3,
}
impl From<PHYW_A> for u8 {
    #[inline(always)]
    fn from(variant: PHYW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PHYW`"]
pub type PHYW_R = crate::R<u8, PHYW_A>;
impl PHYW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYW_A {
        match self.bits {
            0 => PHYW_A::PHYW_0,
            1 => PHYW_A::PHYW_1,
            2 => PHYW_A::PHYW_2,
            3 => PHYW_A::PHYW_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PHYW_0`"]
    #[inline(always)]
    pub fn is_phyw_0(&self) -> bool {
        *self == PHYW_A::PHYW_0
    }
    #[doc = "Checks if the value of the field is `PHYW_1`"]
    #[inline(always)]
    pub fn is_phyw_1(&self) -> bool {
        *self == PHYW_A::PHYW_1
    }
    #[doc = "Checks if the value of the field is `PHYW_2`"]
    #[inline(always)]
    pub fn is_phyw_2(&self) -> bool {
        *self == PHYW_A::PHYW_2
    }
    #[doc = "Checks if the value of the field is `PHYW_3`"]
    #[inline(always)]
    pub fn is_phyw_3(&self) -> bool {
        *self == PHYW_A::PHYW_3
    }
}
#[doc = "Transciever type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PHYM_A {
    #[doc = "0: UTMI/UMTI+"]
    PHYM_0 = 0,
    #[doc = "1: ULPI DDR"]
    PHYM_1 = 1,
    #[doc = "2: ULPI"]
    PHYM_2 = 2,
    #[doc = "3: Serial Only"]
    PHYM_3 = 3,
    #[doc = "4: Software programmable - reset to UTMI/UTMI+"]
    PHYM_4 = 4,
    #[doc = "5: Software programmable - reset to ULPI DDR"]
    PHYM_5 = 5,
    #[doc = "6: Software programmable - reset to ULPI"]
    PHYM_6 = 6,
    #[doc = "7: Software programmable - reset to Serial"]
    PHYM_7 = 7,
}
impl From<PHYM_A> for u8 {
    #[inline(always)]
    fn from(variant: PHYM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PHYM`"]
pub type PHYM_R = crate::R<u8, PHYM_A>;
impl PHYM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYM_A {
        match self.bits {
            0 => PHYM_A::PHYM_0,
            1 => PHYM_A::PHYM_1,
            2 => PHYM_A::PHYM_2,
            3 => PHYM_A::PHYM_3,
            4 => PHYM_A::PHYM_4,
            5 => PHYM_A::PHYM_5,
            6 => PHYM_A::PHYM_6,
            7 => PHYM_A::PHYM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PHYM_0`"]
    #[inline(always)]
    pub fn is_phym_0(&self) -> bool {
        *self == PHYM_A::PHYM_0
    }
    #[doc = "Checks if the value of the field is `PHYM_1`"]
    #[inline(always)]
    pub fn is_phym_1(&self) -> bool {
        *self == PHYM_A::PHYM_1
    }
    #[doc = "Checks if the value of the field is `PHYM_2`"]
    #[inline(always)]
    pub fn is_phym_2(&self) -> bool {
        *self == PHYM_A::PHYM_2
    }
    #[doc = "Checks if the value of the field is `PHYM_3`"]
    #[inline(always)]
    pub fn is_phym_3(&self) -> bool {
        *self == PHYM_A::PHYM_3
    }
    #[doc = "Checks if the value of the field is `PHYM_4`"]
    #[inline(always)]
    pub fn is_phym_4(&self) -> bool {
        *self == PHYM_A::PHYM_4
    }
    #[doc = "Checks if the value of the field is `PHYM_5`"]
    #[inline(always)]
    pub fn is_phym_5(&self) -> bool {
        *self == PHYM_A::PHYM_5
    }
    #[doc = "Checks if the value of the field is `PHYM_6`"]
    #[inline(always)]
    pub fn is_phym_6(&self) -> bool {
        *self == PHYM_A::PHYM_6
    }
    #[doc = "Checks if the value of the field is `PHYM_7`"]
    #[inline(always)]
    pub fn is_phym_7(&self) -> bool {
        *self == PHYM_A::PHYM_7
    }
}
#[doc = "Serial interface mode capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SM_A {
    #[doc = "0: No Serial Engine, always use parallel signalling."]
    SM_0 = 0,
    #[doc = "1: Serial Engine present, always use serial signalling for FS/LS."]
    SM_1 = 1,
    #[doc = "2: Software programmable - Reset to use parallel signalling for FS/LS"]
    SM_2 = 2,
    #[doc = "3: Software programmable - Reset to use serial signalling for FS/LS"]
    SM_3 = 3,
}
impl From<SM_A> for u8 {
    #[inline(always)]
    fn from(variant: SM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SM`"]
pub type SM_R = crate::R<u8, SM_A>;
impl SM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM_A {
        match self.bits {
            0 => SM_A::SM_0,
            1 => SM_A::SM_1,
            2 => SM_A::SM_2,
            3 => SM_A::SM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM_0`"]
    #[inline(always)]
    pub fn is_sm_0(&self) -> bool {
        *self == SM_A::SM_0
    }
    #[doc = "Checks if the value of the field is `SM_1`"]
    #[inline(always)]
    pub fn is_sm_1(&self) -> bool {
        *self == SM_A::SM_1
    }
    #[doc = "Checks if the value of the field is `SM_2`"]
    #[inline(always)]
    pub fn is_sm_2(&self) -> bool {
        *self == SM_A::SM_2
    }
    #[doc = "Checks if the value of the field is `SM_3`"]
    #[inline(always)]
    pub fn is_sm_3(&self) -> bool {
        *self == SM_A::SM_3
    }
}
impl R {
    #[doc = "Bits 4:5 - Data width of the transciever connected to the controller core. PHYW bit reset value is"]
    #[inline(always)]
    pub fn phyw(&self) -> PHYW_R {
        PHYW_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:8 - Transciever type"]
    #[inline(always)]
    pub fn phym(&self) -> PHYM_R {
        PHYM_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:10 - Serial interface mode capability"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 9) & 0x03) as u8)
    }
}
