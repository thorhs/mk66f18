#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral ID register"]
    pub perid: PERID,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - Peripheral ID Complement register"]
    pub idcomp: IDCOMP,
    _reserved2: [u8; 3usize],
    #[doc = "0x08 - Peripheral Revision register"]
    pub rev: REV,
    _reserved3: [u8; 3usize],
    #[doc = "0x0c - Peripheral Additional Info register"]
    pub addinfo: ADDINFO,
    _reserved4: [u8; 3usize],
    #[doc = "0x10 - OTG Interrupt Status register"]
    pub otgistat: OTGISTAT,
    _reserved5: [u8; 3usize],
    #[doc = "0x14 - OTG Interrupt Control register"]
    pub otgicr: OTGICR,
    _reserved6: [u8; 3usize],
    #[doc = "0x18 - OTG Status register"]
    pub otgstat: OTGSTAT,
    _reserved7: [u8; 3usize],
    #[doc = "0x1c - OTG Control register"]
    pub otgctl: OTGCTL,
    _reserved8: [u8; 99usize],
    #[doc = "0x80 - Interrupt Status register"]
    pub istat: ISTAT,
    _reserved9: [u8; 3usize],
    #[doc = "0x84 - Interrupt Enable register"]
    pub inten: INTEN,
    _reserved10: [u8; 3usize],
    #[doc = "0x88 - Error Interrupt Status register"]
    pub errstat: ERRSTAT,
    _reserved11: [u8; 3usize],
    #[doc = "0x8c - Error Interrupt Enable register"]
    pub erren: ERREN,
    _reserved12: [u8; 3usize],
    #[doc = "0x90 - Status register"]
    pub stat: STAT,
    _reserved13: [u8; 3usize],
    #[doc = "0x94 - Control register"]
    pub ctl: CTL,
    _reserved14: [u8; 3usize],
    #[doc = "0x98 - Address register"]
    pub addr: ADDR,
    _reserved15: [u8; 3usize],
    #[doc = "0x9c - BDT Page register 1"]
    pub bdtpage1: BDTPAGE1,
    _reserved16: [u8; 3usize],
    #[doc = "0xa0 - Frame Number register Low"]
    pub frmnuml: FRMNUML,
    _reserved17: [u8; 3usize],
    #[doc = "0xa4 - Frame Number register High"]
    pub frmnumh: FRMNUMH,
    _reserved18: [u8; 3usize],
    #[doc = "0xa8 - Token register"]
    pub token: TOKEN,
    _reserved19: [u8; 3usize],
    #[doc = "0xac - SOF Threshold register"]
    pub softhld: SOFTHLD,
    _reserved20: [u8; 3usize],
    #[doc = "0xb0 - BDT Page Register 2"]
    pub bdtpage2: BDTPAGE2,
    _reserved21: [u8; 3usize],
    #[doc = "0xb4 - BDT Page Register 3"]
    pub bdtpage3: BDTPAGE3,
    _reserved22: [u8; 11usize],
    #[doc = "0xc0 - Endpoint Control register"]
    pub endpt0: ENDPT,
    _reserved23: [u8; 3usize],
    #[doc = "0xc4 - Endpoint Control register"]
    pub endpt1: ENDPT,
    _reserved24: [u8; 3usize],
    #[doc = "0xc8 - Endpoint Control register"]
    pub endpt2: ENDPT,
    _reserved25: [u8; 3usize],
    #[doc = "0xcc - Endpoint Control register"]
    pub endpt3: ENDPT,
    _reserved26: [u8; 3usize],
    #[doc = "0xd0 - Endpoint Control register"]
    pub endpt4: ENDPT,
    _reserved27: [u8; 3usize],
    #[doc = "0xd4 - Endpoint Control register"]
    pub endpt5: ENDPT,
    _reserved28: [u8; 3usize],
    #[doc = "0xd8 - Endpoint Control register"]
    pub endpt6: ENDPT,
    _reserved29: [u8; 3usize],
    #[doc = "0xdc - Endpoint Control register"]
    pub endpt7: ENDPT,
    _reserved30: [u8; 3usize],
    #[doc = "0xe0 - Endpoint Control register"]
    pub endpt8: ENDPT,
    _reserved31: [u8; 3usize],
    #[doc = "0xe4 - Endpoint Control register"]
    pub endpt9: ENDPT,
    _reserved32: [u8; 3usize],
    #[doc = "0xe8 - Endpoint Control register"]
    pub endpt10: ENDPT,
    _reserved33: [u8; 3usize],
    #[doc = "0xec - Endpoint Control register"]
    pub endpt11: ENDPT,
    _reserved34: [u8; 3usize],
    #[doc = "0xf0 - Endpoint Control register"]
    pub endpt12: ENDPT,
    _reserved35: [u8; 3usize],
    #[doc = "0xf4 - Endpoint Control register"]
    pub endpt13: ENDPT,
    _reserved36: [u8; 3usize],
    #[doc = "0xf8 - Endpoint Control register"]
    pub endpt14: ENDPT,
    _reserved37: [u8; 3usize],
    #[doc = "0xfc - Endpoint Control register"]
    pub endpt15: ENDPT,
    _reserved38: [u8; 3usize],
    #[doc = "0x100 - USB Control register"]
    pub usbctrl: USBCTRL,
    _reserved39: [u8; 3usize],
    #[doc = "0x104 - USB OTG Observe register"]
    pub observe: OBSERVE,
    _reserved40: [u8; 3usize],
    #[doc = "0x108 - USB OTG Control register"]
    pub control: CONTROL,
    _reserved41: [u8; 3usize],
    #[doc = "0x10c - USB Transceiver Control register 0"]
    pub usbtrc0: USBTRC0,
    _reserved42: [u8; 7usize],
    #[doc = "0x114 - Frame Adjust Register"]
    pub usbfrmadjust: USBFRMADJUST,
    _reserved43: [u8; 43usize],
    #[doc = "0x140 - USB Clock recovery control"]
    pub clk_recover_ctrl: CLK_RECOVER_CTRL,
    _reserved44: [u8; 3usize],
    #[doc = "0x144 - IRC48M oscillator enable register"]
    pub clk_recover_irc_en: CLK_RECOVER_IRC_EN,
    _reserved45: [u8; 15usize],
    #[doc = "0x154 - Clock recovery combined interrupt enable"]
    pub clk_recover_int_en: CLK_RECOVER_INT_EN,
    _reserved46: [u8; 7usize],
    #[doc = "0x15c - Clock recovery separated interrupt status"]
    pub clk_recover_int_status: CLK_RECOVER_INT_STATUS,
}
#[doc = "Peripheral ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [perid](perid) module"]
pub type PERID = crate::Reg<u8, _PERID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERID;
#[doc = "`read()` method returns [perid::R](perid::R) reader structure"]
impl crate::Readable for PERID {}
#[doc = "Peripheral ID register"]
pub mod perid;
#[doc = "Peripheral ID Complement register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idcomp](idcomp) module"]
pub type IDCOMP = crate::Reg<u8, _IDCOMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDCOMP;
#[doc = "`read()` method returns [idcomp::R](idcomp::R) reader structure"]
impl crate::Readable for IDCOMP {}
#[doc = "Peripheral ID Complement register"]
pub mod idcomp;
#[doc = "Peripheral Revision register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rev](rev) module"]
pub type REV = crate::Reg<u8, _REV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REV;
#[doc = "`read()` method returns [rev::R](rev::R) reader structure"]
impl crate::Readable for REV {}
#[doc = "Peripheral Revision register"]
pub mod rev;
#[doc = "Peripheral Additional Info register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addinfo](addinfo) module"]
pub type ADDINFO = crate::Reg<u8, _ADDINFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDINFO;
#[doc = "`read()` method returns [addinfo::R](addinfo::R) reader structure"]
impl crate::Readable for ADDINFO {}
#[doc = "Peripheral Additional Info register"]
pub mod addinfo;
#[doc = "OTG Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [otgistat](otgistat) module"]
pub type OTGISTAT = crate::Reg<u8, _OTGISTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTGISTAT;
#[doc = "`read()` method returns [otgistat::R](otgistat::R) reader structure"]
impl crate::Readable for OTGISTAT {}
#[doc = "`write(|w| ..)` method takes [otgistat::W](otgistat::W) writer structure"]
impl crate::Writable for OTGISTAT {}
#[doc = "OTG Interrupt Status register"]
pub mod otgistat;
#[doc = "OTG Interrupt Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [otgicr](otgicr) module"]
pub type OTGICR = crate::Reg<u8, _OTGICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTGICR;
#[doc = "`read()` method returns [otgicr::R](otgicr::R) reader structure"]
impl crate::Readable for OTGICR {}
#[doc = "`write(|w| ..)` method takes [otgicr::W](otgicr::W) writer structure"]
impl crate::Writable for OTGICR {}
#[doc = "OTG Interrupt Control register"]
pub mod otgicr;
#[doc = "OTG Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [otgstat](otgstat) module"]
pub type OTGSTAT = crate::Reg<u8, _OTGSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTGSTAT;
#[doc = "`read()` method returns [otgstat::R](otgstat::R) reader structure"]
impl crate::Readable for OTGSTAT {}
#[doc = "`write(|w| ..)` method takes [otgstat::W](otgstat::W) writer structure"]
impl crate::Writable for OTGSTAT {}
#[doc = "OTG Status register"]
pub mod otgstat;
#[doc = "OTG Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [otgctl](otgctl) module"]
pub type OTGCTL = crate::Reg<u8, _OTGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTGCTL;
#[doc = "`read()` method returns [otgctl::R](otgctl::R) reader structure"]
impl crate::Readable for OTGCTL {}
#[doc = "`write(|w| ..)` method takes [otgctl::W](otgctl::W) writer structure"]
impl crate::Writable for OTGCTL {}
#[doc = "OTG Control register"]
pub mod otgctl;
#[doc = "Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [istat](istat) module"]
pub type ISTAT = crate::Reg<u8, _ISTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISTAT;
#[doc = "`read()` method returns [istat::R](istat::R) reader structure"]
impl crate::Readable for ISTAT {}
#[doc = "`write(|w| ..)` method takes [istat::W](istat::W) writer structure"]
impl crate::Writable for ISTAT {}
#[doc = "Interrupt Status register"]
pub mod istat;
#[doc = "Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u8, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Interrupt Enable register"]
pub mod inten;
#[doc = "Error Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [errstat](errstat) module"]
pub type ERRSTAT = crate::Reg<u8, _ERRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRSTAT;
#[doc = "`read()` method returns [errstat::R](errstat::R) reader structure"]
impl crate::Readable for ERRSTAT {}
#[doc = "`write(|w| ..)` method takes [errstat::W](errstat::W) writer structure"]
impl crate::Writable for ERRSTAT {}
#[doc = "Error Interrupt Status register"]
pub mod errstat;
#[doc = "Error Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [erren](erren) module"]
pub type ERREN = crate::Reg<u8, _ERREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERREN;
#[doc = "`read()` method returns [erren::R](erren::R) reader structure"]
impl crate::Readable for ERREN {}
#[doc = "`write(|w| ..)` method takes [erren::W](erren::W) writer structure"]
impl crate::Writable for ERREN {}
#[doc = "Error Interrupt Enable register"]
pub mod erren;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat](stat) module"]
pub type STAT = crate::Reg<u8, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "Status register"]
pub mod stat;
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u8, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control register"]
pub mod ctl;
#[doc = "Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u8, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "Address register"]
pub mod addr;
#[doc = "BDT Page register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bdtpage1](bdtpage1) module"]
pub type BDTPAGE1 = crate::Reg<u8, _BDTPAGE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTPAGE1;
#[doc = "`read()` method returns [bdtpage1::R](bdtpage1::R) reader structure"]
impl crate::Readable for BDTPAGE1 {}
#[doc = "`write(|w| ..)` method takes [bdtpage1::W](bdtpage1::W) writer structure"]
impl crate::Writable for BDTPAGE1 {}
#[doc = "BDT Page register 1"]
pub mod bdtpage1;
#[doc = "Frame Number register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frmnuml](frmnuml) module"]
pub type FRMNUML = crate::Reg<u8, _FRMNUML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRMNUML;
#[doc = "`read()` method returns [frmnuml::R](frmnuml::R) reader structure"]
impl crate::Readable for FRMNUML {}
#[doc = "`write(|w| ..)` method takes [frmnuml::W](frmnuml::W) writer structure"]
impl crate::Writable for FRMNUML {}
#[doc = "Frame Number register Low"]
pub mod frmnuml;
#[doc = "Frame Number register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frmnumh](frmnumh) module"]
pub type FRMNUMH = crate::Reg<u8, _FRMNUMH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRMNUMH;
#[doc = "`read()` method returns [frmnumh::R](frmnumh::R) reader structure"]
impl crate::Readable for FRMNUMH {}
#[doc = "`write(|w| ..)` method takes [frmnumh::W](frmnumh::W) writer structure"]
impl crate::Writable for FRMNUMH {}
#[doc = "Frame Number register High"]
pub mod frmnumh;
#[doc = "Token register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [token](token) module"]
pub type TOKEN = crate::Reg<u8, _TOKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOKEN;
#[doc = "`read()` method returns [token::R](token::R) reader structure"]
impl crate::Readable for TOKEN {}
#[doc = "`write(|w| ..)` method takes [token::W](token::W) writer structure"]
impl crate::Writable for TOKEN {}
#[doc = "Token register"]
pub mod token;
#[doc = "SOF Threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [softhld](softhld) module"]
pub type SOFTHLD = crate::Reg<u8, _SOFTHLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOFTHLD;
#[doc = "`read()` method returns [softhld::R](softhld::R) reader structure"]
impl crate::Readable for SOFTHLD {}
#[doc = "`write(|w| ..)` method takes [softhld::W](softhld::W) writer structure"]
impl crate::Writable for SOFTHLD {}
#[doc = "SOF Threshold register"]
pub mod softhld;
#[doc = "BDT Page Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bdtpage2](bdtpage2) module"]
pub type BDTPAGE2 = crate::Reg<u8, _BDTPAGE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTPAGE2;
#[doc = "`read()` method returns [bdtpage2::R](bdtpage2::R) reader structure"]
impl crate::Readable for BDTPAGE2 {}
#[doc = "`write(|w| ..)` method takes [bdtpage2::W](bdtpage2::W) writer structure"]
impl crate::Writable for BDTPAGE2 {}
#[doc = "BDT Page Register 2"]
pub mod bdtpage2;
#[doc = "BDT Page Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bdtpage3](bdtpage3) module"]
pub type BDTPAGE3 = crate::Reg<u8, _BDTPAGE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTPAGE3;
#[doc = "`read()` method returns [bdtpage3::R](bdtpage3::R) reader structure"]
impl crate::Readable for BDTPAGE3 {}
#[doc = "`write(|w| ..)` method takes [bdtpage3::W](bdtpage3::W) writer structure"]
impl crate::Writable for BDTPAGE3 {}
#[doc = "BDT Page Register 3"]
pub mod bdtpage3;
#[doc = "Endpoint Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [endpt](endpt) module"]
pub type ENDPT = crate::Reg<u8, _ENDPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDPT;
#[doc = "`read()` method returns [endpt::R](endpt::R) reader structure"]
impl crate::Readable for ENDPT {}
#[doc = "`write(|w| ..)` method takes [endpt::W](endpt::W) writer structure"]
impl crate::Writable for ENDPT {}
#[doc = "Endpoint Control register"]
pub mod endpt;
#[doc = "USB Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbctrl](usbctrl) module"]
pub type USBCTRL = crate::Reg<u8, _USBCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCTRL;
#[doc = "`read()` method returns [usbctrl::R](usbctrl::R) reader structure"]
impl crate::Readable for USBCTRL {}
#[doc = "`write(|w| ..)` method takes [usbctrl::W](usbctrl::W) writer structure"]
impl crate::Writable for USBCTRL {}
#[doc = "USB Control register"]
pub mod usbctrl;
#[doc = "USB OTG Observe register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [observe](observe) module"]
pub type OBSERVE = crate::Reg<u8, _OBSERVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSERVE;
#[doc = "`read()` method returns [observe::R](observe::R) reader structure"]
impl crate::Readable for OBSERVE {}
#[doc = "USB OTG Observe register"]
pub mod observe;
#[doc = "USB OTG Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [control](control) module"]
pub type CONTROL = crate::Reg<u8, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "USB OTG Control register"]
pub mod control;
#[doc = "USB Transceiver Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbtrc0](usbtrc0) module"]
pub type USBTRC0 = crate::Reg<u8, _USBTRC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBTRC0;
#[doc = "`read()` method returns [usbtrc0::R](usbtrc0::R) reader structure"]
impl crate::Readable for USBTRC0 {}
#[doc = "`write(|w| ..)` method takes [usbtrc0::W](usbtrc0::W) writer structure"]
impl crate::Writable for USBTRC0 {}
#[doc = "USB Transceiver Control register 0"]
pub mod usbtrc0;
#[doc = "Frame Adjust Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbfrmadjust](usbfrmadjust) module"]
pub type USBFRMADJUST = crate::Reg<u8, _USBFRMADJUST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBFRMADJUST;
#[doc = "`read()` method returns [usbfrmadjust::R](usbfrmadjust::R) reader structure"]
impl crate::Readable for USBFRMADJUST {}
#[doc = "`write(|w| ..)` method takes [usbfrmadjust::W](usbfrmadjust::W) writer structure"]
impl crate::Writable for USBFRMADJUST {}
#[doc = "Frame Adjust Register"]
pub mod usbfrmadjust;
#[doc = "USB Clock recovery control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_recover_ctrl](clk_recover_ctrl) module"]
pub type CLK_RECOVER_CTRL = crate::Reg<u8, _CLK_RECOVER_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_RECOVER_CTRL;
#[doc = "`read()` method returns [clk_recover_ctrl::R](clk_recover_ctrl::R) reader structure"]
impl crate::Readable for CLK_RECOVER_CTRL {}
#[doc = "`write(|w| ..)` method takes [clk_recover_ctrl::W](clk_recover_ctrl::W) writer structure"]
impl crate::Writable for CLK_RECOVER_CTRL {}
#[doc = "USB Clock recovery control"]
pub mod clk_recover_ctrl;
#[doc = "IRC48M oscillator enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_recover_irc_en](clk_recover_irc_en) module"]
pub type CLK_RECOVER_IRC_EN = crate::Reg<u8, _CLK_RECOVER_IRC_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_RECOVER_IRC_EN;
#[doc = "`read()` method returns [clk_recover_irc_en::R](clk_recover_irc_en::R) reader structure"]
impl crate::Readable for CLK_RECOVER_IRC_EN {}
#[doc = "`write(|w| ..)` method takes [clk_recover_irc_en::W](clk_recover_irc_en::W) writer structure"]
impl crate::Writable for CLK_RECOVER_IRC_EN {}
#[doc = "IRC48M oscillator enable register"]
pub mod clk_recover_irc_en;
#[doc = "Clock recovery combined interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_recover_int_en](clk_recover_int_en) module"]
pub type CLK_RECOVER_INT_EN = crate::Reg<u8, _CLK_RECOVER_INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_RECOVER_INT_EN;
#[doc = "`read()` method returns [clk_recover_int_en::R](clk_recover_int_en::R) reader structure"]
impl crate::Readable for CLK_RECOVER_INT_EN {}
#[doc = "`write(|w| ..)` method takes [clk_recover_int_en::W](clk_recover_int_en::W) writer structure"]
impl crate::Writable for CLK_RECOVER_INT_EN {}
#[doc = "Clock recovery combined interrupt enable"]
pub mod clk_recover_int_en;
#[doc = "Clock recovery separated interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_recover_int_status](clk_recover_int_status) module"]
pub type CLK_RECOVER_INT_STATUS = crate::Reg<u8, _CLK_RECOVER_INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_RECOVER_INT_STATUS;
#[doc = "`read()` method returns [clk_recover_int_status::R](clk_recover_int_status::R) reader structure"]
impl crate::Readable for CLK_RECOVER_INT_STATUS {}
#[doc = "`write(|w| ..)` method takes [clk_recover_int_status::W](clk_recover_int_status::W) writer structure"]
impl crate::Writable for CLK_RECOVER_INT_STATUS {}
#[doc = "Clock recovery separated interrupt status"]
pub mod clk_recover_int_status;
