#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral ID register"]
    pub perid: PERID,
    _reserved0: [u8; 3usize],
    #[doc = "0x04 - Peripheral ID Complement register"]
    pub idcomp: IDCOMP,
    _reserved1: [u8; 3usize],
    #[doc = "0x08 - Peripheral Revision register"]
    pub rev: REV,
    _reserved2: [u8; 3usize],
    #[doc = "0x0c - Peripheral Additional Info register"]
    pub addinfo: ADDINFO,
    _reserved3: [u8; 3usize],
    #[doc = "0x10 - OTG Interrupt Status register"]
    pub otgistat: OTGISTAT,
    _reserved4: [u8; 3usize],
    #[doc = "0x14 - OTG Interrupt Control register"]
    pub otgicr: OTGICR,
    _reserved5: [u8; 3usize],
    #[doc = "0x18 - OTG Status register"]
    pub otgstat: OTGSTAT,
    _reserved6: [u8; 3usize],
    #[doc = "0x1c - OTG Control register"]
    pub otgctl: OTGCTL,
    _reserved7: [u8; 99usize],
    #[doc = "0x80 - Interrupt Status register"]
    pub istat: ISTAT,
    _reserved8: [u8; 3usize],
    #[doc = "0x84 - Interrupt Enable register"]
    pub inten: INTEN,
    _reserved9: [u8; 3usize],
    #[doc = "0x88 - Error Interrupt Status register"]
    pub errstat: ERRSTAT,
    _reserved10: [u8; 3usize],
    #[doc = "0x8c - Error Interrupt Enable register"]
    pub erren: ERREN,
    _reserved11: [u8; 3usize],
    #[doc = "0x90 - Status register"]
    pub stat: STAT,
    _reserved12: [u8; 3usize],
    #[doc = "0x94 - Control register"]
    pub ctl: CTL,
    _reserved13: [u8; 3usize],
    #[doc = "0x98 - Address register"]
    pub addr: ADDR,
    _reserved14: [u8; 3usize],
    #[doc = "0x9c - BDT Page register 1"]
    pub bdtpage1: BDTPAGE1,
    _reserved15: [u8; 3usize],
    #[doc = "0xa0 - Frame Number register Low"]
    pub frmnuml: FRMNUML,
    _reserved16: [u8; 3usize],
    #[doc = "0xa4 - Frame Number register High"]
    pub frmnumh: FRMNUMH,
    _reserved17: [u8; 3usize],
    #[doc = "0xa8 - Token register"]
    pub token: TOKEN,
    _reserved18: [u8; 3usize],
    #[doc = "0xac - SOF Threshold register"]
    pub softhld: SOFTHLD,
    _reserved19: [u8; 3usize],
    #[doc = "0xb0 - BDT Page Register 2"]
    pub bdtpage2: BDTPAGE2,
    _reserved20: [u8; 3usize],
    #[doc = "0xb4 - BDT Page Register 3"]
    pub bdtpage3: BDTPAGE3,
    _reserved21: [u8; 11usize],
    #[doc = "0xc0 - Endpoint Control register"]
    pub endpt0: ENDPT,
    _reserved22: [u8; 3usize],
    #[doc = "0xc4 - Endpoint Control register"]
    pub endpt1: ENDPT,
    _reserved23: [u8; 3usize],
    #[doc = "0xc8 - Endpoint Control register"]
    pub endpt2: ENDPT,
    _reserved24: [u8; 3usize],
    #[doc = "0xcc - Endpoint Control register"]
    pub endpt3: ENDPT,
    _reserved25: [u8; 3usize],
    #[doc = "0xd0 - Endpoint Control register"]
    pub endpt4: ENDPT,
    _reserved26: [u8; 3usize],
    #[doc = "0xd4 - Endpoint Control register"]
    pub endpt5: ENDPT,
    _reserved27: [u8; 3usize],
    #[doc = "0xd8 - Endpoint Control register"]
    pub endpt6: ENDPT,
    _reserved28: [u8; 3usize],
    #[doc = "0xdc - Endpoint Control register"]
    pub endpt7: ENDPT,
    _reserved29: [u8; 3usize],
    #[doc = "0xe0 - Endpoint Control register"]
    pub endpt8: ENDPT,
    _reserved30: [u8; 3usize],
    #[doc = "0xe4 - Endpoint Control register"]
    pub endpt9: ENDPT,
    _reserved31: [u8; 3usize],
    #[doc = "0xe8 - Endpoint Control register"]
    pub endpt10: ENDPT,
    _reserved32: [u8; 3usize],
    #[doc = "0xec - Endpoint Control register"]
    pub endpt11: ENDPT,
    _reserved33: [u8; 3usize],
    #[doc = "0xf0 - Endpoint Control register"]
    pub endpt12: ENDPT,
    _reserved34: [u8; 3usize],
    #[doc = "0xf4 - Endpoint Control register"]
    pub endpt13: ENDPT,
    _reserved35: [u8; 3usize],
    #[doc = "0xf8 - Endpoint Control register"]
    pub endpt14: ENDPT,
    _reserved36: [u8; 3usize],
    #[doc = "0xfc - Endpoint Control register"]
    pub endpt15: ENDPT,
    _reserved37: [u8; 3usize],
    #[doc = "0x100 - USB Control register"]
    pub usbctrl: USBCTRL,
    _reserved38: [u8; 3usize],
    #[doc = "0x104 - USB OTG Observe register"]
    pub observe: OBSERVE,
    _reserved39: [u8; 3usize],
    #[doc = "0x108 - USB OTG Control register"]
    pub control: CONTROL,
    _reserved40: [u8; 3usize],
    #[doc = "0x10c - USB Transceiver Control register 0"]
    pub usbtrc0: USBTRC0,
    _reserved41: [u8; 7usize],
    #[doc = "0x114 - Frame Adjust Register"]
    pub usbfrmadjust: USBFRMADJUST,
    _reserved42: [u8; 43usize],
    #[doc = "0x140 - USB Clock recovery control"]
    pub clk_recover_ctrl: CLK_RECOVER_CTRL,
    _reserved43: [u8; 3usize],
    #[doc = "0x144 - IRC48M oscillator enable register"]
    pub clk_recover_irc_en: CLK_RECOVER_IRC_EN,
    _reserved44: [u8; 15usize],
    #[doc = "0x154 - Clock recovery combined interrupt enable"]
    pub clk_recover_int_en: CLK_RECOVER_INT_EN,
    _reserved45: [u8; 7usize],
    #[doc = "0x15c - Clock recovery separated interrupt status"]
    pub clk_recover_int_status: CLK_RECOVER_INT_STATUS,
}
#[doc = "Peripheral ID register"]
pub struct PERID {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Peripheral ID register"]
pub mod perid;
#[doc = "Peripheral ID Complement register"]
pub struct IDCOMP {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Peripheral ID Complement register"]
pub mod idcomp;
#[doc = "Peripheral Revision register"]
pub struct REV {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Peripheral Revision register"]
pub mod rev;
#[doc = "Peripheral Additional Info register"]
pub struct ADDINFO {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Peripheral Additional Info register"]
pub mod addinfo;
#[doc = "OTG Interrupt Status register"]
pub struct OTGISTAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "OTG Interrupt Status register"]
pub mod otgistat;
#[doc = "OTG Interrupt Control register"]
pub struct OTGICR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "OTG Interrupt Control register"]
pub mod otgicr;
#[doc = "OTG Status register"]
pub struct OTGSTAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "OTG Status register"]
pub mod otgstat;
#[doc = "OTG Control register"]
pub struct OTGCTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "OTG Control register"]
pub mod otgctl;
#[doc = "Interrupt Status register"]
pub struct ISTAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Status register"]
pub mod istat;
#[doc = "Interrupt Enable register"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Interrupt Enable register"]
pub mod inten;
#[doc = "Error Interrupt Status register"]
pub struct ERRSTAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Error Interrupt Status register"]
pub mod errstat;
#[doc = "Error Interrupt Enable register"]
pub struct ERREN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Error Interrupt Enable register"]
pub mod erren;
#[doc = "Status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Status register"]
pub mod stat;
#[doc = "Control register"]
pub struct CTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Control register"]
pub mod ctl;
#[doc = "Address register"]
pub struct ADDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Address register"]
pub mod addr;
#[doc = "BDT Page register 1"]
pub struct BDTPAGE1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "BDT Page register 1"]
pub mod bdtpage1;
#[doc = "Frame Number register Low"]
pub struct FRMNUML {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Frame Number register Low"]
pub mod frmnuml;
#[doc = "Frame Number register High"]
pub struct FRMNUMH {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Frame Number register High"]
pub mod frmnumh;
#[doc = "Token register"]
pub struct TOKEN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Token register"]
pub mod token;
#[doc = "SOF Threshold register"]
pub struct SOFTHLD {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "SOF Threshold register"]
pub mod softhld;
#[doc = "BDT Page Register 2"]
pub struct BDTPAGE2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "BDT Page Register 2"]
pub mod bdtpage2;
#[doc = "BDT Page Register 3"]
pub struct BDTPAGE3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "BDT Page Register 3"]
pub mod bdtpage3;
#[doc = "Endpoint Control register"]
pub struct ENDPT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Endpoint Control register"]
pub mod endpt;
#[doc = "USB Control register"]
pub struct USBCTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Control register"]
pub mod usbctrl;
#[doc = "USB OTG Observe register"]
pub struct OBSERVE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB OTG Observe register"]
pub mod observe;
#[doc = "USB OTG Control register"]
pub struct CONTROL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB OTG Control register"]
pub mod control;
#[doc = "USB Transceiver Control register 0"]
pub struct USBTRC0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transceiver Control register 0"]
pub mod usbtrc0;
#[doc = "Frame Adjust Register"]
pub struct USBFRMADJUST {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Frame Adjust Register"]
pub mod usbfrmadjust;
#[doc = "USB Clock recovery control"]
pub struct CLK_RECOVER_CTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Clock recovery control"]
pub mod clk_recover_ctrl;
#[doc = "IRC48M oscillator enable register"]
pub struct CLK_RECOVER_IRC_EN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "IRC48M oscillator enable register"]
pub mod clk_recover_irc_en;
#[doc = "Clock recovery combined interrupt enable"]
pub struct CLK_RECOVER_INT_EN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clock recovery combined interrupt enable"]
pub mod clk_recover_int_en;
#[doc = "Clock recovery separated interrupt status"]
pub struct CLK_RECOVER_INT_STATUS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clock recovery separated interrupt status"]
pub mod clk_recover_int_status;
