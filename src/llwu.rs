#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LLWU Pin Enable 1 register"]
    pub pe1: PE1,
    #[doc = "0x01 - LLWU Pin Enable 2 register"]
    pub pe2: PE2,
    #[doc = "0x02 - LLWU Pin Enable 3 register"]
    pub pe3: PE3,
    #[doc = "0x03 - LLWU Pin Enable 4 register"]
    pub pe4: PE4,
    #[doc = "0x04 - LLWU Pin Enable 5 register"]
    pub pe5: PE5,
    #[doc = "0x05 - LLWU Pin Enable 6 register"]
    pub pe6: PE6,
    #[doc = "0x06 - LLWU Pin Enable 7 register"]
    pub pe7: PE7,
    #[doc = "0x07 - LLWU Pin Enable 8 register"]
    pub pe8: PE8,
    #[doc = "0x08 - LLWU Module Enable register"]
    pub me: ME,
    #[doc = "0x09 - LLWU Pin Flag 1 register"]
    pub pf1: PF1,
    #[doc = "0x0a - LLWU Pin Flag 2 register"]
    pub pf2: PF2,
    #[doc = "0x0b - LLWU Pin Flag 3 register"]
    pub pf3: PF3,
    #[doc = "0x0c - LLWU Pin Flag 4 register"]
    pub pf4: PF4,
    #[doc = "0x0d - LLWU Module Flag 5 register"]
    pub mf5: MF5,
    #[doc = "0x0e - LLWU Pin Filter 1 register"]
    pub filt1: FILT1,
    #[doc = "0x0f - LLWU Pin Filter 2 register"]
    pub filt2: FILT2,
    #[doc = "0x10 - LLWU Pin Filter 3 register"]
    pub filt3: FILT3,
    #[doc = "0x11 - LLWU Pin Filter 4 register"]
    pub filt4: FILT4,
}
#[doc = "LLWU Pin Enable 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe1](pe1) module"]
pub type PE1 = crate::Reg<u8, _PE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE1;
#[doc = "`read()` method returns [pe1::R](pe1::R) reader structure"]
impl crate::Readable for PE1 {}
#[doc = "`write(|w| ..)` method takes [pe1::W](pe1::W) writer structure"]
impl crate::Writable for PE1 {}
#[doc = "LLWU Pin Enable 1 register"]
pub mod pe1;
#[doc = "LLWU Pin Enable 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe2](pe2) module"]
pub type PE2 = crate::Reg<u8, _PE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE2;
#[doc = "`read()` method returns [pe2::R](pe2::R) reader structure"]
impl crate::Readable for PE2 {}
#[doc = "`write(|w| ..)` method takes [pe2::W](pe2::W) writer structure"]
impl crate::Writable for PE2 {}
#[doc = "LLWU Pin Enable 2 register"]
pub mod pe2;
#[doc = "LLWU Pin Enable 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe3](pe3) module"]
pub type PE3 = crate::Reg<u8, _PE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE3;
#[doc = "`read()` method returns [pe3::R](pe3::R) reader structure"]
impl crate::Readable for PE3 {}
#[doc = "`write(|w| ..)` method takes [pe3::W](pe3::W) writer structure"]
impl crate::Writable for PE3 {}
#[doc = "LLWU Pin Enable 3 register"]
pub mod pe3;
#[doc = "LLWU Pin Enable 4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe4](pe4) module"]
pub type PE4 = crate::Reg<u8, _PE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE4;
#[doc = "`read()` method returns [pe4::R](pe4::R) reader structure"]
impl crate::Readable for PE4 {}
#[doc = "`write(|w| ..)` method takes [pe4::W](pe4::W) writer structure"]
impl crate::Writable for PE4 {}
#[doc = "LLWU Pin Enable 4 register"]
pub mod pe4;
#[doc = "LLWU Pin Enable 5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe5](pe5) module"]
pub type PE5 = crate::Reg<u8, _PE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE5;
#[doc = "`read()` method returns [pe5::R](pe5::R) reader structure"]
impl crate::Readable for PE5 {}
#[doc = "`write(|w| ..)` method takes [pe5::W](pe5::W) writer structure"]
impl crate::Writable for PE5 {}
#[doc = "LLWU Pin Enable 5 register"]
pub mod pe5;
#[doc = "LLWU Pin Enable 6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe6](pe6) module"]
pub type PE6 = crate::Reg<u8, _PE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE6;
#[doc = "`read()` method returns [pe6::R](pe6::R) reader structure"]
impl crate::Readable for PE6 {}
#[doc = "`write(|w| ..)` method takes [pe6::W](pe6::W) writer structure"]
impl crate::Writable for PE6 {}
#[doc = "LLWU Pin Enable 6 register"]
pub mod pe6;
#[doc = "LLWU Pin Enable 7 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe7](pe7) module"]
pub type PE7 = crate::Reg<u8, _PE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE7;
#[doc = "`read()` method returns [pe7::R](pe7::R) reader structure"]
impl crate::Readable for PE7 {}
#[doc = "`write(|w| ..)` method takes [pe7::W](pe7::W) writer structure"]
impl crate::Writable for PE7 {}
#[doc = "LLWU Pin Enable 7 register"]
pub mod pe7;
#[doc = "LLWU Pin Enable 8 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pe8](pe8) module"]
pub type PE8 = crate::Reg<u8, _PE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE8;
#[doc = "`read()` method returns [pe8::R](pe8::R) reader structure"]
impl crate::Readable for PE8 {}
#[doc = "`write(|w| ..)` method takes [pe8::W](pe8::W) writer structure"]
impl crate::Writable for PE8 {}
#[doc = "LLWU Pin Enable 8 register"]
pub mod pe8;
#[doc = "LLWU Module Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [me](me) module"]
pub type ME = crate::Reg<u8, _ME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ME;
#[doc = "`read()` method returns [me::R](me::R) reader structure"]
impl crate::Readable for ME {}
#[doc = "`write(|w| ..)` method takes [me::W](me::W) writer structure"]
impl crate::Writable for ME {}
#[doc = "LLWU Module Enable register"]
pub mod me;
#[doc = "LLWU Pin Flag 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf1](pf1) module"]
pub type PF1 = crate::Reg<u8, _PF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF1;
#[doc = "`read()` method returns [pf1::R](pf1::R) reader structure"]
impl crate::Readable for PF1 {}
#[doc = "`write(|w| ..)` method takes [pf1::W](pf1::W) writer structure"]
impl crate::Writable for PF1 {}
#[doc = "LLWU Pin Flag 1 register"]
pub mod pf1;
#[doc = "LLWU Pin Flag 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf2](pf2) module"]
pub type PF2 = crate::Reg<u8, _PF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF2;
#[doc = "`read()` method returns [pf2::R](pf2::R) reader structure"]
impl crate::Readable for PF2 {}
#[doc = "`write(|w| ..)` method takes [pf2::W](pf2::W) writer structure"]
impl crate::Writable for PF2 {}
#[doc = "LLWU Pin Flag 2 register"]
pub mod pf2;
#[doc = "LLWU Pin Flag 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf3](pf3) module"]
pub type PF3 = crate::Reg<u8, _PF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF3;
#[doc = "`read()` method returns [pf3::R](pf3::R) reader structure"]
impl crate::Readable for PF3 {}
#[doc = "`write(|w| ..)` method takes [pf3::W](pf3::W) writer structure"]
impl crate::Writable for PF3 {}
#[doc = "LLWU Pin Flag 3 register"]
pub mod pf3;
#[doc = "LLWU Pin Flag 4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pf4](pf4) module"]
pub type PF4 = crate::Reg<u8, _PF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF4;
#[doc = "`read()` method returns [pf4::R](pf4::R) reader structure"]
impl crate::Readable for PF4 {}
#[doc = "`write(|w| ..)` method takes [pf4::W](pf4::W) writer structure"]
impl crate::Writable for PF4 {}
#[doc = "LLWU Pin Flag 4 register"]
pub mod pf4;
#[doc = "LLWU Module Flag 5 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mf5](mf5) module"]
pub type MF5 = crate::Reg<u8, _MF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MF5;
#[doc = "`read()` method returns [mf5::R](mf5::R) reader structure"]
impl crate::Readable for MF5 {}
#[doc = "LLWU Module Flag 5 register"]
pub mod mf5;
#[doc = "LLWU Pin Filter 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [filt1](filt1) module"]
pub type FILT1 = crate::Reg<u8, _FILT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILT1;
#[doc = "`read()` method returns [filt1::R](filt1::R) reader structure"]
impl crate::Readable for FILT1 {}
#[doc = "`write(|w| ..)` method takes [filt1::W](filt1::W) writer structure"]
impl crate::Writable for FILT1 {}
#[doc = "LLWU Pin Filter 1 register"]
pub mod filt1;
#[doc = "LLWU Pin Filter 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [filt2](filt2) module"]
pub type FILT2 = crate::Reg<u8, _FILT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILT2;
#[doc = "`read()` method returns [filt2::R](filt2::R) reader structure"]
impl crate::Readable for FILT2 {}
#[doc = "`write(|w| ..)` method takes [filt2::W](filt2::W) writer structure"]
impl crate::Writable for FILT2 {}
#[doc = "LLWU Pin Filter 2 register"]
pub mod filt2;
#[doc = "LLWU Pin Filter 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [filt3](filt3) module"]
pub type FILT3 = crate::Reg<u8, _FILT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILT3;
#[doc = "`read()` method returns [filt3::R](filt3::R) reader structure"]
impl crate::Readable for FILT3 {}
#[doc = "`write(|w| ..)` method takes [filt3::W](filt3::W) writer structure"]
impl crate::Writable for FILT3 {}
#[doc = "LLWU Pin Filter 3 register"]
pub mod filt3;
#[doc = "LLWU Pin Filter 4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [filt4](filt4) module"]
pub type FILT4 = crate::Reg<u8, _FILT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILT4;
#[doc = "`read()` method returns [filt4::R](filt4::R) reader structure"]
impl crate::Readable for FILT4 {}
#[doc = "`write(|w| ..)` method takes [filt4::W](filt4::W) writer structure"]
impl crate::Writable for FILT4 {}
#[doc = "LLWU Pin Filter 4 register"]
pub mod filt4;
