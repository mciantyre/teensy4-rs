#[doc = "Reader of register CSICR2"]
pub type R = crate::R<u32, super::CSICR2>;
#[doc = "Writer for register CSICR2"]
pub type W = crate::W<u32, super::CSICR2>;
#[doc = "Register CSICR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSICR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSC`"]
pub type HSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSC`"]
pub struct HSC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `VSC`"]
pub type VSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSC`"]
pub struct VSC_W<'a> {
    w: &'a mut W,
}
impl<'a> VSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Live View Resolution Mode. Selects the grid size used for live view resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVRM_A {
    #[doc = "0: 512 x 384"]
    LVRM_0 = 0,
    #[doc = "1: 448 x 336"]
    LVRM_1 = 1,
    #[doc = "2: 384 x 288"]
    LVRM_2 = 2,
    #[doc = "3: 384 x 256"]
    LVRM_3 = 3,
    #[doc = "4: 320 x 240"]
    LVRM_4 = 4,
    #[doc = "5: 288 x 216"]
    LVRM_5 = 5,
    #[doc = "6: 400 x 300"]
    LVRM_6 = 6,
}
impl From<LVRM_A> for u8 {
    #[inline(always)]
    fn from(variant: LVRM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LVRM`"]
pub type LVRM_R = crate::R<u8, LVRM_A>;
impl LVRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LVRM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LVRM_A::LVRM_0),
            1 => Val(LVRM_A::LVRM_1),
            2 => Val(LVRM_A::LVRM_2),
            3 => Val(LVRM_A::LVRM_3),
            4 => Val(LVRM_A::LVRM_4),
            5 => Val(LVRM_A::LVRM_5),
            6 => Val(LVRM_A::LVRM_6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LVRM_0`"]
    #[inline(always)]
    pub fn is_lvrm_0(&self) -> bool {
        *self == LVRM_A::LVRM_0
    }
    #[doc = "Checks if the value of the field is `LVRM_1`"]
    #[inline(always)]
    pub fn is_lvrm_1(&self) -> bool {
        *self == LVRM_A::LVRM_1
    }
    #[doc = "Checks if the value of the field is `LVRM_2`"]
    #[inline(always)]
    pub fn is_lvrm_2(&self) -> bool {
        *self == LVRM_A::LVRM_2
    }
    #[doc = "Checks if the value of the field is `LVRM_3`"]
    #[inline(always)]
    pub fn is_lvrm_3(&self) -> bool {
        *self == LVRM_A::LVRM_3
    }
    #[doc = "Checks if the value of the field is `LVRM_4`"]
    #[inline(always)]
    pub fn is_lvrm_4(&self) -> bool {
        *self == LVRM_A::LVRM_4
    }
    #[doc = "Checks if the value of the field is `LVRM_5`"]
    #[inline(always)]
    pub fn is_lvrm_5(&self) -> bool {
        *self == LVRM_A::LVRM_5
    }
    #[doc = "Checks if the value of the field is `LVRM_6`"]
    #[inline(always)]
    pub fn is_lvrm_6(&self) -> bool {
        *self == LVRM_A::LVRM_6
    }
}
#[doc = "Write proxy for field `LVRM`"]
pub struct LVRM_W<'a> {
    w: &'a mut W,
}
impl<'a> LVRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVRM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "512 x 384"]
    #[inline(always)]
    pub fn lvrm_0(self) -> &'a mut W {
        self.variant(LVRM_A::LVRM_0)
    }
    #[doc = "448 x 336"]
    #[inline(always)]
    pub fn lvrm_1(self) -> &'a mut W {
        self.variant(LVRM_A::LVRM_1)
    }
    #[doc = "384 x 288"]
    #[inline(always)]
    pub fn lvrm_2(self) -> &'a mut W {
        self.variant(LVRM_A::LVRM_2)
    }
    #[doc = "384 x 256"]
    #[inline(always)]
    pub fn lvrm_3(self) -> &'a mut W {
        self.variant(LVRM_A::LVRM_3)
    }
    #[doc = "320 x 240"]
    #[inline(always)]
    pub fn lvrm_4(self) -> &'a mut W {
        self.variant(LVRM_A::LVRM_4)
    }
    #[doc = "288 x 216"]
    #[inline(always)]
    pub fn lvrm_5(self) -> &'a mut W {
        self.variant(LVRM_A::LVRM_5)
    }
    #[doc = "400 x 300"]
    #[inline(always)]
    pub fn lvrm_6(self) -> &'a mut W {
        self.variant(LVRM_A::LVRM_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Bayer Tile Start. Controls the Bayer pattern starting point.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BTS_A {
    #[doc = "0: GR"]
    BTS_0 = 0,
    #[doc = "1: RG"]
    BTS_1 = 1,
    #[doc = "2: BG"]
    BTS_2 = 2,
    #[doc = "3: GB"]
    BTS_3 = 3,
}
impl From<BTS_A> for u8 {
    #[inline(always)]
    fn from(variant: BTS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BTS`"]
pub type BTS_R = crate::R<u8, BTS_A>;
impl BTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTS_A {
        match self.bits {
            0 => BTS_A::BTS_0,
            1 => BTS_A::BTS_1,
            2 => BTS_A::BTS_2,
            3 => BTS_A::BTS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BTS_0`"]
    #[inline(always)]
    pub fn is_bts_0(&self) -> bool {
        *self == BTS_A::BTS_0
    }
    #[doc = "Checks if the value of the field is `BTS_1`"]
    #[inline(always)]
    pub fn is_bts_1(&self) -> bool {
        *self == BTS_A::BTS_1
    }
    #[doc = "Checks if the value of the field is `BTS_2`"]
    #[inline(always)]
    pub fn is_bts_2(&self) -> bool {
        *self == BTS_A::BTS_2
    }
    #[doc = "Checks if the value of the field is `BTS_3`"]
    #[inline(always)]
    pub fn is_bts_3(&self) -> bool {
        *self == BTS_A::BTS_3
    }
}
#[doc = "Write proxy for field `BTS`"]
pub struct BTS_W<'a> {
    w: &'a mut W,
}
impl<'a> BTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BTS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "GR"]
    #[inline(always)]
    pub fn bts_0(self) -> &'a mut W {
        self.variant(BTS_A::BTS_0)
    }
    #[doc = "RG"]
    #[inline(always)]
    pub fn bts_1(self) -> &'a mut W {
        self.variant(BTS_A::BTS_1)
    }
    #[doc = "BG"]
    #[inline(always)]
    pub fn bts_2(self) -> &'a mut W {
        self.variant(BTS_A::BTS_2)
    }
    #[doc = "GB"]
    #[inline(always)]
    pub fn bts_3(self) -> &'a mut W {
        self.variant(BTS_A::BTS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Skip Count Enable. Enables or disables the skip count feature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCE_A {
    #[doc = "0: Skip count disable"]
    SCE_0 = 0,
    #[doc = "1: Skip count enable"]
    SCE_1 = 1,
}
impl From<SCE_A> for bool {
    #[inline(always)]
    fn from(variant: SCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCE`"]
pub type SCE_R = crate::R<bool, SCE_A>;
impl SCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCE_A {
        match self.bits {
            false => SCE_A::SCE_0,
            true => SCE_A::SCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCE_0`"]
    #[inline(always)]
    pub fn is_sce_0(&self) -> bool {
        *self == SCE_A::SCE_0
    }
    #[doc = "Checks if the value of the field is `SCE_1`"]
    #[inline(always)]
    pub fn is_sce_1(&self) -> bool {
        *self == SCE_A::SCE_1
    }
}
#[doc = "Write proxy for field `SCE`"]
pub struct SCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Skip count disable"]
    #[inline(always)]
    pub fn sce_0(self) -> &'a mut W {
        self.variant(SCE_A::SCE_0)
    }
    #[doc = "Skip count enable"]
    #[inline(always)]
    pub fn sce_1(self) -> &'a mut W {
        self.variant(SCE_A::SCE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Auto Focus Spread. Selects which green pixels are used for auto-focus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AFS_A {
    #[doc = "0: Abs Diff on consecutive green pixels"]
    AFS_0 = 0,
    #[doc = "1: Abs Diff on every third green pixels"]
    AFS_1 = 1,
    #[doc = "2: Abs Diff on every four green pixels"]
    AFS_2 = 2,
}
impl From<AFS_A> for u8 {
    #[inline(always)]
    fn from(variant: AFS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AFS`"]
pub type AFS_R = crate::R<u8, AFS_A>;
impl AFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AFS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AFS_A::AFS_0),
            1 => Val(AFS_A::AFS_1),
            2 => Val(AFS_A::AFS_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AFS_0`"]
    #[inline(always)]
    pub fn is_afs_0(&self) -> bool {
        *self == AFS_A::AFS_0
    }
    #[doc = "Checks if the value of the field is `AFS_1`"]
    #[inline(always)]
    pub fn is_afs_1(&self) -> bool {
        *self == AFS_A::AFS_1
    }
    #[doc = "Checks if the value of the field is `AFS_2`"]
    #[inline(always)]
    pub fn is_afs_2(&self) -> bool {
        *self == AFS_A::AFS_2
    }
}
#[doc = "Write proxy for field `AFS`"]
pub struct AFS_W<'a> {
    w: &'a mut W,
}
impl<'a> AFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Abs Diff on consecutive green pixels"]
    #[inline(always)]
    pub fn afs_0(self) -> &'a mut W {
        self.variant(AFS_A::AFS_0)
    }
    #[doc = "Abs Diff on every third green pixels"]
    #[inline(always)]
    pub fn afs_1(self) -> &'a mut W {
        self.variant(AFS_A::AFS_1)
    }
    #[doc = "Abs Diff on every four green pixels"]
    #[inline(always)]
    pub fn afs_2(self) -> &'a mut W {
        self.variant(AFS_A::AFS_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Double Resolution Mode. Controls size of statistics grid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRM_A {
    #[doc = "0: Stats grid of 8 x 6"]
    DRM_0 = 0,
    #[doc = "1: Stats grid of 8 x 12"]
    DRM_1 = 1,
}
impl From<DRM_A> for bool {
    #[inline(always)]
    fn from(variant: DRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRM`"]
pub type DRM_R = crate::R<bool, DRM_A>;
impl DRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRM_A {
        match self.bits {
            false => DRM_A::DRM_0,
            true => DRM_A::DRM_1,
        }
    }
    #[doc = "Checks if the value of the field is `DRM_0`"]
    #[inline(always)]
    pub fn is_drm_0(&self) -> bool {
        *self == DRM_A::DRM_0
    }
    #[doc = "Checks if the value of the field is `DRM_1`"]
    #[inline(always)]
    pub fn is_drm_1(&self) -> bool {
        *self == DRM_A::DRM_1
    }
}
#[doc = "Write proxy for field `DRM`"]
pub struct DRM_W<'a> {
    w: &'a mut W,
}
impl<'a> DRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stats grid of 8 x 6"]
    #[inline(always)]
    pub fn drm_0(self) -> &'a mut W {
        self.variant(DRM_A::DRM_0)
    }
    #[doc = "Stats grid of 8 x 12"]
    #[inline(always)]
    pub fn drm_1(self) -> &'a mut W {
        self.variant(DRM_A::DRM_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Burst Type of DMA Transfer from STATFIFO. Selects the burst type of DMA transfer from STATFIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA_BURST_TYPE_SFF_A {
    #[doc = "0: INCR8"]
    DMA_BURST_TYPE_SFF_0 = 0,
    #[doc = "1: INCR4"]
    DMA_BURST_TYPE_SFF_1 = 1,
    #[doc = "3: INCR16"]
    DMA_BURST_TYPE_SFF_3 = 3,
}
impl From<DMA_BURST_TYPE_SFF_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_BURST_TYPE_SFF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMA_BURST_TYPE_SFF`"]
pub type DMA_BURST_TYPE_SFF_R = crate::R<u8, DMA_BURST_TYPE_SFF_A>;
impl DMA_BURST_TYPE_SFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMA_BURST_TYPE_SFF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMA_BURST_TYPE_SFF_A::DMA_BURST_TYPE_SFF_0),
            1 => Val(DMA_BURST_TYPE_SFF_A::DMA_BURST_TYPE_SFF_1),
            3 => Val(DMA_BURST_TYPE_SFF_A::DMA_BURST_TYPE_SFF_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_BURST_TYPE_SFF_0`"]
    #[inline(always)]
    pub fn is_dma_burst_type_sff_0(&self) -> bool {
        *self == DMA_BURST_TYPE_SFF_A::DMA_BURST_TYPE_SFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_BURST_TYPE_SFF_1`"]
    #[inline(always)]
    pub fn is_dma_burst_type_sff_1(&self) -> bool {
        *self == DMA_BURST_TYPE_SFF_A::DMA_BURST_TYPE_SFF_1
    }
    #[doc = "Checks if the value of the field is `DMA_BURST_TYPE_SFF_3`"]
    #[inline(always)]
    pub fn is_dma_burst_type_sff_3(&self) -> bool {
        *self == DMA_BURST_TYPE_SFF_A::DMA_BURST_TYPE_SFF_3
    }
}
#[doc = "Write proxy for field `DMA_BURST_TYPE_SFF`"]
pub struct DMA_BURST_TYPE_SFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURST_TYPE_SFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_BURST_TYPE_SFF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "INCR8"]
    #[inline(always)]
    pub fn dma_burst_type_sff_0(self) -> &'a mut W {
        self.variant(DMA_BURST_TYPE_SFF_A::DMA_BURST_TYPE_SFF_0)
    }
    #[doc = "INCR4"]
    #[inline(always)]
    pub fn dma_burst_type_sff_1(self) -> &'a mut W {
        self.variant(DMA_BURST_TYPE_SFF_A::DMA_BURST_TYPE_SFF_1)
    }
    #[doc = "INCR16"]
    #[inline(always)]
    pub fn dma_burst_type_sff_3(self) -> &'a mut W {
        self.variant(DMA_BURST_TYPE_SFF_A::DMA_BURST_TYPE_SFF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Burst Type of DMA Transfer from RxFIFO. Selects the burst type of DMA transfer from RxFIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA_BURST_TYPE_RFF_A {
    #[doc = "0: INCR8"]
    DMA_BURST_TYPE_RFF_0 = 0,
    #[doc = "1: INCR4"]
    DMA_BURST_TYPE_RFF_1 = 1,
    #[doc = "3: INCR16"]
    DMA_BURST_TYPE_RFF_3 = 3,
}
impl From<DMA_BURST_TYPE_RFF_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_BURST_TYPE_RFF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMA_BURST_TYPE_RFF`"]
pub type DMA_BURST_TYPE_RFF_R = crate::R<u8, DMA_BURST_TYPE_RFF_A>;
impl DMA_BURST_TYPE_RFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMA_BURST_TYPE_RFF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMA_BURST_TYPE_RFF_A::DMA_BURST_TYPE_RFF_0),
            1 => Val(DMA_BURST_TYPE_RFF_A::DMA_BURST_TYPE_RFF_1),
            3 => Val(DMA_BURST_TYPE_RFF_A::DMA_BURST_TYPE_RFF_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_BURST_TYPE_RFF_0`"]
    #[inline(always)]
    pub fn is_dma_burst_type_rff_0(&self) -> bool {
        *self == DMA_BURST_TYPE_RFF_A::DMA_BURST_TYPE_RFF_0
    }
    #[doc = "Checks if the value of the field is `DMA_BURST_TYPE_RFF_1`"]
    #[inline(always)]
    pub fn is_dma_burst_type_rff_1(&self) -> bool {
        *self == DMA_BURST_TYPE_RFF_A::DMA_BURST_TYPE_RFF_1
    }
    #[doc = "Checks if the value of the field is `DMA_BURST_TYPE_RFF_3`"]
    #[inline(always)]
    pub fn is_dma_burst_type_rff_3(&self) -> bool {
        *self == DMA_BURST_TYPE_RFF_A::DMA_BURST_TYPE_RFF_3
    }
}
#[doc = "Write proxy for field `DMA_BURST_TYPE_RFF`"]
pub struct DMA_BURST_TYPE_RFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BURST_TYPE_RFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_BURST_TYPE_RFF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "INCR8"]
    #[inline(always)]
    pub fn dma_burst_type_rff_0(self) -> &'a mut W {
        self.variant(DMA_BURST_TYPE_RFF_A::DMA_BURST_TYPE_RFF_0)
    }
    #[doc = "INCR4"]
    #[inline(always)]
    pub fn dma_burst_type_rff_1(self) -> &'a mut W {
        self.variant(DMA_BURST_TYPE_RFF_A::DMA_BURST_TYPE_RFF_1)
    }
    #[doc = "INCR16"]
    #[inline(always)]
    pub fn dma_burst_type_rff_3(self) -> &'a mut W {
        self.variant(DMA_BURST_TYPE_RFF_A::DMA_BURST_TYPE_RFF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Horizontal Skip Count"]
    #[inline(always)]
    pub fn hsc(&self) -> HSC_R {
        HSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Vertical Skip Count. Contains the number of rows to skip. SCE must be 1, otherwise VSC is ignored."]
    #[inline(always)]
    pub fn vsc(&self) -> VSC_R {
        VSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Live View Resolution Mode. Selects the grid size used for live view resolution."]
    #[inline(always)]
    pub fn lvrm(&self) -> LVRM_R {
        LVRM_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:20 - Bayer Tile Start. Controls the Bayer pattern starting point."]
    #[inline(always)]
    pub fn bts(&self) -> BTS_R {
        BTS_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Skip Count Enable. Enables or disables the skip count feature."]
    #[inline(always)]
    pub fn sce(&self) -> SCE_R {
        SCE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Auto Focus Spread. Selects which green pixels are used for auto-focus."]
    #[inline(always)]
    pub fn afs(&self) -> AFS_R {
        AFS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Double Resolution Mode. Controls size of statistics grid."]
    #[inline(always)]
    pub fn drm(&self) -> DRM_R {
        DRM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Burst Type of DMA Transfer from STATFIFO. Selects the burst type of DMA transfer from STATFIFO."]
    #[inline(always)]
    pub fn dma_burst_type_sff(&self) -> DMA_BURST_TYPE_SFF_R {
        DMA_BURST_TYPE_SFF_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Burst Type of DMA Transfer from RxFIFO. Selects the burst type of DMA transfer from RxFIFO."]
    #[inline(always)]
    pub fn dma_burst_type_rff(&self) -> DMA_BURST_TYPE_RFF_R {
        DMA_BURST_TYPE_RFF_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Horizontal Skip Count"]
    #[inline(always)]
    pub fn hsc(&mut self) -> HSC_W {
        HSC_W { w: self }
    }
    #[doc = "Bits 8:15 - Vertical Skip Count. Contains the number of rows to skip. SCE must be 1, otherwise VSC is ignored."]
    #[inline(always)]
    pub fn vsc(&mut self) -> VSC_W {
        VSC_W { w: self }
    }
    #[doc = "Bits 16:18 - Live View Resolution Mode. Selects the grid size used for live view resolution."]
    #[inline(always)]
    pub fn lvrm(&mut self) -> LVRM_W {
        LVRM_W { w: self }
    }
    #[doc = "Bits 19:20 - Bayer Tile Start. Controls the Bayer pattern starting point."]
    #[inline(always)]
    pub fn bts(&mut self) -> BTS_W {
        BTS_W { w: self }
    }
    #[doc = "Bit 23 - Skip Count Enable. Enables or disables the skip count feature."]
    #[inline(always)]
    pub fn sce(&mut self) -> SCE_W {
        SCE_W { w: self }
    }
    #[doc = "Bits 24:25 - Auto Focus Spread. Selects which green pixels are used for auto-focus."]
    #[inline(always)]
    pub fn afs(&mut self) -> AFS_W {
        AFS_W { w: self }
    }
    #[doc = "Bit 26 - Double Resolution Mode. Controls size of statistics grid."]
    #[inline(always)]
    pub fn drm(&mut self) -> DRM_W {
        DRM_W { w: self }
    }
    #[doc = "Bits 28:29 - Burst Type of DMA Transfer from STATFIFO. Selects the burst type of DMA transfer from STATFIFO."]
    #[inline(always)]
    pub fn dma_burst_type_sff(&mut self) -> DMA_BURST_TYPE_SFF_W {
        DMA_BURST_TYPE_SFF_W { w: self }
    }
    #[doc = "Bits 30:31 - Burst Type of DMA Transfer from RxFIFO. Selects the burst type of DMA transfer from RxFIFO."]
    #[inline(always)]
    pub fn dma_burst_type_rff(&mut self) -> DMA_BURST_TYPE_RFF_W {
        DMA_BURST_TYPE_RFF_W { w: self }
    }
}
