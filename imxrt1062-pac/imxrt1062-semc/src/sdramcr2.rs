#[doc = "Reader of register SDRAMCR2"]
pub type R = crate::R<u32, super::SDRAMCR2>;
#[doc = "Writer for register SDRAMCR2"]
pub type W = crate::W<u32, super::SDRAMCR2>;
#[doc = "Register SDRAMCR2 `reset()`'s with value 0x8000_0eee"]
impl crate::ResetValue for super::SDRAMCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0eee
    }
}
#[doc = "Reader of field `SRRC`"]
pub type SRRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRRC`"]
pub struct SRRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `REF2REF`"]
pub type REF2REF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REF2REF`"]
pub struct REF2REF_W<'a> {
    w: &'a mut W,
}
impl<'a> REF2REF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ACT2ACT`"]
pub type ACT2ACT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT2ACT`"]
pub struct ACT2ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT2ACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "SDRAM Idle timeout\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ITO_A {
    #[doc = "0: IDLE timeout period is 256*Prescale period."]
    ITO_0 = 0,
    #[doc = "1: IDLE timeout period is ITO*Prescale period."]
    ITO_1 = 1,
    #[doc = "2: IDLE timeout period is ITO*Prescale period."]
    ITO_2 = 2,
    #[doc = "3: IDLE timeout period is ITO*Prescale period."]
    ITO_3 = 3,
    #[doc = "4: IDLE timeout period is ITO*Prescale period."]
    ITO_4 = 4,
    #[doc = "5: IDLE timeout period is ITO*Prescale period."]
    ITO_5 = 5,
    #[doc = "6: IDLE timeout period is ITO*Prescale period."]
    ITO_6 = 6,
    #[doc = "7: IDLE timeout period is ITO*Prescale period."]
    ITO_7 = 7,
    #[doc = "8: IDLE timeout period is ITO*Prescale period."]
    ITO_8 = 8,
    #[doc = "9: IDLE timeout period is ITO*Prescale period."]
    ITO_9 = 9,
}
impl From<ITO_A> for u8 {
    #[inline(always)]
    fn from(variant: ITO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ITO`"]
pub type ITO_R = crate::R<u8, ITO_A>;
impl ITO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ITO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ITO_A::ITO_0),
            1 => Val(ITO_A::ITO_1),
            2 => Val(ITO_A::ITO_2),
            3 => Val(ITO_A::ITO_3),
            4 => Val(ITO_A::ITO_4),
            5 => Val(ITO_A::ITO_5),
            6 => Val(ITO_A::ITO_6),
            7 => Val(ITO_A::ITO_7),
            8 => Val(ITO_A::ITO_8),
            9 => Val(ITO_A::ITO_9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ITO_0`"]
    #[inline(always)]
    pub fn is_ito_0(&self) -> bool {
        *self == ITO_A::ITO_0
    }
    #[doc = "Checks if the value of the field is `ITO_1`"]
    #[inline(always)]
    pub fn is_ito_1(&self) -> bool {
        *self == ITO_A::ITO_1
    }
    #[doc = "Checks if the value of the field is `ITO_2`"]
    #[inline(always)]
    pub fn is_ito_2(&self) -> bool {
        *self == ITO_A::ITO_2
    }
    #[doc = "Checks if the value of the field is `ITO_3`"]
    #[inline(always)]
    pub fn is_ito_3(&self) -> bool {
        *self == ITO_A::ITO_3
    }
    #[doc = "Checks if the value of the field is `ITO_4`"]
    #[inline(always)]
    pub fn is_ito_4(&self) -> bool {
        *self == ITO_A::ITO_4
    }
    #[doc = "Checks if the value of the field is `ITO_5`"]
    #[inline(always)]
    pub fn is_ito_5(&self) -> bool {
        *self == ITO_A::ITO_5
    }
    #[doc = "Checks if the value of the field is `ITO_6`"]
    #[inline(always)]
    pub fn is_ito_6(&self) -> bool {
        *self == ITO_A::ITO_6
    }
    #[doc = "Checks if the value of the field is `ITO_7`"]
    #[inline(always)]
    pub fn is_ito_7(&self) -> bool {
        *self == ITO_A::ITO_7
    }
    #[doc = "Checks if the value of the field is `ITO_8`"]
    #[inline(always)]
    pub fn is_ito_8(&self) -> bool {
        *self == ITO_A::ITO_8
    }
    #[doc = "Checks if the value of the field is `ITO_9`"]
    #[inline(always)]
    pub fn is_ito_9(&self) -> bool {
        *self == ITO_A::ITO_9
    }
}
#[doc = "Write proxy for field `ITO`"]
pub struct ITO_W<'a> {
    w: &'a mut W,
}
impl<'a> ITO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IDLE timeout period is 256*Prescale period."]
    #[inline(always)]
    pub fn ito_0(self) -> &'a mut W {
        self.variant(ITO_A::ITO_0)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_1(self) -> &'a mut W {
        self.variant(ITO_A::ITO_1)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_2(self) -> &'a mut W {
        self.variant(ITO_A::ITO_2)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_3(self) -> &'a mut W {
        self.variant(ITO_A::ITO_3)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_4(self) -> &'a mut W {
        self.variant(ITO_A::ITO_4)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_5(self) -> &'a mut W {
        self.variant(ITO_A::ITO_5)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_6(self) -> &'a mut W {
        self.variant(ITO_A::ITO_6)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_7(self) -> &'a mut W {
        self.variant(ITO_A::ITO_7)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_8(self) -> &'a mut W {
        self.variant(ITO_A::ITO_8)
    }
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito_9(self) -> &'a mut W {
        self.variant(ITO_A::ITO_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Self Refresh Recovery time"]
    #[inline(always)]
    pub fn srrc(&self) -> SRRC_R {
        SRRC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Refresh to Refresh wait time"]
    #[inline(always)]
    pub fn ref2ref(&self) -> REF2REF_R {
        REF2REF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ACT to ACT wait time"]
    #[inline(always)]
    pub fn act2act(&self) -> ACT2ACT_R {
        ACT2ACT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SDRAM Idle timeout"]
    #[inline(always)]
    pub fn ito(&self) -> ITO_R {
        ITO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Refresh Recovery time"]
    #[inline(always)]
    pub fn srrc(&mut self) -> SRRC_W {
        SRRC_W { w: self }
    }
    #[doc = "Bits 8:15 - Refresh to Refresh wait time"]
    #[inline(always)]
    pub fn ref2ref(&mut self) -> REF2REF_W {
        REF2REF_W { w: self }
    }
    #[doc = "Bits 16:23 - ACT to ACT wait time"]
    #[inline(always)]
    pub fn act2act(&mut self) -> ACT2ACT_W {
        ACT2ACT_W { w: self }
    }
    #[doc = "Bits 24:31 - SDRAM Idle timeout"]
    #[inline(always)]
    pub fn ito(&mut self) -> ITO_W {
        ITO_W { w: self }
    }
}
