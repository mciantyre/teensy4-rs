#[doc = "Reader of register MASK"]
pub type R = crate::R<u16, super::MASK>;
#[doc = "Writer for register MASK"]
pub type W = crate::W<u16, super::MASK>;
#[doc = "Register MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::MASK {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PWM_X Masks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MASKX_A {
    #[doc = "0: PWM_X output normal."]
    MASKX_0 = 0,
    #[doc = "1: PWM_X output masked."]
    MASKX_1 = 1,
}
impl From<MASKX_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MASKX`"]
pub type MASKX_R = crate::R<u8, MASKX_A>;
impl MASKX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MASKX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MASKX_A::MASKX_0),
            1 => Val(MASKX_A::MASKX_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASKX_0`"]
    #[inline(always)]
    pub fn is_maskx_0(&self) -> bool {
        *self == MASKX_A::MASKX_0
    }
    #[doc = "Checks if the value of the field is `MASKX_1`"]
    #[inline(always)]
    pub fn is_maskx_1(&self) -> bool {
        *self == MASKX_A::MASKX_1
    }
}
#[doc = "Write proxy for field `MASKX`"]
pub struct MASKX_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASKX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM_X output normal."]
    #[inline(always)]
    pub fn maskx_0(self) -> &'a mut W {
        self.variant(MASKX_A::MASKX_0)
    }
    #[doc = "PWM_X output masked."]
    #[inline(always)]
    pub fn maskx_1(self) -> &'a mut W {
        self.variant(MASKX_A::MASKX_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "PWM_B Masks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MASKB_A {
    #[doc = "0: PWM_B output normal."]
    MASKB_0 = 0,
    #[doc = "1: PWM_B output masked."]
    MASKB_1 = 1,
}
impl From<MASKB_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MASKB`"]
pub type MASKB_R = crate::R<u8, MASKB_A>;
impl MASKB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MASKB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MASKB_A::MASKB_0),
            1 => Val(MASKB_A::MASKB_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASKB_0`"]
    #[inline(always)]
    pub fn is_maskb_0(&self) -> bool {
        *self == MASKB_A::MASKB_0
    }
    #[doc = "Checks if the value of the field is `MASKB_1`"]
    #[inline(always)]
    pub fn is_maskb_1(&self) -> bool {
        *self == MASKB_A::MASKB_1
    }
}
#[doc = "Write proxy for field `MASKB`"]
pub struct MASKB_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASKB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM_B output normal."]
    #[inline(always)]
    pub fn maskb_0(self) -> &'a mut W {
        self.variant(MASKB_A::MASKB_0)
    }
    #[doc = "PWM_B output masked."]
    #[inline(always)]
    pub fn maskb_1(self) -> &'a mut W {
        self.variant(MASKB_A::MASKB_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "PWM_A Masks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MASKA_A {
    #[doc = "0: PWM_A output normal."]
    MASKA_0 = 0,
    #[doc = "1: PWM_A output masked."]
    MASKA_1 = 1,
}
impl From<MASKA_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MASKA`"]
pub type MASKA_R = crate::R<u8, MASKA_A>;
impl MASKA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MASKA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MASKA_A::MASKA_0),
            1 => Val(MASKA_A::MASKA_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MASKA_0`"]
    #[inline(always)]
    pub fn is_maska_0(&self) -> bool {
        *self == MASKA_A::MASKA_0
    }
    #[doc = "Checks if the value of the field is `MASKA_1`"]
    #[inline(always)]
    pub fn is_maska_1(&self) -> bool {
        *self == MASKA_A::MASKA_1
    }
}
#[doc = "Write proxy for field `MASKA`"]
pub struct MASKA_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASKA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM_A output normal."]
    #[inline(always)]
    pub fn maska_0(self) -> &'a mut W {
        self.variant(MASKA_A::MASKA_0)
    }
    #[doc = "PWM_A output masked."]
    #[inline(always)]
    pub fn maska_1(self) -> &'a mut W {
        self.variant(MASKA_A::MASKA_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Update Mask Bits Immediately\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UPDATE_MASK_AW {
    #[doc = "0: Normal operation. MASK* bits within the corresponding submodule are not updated until a FORCE_OUT event occurs within the submodule."]
    UPDATE_MASK_0 = 0,
    #[doc = "1: Immediate operation. MASK* bits within the corresponding submodule are updated on the following clock edge after setting this bit."]
    UPDATE_MASK_1 = 1,
}
impl From<UPDATE_MASK_AW> for u8 {
    #[inline(always)]
    fn from(variant: UPDATE_MASK_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `UPDATE_MASK`"]
pub struct UPDATE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATE_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDATE_MASK_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal operation. MASK* bits within the corresponding submodule are not updated until a FORCE_OUT event occurs within the submodule."]
    #[inline(always)]
    pub fn update_mask_0(self) -> &'a mut W {
        self.variant(UPDATE_MASK_AW::UPDATE_MASK_0)
    }
    #[doc = "Immediate operation. MASK* bits within the corresponding submodule are updated on the following clock edge after setting this bit."]
    #[inline(always)]
    pub fn update_mask_1(self) -> &'a mut W {
        self.variant(UPDATE_MASK_AW::UPDATE_MASK_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PWM_X Masks"]
    #[inline(always)]
    pub fn maskx(&self) -> MASKX_R {
        MASKX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PWM_B Masks"]
    #[inline(always)]
    pub fn maskb(&self) -> MASKB_R {
        MASKB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PWM_A Masks"]
    #[inline(always)]
    pub fn maska(&self) -> MASKA_R {
        MASKA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM_X Masks"]
    #[inline(always)]
    pub fn maskx(&mut self) -> MASKX_W {
        MASKX_W { w: self }
    }
    #[doc = "Bits 4:7 - PWM_B Masks"]
    #[inline(always)]
    pub fn maskb(&mut self) -> MASKB_W {
        MASKB_W { w: self }
    }
    #[doc = "Bits 8:11 - PWM_A Masks"]
    #[inline(always)]
    pub fn maska(&mut self) -> MASKA_W {
        MASKA_W { w: self }
    }
    #[doc = "Bits 12:15 - Update Mask Bits Immediately"]
    #[inline(always)]
    pub fn update_mask(&mut self) -> UPDATE_MASK_W {
        UPDATE_MASK_W { w: self }
    }
}
