#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control/Error Status Register"]
    pub cesr: CESR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Error Address Register, slave port n"]
    pub ear0: EAR,
    #[doc = "0x14 - Error Detail Register, slave port n"]
    pub edr0: EDR,
    #[doc = "0x18 - Error Address Register, slave port n"]
    pub ear1: EAR,
    #[doc = "0x1c - Error Detail Register, slave port n"]
    pub edr1: EDR,
    #[doc = "0x20 - Error Address Register, slave port n"]
    pub ear2: EAR,
    #[doc = "0x24 - Error Detail Register, slave port n"]
    pub edr2: EDR,
    #[doc = "0x28 - Error Address Register, slave port n"]
    pub ear3: EAR,
    #[doc = "0x2c - Error Detail Register, slave port n"]
    pub edr3: EDR,
    #[doc = "0x30 - Error Address Register, slave port n"]
    pub ear4: EAR,
    #[doc = "0x34 - Error Detail Register, slave port n"]
    pub edr4: EDR,
    _reserved11: [u8; 968usize],
    #[doc = "0x400 - Region Descriptor n, Word 0"]
    pub rgd0_word0: RGD_WORD0,
    #[doc = "0x404 - Region Descriptor n, Word 1"]
    pub rgd0_word1: RGD_WORD1,
    #[doc = "0x408 - Region Descriptor n, Word 2"]
    pub rgd0_word2: RGD_WORD2,
    #[doc = "0x40c - Region Descriptor n, Word 3"]
    pub rgd0_word3: RGD_WORD3,
    #[doc = "0x410 - Region Descriptor n, Word 0"]
    pub rgd1_word0: RGD_WORD0,
    #[doc = "0x414 - Region Descriptor n, Word 1"]
    pub rgd1_word1: RGD_WORD1,
    #[doc = "0x418 - Region Descriptor n, Word 2"]
    pub rgd1_word2: RGD_WORD2,
    #[doc = "0x41c - Region Descriptor n, Word 3"]
    pub rgd1_word3: RGD_WORD3,
    #[doc = "0x420 - Region Descriptor n, Word 0"]
    pub rgd2_word0: RGD_WORD0,
    #[doc = "0x424 - Region Descriptor n, Word 1"]
    pub rgd2_word1: RGD_WORD1,
    #[doc = "0x428 - Region Descriptor n, Word 2"]
    pub rgd2_word2: RGD_WORD2,
    #[doc = "0x42c - Region Descriptor n, Word 3"]
    pub rgd2_word3: RGD_WORD3,
    #[doc = "0x430 - Region Descriptor n, Word 0"]
    pub rgd3_word0: RGD_WORD0,
    #[doc = "0x434 - Region Descriptor n, Word 1"]
    pub rgd3_word1: RGD_WORD1,
    #[doc = "0x438 - Region Descriptor n, Word 2"]
    pub rgd3_word2: RGD_WORD2,
    #[doc = "0x43c - Region Descriptor n, Word 3"]
    pub rgd3_word3: RGD_WORD3,
    #[doc = "0x440 - Region Descriptor n, Word 0"]
    pub rgd4_word0: RGD_WORD0,
    #[doc = "0x444 - Region Descriptor n, Word 1"]
    pub rgd4_word1: RGD_WORD1,
    #[doc = "0x448 - Region Descriptor n, Word 2"]
    pub rgd4_word2: RGD_WORD2,
    #[doc = "0x44c - Region Descriptor n, Word 3"]
    pub rgd4_word3: RGD_WORD3,
    #[doc = "0x450 - Region Descriptor n, Word 0"]
    pub rgd5_word0: RGD_WORD0,
    #[doc = "0x454 - Region Descriptor n, Word 1"]
    pub rgd5_word1: RGD_WORD1,
    #[doc = "0x458 - Region Descriptor n, Word 2"]
    pub rgd5_word2: RGD_WORD2,
    #[doc = "0x45c - Region Descriptor n, Word 3"]
    pub rgd5_word3: RGD_WORD3,
    #[doc = "0x460 - Region Descriptor n, Word 0"]
    pub rgd6_word0: RGD_WORD0,
    #[doc = "0x464 - Region Descriptor n, Word 1"]
    pub rgd6_word1: RGD_WORD1,
    #[doc = "0x468 - Region Descriptor n, Word 2"]
    pub rgd6_word2: RGD_WORD2,
    #[doc = "0x46c - Region Descriptor n, Word 3"]
    pub rgd6_word3: RGD_WORD3,
    #[doc = "0x470 - Region Descriptor n, Word 0"]
    pub rgd7_word0: RGD_WORD0,
    #[doc = "0x474 - Region Descriptor n, Word 1"]
    pub rgd7_word1: RGD_WORD1,
    #[doc = "0x478 - Region Descriptor n, Word 2"]
    pub rgd7_word2: RGD_WORD2,
    #[doc = "0x47c - Region Descriptor n, Word 3"]
    pub rgd7_word3: RGD_WORD3,
    #[doc = "0x480 - Region Descriptor n, Word 0"]
    pub rgd8_word0: RGD_WORD0,
    #[doc = "0x484 - Region Descriptor n, Word 1"]
    pub rgd8_word1: RGD_WORD1,
    #[doc = "0x488 - Region Descriptor n, Word 2"]
    pub rgd8_word2: RGD_WORD2,
    #[doc = "0x48c - Region Descriptor n, Word 3"]
    pub rgd8_word3: RGD_WORD3,
    #[doc = "0x490 - Region Descriptor n, Word 0"]
    pub rgd9_word0: RGD_WORD0,
    #[doc = "0x494 - Region Descriptor n, Word 1"]
    pub rgd9_word1: RGD_WORD1,
    #[doc = "0x498 - Region Descriptor n, Word 2"]
    pub rgd9_word2: RGD_WORD2,
    #[doc = "0x49c - Region Descriptor n, Word 3"]
    pub rgd9_word3: RGD_WORD3,
    #[doc = "0x4a0 - Region Descriptor n, Word 0"]
    pub rgd10_word0: RGD_WORD0,
    #[doc = "0x4a4 - Region Descriptor n, Word 1"]
    pub rgd10_word1: RGD_WORD1,
    #[doc = "0x4a8 - Region Descriptor n, Word 2"]
    pub rgd10_word2: RGD_WORD2,
    #[doc = "0x4ac - Region Descriptor n, Word 3"]
    pub rgd10_word3: RGD_WORD3,
    #[doc = "0x4b0 - Region Descriptor n, Word 0"]
    pub rgd11_word0: RGD_WORD0,
    #[doc = "0x4b4 - Region Descriptor n, Word 1"]
    pub rgd11_word1: RGD_WORD1,
    #[doc = "0x4b8 - Region Descriptor n, Word 2"]
    pub rgd11_word2: RGD_WORD2,
    #[doc = "0x4bc - Region Descriptor n, Word 3"]
    pub rgd11_word3: RGD_WORD3,
    _reserved59: [u8; 832usize],
    #[doc = "0x800 - Region Descriptor Alternate Access Control n"]
    pub rgdaac: [RGDAAC; 12],
}
#[doc = "Control/Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cesr](cesr) module"]
pub type CESR = crate::Reg<u32, _CESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CESR;
#[doc = "`read()` method returns [cesr::R](cesr::R) reader structure"]
impl crate::Readable for CESR {}
#[doc = "`write(|w| ..)` method takes [cesr::W](cesr::W) writer structure"]
impl crate::Writable for CESR {}
#[doc = "Control/Error Status Register"]
pub mod cesr;
#[doc = "Error Address Register, slave port n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ear](ear) module"]
pub type EAR = crate::Reg<u32, _EAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EAR;
#[doc = "`read()` method returns [ear::R](ear::R) reader structure"]
impl crate::Readable for EAR {}
#[doc = "Error Address Register, slave port n"]
pub mod ear;
#[doc = "Error Detail Register, slave port n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [edr](edr) module"]
pub type EDR = crate::Reg<u32, _EDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EDR;
#[doc = "`read()` method returns [edr::R](edr::R) reader structure"]
impl crate::Readable for EDR {}
#[doc = "Error Detail Register, slave port n"]
pub mod edr;
#[doc = "Region Descriptor n, Word 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rgd_word0](rgd_word0) module"]
pub type RGD_WORD0 = crate::Reg<u32, _RGD_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGD_WORD0;
#[doc = "`read()` method returns [rgd_word0::R](rgd_word0::R) reader structure"]
impl crate::Readable for RGD_WORD0 {}
#[doc = "`write(|w| ..)` method takes [rgd_word0::W](rgd_word0::W) writer structure"]
impl crate::Writable for RGD_WORD0 {}
#[doc = "Region Descriptor n, Word 0"]
pub mod rgd_word0;
#[doc = "Region Descriptor n, Word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rgd_word1](rgd_word1) module"]
pub type RGD_WORD1 = crate::Reg<u32, _RGD_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGD_WORD1;
#[doc = "`read()` method returns [rgd_word1::R](rgd_word1::R) reader structure"]
impl crate::Readable for RGD_WORD1 {}
#[doc = "`write(|w| ..)` method takes [rgd_word1::W](rgd_word1::W) writer structure"]
impl crate::Writable for RGD_WORD1 {}
#[doc = "Region Descriptor n, Word 1"]
pub mod rgd_word1;
#[doc = "Region Descriptor n, Word 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rgd_word2](rgd_word2) module"]
pub type RGD_WORD2 = crate::Reg<u32, _RGD_WORD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGD_WORD2;
#[doc = "`read()` method returns [rgd_word2::R](rgd_word2::R) reader structure"]
impl crate::Readable for RGD_WORD2 {}
#[doc = "`write(|w| ..)` method takes [rgd_word2::W](rgd_word2::W) writer structure"]
impl crate::Writable for RGD_WORD2 {}
#[doc = "Region Descriptor n, Word 2"]
pub mod rgd_word2;
#[doc = "Region Descriptor n, Word 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rgd_word3](rgd_word3) module"]
pub type RGD_WORD3 = crate::Reg<u32, _RGD_WORD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGD_WORD3;
#[doc = "`read()` method returns [rgd_word3::R](rgd_word3::R) reader structure"]
impl crate::Readable for RGD_WORD3 {}
#[doc = "`write(|w| ..)` method takes [rgd_word3::W](rgd_word3::W) writer structure"]
impl crate::Writable for RGD_WORD3 {}
#[doc = "Region Descriptor n, Word 3"]
pub mod rgd_word3;
#[doc = "Region Descriptor Alternate Access Control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rgdaac](rgdaac) module"]
pub type RGDAAC = crate::Reg<u32, _RGDAAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGDAAC;
#[doc = "`read()` method returns [rgdaac::R](rgdaac::R) reader structure"]
impl crate::Readable for RGDAAC {}
#[doc = "`write(|w| ..)` method takes [rgdaac::W](rgdaac::W) writer structure"]
impl crate::Writable for RGDAAC {}
#[doc = "Region Descriptor Alternate Access Control n"]
pub mod rgdaac;
