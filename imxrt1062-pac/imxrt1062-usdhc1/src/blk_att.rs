#[doc = "Reader of register BLK_ATT"]
pub type R = crate::R<u32, super::BLK_ATT>;
#[doc = "Writer for register BLK_ATT"]
pub type W = crate::W<u32, super::BLK_ATT>;
#[doc = "Register BLK_ATT `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK_ATT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Block Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum BLKSIZE_A {
    #[doc = "0: No data transfer"]
    BLKSIZE_0 = 0,
    #[doc = "1: 1 Byte"]
    BLKSIZE_1 = 1,
    #[doc = "2: 2 Bytes"]
    BLKSIZE_2 = 2,
    #[doc = "3: 3 Bytes"]
    BLKSIZE_3 = 3,
    #[doc = "4: 4 Bytes"]
    BLKSIZE_4 = 4,
    #[doc = "511: 511 Bytes"]
    BLKSIZE_511 = 511,
    #[doc = "512: 512 Bytes"]
    BLKSIZE_512 = 512,
    #[doc = "2048: 2048 Bytes"]
    BLKSIZE_2048 = 2048,
    #[doc = "4096: 4096 Bytes"]
    BLKSIZE_4096 = 4096,
}
impl From<BLKSIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: BLKSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLKSIZE`"]
pub type BLKSIZE_R = crate::R<u16, BLKSIZE_A>;
impl BLKSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, BLKSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BLKSIZE_A::BLKSIZE_0),
            1 => Val(BLKSIZE_A::BLKSIZE_1),
            2 => Val(BLKSIZE_A::BLKSIZE_2),
            3 => Val(BLKSIZE_A::BLKSIZE_3),
            4 => Val(BLKSIZE_A::BLKSIZE_4),
            511 => Val(BLKSIZE_A::BLKSIZE_511),
            512 => Val(BLKSIZE_A::BLKSIZE_512),
            2048 => Val(BLKSIZE_A::BLKSIZE_2048),
            4096 => Val(BLKSIZE_A::BLKSIZE_4096),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_0`"]
    #[inline(always)]
    pub fn is_blksize_0(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_0
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_1`"]
    #[inline(always)]
    pub fn is_blksize_1(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_1
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_2`"]
    #[inline(always)]
    pub fn is_blksize_2(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_2
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_3`"]
    #[inline(always)]
    pub fn is_blksize_3(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_3
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_4`"]
    #[inline(always)]
    pub fn is_blksize_4(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_4
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_511`"]
    #[inline(always)]
    pub fn is_blksize_511(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_511
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_512`"]
    #[inline(always)]
    pub fn is_blksize_512(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_512
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_2048`"]
    #[inline(always)]
    pub fn is_blksize_2048(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_2048
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_4096`"]
    #[inline(always)]
    pub fn is_blksize_4096(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_4096
    }
}
#[doc = "Write proxy for field `BLKSIZE`"]
pub struct BLKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLKSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No data transfer"]
    #[inline(always)]
    pub fn blksize_0(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_0)
    }
    #[doc = "1 Byte"]
    #[inline(always)]
    pub fn blksize_1(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_1)
    }
    #[doc = "2 Bytes"]
    #[inline(always)]
    pub fn blksize_2(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_2)
    }
    #[doc = "3 Bytes"]
    #[inline(always)]
    pub fn blksize_3(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_3)
    }
    #[doc = "4 Bytes"]
    #[inline(always)]
    pub fn blksize_4(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_4)
    }
    #[doc = "511 Bytes"]
    #[inline(always)]
    pub fn blksize_511(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_511)
    }
    #[doc = "512 Bytes"]
    #[inline(always)]
    pub fn blksize_512(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_512)
    }
    #[doc = "2048 Bytes"]
    #[inline(always)]
    pub fn blksize_2048(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_2048)
    }
    #[doc = "4096 Bytes"]
    #[inline(always)]
    pub fn blksize_4096(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_4096)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Block Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum BLKCNT_A {
    #[doc = "0: Stop Count"]
    BLKCNT_0 = 0,
    #[doc = "1: 1 block"]
    BLKCNT_1 = 1,
    #[doc = "2: 2 blocks"]
    BLKCNT_2 = 2,
    #[doc = "65535: 65535 blocks"]
    BLKCNT_65535 = 65535,
}
impl From<BLKCNT_A> for u16 {
    #[inline(always)]
    fn from(variant: BLKCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLKCNT`"]
pub type BLKCNT_R = crate::R<u16, BLKCNT_A>;
impl BLKCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, BLKCNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BLKCNT_A::BLKCNT_0),
            1 => Val(BLKCNT_A::BLKCNT_1),
            2 => Val(BLKCNT_A::BLKCNT_2),
            65535 => Val(BLKCNT_A::BLKCNT_65535),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BLKCNT_0`"]
    #[inline(always)]
    pub fn is_blkcnt_0(&self) -> bool {
        *self == BLKCNT_A::BLKCNT_0
    }
    #[doc = "Checks if the value of the field is `BLKCNT_1`"]
    #[inline(always)]
    pub fn is_blkcnt_1(&self) -> bool {
        *self == BLKCNT_A::BLKCNT_1
    }
    #[doc = "Checks if the value of the field is `BLKCNT_2`"]
    #[inline(always)]
    pub fn is_blkcnt_2(&self) -> bool {
        *self == BLKCNT_A::BLKCNT_2
    }
    #[doc = "Checks if the value of the field is `BLKCNT_65535`"]
    #[inline(always)]
    pub fn is_blkcnt_65535(&self) -> bool {
        *self == BLKCNT_A::BLKCNT_65535
    }
}
#[doc = "Write proxy for field `BLKCNT`"]
pub struct BLKCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLKCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLKCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Stop Count"]
    #[inline(always)]
    pub fn blkcnt_0(self) -> &'a mut W {
        self.variant(BLKCNT_A::BLKCNT_0)
    }
    #[doc = "1 block"]
    #[inline(always)]
    pub fn blkcnt_1(self) -> &'a mut W {
        self.variant(BLKCNT_A::BLKCNT_1)
    }
    #[doc = "2 blocks"]
    #[inline(always)]
    pub fn blkcnt_2(self) -> &'a mut W {
        self.variant(BLKCNT_A::BLKCNT_2)
    }
    #[doc = "65535 blocks"]
    #[inline(always)]
    pub fn blkcnt_65535(self) -> &'a mut W {
        self.variant(BLKCNT_A::BLKCNT_65535)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Block Size"]
    #[inline(always)]
    pub fn blksize(&self) -> BLKSIZE_R {
        BLKSIZE_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:31 - Block Count"]
    #[inline(always)]
    pub fn blkcnt(&self) -> BLKCNT_R {
        BLKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Block Size"]
    #[inline(always)]
    pub fn blksize(&mut self) -> BLKSIZE_W {
        BLKSIZE_W { w: self }
    }
    #[doc = "Bits 16:31 - Block Count"]
    #[inline(always)]
    pub fn blkcnt(&mut self) -> BLKCNT_W {
        BLKCNT_W { w: self }
    }
}
