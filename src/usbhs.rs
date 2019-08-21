#[doc = r"Register block"]
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
    _reserved6: [u8; 104usize],
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
    _reserved11: [u8; 108usize],
    #[doc = "0x100 - Host Controller Interface Version and Capability Registers Length Register"]
    pub hciversion: HCIVERSION,
    #[doc = "0x104 - Host Controller Structural Parameters Register"]
    pub hcsparams: HCSPARAMS,
    #[doc = "0x108 - Host Controller Capability Parameters Register"]
    pub hccparams: HCCPARAMS,
    _reserved14: [u8; 22usize],
    #[doc = "0x122 - Device Controller Interface Version"]
    pub dciversion: DCIVERSION,
    #[doc = "0x124 - Device Controller Capability Parameters"]
    pub dccparams: DCCPARAMS,
    _reserved16: [u8; 24usize],
    #[doc = "0x140 - USB Command Register"]
    pub usbcmd: USBCMD,
    #[doc = "0x144 - USB Status Register"]
    pub usbsts: USBSTS,
    #[doc = "0x148 - USB Interrupt Enable Register"]
    pub usbintr: USBINTR,
    #[doc = "0x14c - Frame Index Register"]
    pub frindex: FRINDEX,
    _reserved20: [u8; 4usize],
    _reserved_20_deviceaddr: [u8; 4usize],
    _reserved_21_eplistaddr: [u8; 4usize],
    #[doc = "0x15c - Host TT Asynchronous Buffer Control"]
    pub ttctrl: TTCTRL,
    #[doc = "0x160 - Master Interface Data Burst Size Register"]
    pub burstsize: BURSTSIZE,
    #[doc = "0x164 - Transmit FIFO Tuning Control Register"]
    pub txfilltuning: TXFILLTUNING,
    _reserved25: [u8; 16usize],
    #[doc = "0x178 - Endpoint NAK Register"]
    pub endptnak: ENDPTNAK,
    #[doc = "0x17c - Endpoint NAK Enable Register"]
    pub endptnaken: ENDPTNAKEN,
    #[doc = "0x180 - Configure Flag Register"]
    pub configflag: CONFIGFLAG,
    #[doc = "0x184 - Port Status and Control Registers"]
    pub portsc1: PORTSC1,
    _reserved29: [u8; 28usize],
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
    _reserved44: [u8; 32usize],
    #[doc = "0x200 - USB General Control Register"]
    pub usbgenctrl: USBGENCTRL,
}
impl RegisterBlock {
    #[doc = "0x154 - Periodic Frame List Base Address Register"]
    #[inline(always)]
    pub fn periodiclistbase(&self) -> &PERIODICLISTBASE {
        unsafe { &*(((self as *const Self) as *const u8).add(340usize) as *const PERIODICLISTBASE) }
    }
    #[doc = "0x154 - Periodic Frame List Base Address Register"]
    #[inline(always)]
    pub fn periodiclistbase_mut(&self) -> &mut PERIODICLISTBASE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(340usize) as *mut PERIODICLISTBASE) }
    }
    #[doc = "0x154 - Device Address Register"]
    #[inline(always)]
    pub fn deviceaddr(&self) -> &DEVICEADDR {
        unsafe { &*(((self as *const Self) as *const u8).add(340usize) as *const DEVICEADDR) }
    }
    #[doc = "0x154 - Device Address Register"]
    #[inline(always)]
    pub fn deviceaddr_mut(&self) -> &mut DEVICEADDR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(340usize) as *mut DEVICEADDR) }
    }
    #[doc = "0x158 - Endpoint List Address Register"]
    #[inline(always)]
    pub fn eplistaddr(&self) -> &EPLISTADDR {
        unsafe { &*(((self as *const Self) as *const u8).add(344usize) as *const EPLISTADDR) }
    }
    #[doc = "0x158 - Endpoint List Address Register"]
    #[inline(always)]
    pub fn eplistaddr_mut(&self) -> &mut EPLISTADDR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(344usize) as *mut EPLISTADDR) }
    }
    #[doc = "0x158 - Current Asynchronous List Address Register"]
    #[inline(always)]
    pub fn asynclistaddr(&self) -> &ASYNCLISTADDR {
        unsafe { &*(((self as *const Self) as *const u8).add(344usize) as *const ASYNCLISTADDR) }
    }
    #[doc = "0x158 - Current Asynchronous List Address Register"]
    #[inline(always)]
    pub fn asynclistaddr_mut(&self) -> &mut ASYNCLISTADDR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(344usize) as *mut ASYNCLISTADDR) }
    }
}
#[doc = "Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "Identification Register"]
pub mod id;
#[doc = "General Hardware Parameters Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hwgeneral](hwgeneral) module"]
pub type HWGENERAL = crate::Reg<u32, _HWGENERAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWGENERAL;
#[doc = "`read()` method returns [hwgeneral::R](hwgeneral::R) reader structure"]
impl crate::Readable for HWGENERAL {}
#[doc = "General Hardware Parameters Register"]
pub mod hwgeneral;
#[doc = "Host Hardware Parameters Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hwhost](hwhost) module"]
pub type HWHOST = crate::Reg<u32, _HWHOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWHOST;
#[doc = "`read()` method returns [hwhost::R](hwhost::R) reader structure"]
impl crate::Readable for HWHOST {}
#[doc = "Host Hardware Parameters Register"]
pub mod hwhost;
#[doc = "Device Hardware Parameters Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hwdevice](hwdevice) module"]
pub type HWDEVICE = crate::Reg<u32, _HWDEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWDEVICE;
#[doc = "`read()` method returns [hwdevice::R](hwdevice::R) reader structure"]
impl crate::Readable for HWDEVICE {}
#[doc = "Device Hardware Parameters Register"]
pub mod hwdevice;
#[doc = "Transmit Buffer Hardware Parameters Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hwtxbuf](hwtxbuf) module"]
pub type HWTXBUF = crate::Reg<u32, _HWTXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWTXBUF;
#[doc = "`read()` method returns [hwtxbuf::R](hwtxbuf::R) reader structure"]
impl crate::Readable for HWTXBUF {}
#[doc = "Transmit Buffer Hardware Parameters Register"]
pub mod hwtxbuf;
#[doc = "Receive Buffer Hardware Parameters Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hwrxbuf](hwrxbuf) module"]
pub type HWRXBUF = crate::Reg<u32, _HWRXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWRXBUF;
#[doc = "`read()` method returns [hwrxbuf::R](hwrxbuf::R) reader structure"]
impl crate::Readable for HWRXBUF {}
#[doc = "Receive Buffer Hardware Parameters Register"]
pub mod hwrxbuf;
#[doc = "General Purpose Timer n Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gptimerld](gptimerld) module"]
pub type GPTIMERLD = crate::Reg<u32, _GPTIMERLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTIMERLD;
#[doc = "`read()` method returns [gptimerld::R](gptimerld::R) reader structure"]
impl crate::Readable for GPTIMERLD {}
#[doc = "`write(|w| ..)` method takes [gptimerld::W](gptimerld::W) writer structure"]
impl crate::Writable for GPTIMERLD {}
#[doc = "General Purpose Timer n Load Register"]
pub mod gptimerld;
#[doc = "General Purpose Timer n Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gptimerctl](gptimerctl) module"]
pub type GPTIMERCTL = crate::Reg<u32, _GPTIMERCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPTIMERCTL;
#[doc = "`read()` method returns [gptimerctl::R](gptimerctl::R) reader structure"]
impl crate::Readable for GPTIMERCTL {}
#[doc = "`write(|w| ..)` method takes [gptimerctl::W](gptimerctl::W) writer structure"]
impl crate::Writable for GPTIMERCTL {}
#[doc = "General Purpose Timer n Control Register"]
pub mod gptimerctl;
#[doc = "System Bus Interface Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_sbuscfg](usb_sbuscfg) module"]
pub type USB_SBUSCFG = crate::Reg<u32, _USB_SBUSCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_SBUSCFG;
#[doc = "`read()` method returns [usb_sbuscfg::R](usb_sbuscfg::R) reader structure"]
impl crate::Readable for USB_SBUSCFG {}
#[doc = "`write(|w| ..)` method takes [usb_sbuscfg::W](usb_sbuscfg::W) writer structure"]
impl crate::Writable for USB_SBUSCFG {}
#[doc = "System Bus Interface Configuration Register"]
pub mod usb_sbuscfg;
#[doc = "Host Controller Interface Version and Capability Registers Length Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hciversion](hciversion) module"]
pub type HCIVERSION = crate::Reg<u32, _HCIVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCIVERSION;
#[doc = "`read()` method returns [hciversion::R](hciversion::R) reader structure"]
impl crate::Readable for HCIVERSION {}
#[doc = "Host Controller Interface Version and Capability Registers Length Register"]
pub mod hciversion;
#[doc = "Host Controller Structural Parameters Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hcsparams](hcsparams) module"]
pub type HCSPARAMS = crate::Reg<u32, _HCSPARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCSPARAMS;
#[doc = "`read()` method returns [hcsparams::R](hcsparams::R) reader structure"]
impl crate::Readable for HCSPARAMS {}
#[doc = "Host Controller Structural Parameters Register"]
pub mod hcsparams;
#[doc = "Host Controller Capability Parameters Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hccparams](hccparams) module"]
pub type HCCPARAMS = crate::Reg<u32, _HCCPARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCPARAMS;
#[doc = "`read()` method returns [hccparams::R](hccparams::R) reader structure"]
impl crate::Readable for HCCPARAMS {}
#[doc = "Host Controller Capability Parameters Register"]
pub mod hccparams;
#[doc = "Device Controller Interface Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dciversion](dciversion) module"]
pub type DCIVERSION = crate::Reg<u16, _DCIVERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCIVERSION;
#[doc = "`read()` method returns [dciversion::R](dciversion::R) reader structure"]
impl crate::Readable for DCIVERSION {}
#[doc = "Device Controller Interface Version"]
pub mod dciversion;
#[doc = "Device Controller Capability Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dccparams](dccparams) module"]
pub type DCCPARAMS = crate::Reg<u32, _DCCPARAMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCCPARAMS;
#[doc = "`read()` method returns [dccparams::R](dccparams::R) reader structure"]
impl crate::Readable for DCCPARAMS {}
#[doc = "Device Controller Capability Parameters"]
pub mod dccparams;
#[doc = "USB Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbcmd](usbcmd) module"]
pub type USBCMD = crate::Reg<u32, _USBCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCMD;
#[doc = "`read()` method returns [usbcmd::R](usbcmd::R) reader structure"]
impl crate::Readable for USBCMD {}
#[doc = "`write(|w| ..)` method takes [usbcmd::W](usbcmd::W) writer structure"]
impl crate::Writable for USBCMD {}
#[doc = "USB Command Register"]
pub mod usbcmd;
#[doc = "USB Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbsts](usbsts) module"]
pub type USBSTS = crate::Reg<u32, _USBSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBSTS;
#[doc = "`read()` method returns [usbsts::R](usbsts::R) reader structure"]
impl crate::Readable for USBSTS {}
#[doc = "`write(|w| ..)` method takes [usbsts::W](usbsts::W) writer structure"]
impl crate::Writable for USBSTS {}
#[doc = "USB Status Register"]
pub mod usbsts;
#[doc = "USB Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbintr](usbintr) module"]
pub type USBINTR = crate::Reg<u32, _USBINTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBINTR;
#[doc = "`read()` method returns [usbintr::R](usbintr::R) reader structure"]
impl crate::Readable for USBINTR {}
#[doc = "`write(|w| ..)` method takes [usbintr::W](usbintr::W) writer structure"]
impl crate::Writable for USBINTR {}
#[doc = "USB Interrupt Enable Register"]
pub mod usbintr;
#[doc = "Frame Index Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frindex](frindex) module"]
pub type FRINDEX = crate::Reg<u32, _FRINDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRINDEX;
#[doc = "`read()` method returns [frindex::R](frindex::R) reader structure"]
impl crate::Readable for FRINDEX {}
#[doc = "`write(|w| ..)` method takes [frindex::W](frindex::W) writer structure"]
impl crate::Writable for FRINDEX {}
#[doc = "Frame Index Register"]
pub mod frindex;
#[doc = "Device Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deviceaddr](deviceaddr) module"]
pub type DEVICEADDR = crate::Reg<u32, _DEVICEADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICEADDR;
#[doc = "`read()` method returns [deviceaddr::R](deviceaddr::R) reader structure"]
impl crate::Readable for DEVICEADDR {}
#[doc = "`write(|w| ..)` method takes [deviceaddr::W](deviceaddr::W) writer structure"]
impl crate::Writable for DEVICEADDR {}
#[doc = "Device Address Register"]
pub mod deviceaddr;
#[doc = "Periodic Frame List Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [periodiclistbase](periodiclistbase) module"]
pub type PERIODICLISTBASE = crate::Reg<u32, _PERIODICLISTBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIODICLISTBASE;
#[doc = "`read()` method returns [periodiclistbase::R](periodiclistbase::R) reader structure"]
impl crate::Readable for PERIODICLISTBASE {}
#[doc = "`write(|w| ..)` method takes [periodiclistbase::W](periodiclistbase::W) writer structure"]
impl crate::Writable for PERIODICLISTBASE {}
#[doc = "Periodic Frame List Base Address Register"]
pub mod periodiclistbase;
#[doc = "Current Asynchronous List Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [asynclistaddr](asynclistaddr) module"]
pub type ASYNCLISTADDR = crate::Reg<u32, _ASYNCLISTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASYNCLISTADDR;
#[doc = "`read()` method returns [asynclistaddr::R](asynclistaddr::R) reader structure"]
impl crate::Readable for ASYNCLISTADDR {}
#[doc = "`write(|w| ..)` method takes [asynclistaddr::W](asynclistaddr::W) writer structure"]
impl crate::Writable for ASYNCLISTADDR {}
#[doc = "Current Asynchronous List Address Register"]
pub mod asynclistaddr;
#[doc = "Endpoint List Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eplistaddr](eplistaddr) module"]
pub type EPLISTADDR = crate::Reg<u32, _EPLISTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPLISTADDR;
#[doc = "`read()` method returns [eplistaddr::R](eplistaddr::R) reader structure"]
impl crate::Readable for EPLISTADDR {}
#[doc = "`write(|w| ..)` method takes [eplistaddr::W](eplistaddr::W) writer structure"]
impl crate::Writable for EPLISTADDR {}
#[doc = "Endpoint List Address Register"]
pub mod eplistaddr;
#[doc = "Host TT Asynchronous Buffer Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ttctrl](ttctrl) module"]
pub type TTCTRL = crate::Reg<u32, _TTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTCTRL;
#[doc = "`read()` method returns [ttctrl::R](ttctrl::R) reader structure"]
impl crate::Readable for TTCTRL {}
#[doc = "Host TT Asynchronous Buffer Control"]
pub mod ttctrl;
#[doc = "Master Interface Data Burst Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [burstsize](burstsize) module"]
pub type BURSTSIZE = crate::Reg<u32, _BURSTSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BURSTSIZE;
#[doc = "`read()` method returns [burstsize::R](burstsize::R) reader structure"]
impl crate::Readable for BURSTSIZE {}
#[doc = "`write(|w| ..)` method takes [burstsize::W](burstsize::W) writer structure"]
impl crate::Writable for BURSTSIZE {}
#[doc = "Master Interface Data Burst Size Register"]
pub mod burstsize;
#[doc = "Transmit FIFO Tuning Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txfilltuning](txfilltuning) module"]
pub type TXFILLTUNING = crate::Reg<u32, _TXFILLTUNING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFILLTUNING;
#[doc = "`read()` method returns [txfilltuning::R](txfilltuning::R) reader structure"]
impl crate::Readable for TXFILLTUNING {}
#[doc = "`write(|w| ..)` method takes [txfilltuning::W](txfilltuning::W) writer structure"]
impl crate::Writable for TXFILLTUNING {}
#[doc = "Transmit FIFO Tuning Control Register"]
pub mod txfilltuning;
#[doc = "Endpoint NAK Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [endptnak](endptnak) module"]
pub type ENDPTNAK = crate::Reg<u32, _ENDPTNAK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTNAK;
#[doc = "`read()` method returns [endptnak::R](endptnak::R) reader structure"]
impl crate::Readable for ENDPTNAK {}
#[doc = "`write(|w| ..)` method takes [endptnak::W](endptnak::W) writer structure"]
impl crate::Writable for ENDPTNAK {}
#[doc = "Endpoint NAK Register"]
pub mod endptnak;
#[doc = "Endpoint NAK Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [endptnaken](endptnaken) module"]
pub type ENDPTNAKEN = crate::Reg<u32, _ENDPTNAKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPTNAKEN;
#[doc = "`read()` method returns [endptnaken::R](endptnaken::R) reader structure"]
impl crate::Readable for ENDPTNAKEN {}
#[doc = "`write(|w| ..)` method takes [endptnaken::W](endptnaken::W) writer structure"]
impl crate::Writable for ENDPTNAKEN {}
#[doc = "Endpoint NAK Enable Register"]
pub mod endptnaken;
#[doc = "Configure Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [configflag](configflag) module"]
pub type CONFIGFLAG = crate::Reg<u32, _CONFIGFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIGFLAG;
#[doc = "`read()` method returns [configflag::R](configflag::R) reader structure"]
impl crate::Readable for CONFIGFLAG {}
#[doc = "Configure Flag Register"]
pub mod configflag;
#[doc = "Port Status and Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [portsc1](portsc1) module"]
pub type PORTSC1 = crate::Reg<u32, _PORTSC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTSC1;
#[doc = "`read()` method returns [portsc1::R](portsc1::R) reader structure"]
impl crate::Readable for PORTSC1 {}
#[doc = "`write(|w| ..)` method takes [portsc1::W](portsc1::W) writer structure"]
impl crate::Writable for PORTSC1 {}
#[doc = "Port Status and Control Registers"]
pub mod portsc1;
#[doc = "On-the-Go Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [otgsc](otgsc) module"]
pub type OTGSC = crate::Reg<u32, _OTGSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTGSC;
#[doc = "`read()` method returns [otgsc::R](otgsc::R) reader structure"]
impl crate::Readable for OTGSC {}
#[doc = "`write(|w| ..)` method takes [otgsc::W](otgsc::W) writer structure"]
impl crate::Writable for OTGSC {}
#[doc = "On-the-Go Status and Control Register"]
pub mod otgsc;
#[doc = "USB Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbmode](usbmode) module"]
pub type USBMODE = crate::Reg<u32, _USBMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBMODE;
#[doc = "`read()` method returns [usbmode::R](usbmode::R) reader structure"]
impl crate::Readable for USBMODE {}
#[doc = "`write(|w| ..)` method takes [usbmode::W](usbmode::W) writer structure"]
impl crate::Writable for USBMODE {}
#[doc = "USB Mode Register"]
pub mod usbmode;
#[doc = "Endpoint Setup Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [epsetupsr](epsetupsr) module"]
pub type EPSETUPSR = crate::Reg<u32, _EPSETUPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPSETUPSR;
#[doc = "`read()` method returns [epsetupsr::R](epsetupsr::R) reader structure"]
impl crate::Readable for EPSETUPSR {}
#[doc = "`write(|w| ..)` method takes [epsetupsr::W](epsetupsr::W) writer structure"]
impl crate::Writable for EPSETUPSR {}
#[doc = "Endpoint Setup Status Register"]
pub mod epsetupsr;
#[doc = "Endpoint Initialization Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [epprime](epprime) module"]
pub type EPPRIME = crate::Reg<u32, _EPPRIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPPRIME;
#[doc = "`read()` method returns [epprime::R](epprime::R) reader structure"]
impl crate::Readable for EPPRIME {}
#[doc = "`write(|w| ..)` method takes [epprime::W](epprime::W) writer structure"]
impl crate::Writable for EPPRIME {}
#[doc = "Endpoint Initialization Register"]
pub mod epprime;
#[doc = "Endpoint Flush Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [epflush](epflush) module"]
pub type EPFLUSH = crate::Reg<u32, _EPFLUSH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPFLUSH;
#[doc = "`read()` method returns [epflush::R](epflush::R) reader structure"]
impl crate::Readable for EPFLUSH {}
#[doc = "`write(|w| ..)` method takes [epflush::W](epflush::W) writer structure"]
impl crate::Writable for EPFLUSH {}
#[doc = "Endpoint Flush Register"]
pub mod epflush;
#[doc = "Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [epsr](epsr) module"]
pub type EPSR = crate::Reg<u32, _EPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPSR;
#[doc = "`read()` method returns [epsr::R](epsr::R) reader structure"]
impl crate::Readable for EPSR {}
#[doc = "Endpoint Status Register"]
pub mod epsr;
#[doc = "Endpoint Complete Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [epcomplete](epcomplete) module"]
pub type EPCOMPLETE = crate::Reg<u32, _EPCOMPLETE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPCOMPLETE;
#[doc = "`read()` method returns [epcomplete::R](epcomplete::R) reader structure"]
impl crate::Readable for EPCOMPLETE {}
#[doc = "`write(|w| ..)` method takes [epcomplete::W](epcomplete::W) writer structure"]
impl crate::Writable for EPCOMPLETE {}
#[doc = "Endpoint Complete Register"]
pub mod epcomplete;
#[doc = "Endpoint Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [epcr0](epcr0) module"]
pub type EPCR0 = crate::Reg<u32, _EPCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPCR0;
#[doc = "`read()` method returns [epcr0::R](epcr0::R) reader structure"]
impl crate::Readable for EPCR0 {}
#[doc = "`write(|w| ..)` method takes [epcr0::W](epcr0::W) writer structure"]
impl crate::Writable for EPCR0 {}
#[doc = "Endpoint Control Register 0"]
pub mod epcr0;
#[doc = "Endpoint Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [epcr](epcr) module"]
pub type EPCR = crate::Reg<u32, _EPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPCR;
#[doc = "`read()` method returns [epcr::R](epcr::R) reader structure"]
impl crate::Readable for EPCR {}
#[doc = "`write(|w| ..)` method takes [epcr::W](epcr::W) writer structure"]
impl crate::Writable for EPCR {}
#[doc = "Endpoint Control Register n"]
pub mod epcr;
#[doc = "USB General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbgenctrl](usbgenctrl) module"]
pub type USBGENCTRL = crate::Reg<u32, _USBGENCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBGENCTRL;
#[doc = "`read()` method returns [usbgenctrl::R](usbgenctrl::R) reader structure"]
impl crate::Readable for USBGENCTRL {}
#[doc = "`write(|w| ..)` method takes [usbgenctrl::W](usbgenctrl::W) writer structure"]
impl crate::Writable for USBGENCTRL {}
#[doc = "USB General Control Register"]
pub mod usbgenctrl;
