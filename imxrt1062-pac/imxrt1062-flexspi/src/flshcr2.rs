#[doc = "Reader of register FLSHCR2%s"]
pub type R = crate::R<u32, super::FLSHCR2>;
#[doc = "Writer for register FLSHCR2%s"]
pub type W = crate::W<u32, super::FLSHCR2>;
#[doc = "Register FLSHCR2%s `reset()`'s with value 0"]
impl crate::ResetValue for super::FLSHCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ARDSEQID`"]
pub type ARDSEQID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ARDSEQID`"]
pub struct ARDSEQID_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDSEQID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ARDSEQNUM`"]
pub type ARDSEQNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ARDSEQNUM`"]
pub struct ARDSEQNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDSEQNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `AWRSEQID`"]
pub type AWRSEQID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AWRSEQID`"]
pub struct AWRSEQID_W<'a> {
    w: &'a mut W,
}
impl<'a> AWRSEQID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `AWRSEQNUM`"]
pub type AWRSEQNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AWRSEQNUM`"]
pub struct AWRSEQNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> AWRSEQNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `AWRWAIT`"]
pub type AWRWAIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AWRWAIT`"]
pub struct AWRWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> AWRWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "AWRWAIT unit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AWRWAITUNIT_A {
    #[doc = "0: The AWRWAIT unit is 2 ahb clock cycle"]
    AWRWAITUNIT_0 = 0,
    #[doc = "1: The AWRWAIT unit is 8 ahb clock cycle"]
    AWRWAITUNIT_1 = 1,
    #[doc = "2: The AWRWAIT unit is 32 ahb clock cycle"]
    AWRWAITUNIT_2 = 2,
    #[doc = "3: The AWRWAIT unit is 128 ahb clock cycle"]
    AWRWAITUNIT_3 = 3,
    #[doc = "4: The AWRWAIT unit is 512 ahb clock cycle"]
    AWRWAITUNIT_4 = 4,
    #[doc = "5: The AWRWAIT unit is 2048 ahb clock cycle"]
    AWRWAITUNIT_5 = 5,
    #[doc = "6: The AWRWAIT unit is 8192 ahb clock cycle"]
    AWRWAITUNIT_6 = 6,
    #[doc = "7: The AWRWAIT unit is 32768 ahb clock cycle"]
    AWRWAITUNIT_7 = 7,
}
impl From<AWRWAITUNIT_A> for u8 {
    #[inline(always)]
    fn from(variant: AWRWAITUNIT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AWRWAITUNIT`"]
pub type AWRWAITUNIT_R = crate::R<u8, AWRWAITUNIT_A>;
impl AWRWAITUNIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWRWAITUNIT_A {
        match self.bits {
            0 => AWRWAITUNIT_A::AWRWAITUNIT_0,
            1 => AWRWAITUNIT_A::AWRWAITUNIT_1,
            2 => AWRWAITUNIT_A::AWRWAITUNIT_2,
            3 => AWRWAITUNIT_A::AWRWAITUNIT_3,
            4 => AWRWAITUNIT_A::AWRWAITUNIT_4,
            5 => AWRWAITUNIT_A::AWRWAITUNIT_5,
            6 => AWRWAITUNIT_A::AWRWAITUNIT_6,
            7 => AWRWAITUNIT_A::AWRWAITUNIT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_0`"]
    #[inline(always)]
    pub fn is_awrwaitunit_0(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_0
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_1`"]
    #[inline(always)]
    pub fn is_awrwaitunit_1(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_1
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_2`"]
    #[inline(always)]
    pub fn is_awrwaitunit_2(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_2
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_3`"]
    #[inline(always)]
    pub fn is_awrwaitunit_3(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_3
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_4`"]
    #[inline(always)]
    pub fn is_awrwaitunit_4(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_4
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_5`"]
    #[inline(always)]
    pub fn is_awrwaitunit_5(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_5
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_6`"]
    #[inline(always)]
    pub fn is_awrwaitunit_6(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_6
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_7`"]
    #[inline(always)]
    pub fn is_awrwaitunit_7(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_7
    }
}
#[doc = "Write proxy for field `AWRWAITUNIT`"]
pub struct AWRWAITUNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> AWRWAITUNIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWRWAITUNIT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The AWRWAIT unit is 2 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_0(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_0)
    }
    #[doc = "The AWRWAIT unit is 8 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_1(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_1)
    }
    #[doc = "The AWRWAIT unit is 32 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_2(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_2)
    }
    #[doc = "The AWRWAIT unit is 128 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_3(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_3)
    }
    #[doc = "The AWRWAIT unit is 512 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_4(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_4)
    }
    #[doc = "The AWRWAIT unit is 2048 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_5(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_5)
    }
    #[doc = "The AWRWAIT unit is 8192 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_6(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_6)
    }
    #[doc = "The AWRWAIT unit is 32768 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_7(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `CLRINSTRPTR`"]
pub type CLRINSTRPTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLRINSTRPTR`"]
pub struct CLRINSTRPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRINSTRPTR_W<'a> {
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
    #[doc = "Bits 0:3 - Sequence Index for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub fn ardseqid(&self) -> ARDSEQID_R {
        ARDSEQID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - Sequence Number for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub fn ardseqnum(&self) -> ARDSEQNUM_R {
        ARDSEQNUM_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Sequence Index for AHB Write triggered Command."]
    #[inline(always)]
    pub fn awrseqid(&self) -> AWRSEQID_R {
        AWRSEQID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Sequence Number for AHB Write triggered Command."]
    #[inline(always)]
    pub fn awrseqnum(&self) -> AWRSEQNUM_R {
        AWRSEQNUM_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:27 - For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    #[inline(always)]
    pub fn awrwait(&self) -> AWRWAIT_R {
        AWRWAIT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30 - AWRWAIT unit"]
    #[inline(always)]
    pub fn awrwaitunit(&self) -> AWRWAITUNIT_R {
        AWRWAITUNIT_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    #[inline(always)]
    pub fn clrinstrptr(&self) -> CLRINSTRPTR_R {
        CLRINSTRPTR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sequence Index for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub fn ardseqid(&mut self) -> ARDSEQID_W {
        ARDSEQID_W { w: self }
    }
    #[doc = "Bits 5:7 - Sequence Number for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub fn ardseqnum(&mut self) -> ARDSEQNUM_W {
        ARDSEQNUM_W { w: self }
    }
    #[doc = "Bits 8:11 - Sequence Index for AHB Write triggered Command."]
    #[inline(always)]
    pub fn awrseqid(&mut self) -> AWRSEQID_W {
        AWRSEQID_W { w: self }
    }
    #[doc = "Bits 13:15 - Sequence Number for AHB Write triggered Command."]
    #[inline(always)]
    pub fn awrseqnum(&mut self) -> AWRSEQNUM_W {
        AWRSEQNUM_W { w: self }
    }
    #[doc = "Bits 16:27 - For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    #[inline(always)]
    pub fn awrwait(&mut self) -> AWRWAIT_W {
        AWRWAIT_W { w: self }
    }
    #[doc = "Bits 28:30 - AWRWAIT unit"]
    #[inline(always)]
    pub fn awrwaitunit(&mut self) -> AWRWAITUNIT_W {
        AWRWAITUNIT_W { w: self }
    }
    #[doc = "Bit 31 - Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    #[inline(always)]
    pub fn clrinstrptr(&mut self) -> CLRINSTRPTR_W {
        CLRINSTRPTR_W { w: self }
    }
}
