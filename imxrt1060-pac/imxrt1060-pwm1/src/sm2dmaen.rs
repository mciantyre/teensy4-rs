#[doc = "Reader of register SM2DMAEN"]
pub type R = crate::R<u16, super::SM2DMAEN>;
#[doc = "Writer for register SM2DMAEN"]
pub type W = crate::W<u16, super::SM2DMAEN>;
#[doc = "Register SM2DMAEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SM2DMAEN {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CX0DE`"]
pub type CX0DE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CX0DE`"]
pub struct CX0DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CX0DE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CX1DE`"]
pub type CX1DE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CX1DE`"]
pub struct CX1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CX1DE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CB0DE`"]
pub type CB0DE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CB0DE`"]
pub struct CB0DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CB0DE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CB1DE`"]
pub type CB1DE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CB1DE`"]
pub struct CB1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CB1DE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CA0DE`"]
pub type CA0DE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CA0DE`"]
pub struct CA0DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CA0DE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CA1DE`"]
pub type CA1DE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CA1DE`"]
pub struct CA1DE_W<'a> {
    w: &'a mut W,
}
impl<'a> CA1DE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Capture DMA Enable Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTDE_A {
    #[doc = "0: Read DMA requests disabled."]
    CAPTDE_0,
    #[doc = "1: Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to also be set in order to determine to which watermark(s) the DMA request is sensitive."]
    CAPTDE_1,
    #[doc = "2: A local sync (VAL1 matches counter) sets the read DMA request."]
    CAPTDE_2,
    #[doc = "3: A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    CAPTDE_3,
}
impl From<CAPTDE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTDE_A) -> Self {
        match variant {
            CAPTDE_A::CAPTDE_0 => 0,
            CAPTDE_A::CAPTDE_1 => 1,
            CAPTDE_A::CAPTDE_2 => 2,
            CAPTDE_A::CAPTDE_3 => 3,
        }
    }
}
#[doc = "Reader of field `CAPTDE`"]
pub type CAPTDE_R = crate::R<u8, CAPTDE_A>;
impl CAPTDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTDE_A {
        match self.bits {
            0 => CAPTDE_A::CAPTDE_0,
            1 => CAPTDE_A::CAPTDE_1,
            2 => CAPTDE_A::CAPTDE_2,
            3 => CAPTDE_A::CAPTDE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAPTDE_0`"]
    #[inline(always)]
    pub fn is_captde_0(&self) -> bool {
        *self == CAPTDE_A::CAPTDE_0
    }
    #[doc = "Checks if the value of the field is `CAPTDE_1`"]
    #[inline(always)]
    pub fn is_captde_1(&self) -> bool {
        *self == CAPTDE_A::CAPTDE_1
    }
    #[doc = "Checks if the value of the field is `CAPTDE_2`"]
    #[inline(always)]
    pub fn is_captde_2(&self) -> bool {
        *self == CAPTDE_A::CAPTDE_2
    }
    #[doc = "Checks if the value of the field is `CAPTDE_3`"]
    #[inline(always)]
    pub fn is_captde_3(&self) -> bool {
        *self == CAPTDE_A::CAPTDE_3
    }
}
#[doc = "Write proxy for field `CAPTDE`"]
pub struct CAPTDE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTDE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Read DMA requests disabled."]
    #[inline(always)]
    pub fn captde_0(self) -> &'a mut W {
        self.variant(CAPTDE_A::CAPTDE_0)
    }
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to also be set in order to determine to which watermark(s) the DMA request is sensitive."]
    #[inline(always)]
    pub fn captde_1(self) -> &'a mut W {
        self.variant(CAPTDE_A::CAPTDE_1)
    }
    #[doc = "A local sync (VAL1 matches counter) sets the read DMA request."]
    #[inline(always)]
    pub fn captde_2(self) -> &'a mut W {
        self.variant(CAPTDE_A::CAPTDE_2)
    }
    #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    #[inline(always)]
    pub fn captde_3(self) -> &'a mut W {
        self.variant(CAPTDE_A::CAPTDE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u16) & 0x03) << 6);
        self.w
    }
}
#[doc = "FIFO Watermark AND Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAND_A {
    #[doc = "0: Selected FIFO watermarks are OR'ed together."]
    FAND_0,
    #[doc = "1: Selected FIFO watermarks are AND'ed together."]
    FAND_1,
}
impl From<FAND_A> for bool {
    #[inline(always)]
    fn from(variant: FAND_A) -> Self {
        match variant {
            FAND_A::FAND_0 => false,
            FAND_A::FAND_1 => true,
        }
    }
}
#[doc = "Reader of field `FAND`"]
pub type FAND_R = crate::R<bool, FAND_A>;
impl FAND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAND_A {
        match self.bits {
            false => FAND_A::FAND_0,
            true => FAND_A::FAND_1,
        }
    }
    #[doc = "Checks if the value of the field is `FAND_0`"]
    #[inline(always)]
    pub fn is_fand_0(&self) -> bool {
        *self == FAND_A::FAND_0
    }
    #[doc = "Checks if the value of the field is `FAND_1`"]
    #[inline(always)]
    pub fn is_fand_1(&self) -> bool {
        *self == FAND_A::FAND_1
    }
}
#[doc = "Write proxy for field `FAND`"]
pub struct FAND_W<'a> {
    w: &'a mut W,
}
impl<'a> FAND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    #[inline(always)]
    pub fn fand_0(self) -> &'a mut W {
        self.variant(FAND_A::FAND_0)
    }
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    #[inline(always)]
    pub fn fand_1(self) -> &'a mut W {
        self.variant(FAND_A::FAND_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Value Registers DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALDE_A {
    #[doc = "0: DMA write requests disabled"]
    VALDE_0,
    #[doc = "1: DMA write requests for the VALx and FRACVALx registers enabled"]
    VALDE_1,
}
impl From<VALDE_A> for bool {
    #[inline(always)]
    fn from(variant: VALDE_A) -> Self {
        match variant {
            VALDE_A::VALDE_0 => false,
            VALDE_A::VALDE_1 => true,
        }
    }
}
#[doc = "Reader of field `VALDE`"]
pub type VALDE_R = crate::R<bool, VALDE_A>;
impl VALDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALDE_A {
        match self.bits {
            false => VALDE_A::VALDE_0,
            true => VALDE_A::VALDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALDE_0`"]
    #[inline(always)]
    pub fn is_valde_0(&self) -> bool {
        *self == VALDE_A::VALDE_0
    }
    #[doc = "Checks if the value of the field is `VALDE_1`"]
    #[inline(always)]
    pub fn is_valde_1(&self) -> bool {
        *self == VALDE_A::VALDE_1
    }
}
#[doc = "Write proxy for field `VALDE`"]
pub struct VALDE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VALDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA write requests disabled"]
    #[inline(always)]
    pub fn valde_0(self) -> &'a mut W {
        self.variant(VALDE_A::VALDE_0)
    }
    #[doc = "DMA write requests for the VALx and FRACVALx registers enabled"]
    #[inline(always)]
    pub fn valde_1(self) -> &'a mut W {
        self.variant(VALDE_A::VALDE_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub fn cx0de(&self) -> CX0DE_R {
        CX0DE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub fn cx1de(&self) -> CX1DE_R {
        CX1DE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub fn cb0de(&self) -> CB0DE_R {
        CB0DE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub fn cb1de(&self) -> CB1DE_R {
        CB1DE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub fn ca0de(&self) -> CA0DE_R {
        CA0DE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub fn ca1de(&self) -> CA1DE_R {
        CA1DE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Capture DMA Enable Source Select"]
    #[inline(always)]
    pub fn captde(&self) -> CAPTDE_R {
        CAPTDE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - FIFO Watermark AND Control"]
    #[inline(always)]
    pub fn fand(&self) -> FAND_R {
        FAND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Value Registers DMA Enable"]
    #[inline(always)]
    pub fn valde(&self) -> VALDE_R {
        VALDE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture X0 FIFO DMA Enable"]
    #[inline(always)]
    pub fn cx0de(&mut self) -> CX0DE_W {
        CX0DE_W { w: self }
    }
    #[doc = "Bit 1 - Capture X1 FIFO DMA Enable"]
    #[inline(always)]
    pub fn cx1de(&mut self) -> CX1DE_W {
        CX1DE_W { w: self }
    }
    #[doc = "Bit 2 - Capture B0 FIFO DMA Enable"]
    #[inline(always)]
    pub fn cb0de(&mut self) -> CB0DE_W {
        CB0DE_W { w: self }
    }
    #[doc = "Bit 3 - Capture B1 FIFO DMA Enable"]
    #[inline(always)]
    pub fn cb1de(&mut self) -> CB1DE_W {
        CB1DE_W { w: self }
    }
    #[doc = "Bit 4 - Capture A0 FIFO DMA Enable"]
    #[inline(always)]
    pub fn ca0de(&mut self) -> CA0DE_W {
        CA0DE_W { w: self }
    }
    #[doc = "Bit 5 - Capture A1 FIFO DMA Enable"]
    #[inline(always)]
    pub fn ca1de(&mut self) -> CA1DE_W {
        CA1DE_W { w: self }
    }
    #[doc = "Bits 6:7 - Capture DMA Enable Source Select"]
    #[inline(always)]
    pub fn captde(&mut self) -> CAPTDE_W {
        CAPTDE_W { w: self }
    }
    #[doc = "Bit 8 - FIFO Watermark AND Control"]
    #[inline(always)]
    pub fn fand(&mut self) -> FAND_W {
        FAND_W { w: self }
    }
    #[doc = "Bit 9 - Value Registers DMA Enable"]
    #[inline(always)]
    pub fn valde(&mut self) -> VALDE_W {
        VALDE_W { w: self }
    }
}
