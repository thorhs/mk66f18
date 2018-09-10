#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub control: CONTROL,
    #[doc = "0x04 - Clock register"]
    pub clock: CLOCK,
    #[doc = "0x08 - Status register"]
    pub status: STATUS,
    #[doc = "0x0c - Signal Override Register"]
    pub signal_override: SIGNAL_OVERRIDE,
    #[doc = "0x10 - TIMER0 register"]
    pub timer0: TIMER0,
    #[doc = "0x14 - TIMER1 register"]
    pub timer1: TIMER1,
    #[doc = "0x18 - TIMER2_BC11 register"]
    pub timer2_bc11: TIMER2_BC11,
}
#[doc = "Control register"]
pub struct CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod control;
#[doc = "Clock register"]
pub struct CLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock register"]
pub mod clock;
#[doc = "Status register"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod status;
#[doc = "Signal Override Register"]
pub struct SIGNAL_OVERRIDE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Signal Override Register"]
pub mod signal_override;
#[doc = "TIMER0 register"]
pub struct TIMER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMER0 register"]
pub mod timer0;
#[doc = "TIMER1 register"]
pub struct TIMER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMER1 register"]
pub mod timer1;
#[doc = "TIMER2_BC11 register"]
pub struct TIMER2_BC11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMER2_BC11 register"]
pub mod timer2_bc11;
#[doc = "TIMER2_BC12 register"]
pub struct TIMER2_BC12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TIMER2_BC12 register"]
pub mod timer2_bc12;
