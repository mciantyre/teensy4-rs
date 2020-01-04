#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(clippy::all)]
#![no_std]

mod generic;
pub use generic::*;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA System Address"]
    pub ds_addr: DS_ADDR,
    #[doc = "0x04 - Block Attributes"]
    pub blk_att: BLK_ATT,
    #[doc = "0x08 - Command Argument"]
    pub cmd_arg: CMD_ARG,
    #[doc = "0x0c - Command Transfer Type"]
    pub cmd_xfr_typ: CMD_XFR_TYP,
    #[doc = "0x10 - Command Response0"]
    pub cmd_rsp0: CMD_RSP0,
    #[doc = "0x14 - Command Response1"]
    pub cmd_rsp1: CMD_RSP1,
    #[doc = "0x18 - Command Response2"]
    pub cmd_rsp2: CMD_RSP2,
    #[doc = "0x1c - Command Response3"]
    pub cmd_rsp3: CMD_RSP3,
    #[doc = "0x20 - Data Buffer Access Port"]
    pub data_buff_acc_port: DATA_BUFF_ACC_PORT,
    #[doc = "0x24 - Present State"]
    pub pres_state: PRES_STATE,
    #[doc = "0x28 - Protocol Control"]
    pub prot_ctrl: PROT_CTRL,
    #[doc = "0x2c - System Control"]
    pub sys_ctrl: SYS_CTRL,
    #[doc = "0x30 - Interrupt Status"]
    pub int_status: INT_STATUS,
    #[doc = "0x34 - Interrupt Status Enable"]
    pub int_status_en: INT_STATUS_EN,
    #[doc = "0x38 - Interrupt Signal Enable"]
    pub int_signal_en: INT_SIGNAL_EN,
    #[doc = "0x3c - Auto CMD12 Error Status"]
    pub autocmd12_err_status: AUTOCMD12_ERR_STATUS,
    #[doc = "0x40 - Host Controller Capabilities"]
    pub host_ctrl_cap: HOST_CTRL_CAP,
    #[doc = "0x44 - Watermark Level"]
    pub wtmk_lvl: WTMK_LVL,
    #[doc = "0x48 - Mixer Control"]
    pub mix_ctrl: MIX_CTRL,
    _reserved19: [u8; 4usize],
    #[doc = "0x50 - Force Event"]
    pub force_event: FORCE_EVENT,
    #[doc = "0x54 - ADMA Error Status Register"]
    pub adma_err_status: ADMA_ERR_STATUS,
    #[doc = "0x58 - ADMA System Address"]
    pub adma_sys_addr: ADMA_SYS_ADDR,
    _reserved22: [u8; 4usize],
    #[doc = "0x60 - DLL (Delay Line) Control"]
    pub dll_ctrl: DLL_CTRL,
    #[doc = "0x64 - DLL Status"]
    pub dll_status: DLL_STATUS,
    #[doc = "0x68 - CLK Tuning Control and Status"]
    pub clk_tune_ctrl_status: CLK_TUNE_CTRL_STATUS,
    _reserved25: [u8; 84usize],
    #[doc = "0xc0 - Vendor Specific Register"]
    pub vend_spec: VEND_SPEC,
    #[doc = "0xc4 - MMC Boot Register"]
    pub mmc_boot: MMC_BOOT,
    #[doc = "0xc8 - Vendor Specific 2 Register"]
    pub vend_spec2: VEND_SPEC2,
    #[doc = "0xcc - Tuning Control Register"]
    pub tuning_ctrl: TUNING_CTRL,
}
#[doc = "DMA System Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ds_addr](ds_addr) module"]
pub type DS_ADDR = crate::Reg<u32, _DS_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DS_ADDR;
#[doc = "`read()` method returns [ds_addr::R](ds_addr::R) reader structure"]
impl crate::Readable for DS_ADDR {}
#[doc = "`write(|w| ..)` method takes [ds_addr::W](ds_addr::W) writer structure"]
impl crate::Writable for DS_ADDR {}
#[doc = "DMA System Address"]
pub mod ds_addr;
#[doc = "Block Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk_att](blk_att) module"]
pub type BLK_ATT = crate::Reg<u32, _BLK_ATT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK_ATT;
#[doc = "`read()` method returns [blk_att::R](blk_att::R) reader structure"]
impl crate::Readable for BLK_ATT {}
#[doc = "`write(|w| ..)` method takes [blk_att::W](blk_att::W) writer structure"]
impl crate::Writable for BLK_ATT {}
#[doc = "Block Attributes"]
pub mod blk_att;
#[doc = "Command Argument\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_arg](cmd_arg) module"]
pub type CMD_ARG = crate::Reg<u32, _CMD_ARG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_ARG;
#[doc = "`read()` method returns [cmd_arg::R](cmd_arg::R) reader structure"]
impl crate::Readable for CMD_ARG {}
#[doc = "`write(|w| ..)` method takes [cmd_arg::W](cmd_arg::W) writer structure"]
impl crate::Writable for CMD_ARG {}
#[doc = "Command Argument"]
pub mod cmd_arg;
#[doc = "Command Transfer Type\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_xfr_typ](cmd_xfr_typ) module"]
pub type CMD_XFR_TYP = crate::Reg<u32, _CMD_XFR_TYP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_XFR_TYP;
#[doc = "`read()` method returns [cmd_xfr_typ::R](cmd_xfr_typ::R) reader structure"]
impl crate::Readable for CMD_XFR_TYP {}
#[doc = "`write(|w| ..)` method takes [cmd_xfr_typ::W](cmd_xfr_typ::W) writer structure"]
impl crate::Writable for CMD_XFR_TYP {}
#[doc = "Command Transfer Type"]
pub mod cmd_xfr_typ;
#[doc = "Command Response0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_rsp0](cmd_rsp0) module"]
pub type CMD_RSP0 = crate::Reg<u32, _CMD_RSP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_RSP0;
#[doc = "`read()` method returns [cmd_rsp0::R](cmd_rsp0::R) reader structure"]
impl crate::Readable for CMD_RSP0 {}
#[doc = "Command Response0"]
pub mod cmd_rsp0;
#[doc = "Command Response1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_rsp1](cmd_rsp1) module"]
pub type CMD_RSP1 = crate::Reg<u32, _CMD_RSP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_RSP1;
#[doc = "`read()` method returns [cmd_rsp1::R](cmd_rsp1::R) reader structure"]
impl crate::Readable for CMD_RSP1 {}
#[doc = "Command Response1"]
pub mod cmd_rsp1;
#[doc = "Command Response2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_rsp2](cmd_rsp2) module"]
pub type CMD_RSP2 = crate::Reg<u32, _CMD_RSP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_RSP2;
#[doc = "`read()` method returns [cmd_rsp2::R](cmd_rsp2::R) reader structure"]
impl crate::Readable for CMD_RSP2 {}
#[doc = "Command Response2"]
pub mod cmd_rsp2;
#[doc = "Command Response3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_rsp3](cmd_rsp3) module"]
pub type CMD_RSP3 = crate::Reg<u32, _CMD_RSP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD_RSP3;
#[doc = "`read()` method returns [cmd_rsp3::R](cmd_rsp3::R) reader structure"]
impl crate::Readable for CMD_RSP3 {}
#[doc = "Command Response3"]
pub mod cmd_rsp3;
#[doc = "Data Buffer Access Port\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_buff_acc_port](data_buff_acc_port) module"]
pub type DATA_BUFF_ACC_PORT = crate::Reg<u32, _DATA_BUFF_ACC_PORT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_BUFF_ACC_PORT;
#[doc = "`read()` method returns [data_buff_acc_port::R](data_buff_acc_port::R) reader structure"]
impl crate::Readable for DATA_BUFF_ACC_PORT {}
#[doc = "`write(|w| ..)` method takes [data_buff_acc_port::W](data_buff_acc_port::W) writer structure"]
impl crate::Writable for DATA_BUFF_ACC_PORT {}
#[doc = "Data Buffer Access Port"]
pub mod data_buff_acc_port;
#[doc = "Present State\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pres_state](pres_state) module"]
pub type PRES_STATE = crate::Reg<u32, _PRES_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRES_STATE;
#[doc = "`read()` method returns [pres_state::R](pres_state::R) reader structure"]
impl crate::Readable for PRES_STATE {}
#[doc = "Present State"]
pub mod pres_state;
#[doc = "Protocol Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prot_ctrl](prot_ctrl) module"]
pub type PROT_CTRL = crate::Reg<u32, _PROT_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROT_CTRL;
#[doc = "`read()` method returns [prot_ctrl::R](prot_ctrl::R) reader structure"]
impl crate::Readable for PROT_CTRL {}
#[doc = "`write(|w| ..)` method takes [prot_ctrl::W](prot_ctrl::W) writer structure"]
impl crate::Writable for PROT_CTRL {}
#[doc = "Protocol Control"]
pub mod prot_ctrl;
#[doc = "System Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_ctrl](sys_ctrl) module"]
pub type SYS_CTRL = crate::Reg<u32, _SYS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_CTRL;
#[doc = "`read()` method returns [sys_ctrl::R](sys_ctrl::R) reader structure"]
impl crate::Readable for SYS_CTRL {}
#[doc = "`write(|w| ..)` method takes [sys_ctrl::W](sys_ctrl::W) writer structure"]
impl crate::Writable for SYS_CTRL {}
#[doc = "System Control"]
pub mod sys_ctrl;
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](int_status) module"]
pub type INT_STATUS = crate::Reg<u32, _INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STATUS;
#[doc = "`read()` method returns [int_status::R](int_status::R) reader structure"]
impl crate::Readable for INT_STATUS {}
#[doc = "`write(|w| ..)` method takes [int_status::W](int_status::W) writer structure"]
impl crate::Writable for INT_STATUS {}
#[doc = "Interrupt Status"]
pub mod int_status;
#[doc = "Interrupt Status Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status_en](int_status_en) module"]
pub type INT_STATUS_EN = crate::Reg<u32, _INT_STATUS_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STATUS_EN;
#[doc = "`read()` method returns [int_status_en::R](int_status_en::R) reader structure"]
impl crate::Readable for INT_STATUS_EN {}
#[doc = "`write(|w| ..)` method takes [int_status_en::W](int_status_en::W) writer structure"]
impl crate::Writable for INT_STATUS_EN {}
#[doc = "Interrupt Status Enable"]
pub mod int_status_en;
#[doc = "Interrupt Signal Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_signal_en](int_signal_en) module"]
pub type INT_SIGNAL_EN = crate::Reg<u32, _INT_SIGNAL_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_SIGNAL_EN;
#[doc = "`read()` method returns [int_signal_en::R](int_signal_en::R) reader structure"]
impl crate::Readable for INT_SIGNAL_EN {}
#[doc = "`write(|w| ..)` method takes [int_signal_en::W](int_signal_en::W) writer structure"]
impl crate::Writable for INT_SIGNAL_EN {}
#[doc = "Interrupt Signal Enable"]
pub mod int_signal_en;
#[doc = "Auto CMD12 Error Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autocmd12_err_status](autocmd12_err_status) module"]
pub type AUTOCMD12_ERR_STATUS = crate::Reg<u32, _AUTOCMD12_ERR_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTOCMD12_ERR_STATUS;
#[doc = "`read()` method returns [autocmd12_err_status::R](autocmd12_err_status::R) reader structure"]
impl crate::Readable for AUTOCMD12_ERR_STATUS {}
#[doc = "`write(|w| ..)` method takes [autocmd12_err_status::W](autocmd12_err_status::W) writer structure"]
impl crate::Writable for AUTOCMD12_ERR_STATUS {}
#[doc = "Auto CMD12 Error Status"]
pub mod autocmd12_err_status;
#[doc = "Host Controller Capabilities\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ctrl_cap](host_ctrl_cap) module"]
pub type HOST_CTRL_CAP = crate::Reg<u32, _HOST_CTRL_CAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_CTRL_CAP;
#[doc = "`read()` method returns [host_ctrl_cap::R](host_ctrl_cap::R) reader structure"]
impl crate::Readable for HOST_CTRL_CAP {}
#[doc = "`write(|w| ..)` method takes [host_ctrl_cap::W](host_ctrl_cap::W) writer structure"]
impl crate::Writable for HOST_CTRL_CAP {}
#[doc = "Host Controller Capabilities"]
pub mod host_ctrl_cap;
#[doc = "Watermark Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtmk_lvl](wtmk_lvl) module"]
pub type WTMK_LVL = crate::Reg<u32, _WTMK_LVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WTMK_LVL;
#[doc = "`read()` method returns [wtmk_lvl::R](wtmk_lvl::R) reader structure"]
impl crate::Readable for WTMK_LVL {}
#[doc = "`write(|w| ..)` method takes [wtmk_lvl::W](wtmk_lvl::W) writer structure"]
impl crate::Writable for WTMK_LVL {}
#[doc = "Watermark Level"]
pub mod wtmk_lvl;
#[doc = "Mixer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mix_ctrl](mix_ctrl) module"]
pub type MIX_CTRL = crate::Reg<u32, _MIX_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIX_CTRL;
#[doc = "`read()` method returns [mix_ctrl::R](mix_ctrl::R) reader structure"]
impl crate::Readable for MIX_CTRL {}
#[doc = "`write(|w| ..)` method takes [mix_ctrl::W](mix_ctrl::W) writer structure"]
impl crate::Writable for MIX_CTRL {}
#[doc = "Mixer Control"]
pub mod mix_ctrl;
#[doc = "Force Event\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [force_event](force_event) module"]
pub type FORCE_EVENT = crate::Reg<u32, _FORCE_EVENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FORCE_EVENT;
#[doc = "`read()` method returns [force_event::R](force_event::R) reader structure"]
impl crate::Readable for FORCE_EVENT {}
#[doc = "`write(|w| ..)` method takes [force_event::W](force_event::W) writer structure"]
impl crate::Writable for FORCE_EVENT {}
#[doc = "Force Event"]
pub mod force_event;
#[doc = "ADMA Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adma_err_status](adma_err_status) module"]
pub type ADMA_ERR_STATUS = crate::Reg<u32, _ADMA_ERR_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADMA_ERR_STATUS;
#[doc = "`read()` method returns [adma_err_status::R](adma_err_status::R) reader structure"]
impl crate::Readable for ADMA_ERR_STATUS {}
#[doc = "ADMA Error Status Register"]
pub mod adma_err_status;
#[doc = "ADMA System Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adma_sys_addr](adma_sys_addr) module"]
pub type ADMA_SYS_ADDR = crate::Reg<u32, _ADMA_SYS_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADMA_SYS_ADDR;
#[doc = "`read()` method returns [adma_sys_addr::R](adma_sys_addr::R) reader structure"]
impl crate::Readable for ADMA_SYS_ADDR {}
#[doc = "`write(|w| ..)` method takes [adma_sys_addr::W](adma_sys_addr::W) writer structure"]
impl crate::Writable for ADMA_SYS_ADDR {}
#[doc = "ADMA System Address"]
pub mod adma_sys_addr;
#[doc = "DLL (Delay Line) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dll_ctrl](dll_ctrl) module"]
pub type DLL_CTRL = crate::Reg<u32, _DLL_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLL_CTRL;
#[doc = "`read()` method returns [dll_ctrl::R](dll_ctrl::R) reader structure"]
impl crate::Readable for DLL_CTRL {}
#[doc = "`write(|w| ..)` method takes [dll_ctrl::W](dll_ctrl::W) writer structure"]
impl crate::Writable for DLL_CTRL {}
#[doc = "DLL (Delay Line) Control"]
pub mod dll_ctrl;
#[doc = "DLL Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dll_status](dll_status) module"]
pub type DLL_STATUS = crate::Reg<u32, _DLL_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLL_STATUS;
#[doc = "`read()` method returns [dll_status::R](dll_status::R) reader structure"]
impl crate::Readable for DLL_STATUS {}
#[doc = "DLL Status"]
pub mod dll_status;
#[doc = "CLK Tuning Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_tune_ctrl_status](clk_tune_ctrl_status) module"]
pub type CLK_TUNE_CTRL_STATUS = crate::Reg<u32, _CLK_TUNE_CTRL_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TUNE_CTRL_STATUS;
#[doc = "`read()` method returns [clk_tune_ctrl_status::R](clk_tune_ctrl_status::R) reader structure"]
impl crate::Readable for CLK_TUNE_CTRL_STATUS {}
#[doc = "`write(|w| ..)` method takes [clk_tune_ctrl_status::W](clk_tune_ctrl_status::W) writer structure"]
impl crate::Writable for CLK_TUNE_CTRL_STATUS {}
#[doc = "CLK Tuning Control and Status"]
pub mod clk_tune_ctrl_status;
#[doc = "Vendor Specific Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vend_spec](vend_spec) module"]
pub type VEND_SPEC = crate::Reg<u32, _VEND_SPEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VEND_SPEC;
#[doc = "`read()` method returns [vend_spec::R](vend_spec::R) reader structure"]
impl crate::Readable for VEND_SPEC {}
#[doc = "`write(|w| ..)` method takes [vend_spec::W](vend_spec::W) writer structure"]
impl crate::Writable for VEND_SPEC {}
#[doc = "Vendor Specific Register"]
pub mod vend_spec;
#[doc = "MMC Boot Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_boot](mmc_boot) module"]
pub type MMC_BOOT = crate::Reg<u32, _MMC_BOOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMC_BOOT;
#[doc = "`read()` method returns [mmc_boot::R](mmc_boot::R) reader structure"]
impl crate::Readable for MMC_BOOT {}
#[doc = "`write(|w| ..)` method takes [mmc_boot::W](mmc_boot::W) writer structure"]
impl crate::Writable for MMC_BOOT {}
#[doc = "MMC Boot Register"]
pub mod mmc_boot;
#[doc = "Vendor Specific 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vend_spec2](vend_spec2) module"]
pub type VEND_SPEC2 = crate::Reg<u32, _VEND_SPEC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VEND_SPEC2;
#[doc = "`read()` method returns [vend_spec2::R](vend_spec2::R) reader structure"]
impl crate::Readable for VEND_SPEC2 {}
#[doc = "`write(|w| ..)` method takes [vend_spec2::W](vend_spec2::W) writer structure"]
impl crate::Writable for VEND_SPEC2 {}
#[doc = "Vendor Specific 2 Register"]
pub mod vend_spec2;
#[doc = "Tuning Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tuning_ctrl](tuning_ctrl) module"]
pub type TUNING_CTRL = crate::Reg<u32, _TUNING_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TUNING_CTRL;
#[doc = "`read()` method returns [tuning_ctrl::R](tuning_ctrl::R) reader structure"]
impl crate::Readable for TUNING_CTRL {}
#[doc = "`write(|w| ..)` method takes [tuning_ctrl::W](tuning_ctrl::W) writer structure"]
impl crate::Writable for TUNING_CTRL {}
#[doc = "Tuning Control Register"]
pub mod tuning_ctrl;
