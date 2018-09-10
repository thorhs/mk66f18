#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Time Seconds Register"]
    pub tsr: TSR,
    #[doc = "0x04 - RTC Time Prescaler Register"]
    pub tpr: TPR,
    #[doc = "0x08 - RTC Time Alarm Register"]
    pub tar: TAR,
    #[doc = "0x0c - RTC Time Compensation Register"]
    pub tcr: TCR,
    #[doc = "0x10 - RTC Control Register"]
    pub cr: CR,
    #[doc = "0x14 - RTC Status Register"]
    pub sr: SR,
    #[doc = "0x18 - RTC Lock Register"]
    pub lr: LR,
    #[doc = "0x1c - RTC Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x20 - RTC Tamper Time Seconds Register"]
    pub ttsr: TTSR,
    #[doc = "0x24 - RTC Monotonic Enable Register"]
    pub mer: MER,
    #[doc = "0x28 - RTC Monotonic Counter Low Register"]
    pub mclr: MCLR,
    #[doc = "0x2c - RTC Monotonic Counter High Register"]
    pub mchr: MCHR,
    _reserved0: [u8; 2000usize],
    #[doc = "0x800 - RTC Write Access Register"]
    pub war: WAR,
    #[doc = "0x804 - RTC Read Access Register"]
    pub rar: RAR,
}
#[doc = "RTC Time Seconds Register"]
pub struct TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Time Seconds Register"]
pub mod tsr;
#[doc = "RTC Time Prescaler Register"]
pub struct TPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Time Prescaler Register"]
pub mod tpr;
#[doc = "RTC Time Alarm Register"]
pub struct TAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Time Alarm Register"]
pub mod tar;
#[doc = "RTC Time Compensation Register"]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Time Compensation Register"]
pub mod tcr;
#[doc = "RTC Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Control Register"]
pub mod cr;
#[doc = "RTC Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Status Register"]
pub mod sr;
#[doc = "RTC Lock Register"]
pub struct LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Lock Register"]
pub mod lr;
#[doc = "RTC Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Interrupt Enable Register"]
pub mod ier;
#[doc = "RTC Tamper Time Seconds Register"]
pub struct TTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Tamper Time Seconds Register"]
pub mod ttsr;
#[doc = "RTC Monotonic Enable Register"]
pub struct MER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Monotonic Enable Register"]
pub mod mer;
#[doc = "RTC Monotonic Counter Low Register"]
pub struct MCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Monotonic Counter Low Register"]
pub mod mclr;
#[doc = "RTC Monotonic Counter High Register"]
pub struct MCHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Monotonic Counter High Register"]
pub mod mchr;
#[doc = "RTC Write Access Register"]
pub struct WAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Write Access Register"]
pub mod war;
#[doc = "RTC Read Access Register"]
pub struct RAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Read Access Register"]
pub mod rar;
