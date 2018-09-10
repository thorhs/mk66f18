#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB PHY Power-Down Register"]
    pub pwd: PWD,
    #[doc = "0x04 - USB PHY Power-Down Register"]
    pub pwd_set: PWD_SET,
    #[doc = "0x08 - USB PHY Power-Down Register"]
    pub pwd_clr: PWD_CLR,
    #[doc = "0x0c - USB PHY Power-Down Register"]
    pub pwd_tog: PWD_TOG,
    #[doc = "0x10 - USB PHY Transmitter Control Register"]
    pub tx: TX,
    #[doc = "0x14 - USB PHY Transmitter Control Register"]
    pub tx_set: TX_SET,
    #[doc = "0x18 - USB PHY Transmitter Control Register"]
    pub tx_clr: TX_CLR,
    #[doc = "0x1c - USB PHY Transmitter Control Register"]
    pub tx_tog: TX_TOG,
    #[doc = "0x20 - USB PHY Receiver Control Register"]
    pub rx: RX,
    #[doc = "0x24 - USB PHY Receiver Control Register"]
    pub rx_set: RX_SET,
    #[doc = "0x28 - USB PHY Receiver Control Register"]
    pub rx_clr: RX_CLR,
    #[doc = "0x2c - USB PHY Receiver Control Register"]
    pub rx_tog: RX_TOG,
    #[doc = "0x30 - USB PHY General Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x34 - USB PHY General Control Register"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x38 - USB PHY General Control Register"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x3c - USB PHY General Control Register"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x40 - USB PHY Status Register"]
    pub status: STATUS,
    _reserved0: [u8; 12usize],
    #[doc = "0x50 - USB PHY Debug Register"]
    pub debug: DEBUG,
    #[doc = "0x54 - USB PHY Debug Register"]
    pub debug_set: DEBUG_SET,
    #[doc = "0x58 - USB PHY Debug Register"]
    pub debug_clr: DEBUG_CLR,
    #[doc = "0x5c - USB PHY Debug Register"]
    pub debug_tog: DEBUG_TOG,
    #[doc = "0x60 - UTMI Debug Status Register 0"]
    pub debug0_status: DEBUG0_STATUS,
    _reserved1: [u8; 12usize],
    #[doc = "0x70 - UTMI Debug Status Register 1"]
    pub debug1: DEBUG1,
    #[doc = "0x74 - UTMI Debug Status Register 1"]
    pub debug1_set: DEBUG1_SET,
    #[doc = "0x78 - UTMI Debug Status Register 1"]
    pub debug1_clr: DEBUG1_CLR,
    #[doc = "0x7c - UTMI Debug Status Register 1"]
    pub debug1_tog: DEBUG1_TOG,
    #[doc = "0x80 - UTMI RTL Version"]
    pub version: VERSION,
    _reserved2: [u8; 28usize],
    #[doc = "0xa0 - USB PHY PLL Control/Status Register"]
    pub pll_sic: PLL_SIC,
    #[doc = "0xa4 - USB PHY PLL Control/Status Register"]
    pub pll_sic_set: PLL_SIC_SET,
    #[doc = "0xa8 - USB PHY PLL Control/Status Register"]
    pub pll_sic_clr: PLL_SIC_CLR,
    #[doc = "0xac - USB PHY PLL Control/Status Register"]
    pub pll_sic_tog: PLL_SIC_TOG,
    _reserved3: [u8; 16usize],
    #[doc = "0xc0 - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect: USB1_VBUS_DETECT,
    #[doc = "0xc4 - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect_set: USB1_VBUS_DETECT_SET,
    #[doc = "0xc8 - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect_clr: USB1_VBUS_DETECT_CLR,
    #[doc = "0xcc - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect_tog: USB1_VBUS_DETECT_TOG,
    #[doc = "0xd0 - USB PHY VBUS Detector Status Register"]
    pub usb1_vbus_det_stat: USB1_VBUS_DET_STAT,
    _reserved4: [u8; 28usize],
    #[doc = "0xf0 - USB PHY Charger Detect Status Register"]
    pub usb1_chrg_det_stat: USB1_CHRG_DET_STAT,
    _reserved5: [u8; 12usize],
    #[doc = "0x100 - USB PHY Analog Control Register"]
    pub anactrl: ANACTRL,
    #[doc = "0x104 - USB PHY Analog Control Register"]
    pub anactrl_set: ANACTRL_SET,
    #[doc = "0x108 - USB PHY Analog Control Register"]
    pub anactrl_clr: ANACTRL_CLR,
    #[doc = "0x10c - USB PHY Analog Control Register"]
    pub anactrl_tog: ANACTRL_TOG,
    #[doc = "0x110 - USB PHY Loopback Control/Status Register"]
    pub usb1_loopback: USB1_LOOPBACK,
    #[doc = "0x114 - USB PHY Loopback Control/Status Register"]
    pub usb1_loopback_set: USB1_LOOPBACK_SET,
    #[doc = "0x118 - USB PHY Loopback Control/Status Register"]
    pub usb1_loopback_clr: USB1_LOOPBACK_CLR,
    #[doc = "0x11c - USB PHY Loopback Control/Status Register"]
    pub usb1_loopback_tog: USB1_LOOPBACK_TOG,
    #[doc = "0x120 - USB PHY Loopback Packet Number Select Register"]
    pub usb1_loopback_hsfscnt: USB1_LOOPBACK_HSFSCNT,
    #[doc = "0x124 - USB PHY Loopback Packet Number Select Register"]
    pub usb1_loopback_hsfscnt_set: USB1_LOOPBACK_HSFSCNT_SET,
    #[doc = "0x128 - USB PHY Loopback Packet Number Select Register"]
    pub usb1_loopback_hsfscnt_clr: USB1_LOOPBACK_HSFSCNT_CLR,
    #[doc = "0x12c - USB PHY Loopback Packet Number Select Register"]
    pub usb1_loopback_hsfscnt_tog: USB1_LOOPBACK_HSFSCNT_TOG,
    #[doc = "0x130 - USB PHY Trim Override Enable Register"]
    pub trim_override_en: TRIM_OVERRIDE_EN,
    #[doc = "0x134 - USB PHY Trim Override Enable Register"]
    pub trim_override_en_set: TRIM_OVERRIDE_EN_SET,
    #[doc = "0x138 - USB PHY Trim Override Enable Register"]
    pub trim_override_en_clr: TRIM_OVERRIDE_EN_CLR,
    #[doc = "0x13c - USB PHY Trim Override Enable Register"]
    pub trim_override_en_tog: TRIM_OVERRIDE_EN_TOG,
}
#[doc = "USB PHY Power-Down Register"]
pub struct PWD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Power-Down Register"]
pub mod pwd;
#[doc = "USB PHY Power-Down Register"]
pub struct PWD_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_set;
#[doc = "USB PHY Power-Down Register"]
pub struct PWD_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_clr;
#[doc = "USB PHY Power-Down Register"]
pub struct PWD_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_tog;
#[doc = "USB PHY Transmitter Control Register"]
pub struct TX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx;
#[doc = "USB PHY Transmitter Control Register"]
pub struct TX_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_set;
#[doc = "USB PHY Transmitter Control Register"]
pub struct TX_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_clr;
#[doc = "USB PHY Transmitter Control Register"]
pub struct TX_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_tog;
#[doc = "USB PHY Receiver Control Register"]
pub struct RX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Receiver Control Register"]
pub mod rx;
#[doc = "USB PHY Receiver Control Register"]
pub struct RX_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_set;
#[doc = "USB PHY Receiver Control Register"]
pub struct RX_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_clr;
#[doc = "USB PHY Receiver Control Register"]
pub struct RX_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_tog;
#[doc = "USB PHY General Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY General Control Register"]
pub mod ctrl;
#[doc = "USB PHY General Control Register"]
pub struct CTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY General Control Register"]
pub mod ctrl_set;
#[doc = "USB PHY General Control Register"]
pub struct CTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY General Control Register"]
pub mod ctrl_clr;
#[doc = "USB PHY General Control Register"]
pub struct CTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY General Control Register"]
pub mod ctrl_tog;
#[doc = "USB PHY Status Register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Status Register"]
pub mod status;
#[doc = "USB PHY Debug Register"]
pub struct DEBUG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Debug Register"]
pub mod debug;
#[doc = "USB PHY Debug Register"]
pub struct DEBUG_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Debug Register"]
pub mod debug_set;
#[doc = "USB PHY Debug Register"]
pub struct DEBUG_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Debug Register"]
pub mod debug_clr;
#[doc = "USB PHY Debug Register"]
pub struct DEBUG_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Debug Register"]
pub mod debug_tog;
#[doc = "UTMI Debug Status Register 0"]
pub struct DEBUG0_STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UTMI Debug Status Register 0"]
pub mod debug0_status;
#[doc = "UTMI Debug Status Register 1"]
pub struct DEBUG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1;
#[doc = "UTMI Debug Status Register 1"]
pub struct DEBUG1_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_set;
#[doc = "UTMI Debug Status Register 1"]
pub struct DEBUG1_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_clr;
#[doc = "UTMI Debug Status Register 1"]
pub struct DEBUG1_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UTMI Debug Status Register 1"]
pub mod debug1_tog;
#[doc = "UTMI RTL Version"]
pub struct VERSION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UTMI RTL Version"]
pub mod version;
#[doc = "USB PHY PLL Control/Status Register"]
pub struct PLL_SIC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic;
#[doc = "USB PHY PLL Control/Status Register"]
pub struct PLL_SIC_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_set;
#[doc = "USB PHY PLL Control/Status Register"]
pub struct PLL_SIC_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_clr;
#[doc = "USB PHY PLL Control/Status Register"]
pub struct PLL_SIC_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_tog;
#[doc = "USB PHY VBUS Detect Control Register"]
pub struct USB1_VBUS_DETECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect;
#[doc = "USB PHY VBUS Detect Control Register"]
pub struct USB1_VBUS_DETECT_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_set;
#[doc = "USB PHY VBUS Detect Control Register"]
pub struct USB1_VBUS_DETECT_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_clr;
#[doc = "USB PHY VBUS Detect Control Register"]
pub struct USB1_VBUS_DETECT_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_tog;
#[doc = "USB PHY VBUS Detector Status Register"]
pub struct USB1_VBUS_DET_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY VBUS Detector Status Register"]
pub mod usb1_vbus_det_stat;
#[doc = "USB PHY Charger Detect Status Register"]
pub struct USB1_CHRG_DET_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Charger Detect Status Register"]
pub mod usb1_chrg_det_stat;
#[doc = "USB PHY Analog Control Register"]
pub struct ANACTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl;
#[doc = "USB PHY Analog Control Register"]
pub struct ANACTRL_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_set;
#[doc = "USB PHY Analog Control Register"]
pub struct ANACTRL_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_clr;
#[doc = "USB PHY Analog Control Register"]
pub struct ANACTRL_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_tog;
#[doc = "USB PHY Loopback Control/Status Register"]
pub struct USB1_LOOPBACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Loopback Control/Status Register"]
pub mod usb1_loopback;
#[doc = "USB PHY Loopback Control/Status Register"]
pub struct USB1_LOOPBACK_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Loopback Control/Status Register"]
pub mod usb1_loopback_set;
#[doc = "USB PHY Loopback Control/Status Register"]
pub struct USB1_LOOPBACK_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Loopback Control/Status Register"]
pub mod usb1_loopback_clr;
#[doc = "USB PHY Loopback Control/Status Register"]
pub struct USB1_LOOPBACK_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Loopback Control/Status Register"]
pub mod usb1_loopback_tog;
#[doc = "USB PHY Loopback Packet Number Select Register"]
pub struct USB1_LOOPBACK_HSFSCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Loopback Packet Number Select Register"]
pub mod usb1_loopback_hsfscnt;
#[doc = "USB PHY Loopback Packet Number Select Register"]
pub struct USB1_LOOPBACK_HSFSCNT_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Loopback Packet Number Select Register"]
pub mod usb1_loopback_hsfscnt_set;
#[doc = "USB PHY Loopback Packet Number Select Register"]
pub struct USB1_LOOPBACK_HSFSCNT_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Loopback Packet Number Select Register"]
pub mod usb1_loopback_hsfscnt_clr;
#[doc = "USB PHY Loopback Packet Number Select Register"]
pub struct USB1_LOOPBACK_HSFSCNT_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Loopback Packet Number Select Register"]
pub mod usb1_loopback_hsfscnt_tog;
#[doc = "USB PHY Trim Override Enable Register"]
pub struct TRIM_OVERRIDE_EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Trim Override Enable Register"]
pub mod trim_override_en;
#[doc = "USB PHY Trim Override Enable Register"]
pub struct TRIM_OVERRIDE_EN_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Trim Override Enable Register"]
pub mod trim_override_en_set;
#[doc = "USB PHY Trim Override Enable Register"]
pub struct TRIM_OVERRIDE_EN_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Trim Override Enable Register"]
pub mod trim_override_en_clr;
#[doc = "USB PHY Trim Override Enable Register"]
pub struct TRIM_OVERRIDE_EN_TOG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB PHY Trim Override Enable Register"]
pub mod trim_override_en_tog;
