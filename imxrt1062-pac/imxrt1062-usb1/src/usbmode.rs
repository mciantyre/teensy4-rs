#[doc = "Reader of register USBMODE"]
pub type R = crate::R<u32, super::USBMODE>;
#[doc = "Writer for register USBMODE"]
pub type W = crate::W<u32, super::USBMODE>;
#[doc = "Register USBMODE `reset()`'s with value 0x5000"]
impl crate::ResetValue for super::USBMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5000
    }
}
#[doc = "Controller Mode - R/WO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: Idle \\[Default for combination host/device\\]"]
    CM_0 = 0,
    #[doc = "2: Device Controller \\[Default for device only controller\\]"]
    CM_2 = 2,
    #[doc = "3: Host Controller \\[Default for host only controller\\]"]
    CM_3 = 3,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CM`"]
pub type CM_R = crate::R<u8, CM_A>;
impl CM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CM_A::CM_0),
            2 => Val(CM_A::CM_2),
            3 => Val(CM_A::CM_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CM_0`"]
    #[inline(always)]
    pub fn is_cm_0(&self) -> bool {
        *self == CM_A::CM_0
    }
    #[doc = "Checks if the value of the field is `CM_2`"]
    #[inline(always)]
    pub fn is_cm_2(&self) -> bool {
        *self == CM_A::CM_2
    }
    #[doc = "Checks if the value of the field is `CM_3`"]
    #[inline(always)]
    pub fn is_cm_3(&self) -> bool {
        *self == CM_A::CM_3
    }
}
#[doc = "Write proxy for field `CM`"]
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Idle \\[Default for combination host/device\\]"]
    #[inline(always)]
    pub fn cm_0(self) -> &'a mut W {
        self.variant(CM_A::CM_0)
    }
    #[doc = "Device Controller \\[Default for device only controller\\]"]
    #[inline(always)]
    pub fn cm_2(self) -> &'a mut W {
        self.variant(CM_A::CM_2)
    }
    #[doc = "Host Controller \\[Default for host only controller\\]"]
    #[inline(always)]
    pub fn cm_3(self) -> &'a mut W {
        self.variant(CM_A::CM_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Endian Select - Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ES_A {
    #[doc = "0: Little Endian \\[Default\\]"]
    ES_0 = 0,
    #[doc = "1: Big Endian"]
    ES_1 = 1,
}
impl From<ES_A> for bool {
    #[inline(always)]
    fn from(variant: ES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ES`"]
pub type ES_R = crate::R<bool, ES_A>;
impl ES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ES_A {
        match self.bits {
            false => ES_A::ES_0,
            true => ES_A::ES_1,
        }
    }
    #[doc = "Checks if the value of the field is `ES_0`"]
    #[inline(always)]
    pub fn is_es_0(&self) -> bool {
        *self == ES_A::ES_0
    }
    #[doc = "Checks if the value of the field is `ES_1`"]
    #[inline(always)]
    pub fn is_es_1(&self) -> bool {
        *self == ES_A::ES_1
    }
}
#[doc = "Write proxy for field `ES`"]
pub struct ES_W<'a> {
    w: &'a mut W,
}
impl<'a> ES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ES_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Little Endian \\[Default\\]"]
    #[inline(always)]
    pub fn es_0(self) -> &'a mut W {
        self.variant(ES_A::ES_0)
    }
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn es_1(self) -> &'a mut W {
        self.variant(ES_A::ES_1)
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
#[doc = "Setup Lockout Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOM_A {
    #[doc = "0: Setup Lockouts On (default);"]
    SLOM_0 = 0,
    #[doc = "1: Setup Lockouts Off (DCD requires use of Setup Data Buffer Tripwire in USBCMDUSB Command Register ."]
    SLOM_1 = 1,
}
impl From<SLOM_A> for bool {
    #[inline(always)]
    fn from(variant: SLOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLOM`"]
pub type SLOM_R = crate::R<bool, SLOM_A>;
impl SLOM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOM_A {
        match self.bits {
            false => SLOM_A::SLOM_0,
            true => SLOM_A::SLOM_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLOM_0`"]
    #[inline(always)]
    pub fn is_slom_0(&self) -> bool {
        *self == SLOM_A::SLOM_0
    }
    #[doc = "Checks if the value of the field is `SLOM_1`"]
    #[inline(always)]
    pub fn is_slom_1(&self) -> bool {
        *self == SLOM_A::SLOM_1
    }
}
#[doc = "Write proxy for field `SLOM`"]
pub struct SLOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Setup Lockouts On (default);"]
    #[inline(always)]
    pub fn slom_0(self) -> &'a mut W {
        self.variant(SLOM_A::SLOM_0)
    }
    #[doc = "Setup Lockouts Off (DCD requires use of Setup Data Buffer Tripwire in USBCMDUSB Command Register ."]
    #[inline(always)]
    pub fn slom_1(self) -> &'a mut W {
        self.variant(SLOM_A::SLOM_1)
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
#[doc = "Reader of field `SDIS`"]
pub type SDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIS`"]
pub struct SDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Controller Mode - R/WO"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Endian Select - Read/Write"]
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Setup Lockout Mode"]
    #[inline(always)]
    pub fn slom(&self) -> SLOM_R {
        SLOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stream Disable Mode"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controller Mode - R/WO"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    #[doc = "Bit 2 - Endian Select - Read/Write"]
    #[inline(always)]
    pub fn es(&mut self) -> ES_W {
        ES_W { w: self }
    }
    #[doc = "Bit 3 - Setup Lockout Mode"]
    #[inline(always)]
    pub fn slom(&mut self) -> SLOM_W {
        SLOM_W { w: self }
    }
    #[doc = "Bit 4 - Stream Disable Mode"]
    #[inline(always)]
    pub fn sdis(&mut self) -> SDIS_W {
        SDIS_W { w: self }
    }
}
