#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCG Control 1 Register"]
    pub c1: C1,
    #[doc = "0x01 - MCG Control 2 Register"]
    pub c2: C2,
    #[doc = "0x02 - MCG Control 3 Register"]
    pub c3: C3,
    #[doc = "0x03 - MCG Control 4 Register"]
    pub c4: C4,
    #[doc = "0x04 - MCG Control 5 Register"]
    pub c5: C5,
    #[doc = "0x05 - MCG Control 6 Register"]
    pub c6: C6,
    #[doc = "0x06 - MCG Status Register"]
    pub s: S,
    _reserved7: [u8; 1usize],
    #[doc = "0x08 - MCG Status and Control Register"]
    pub sc: SC,
    _reserved8: [u8; 1usize],
    #[doc = "0x0a - MCG Auto Trim Compare Value High Register"]
    pub atcvh: ATCVH,
    #[doc = "0x0b - MCG Auto Trim Compare Value Low Register"]
    pub atcvl: ATCVL,
    #[doc = "0x0c - MCG Control 7 Register"]
    pub c7: C7,
    #[doc = "0x0d - MCG Control 8 Register"]
    pub c8: C8,
    #[doc = "0x0e - MCG Control 9 Register"]
    pub c9: C9,
    _reserved13: [u8; 1usize],
    #[doc = "0x10 - MCG Control 11 Register"]
    pub c11: C11,
    _reserved14: [u8; 1usize],
    #[doc = "0x12 - MCG Status 2 Register"]
    pub s2: S2,
}
#[doc = "MCG Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c1](c1) module"]
pub type C1 = crate::Reg<u8, _C1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1;
#[doc = "`read()` method returns [c1::R](c1::R) reader structure"]
impl crate::Readable for C1 {}
#[doc = "`write(|w| ..)` method takes [c1::W](c1::W) writer structure"]
impl crate::Writable for C1 {}
#[doc = "MCG Control 1 Register"]
pub mod c1;
#[doc = "MCG Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c2](c2) module"]
pub type C2 = crate::Reg<u8, _C2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2;
#[doc = "`read()` method returns [c2::R](c2::R) reader structure"]
impl crate::Readable for C2 {}
#[doc = "`write(|w| ..)` method takes [c2::W](c2::W) writer structure"]
impl crate::Writable for C2 {}
#[doc = "MCG Control 2 Register"]
pub mod c2;
#[doc = "MCG Control 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c3](c3) module"]
pub type C3 = crate::Reg<u8, _C3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3;
#[doc = "`read()` method returns [c3::R](c3::R) reader structure"]
impl crate::Readable for C3 {}
#[doc = "`write(|w| ..)` method takes [c3::W](c3::W) writer structure"]
impl crate::Writable for C3 {}
#[doc = "MCG Control 3 Register"]
pub mod c3;
#[doc = "MCG Control 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c4](c4) module"]
pub type C4 = crate::Reg<u8, _C4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4;
#[doc = "`read()` method returns [c4::R](c4::R) reader structure"]
impl crate::Readable for C4 {}
#[doc = "`write(|w| ..)` method takes [c4::W](c4::W) writer structure"]
impl crate::Writable for C4 {}
#[doc = "MCG Control 4 Register"]
pub mod c4;
#[doc = "MCG Control 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c5](c5) module"]
pub type C5 = crate::Reg<u8, _C5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5;
#[doc = "`read()` method returns [c5::R](c5::R) reader structure"]
impl crate::Readable for C5 {}
#[doc = "`write(|w| ..)` method takes [c5::W](c5::W) writer structure"]
impl crate::Writable for C5 {}
#[doc = "MCG Control 5 Register"]
pub mod c5;
#[doc = "MCG Control 6 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c6](c6) module"]
pub type C6 = crate::Reg<u8, _C6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6;
#[doc = "`read()` method returns [c6::R](c6::R) reader structure"]
impl crate::Readable for C6 {}
#[doc = "`write(|w| ..)` method takes [c6::W](c6::W) writer structure"]
impl crate::Writable for C6 {}
#[doc = "MCG Control 6 Register"]
pub mod c6;
#[doc = "MCG Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [s](s) module"]
pub type S = crate::Reg<u8, _S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S;
#[doc = "`read()` method returns [s::R](s::R) reader structure"]
impl crate::Readable for S {}
#[doc = "`write(|w| ..)` method takes [s::W](s::W) writer structure"]
impl crate::Writable for S {}
#[doc = "MCG Status Register"]
pub mod s;
#[doc = "MCG Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sc](sc) module"]
pub type SC = crate::Reg<u8, _SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC;
#[doc = "`read()` method returns [sc::R](sc::R) reader structure"]
impl crate::Readable for SC {}
#[doc = "`write(|w| ..)` method takes [sc::W](sc::W) writer structure"]
impl crate::Writable for SC {}
#[doc = "MCG Status and Control Register"]
pub mod sc;
#[doc = "MCG Auto Trim Compare Value High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [atcvh](atcvh) module"]
pub type ATCVH = crate::Reg<u8, _ATCVH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATCVH;
#[doc = "`read()` method returns [atcvh::R](atcvh::R) reader structure"]
impl crate::Readable for ATCVH {}
#[doc = "`write(|w| ..)` method takes [atcvh::W](atcvh::W) writer structure"]
impl crate::Writable for ATCVH {}
#[doc = "MCG Auto Trim Compare Value High Register"]
pub mod atcvh;
#[doc = "MCG Auto Trim Compare Value Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [atcvl](atcvl) module"]
pub type ATCVL = crate::Reg<u8, _ATCVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATCVL;
#[doc = "`read()` method returns [atcvl::R](atcvl::R) reader structure"]
impl crate::Readable for ATCVL {}
#[doc = "`write(|w| ..)` method takes [atcvl::W](atcvl::W) writer structure"]
impl crate::Writable for ATCVL {}
#[doc = "MCG Auto Trim Compare Value Low Register"]
pub mod atcvl;
#[doc = "MCG Control 7 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c7](c7) module"]
pub type C7 = crate::Reg<u8, _C7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7;
#[doc = "`read()` method returns [c7::R](c7::R) reader structure"]
impl crate::Readable for C7 {}
#[doc = "`write(|w| ..)` method takes [c7::W](c7::W) writer structure"]
impl crate::Writable for C7 {}
#[doc = "MCG Control 7 Register"]
pub mod c7;
#[doc = "MCG Control 8 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c8](c8) module"]
pub type C8 = crate::Reg<u8, _C8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8;
#[doc = "`read()` method returns [c8::R](c8::R) reader structure"]
impl crate::Readable for C8 {}
#[doc = "`write(|w| ..)` method takes [c8::W](c8::W) writer structure"]
impl crate::Writable for C8 {}
#[doc = "MCG Control 8 Register"]
pub mod c8;
#[doc = "MCG Control 9 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c9](c9) module"]
pub type C9 = crate::Reg<u8, _C9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9;
#[doc = "`read()` method returns [c9::R](c9::R) reader structure"]
impl crate::Readable for C9 {}
#[doc = "`write(|w| ..)` method takes [c9::W](c9::W) writer structure"]
impl crate::Writable for C9 {}
#[doc = "MCG Control 9 Register"]
pub mod c9;
#[doc = "MCG Control 11 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c11](c11) module"]
pub type C11 = crate::Reg<u8, _C11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11;
#[doc = "`read()` method returns [c11::R](c11::R) reader structure"]
impl crate::Readable for C11 {}
#[doc = "`write(|w| ..)` method takes [c11::W](c11::W) writer structure"]
impl crate::Writable for C11 {}
#[doc = "MCG Control 11 Register"]
pub mod c11;
#[doc = "MCG Status 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [s2](s2) module"]
pub type S2 = crate::Reg<u8, _S2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S2;
#[doc = "`read()` method returns [s2::R](s2::R) reader structure"]
impl crate::Readable for S2 {}
#[doc = "MCG Status 2 Register"]
pub mod s2;
