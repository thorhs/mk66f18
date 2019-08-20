#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Privilege Register A"]
    pub mpra: MPRA,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - Peripheral Access Control Register"]
    pub pacra: PACRA,
    #[doc = "0x24 - Peripheral Access Control Register"]
    pub pacrb: PACRB,
    #[doc = "0x28 - Peripheral Access Control Register"]
    pub pacrc: PACRC,
    #[doc = "0x2c - Peripheral Access Control Register"]
    pub pacrd: PACRD,
    _reserved5: [u8; 16usize],
    #[doc = "0x40 - Peripheral Access Control Register"]
    pub pacre: PACRE,
    #[doc = "0x44 - Peripheral Access Control Register"]
    pub pacrf: PACRF,
    #[doc = "0x48 - Peripheral Access Control Register"]
    pub pacrg: PACRG,
    #[doc = "0x4c - Peripheral Access Control Register"]
    pub pacrh: PACRH,
    #[doc = "0x50 - Peripheral Access Control Register"]
    pub pacri: PACRI,
    #[doc = "0x54 - Peripheral Access Control Register"]
    pub pacrj: PACRJ,
    #[doc = "0x58 - Peripheral Access Control Register"]
    pub pacrk: PACRK,
    #[doc = "0x5c - Peripheral Access Control Register"]
    pub pacrl: PACRL,
    #[doc = "0x60 - Peripheral Access Control Register"]
    pub pacrm: PACRM,
    #[doc = "0x64 - Peripheral Access Control Register"]
    pub pacrn: PACRN,
    #[doc = "0x68 - Peripheral Access Control Register"]
    pub pacro: PACRO,
    #[doc = "0x6c - Peripheral Access Control Register"]
    pub pacrp: PACRP,
}
#[doc = "Master Privilege Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpra](mpra) module"]
pub type MPRA = crate::Reg<u32, _MPRA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPRA;
#[doc = "`read()` method returns [mpra::R](mpra::R) reader structure"]
impl crate::Readable for MPRA {}
#[doc = "`write(|w| ..)` method takes [mpra::W](mpra::W) writer structure"]
impl crate::Writable for MPRA {}
#[doc = "Master Privilege Register A"]
pub mod mpra;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacra](pacra) module"]
pub type PACRA = crate::Reg<u32, _PACRA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRA;
#[doc = "`read()` method returns [pacra::R](pacra::R) reader structure"]
impl crate::Readable for PACRA {}
#[doc = "`write(|w| ..)` method takes [pacra::W](pacra::W) writer structure"]
impl crate::Writable for PACRA {}
#[doc = "Peripheral Access Control Register"]
pub mod pacra;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacrb](pacrb) module"]
pub type PACRB = crate::Reg<u32, _PACRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRB;
#[doc = "`read()` method returns [pacrb::R](pacrb::R) reader structure"]
impl crate::Readable for PACRB {}
#[doc = "`write(|w| ..)` method takes [pacrb::W](pacrb::W) writer structure"]
impl crate::Writable for PACRB {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrb;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacrc](pacrc) module"]
pub type PACRC = crate::Reg<u32, _PACRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRC;
#[doc = "`read()` method returns [pacrc::R](pacrc::R) reader structure"]
impl crate::Readable for PACRC {}
#[doc = "`write(|w| ..)` method takes [pacrc::W](pacrc::W) writer structure"]
impl crate::Writable for PACRC {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrc;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacrd](pacrd) module"]
pub type PACRD = crate::Reg<u32, _PACRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRD;
#[doc = "`read()` method returns [pacrd::R](pacrd::R) reader structure"]
impl crate::Readable for PACRD {}
#[doc = "`write(|w| ..)` method takes [pacrd::W](pacrd::W) writer structure"]
impl crate::Writable for PACRD {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrd;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacre](pacre) module"]
pub type PACRE = crate::Reg<u32, _PACRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRE;
#[doc = "`read()` method returns [pacre::R](pacre::R) reader structure"]
impl crate::Readable for PACRE {}
#[doc = "`write(|w| ..)` method takes [pacre::W](pacre::W) writer structure"]
impl crate::Writable for PACRE {}
#[doc = "Peripheral Access Control Register"]
pub mod pacre;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacrf](pacrf) module"]
pub type PACRF = crate::Reg<u32, _PACRF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRF;
#[doc = "`read()` method returns [pacrf::R](pacrf::R) reader structure"]
impl crate::Readable for PACRF {}
#[doc = "`write(|w| ..)` method takes [pacrf::W](pacrf::W) writer structure"]
impl crate::Writable for PACRF {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrf;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacrg](pacrg) module"]
pub type PACRG = crate::Reg<u32, _PACRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRG;
#[doc = "`read()` method returns [pacrg::R](pacrg::R) reader structure"]
impl crate::Readable for PACRG {}
#[doc = "`write(|w| ..)` method takes [pacrg::W](pacrg::W) writer structure"]
impl crate::Writable for PACRG {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrg;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacrh](pacrh) module"]
pub type PACRH = crate::Reg<u32, _PACRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRH;
#[doc = "`read()` method returns [pacrh::R](pacrh::R) reader structure"]
impl crate::Readable for PACRH {}
#[doc = "`write(|w| ..)` method takes [pacrh::W](pacrh::W) writer structure"]
impl crate::Writable for PACRH {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrh;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacri](pacri) module"]
pub type PACRI = crate::Reg<u32, _PACRI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRI;
#[doc = "`read()` method returns [pacri::R](pacri::R) reader structure"]
impl crate::Readable for PACRI {}
#[doc = "`write(|w| ..)` method takes [pacri::W](pacri::W) writer structure"]
impl crate::Writable for PACRI {}
#[doc = "Peripheral Access Control Register"]
pub mod pacri;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacrj](pacrj) module"]
pub type PACRJ = crate::Reg<u32, _PACRJ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRJ;
#[doc = "`read()` method returns [pacrj::R](pacrj::R) reader structure"]
impl crate::Readable for PACRJ {}
#[doc = "`write(|w| ..)` method takes [pacrj::W](pacrj::W) writer structure"]
impl crate::Writable for PACRJ {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrj;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacrk](pacrk) module"]
pub type PACRK = crate::Reg<u32, _PACRK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRK;
#[doc = "`read()` method returns [pacrk::R](pacrk::R) reader structure"]
impl crate::Readable for PACRK {}
#[doc = "`write(|w| ..)` method takes [pacrk::W](pacrk::W) writer structure"]
impl crate::Writable for PACRK {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrk;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacrl](pacrl) module"]
pub type PACRL = crate::Reg<u32, _PACRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRL;
#[doc = "`read()` method returns [pacrl::R](pacrl::R) reader structure"]
impl crate::Readable for PACRL {}
#[doc = "`write(|w| ..)` method takes [pacrl::W](pacrl::W) writer structure"]
impl crate::Writable for PACRL {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrl;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacrm](pacrm) module"]
pub type PACRM = crate::Reg<u32, _PACRM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRM;
#[doc = "`read()` method returns [pacrm::R](pacrm::R) reader structure"]
impl crate::Readable for PACRM {}
#[doc = "`write(|w| ..)` method takes [pacrm::W](pacrm::W) writer structure"]
impl crate::Writable for PACRM {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrm;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacrn](pacrn) module"]
pub type PACRN = crate::Reg<u32, _PACRN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRN;
#[doc = "`read()` method returns [pacrn::R](pacrn::R) reader structure"]
impl crate::Readable for PACRN {}
#[doc = "`write(|w| ..)` method takes [pacrn::W](pacrn::W) writer structure"]
impl crate::Writable for PACRN {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrn;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacro](pacro) module"]
pub type PACRO = crate::Reg<u32, _PACRO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRO;
#[doc = "`read()` method returns [pacro::R](pacro::R) reader structure"]
impl crate::Readable for PACRO {}
#[doc = "`write(|w| ..)` method takes [pacro::W](pacro::W) writer structure"]
impl crate::Writable for PACRO {}
#[doc = "Peripheral Access Control Register"]
pub mod pacro;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pacrp](pacrp) module"]
pub type PACRP = crate::Reg<u32, _PACRP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRP;
#[doc = "`read()` method returns [pacrp::R](pacrp::R) reader structure"]
impl crate::Readable for PACRP {}
#[doc = "`write(|w| ..)` method takes [pacrp::W](pacrp::W) writer structure"]
impl crate::Writable for PACRP {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrp;
