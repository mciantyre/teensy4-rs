#[doc = "Reader of register IPCR1"]
pub type R = crate::R<u32, super::IPCR1>;
#[doc = "Writer for register IPCR1"]
pub type W = crate::W<u32, super::IPCR1>;
#[doc = "Register IPCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IPCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Data Size in Byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATSZ_A {
    #[doc = "0: 4"]
    DATSZ_0 = 0,
    #[doc = "1: 1"]
    DATSZ_1 = 1,
    #[doc = "2: 2"]
    DATSZ_2 = 2,
    #[doc = "3: 3"]
    DATSZ_3 = 3,
    #[doc = "4: 4"]
    DATSZ_4 = 4,
    #[doc = "5: 4"]
    DATSZ_5 = 5,
    #[doc = "6: 4"]
    DATSZ_6 = 6,
    #[doc = "7: 4"]
    DATSZ_7 = 7,
}
impl From<DATSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: DATSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DATSZ`"]
pub type DATSZ_R = crate::R<u8, DATSZ_A>;
impl DATSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATSZ_A {
        match self.bits {
            0 => DATSZ_A::DATSZ_0,
            1 => DATSZ_A::DATSZ_1,
            2 => DATSZ_A::DATSZ_2,
            3 => DATSZ_A::DATSZ_3,
            4 => DATSZ_A::DATSZ_4,
            5 => DATSZ_A::DATSZ_5,
            6 => DATSZ_A::DATSZ_6,
            7 => DATSZ_A::DATSZ_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATSZ_0`"]
    #[inline(always)]
    pub fn is_datsz_0(&self) -> bool {
        *self == DATSZ_A::DATSZ_0
    }
    #[doc = "Checks if the value of the field is `DATSZ_1`"]
    #[inline(always)]
    pub fn is_datsz_1(&self) -> bool {
        *self == DATSZ_A::DATSZ_1
    }
    #[doc = "Checks if the value of the field is `DATSZ_2`"]
    #[inline(always)]
    pub fn is_datsz_2(&self) -> bool {
        *self == DATSZ_A::DATSZ_2
    }
    #[doc = "Checks if the value of the field is `DATSZ_3`"]
    #[inline(always)]
    pub fn is_datsz_3(&self) -> bool {
        *self == DATSZ_A::DATSZ_3
    }
    #[doc = "Checks if the value of the field is `DATSZ_4`"]
    #[inline(always)]
    pub fn is_datsz_4(&self) -> bool {
        *self == DATSZ_A::DATSZ_4
    }
    #[doc = "Checks if the value of the field is `DATSZ_5`"]
    #[inline(always)]
    pub fn is_datsz_5(&self) -> bool {
        *self == DATSZ_A::DATSZ_5
    }
    #[doc = "Checks if the value of the field is `DATSZ_6`"]
    #[inline(always)]
    pub fn is_datsz_6(&self) -> bool {
        *self == DATSZ_A::DATSZ_6
    }
    #[doc = "Checks if the value of the field is `DATSZ_7`"]
    #[inline(always)]
    pub fn is_datsz_7(&self) -> bool {
        *self == DATSZ_A::DATSZ_7
    }
}
#[doc = "Write proxy for field `DATSZ`"]
pub struct DATSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> DATSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATSZ_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn datsz_0(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_0)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn datsz_1(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_1)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn datsz_2(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_2)
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn datsz_3(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_3)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn datsz_4(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_4)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn datsz_5(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_5)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn datsz_6(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_6)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn datsz_7(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `NAND_EXT_ADDR`"]
pub type NAND_EXT_ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NAND_EXT_ADDR`"]
pub struct NAND_EXT_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NAND_EXT_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Data Size in Byte"]
    #[inline(always)]
    pub fn datsz(&self) -> DATSZ_R {
        DATSZ_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - NAND Extended Address"]
    #[inline(always)]
    pub fn nand_ext_addr(&self) -> NAND_EXT_ADDR_R {
        NAND_EXT_ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data Size in Byte"]
    #[inline(always)]
    pub fn datsz(&mut self) -> DATSZ_W {
        DATSZ_W { w: self }
    }
    #[doc = "Bits 8:15 - NAND Extended Address"]
    #[inline(always)]
    pub fn nand_ext_addr(&mut self) -> NAND_EXT_ADDR_W {
        NAND_EXT_ADDR_W { w: self }
    }
}
