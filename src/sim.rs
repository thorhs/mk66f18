#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Options Register 1"]
    pub sopt1: SOPT1,
    #[doc = "0x04 - SOPT1 Configuration Register"]
    pub sopt1cfg: SOPT1CFG,
    #[doc = "0x08 - USB PHY Control Register"]
    pub usbphyctl: USBPHYCTL,
    _reserved3: [u8; 4088usize],
    #[doc = "0x1004 - System Options Register 2"]
    pub sopt2: SOPT2,
    _reserved4: [u8; 4usize],
    #[doc = "0x100c - System Options Register 4"]
    pub sopt4: SOPT4,
    #[doc = "0x1010 - System Options Register 5"]
    pub sopt5: SOPT5,
    _reserved6: [u8; 4usize],
    #[doc = "0x1018 - System Options Register 7"]
    pub sopt7: SOPT7,
    #[doc = "0x101c - System Options Register 8"]
    pub sopt8: SOPT8,
    #[doc = "0x1020 - System Options Register 9"]
    pub sopt9: SOPT9,
    #[doc = "0x1024 - System Device Identification Register"]
    pub sdid: SDID,
    #[doc = "0x1028 - System Clock Gating Control Register 1"]
    pub scgc1: SCGC1,
    #[doc = "0x102c - System Clock Gating Control Register 2"]
    pub scgc2: SCGC2,
    #[doc = "0x1030 - System Clock Gating Control Register 3"]
    pub scgc3: SCGC3,
    #[doc = "0x1034 - System Clock Gating Control Register 4"]
    pub scgc4: SCGC4,
    #[doc = "0x1038 - System Clock Gating Control Register 5"]
    pub scgc5: SCGC5,
    #[doc = "0x103c - System Clock Gating Control Register 6"]
    pub scgc6: SCGC6,
    #[doc = "0x1040 - System Clock Gating Control Register 7"]
    pub scgc7: SCGC7,
    #[doc = "0x1044 - System Clock Divider Register 1"]
    pub clkdiv1: CLKDIV1,
    #[doc = "0x1048 - System Clock Divider Register 2"]
    pub clkdiv2: CLKDIV2,
    #[doc = "0x104c - Flash Configuration Register 1"]
    pub fcfg1: FCFG1,
    #[doc = "0x1050 - Flash Configuration Register 2"]
    pub fcfg2: FCFG2,
    #[doc = "0x1054 - Unique Identification Register High"]
    pub uidh: UIDH,
    #[doc = "0x1058 - Unique Identification Register Mid-High"]
    pub uidmh: UIDMH,
    #[doc = "0x105c - Unique Identification Register Mid Low"]
    pub uidml: UIDML,
    #[doc = "0x1060 - Unique Identification Register Low"]
    pub uidl: UIDL,
    #[doc = "0x1064 - System Clock Divider Register 3"]
    pub clkdiv3: CLKDIV3,
    #[doc = "0x1068 - System Clock Divider Register 4"]
    pub clkdiv4: CLKDIV4,
}
#[doc = "System Options Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sopt1](sopt1) module"]
pub type SOPT1 = crate::Reg<u32, _SOPT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT1;
#[doc = "`read()` method returns [sopt1::R](sopt1::R) reader structure"]
impl crate::Readable for SOPT1 {}
#[doc = "`write(|w| ..)` method takes [sopt1::W](sopt1::W) writer structure"]
impl crate::Writable for SOPT1 {}
#[doc = "System Options Register 1"]
pub mod sopt1;
#[doc = "SOPT1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sopt1cfg](sopt1cfg) module"]
pub type SOPT1CFG = crate::Reg<u32, _SOPT1CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT1CFG;
#[doc = "`read()` method returns [sopt1cfg::R](sopt1cfg::R) reader structure"]
impl crate::Readable for SOPT1CFG {}
#[doc = "`write(|w| ..)` method takes [sopt1cfg::W](sopt1cfg::W) writer structure"]
impl crate::Writable for SOPT1CFG {}
#[doc = "SOPT1 Configuration Register"]
pub mod sopt1cfg;
#[doc = "USB PHY Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbphyctl](usbphyctl) module"]
pub type USBPHYCTL = crate::Reg<u32, _USBPHYCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPHYCTL;
#[doc = "`read()` method returns [usbphyctl::R](usbphyctl::R) reader structure"]
impl crate::Readable for USBPHYCTL {}
#[doc = "`write(|w| ..)` method takes [usbphyctl::W](usbphyctl::W) writer structure"]
impl crate::Writable for USBPHYCTL {}
#[doc = "USB PHY Control Register"]
pub mod usbphyctl;
#[doc = "System Options Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sopt2](sopt2) module"]
pub type SOPT2 = crate::Reg<u32, _SOPT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT2;
#[doc = "`read()` method returns [sopt2::R](sopt2::R) reader structure"]
impl crate::Readable for SOPT2 {}
#[doc = "`write(|w| ..)` method takes [sopt2::W](sopt2::W) writer structure"]
impl crate::Writable for SOPT2 {}
#[doc = "System Options Register 2"]
pub mod sopt2;
#[doc = "System Options Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sopt4](sopt4) module"]
pub type SOPT4 = crate::Reg<u32, _SOPT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT4;
#[doc = "`read()` method returns [sopt4::R](sopt4::R) reader structure"]
impl crate::Readable for SOPT4 {}
#[doc = "`write(|w| ..)` method takes [sopt4::W](sopt4::W) writer structure"]
impl crate::Writable for SOPT4 {}
#[doc = "System Options Register 4"]
pub mod sopt4;
#[doc = "System Options Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sopt5](sopt5) module"]
pub type SOPT5 = crate::Reg<u32, _SOPT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT5;
#[doc = "`read()` method returns [sopt5::R](sopt5::R) reader structure"]
impl crate::Readable for SOPT5 {}
#[doc = "`write(|w| ..)` method takes [sopt5::W](sopt5::W) writer structure"]
impl crate::Writable for SOPT5 {}
#[doc = "System Options Register 5"]
pub mod sopt5;
#[doc = "System Options Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sopt7](sopt7) module"]
pub type SOPT7 = crate::Reg<u32, _SOPT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT7;
#[doc = "`read()` method returns [sopt7::R](sopt7::R) reader structure"]
impl crate::Readable for SOPT7 {}
#[doc = "`write(|w| ..)` method takes [sopt7::W](sopt7::W) writer structure"]
impl crate::Writable for SOPT7 {}
#[doc = "System Options Register 7"]
pub mod sopt7;
#[doc = "System Options Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sopt8](sopt8) module"]
pub type SOPT8 = crate::Reg<u32, _SOPT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT8;
#[doc = "`read()` method returns [sopt8::R](sopt8::R) reader structure"]
impl crate::Readable for SOPT8 {}
#[doc = "`write(|w| ..)` method takes [sopt8::W](sopt8::W) writer structure"]
impl crate::Writable for SOPT8 {}
#[doc = "System Options Register 8"]
pub mod sopt8;
#[doc = "System Options Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sopt9](sopt9) module"]
pub type SOPT9 = crate::Reg<u32, _SOPT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOPT9;
#[doc = "`read()` method returns [sopt9::R](sopt9::R) reader structure"]
impl crate::Readable for SOPT9 {}
#[doc = "`write(|w| ..)` method takes [sopt9::W](sopt9::W) writer structure"]
impl crate::Writable for SOPT9 {}
#[doc = "System Options Register 9"]
pub mod sopt9;
#[doc = "System Device Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdid](sdid) module"]
pub type SDID = crate::Reg<u32, _SDID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDID;
#[doc = "`read()` method returns [sdid::R](sdid::R) reader structure"]
impl crate::Readable for SDID {}
#[doc = "System Device Identification Register"]
pub mod sdid;
#[doc = "System Clock Gating Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgc1](scgc1) module"]
pub type SCGC1 = crate::Reg<u32, _SCGC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC1;
#[doc = "`read()` method returns [scgc1::R](scgc1::R) reader structure"]
impl crate::Readable for SCGC1 {}
#[doc = "`write(|w| ..)` method takes [scgc1::W](scgc1::W) writer structure"]
impl crate::Writable for SCGC1 {}
#[doc = "System Clock Gating Control Register 1"]
pub mod scgc1;
#[doc = "System Clock Gating Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgc2](scgc2) module"]
pub type SCGC2 = crate::Reg<u32, _SCGC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC2;
#[doc = "`read()` method returns [scgc2::R](scgc2::R) reader structure"]
impl crate::Readable for SCGC2 {}
#[doc = "`write(|w| ..)` method takes [scgc2::W](scgc2::W) writer structure"]
impl crate::Writable for SCGC2 {}
#[doc = "System Clock Gating Control Register 2"]
pub mod scgc2;
#[doc = "System Clock Gating Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgc3](scgc3) module"]
pub type SCGC3 = crate::Reg<u32, _SCGC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC3;
#[doc = "`read()` method returns [scgc3::R](scgc3::R) reader structure"]
impl crate::Readable for SCGC3 {}
#[doc = "`write(|w| ..)` method takes [scgc3::W](scgc3::W) writer structure"]
impl crate::Writable for SCGC3 {}
#[doc = "System Clock Gating Control Register 3"]
pub mod scgc3;
#[doc = "System Clock Gating Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgc4](scgc4) module"]
pub type SCGC4 = crate::Reg<u32, _SCGC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC4;
#[doc = "`read()` method returns [scgc4::R](scgc4::R) reader structure"]
impl crate::Readable for SCGC4 {}
#[doc = "`write(|w| ..)` method takes [scgc4::W](scgc4::W) writer structure"]
impl crate::Writable for SCGC4 {}
#[doc = "System Clock Gating Control Register 4"]
pub mod scgc4;
#[doc = "System Clock Gating Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgc5](scgc5) module"]
pub type SCGC5 = crate::Reg<u32, _SCGC5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC5;
#[doc = "`read()` method returns [scgc5::R](scgc5::R) reader structure"]
impl crate::Readable for SCGC5 {}
#[doc = "`write(|w| ..)` method takes [scgc5::W](scgc5::W) writer structure"]
impl crate::Writable for SCGC5 {}
#[doc = "System Clock Gating Control Register 5"]
pub mod scgc5;
#[doc = "System Clock Gating Control Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgc6](scgc6) module"]
pub type SCGC6 = crate::Reg<u32, _SCGC6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC6;
#[doc = "`read()` method returns [scgc6::R](scgc6::R) reader structure"]
impl crate::Readable for SCGC6 {}
#[doc = "`write(|w| ..)` method takes [scgc6::W](scgc6::W) writer structure"]
impl crate::Writable for SCGC6 {}
#[doc = "System Clock Gating Control Register 6"]
pub mod scgc6;
#[doc = "System Clock Gating Control Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgc7](scgc7) module"]
pub type SCGC7 = crate::Reg<u32, _SCGC7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC7;
#[doc = "`read()` method returns [scgc7::R](scgc7::R) reader structure"]
impl crate::Readable for SCGC7 {}
#[doc = "`write(|w| ..)` method takes [scgc7::W](scgc7::W) writer structure"]
impl crate::Writable for SCGC7 {}
#[doc = "System Clock Gating Control Register 7"]
pub mod scgc7;
#[doc = "System Clock Divider Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkdiv1](clkdiv1) module"]
pub type CLKDIV1 = crate::Reg<u32, _CLKDIV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV1;
#[doc = "`read()` method returns [clkdiv1::R](clkdiv1::R) reader structure"]
impl crate::Readable for CLKDIV1 {}
#[doc = "`write(|w| ..)` method takes [clkdiv1::W](clkdiv1::W) writer structure"]
impl crate::Writable for CLKDIV1 {}
#[doc = "System Clock Divider Register 1"]
pub mod clkdiv1;
#[doc = "System Clock Divider Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkdiv2](clkdiv2) module"]
pub type CLKDIV2 = crate::Reg<u32, _CLKDIV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV2;
#[doc = "`read()` method returns [clkdiv2::R](clkdiv2::R) reader structure"]
impl crate::Readable for CLKDIV2 {}
#[doc = "`write(|w| ..)` method takes [clkdiv2::W](clkdiv2::W) writer structure"]
impl crate::Writable for CLKDIV2 {}
#[doc = "System Clock Divider Register 2"]
pub mod clkdiv2;
#[doc = "Flash Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcfg1](fcfg1) module"]
pub type FCFG1 = crate::Reg<u32, _FCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG1;
#[doc = "`read()` method returns [fcfg1::R](fcfg1::R) reader structure"]
impl crate::Readable for FCFG1 {}
#[doc = "`write(|w| ..)` method takes [fcfg1::W](fcfg1::W) writer structure"]
impl crate::Writable for FCFG1 {}
#[doc = "Flash Configuration Register 1"]
pub mod fcfg1;
#[doc = "Flash Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcfg2](fcfg2) module"]
pub type FCFG2 = crate::Reg<u32, _FCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG2;
#[doc = "`read()` method returns [fcfg2::R](fcfg2::R) reader structure"]
impl crate::Readable for FCFG2 {}
#[doc = "Flash Configuration Register 2"]
pub mod fcfg2;
#[doc = "Unique Identification Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uidh](uidh) module"]
pub type UIDH = crate::Reg<u32, _UIDH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDH;
#[doc = "`read()` method returns [uidh::R](uidh::R) reader structure"]
impl crate::Readable for UIDH {}
#[doc = "Unique Identification Register High"]
pub mod uidh;
#[doc = "Unique Identification Register Mid-High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uidmh](uidmh) module"]
pub type UIDMH = crate::Reg<u32, _UIDMH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDMH;
#[doc = "`read()` method returns [uidmh::R](uidmh::R) reader structure"]
impl crate::Readable for UIDMH {}
#[doc = "Unique Identification Register Mid-High"]
pub mod uidmh;
#[doc = "Unique Identification Register Mid Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uidml](uidml) module"]
pub type UIDML = crate::Reg<u32, _UIDML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDML;
#[doc = "`read()` method returns [uidml::R](uidml::R) reader structure"]
impl crate::Readable for UIDML {}
#[doc = "Unique Identification Register Mid Low"]
pub mod uidml;
#[doc = "Unique Identification Register Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uidl](uidl) module"]
pub type UIDL = crate::Reg<u32, _UIDL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDL;
#[doc = "`read()` method returns [uidl::R](uidl::R) reader structure"]
impl crate::Readable for UIDL {}
#[doc = "Unique Identification Register Low"]
pub mod uidl;
#[doc = "System Clock Divider Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkdiv3](clkdiv3) module"]
pub type CLKDIV3 = crate::Reg<u32, _CLKDIV3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV3;
#[doc = "`read()` method returns [clkdiv3::R](clkdiv3::R) reader structure"]
impl crate::Readable for CLKDIV3 {}
#[doc = "`write(|w| ..)` method takes [clkdiv3::W](clkdiv3::W) writer structure"]
impl crate::Writable for CLKDIV3 {}
#[doc = "System Clock Divider Register 3"]
pub mod clkdiv3;
#[doc = "System Clock Divider Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkdiv4](clkdiv4) module"]
pub type CLKDIV4 = crate::Reg<u32, _CLKDIV4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV4;
#[doc = "`read()` method returns [clkdiv4::R](clkdiv4::R) reader structure"]
impl crate::Readable for CLKDIV4 {}
#[doc = "`write(|w| ..)` method takes [clkdiv4::W](clkdiv4::W) writer structure"]
impl crate::Writable for CLKDIV4 {}
#[doc = "System Clock Divider Register 4"]
pub mod clkdiv4;
