#[doc = "Reader of register CNTR"]
pub type R = crate::R<u32, super::CNTR>;
#[doc = "Writer for register CNTR"]
pub type W = crate::W<u32, super::CNTR>;
#[doc = "Register CNTR `reset()`'s with value 0x0052_0000"]
impl crate::ResetValue for super::CNTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0052_0000
    }
}
#[doc = "MEGA domain power down request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEGA_PDN_REQ_A {
    #[doc = "0: No Request"]
    MEGA_PDN_REQ_0 = 0,
    #[doc = "1: Request power down sequence"]
    MEGA_PDN_REQ_1 = 1,
}
impl From<MEGA_PDN_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: MEGA_PDN_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEGA_PDN_REQ`"]
pub type MEGA_PDN_REQ_R = crate::R<bool, MEGA_PDN_REQ_A>;
impl MEGA_PDN_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEGA_PDN_REQ_A {
        match self.bits {
            false => MEGA_PDN_REQ_A::MEGA_PDN_REQ_0,
            true => MEGA_PDN_REQ_A::MEGA_PDN_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEGA_PDN_REQ_0`"]
    #[inline(always)]
    pub fn is_mega_pdn_req_0(&self) -> bool {
        *self == MEGA_PDN_REQ_A::MEGA_PDN_REQ_0
    }
    #[doc = "Checks if the value of the field is `MEGA_PDN_REQ_1`"]
    #[inline(always)]
    pub fn is_mega_pdn_req_1(&self) -> bool {
        *self == MEGA_PDN_REQ_A::MEGA_PDN_REQ_1
    }
}
#[doc = "Write proxy for field `MEGA_PDN_REQ`"]
pub struct MEGA_PDN_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MEGA_PDN_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEGA_PDN_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Request"]
    #[inline(always)]
    pub fn mega_pdn_req_0(self) -> &'a mut W {
        self.variant(MEGA_PDN_REQ_A::MEGA_PDN_REQ_0)
    }
    #[doc = "Request power down sequence"]
    #[inline(always)]
    pub fn mega_pdn_req_1(self) -> &'a mut W {
        self.variant(MEGA_PDN_REQ_A::MEGA_PDN_REQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "MEGA domain power up request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEGA_PUP_REQ_A {
    #[doc = "0: No Request"]
    MEGA_PUP_REQ_0 = 0,
    #[doc = "1: Request power up sequence"]
    MEGA_PUP_REQ_1 = 1,
}
impl From<MEGA_PUP_REQ_A> for bool {
    #[inline(always)]
    fn from(variant: MEGA_PUP_REQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEGA_PUP_REQ`"]
pub type MEGA_PUP_REQ_R = crate::R<bool, MEGA_PUP_REQ_A>;
impl MEGA_PUP_REQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEGA_PUP_REQ_A {
        match self.bits {
            false => MEGA_PUP_REQ_A::MEGA_PUP_REQ_0,
            true => MEGA_PUP_REQ_A::MEGA_PUP_REQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEGA_PUP_REQ_0`"]
    #[inline(always)]
    pub fn is_mega_pup_req_0(&self) -> bool {
        *self == MEGA_PUP_REQ_A::MEGA_PUP_REQ_0
    }
    #[doc = "Checks if the value of the field is `MEGA_PUP_REQ_1`"]
    #[inline(always)]
    pub fn is_mega_pup_req_1(&self) -> bool {
        *self == MEGA_PUP_REQ_A::MEGA_PUP_REQ_1
    }
}
#[doc = "Write proxy for field `MEGA_PUP_REQ`"]
pub struct MEGA_PUP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MEGA_PUP_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEGA_PUP_REQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Request"]
    #[inline(always)]
    pub fn mega_pup_req_0(self) -> &'a mut W {
        self.variant(MEGA_PUP_REQ_A::MEGA_PUP_REQ_0)
    }
    #[doc = "Request power up sequence"]
    #[inline(always)]
    pub fn mega_pup_req_1(self) -> &'a mut W {
        self.variant(MEGA_PUP_REQ_A::MEGA_PUP_REQ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "FlexRAM PDRAM0 Power Gate Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDRAM0_PGE_A {
    #[doc = "0: FlexRAM PDRAM0 domain will keep power on even if CPU core is power down."]
    PDRAM0_PGE_0 = 0,
    #[doc = "1: FlexRAM PDRAM0 domain will be power down once when CPU core is power down."]
    PDRAM0_PGE_1 = 1,
}
impl From<PDRAM0_PGE_A> for bool {
    #[inline(always)]
    fn from(variant: PDRAM0_PGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDRAM0_PGE`"]
pub type PDRAM0_PGE_R = crate::R<bool, PDRAM0_PGE_A>;
impl PDRAM0_PGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDRAM0_PGE_A {
        match self.bits {
            false => PDRAM0_PGE_A::PDRAM0_PGE_0,
            true => PDRAM0_PGE_A::PDRAM0_PGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PDRAM0_PGE_0`"]
    #[inline(always)]
    pub fn is_pdram0_pge_0(&self) -> bool {
        *self == PDRAM0_PGE_A::PDRAM0_PGE_0
    }
    #[doc = "Checks if the value of the field is `PDRAM0_PGE_1`"]
    #[inline(always)]
    pub fn is_pdram0_pge_1(&self) -> bool {
        *self == PDRAM0_PGE_A::PDRAM0_PGE_1
    }
}
#[doc = "Write proxy for field `PDRAM0_PGE`"]
pub struct PDRAM0_PGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDRAM0_PGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDRAM0_PGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FlexRAM PDRAM0 domain will keep power on even if CPU core is power down."]
    #[inline(always)]
    pub fn pdram0_pge_0(self) -> &'a mut W {
        self.variant(PDRAM0_PGE_A::PDRAM0_PGE_0)
    }
    #[doc = "FlexRAM PDRAM0 domain will be power down once when CPU core is power down."]
    #[inline(always)]
    pub fn pdram0_pge_1(self) -> &'a mut W {
        self.variant(PDRAM0_PGE_A::PDRAM0_PGE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - MEGA domain power down request"]
    #[inline(always)]
    pub fn mega_pdn_req(&self) -> MEGA_PDN_REQ_R {
        MEGA_PDN_REQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MEGA domain power up request"]
    #[inline(always)]
    pub fn mega_pup_req(&self) -> MEGA_PUP_REQ_R {
        MEGA_PUP_REQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 22 - FlexRAM PDRAM0 Power Gate Enable"]
    #[inline(always)]
    pub fn pdram0_pge(&self) -> PDRAM0_PGE_R {
        PDRAM0_PGE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - MEGA domain power down request"]
    #[inline(always)]
    pub fn mega_pdn_req(&mut self) -> MEGA_PDN_REQ_W {
        MEGA_PDN_REQ_W { w: self }
    }
    #[doc = "Bit 3 - MEGA domain power up request"]
    #[inline(always)]
    pub fn mega_pup_req(&mut self) -> MEGA_PUP_REQ_W {
        MEGA_PUP_REQ_W { w: self }
    }
    #[doc = "Bit 22 - FlexRAM PDRAM0 Power Gate Enable"]
    #[inline(always)]
    pub fn pdram0_pge(&mut self) -> PDRAM0_PGE_W {
        PDRAM0_PGE_W { w: self }
    }
}
