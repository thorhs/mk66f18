#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip Select Address Register"]
    pub csar0: CSAR,
    #[doc = "0x04 - Chip Select Mask Register"]
    pub csmr0: CSMR,
    #[doc = "0x08 - Chip Select Control Register"]
    pub cscr0: CSCR,
    #[doc = "0x0c - Chip Select Address Register"]
    pub csar1: CSAR,
    #[doc = "0x10 - Chip Select Mask Register"]
    pub csmr1: CSMR,
    #[doc = "0x14 - Chip Select Control Register"]
    pub cscr1: CSCR,
    #[doc = "0x18 - Chip Select Address Register"]
    pub csar2: CSAR,
    #[doc = "0x1c - Chip Select Mask Register"]
    pub csmr2: CSMR,
    #[doc = "0x20 - Chip Select Control Register"]
    pub cscr2: CSCR,
    #[doc = "0x24 - Chip Select Address Register"]
    pub csar3: CSAR,
    #[doc = "0x28 - Chip Select Mask Register"]
    pub csmr3: CSMR,
    #[doc = "0x2c - Chip Select Control Register"]
    pub cscr3: CSCR,
    #[doc = "0x30 - Chip Select Address Register"]
    pub csar4: CSAR,
    #[doc = "0x34 - Chip Select Mask Register"]
    pub csmr4: CSMR,
    #[doc = "0x38 - Chip Select Control Register"]
    pub cscr4: CSCR,
    #[doc = "0x3c - Chip Select Address Register"]
    pub csar5: CSAR,
    #[doc = "0x40 - Chip Select Mask Register"]
    pub csmr5: CSMR,
    #[doc = "0x44 - Chip Select Control Register"]
    pub cscr5: CSCR,
    _reserved18: [u8; 24usize],
    #[doc = "0x60 - Chip Select port Multiplexing Control Register"]
    pub cspmcr: CSPMCR,
}
#[doc = "Chip Select Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csar](csar) module"]
pub type CSAR = crate::Reg<u32, _CSAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSAR;
#[doc = "`read()` method returns [csar::R](csar::R) reader structure"]
impl crate::Readable for CSAR {}
#[doc = "`write(|w| ..)` method takes [csar::W](csar::W) writer structure"]
impl crate::Writable for CSAR {}
#[doc = "Chip Select Address Register"]
pub mod csar;
#[doc = "Chip Select Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csmr](csmr) module"]
pub type CSMR = crate::Reg<u32, _CSMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSMR;
#[doc = "`read()` method returns [csmr::R](csmr::R) reader structure"]
impl crate::Readable for CSMR {}
#[doc = "`write(|w| ..)` method takes [csmr::W](csmr::W) writer structure"]
impl crate::Writable for CSMR {}
#[doc = "Chip Select Mask Register"]
pub mod csmr;
#[doc = "Chip Select Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cscr](cscr) module"]
pub type CSCR = crate::Reg<u32, _CSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCR;
#[doc = "`read()` method returns [cscr::R](cscr::R) reader structure"]
impl crate::Readable for CSCR {}
#[doc = "`write(|w| ..)` method takes [cscr::W](cscr::W) writer structure"]
impl crate::Writable for CSCR {}
#[doc = "Chip Select Control Register"]
pub mod cscr;
#[doc = "Chip Select port Multiplexing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspmcr](cspmcr) module"]
pub type CSPMCR = crate::Reg<u32, _CSPMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPMCR;
#[doc = "`read()` method returns [cspmcr::R](cspmcr::R) reader structure"]
impl crate::Readable for CSPMCR {}
#[doc = "`write(|w| ..)` method takes [cspmcr::W](cspmcr::W) writer structure"]
impl crate::Writable for CSPMCR {}
#[doc = "Chip Select port Multiplexing Control Register"]
pub mod cspmcr;
