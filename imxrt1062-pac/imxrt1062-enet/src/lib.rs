#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Interrupt Event Register"]
    pub eir: EIR,
    #[doc = "0x08 - Interrupt Mask Register"]
    pub eimr: EIMR,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - Receive Descriptor Active Register"]
    pub rdar: RDAR,
    #[doc = "0x14 - Transmit Descriptor Active Register"]
    pub tdar: TDAR,
    _reserved4: [u8; 12usize],
    #[doc = "0x24 - Ethernet Control Register"]
    pub ecr: ECR,
    _reserved5: [u8; 24usize],
    #[doc = "0x40 - MII Management Frame Register"]
    pub mmfr: MMFR,
    #[doc = "0x44 - MII Speed Control Register"]
    pub mscr: MSCR,
    _reserved7: [u8; 28usize],
    #[doc = "0x64 - MIB Control Register"]
    pub mibc: MIBC,
    _reserved8: [u8; 28usize],
    #[doc = "0x84 - Receive Control Register"]
    pub rcr: RCR,
    _reserved9: [u8; 60usize],
    #[doc = "0xc4 - Transmit Control Register"]
    pub tcr: TCR,
    _reserved10: [u8; 28usize],
    #[doc = "0xe4 - Physical Address Lower Register"]
    pub palr: PALR,
    #[doc = "0xe8 - Physical Address Upper Register"]
    pub paur: PAUR,
    #[doc = "0xec - Opcode/Pause Duration Register"]
    pub opd: OPD,
    #[doc = "0xf0 - Transmit Interrupt Coalescing Register"]
    pub txic: TXIC,
    _reserved14: [u8; 12usize],
    #[doc = "0x100 - Receive Interrupt Coalescing Register"]
    pub rxic: RXIC,
    _reserved15: [u8; 20usize],
    #[doc = "0x118 - Descriptor Individual Upper Address Register"]
    pub iaur: IAUR,
    #[doc = "0x11c - Descriptor Individual Lower Address Register"]
    pub ialr: IALR,
    #[doc = "0x120 - Descriptor Group Upper Address Register"]
    pub gaur: GAUR,
    #[doc = "0x124 - Descriptor Group Lower Address Register"]
    pub galr: GALR,
    _reserved19: [u8; 28usize],
    #[doc = "0x144 - Transmit FIFO Watermark Register"]
    pub tfwr: TFWR,
    _reserved20: [u8; 56usize],
    #[doc = "0x180 - Receive Descriptor Ring Start Register"]
    pub rdsr: RDSR,
    #[doc = "0x184 - Transmit Buffer Descriptor Ring Start Register"]
    pub tdsr: TDSR,
    #[doc = "0x188 - Maximum Receive Buffer Size Register"]
    pub mrbr: MRBR,
    _reserved23: [u8; 4usize],
    #[doc = "0x190 - Receive FIFO Section Full Threshold"]
    pub rsfl: RSFL,
    #[doc = "0x194 - Receive FIFO Section Empty Threshold"]
    pub rsem: RSEM,
    #[doc = "0x198 - Receive FIFO Almost Empty Threshold"]
    pub raem: RAEM,
    #[doc = "0x19c - Receive FIFO Almost Full Threshold"]
    pub rafl: RAFL,
    #[doc = "0x1a0 - Transmit FIFO Section Empty Threshold"]
    pub tsem: TSEM,
    #[doc = "0x1a4 - Transmit FIFO Almost Empty Threshold"]
    pub taem: TAEM,
    #[doc = "0x1a8 - Transmit FIFO Almost Full Threshold"]
    pub tafl: TAFL,
    #[doc = "0x1ac - Transmit Inter-Packet Gap"]
    pub tipg: TIPG,
    #[doc = "0x1b0 - Frame Truncation Length"]
    pub ftrl: FTRL,
    _reserved32: [u8; 12usize],
    #[doc = "0x1c0 - Transmit Accelerator Function Configuration"]
    pub tacc: TACC,
    #[doc = "0x1c4 - Receive Accelerator Function Configuration"]
    pub racc: RACC,
    _reserved34: [u8; 56usize],
    #[doc = "0x200 - Reserved Statistic Register"]
    pub rmon_t_drop: RMON_T_DROP,
    #[doc = "0x204 - Tx Packet Count Statistic Register"]
    pub rmon_t_packets: RMON_T_PACKETS,
    #[doc = "0x208 - Tx Broadcast Packets Statistic Register"]
    pub rmon_t_bc_pkt: RMON_T_BC_PKT,
    #[doc = "0x20c - Tx Multicast Packets Statistic Register"]
    pub rmon_t_mc_pkt: RMON_T_MC_PKT,
    #[doc = "0x210 - Tx Packets with CRC/Align Error Statistic Register"]
    pub rmon_t_crc_align: RMON_T_CRC_ALIGN,
    #[doc = "0x214 - Tx Packets Less Than Bytes and Good CRC Statistic Register"]
    pub rmon_t_undersize: RMON_T_UNDERSIZE,
    #[doc = "0x218 - Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
    pub rmon_t_oversize: RMON_T_OVERSIZE,
    #[doc = "0x21c - Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
    pub rmon_t_frag: RMON_T_FRAG,
    #[doc = "0x220 - Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
    pub rmon_t_jab: RMON_T_JAB,
    #[doc = "0x224 - Tx Collision Count Statistic Register"]
    pub rmon_t_col: RMON_T_COL,
    #[doc = "0x228 - Tx 64-Byte Packets Statistic Register"]
    pub rmon_t_p64: RMON_T_P64,
    #[doc = "0x22c - Tx 65- to 127-byte Packets Statistic Register"]
    pub rmon_t_p65to127: RMON_T_P65TO127,
    #[doc = "0x230 - Tx 128- to 255-byte Packets Statistic Register"]
    pub rmon_t_p128to255: RMON_T_P128TO255,
    #[doc = "0x234 - Tx 256- to 511-byte Packets Statistic Register"]
    pub rmon_t_p256to511: RMON_T_P256TO511,
    #[doc = "0x238 - Tx 512- to 1023-byte Packets Statistic Register"]
    pub rmon_t_p512to1023: RMON_T_P512TO1023,
    #[doc = "0x23c - Tx 1024- to 2047-byte Packets Statistic Register"]
    pub rmon_t_p1024to2047: RMON_T_P1024TO2047,
    #[doc = "0x240 - Tx Packets Greater Than 2048 Bytes Statistic Register"]
    pub rmon_t_p_gte2048: RMON_T_P_GTE2048,
    #[doc = "0x244 - Tx Octets Statistic Register"]
    pub rmon_t_octets: RMON_T_OCTETS,
    #[doc = "0x248 - Reserved Statistic Register"]
    pub ieee_t_drop: IEEE_T_DROP,
    #[doc = "0x24c - Frames Transmitted OK Statistic Register"]
    pub ieee_t_frame_ok: IEEE_T_FRAME_OK,
    #[doc = "0x250 - Frames Transmitted with Single Collision Statistic Register"]
    pub ieee_t_1col: IEEE_T_1COL,
    #[doc = "0x254 - Frames Transmitted with Multiple Collisions Statistic Register"]
    pub ieee_t_mcol: IEEE_T_MCOL,
    #[doc = "0x258 - Frames Transmitted after Deferral Delay Statistic Register"]
    pub ieee_t_def: IEEE_T_DEF,
    #[doc = "0x25c - Frames Transmitted with Late Collision Statistic Register"]
    pub ieee_t_lcol: IEEE_T_LCOL,
    #[doc = "0x260 - Frames Transmitted with Excessive Collisions Statistic Register"]
    pub ieee_t_excol: IEEE_T_EXCOL,
    #[doc = "0x264 - Frames Transmitted with Tx FIFO Underrun Statistic Register"]
    pub ieee_t_macerr: IEEE_T_MACERR,
    #[doc = "0x268 - Frames Transmitted with Carrier Sense Error Statistic Register"]
    pub ieee_t_cserr: IEEE_T_CSERR,
    #[doc = "0x26c - Reserved Statistic Register"]
    pub ieee_t_sqe: IEEE_T_SQE,
    #[doc = "0x270 - Flow Control Pause Frames Transmitted Statistic Register"]
    pub ieee_t_fdxfc: IEEE_T_FDXFC,
    #[doc = "0x274 - Octet Count for Frames Transmitted w/o Error Statistic Register"]
    pub ieee_t_octets_ok: IEEE_T_OCTETS_OK,
    _reserved64: [u8; 12usize],
    #[doc = "0x284 - Rx Packet Count Statistic Register"]
    pub rmon_r_packets: RMON_R_PACKETS,
    #[doc = "0x288 - Rx Broadcast Packets Statistic Register"]
    pub rmon_r_bc_pkt: RMON_R_BC_PKT,
    #[doc = "0x28c - Rx Multicast Packets Statistic Register"]
    pub rmon_r_mc_pkt: RMON_R_MC_PKT,
    #[doc = "0x290 - Rx Packets with CRC/Align Error Statistic Register"]
    pub rmon_r_crc_align: RMON_R_CRC_ALIGN,
    #[doc = "0x294 - Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
    pub rmon_r_undersize: RMON_R_UNDERSIZE,
    #[doc = "0x298 - Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
    pub rmon_r_oversize: RMON_R_OVERSIZE,
    #[doc = "0x29c - Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
    pub rmon_r_frag: RMON_R_FRAG,
    #[doc = "0x2a0 - Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
    pub rmon_r_jab: RMON_R_JAB,
    #[doc = "0x2a4 - Reserved Statistic Register"]
    pub rmon_r_resvd_0: RMON_R_RESVD_0,
    #[doc = "0x2a8 - Rx 64-Byte Packets Statistic Register"]
    pub rmon_r_p64: RMON_R_P64,
    #[doc = "0x2ac - Rx 65- to 127-Byte Packets Statistic Register"]
    pub rmon_r_p65to127: RMON_R_P65TO127,
    #[doc = "0x2b0 - Rx 128- to 255-Byte Packets Statistic Register"]
    pub rmon_r_p128to255: RMON_R_P128TO255,
    #[doc = "0x2b4 - Rx 256- to 511-Byte Packets Statistic Register"]
    pub rmon_r_p256to511: RMON_R_P256TO511,
    #[doc = "0x2b8 - Rx 512- to 1023-Byte Packets Statistic Register"]
    pub rmon_r_p512to1023: RMON_R_P512TO1023,
    #[doc = "0x2bc - Rx 1024- to 2047-Byte Packets Statistic Register"]
    pub rmon_r_p1024to2047: RMON_R_P1024TO2047,
    #[doc = "0x2c0 - Rx Packets Greater than 2048 Bytes Statistic Register"]
    pub rmon_r_p_gte2048: RMON_R_P_GTE2048,
    #[doc = "0x2c4 - Rx Octets Statistic Register"]
    pub rmon_r_octets: RMON_R_OCTETS,
    #[doc = "0x2c8 - Frames not Counted Correctly Statistic Register"]
    pub ieee_r_drop: IEEE_R_DROP,
    #[doc = "0x2cc - Frames Received OK Statistic Register"]
    pub ieee_r_frame_ok: IEEE_R_FRAME_OK,
    #[doc = "0x2d0 - Frames Received with CRC Error Statistic Register"]
    pub ieee_r_crc: IEEE_R_CRC,
    #[doc = "0x2d4 - Frames Received with Alignment Error Statistic Register"]
    pub ieee_r_align: IEEE_R_ALIGN,
    #[doc = "0x2d8 - Receive FIFO Overflow Count Statistic Register"]
    pub ieee_r_macerr: IEEE_R_MACERR,
    #[doc = "0x2dc - Flow Control Pause Frames Received Statistic Register"]
    pub ieee_r_fdxfc: IEEE_R_FDXFC,
    #[doc = "0x2e0 - Octet Count for Frames Received without Error Statistic Register"]
    pub ieee_r_octets_ok: IEEE_R_OCTETS_OK,
    _reserved88: [u8; 284usize],
    #[doc = "0x400 - Adjustable Timer Control Register"]
    pub atcr: ATCR,
    #[doc = "0x404 - Timer Value Register"]
    pub atvr: ATVR,
    #[doc = "0x408 - Timer Offset Register"]
    pub atoff: ATOFF,
    #[doc = "0x40c - Timer Period Register"]
    pub atper: ATPER,
    #[doc = "0x410 - Timer Correction Register"]
    pub atcor: ATCOR,
    #[doc = "0x414 - Time-Stamping Clock Period Register"]
    pub atinc: ATINC,
    #[doc = "0x418 - Timestamp of Last Transmitted Frame"]
    pub atstmp: ATSTMP,
    _reserved95: [u8; 488usize],
    #[doc = "0x604 - Timer Global Status Register"]
    pub tgsr: TGSR,
    #[doc = "0x608 - Timer Control Status Register"]
    pub tcsr0: TCSR,
    #[doc = "0x60c - Timer Compare Capture Register"]
    pub tccr0: TCCR,
    #[doc = "0x610 - Timer Control Status Register"]
    pub tcsr1: TCSR,
    #[doc = "0x614 - Timer Compare Capture Register"]
    pub tccr1: TCCR,
    #[doc = "0x618 - Timer Control Status Register"]
    pub tcsr2: TCSR,
    #[doc = "0x61c - Timer Compare Capture Register"]
    pub tccr2: TCCR,
    #[doc = "0x620 - Timer Control Status Register"]
    pub tcsr3: TCSR,
    #[doc = "0x624 - Timer Compare Capture Register"]
    pub tccr3: TCCR,
}
#[doc = "Interrupt Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eir](eir) module"]
pub type EIR = crate::Reg<u32, _EIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EIR;
#[doc = "`read()` method returns [eir::R](eir::R) reader structure"]
impl crate::Readable for EIR {}
#[doc = "`write(|w| ..)` method takes [eir::W](eir::W) writer structure"]
impl crate::Writable for EIR {}
#[doc = "Interrupt Event Register"]
pub mod eir;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eimr](eimr) module"]
pub type EIMR = crate::Reg<u32, _EIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EIMR;
#[doc = "`read()` method returns [eimr::R](eimr::R) reader structure"]
impl crate::Readable for EIMR {}
#[doc = "`write(|w| ..)` method takes [eimr::W](eimr::W) writer structure"]
impl crate::Writable for EIMR {}
#[doc = "Interrupt Mask Register"]
pub mod eimr;
#[doc = "Receive Descriptor Active Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdar](rdar) module"]
pub type RDAR = crate::Reg<u32, _RDAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDAR;
#[doc = "`read()` method returns [rdar::R](rdar::R) reader structure"]
impl crate::Readable for RDAR {}
#[doc = "`write(|w| ..)` method takes [rdar::W](rdar::W) writer structure"]
impl crate::Writable for RDAR {}
#[doc = "Receive Descriptor Active Register"]
pub mod rdar;
#[doc = "Transmit Descriptor Active Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdar](tdar) module"]
pub type TDAR = crate::Reg<u32, _TDAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDAR;
#[doc = "`read()` method returns [tdar::R](tdar::R) reader structure"]
impl crate::Readable for TDAR {}
#[doc = "`write(|w| ..)` method takes [tdar::W](tdar::W) writer structure"]
impl crate::Writable for TDAR {}
#[doc = "Transmit Descriptor Active Register"]
pub mod tdar;
#[doc = "Ethernet Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](ecr) module"]
pub type ECR = crate::Reg<u32, _ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECR;
#[doc = "`read()` method returns [ecr::R](ecr::R) reader structure"]
impl crate::Readable for ECR {}
#[doc = "`write(|w| ..)` method takes [ecr::W](ecr::W) writer structure"]
impl crate::Writable for ECR {}
#[doc = "Ethernet Control Register"]
pub mod ecr;
#[doc = "MII Management Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmfr](mmfr) module"]
pub type MMFR = crate::Reg<u32, _MMFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMFR;
#[doc = "`read()` method returns [mmfr::R](mmfr::R) reader structure"]
impl crate::Readable for MMFR {}
#[doc = "`write(|w| ..)` method takes [mmfr::W](mmfr::W) writer structure"]
impl crate::Writable for MMFR {}
#[doc = "MII Management Frame Register"]
pub mod mmfr;
#[doc = "MII Speed Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mscr](mscr) module"]
pub type MSCR = crate::Reg<u32, _MSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSCR;
#[doc = "`read()` method returns [mscr::R](mscr::R) reader structure"]
impl crate::Readable for MSCR {}
#[doc = "`write(|w| ..)` method takes [mscr::W](mscr::W) writer structure"]
impl crate::Writable for MSCR {}
#[doc = "MII Speed Control Register"]
pub mod mscr;
#[doc = "MIB Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mibc](mibc) module"]
pub type MIBC = crate::Reg<u32, _MIBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIBC;
#[doc = "`read()` method returns [mibc::R](mibc::R) reader structure"]
impl crate::Readable for MIBC {}
#[doc = "`write(|w| ..)` method takes [mibc::W](mibc::W) writer structure"]
impl crate::Writable for MIBC {}
#[doc = "MIB Control Register"]
pub mod mibc;
#[doc = "Receive Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr](rcr) module"]
pub type RCR = crate::Reg<u32, _RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR;
#[doc = "`read()` method returns [rcr::R](rcr::R) reader structure"]
impl crate::Readable for RCR {}
#[doc = "`write(|w| ..)` method takes [rcr::W](rcr::W) writer structure"]
impl crate::Writable for RCR {}
#[doc = "Receive Control Register"]
pub mod rcr;
#[doc = "Transmit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "Transmit Control Register"]
pub mod tcr;
#[doc = "Physical Address Lower Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [palr](palr) module"]
pub type PALR = crate::Reg<u32, _PALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PALR;
#[doc = "`read()` method returns [palr::R](palr::R) reader structure"]
impl crate::Readable for PALR {}
#[doc = "`write(|w| ..)` method takes [palr::W](palr::W) writer structure"]
impl crate::Writable for PALR {}
#[doc = "Physical Address Lower Register"]
pub mod palr;
#[doc = "Physical Address Upper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paur](paur) module"]
pub type PAUR = crate::Reg<u32, _PAUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAUR;
#[doc = "`read()` method returns [paur::R](paur::R) reader structure"]
impl crate::Readable for PAUR {}
#[doc = "`write(|w| ..)` method takes [paur::W](paur::W) writer structure"]
impl crate::Writable for PAUR {}
#[doc = "Physical Address Upper Register"]
pub mod paur;
#[doc = "Opcode/Pause Duration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opd](opd) module"]
pub type OPD = crate::Reg<u32, _OPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPD;
#[doc = "`read()` method returns [opd::R](opd::R) reader structure"]
impl crate::Readable for OPD {}
#[doc = "`write(|w| ..)` method takes [opd::W](opd::W) writer structure"]
impl crate::Writable for OPD {}
#[doc = "Opcode/Pause Duration Register"]
pub mod opd;
#[doc = "Transmit Interrupt Coalescing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txic](txic) module"]
pub type TXIC = crate::Reg<u32, _TXIC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXIC;
#[doc = "`read()` method returns [txic::R](txic::R) reader structure"]
impl crate::Readable for TXIC {}
#[doc = "`write(|w| ..)` method takes [txic::W](txic::W) writer structure"]
impl crate::Writable for TXIC {}
#[doc = "Transmit Interrupt Coalescing Register"]
pub mod txic;
#[doc = "Receive Interrupt Coalescing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxic](rxic) module"]
pub type RXIC = crate::Reg<u32, _RXIC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIC;
#[doc = "`read()` method returns [rxic::R](rxic::R) reader structure"]
impl crate::Readable for RXIC {}
#[doc = "`write(|w| ..)` method takes [rxic::W](rxic::W) writer structure"]
impl crate::Writable for RXIC {}
#[doc = "Receive Interrupt Coalescing Register"]
pub mod rxic;
#[doc = "Descriptor Individual Upper Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iaur](iaur) module"]
pub type IAUR = crate::Reg<u32, _IAUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IAUR;
#[doc = "`read()` method returns [iaur::R](iaur::R) reader structure"]
impl crate::Readable for IAUR {}
#[doc = "`write(|w| ..)` method takes [iaur::W](iaur::W) writer structure"]
impl crate::Writable for IAUR {}
#[doc = "Descriptor Individual Upper Address Register"]
pub mod iaur;
#[doc = "Descriptor Individual Lower Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ialr](ialr) module"]
pub type IALR = crate::Reg<u32, _IALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IALR;
#[doc = "`read()` method returns [ialr::R](ialr::R) reader structure"]
impl crate::Readable for IALR {}
#[doc = "`write(|w| ..)` method takes [ialr::W](ialr::W) writer structure"]
impl crate::Writable for IALR {}
#[doc = "Descriptor Individual Lower Address Register"]
pub mod ialr;
#[doc = "Descriptor Group Upper Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gaur](gaur) module"]
pub type GAUR = crate::Reg<u32, _GAUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAUR;
#[doc = "`read()` method returns [gaur::R](gaur::R) reader structure"]
impl crate::Readable for GAUR {}
#[doc = "`write(|w| ..)` method takes [gaur::W](gaur::W) writer structure"]
impl crate::Writable for GAUR {}
#[doc = "Descriptor Group Upper Address Register"]
pub mod gaur;
#[doc = "Descriptor Group Lower Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [galr](galr) module"]
pub type GALR = crate::Reg<u32, _GALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GALR;
#[doc = "`read()` method returns [galr::R](galr::R) reader structure"]
impl crate::Readable for GALR {}
#[doc = "`write(|w| ..)` method takes [galr::W](galr::W) writer structure"]
impl crate::Writable for GALR {}
#[doc = "Descriptor Group Lower Address Register"]
pub mod galr;
#[doc = "Transmit FIFO Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfwr](tfwr) module"]
pub type TFWR = crate::Reg<u32, _TFWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFWR;
#[doc = "`read()` method returns [tfwr::R](tfwr::R) reader structure"]
impl crate::Readable for TFWR {}
#[doc = "`write(|w| ..)` method takes [tfwr::W](tfwr::W) writer structure"]
impl crate::Writable for TFWR {}
#[doc = "Transmit FIFO Watermark Register"]
pub mod tfwr;
#[doc = "Receive Descriptor Ring Start Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdsr](rdsr) module"]
pub type RDSR = crate::Reg<u32, _RDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDSR;
#[doc = "`read()` method returns [rdsr::R](rdsr::R) reader structure"]
impl crate::Readable for RDSR {}
#[doc = "`write(|w| ..)` method takes [rdsr::W](rdsr::W) writer structure"]
impl crate::Writable for RDSR {}
#[doc = "Receive Descriptor Ring Start Register"]
pub mod rdsr;
#[doc = "Transmit Buffer Descriptor Ring Start Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdsr](tdsr) module"]
pub type TDSR = crate::Reg<u32, _TDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDSR;
#[doc = "`read()` method returns [tdsr::R](tdsr::R) reader structure"]
impl crate::Readable for TDSR {}
#[doc = "`write(|w| ..)` method takes [tdsr::W](tdsr::W) writer structure"]
impl crate::Writable for TDSR {}
#[doc = "Transmit Buffer Descriptor Ring Start Register"]
pub mod tdsr;
#[doc = "Maximum Receive Buffer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrbr](mrbr) module"]
pub type MRBR = crate::Reg<u32, _MRBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRBR;
#[doc = "`read()` method returns [mrbr::R](mrbr::R) reader structure"]
impl crate::Readable for MRBR {}
#[doc = "`write(|w| ..)` method takes [mrbr::W](mrbr::W) writer structure"]
impl crate::Writable for MRBR {}
#[doc = "Maximum Receive Buffer Size Register"]
pub mod mrbr;
#[doc = "Receive FIFO Section Full Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsfl](rsfl) module"]
pub type RSFL = crate::Reg<u32, _RSFL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSFL;
#[doc = "`read()` method returns [rsfl::R](rsfl::R) reader structure"]
impl crate::Readable for RSFL {}
#[doc = "`write(|w| ..)` method takes [rsfl::W](rsfl::W) writer structure"]
impl crate::Writable for RSFL {}
#[doc = "Receive FIFO Section Full Threshold"]
pub mod rsfl;
#[doc = "Receive FIFO Section Empty Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsem](rsem) module"]
pub type RSEM = crate::Reg<u32, _RSEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSEM;
#[doc = "`read()` method returns [rsem::R](rsem::R) reader structure"]
impl crate::Readable for RSEM {}
#[doc = "`write(|w| ..)` method takes [rsem::W](rsem::W) writer structure"]
impl crate::Writable for RSEM {}
#[doc = "Receive FIFO Section Empty Threshold"]
pub mod rsem;
#[doc = "Receive FIFO Almost Empty Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [raem](raem) module"]
pub type RAEM = crate::Reg<u32, _RAEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAEM;
#[doc = "`read()` method returns [raem::R](raem::R) reader structure"]
impl crate::Readable for RAEM {}
#[doc = "`write(|w| ..)` method takes [raem::W](raem::W) writer structure"]
impl crate::Writable for RAEM {}
#[doc = "Receive FIFO Almost Empty Threshold"]
pub mod raem;
#[doc = "Receive FIFO Almost Full Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rafl](rafl) module"]
pub type RAFL = crate::Reg<u32, _RAFL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAFL;
#[doc = "`read()` method returns [rafl::R](rafl::R) reader structure"]
impl crate::Readable for RAFL {}
#[doc = "`write(|w| ..)` method takes [rafl::W](rafl::W) writer structure"]
impl crate::Writable for RAFL {}
#[doc = "Receive FIFO Almost Full Threshold"]
pub mod rafl;
#[doc = "Transmit FIFO Section Empty Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsem](tsem) module"]
pub type TSEM = crate::Reg<u32, _TSEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSEM;
#[doc = "`read()` method returns [tsem::R](tsem::R) reader structure"]
impl crate::Readable for TSEM {}
#[doc = "`write(|w| ..)` method takes [tsem::W](tsem::W) writer structure"]
impl crate::Writable for TSEM {}
#[doc = "Transmit FIFO Section Empty Threshold"]
pub mod tsem;
#[doc = "Transmit FIFO Almost Empty Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taem](taem) module"]
pub type TAEM = crate::Reg<u32, _TAEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAEM;
#[doc = "`read()` method returns [taem::R](taem::R) reader structure"]
impl crate::Readable for TAEM {}
#[doc = "`write(|w| ..)` method takes [taem::W](taem::W) writer structure"]
impl crate::Writable for TAEM {}
#[doc = "Transmit FIFO Almost Empty Threshold"]
pub mod taem;
#[doc = "Transmit FIFO Almost Full Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tafl](tafl) module"]
pub type TAFL = crate::Reg<u32, _TAFL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAFL;
#[doc = "`read()` method returns [tafl::R](tafl::R) reader structure"]
impl crate::Readable for TAFL {}
#[doc = "`write(|w| ..)` method takes [tafl::W](tafl::W) writer structure"]
impl crate::Writable for TAFL {}
#[doc = "Transmit FIFO Almost Full Threshold"]
pub mod tafl;
#[doc = "Transmit Inter-Packet Gap\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tipg](tipg) module"]
pub type TIPG = crate::Reg<u32, _TIPG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIPG;
#[doc = "`read()` method returns [tipg::R](tipg::R) reader structure"]
impl crate::Readable for TIPG {}
#[doc = "`write(|w| ..)` method takes [tipg::W](tipg::W) writer structure"]
impl crate::Writable for TIPG {}
#[doc = "Transmit Inter-Packet Gap"]
pub mod tipg;
#[doc = "Frame Truncation Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftrl](ftrl) module"]
pub type FTRL = crate::Reg<u32, _FTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTRL;
#[doc = "`read()` method returns [ftrl::R](ftrl::R) reader structure"]
impl crate::Readable for FTRL {}
#[doc = "`write(|w| ..)` method takes [ftrl::W](ftrl::W) writer structure"]
impl crate::Writable for FTRL {}
#[doc = "Frame Truncation Length"]
pub mod ftrl;
#[doc = "Transmit Accelerator Function Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tacc](tacc) module"]
pub type TACC = crate::Reg<u32, _TACC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TACC;
#[doc = "`read()` method returns [tacc::R](tacc::R) reader structure"]
impl crate::Readable for TACC {}
#[doc = "`write(|w| ..)` method takes [tacc::W](tacc::W) writer structure"]
impl crate::Writable for TACC {}
#[doc = "Transmit Accelerator Function Configuration"]
pub mod tacc;
#[doc = "Receive Accelerator Function Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [racc](racc) module"]
pub type RACC = crate::Reg<u32, _RACC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RACC;
#[doc = "`read()` method returns [racc::R](racc::R) reader structure"]
impl crate::Readable for RACC {}
#[doc = "`write(|w| ..)` method takes [racc::W](racc::W) writer structure"]
impl crate::Writable for RACC {}
#[doc = "Receive Accelerator Function Configuration"]
pub mod racc;
#[doc = "Reserved Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_drop](rmon_t_drop) module"]
pub type RMON_T_DROP = crate::Reg<u32, _RMON_T_DROP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_DROP;
#[doc = "`read()` method returns [rmon_t_drop::R](rmon_t_drop::R) reader structure"]
impl crate::Readable for RMON_T_DROP {}
#[doc = "Reserved Statistic Register"]
pub mod rmon_t_drop;
#[doc = "Tx Packet Count Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_packets](rmon_t_packets) module"]
pub type RMON_T_PACKETS = crate::Reg<u32, _RMON_T_PACKETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_PACKETS;
#[doc = "`read()` method returns [rmon_t_packets::R](rmon_t_packets::R) reader structure"]
impl crate::Readable for RMON_T_PACKETS {}
#[doc = "Tx Packet Count Statistic Register"]
pub mod rmon_t_packets;
#[doc = "Tx Broadcast Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_bc_pkt](rmon_t_bc_pkt) module"]
pub type RMON_T_BC_PKT = crate::Reg<u32, _RMON_T_BC_PKT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_BC_PKT;
#[doc = "`read()` method returns [rmon_t_bc_pkt::R](rmon_t_bc_pkt::R) reader structure"]
impl crate::Readable for RMON_T_BC_PKT {}
#[doc = "Tx Broadcast Packets Statistic Register"]
pub mod rmon_t_bc_pkt;
#[doc = "Tx Multicast Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_mc_pkt](rmon_t_mc_pkt) module"]
pub type RMON_T_MC_PKT = crate::Reg<u32, _RMON_T_MC_PKT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_MC_PKT;
#[doc = "`read()` method returns [rmon_t_mc_pkt::R](rmon_t_mc_pkt::R) reader structure"]
impl crate::Readable for RMON_T_MC_PKT {}
#[doc = "Tx Multicast Packets Statistic Register"]
pub mod rmon_t_mc_pkt;
#[doc = "Tx Packets with CRC/Align Error Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_crc_align](rmon_t_crc_align) module"]
pub type RMON_T_CRC_ALIGN = crate::Reg<u32, _RMON_T_CRC_ALIGN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_CRC_ALIGN;
#[doc = "`read()` method returns [rmon_t_crc_align::R](rmon_t_crc_align::R) reader structure"]
impl crate::Readable for RMON_T_CRC_ALIGN {}
#[doc = "Tx Packets with CRC/Align Error Statistic Register"]
pub mod rmon_t_crc_align;
#[doc = "Tx Packets Less Than Bytes and Good CRC Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_undersize](rmon_t_undersize) module"]
pub type RMON_T_UNDERSIZE = crate::Reg<u32, _RMON_T_UNDERSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_UNDERSIZE;
#[doc = "`read()` method returns [rmon_t_undersize::R](rmon_t_undersize::R) reader structure"]
impl crate::Readable for RMON_T_UNDERSIZE {}
#[doc = "Tx Packets Less Than Bytes and Good CRC Statistic Register"]
pub mod rmon_t_undersize;
#[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_oversize](rmon_t_oversize) module"]
pub type RMON_T_OVERSIZE = crate::Reg<u32, _RMON_T_OVERSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_OVERSIZE;
#[doc = "`read()` method returns [rmon_t_oversize::R](rmon_t_oversize::R) reader structure"]
impl crate::Readable for RMON_T_OVERSIZE {}
#[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
pub mod rmon_t_oversize;
#[doc = "Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_frag](rmon_t_frag) module"]
pub type RMON_T_FRAG = crate::Reg<u32, _RMON_T_FRAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_FRAG;
#[doc = "`read()` method returns [rmon_t_frag::R](rmon_t_frag::R) reader structure"]
impl crate::Readable for RMON_T_FRAG {}
#[doc = "Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
pub mod rmon_t_frag;
#[doc = "Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_jab](rmon_t_jab) module"]
pub type RMON_T_JAB = crate::Reg<u32, _RMON_T_JAB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_JAB;
#[doc = "`read()` method returns [rmon_t_jab::R](rmon_t_jab::R) reader structure"]
impl crate::Readable for RMON_T_JAB {}
#[doc = "Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
pub mod rmon_t_jab;
#[doc = "Tx Collision Count Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_col](rmon_t_col) module"]
pub type RMON_T_COL = crate::Reg<u32, _RMON_T_COL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_COL;
#[doc = "`read()` method returns [rmon_t_col::R](rmon_t_col::R) reader structure"]
impl crate::Readable for RMON_T_COL {}
#[doc = "Tx Collision Count Statistic Register"]
pub mod rmon_t_col;
#[doc = "Tx 64-Byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_p64](rmon_t_p64) module"]
pub type RMON_T_P64 = crate::Reg<u32, _RMON_T_P64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_P64;
#[doc = "`read()` method returns [rmon_t_p64::R](rmon_t_p64::R) reader structure"]
impl crate::Readable for RMON_T_P64 {}
#[doc = "Tx 64-Byte Packets Statistic Register"]
pub mod rmon_t_p64;
#[doc = "Tx 65- to 127-byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_p65to127](rmon_t_p65to127) module"]
pub type RMON_T_P65TO127 = crate::Reg<u32, _RMON_T_P65TO127>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_P65TO127;
#[doc = "`read()` method returns [rmon_t_p65to127::R](rmon_t_p65to127::R) reader structure"]
impl crate::Readable for RMON_T_P65TO127 {}
#[doc = "Tx 65- to 127-byte Packets Statistic Register"]
pub mod rmon_t_p65to127;
#[doc = "Tx 128- to 255-byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_p128to255](rmon_t_p128to255) module"]
pub type RMON_T_P128TO255 = crate::Reg<u32, _RMON_T_P128TO255>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_P128TO255;
#[doc = "`read()` method returns [rmon_t_p128to255::R](rmon_t_p128to255::R) reader structure"]
impl crate::Readable for RMON_T_P128TO255 {}
#[doc = "Tx 128- to 255-byte Packets Statistic Register"]
pub mod rmon_t_p128to255;
#[doc = "Tx 256- to 511-byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_p256to511](rmon_t_p256to511) module"]
pub type RMON_T_P256TO511 = crate::Reg<u32, _RMON_T_P256TO511>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_P256TO511;
#[doc = "`read()` method returns [rmon_t_p256to511::R](rmon_t_p256to511::R) reader structure"]
impl crate::Readable for RMON_T_P256TO511 {}
#[doc = "Tx 256- to 511-byte Packets Statistic Register"]
pub mod rmon_t_p256to511;
#[doc = "Tx 512- to 1023-byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_p512to1023](rmon_t_p512to1023) module"]
pub type RMON_T_P512TO1023 = crate::Reg<u32, _RMON_T_P512TO1023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_P512TO1023;
#[doc = "`read()` method returns [rmon_t_p512to1023::R](rmon_t_p512to1023::R) reader structure"]
impl crate::Readable for RMON_T_P512TO1023 {}
#[doc = "Tx 512- to 1023-byte Packets Statistic Register"]
pub mod rmon_t_p512to1023;
#[doc = "Tx 1024- to 2047-byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_p1024to2047](rmon_t_p1024to2047) module"]
pub type RMON_T_P1024TO2047 = crate::Reg<u32, _RMON_T_P1024TO2047>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_P1024TO2047;
#[doc = "`read()` method returns [rmon_t_p1024to2047::R](rmon_t_p1024to2047::R) reader structure"]
impl crate::Readable for RMON_T_P1024TO2047 {}
#[doc = "Tx 1024- to 2047-byte Packets Statistic Register"]
pub mod rmon_t_p1024to2047;
#[doc = "Tx Packets Greater Than 2048 Bytes Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_p_gte2048](rmon_t_p_gte2048) module"]
pub type RMON_T_P_GTE2048 = crate::Reg<u32, _RMON_T_P_GTE2048>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_P_GTE2048;
#[doc = "`read()` method returns [rmon_t_p_gte2048::R](rmon_t_p_gte2048::R) reader structure"]
impl crate::Readable for RMON_T_P_GTE2048 {}
#[doc = "Tx Packets Greater Than 2048 Bytes Statistic Register"]
pub mod rmon_t_p_gte2048;
#[doc = "Tx Octets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_octets](rmon_t_octets) module"]
pub type RMON_T_OCTETS = crate::Reg<u32, _RMON_T_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_T_OCTETS;
#[doc = "`read()` method returns [rmon_t_octets::R](rmon_t_octets::R) reader structure"]
impl crate::Readable for RMON_T_OCTETS {}
#[doc = "Tx Octets Statistic Register"]
pub mod rmon_t_octets;
#[doc = "Reserved Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_drop](ieee_t_drop) module"]
pub type IEEE_T_DROP = crate::Reg<u32, _IEEE_T_DROP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_T_DROP;
#[doc = "`read()` method returns [ieee_t_drop::R](ieee_t_drop::R) reader structure"]
impl crate::Readable for IEEE_T_DROP {}
#[doc = "Reserved Statistic Register"]
pub mod ieee_t_drop;
#[doc = "Frames Transmitted OK Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_frame_ok](ieee_t_frame_ok) module"]
pub type IEEE_T_FRAME_OK = crate::Reg<u32, _IEEE_T_FRAME_OK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_T_FRAME_OK;
#[doc = "`read()` method returns [ieee_t_frame_ok::R](ieee_t_frame_ok::R) reader structure"]
impl crate::Readable for IEEE_T_FRAME_OK {}
#[doc = "Frames Transmitted OK Statistic Register"]
pub mod ieee_t_frame_ok;
#[doc = "Frames Transmitted with Single Collision Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_1col](ieee_t_1col) module"]
pub type IEEE_T_1COL = crate::Reg<u32, _IEEE_T_1COL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_T_1COL;
#[doc = "`read()` method returns [ieee_t_1col::R](ieee_t_1col::R) reader structure"]
impl crate::Readable for IEEE_T_1COL {}
#[doc = "Frames Transmitted with Single Collision Statistic Register"]
pub mod ieee_t_1col;
#[doc = "Frames Transmitted with Multiple Collisions Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_mcol](ieee_t_mcol) module"]
pub type IEEE_T_MCOL = crate::Reg<u32, _IEEE_T_MCOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_T_MCOL;
#[doc = "`read()` method returns [ieee_t_mcol::R](ieee_t_mcol::R) reader structure"]
impl crate::Readable for IEEE_T_MCOL {}
#[doc = "Frames Transmitted with Multiple Collisions Statistic Register"]
pub mod ieee_t_mcol;
#[doc = "Frames Transmitted after Deferral Delay Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_def](ieee_t_def) module"]
pub type IEEE_T_DEF = crate::Reg<u32, _IEEE_T_DEF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_T_DEF;
#[doc = "`read()` method returns [ieee_t_def::R](ieee_t_def::R) reader structure"]
impl crate::Readable for IEEE_T_DEF {}
#[doc = "Frames Transmitted after Deferral Delay Statistic Register"]
pub mod ieee_t_def;
#[doc = "Frames Transmitted with Late Collision Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_lcol](ieee_t_lcol) module"]
pub type IEEE_T_LCOL = crate::Reg<u32, _IEEE_T_LCOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_T_LCOL;
#[doc = "`read()` method returns [ieee_t_lcol::R](ieee_t_lcol::R) reader structure"]
impl crate::Readable for IEEE_T_LCOL {}
#[doc = "Frames Transmitted with Late Collision Statistic Register"]
pub mod ieee_t_lcol;
#[doc = "Frames Transmitted with Excessive Collisions Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_excol](ieee_t_excol) module"]
pub type IEEE_T_EXCOL = crate::Reg<u32, _IEEE_T_EXCOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_T_EXCOL;
#[doc = "`read()` method returns [ieee_t_excol::R](ieee_t_excol::R) reader structure"]
impl crate::Readable for IEEE_T_EXCOL {}
#[doc = "Frames Transmitted with Excessive Collisions Statistic Register"]
pub mod ieee_t_excol;
#[doc = "Frames Transmitted with Tx FIFO Underrun Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_macerr](ieee_t_macerr) module"]
pub type IEEE_T_MACERR = crate::Reg<u32, _IEEE_T_MACERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_T_MACERR;
#[doc = "`read()` method returns [ieee_t_macerr::R](ieee_t_macerr::R) reader structure"]
impl crate::Readable for IEEE_T_MACERR {}
#[doc = "Frames Transmitted with Tx FIFO Underrun Statistic Register"]
pub mod ieee_t_macerr;
#[doc = "Frames Transmitted with Carrier Sense Error Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_cserr](ieee_t_cserr) module"]
pub type IEEE_T_CSERR = crate::Reg<u32, _IEEE_T_CSERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_T_CSERR;
#[doc = "`read()` method returns [ieee_t_cserr::R](ieee_t_cserr::R) reader structure"]
impl crate::Readable for IEEE_T_CSERR {}
#[doc = "Frames Transmitted with Carrier Sense Error Statistic Register"]
pub mod ieee_t_cserr;
#[doc = "Reserved Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_sqe](ieee_t_sqe) module"]
pub type IEEE_T_SQE = crate::Reg<u32, _IEEE_T_SQE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_T_SQE;
#[doc = "`read()` method returns [ieee_t_sqe::R](ieee_t_sqe::R) reader structure"]
impl crate::Readable for IEEE_T_SQE {}
#[doc = "Reserved Statistic Register"]
pub mod ieee_t_sqe;
#[doc = "Flow Control Pause Frames Transmitted Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_fdxfc](ieee_t_fdxfc) module"]
pub type IEEE_T_FDXFC = crate::Reg<u32, _IEEE_T_FDXFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_T_FDXFC;
#[doc = "`read()` method returns [ieee_t_fdxfc::R](ieee_t_fdxfc::R) reader structure"]
impl crate::Readable for IEEE_T_FDXFC {}
#[doc = "Flow Control Pause Frames Transmitted Statistic Register"]
pub mod ieee_t_fdxfc;
#[doc = "Octet Count for Frames Transmitted w/o Error Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_octets_ok](ieee_t_octets_ok) module"]
pub type IEEE_T_OCTETS_OK = crate::Reg<u32, _IEEE_T_OCTETS_OK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_T_OCTETS_OK;
#[doc = "`read()` method returns [ieee_t_octets_ok::R](ieee_t_octets_ok::R) reader structure"]
impl crate::Readable for IEEE_T_OCTETS_OK {}
#[doc = "Octet Count for Frames Transmitted w/o Error Statistic Register"]
pub mod ieee_t_octets_ok;
#[doc = "Rx Packet Count Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_packets](rmon_r_packets) module"]
pub type RMON_R_PACKETS = crate::Reg<u32, _RMON_R_PACKETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_PACKETS;
#[doc = "`read()` method returns [rmon_r_packets::R](rmon_r_packets::R) reader structure"]
impl crate::Readable for RMON_R_PACKETS {}
#[doc = "Rx Packet Count Statistic Register"]
pub mod rmon_r_packets;
#[doc = "Rx Broadcast Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_bc_pkt](rmon_r_bc_pkt) module"]
pub type RMON_R_BC_PKT = crate::Reg<u32, _RMON_R_BC_PKT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_BC_PKT;
#[doc = "`read()` method returns [rmon_r_bc_pkt::R](rmon_r_bc_pkt::R) reader structure"]
impl crate::Readable for RMON_R_BC_PKT {}
#[doc = "Rx Broadcast Packets Statistic Register"]
pub mod rmon_r_bc_pkt;
#[doc = "Rx Multicast Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_mc_pkt](rmon_r_mc_pkt) module"]
pub type RMON_R_MC_PKT = crate::Reg<u32, _RMON_R_MC_PKT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_MC_PKT;
#[doc = "`read()` method returns [rmon_r_mc_pkt::R](rmon_r_mc_pkt::R) reader structure"]
impl crate::Readable for RMON_R_MC_PKT {}
#[doc = "Rx Multicast Packets Statistic Register"]
pub mod rmon_r_mc_pkt;
#[doc = "Rx Packets with CRC/Align Error Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_crc_align](rmon_r_crc_align) module"]
pub type RMON_R_CRC_ALIGN = crate::Reg<u32, _RMON_R_CRC_ALIGN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_CRC_ALIGN;
#[doc = "`read()` method returns [rmon_r_crc_align::R](rmon_r_crc_align::R) reader structure"]
impl crate::Readable for RMON_R_CRC_ALIGN {}
#[doc = "Rx Packets with CRC/Align Error Statistic Register"]
pub mod rmon_r_crc_align;
#[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_undersize](rmon_r_undersize) module"]
pub type RMON_R_UNDERSIZE = crate::Reg<u32, _RMON_R_UNDERSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_UNDERSIZE;
#[doc = "`read()` method returns [rmon_r_undersize::R](rmon_r_undersize::R) reader structure"]
impl crate::Readable for RMON_R_UNDERSIZE {}
#[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
pub mod rmon_r_undersize;
#[doc = "Rx Packets Greater Than MAX_FL and Good CRC Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_oversize](rmon_r_oversize) module"]
pub type RMON_R_OVERSIZE = crate::Reg<u32, _RMON_R_OVERSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_OVERSIZE;
#[doc = "`read()` method returns [rmon_r_oversize::R](rmon_r_oversize::R) reader structure"]
impl crate::Readable for RMON_R_OVERSIZE {}
#[doc = "Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
pub mod rmon_r_oversize;
#[doc = "Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_frag](rmon_r_frag) module"]
pub type RMON_R_FRAG = crate::Reg<u32, _RMON_R_FRAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_FRAG;
#[doc = "`read()` method returns [rmon_r_frag::R](rmon_r_frag::R) reader structure"]
impl crate::Readable for RMON_R_FRAG {}
#[doc = "Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
pub mod rmon_r_frag;
#[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_jab](rmon_r_jab) module"]
pub type RMON_R_JAB = crate::Reg<u32, _RMON_R_JAB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_JAB;
#[doc = "`read()` method returns [rmon_r_jab::R](rmon_r_jab::R) reader structure"]
impl crate::Readable for RMON_R_JAB {}
#[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
pub mod rmon_r_jab;
#[doc = "Reserved Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_resvd_0](rmon_r_resvd_0) module"]
pub type RMON_R_RESVD_0 = crate::Reg<u32, _RMON_R_RESVD_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_RESVD_0;
#[doc = "`read()` method returns [rmon_r_resvd_0::R](rmon_r_resvd_0::R) reader structure"]
impl crate::Readable for RMON_R_RESVD_0 {}
#[doc = "Reserved Statistic Register"]
pub mod rmon_r_resvd_0;
#[doc = "Rx 64-Byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_p64](rmon_r_p64) module"]
pub type RMON_R_P64 = crate::Reg<u32, _RMON_R_P64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_P64;
#[doc = "`read()` method returns [rmon_r_p64::R](rmon_r_p64::R) reader structure"]
impl crate::Readable for RMON_R_P64 {}
#[doc = "Rx 64-Byte Packets Statistic Register"]
pub mod rmon_r_p64;
#[doc = "Rx 65- to 127-Byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_p65to127](rmon_r_p65to127) module"]
pub type RMON_R_P65TO127 = crate::Reg<u32, _RMON_R_P65TO127>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_P65TO127;
#[doc = "`read()` method returns [rmon_r_p65to127::R](rmon_r_p65to127::R) reader structure"]
impl crate::Readable for RMON_R_P65TO127 {}
#[doc = "Rx 65- to 127-Byte Packets Statistic Register"]
pub mod rmon_r_p65to127;
#[doc = "Rx 128- to 255-Byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_p128to255](rmon_r_p128to255) module"]
pub type RMON_R_P128TO255 = crate::Reg<u32, _RMON_R_P128TO255>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_P128TO255;
#[doc = "`read()` method returns [rmon_r_p128to255::R](rmon_r_p128to255::R) reader structure"]
impl crate::Readable for RMON_R_P128TO255 {}
#[doc = "Rx 128- to 255-Byte Packets Statistic Register"]
pub mod rmon_r_p128to255;
#[doc = "Rx 256- to 511-Byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_p256to511](rmon_r_p256to511) module"]
pub type RMON_R_P256TO511 = crate::Reg<u32, _RMON_R_P256TO511>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_P256TO511;
#[doc = "`read()` method returns [rmon_r_p256to511::R](rmon_r_p256to511::R) reader structure"]
impl crate::Readable for RMON_R_P256TO511 {}
#[doc = "Rx 256- to 511-Byte Packets Statistic Register"]
pub mod rmon_r_p256to511;
#[doc = "Rx 512- to 1023-Byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_p512to1023](rmon_r_p512to1023) module"]
pub type RMON_R_P512TO1023 = crate::Reg<u32, _RMON_R_P512TO1023>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_P512TO1023;
#[doc = "`read()` method returns [rmon_r_p512to1023::R](rmon_r_p512to1023::R) reader structure"]
impl crate::Readable for RMON_R_P512TO1023 {}
#[doc = "Rx 512- to 1023-Byte Packets Statistic Register"]
pub mod rmon_r_p512to1023;
#[doc = "Rx 1024- to 2047-Byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_p1024to2047](rmon_r_p1024to2047) module"]
pub type RMON_R_P1024TO2047 = crate::Reg<u32, _RMON_R_P1024TO2047>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_P1024TO2047;
#[doc = "`read()` method returns [rmon_r_p1024to2047::R](rmon_r_p1024to2047::R) reader structure"]
impl crate::Readable for RMON_R_P1024TO2047 {}
#[doc = "Rx 1024- to 2047-Byte Packets Statistic Register"]
pub mod rmon_r_p1024to2047;
#[doc = "Rx Packets Greater than 2048 Bytes Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_p_gte2048](rmon_r_p_gte2048) module"]
pub type RMON_R_P_GTE2048 = crate::Reg<u32, _RMON_R_P_GTE2048>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_P_GTE2048;
#[doc = "`read()` method returns [rmon_r_p_gte2048::R](rmon_r_p_gte2048::R) reader structure"]
impl crate::Readable for RMON_R_P_GTE2048 {}
#[doc = "Rx Packets Greater than 2048 Bytes Statistic Register"]
pub mod rmon_r_p_gte2048;
#[doc = "Rx Octets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_octets](rmon_r_octets) module"]
pub type RMON_R_OCTETS = crate::Reg<u32, _RMON_R_OCTETS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMON_R_OCTETS;
#[doc = "`read()` method returns [rmon_r_octets::R](rmon_r_octets::R) reader structure"]
impl crate::Readable for RMON_R_OCTETS {}
#[doc = "Rx Octets Statistic Register"]
pub mod rmon_r_octets;
#[doc = "Frames not Counted Correctly Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_r_drop](ieee_r_drop) module"]
pub type IEEE_R_DROP = crate::Reg<u32, _IEEE_R_DROP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_R_DROP;
#[doc = "`read()` method returns [ieee_r_drop::R](ieee_r_drop::R) reader structure"]
impl crate::Readable for IEEE_R_DROP {}
#[doc = "Frames not Counted Correctly Statistic Register"]
pub mod ieee_r_drop;
#[doc = "Frames Received OK Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_r_frame_ok](ieee_r_frame_ok) module"]
pub type IEEE_R_FRAME_OK = crate::Reg<u32, _IEEE_R_FRAME_OK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_R_FRAME_OK;
#[doc = "`read()` method returns [ieee_r_frame_ok::R](ieee_r_frame_ok::R) reader structure"]
impl crate::Readable for IEEE_R_FRAME_OK {}
#[doc = "Frames Received OK Statistic Register"]
pub mod ieee_r_frame_ok;
#[doc = "Frames Received with CRC Error Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_r_crc](ieee_r_crc) module"]
pub type IEEE_R_CRC = crate::Reg<u32, _IEEE_R_CRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_R_CRC;
#[doc = "`read()` method returns [ieee_r_crc::R](ieee_r_crc::R) reader structure"]
impl crate::Readable for IEEE_R_CRC {}
#[doc = "Frames Received with CRC Error Statistic Register"]
pub mod ieee_r_crc;
#[doc = "Frames Received with Alignment Error Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_r_align](ieee_r_align) module"]
pub type IEEE_R_ALIGN = crate::Reg<u32, _IEEE_R_ALIGN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_R_ALIGN;
#[doc = "`read()` method returns [ieee_r_align::R](ieee_r_align::R) reader structure"]
impl crate::Readable for IEEE_R_ALIGN {}
#[doc = "Frames Received with Alignment Error Statistic Register"]
pub mod ieee_r_align;
#[doc = "Receive FIFO Overflow Count Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_r_macerr](ieee_r_macerr) module"]
pub type IEEE_R_MACERR = crate::Reg<u32, _IEEE_R_MACERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_R_MACERR;
#[doc = "`read()` method returns [ieee_r_macerr::R](ieee_r_macerr::R) reader structure"]
impl crate::Readable for IEEE_R_MACERR {}
#[doc = "Receive FIFO Overflow Count Statistic Register"]
pub mod ieee_r_macerr;
#[doc = "Flow Control Pause Frames Received Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_r_fdxfc](ieee_r_fdxfc) module"]
pub type IEEE_R_FDXFC = crate::Reg<u32, _IEEE_R_FDXFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_R_FDXFC;
#[doc = "`read()` method returns [ieee_r_fdxfc::R](ieee_r_fdxfc::R) reader structure"]
impl crate::Readable for IEEE_R_FDXFC {}
#[doc = "Flow Control Pause Frames Received Statistic Register"]
pub mod ieee_r_fdxfc;
#[doc = "Octet Count for Frames Received without Error Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_r_octets_ok](ieee_r_octets_ok) module"]
pub type IEEE_R_OCTETS_OK = crate::Reg<u32, _IEEE_R_OCTETS_OK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEEE_R_OCTETS_OK;
#[doc = "`read()` method returns [ieee_r_octets_ok::R](ieee_r_octets_ok::R) reader structure"]
impl crate::Readable for IEEE_R_OCTETS_OK {}
#[doc = "Octet Count for Frames Received without Error Statistic Register"]
pub mod ieee_r_octets_ok;
#[doc = "Adjustable Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcr](atcr) module"]
pub type ATCR = crate::Reg<u32, _ATCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATCR;
#[doc = "`read()` method returns [atcr::R](atcr::R) reader structure"]
impl crate::Readable for ATCR {}
#[doc = "`write(|w| ..)` method takes [atcr::W](atcr::W) writer structure"]
impl crate::Writable for ATCR {}
#[doc = "Adjustable Timer Control Register"]
pub mod atcr;
#[doc = "Timer Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atvr](atvr) module"]
pub type ATVR = crate::Reg<u32, _ATVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATVR;
#[doc = "`read()` method returns [atvr::R](atvr::R) reader structure"]
impl crate::Readable for ATVR {}
#[doc = "`write(|w| ..)` method takes [atvr::W](atvr::W) writer structure"]
impl crate::Writable for ATVR {}
#[doc = "Timer Value Register"]
pub mod atvr;
#[doc = "Timer Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atoff](atoff) module"]
pub type ATOFF = crate::Reg<u32, _ATOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATOFF;
#[doc = "`read()` method returns [atoff::R](atoff::R) reader structure"]
impl crate::Readable for ATOFF {}
#[doc = "`write(|w| ..)` method takes [atoff::W](atoff::W) writer structure"]
impl crate::Writable for ATOFF {}
#[doc = "Timer Offset Register"]
pub mod atoff;
#[doc = "Timer Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atper](atper) module"]
pub type ATPER = crate::Reg<u32, _ATPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATPER;
#[doc = "`read()` method returns [atper::R](atper::R) reader structure"]
impl crate::Readable for ATPER {}
#[doc = "`write(|w| ..)` method takes [atper::W](atper::W) writer structure"]
impl crate::Writable for ATPER {}
#[doc = "Timer Period Register"]
pub mod atper;
#[doc = "Timer Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcor](atcor) module"]
pub type ATCOR = crate::Reg<u32, _ATCOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATCOR;
#[doc = "`read()` method returns [atcor::R](atcor::R) reader structure"]
impl crate::Readable for ATCOR {}
#[doc = "`write(|w| ..)` method takes [atcor::W](atcor::W) writer structure"]
impl crate::Writable for ATCOR {}
#[doc = "Timer Correction Register"]
pub mod atcor;
#[doc = "Time-Stamping Clock Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atinc](atinc) module"]
pub type ATINC = crate::Reg<u32, _ATINC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATINC;
#[doc = "`read()` method returns [atinc::R](atinc::R) reader structure"]
impl crate::Readable for ATINC {}
#[doc = "`write(|w| ..)` method takes [atinc::W](atinc::W) writer structure"]
impl crate::Writable for ATINC {}
#[doc = "Time-Stamping Clock Period Register"]
pub mod atinc;
#[doc = "Timestamp of Last Transmitted Frame\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atstmp](atstmp) module"]
pub type ATSTMP = crate::Reg<u32, _ATSTMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATSTMP;
#[doc = "`read()` method returns [atstmp::R](atstmp::R) reader structure"]
impl crate::Readable for ATSTMP {}
#[doc = "Timestamp of Last Transmitted Frame"]
pub mod atstmp;
#[doc = "Timer Global Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tgsr](tgsr) module"]
pub type TGSR = crate::Reg<u32, _TGSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TGSR;
#[doc = "`read()` method returns [tgsr::R](tgsr::R) reader structure"]
impl crate::Readable for TGSR {}
#[doc = "`write(|w| ..)` method takes [tgsr::W](tgsr::W) writer structure"]
impl crate::Writable for TGSR {}
#[doc = "Timer Global Status Register"]
pub mod tgsr;
#[doc = "Timer Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcsr](tcsr) module"]
pub type TCSR = crate::Reg<u32, _TCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCSR;
#[doc = "`read()` method returns [tcsr::R](tcsr::R) reader structure"]
impl crate::Readable for TCSR {}
#[doc = "`write(|w| ..)` method takes [tcsr::W](tcsr::W) writer structure"]
impl crate::Writable for TCSR {}
#[doc = "Timer Control Status Register"]
pub mod tcsr;
#[doc = "Timer Compare Capture Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr](tccr) module"]
pub type TCCR = crate::Reg<u32, _TCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCCR;
#[doc = "`read()` method returns [tccr::R](tccr::R) reader structure"]
impl crate::Readable for TCCR {}
#[doc = "`write(|w| ..)` method takes [tccr::W](tccr::W) writer structure"]
impl crate::Writable for TCCR {}
#[doc = "Timer Compare Capture Register"]
pub mod tccr;
