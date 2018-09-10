#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Status and Control Register High"]
    pub stctrlh: STCTRLH,
    #[doc = "0x02 - Watchdog Status and Control Register Low"]
    pub stctrll: STCTRLL,
    #[doc = "0x04 - Watchdog Time-out Value Register High"]
    pub tovalh: TOVALH,
    #[doc = "0x06 - Watchdog Time-out Value Register Low"]
    pub tovall: TOVALL,
    #[doc = "0x08 - Watchdog Window Register High"]
    pub winh: WINH,
    #[doc = "0x0a - Watchdog Window Register Low"]
    pub winl: WINL,
    #[doc = "0x0c - Watchdog Refresh register"]
    pub refresh: REFRESH,
    #[doc = "0x0e - Watchdog Unlock register"]
    pub unlock: UNLOCK,
    #[doc = "0x10 - Watchdog Timer Output Register High"]
    pub tmrouth: TMROUTH,
    #[doc = "0x12 - Watchdog Timer Output Register Low"]
    pub tmroutl: TMROUTL,
    #[doc = "0x14 - Watchdog Reset Count register"]
    pub rstcnt: RSTCNT,
    #[doc = "0x16 - Watchdog Prescaler register"]
    pub presc: PRESC,
}
#[doc = "Watchdog Status and Control Register High"]
pub struct STCTRLH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Status and Control Register High"]
pub mod stctrlh;
#[doc = "Watchdog Status and Control Register Low"]
pub struct STCTRLL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Status and Control Register Low"]
pub mod stctrll;
#[doc = "Watchdog Time-out Value Register High"]
pub struct TOVALH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Time-out Value Register High"]
pub mod tovalh;
#[doc = "Watchdog Time-out Value Register Low"]
pub struct TOVALL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Time-out Value Register Low"]
pub mod tovall;
#[doc = "Watchdog Window Register High"]
pub struct WINH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Window Register High"]
pub mod winh;
#[doc = "Watchdog Window Register Low"]
pub struct WINL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Window Register Low"]
pub mod winl;
#[doc = "Watchdog Refresh register"]
pub struct REFRESH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Refresh register"]
pub mod refresh;
#[doc = "Watchdog Unlock register"]
pub struct UNLOCK {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Unlock register"]
pub mod unlock;
#[doc = "Watchdog Timer Output Register High"]
pub struct TMROUTH {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Timer Output Register High"]
pub mod tmrouth;
#[doc = "Watchdog Timer Output Register Low"]
pub struct TMROUTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Timer Output Register Low"]
pub mod tmroutl;
#[doc = "Watchdog Reset Count register"]
pub struct RSTCNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Reset Count register"]
pub mod rstcnt;
#[doc = "Watchdog Prescaler register"]
pub struct PRESC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Prescaler register"]
pub mod presc;
