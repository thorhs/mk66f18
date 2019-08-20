#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA System Address register"]
    pub dsaddr: DSADDR,
    #[doc = "0x04 - Block Attributes register"]
    pub blkattr: BLKATTR,
    #[doc = "0x08 - Command Argument register"]
    pub cmdarg: CMDARG,
    #[doc = "0x0c - Transfer Type register"]
    pub xfertyp: XFERTYP,
    #[doc = "0x10 - Command Response 0"]
    pub cmdrsp0: CMDRSP0,
    #[doc = "0x14 - Command Response 1"]
    pub cmdrsp1: CMDRSP1,
    #[doc = "0x18 - Command Response 2"]
    pub cmdrsp2: CMDRSP2,
    #[doc = "0x1c - Command Response 3"]
    pub cmdrsp3: CMDRSP3,
    #[doc = "0x20 - Buffer Data Port register"]
    pub datport: DATPORT,
    #[doc = "0x24 - Present State register"]
    pub prsstat: PRSSTAT,
    #[doc = "0x28 - Protocol Control register"]
    pub proctl: PROCTL,
    #[doc = "0x2c - System Control register"]
    pub sysctl: SYSCTL,
    #[doc = "0x30 - Interrupt Status register"]
    pub irqstat: IRQSTAT,
    #[doc = "0x34 - Interrupt Status Enable register"]
    pub irqstaten: IRQSTATEN,
    #[doc = "0x38 - Interrupt Signal Enable register"]
    pub irqsigen: IRQSIGEN,
    #[doc = "0x3c - Auto CMD12 Error Status Register"]
    pub ac12err: AC12ERR,
    #[doc = "0x40 - Host Controller Capabilities"]
    pub htcapblt: HTCAPBLT,
    #[doc = "0x44 - Watermark Level Register"]
    pub wml: WML,
    _reserved18: [u8; 8usize],
    #[doc = "0x50 - Force Event register"]
    pub fevt: FEVT,
    #[doc = "0x54 - ADMA Error Status register"]
    pub admaes: ADMAES,
    #[doc = "0x58 - ADMA System Addressregister"]
    pub adsaddr: ADSADDR,
    _reserved21: [u8; 100usize],
    #[doc = "0xc0 - Vendor Specific register"]
    pub vendor: VENDOR,
    #[doc = "0xc4 - MMC Boot register"]
    pub mmcboot: MMCBOOT,
    _reserved23: [u8; 52usize],
    #[doc = "0xfc - Host Controller Version"]
    pub hostver: HOSTVER,
}
#[doc = "DMA System Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dsaddr](dsaddr) module"]
pub type DSADDR = crate::Reg<u32, _DSADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSADDR;
#[doc = "`read()` method returns [dsaddr::R](dsaddr::R) reader structure"]
impl crate::Readable for DSADDR {}
#[doc = "`write(|w| ..)` method takes [dsaddr::W](dsaddr::W) writer structure"]
impl crate::Writable for DSADDR {}
#[doc = "DMA System Address register"]
pub mod dsaddr;
#[doc = "Block Attributes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blkattr](blkattr) module"]
pub type BLKATTR = crate::Reg<u32, _BLKATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLKATTR;
#[doc = "`read()` method returns [blkattr::R](blkattr::R) reader structure"]
impl crate::Readable for BLKATTR {}
#[doc = "`write(|w| ..)` method takes [blkattr::W](blkattr::W) writer structure"]
impl crate::Writable for BLKATTR {}
#[doc = "Block Attributes register"]
pub mod blkattr;
#[doc = "Command Argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdarg](cmdarg) module"]
pub type CMDARG = crate::Reg<u32, _CMDARG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDARG;
#[doc = "`read()` method returns [cmdarg::R](cmdarg::R) reader structure"]
impl crate::Readable for CMDARG {}
#[doc = "`write(|w| ..)` method takes [cmdarg::W](cmdarg::W) writer structure"]
impl crate::Writable for CMDARG {}
#[doc = "Command Argument register"]
pub mod cmdarg;
#[doc = "Transfer Type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xfertyp](xfertyp) module"]
pub type XFERTYP = crate::Reg<u32, _XFERTYP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XFERTYP;
#[doc = "`read()` method returns [xfertyp::R](xfertyp::R) reader structure"]
impl crate::Readable for XFERTYP {}
#[doc = "`write(|w| ..)` method takes [xfertyp::W](xfertyp::W) writer structure"]
impl crate::Writable for XFERTYP {}
#[doc = "Transfer Type register"]
pub mod xfertyp;
#[doc = "Command Response 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdrsp0](cmdrsp0) module"]
pub type CMDRSP0 = crate::Reg<u32, _CMDRSP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDRSP0;
#[doc = "`read()` method returns [cmdrsp0::R](cmdrsp0::R) reader structure"]
impl crate::Readable for CMDRSP0 {}
#[doc = "Command Response 0"]
pub mod cmdrsp0;
#[doc = "Command Response 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdrsp1](cmdrsp1) module"]
pub type CMDRSP1 = crate::Reg<u32, _CMDRSP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDRSP1;
#[doc = "`read()` method returns [cmdrsp1::R](cmdrsp1::R) reader structure"]
impl crate::Readable for CMDRSP1 {}
#[doc = "Command Response 1"]
pub mod cmdrsp1;
#[doc = "Command Response 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdrsp2](cmdrsp2) module"]
pub type CMDRSP2 = crate::Reg<u32, _CMDRSP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDRSP2;
#[doc = "`read()` method returns [cmdrsp2::R](cmdrsp2::R) reader structure"]
impl crate::Readable for CMDRSP2 {}
#[doc = "Command Response 2"]
pub mod cmdrsp2;
#[doc = "Command Response 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdrsp3](cmdrsp3) module"]
pub type CMDRSP3 = crate::Reg<u32, _CMDRSP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDRSP3;
#[doc = "`read()` method returns [cmdrsp3::R](cmdrsp3::R) reader structure"]
impl crate::Readable for CMDRSP3 {}
#[doc = "Command Response 3"]
pub mod cmdrsp3;
#[doc = "Buffer Data Port register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [datport](datport) module"]
pub type DATPORT = crate::Reg<u32, _DATPORT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATPORT;
#[doc = "`read()` method returns [datport::R](datport::R) reader structure"]
impl crate::Readable for DATPORT {}
#[doc = "`write(|w| ..)` method takes [datport::W](datport::W) writer structure"]
impl crate::Writable for DATPORT {}
#[doc = "Buffer Data Port register"]
pub mod datport;
#[doc = "Present State register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prsstat](prsstat) module"]
pub type PRSSTAT = crate::Reg<u32, _PRSSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSSTAT;
#[doc = "`read()` method returns [prsstat::R](prsstat::R) reader structure"]
impl crate::Readable for PRSSTAT {}
#[doc = "Present State register"]
pub mod prsstat;
#[doc = "Protocol Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [proctl](proctl) module"]
pub type PROCTL = crate::Reg<u32, _PROCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROCTL;
#[doc = "`read()` method returns [proctl::R](proctl::R) reader structure"]
impl crate::Readable for PROCTL {}
#[doc = "`write(|w| ..)` method takes [proctl::W](proctl::W) writer structure"]
impl crate::Writable for PROCTL {}
#[doc = "Protocol Control register"]
pub mod proctl;
#[doc = "System Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysctl](sysctl) module"]
pub type SYSCTL = crate::Reg<u32, _SYSCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTL;
#[doc = "`read()` method returns [sysctl::R](sysctl::R) reader structure"]
impl crate::Readable for SYSCTL {}
#[doc = "`write(|w| ..)` method takes [sysctl::W](sysctl::W) writer structure"]
impl crate::Writable for SYSCTL {}
#[doc = "System Control register"]
pub mod sysctl;
#[doc = "Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irqstat](irqstat) module"]
pub type IRQSTAT = crate::Reg<u32, _IRQSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSTAT;
#[doc = "`read()` method returns [irqstat::R](irqstat::R) reader structure"]
impl crate::Readable for IRQSTAT {}
#[doc = "`write(|w| ..)` method takes [irqstat::W](irqstat::W) writer structure"]
impl crate::Writable for IRQSTAT {}
#[doc = "Interrupt Status register"]
pub mod irqstat;
#[doc = "Interrupt Status Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irqstaten](irqstaten) module"]
pub type IRQSTATEN = crate::Reg<u32, _IRQSTATEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSTATEN;
#[doc = "`read()` method returns [irqstaten::R](irqstaten::R) reader structure"]
impl crate::Readable for IRQSTATEN {}
#[doc = "`write(|w| ..)` method takes [irqstaten::W](irqstaten::W) writer structure"]
impl crate::Writable for IRQSTATEN {}
#[doc = "Interrupt Status Enable register"]
pub mod irqstaten;
#[doc = "Interrupt Signal Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irqsigen](irqsigen) module"]
pub type IRQSIGEN = crate::Reg<u32, _IRQSIGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSIGEN;
#[doc = "`read()` method returns [irqsigen::R](irqsigen::R) reader structure"]
impl crate::Readable for IRQSIGEN {}
#[doc = "`write(|w| ..)` method takes [irqsigen::W](irqsigen::W) writer structure"]
impl crate::Writable for IRQSIGEN {}
#[doc = "Interrupt Signal Enable register"]
pub mod irqsigen;
#[doc = "Auto CMD12 Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ac12err](ac12err) module"]
pub type AC12ERR = crate::Reg<u32, _AC12ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AC12ERR;
#[doc = "`read()` method returns [ac12err::R](ac12err::R) reader structure"]
impl crate::Readable for AC12ERR {}
#[doc = "Auto CMD12 Error Status Register"]
pub mod ac12err;
#[doc = "Host Controller Capabilities\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [htcapblt](htcapblt) module"]
pub type HTCAPBLT = crate::Reg<u32, _HTCAPBLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HTCAPBLT;
#[doc = "`read()` method returns [htcapblt::R](htcapblt::R) reader structure"]
impl crate::Readable for HTCAPBLT {}
#[doc = "Host Controller Capabilities"]
pub mod htcapblt;
#[doc = "Watermark Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wml](wml) module"]
pub type WML = crate::Reg<u32, _WML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WML;
#[doc = "`read()` method returns [wml::R](wml::R) reader structure"]
impl crate::Readable for WML {}
#[doc = "`write(|w| ..)` method takes [wml::W](wml::W) writer structure"]
impl crate::Writable for WML {}
#[doc = "Watermark Level Register"]
pub mod wml;
#[doc = "Force Event register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fevt](fevt) module"]
pub type FEVT = crate::Reg<u32, _FEVT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEVT;
#[doc = "`write(|w| ..)` method takes [fevt::W](fevt::W) writer structure"]
impl crate::Writable for FEVT {}
#[doc = "Force Event register"]
pub mod fevt;
#[doc = "ADMA Error Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [admaes](admaes) module"]
pub type ADMAES = crate::Reg<u32, _ADMAES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADMAES;
#[doc = "`read()` method returns [admaes::R](admaes::R) reader structure"]
impl crate::Readable for ADMAES {}
#[doc = "ADMA Error Status register"]
pub mod admaes;
#[doc = "ADMA System Addressregister\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adsaddr](adsaddr) module"]
pub type ADSADDR = crate::Reg<u32, _ADSADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADSADDR;
#[doc = "`read()` method returns [adsaddr::R](adsaddr::R) reader structure"]
impl crate::Readable for ADSADDR {}
#[doc = "`write(|w| ..)` method takes [adsaddr::W](adsaddr::W) writer structure"]
impl crate::Writable for ADSADDR {}
#[doc = "ADMA System Addressregister"]
pub mod adsaddr;
#[doc = "Vendor Specific register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vendor](vendor) module"]
pub type VENDOR = crate::Reg<u32, _VENDOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VENDOR;
#[doc = "`read()` method returns [vendor::R](vendor::R) reader structure"]
impl crate::Readable for VENDOR {}
#[doc = "`write(|w| ..)` method takes [vendor::W](vendor::W) writer structure"]
impl crate::Writable for VENDOR {}
#[doc = "Vendor Specific register"]
pub mod vendor;
#[doc = "MMC Boot register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mmcboot](mmcboot) module"]
pub type MMCBOOT = crate::Reg<u32, _MMCBOOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMCBOOT;
#[doc = "`read()` method returns [mmcboot::R](mmcboot::R) reader structure"]
impl crate::Readable for MMCBOOT {}
#[doc = "`write(|w| ..)` method takes [mmcboot::W](mmcboot::W) writer structure"]
impl crate::Writable for MMCBOOT {}
#[doc = "MMC Boot register"]
pub mod mmcboot;
#[doc = "Host Controller Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hostver](hostver) module"]
pub type HOSTVER = crate::Reg<u32, _HOSTVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOSTVER;
#[doc = "`read()` method returns [hostver::R](hostver::R) reader structure"]
impl crate::Readable for HOSTVER {}
#[doc = "Host Controller Version"]
pub mod hostver;
