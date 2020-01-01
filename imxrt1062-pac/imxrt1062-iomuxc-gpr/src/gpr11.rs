#[doc = "Reader of register GPR11"]
pub type R = crate::R<u32, super::GPR11>;
#[doc = "Writer for register GPR11"]
pub type W = crate::W<u32, super::GPR11>;
#[doc = "Register GPR11 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPR11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Access control of memory region-0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum M7_APC_AC_R0_CTRL_A {
    #[doc = "0: No access protection"]
    M7_APC_AC_R0_CTRL_0 = 0,
    #[doc = "1: M7 debug protection enabled"]
    M7_APC_AC_R0_CTRL_1 = 1,
    #[doc = "2: FlexSPI access protection"]
    M7_APC_AC_R0_CTRL_2 = 2,
    #[doc = "3: Both M7 debug and FlexSPI access are protected"]
    M7_APC_AC_R0_CTRL_3 = 3,
}
impl From<M7_APC_AC_R0_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: M7_APC_AC_R0_CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `M7_APC_AC_R0_CTRL`"]
pub type M7_APC_AC_R0_CTRL_R = crate::R<u8, M7_APC_AC_R0_CTRL_A>;
impl M7_APC_AC_R0_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7_APC_AC_R0_CTRL_A {
        match self.bits {
            0 => M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_0,
            1 => M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_1,
            2 => M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_2,
            3 => M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R0_CTRL_0`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r0_ctrl_0(&self) -> bool {
        *self == M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_0
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R0_CTRL_1`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r0_ctrl_1(&self) -> bool {
        *self == M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_1
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R0_CTRL_2`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r0_ctrl_2(&self) -> bool {
        *self == M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_2
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R0_CTRL_3`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r0_ctrl_3(&self) -> bool {
        *self == M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_3
    }
}
#[doc = "Write proxy for field `M7_APC_AC_R0_CTRL`"]
pub struct M7_APC_AC_R0_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> M7_APC_AC_R0_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M7_APC_AC_R0_CTRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No access protection"]
    #[inline(always)]
    pub fn m7_apc_ac_r0_ctrl_0(self) -> &'a mut W {
        self.variant(M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_0)
    }
    #[doc = "M7 debug protection enabled"]
    #[inline(always)]
    pub fn m7_apc_ac_r0_ctrl_1(self) -> &'a mut W {
        self.variant(M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_1)
    }
    #[doc = "FlexSPI access protection"]
    #[inline(always)]
    pub fn m7_apc_ac_r0_ctrl_2(self) -> &'a mut W {
        self.variant(M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_2)
    }
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    #[inline(always)]
    pub fn m7_apc_ac_r0_ctrl_3(self) -> &'a mut W {
        self.variant(M7_APC_AC_R0_CTRL_A::M7_APC_AC_R0_CTRL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Access control of memory region-1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum M7_APC_AC_R1_CTRL_A {
    #[doc = "0: No access protection"]
    M7_APC_AC_R1_CTRL_0 = 0,
    #[doc = "1: M7 debug protection enabled"]
    M7_APC_AC_R1_CTRL_1 = 1,
    #[doc = "2: FlexSPI access protection"]
    M7_APC_AC_R1_CTRL_2 = 2,
    #[doc = "3: Both M7 debug and FlexSPI access are protected"]
    M7_APC_AC_R1_CTRL_3 = 3,
}
impl From<M7_APC_AC_R1_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: M7_APC_AC_R1_CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `M7_APC_AC_R1_CTRL`"]
pub type M7_APC_AC_R1_CTRL_R = crate::R<u8, M7_APC_AC_R1_CTRL_A>;
impl M7_APC_AC_R1_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7_APC_AC_R1_CTRL_A {
        match self.bits {
            0 => M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_0,
            1 => M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_1,
            2 => M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_2,
            3 => M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R1_CTRL_0`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r1_ctrl_0(&self) -> bool {
        *self == M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_0
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R1_CTRL_1`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r1_ctrl_1(&self) -> bool {
        *self == M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_1
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R1_CTRL_2`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r1_ctrl_2(&self) -> bool {
        *self == M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_2
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R1_CTRL_3`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r1_ctrl_3(&self) -> bool {
        *self == M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_3
    }
}
#[doc = "Write proxy for field `M7_APC_AC_R1_CTRL`"]
pub struct M7_APC_AC_R1_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> M7_APC_AC_R1_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M7_APC_AC_R1_CTRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No access protection"]
    #[inline(always)]
    pub fn m7_apc_ac_r1_ctrl_0(self) -> &'a mut W {
        self.variant(M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_0)
    }
    #[doc = "M7 debug protection enabled"]
    #[inline(always)]
    pub fn m7_apc_ac_r1_ctrl_1(self) -> &'a mut W {
        self.variant(M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_1)
    }
    #[doc = "FlexSPI access protection"]
    #[inline(always)]
    pub fn m7_apc_ac_r1_ctrl_2(self) -> &'a mut W {
        self.variant(M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_2)
    }
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    #[inline(always)]
    pub fn m7_apc_ac_r1_ctrl_3(self) -> &'a mut W {
        self.variant(M7_APC_AC_R1_CTRL_A::M7_APC_AC_R1_CTRL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Access control of memory region-2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum M7_APC_AC_R2_CTRL_A {
    #[doc = "0: No access protection"]
    M7_APC_AC_R2_CTRL_0 = 0,
    #[doc = "1: M7 debug protection enabled"]
    M7_APC_AC_R2_CTRL_1 = 1,
    #[doc = "2: FlexSPI access protection"]
    M7_APC_AC_R2_CTRL_2 = 2,
    #[doc = "3: Both M7 debug and FlexSPI access are protected"]
    M7_APC_AC_R2_CTRL_3 = 3,
}
impl From<M7_APC_AC_R2_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: M7_APC_AC_R2_CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `M7_APC_AC_R2_CTRL`"]
pub type M7_APC_AC_R2_CTRL_R = crate::R<u8, M7_APC_AC_R2_CTRL_A>;
impl M7_APC_AC_R2_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7_APC_AC_R2_CTRL_A {
        match self.bits {
            0 => M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_0,
            1 => M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_1,
            2 => M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_2,
            3 => M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R2_CTRL_0`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r2_ctrl_0(&self) -> bool {
        *self == M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_0
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R2_CTRL_1`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r2_ctrl_1(&self) -> bool {
        *self == M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_1
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R2_CTRL_2`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r2_ctrl_2(&self) -> bool {
        *self == M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_2
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R2_CTRL_3`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r2_ctrl_3(&self) -> bool {
        *self == M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_3
    }
}
#[doc = "Write proxy for field `M7_APC_AC_R2_CTRL`"]
pub struct M7_APC_AC_R2_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> M7_APC_AC_R2_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M7_APC_AC_R2_CTRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No access protection"]
    #[inline(always)]
    pub fn m7_apc_ac_r2_ctrl_0(self) -> &'a mut W {
        self.variant(M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_0)
    }
    #[doc = "M7 debug protection enabled"]
    #[inline(always)]
    pub fn m7_apc_ac_r2_ctrl_1(self) -> &'a mut W {
        self.variant(M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_1)
    }
    #[doc = "FlexSPI access protection"]
    #[inline(always)]
    pub fn m7_apc_ac_r2_ctrl_2(self) -> &'a mut W {
        self.variant(M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_2)
    }
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    #[inline(always)]
    pub fn m7_apc_ac_r2_ctrl_3(self) -> &'a mut W {
        self.variant(M7_APC_AC_R2_CTRL_A::M7_APC_AC_R2_CTRL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Access control of memory region-3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum M7_APC_AC_R3_CTRL_A {
    #[doc = "0: No access protection"]
    M7_APC_AC_R3_CTRL_0 = 0,
    #[doc = "1: M7 debug protection enabled"]
    M7_APC_AC_R3_CTRL_1 = 1,
    #[doc = "2: FlexSPI access protection"]
    M7_APC_AC_R3_CTRL_2 = 2,
    #[doc = "3: Both M7 debug and FlexSPI access are protected"]
    M7_APC_AC_R3_CTRL_3 = 3,
}
impl From<M7_APC_AC_R3_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: M7_APC_AC_R3_CTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `M7_APC_AC_R3_CTRL`"]
pub type M7_APC_AC_R3_CTRL_R = crate::R<u8, M7_APC_AC_R3_CTRL_A>;
impl M7_APC_AC_R3_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7_APC_AC_R3_CTRL_A {
        match self.bits {
            0 => M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_0,
            1 => M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_1,
            2 => M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_2,
            3 => M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R3_CTRL_0`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r3_ctrl_0(&self) -> bool {
        *self == M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_0
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R3_CTRL_1`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r3_ctrl_1(&self) -> bool {
        *self == M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_1
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R3_CTRL_2`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r3_ctrl_2(&self) -> bool {
        *self == M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_2
    }
    #[doc = "Checks if the value of the field is `M7_APC_AC_R3_CTRL_3`"]
    #[inline(always)]
    pub fn is_m7_apc_ac_r3_ctrl_3(&self) -> bool {
        *self == M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_3
    }
}
#[doc = "Write proxy for field `M7_APC_AC_R3_CTRL`"]
pub struct M7_APC_AC_R3_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> M7_APC_AC_R3_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M7_APC_AC_R3_CTRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No access protection"]
    #[inline(always)]
    pub fn m7_apc_ac_r3_ctrl_0(self) -> &'a mut W {
        self.variant(M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_0)
    }
    #[doc = "M7 debug protection enabled"]
    #[inline(always)]
    pub fn m7_apc_ac_r3_ctrl_1(self) -> &'a mut W {
        self.variant(M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_1)
    }
    #[doc = "FlexSPI access protection"]
    #[inline(always)]
    pub fn m7_apc_ac_r3_ctrl_2(self) -> &'a mut W {
        self.variant(M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_2)
    }
    #[doc = "Both M7 debug and FlexSPI access are protected"]
    #[inline(always)]
    pub fn m7_apc_ac_r3_ctrl_3(self) -> &'a mut W {
        self.variant(M7_APC_AC_R3_CTRL_A::M7_APC_AC_R3_CTRL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `BEE_DE_RX_EN`"]
pub type BEE_DE_RX_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BEE_DE_RX_EN`"]
pub struct BEE_DE_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BEE_DE_RX_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Access control of memory region-0"]
    #[inline(always)]
    pub fn m7_apc_ac_r0_ctrl(&self) -> M7_APC_AC_R0_CTRL_R {
        M7_APC_AC_R0_CTRL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Access control of memory region-1"]
    #[inline(always)]
    pub fn m7_apc_ac_r1_ctrl(&self) -> M7_APC_AC_R1_CTRL_R {
        M7_APC_AC_R1_CTRL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Access control of memory region-2"]
    #[inline(always)]
    pub fn m7_apc_ac_r2_ctrl(&self) -> M7_APC_AC_R2_CTRL_R {
        M7_APC_AC_R2_CTRL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Access control of memory region-3"]
    #[inline(always)]
    pub fn m7_apc_ac_r3_ctrl(&self) -> M7_APC_AC_R3_CTRL_R {
        M7_APC_AC_R3_CTRL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - BEE data decryption of memory region-n (n = 3 to 0)"]
    #[inline(always)]
    pub fn bee_de_rx_en(&self) -> BEE_DE_RX_EN_R {
        BEE_DE_RX_EN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Access control of memory region-0"]
    #[inline(always)]
    pub fn m7_apc_ac_r0_ctrl(&mut self) -> M7_APC_AC_R0_CTRL_W {
        M7_APC_AC_R0_CTRL_W { w: self }
    }
    #[doc = "Bits 2:3 - Access control of memory region-1"]
    #[inline(always)]
    pub fn m7_apc_ac_r1_ctrl(&mut self) -> M7_APC_AC_R1_CTRL_W {
        M7_APC_AC_R1_CTRL_W { w: self }
    }
    #[doc = "Bits 4:5 - Access control of memory region-2"]
    #[inline(always)]
    pub fn m7_apc_ac_r2_ctrl(&mut self) -> M7_APC_AC_R2_CTRL_W {
        M7_APC_AC_R2_CTRL_W { w: self }
    }
    #[doc = "Bits 6:7 - Access control of memory region-3"]
    #[inline(always)]
    pub fn m7_apc_ac_r3_ctrl(&mut self) -> M7_APC_AC_R3_CTRL_W {
        M7_APC_AC_R3_CTRL_W { w: self }
    }
    #[doc = "Bits 8:11 - BEE data decryption of memory region-n (n = 3 to 0)"]
    #[inline(always)]
    pub fn bee_de_rx_en(&mut self) -> BEE_DE_RX_EN_W {
        BEE_DE_RX_EN_W { w: self }
    }
}
