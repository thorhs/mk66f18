#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Identification Register"]
    pub id: ID,
    #[doc = "0x04 - General Hardware Parameters Register"]
    pub hwgeneral: HWGENERAL,
    #[doc = "0x08 - Host Hardware Parameters Register"]
    pub hwhost: HWHOST,
    #[doc = "0x0c - Device Hardware Parameters Register"]
    pub hwdevice: HWDEVICE,
    #[doc = "0x10 - Transmit Buffer Hardware Parameters Register"]
    pub hwtxbuf: HWTXBUF,
    #[doc = "0x14 - Receive Buffer Hardware Parameters Register"]
    pub hwrxbuf: HWRXBUF,
    _reserved0: [u8; 104usize],
    #[doc = "0x80 - General Purpose Timer n Load Register"]
    pub gptimer0ld: GPTIMERLD,
    #[doc = "0x84 - General Purpose Timer n Control Register"]
    pub gptimer0ctl: GPTIMERCTL,
    #[doc = "0x88 - General Purpose Timer n Load Register"]
    pub gptimer1ld: GPTIMERLD,
    #[doc = "0x8c - General Purpose Timer n Control Register"]
    pub gptimer1ctl: GPTIMERCTL,
    #[doc = "0x90 - System Bus Interface Configuration Register"]
    pub usb_sbuscfg: USB_SBUSCFG,
    _reserved1: [u8; 108usize],
    #[doc = "0x100 - Host Controller Interface Version and Capability Registers Length Register"]
    pub hciversion: HCIVERSION,
    #[doc = "0x104 - Host Controller Structural Parameters Register"]
    pub hcsparams: HCSPARAMS,
    #[doc = "0x108 - Host Controller Capability Parameters Register"]
    pub hccparams: HCCPARAMS,
    _reserved2: [u8; 22usize],
    #[doc = "0x122 - Device Controller Interface Version"]
    pub dciversion: DCIVERSION,
    #[doc = "0x124 - Device Controller Capability Parameters"]
    pub dccparams: DCCPARAMS,
    _reserved3: [u8; 24usize],
    #[doc = "0x140 - USB Command Register"]
    pub usbcmd: USBCMD,
    #[doc = "0x144 - USB Status Register"]
    pub usbsts: USBSTS,
    #[doc = "0x148 - USB Interrupt Enable Register"]
    pub usbintr: USBINTR,
    #[doc = "0x14c - Frame Index Register"]
    pub frindex: FRINDEX,
    _reserved4: [u8; 4usize],
    #[doc = "0x154 - Device Address Register"]
    pub deviceaddr: DEVICEADDR,
    #[doc = "0x158 - Current Asynchronous List Address Register"]
    pub asynclistaddr: ASYNCLISTADDR,
    #[doc = "0x15c - Host TT Asynchronous Buffer Control"]
    pub ttctrl: TTCTRL,
    #[doc = "0x160 - Master Interface Data Burst Size Register"]
    pub burstsize: BURSTSIZE,
    #[doc = "0x164 - Transmit FIFO Tuning Control Register"]
    pub txfilltuning: TXFILLTUNING,
    _reserved5: [u8; 16usize],
    #[doc = "0x178 - Endpoint NAK Register"]
    pub endptnak: ENDPTNAK,
    #[doc = "0x17c - Endpoint NAK Enable Register"]
    pub endptnaken: ENDPTNAKEN,
    #[doc = "0x180 - Configure Flag Register"]
    pub configflag: CONFIGFLAG,
    #[doc = "0x184 - Port Status and Control Registers"]
    pub portsc1: PORTSC1,
    _reserved6: [u8; 28usize],
    #[doc = "0x1a4 - On-the-Go Status and Control Register"]
    pub otgsc: OTGSC,
    #[doc = "0x1a8 - USB Mode Register"]
    pub usbmode: USBMODE,
    #[doc = "0x1ac - Endpoint Setup Status Register"]
    pub epsetupsr: EPSETUPSR,
    #[doc = "0x1b0 - Endpoint Initialization Register"]
    pub epprime: EPPRIME,
    #[doc = "0x1b4 - Endpoint Flush Register"]
    pub epflush: EPFLUSH,
    #[doc = "0x1b8 - Endpoint Status Register"]
    pub epsr: EPSR,
    #[doc = "0x1bc - Endpoint Complete Register"]
    pub epcomplete: EPCOMPLETE,
    #[doc = "0x1c0 - Endpoint Control Register 0"]
    pub epcr0: EPCR0,
    #[doc = "0x1c4 - Endpoint Control Register n"]
    pub epcr1: EPCR,
    #[doc = "0x1c8 - Endpoint Control Register n"]
    pub epcr2: EPCR,
    #[doc = "0x1cc - Endpoint Control Register n"]
    pub epcr3: EPCR,
    #[doc = "0x1d0 - Endpoint Control Register n"]
    pub epcr4: EPCR,
    #[doc = "0x1d4 - Endpoint Control Register n"]
    pub epcr5: EPCR,
    #[doc = "0x1d8 - Endpoint Control Register n"]
    pub epcr6: EPCR,
    #[doc = "0x1dc - Endpoint Control Register n"]
    pub epcr7: EPCR,
    _reserved7: [u8; 32usize],
    #[doc = "0x200 - USB General Control Register"]
    pub usbgenctrl: USBGENCTRL,
}
#[doc = "Identification Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Identification Register"]
pub mod id;
#[doc = "General Hardware Parameters Register"]
pub struct HWGENERAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Hardware Parameters Register"]
pub mod hwgeneral;
#[doc = "Host Hardware Parameters Register"]
pub struct HWHOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Hardware Parameters Register"]
pub mod hwhost;
#[doc = "Device Hardware Parameters Register"]
pub struct HWDEVICE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Hardware Parameters Register"]
pub mod hwdevice;
#[doc = "Transmit Buffer Hardware Parameters Register"]
pub struct HWTXBUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Buffer Hardware Parameters Register"]
pub mod hwtxbuf;
#[doc = "Receive Buffer Hardware Parameters Register"]
pub struct HWRXBUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer Hardware Parameters Register"]
pub mod hwrxbuf;
#[doc = "General Purpose Timer n Load Register"]
pub struct GPTIMERLD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Timer n Load Register"]
pub mod gptimerld;
#[doc = "General Purpose Timer n Control Register"]
pub struct GPTIMERCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Timer n Control Register"]
pub mod gptimerctl;
#[doc = "System Bus Interface Configuration Register"]
pub struct USB_SBUSCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Bus Interface Configuration Register"]
pub mod usb_sbuscfg;
#[doc = "Host Controller Interface Version and Capability Registers Length Register"]
pub struct HCIVERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Controller Interface Version and Capability Registers Length Register"]
pub mod hciversion;
#[doc = "Host Controller Structural Parameters Register"]
pub struct HCSPARAMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Controller Structural Parameters Register"]
pub mod hcsparams;
#[doc = "Host Controller Capability Parameters Register"]
pub struct HCCPARAMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Controller Capability Parameters Register"]
pub mod hccparams;
#[doc = "Device Controller Interface Version"]
pub struct DCIVERSION {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Device Controller Interface Version"]
pub mod dciversion;
#[doc = "Device Controller Capability Parameters"]
pub struct DCCPARAMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Controller Capability Parameters"]
pub mod dccparams;
#[doc = "USB Command Register"]
pub struct USBCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Command Register"]
pub mod usbcmd;
#[doc = "USB Status Register"]
pub struct USBSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Status Register"]
pub mod usbsts;
#[doc = "USB Interrupt Enable Register"]
pub struct USBINTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Interrupt Enable Register"]
pub mod usbintr;
#[doc = "Frame Index Register"]
pub struct FRINDEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frame Index Register"]
pub mod frindex;
#[doc = "Device Address Register"]
pub struct DEVICEADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Address Register"]
pub mod deviceaddr;
#[doc = "Periodic Frame List Base Address Register"]
pub struct PERIODICLISTBASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Periodic Frame List Base Address Register"]
pub mod periodiclistbase;
#[doc = "Current Asynchronous List Address Register"]
pub struct ASYNCLISTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Asynchronous List Address Register"]
pub mod asynclistaddr;
#[doc = "Endpoint List Address Register"]
pub struct EPLISTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint List Address Register"]
pub mod eplistaddr;
#[doc = "Host TT Asynchronous Buffer Control"]
pub struct TTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host TT Asynchronous Buffer Control"]
pub mod ttctrl;
#[doc = "Master Interface Data Burst Size Register"]
pub struct BURSTSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Interface Data Burst Size Register"]
pub mod burstsize;
#[doc = "Transmit FIFO Tuning Control Register"]
pub struct TXFILLTUNING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit FIFO Tuning Control Register"]
pub mod txfilltuning;
#[doc = "Endpoint NAK Register"]
pub struct ENDPTNAK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint NAK Register"]
pub mod endptnak;
#[doc = "Endpoint NAK Enable Register"]
pub struct ENDPTNAKEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint NAK Enable Register"]
pub mod endptnaken;
#[doc = "Configure Flag Register"]
pub struct CONFIGFLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configure Flag Register"]
pub mod configflag;
#[doc = "Port Status and Control Registers"]
pub struct PORTSC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Status and Control Registers"]
pub mod portsc1;
#[doc = "On-the-Go Status and Control Register"]
pub struct OTGSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "On-the-Go Status and Control Register"]
pub mod otgsc;
#[doc = "USB Mode Register"]
pub struct USBMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Mode Register"]
pub mod usbmode;
#[doc = "Endpoint Setup Status Register"]
pub struct EPSETUPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Setup Status Register"]
pub mod epsetupsr;
#[doc = "Endpoint Initialization Register"]
pub struct EPPRIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Initialization Register"]
pub mod epprime;
#[doc = "Endpoint Flush Register"]
pub struct EPFLUSH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Flush Register"]
pub mod epflush;
#[doc = "Endpoint Status Register"]
pub struct EPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Status Register"]
pub mod epsr;
#[doc = "Endpoint Complete Register"]
pub struct EPCOMPLETE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Complete Register"]
pub mod epcomplete;
#[doc = "Endpoint Control Register 0"]
pub struct EPCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Register 0"]
pub mod epcr0;
#[doc = "Endpoint Control Register n"]
pub struct EPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint Control Register n"]
pub mod epcr;
#[doc = "USB General Control Register"]
pub struct USBGENCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB General Control Register"]
pub mod usbgenctrl;
