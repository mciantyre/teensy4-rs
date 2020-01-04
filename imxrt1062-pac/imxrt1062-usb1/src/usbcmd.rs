#[doc = "Reader of register USBCMD"]
pub type R = crate::R<u32, super::USBCMD>;
#[doc = "Writer for register USBCMD"]
pub type W = crate::W<u32, super::USBCMD>;
#[doc = "Register USBCMD `reset()`'s with value 0x0008_0000"]
impl crate::ResetValue for super::USBCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_0000
    }
}
#[doc = "Reader of field `RS`"]
pub type RS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS`"]
pub struct RS_W<'a> {
    w: &'a mut W,
}
impl<'a> RS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `FS_1`"]
pub type FS_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FS_1`"]
pub struct FS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Periodic Schedule Enable- Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSE_A {
    #[doc = "0: Do not process the Periodic Schedule"]
    PSE_0 = 0,
    #[doc = "1: Use the PERIODICLISTBASE register to access the Periodic Schedule."]
    PSE_1 = 1,
}
impl From<PSE_A> for bool {
    #[inline(always)]
    fn from(variant: PSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PSE`"]
pub type PSE_R = crate::R<bool, PSE_A>;
impl PSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSE_A {
        match self.bits {
            false => PSE_A::PSE_0,
            true => PSE_A::PSE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PSE_0`"]
    #[inline(always)]
    pub fn is_pse_0(&self) -> bool {
        *self == PSE_A::PSE_0
    }
    #[doc = "Checks if the value of the field is `PSE_1`"]
    #[inline(always)]
    pub fn is_pse_1(&self) -> bool {
        *self == PSE_A::PSE_1
    }
}
#[doc = "Write proxy for field `PSE`"]
pub struct PSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not process the Periodic Schedule"]
    #[inline(always)]
    pub fn pse_0(self) -> &'a mut W {
        self.variant(PSE_A::PSE_0)
    }
    #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule."]
    #[inline(always)]
    pub fn pse_1(self) -> &'a mut W {
        self.variant(PSE_A::PSE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Asynchronous Schedule Enable - Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASE_A {
    #[doc = "0: Do not process the Asynchronous Schedule."]
    ASE_0 = 0,
    #[doc = "1: Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
    ASE_1 = 1,
}
impl From<ASE_A> for bool {
    #[inline(always)]
    fn from(variant: ASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASE`"]
pub type ASE_R = crate::R<bool, ASE_A>;
impl ASE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASE_A {
        match self.bits {
            false => ASE_A::ASE_0,
            true => ASE_A::ASE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ASE_0`"]
    #[inline(always)]
    pub fn is_ase_0(&self) -> bool {
        *self == ASE_A::ASE_0
    }
    #[doc = "Checks if the value of the field is `ASE_1`"]
    #[inline(always)]
    pub fn is_ase_1(&self) -> bool {
        *self == ASE_A::ASE_1
    }
}
#[doc = "Write proxy for field `ASE`"]
pub struct ASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not process the Asynchronous Schedule."]
    #[inline(always)]
    pub fn ase_0(self) -> &'a mut W {
        self.variant(ASE_A::ASE_0)
    }
    #[doc = "Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
    #[inline(always)]
    pub fn ase_1(self) -> &'a mut W {
        self.variant(ASE_A::ASE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `IAA`"]
pub type IAA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IAA`"]
pub struct IAA_W<'a> {
    w: &'a mut W,
}
impl<'a> IAA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ASP`"]
pub type ASP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ASP`"]
pub struct ASP_W<'a> {
    w: &'a mut W,
}
impl<'a> ASP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ASPE`"]
pub type ASPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASPE`"]
pub struct ASPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ATDTW`"]
pub type ATDTW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATDTW`"]
pub struct ATDTW_W<'a> {
    w: &'a mut W,
}
impl<'a> ATDTW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SUTW`"]
pub type SUTW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUTW`"]
pub struct SUTW_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "See also bits 3-2 Frame List Size - (Read/Write or Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FS_2_A {
    #[doc = "0: 1024 elements (4096 bytes) Default value"]
    FS_2_0 = 0,
    #[doc = "1: 512 elements (2048 bytes)"]
    FS_2_1 = 1,
}
impl From<FS_2_A> for bool {
    #[inline(always)]
    fn from(variant: FS_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FS_2`"]
pub type FS_2_R = crate::R<bool, FS_2_A>;
impl FS_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS_2_A {
        match self.bits {
            false => FS_2_A::FS_2_0,
            true => FS_2_A::FS_2_1,
        }
    }
    #[doc = "Checks if the value of the field is `FS_2_0`"]
    #[inline(always)]
    pub fn is_fs_2_0(&self) -> bool {
        *self == FS_2_A::FS_2_0
    }
    #[doc = "Checks if the value of the field is `FS_2_1`"]
    #[inline(always)]
    pub fn is_fs_2_1(&self) -> bool {
        *self == FS_2_A::FS_2_1
    }
}
#[doc = "Write proxy for field `FS_2`"]
pub struct FS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> FS_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FS_2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "1024 elements (4096 bytes) Default value"]
    #[inline(always)]
    pub fn fs_2_0(self) -> &'a mut W {
        self.variant(FS_2_A::FS_2_0)
    }
    #[doc = "512 elements (2048 bytes)"]
    #[inline(always)]
    pub fn fs_2_1(self) -> &'a mut W {
        self.variant(FS_2_A::FS_2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Interrupt Threshold Control -Read/Write\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ITC_A {
    #[doc = "0: Immediate (no threshold)"]
    ITC_0 = 0,
    #[doc = "1: 1 micro-frame"]
    ITC_1 = 1,
    #[doc = "2: 2 micro-frames"]
    ITC_2 = 2,
    #[doc = "4: 4 micro-frames"]
    ITC_4 = 4,
    #[doc = "8: 8 micro-frames"]
    ITC_8 = 8,
    #[doc = "16: 16 micro-frames"]
    ITC_16 = 16,
    #[doc = "32: 32 micro-frames"]
    ITC_32 = 32,
    #[doc = "64: 64 micro-frames"]
    ITC_64 = 64,
}
impl From<ITC_A> for u8 {
    #[inline(always)]
    fn from(variant: ITC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ITC`"]
pub type ITC_R = crate::R<u8, ITC_A>;
impl ITC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ITC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ITC_A::ITC_0),
            1 => Val(ITC_A::ITC_1),
            2 => Val(ITC_A::ITC_2),
            4 => Val(ITC_A::ITC_4),
            8 => Val(ITC_A::ITC_8),
            16 => Val(ITC_A::ITC_16),
            32 => Val(ITC_A::ITC_32),
            64 => Val(ITC_A::ITC_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ITC_0`"]
    #[inline(always)]
    pub fn is_itc_0(&self) -> bool {
        *self == ITC_A::ITC_0
    }
    #[doc = "Checks if the value of the field is `ITC_1`"]
    #[inline(always)]
    pub fn is_itc_1(&self) -> bool {
        *self == ITC_A::ITC_1
    }
    #[doc = "Checks if the value of the field is `ITC_2`"]
    #[inline(always)]
    pub fn is_itc_2(&self) -> bool {
        *self == ITC_A::ITC_2
    }
    #[doc = "Checks if the value of the field is `ITC_4`"]
    #[inline(always)]
    pub fn is_itc_4(&self) -> bool {
        *self == ITC_A::ITC_4
    }
    #[doc = "Checks if the value of the field is `ITC_8`"]
    #[inline(always)]
    pub fn is_itc_8(&self) -> bool {
        *self == ITC_A::ITC_8
    }
    #[doc = "Checks if the value of the field is `ITC_16`"]
    #[inline(always)]
    pub fn is_itc_16(&self) -> bool {
        *self == ITC_A::ITC_16
    }
    #[doc = "Checks if the value of the field is `ITC_32`"]
    #[inline(always)]
    pub fn is_itc_32(&self) -> bool {
        *self == ITC_A::ITC_32
    }
    #[doc = "Checks if the value of the field is `ITC_64`"]
    #[inline(always)]
    pub fn is_itc_64(&self) -> bool {
        *self == ITC_A::ITC_64
    }
}
#[doc = "Write proxy for field `ITC`"]
pub struct ITC_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Immediate (no threshold)"]
    #[inline(always)]
    pub fn itc_0(self) -> &'a mut W {
        self.variant(ITC_A::ITC_0)
    }
    #[doc = "1 micro-frame"]
    #[inline(always)]
    pub fn itc_1(self) -> &'a mut W {
        self.variant(ITC_A::ITC_1)
    }
    #[doc = "2 micro-frames"]
    #[inline(always)]
    pub fn itc_2(self) -> &'a mut W {
        self.variant(ITC_A::ITC_2)
    }
    #[doc = "4 micro-frames"]
    #[inline(always)]
    pub fn itc_4(self) -> &'a mut W {
        self.variant(ITC_A::ITC_4)
    }
    #[doc = "8 micro-frames"]
    #[inline(always)]
    pub fn itc_8(self) -> &'a mut W {
        self.variant(ITC_A::ITC_8)
    }
    #[doc = "16 micro-frames"]
    #[inline(always)]
    pub fn itc_16(self) -> &'a mut W {
        self.variant(ITC_A::ITC_16)
    }
    #[doc = "32 micro-frames"]
    #[inline(always)]
    pub fn itc_32(self) -> &'a mut W {
        self.variant(ITC_A::ITC_32)
    }
    #[doc = "64 micro-frames"]
    #[inline(always)]
    pub fn itc_64(self) -> &'a mut W {
        self.variant(ITC_A::ITC_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Run/Stop (RS) - Read/Write"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controller Reset (RESET) - Read/Write"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - See description at bit 15"]
    #[inline(always)]
    pub fn fs_1(&self) -> FS_1_R {
        FS_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Periodic Schedule Enable- Read/Write"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable - Read/Write"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt on Async Advance Doorbell - Read/Write"]
    #[inline(always)]
    pub fn iaa(&self) -> IAA_R {
        IAA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park Mode Count - Read/Write"]
    #[inline(always)]
    pub fn asp(&self) -> ASP_R {
        ASP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable - Read/Write"]
    #[inline(always)]
    pub fn aspe(&self) -> ASPE_R {
        ASPE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Add dTD TripWire - Read/Write"]
    #[inline(always)]
    pub fn atdtw(&self) -> ATDTW_R {
        ATDTW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Setup TripWire - Read/Write"]
    #[inline(always)]
    pub fn sutw(&self) -> SUTW_R {
        SUTW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - See also bits 3-2 Frame List Size - (Read/Write or Read Only)"]
    #[inline(always)]
    pub fn fs_2(&self) -> FS_2_R {
        FS_2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control -Read/Write"]
    #[inline(always)]
    pub fn itc(&self) -> ITC_R {
        ITC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop (RS) - Read/Write"]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W {
        RS_W { w: self }
    }
    #[doc = "Bit 1 - Controller Reset (RESET) - Read/Write"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bits 2:3 - See description at bit 15"]
    #[inline(always)]
    pub fn fs_1(&mut self) -> FS_1_W {
        FS_1_W { w: self }
    }
    #[doc = "Bit 4 - Periodic Schedule Enable- Read/Write"]
    #[inline(always)]
    pub fn pse(&mut self) -> PSE_W {
        PSE_W { w: self }
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable - Read/Write"]
    #[inline(always)]
    pub fn ase(&mut self) -> ASE_W {
        ASE_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt on Async Advance Doorbell - Read/Write"]
    #[inline(always)]
    pub fn iaa(&mut self) -> IAA_W {
        IAA_W { w: self }
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park Mode Count - Read/Write"]
    #[inline(always)]
    pub fn asp(&mut self) -> ASP_W {
        ASP_W { w: self }
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable - Read/Write"]
    #[inline(always)]
    pub fn aspe(&mut self) -> ASPE_W {
        ASPE_W { w: self }
    }
    #[doc = "Bit 12 - Add dTD TripWire - Read/Write"]
    #[inline(always)]
    pub fn atdtw(&mut self) -> ATDTW_W {
        ATDTW_W { w: self }
    }
    #[doc = "Bit 13 - Setup TripWire - Read/Write"]
    #[inline(always)]
    pub fn sutw(&mut self) -> SUTW_W {
        SUTW_W { w: self }
    }
    #[doc = "Bit 15 - See also bits 3-2 Frame List Size - (Read/Write or Read Only)"]
    #[inline(always)]
    pub fn fs_2(&mut self) -> FS_2_W {
        FS_2_W { w: self }
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control -Read/Write"]
    #[inline(always)]
    pub fn itc(&mut self) -> ITC_W {
        ITC_W { w: self }
    }
}
