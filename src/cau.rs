#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Direct access register 0"]
    pub cau_direct0: CAU_DIRECT0,
    #[doc = "0x04 - Direct access register 1"]
    pub cau_direct1: CAU_DIRECT1,
    #[doc = "0x08 - Direct access register 2"]
    pub cau_direct2: CAU_DIRECT2,
    #[doc = "0x0c - Direct access register 3"]
    pub cau_direct3: CAU_DIRECT3,
    #[doc = "0x10 - Direct access register 4"]
    pub cau_direct4: CAU_DIRECT4,
    #[doc = "0x14 - Direct access register 5"]
    pub cau_direct5: CAU_DIRECT5,
    #[doc = "0x18 - Direct access register 6"]
    pub cau_direct6: CAU_DIRECT6,
    #[doc = "0x1c - Direct access register 7"]
    pub cau_direct7: CAU_DIRECT7,
    #[doc = "0x20 - Direct access register 8"]
    pub cau_direct8: CAU_DIRECT8,
    #[doc = "0x24 - Direct access register 9"]
    pub cau_direct9: CAU_DIRECT9,
    #[doc = "0x28 - Direct access register 10"]
    pub cau_direct10: CAU_DIRECT10,
    #[doc = "0x2c - Direct access register 11"]
    pub cau_direct11: CAU_DIRECT11,
    #[doc = "0x30 - Direct access register 12"]
    pub cau_direct12: CAU_DIRECT12,
    #[doc = "0x34 - Direct access register 13"]
    pub cau_direct13: CAU_DIRECT13,
    #[doc = "0x38 - Direct access register 14"]
    pub cau_direct14: CAU_DIRECT14,
    #[doc = "0x3c - Direct access register 15"]
    pub cau_direct15: CAU_DIRECT15,
    _reserved16: [u8; 2048usize],
    #[doc = "0x840 - Status register - Load Register command"]
    pub cau_ldr_casr: CAU_LDR_CASR,
    #[doc = "0x844 - Accumulator register - Load Register command"]
    pub cau_ldr_caa: CAU_LDR_CAA,
    #[doc = "0x848 - General Purpose Register 0 - Load Register command"]
    pub cau_ldr_ca0: CAU_LDR_CA0,
    #[doc = "0x84c - General Purpose Register 1 - Load Register command"]
    pub cau_ldr_ca1: CAU_LDR_CA1,
    #[doc = "0x850 - General Purpose Register 2 - Load Register command"]
    pub cau_ldr_ca2: CAU_LDR_CA2,
    #[doc = "0x854 - General Purpose Register 3 - Load Register command"]
    pub cau_ldr_ca3: CAU_LDR_CA3,
    #[doc = "0x858 - General Purpose Register 4 - Load Register command"]
    pub cau_ldr_ca4: CAU_LDR_CA4,
    #[doc = "0x85c - General Purpose Register 5 - Load Register command"]
    pub cau_ldr_ca5: CAU_LDR_CA5,
    #[doc = "0x860 - General Purpose Register 6 - Load Register command"]
    pub cau_ldr_ca6: CAU_LDR_CA6,
    #[doc = "0x864 - General Purpose Register 7 - Load Register command"]
    pub cau_ldr_ca7: CAU_LDR_CA7,
    #[doc = "0x868 - General Purpose Register 8 - Load Register command"]
    pub cau_ldr_ca8: CAU_LDR_CA8,
    _reserved27: [u8; 20usize],
    #[doc = "0x880 - Status register - Store Register command"]
    pub cau_str_casr: CAU_STR_CASR,
    #[doc = "0x884 - Accumulator register - Store Register command"]
    pub cau_str_caa: CAU_STR_CAA,
    #[doc = "0x888 - General Purpose Register 0 - Store Register command"]
    pub cau_str_ca0: CAU_STR_CA0,
    #[doc = "0x88c - General Purpose Register 1 - Store Register command"]
    pub cau_str_ca1: CAU_STR_CA1,
    #[doc = "0x890 - General Purpose Register 2 - Store Register command"]
    pub cau_str_ca2: CAU_STR_CA2,
    #[doc = "0x894 - General Purpose Register 3 - Store Register command"]
    pub cau_str_ca3: CAU_STR_CA3,
    #[doc = "0x898 - General Purpose Register 4 - Store Register command"]
    pub cau_str_ca4: CAU_STR_CA4,
    #[doc = "0x89c - General Purpose Register 5 - Store Register command"]
    pub cau_str_ca5: CAU_STR_CA5,
    #[doc = "0x8a0 - General Purpose Register 6 - Store Register command"]
    pub cau_str_ca6: CAU_STR_CA6,
    #[doc = "0x8a4 - General Purpose Register 7 - Store Register command"]
    pub cau_str_ca7: CAU_STR_CA7,
    #[doc = "0x8a8 - General Purpose Register 8 - Store Register command"]
    pub cau_str_ca8: CAU_STR_CA8,
    _reserved38: [u8; 20usize],
    #[doc = "0x8c0 - Status register - Add Register command"]
    pub cau_adr_casr: CAU_ADR_CASR,
    #[doc = "0x8c4 - Accumulator register - Add to register command"]
    pub cau_adr_caa: CAU_ADR_CAA,
    #[doc = "0x8c8 - General Purpose Register 0 - Add to register command"]
    pub cau_adr_ca0: CAU_ADR_CA0,
    #[doc = "0x8cc - General Purpose Register 1 - Add to register command"]
    pub cau_adr_ca1: CAU_ADR_CA1,
    #[doc = "0x8d0 - General Purpose Register 2 - Add to register command"]
    pub cau_adr_ca2: CAU_ADR_CA2,
    #[doc = "0x8d4 - General Purpose Register 3 - Add to register command"]
    pub cau_adr_ca3: CAU_ADR_CA3,
    #[doc = "0x8d8 - General Purpose Register 4 - Add to register command"]
    pub cau_adr_ca4: CAU_ADR_CA4,
    #[doc = "0x8dc - General Purpose Register 5 - Add to register command"]
    pub cau_adr_ca5: CAU_ADR_CA5,
    #[doc = "0x8e0 - General Purpose Register 6 - Add to register command"]
    pub cau_adr_ca6: CAU_ADR_CA6,
    #[doc = "0x8e4 - General Purpose Register 7 - Add to register command"]
    pub cau_adr_ca7: CAU_ADR_CA7,
    #[doc = "0x8e8 - General Purpose Register 8 - Add to register command"]
    pub cau_adr_ca8: CAU_ADR_CA8,
    _reserved49: [u8; 20usize],
    #[doc = "0x900 - Status register - Reverse and Add to Register command"]
    pub cau_radr_casr: CAU_RADR_CASR,
    #[doc = "0x904 - Accumulator register - Reverse and Add to Register command"]
    pub cau_radr_caa: CAU_RADR_CAA,
    #[doc = "0x908 - General Purpose Register 0 - Reverse and Add to Register command"]
    pub cau_radr_ca0: CAU_RADR_CA0,
    #[doc = "0x90c - General Purpose Register 1 - Reverse and Add to Register command"]
    pub cau_radr_ca1: CAU_RADR_CA1,
    #[doc = "0x910 - General Purpose Register 2 - Reverse and Add to Register command"]
    pub cau_radr_ca2: CAU_RADR_CA2,
    #[doc = "0x914 - General Purpose Register 3 - Reverse and Add to Register command"]
    pub cau_radr_ca3: CAU_RADR_CA3,
    #[doc = "0x918 - General Purpose Register 4 - Reverse and Add to Register command"]
    pub cau_radr_ca4: CAU_RADR_CA4,
    #[doc = "0x91c - General Purpose Register 5 - Reverse and Add to Register command"]
    pub cau_radr_ca5: CAU_RADR_CA5,
    #[doc = "0x920 - General Purpose Register 6 - Reverse and Add to Register command"]
    pub cau_radr_ca6: CAU_RADR_CA6,
    #[doc = "0x924 - General Purpose Register 7 - Reverse and Add to Register command"]
    pub cau_radr_ca7: CAU_RADR_CA7,
    #[doc = "0x928 - General Purpose Register 8 - Reverse and Add to Register command"]
    pub cau_radr_ca8: CAU_RADR_CA8,
    _reserved60: [u8; 84usize],
    #[doc = "0x980 - Status register - Exclusive Or command"]
    pub cau_xor_casr: CAU_XOR_CASR,
    #[doc = "0x984 - Accumulator register - Exclusive Or command"]
    pub cau_xor_caa: CAU_XOR_CAA,
    #[doc = "0x988 - General Purpose Register 0 - Exclusive Or command"]
    pub cau_xor_ca0: CAU_XOR_CA0,
    #[doc = "0x98c - General Purpose Register 1 - Exclusive Or command"]
    pub cau_xor_ca1: CAU_XOR_CA1,
    #[doc = "0x990 - General Purpose Register 2 - Exclusive Or command"]
    pub cau_xor_ca2: CAU_XOR_CA2,
    #[doc = "0x994 - General Purpose Register 3 - Exclusive Or command"]
    pub cau_xor_ca3: CAU_XOR_CA3,
    #[doc = "0x998 - General Purpose Register 4 - Exclusive Or command"]
    pub cau_xor_ca4: CAU_XOR_CA4,
    #[doc = "0x99c - General Purpose Register 5 - Exclusive Or command"]
    pub cau_xor_ca5: CAU_XOR_CA5,
    #[doc = "0x9a0 - General Purpose Register 6 - Exclusive Or command"]
    pub cau_xor_ca6: CAU_XOR_CA6,
    #[doc = "0x9a4 - General Purpose Register 7 - Exclusive Or command"]
    pub cau_xor_ca7: CAU_XOR_CA7,
    #[doc = "0x9a8 - General Purpose Register 8 - Exclusive Or command"]
    pub cau_xor_ca8: CAU_XOR_CA8,
    _reserved71: [u8; 20usize],
    #[doc = "0x9c0 - Status register - Rotate Left command"]
    pub cau_rotl_casr: CAU_ROTL_CASR,
    #[doc = "0x9c4 - Accumulator register - Rotate Left command"]
    pub cau_rotl_caa: CAU_ROTL_CAA,
    #[doc = "0x9c8 - General Purpose Register 0 - Rotate Left command"]
    pub cau_rotl_ca0: CAU_ROTL_CA0,
    #[doc = "0x9cc - General Purpose Register 1 - Rotate Left command"]
    pub cau_rotl_ca1: CAU_ROTL_CA1,
    #[doc = "0x9d0 - General Purpose Register 2 - Rotate Left command"]
    pub cau_rotl_ca2: CAU_ROTL_CA2,
    #[doc = "0x9d4 - General Purpose Register 3 - Rotate Left command"]
    pub cau_rotl_ca3: CAU_ROTL_CA3,
    #[doc = "0x9d8 - General Purpose Register 4 - Rotate Left command"]
    pub cau_rotl_ca4: CAU_ROTL_CA4,
    #[doc = "0x9dc - General Purpose Register 5 - Rotate Left command"]
    pub cau_rotl_ca5: CAU_ROTL_CA5,
    #[doc = "0x9e0 - General Purpose Register 6 - Rotate Left command"]
    pub cau_rotl_ca6: CAU_ROTL_CA6,
    #[doc = "0x9e4 - General Purpose Register 7 - Rotate Left command"]
    pub cau_rotl_ca7: CAU_ROTL_CA7,
    #[doc = "0x9e8 - General Purpose Register 8 - Rotate Left command"]
    pub cau_rotl_ca8: CAU_ROTL_CA8,
    _reserved82: [u8; 276usize],
    #[doc = "0xb00 - Status register - AES Column Operation command"]
    pub cau_aesc_casr: CAU_AESC_CASR,
    #[doc = "0xb04 - Accumulator register - AES Column Operation command"]
    pub cau_aesc_caa: CAU_AESC_CAA,
    #[doc = "0xb08 - General Purpose Register 0 - AES Column Operation command"]
    pub cau_aesc_ca0: CAU_AESC_CA0,
    #[doc = "0xb0c - General Purpose Register 1 - AES Column Operation command"]
    pub cau_aesc_ca1: CAU_AESC_CA1,
    #[doc = "0xb10 - General Purpose Register 2 - AES Column Operation command"]
    pub cau_aesc_ca2: CAU_AESC_CA2,
    #[doc = "0xb14 - General Purpose Register 3 - AES Column Operation command"]
    pub cau_aesc_ca3: CAU_AESC_CA3,
    #[doc = "0xb18 - General Purpose Register 4 - AES Column Operation command"]
    pub cau_aesc_ca4: CAU_AESC_CA4,
    #[doc = "0xb1c - General Purpose Register 5 - AES Column Operation command"]
    pub cau_aesc_ca5: CAU_AESC_CA5,
    #[doc = "0xb20 - General Purpose Register 6 - AES Column Operation command"]
    pub cau_aesc_ca6: CAU_AESC_CA6,
    #[doc = "0xb24 - General Purpose Register 7 - AES Column Operation command"]
    pub cau_aesc_ca7: CAU_AESC_CA7,
    #[doc = "0xb28 - General Purpose Register 8 - AES Column Operation command"]
    pub cau_aesc_ca8: CAU_AESC_CA8,
    _reserved93: [u8; 20usize],
    #[doc = "0xb40 - Status register - AES Inverse Column Operation command"]
    pub cau_aesic_casr: CAU_AESIC_CASR,
    #[doc = "0xb44 - Accumulator register - AES Inverse Column Operation command"]
    pub cau_aesic_caa: CAU_AESIC_CAA,
    #[doc = "0xb48 - General Purpose Register 0 - AES Inverse Column Operation command"]
    pub cau_aesic_ca0: CAU_AESIC_CA0,
    #[doc = "0xb4c - General Purpose Register 1 - AES Inverse Column Operation command"]
    pub cau_aesic_ca1: CAU_AESIC_CA1,
    #[doc = "0xb50 - General Purpose Register 2 - AES Inverse Column Operation command"]
    pub cau_aesic_ca2: CAU_AESIC_CA2,
    #[doc = "0xb54 - General Purpose Register 3 - AES Inverse Column Operation command"]
    pub cau_aesic_ca3: CAU_AESIC_CA3,
    #[doc = "0xb58 - General Purpose Register 4 - AES Inverse Column Operation command"]
    pub cau_aesic_ca4: CAU_AESIC_CA4,
    #[doc = "0xb5c - General Purpose Register 5 - AES Inverse Column Operation command"]
    pub cau_aesic_ca5: CAU_AESIC_CA5,
    #[doc = "0xb60 - General Purpose Register 6 - AES Inverse Column Operation command"]
    pub cau_aesic_ca6: CAU_AESIC_CA6,
    #[doc = "0xb64 - General Purpose Register 7 - AES Inverse Column Operation command"]
    pub cau_aesic_ca7: CAU_AESIC_CA7,
    #[doc = "0xb68 - General Purpose Register 8 - AES Inverse Column Operation command"]
    pub cau_aesic_ca8: CAU_AESIC_CA8,
}
#[doc = "Direct access register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct0](cau_direct0) module"]
pub type CAU_DIRECT0 = crate::Reg<u32, _CAU_DIRECT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT0;
#[doc = "`write(|w| ..)` method takes [cau_direct0::W](cau_direct0::W) writer structure"]
impl crate::Writable for CAU_DIRECT0 {}
#[doc = "Direct access register 0"]
pub mod cau_direct0;
#[doc = "Direct access register 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct1](cau_direct1) module"]
pub type CAU_DIRECT1 = crate::Reg<u32, _CAU_DIRECT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT1;
#[doc = "`write(|w| ..)` method takes [cau_direct1::W](cau_direct1::W) writer structure"]
impl crate::Writable for CAU_DIRECT1 {}
#[doc = "Direct access register 1"]
pub mod cau_direct1;
#[doc = "Direct access register 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct2](cau_direct2) module"]
pub type CAU_DIRECT2 = crate::Reg<u32, _CAU_DIRECT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT2;
#[doc = "`write(|w| ..)` method takes [cau_direct2::W](cau_direct2::W) writer structure"]
impl crate::Writable for CAU_DIRECT2 {}
#[doc = "Direct access register 2"]
pub mod cau_direct2;
#[doc = "Direct access register 3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct3](cau_direct3) module"]
pub type CAU_DIRECT3 = crate::Reg<u32, _CAU_DIRECT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT3;
#[doc = "`write(|w| ..)` method takes [cau_direct3::W](cau_direct3::W) writer structure"]
impl crate::Writable for CAU_DIRECT3 {}
#[doc = "Direct access register 3"]
pub mod cau_direct3;
#[doc = "Direct access register 4\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct4](cau_direct4) module"]
pub type CAU_DIRECT4 = crate::Reg<u32, _CAU_DIRECT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT4;
#[doc = "`write(|w| ..)` method takes [cau_direct4::W](cau_direct4::W) writer structure"]
impl crate::Writable for CAU_DIRECT4 {}
#[doc = "Direct access register 4"]
pub mod cau_direct4;
#[doc = "Direct access register 5\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct5](cau_direct5) module"]
pub type CAU_DIRECT5 = crate::Reg<u32, _CAU_DIRECT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT5;
#[doc = "`write(|w| ..)` method takes [cau_direct5::W](cau_direct5::W) writer structure"]
impl crate::Writable for CAU_DIRECT5 {}
#[doc = "Direct access register 5"]
pub mod cau_direct5;
#[doc = "Direct access register 6\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct6](cau_direct6) module"]
pub type CAU_DIRECT6 = crate::Reg<u32, _CAU_DIRECT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT6;
#[doc = "`write(|w| ..)` method takes [cau_direct6::W](cau_direct6::W) writer structure"]
impl crate::Writable for CAU_DIRECT6 {}
#[doc = "Direct access register 6"]
pub mod cau_direct6;
#[doc = "Direct access register 7\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct7](cau_direct7) module"]
pub type CAU_DIRECT7 = crate::Reg<u32, _CAU_DIRECT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT7;
#[doc = "`write(|w| ..)` method takes [cau_direct7::W](cau_direct7::W) writer structure"]
impl crate::Writable for CAU_DIRECT7 {}
#[doc = "Direct access register 7"]
pub mod cau_direct7;
#[doc = "Direct access register 8\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct8](cau_direct8) module"]
pub type CAU_DIRECT8 = crate::Reg<u32, _CAU_DIRECT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT8;
#[doc = "`write(|w| ..)` method takes [cau_direct8::W](cau_direct8::W) writer structure"]
impl crate::Writable for CAU_DIRECT8 {}
#[doc = "Direct access register 8"]
pub mod cau_direct8;
#[doc = "Direct access register 9\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct9](cau_direct9) module"]
pub type CAU_DIRECT9 = crate::Reg<u32, _CAU_DIRECT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT9;
#[doc = "`write(|w| ..)` method takes [cau_direct9::W](cau_direct9::W) writer structure"]
impl crate::Writable for CAU_DIRECT9 {}
#[doc = "Direct access register 9"]
pub mod cau_direct9;
#[doc = "Direct access register 10\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct10](cau_direct10) module"]
pub type CAU_DIRECT10 = crate::Reg<u32, _CAU_DIRECT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT10;
#[doc = "`write(|w| ..)` method takes [cau_direct10::W](cau_direct10::W) writer structure"]
impl crate::Writable for CAU_DIRECT10 {}
#[doc = "Direct access register 10"]
pub mod cau_direct10;
#[doc = "Direct access register 11\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct11](cau_direct11) module"]
pub type CAU_DIRECT11 = crate::Reg<u32, _CAU_DIRECT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT11;
#[doc = "`write(|w| ..)` method takes [cau_direct11::W](cau_direct11::W) writer structure"]
impl crate::Writable for CAU_DIRECT11 {}
#[doc = "Direct access register 11"]
pub mod cau_direct11;
#[doc = "Direct access register 12\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct12](cau_direct12) module"]
pub type CAU_DIRECT12 = crate::Reg<u32, _CAU_DIRECT12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT12;
#[doc = "`write(|w| ..)` method takes [cau_direct12::W](cau_direct12::W) writer structure"]
impl crate::Writable for CAU_DIRECT12 {}
#[doc = "Direct access register 12"]
pub mod cau_direct12;
#[doc = "Direct access register 13\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct13](cau_direct13) module"]
pub type CAU_DIRECT13 = crate::Reg<u32, _CAU_DIRECT13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT13;
#[doc = "`write(|w| ..)` method takes [cau_direct13::W](cau_direct13::W) writer structure"]
impl crate::Writable for CAU_DIRECT13 {}
#[doc = "Direct access register 13"]
pub mod cau_direct13;
#[doc = "Direct access register 14\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct14](cau_direct14) module"]
pub type CAU_DIRECT14 = crate::Reg<u32, _CAU_DIRECT14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT14;
#[doc = "`write(|w| ..)` method takes [cau_direct14::W](cau_direct14::W) writer structure"]
impl crate::Writable for CAU_DIRECT14 {}
#[doc = "Direct access register 14"]
pub mod cau_direct14;
#[doc = "Direct access register 15\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_direct15](cau_direct15) module"]
pub type CAU_DIRECT15 = crate::Reg<u32, _CAU_DIRECT15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_DIRECT15;
#[doc = "`write(|w| ..)` method takes [cau_direct15::W](cau_direct15::W) writer structure"]
impl crate::Writable for CAU_DIRECT15 {}
#[doc = "Direct access register 15"]
pub mod cau_direct15;
#[doc = "Status register - Load Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_ldr_casr](cau_ldr_casr) module"]
pub type CAU_LDR_CASR = crate::Reg<u32, _CAU_LDR_CASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_LDR_CASR;
#[doc = "`write(|w| ..)` method takes [cau_ldr_casr::W](cau_ldr_casr::W) writer structure"]
impl crate::Writable for CAU_LDR_CASR {}
#[doc = "Status register - Load Register command"]
pub mod cau_ldr_casr;
#[doc = "Accumulator register - Load Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_ldr_caa](cau_ldr_caa) module"]
pub type CAU_LDR_CAA = crate::Reg<u32, _CAU_LDR_CAA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_LDR_CAA;
#[doc = "`write(|w| ..)` method takes [cau_ldr_caa::W](cau_ldr_caa::W) writer structure"]
impl crate::Writable for CAU_LDR_CAA {}
#[doc = "Accumulator register - Load Register command"]
pub mod cau_ldr_caa;
#[doc = "General Purpose Register 0 - Load Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_ldr_ca0](cau_ldr_ca0) module"]
pub type CAU_LDR_CA0 = crate::Reg<u32, _CAU_LDR_CA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_LDR_CA0;
#[doc = "`write(|w| ..)` method takes [cau_ldr_ca0::W](cau_ldr_ca0::W) writer structure"]
impl crate::Writable for CAU_LDR_CA0 {}
#[doc = "General Purpose Register 0 - Load Register command"]
pub mod cau_ldr_ca0;
#[doc = "General Purpose Register 1 - Load Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_ldr_ca1](cau_ldr_ca1) module"]
pub type CAU_LDR_CA1 = crate::Reg<u32, _CAU_LDR_CA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_LDR_CA1;
#[doc = "`write(|w| ..)` method takes [cau_ldr_ca1::W](cau_ldr_ca1::W) writer structure"]
impl crate::Writable for CAU_LDR_CA1 {}
#[doc = "General Purpose Register 1 - Load Register command"]
pub mod cau_ldr_ca1;
#[doc = "General Purpose Register 2 - Load Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_ldr_ca2](cau_ldr_ca2) module"]
pub type CAU_LDR_CA2 = crate::Reg<u32, _CAU_LDR_CA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_LDR_CA2;
#[doc = "`write(|w| ..)` method takes [cau_ldr_ca2::W](cau_ldr_ca2::W) writer structure"]
impl crate::Writable for CAU_LDR_CA2 {}
#[doc = "General Purpose Register 2 - Load Register command"]
pub mod cau_ldr_ca2;
#[doc = "General Purpose Register 3 - Load Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_ldr_ca3](cau_ldr_ca3) module"]
pub type CAU_LDR_CA3 = crate::Reg<u32, _CAU_LDR_CA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_LDR_CA3;
#[doc = "`write(|w| ..)` method takes [cau_ldr_ca3::W](cau_ldr_ca3::W) writer structure"]
impl crate::Writable for CAU_LDR_CA3 {}
#[doc = "General Purpose Register 3 - Load Register command"]
pub mod cau_ldr_ca3;
#[doc = "General Purpose Register 4 - Load Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_ldr_ca4](cau_ldr_ca4) module"]
pub type CAU_LDR_CA4 = crate::Reg<u32, _CAU_LDR_CA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_LDR_CA4;
#[doc = "`write(|w| ..)` method takes [cau_ldr_ca4::W](cau_ldr_ca4::W) writer structure"]
impl crate::Writable for CAU_LDR_CA4 {}
#[doc = "General Purpose Register 4 - Load Register command"]
pub mod cau_ldr_ca4;
#[doc = "General Purpose Register 5 - Load Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_ldr_ca5](cau_ldr_ca5) module"]
pub type CAU_LDR_CA5 = crate::Reg<u32, _CAU_LDR_CA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_LDR_CA5;
#[doc = "`write(|w| ..)` method takes [cau_ldr_ca5::W](cau_ldr_ca5::W) writer structure"]
impl crate::Writable for CAU_LDR_CA5 {}
#[doc = "General Purpose Register 5 - Load Register command"]
pub mod cau_ldr_ca5;
#[doc = "General Purpose Register 6 - Load Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_ldr_ca6](cau_ldr_ca6) module"]
pub type CAU_LDR_CA6 = crate::Reg<u32, _CAU_LDR_CA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_LDR_CA6;
#[doc = "`write(|w| ..)` method takes [cau_ldr_ca6::W](cau_ldr_ca6::W) writer structure"]
impl crate::Writable for CAU_LDR_CA6 {}
#[doc = "General Purpose Register 6 - Load Register command"]
pub mod cau_ldr_ca6;
#[doc = "General Purpose Register 7 - Load Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_ldr_ca7](cau_ldr_ca7) module"]
pub type CAU_LDR_CA7 = crate::Reg<u32, _CAU_LDR_CA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_LDR_CA7;
#[doc = "`write(|w| ..)` method takes [cau_ldr_ca7::W](cau_ldr_ca7::W) writer structure"]
impl crate::Writable for CAU_LDR_CA7 {}
#[doc = "General Purpose Register 7 - Load Register command"]
pub mod cau_ldr_ca7;
#[doc = "General Purpose Register 8 - Load Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_ldr_ca8](cau_ldr_ca8) module"]
pub type CAU_LDR_CA8 = crate::Reg<u32, _CAU_LDR_CA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_LDR_CA8;
#[doc = "`write(|w| ..)` method takes [cau_ldr_ca8::W](cau_ldr_ca8::W) writer structure"]
impl crate::Writable for CAU_LDR_CA8 {}
#[doc = "General Purpose Register 8 - Load Register command"]
pub mod cau_ldr_ca8;
#[doc = "Status register - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_str_casr](cau_str_casr) module"]
pub type CAU_STR_CASR = crate::Reg<u32, _CAU_STR_CASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_STR_CASR;
#[doc = "`read()` method returns [cau_str_casr::R](cau_str_casr::R) reader structure"]
impl crate::Readable for CAU_STR_CASR {}
#[doc = "Status register - Store Register command"]
pub mod cau_str_casr;
#[doc = "Accumulator register - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_str_caa](cau_str_caa) module"]
pub type CAU_STR_CAA = crate::Reg<u32, _CAU_STR_CAA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_STR_CAA;
#[doc = "`read()` method returns [cau_str_caa::R](cau_str_caa::R) reader structure"]
impl crate::Readable for CAU_STR_CAA {}
#[doc = "Accumulator register - Store Register command"]
pub mod cau_str_caa;
#[doc = "General Purpose Register 0 - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_str_ca0](cau_str_ca0) module"]
pub type CAU_STR_CA0 = crate::Reg<u32, _CAU_STR_CA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_STR_CA0;
#[doc = "`read()` method returns [cau_str_ca0::R](cau_str_ca0::R) reader structure"]
impl crate::Readable for CAU_STR_CA0 {}
#[doc = "General Purpose Register 0 - Store Register command"]
pub mod cau_str_ca0;
#[doc = "General Purpose Register 1 - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_str_ca1](cau_str_ca1) module"]
pub type CAU_STR_CA1 = crate::Reg<u32, _CAU_STR_CA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_STR_CA1;
#[doc = "`read()` method returns [cau_str_ca1::R](cau_str_ca1::R) reader structure"]
impl crate::Readable for CAU_STR_CA1 {}
#[doc = "General Purpose Register 1 - Store Register command"]
pub mod cau_str_ca1;
#[doc = "General Purpose Register 2 - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_str_ca2](cau_str_ca2) module"]
pub type CAU_STR_CA2 = crate::Reg<u32, _CAU_STR_CA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_STR_CA2;
#[doc = "`read()` method returns [cau_str_ca2::R](cau_str_ca2::R) reader structure"]
impl crate::Readable for CAU_STR_CA2 {}
#[doc = "General Purpose Register 2 - Store Register command"]
pub mod cau_str_ca2;
#[doc = "General Purpose Register 3 - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_str_ca3](cau_str_ca3) module"]
pub type CAU_STR_CA3 = crate::Reg<u32, _CAU_STR_CA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_STR_CA3;
#[doc = "`read()` method returns [cau_str_ca3::R](cau_str_ca3::R) reader structure"]
impl crate::Readable for CAU_STR_CA3 {}
#[doc = "General Purpose Register 3 - Store Register command"]
pub mod cau_str_ca3;
#[doc = "General Purpose Register 4 - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_str_ca4](cau_str_ca4) module"]
pub type CAU_STR_CA4 = crate::Reg<u32, _CAU_STR_CA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_STR_CA4;
#[doc = "`read()` method returns [cau_str_ca4::R](cau_str_ca4::R) reader structure"]
impl crate::Readable for CAU_STR_CA4 {}
#[doc = "General Purpose Register 4 - Store Register command"]
pub mod cau_str_ca4;
#[doc = "General Purpose Register 5 - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_str_ca5](cau_str_ca5) module"]
pub type CAU_STR_CA5 = crate::Reg<u32, _CAU_STR_CA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_STR_CA5;
#[doc = "`read()` method returns [cau_str_ca5::R](cau_str_ca5::R) reader structure"]
impl crate::Readable for CAU_STR_CA5 {}
#[doc = "General Purpose Register 5 - Store Register command"]
pub mod cau_str_ca5;
#[doc = "General Purpose Register 6 - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_str_ca6](cau_str_ca6) module"]
pub type CAU_STR_CA6 = crate::Reg<u32, _CAU_STR_CA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_STR_CA6;
#[doc = "`read()` method returns [cau_str_ca6::R](cau_str_ca6::R) reader structure"]
impl crate::Readable for CAU_STR_CA6 {}
#[doc = "General Purpose Register 6 - Store Register command"]
pub mod cau_str_ca6;
#[doc = "General Purpose Register 7 - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_str_ca7](cau_str_ca7) module"]
pub type CAU_STR_CA7 = crate::Reg<u32, _CAU_STR_CA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_STR_CA7;
#[doc = "`read()` method returns [cau_str_ca7::R](cau_str_ca7::R) reader structure"]
impl crate::Readable for CAU_STR_CA7 {}
#[doc = "General Purpose Register 7 - Store Register command"]
pub mod cau_str_ca7;
#[doc = "General Purpose Register 8 - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_str_ca8](cau_str_ca8) module"]
pub type CAU_STR_CA8 = crate::Reg<u32, _CAU_STR_CA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_STR_CA8;
#[doc = "`read()` method returns [cau_str_ca8::R](cau_str_ca8::R) reader structure"]
impl crate::Readable for CAU_STR_CA8 {}
#[doc = "General Purpose Register 8 - Store Register command"]
pub mod cau_str_ca8;
#[doc = "Status register - Add Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_adr_casr](cau_adr_casr) module"]
pub type CAU_ADR_CASR = crate::Reg<u32, _CAU_ADR_CASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ADR_CASR;
#[doc = "`write(|w| ..)` method takes [cau_adr_casr::W](cau_adr_casr::W) writer structure"]
impl crate::Writable for CAU_ADR_CASR {}
#[doc = "Status register - Add Register command"]
pub mod cau_adr_casr;
#[doc = "Accumulator register - Add to register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_adr_caa](cau_adr_caa) module"]
pub type CAU_ADR_CAA = crate::Reg<u32, _CAU_ADR_CAA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ADR_CAA;
#[doc = "`write(|w| ..)` method takes [cau_adr_caa::W](cau_adr_caa::W) writer structure"]
impl crate::Writable for CAU_ADR_CAA {}
#[doc = "Accumulator register - Add to register command"]
pub mod cau_adr_caa;
#[doc = "General Purpose Register 0 - Add to register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_adr_ca0](cau_adr_ca0) module"]
pub type CAU_ADR_CA0 = crate::Reg<u32, _CAU_ADR_CA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ADR_CA0;
#[doc = "`write(|w| ..)` method takes [cau_adr_ca0::W](cau_adr_ca0::W) writer structure"]
impl crate::Writable for CAU_ADR_CA0 {}
#[doc = "General Purpose Register 0 - Add to register command"]
pub mod cau_adr_ca0;
#[doc = "General Purpose Register 1 - Add to register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_adr_ca1](cau_adr_ca1) module"]
pub type CAU_ADR_CA1 = crate::Reg<u32, _CAU_ADR_CA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ADR_CA1;
#[doc = "`write(|w| ..)` method takes [cau_adr_ca1::W](cau_adr_ca1::W) writer structure"]
impl crate::Writable for CAU_ADR_CA1 {}
#[doc = "General Purpose Register 1 - Add to register command"]
pub mod cau_adr_ca1;
#[doc = "General Purpose Register 2 - Add to register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_adr_ca2](cau_adr_ca2) module"]
pub type CAU_ADR_CA2 = crate::Reg<u32, _CAU_ADR_CA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ADR_CA2;
#[doc = "`write(|w| ..)` method takes [cau_adr_ca2::W](cau_adr_ca2::W) writer structure"]
impl crate::Writable for CAU_ADR_CA2 {}
#[doc = "General Purpose Register 2 - Add to register command"]
pub mod cau_adr_ca2;
#[doc = "General Purpose Register 3 - Add to register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_adr_ca3](cau_adr_ca3) module"]
pub type CAU_ADR_CA3 = crate::Reg<u32, _CAU_ADR_CA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ADR_CA3;
#[doc = "`write(|w| ..)` method takes [cau_adr_ca3::W](cau_adr_ca3::W) writer structure"]
impl crate::Writable for CAU_ADR_CA3 {}
#[doc = "General Purpose Register 3 - Add to register command"]
pub mod cau_adr_ca3;
#[doc = "General Purpose Register 4 - Add to register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_adr_ca4](cau_adr_ca4) module"]
pub type CAU_ADR_CA4 = crate::Reg<u32, _CAU_ADR_CA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ADR_CA4;
#[doc = "`write(|w| ..)` method takes [cau_adr_ca4::W](cau_adr_ca4::W) writer structure"]
impl crate::Writable for CAU_ADR_CA4 {}
#[doc = "General Purpose Register 4 - Add to register command"]
pub mod cau_adr_ca4;
#[doc = "General Purpose Register 5 - Add to register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_adr_ca5](cau_adr_ca5) module"]
pub type CAU_ADR_CA5 = crate::Reg<u32, _CAU_ADR_CA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ADR_CA5;
#[doc = "`write(|w| ..)` method takes [cau_adr_ca5::W](cau_adr_ca5::W) writer structure"]
impl crate::Writable for CAU_ADR_CA5 {}
#[doc = "General Purpose Register 5 - Add to register command"]
pub mod cau_adr_ca5;
#[doc = "General Purpose Register 6 - Add to register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_adr_ca6](cau_adr_ca6) module"]
pub type CAU_ADR_CA6 = crate::Reg<u32, _CAU_ADR_CA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ADR_CA6;
#[doc = "`write(|w| ..)` method takes [cau_adr_ca6::W](cau_adr_ca6::W) writer structure"]
impl crate::Writable for CAU_ADR_CA6 {}
#[doc = "General Purpose Register 6 - Add to register command"]
pub mod cau_adr_ca6;
#[doc = "General Purpose Register 7 - Add to register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_adr_ca7](cau_adr_ca7) module"]
pub type CAU_ADR_CA7 = crate::Reg<u32, _CAU_ADR_CA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ADR_CA7;
#[doc = "`write(|w| ..)` method takes [cau_adr_ca7::W](cau_adr_ca7::W) writer structure"]
impl crate::Writable for CAU_ADR_CA7 {}
#[doc = "General Purpose Register 7 - Add to register command"]
pub mod cau_adr_ca7;
#[doc = "General Purpose Register 8 - Add to register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_adr_ca8](cau_adr_ca8) module"]
pub type CAU_ADR_CA8 = crate::Reg<u32, _CAU_ADR_CA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ADR_CA8;
#[doc = "`write(|w| ..)` method takes [cau_adr_ca8::W](cau_adr_ca8::W) writer structure"]
impl crate::Writable for CAU_ADR_CA8 {}
#[doc = "General Purpose Register 8 - Add to register command"]
pub mod cau_adr_ca8;
#[doc = "Status register - Reverse and Add to Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_radr_casr](cau_radr_casr) module"]
pub type CAU_RADR_CASR = crate::Reg<u32, _CAU_RADR_CASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_RADR_CASR;
#[doc = "`write(|w| ..)` method takes [cau_radr_casr::W](cau_radr_casr::W) writer structure"]
impl crate::Writable for CAU_RADR_CASR {}
#[doc = "Status register - Reverse and Add to Register command"]
pub mod cau_radr_casr;
#[doc = "Accumulator register - Reverse and Add to Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_radr_caa](cau_radr_caa) module"]
pub type CAU_RADR_CAA = crate::Reg<u32, _CAU_RADR_CAA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_RADR_CAA;
#[doc = "`write(|w| ..)` method takes [cau_radr_caa::W](cau_radr_caa::W) writer structure"]
impl crate::Writable for CAU_RADR_CAA {}
#[doc = "Accumulator register - Reverse and Add to Register command"]
pub mod cau_radr_caa;
#[doc = "General Purpose Register 0 - Reverse and Add to Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_radr_ca0](cau_radr_ca0) module"]
pub type CAU_RADR_CA0 = crate::Reg<u32, _CAU_RADR_CA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_RADR_CA0;
#[doc = "`write(|w| ..)` method takes [cau_radr_ca0::W](cau_radr_ca0::W) writer structure"]
impl crate::Writable for CAU_RADR_CA0 {}
#[doc = "General Purpose Register 0 - Reverse and Add to Register command"]
pub mod cau_radr_ca0;
#[doc = "General Purpose Register 1 - Reverse and Add to Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_radr_ca1](cau_radr_ca1) module"]
pub type CAU_RADR_CA1 = crate::Reg<u32, _CAU_RADR_CA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_RADR_CA1;
#[doc = "`write(|w| ..)` method takes [cau_radr_ca1::W](cau_radr_ca1::W) writer structure"]
impl crate::Writable for CAU_RADR_CA1 {}
#[doc = "General Purpose Register 1 - Reverse and Add to Register command"]
pub mod cau_radr_ca1;
#[doc = "General Purpose Register 2 - Reverse and Add to Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_radr_ca2](cau_radr_ca2) module"]
pub type CAU_RADR_CA2 = crate::Reg<u32, _CAU_RADR_CA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_RADR_CA2;
#[doc = "`write(|w| ..)` method takes [cau_radr_ca2::W](cau_radr_ca2::W) writer structure"]
impl crate::Writable for CAU_RADR_CA2 {}
#[doc = "General Purpose Register 2 - Reverse and Add to Register command"]
pub mod cau_radr_ca2;
#[doc = "General Purpose Register 3 - Reverse and Add to Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_radr_ca3](cau_radr_ca3) module"]
pub type CAU_RADR_CA3 = crate::Reg<u32, _CAU_RADR_CA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_RADR_CA3;
#[doc = "`write(|w| ..)` method takes [cau_radr_ca3::W](cau_radr_ca3::W) writer structure"]
impl crate::Writable for CAU_RADR_CA3 {}
#[doc = "General Purpose Register 3 - Reverse and Add to Register command"]
pub mod cau_radr_ca3;
#[doc = "General Purpose Register 4 - Reverse and Add to Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_radr_ca4](cau_radr_ca4) module"]
pub type CAU_RADR_CA4 = crate::Reg<u32, _CAU_RADR_CA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_RADR_CA4;
#[doc = "`write(|w| ..)` method takes [cau_radr_ca4::W](cau_radr_ca4::W) writer structure"]
impl crate::Writable for CAU_RADR_CA4 {}
#[doc = "General Purpose Register 4 - Reverse and Add to Register command"]
pub mod cau_radr_ca4;
#[doc = "General Purpose Register 5 - Reverse and Add to Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_radr_ca5](cau_radr_ca5) module"]
pub type CAU_RADR_CA5 = crate::Reg<u32, _CAU_RADR_CA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_RADR_CA5;
#[doc = "`write(|w| ..)` method takes [cau_radr_ca5::W](cau_radr_ca5::W) writer structure"]
impl crate::Writable for CAU_RADR_CA5 {}
#[doc = "General Purpose Register 5 - Reverse and Add to Register command"]
pub mod cau_radr_ca5;
#[doc = "General Purpose Register 6 - Reverse and Add to Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_radr_ca6](cau_radr_ca6) module"]
pub type CAU_RADR_CA6 = crate::Reg<u32, _CAU_RADR_CA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_RADR_CA6;
#[doc = "`write(|w| ..)` method takes [cau_radr_ca6::W](cau_radr_ca6::W) writer structure"]
impl crate::Writable for CAU_RADR_CA6 {}
#[doc = "General Purpose Register 6 - Reverse and Add to Register command"]
pub mod cau_radr_ca6;
#[doc = "General Purpose Register 7 - Reverse and Add to Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_radr_ca7](cau_radr_ca7) module"]
pub type CAU_RADR_CA7 = crate::Reg<u32, _CAU_RADR_CA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_RADR_CA7;
#[doc = "`write(|w| ..)` method takes [cau_radr_ca7::W](cau_radr_ca7::W) writer structure"]
impl crate::Writable for CAU_RADR_CA7 {}
#[doc = "General Purpose Register 7 - Reverse and Add to Register command"]
pub mod cau_radr_ca7;
#[doc = "General Purpose Register 8 - Reverse and Add to Register command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_radr_ca8](cau_radr_ca8) module"]
pub type CAU_RADR_CA8 = crate::Reg<u32, _CAU_RADR_CA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_RADR_CA8;
#[doc = "`write(|w| ..)` method takes [cau_radr_ca8::W](cau_radr_ca8::W) writer structure"]
impl crate::Writable for CAU_RADR_CA8 {}
#[doc = "General Purpose Register 8 - Reverse and Add to Register command"]
pub mod cau_radr_ca8;
#[doc = "Status register - Exclusive Or command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_xor_casr](cau_xor_casr) module"]
pub type CAU_XOR_CASR = crate::Reg<u32, _CAU_XOR_CASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_XOR_CASR;
#[doc = "`write(|w| ..)` method takes [cau_xor_casr::W](cau_xor_casr::W) writer structure"]
impl crate::Writable for CAU_XOR_CASR {}
#[doc = "Status register - Exclusive Or command"]
pub mod cau_xor_casr;
#[doc = "Accumulator register - Exclusive Or command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_xor_caa](cau_xor_caa) module"]
pub type CAU_XOR_CAA = crate::Reg<u32, _CAU_XOR_CAA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_XOR_CAA;
#[doc = "`write(|w| ..)` method takes [cau_xor_caa::W](cau_xor_caa::W) writer structure"]
impl crate::Writable for CAU_XOR_CAA {}
#[doc = "Accumulator register - Exclusive Or command"]
pub mod cau_xor_caa;
#[doc = "General Purpose Register 0 - Exclusive Or command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_xor_ca0](cau_xor_ca0) module"]
pub type CAU_XOR_CA0 = crate::Reg<u32, _CAU_XOR_CA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_XOR_CA0;
#[doc = "`write(|w| ..)` method takes [cau_xor_ca0::W](cau_xor_ca0::W) writer structure"]
impl crate::Writable for CAU_XOR_CA0 {}
#[doc = "General Purpose Register 0 - Exclusive Or command"]
pub mod cau_xor_ca0;
#[doc = "General Purpose Register 1 - Exclusive Or command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_xor_ca1](cau_xor_ca1) module"]
pub type CAU_XOR_CA1 = crate::Reg<u32, _CAU_XOR_CA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_XOR_CA1;
#[doc = "`write(|w| ..)` method takes [cau_xor_ca1::W](cau_xor_ca1::W) writer structure"]
impl crate::Writable for CAU_XOR_CA1 {}
#[doc = "General Purpose Register 1 - Exclusive Or command"]
pub mod cau_xor_ca1;
#[doc = "General Purpose Register 2 - Exclusive Or command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_xor_ca2](cau_xor_ca2) module"]
pub type CAU_XOR_CA2 = crate::Reg<u32, _CAU_XOR_CA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_XOR_CA2;
#[doc = "`write(|w| ..)` method takes [cau_xor_ca2::W](cau_xor_ca2::W) writer structure"]
impl crate::Writable for CAU_XOR_CA2 {}
#[doc = "General Purpose Register 2 - Exclusive Or command"]
pub mod cau_xor_ca2;
#[doc = "General Purpose Register 3 - Exclusive Or command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_xor_ca3](cau_xor_ca3) module"]
pub type CAU_XOR_CA3 = crate::Reg<u32, _CAU_XOR_CA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_XOR_CA3;
#[doc = "`write(|w| ..)` method takes [cau_xor_ca3::W](cau_xor_ca3::W) writer structure"]
impl crate::Writable for CAU_XOR_CA3 {}
#[doc = "General Purpose Register 3 - Exclusive Or command"]
pub mod cau_xor_ca3;
#[doc = "General Purpose Register 4 - Exclusive Or command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_xor_ca4](cau_xor_ca4) module"]
pub type CAU_XOR_CA4 = crate::Reg<u32, _CAU_XOR_CA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_XOR_CA4;
#[doc = "`write(|w| ..)` method takes [cau_xor_ca4::W](cau_xor_ca4::W) writer structure"]
impl crate::Writable for CAU_XOR_CA4 {}
#[doc = "General Purpose Register 4 - Exclusive Or command"]
pub mod cau_xor_ca4;
#[doc = "General Purpose Register 5 - Exclusive Or command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_xor_ca5](cau_xor_ca5) module"]
pub type CAU_XOR_CA5 = crate::Reg<u32, _CAU_XOR_CA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_XOR_CA5;
#[doc = "`write(|w| ..)` method takes [cau_xor_ca5::W](cau_xor_ca5::W) writer structure"]
impl crate::Writable for CAU_XOR_CA5 {}
#[doc = "General Purpose Register 5 - Exclusive Or command"]
pub mod cau_xor_ca5;
#[doc = "General Purpose Register 6 - Exclusive Or command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_xor_ca6](cau_xor_ca6) module"]
pub type CAU_XOR_CA6 = crate::Reg<u32, _CAU_XOR_CA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_XOR_CA6;
#[doc = "`write(|w| ..)` method takes [cau_xor_ca6::W](cau_xor_ca6::W) writer structure"]
impl crate::Writable for CAU_XOR_CA6 {}
#[doc = "General Purpose Register 6 - Exclusive Or command"]
pub mod cau_xor_ca6;
#[doc = "General Purpose Register 7 - Exclusive Or command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_xor_ca7](cau_xor_ca7) module"]
pub type CAU_XOR_CA7 = crate::Reg<u32, _CAU_XOR_CA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_XOR_CA7;
#[doc = "`write(|w| ..)` method takes [cau_xor_ca7::W](cau_xor_ca7::W) writer structure"]
impl crate::Writable for CAU_XOR_CA7 {}
#[doc = "General Purpose Register 7 - Exclusive Or command"]
pub mod cau_xor_ca7;
#[doc = "General Purpose Register 8 - Exclusive Or command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_xor_ca8](cau_xor_ca8) module"]
pub type CAU_XOR_CA8 = crate::Reg<u32, _CAU_XOR_CA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_XOR_CA8;
#[doc = "`write(|w| ..)` method takes [cau_xor_ca8::W](cau_xor_ca8::W) writer structure"]
impl crate::Writable for CAU_XOR_CA8 {}
#[doc = "General Purpose Register 8 - Exclusive Or command"]
pub mod cau_xor_ca8;
#[doc = "Status register - Rotate Left command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_rotl_casr](cau_rotl_casr) module"]
pub type CAU_ROTL_CASR = crate::Reg<u32, _CAU_ROTL_CASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ROTL_CASR;
#[doc = "`write(|w| ..)` method takes [cau_rotl_casr::W](cau_rotl_casr::W) writer structure"]
impl crate::Writable for CAU_ROTL_CASR {}
#[doc = "Status register - Rotate Left command"]
pub mod cau_rotl_casr;
#[doc = "Accumulator register - Rotate Left command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_rotl_caa](cau_rotl_caa) module"]
pub type CAU_ROTL_CAA = crate::Reg<u32, _CAU_ROTL_CAA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ROTL_CAA;
#[doc = "`write(|w| ..)` method takes [cau_rotl_caa::W](cau_rotl_caa::W) writer structure"]
impl crate::Writable for CAU_ROTL_CAA {}
#[doc = "Accumulator register - Rotate Left command"]
pub mod cau_rotl_caa;
#[doc = "General Purpose Register 0 - Rotate Left command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_rotl_ca0](cau_rotl_ca0) module"]
pub type CAU_ROTL_CA0 = crate::Reg<u32, _CAU_ROTL_CA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ROTL_CA0;
#[doc = "`write(|w| ..)` method takes [cau_rotl_ca0::W](cau_rotl_ca0::W) writer structure"]
impl crate::Writable for CAU_ROTL_CA0 {}
#[doc = "General Purpose Register 0 - Rotate Left command"]
pub mod cau_rotl_ca0;
#[doc = "General Purpose Register 1 - Rotate Left command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_rotl_ca1](cau_rotl_ca1) module"]
pub type CAU_ROTL_CA1 = crate::Reg<u32, _CAU_ROTL_CA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ROTL_CA1;
#[doc = "`write(|w| ..)` method takes [cau_rotl_ca1::W](cau_rotl_ca1::W) writer structure"]
impl crate::Writable for CAU_ROTL_CA1 {}
#[doc = "General Purpose Register 1 - Rotate Left command"]
pub mod cau_rotl_ca1;
#[doc = "General Purpose Register 2 - Rotate Left command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_rotl_ca2](cau_rotl_ca2) module"]
pub type CAU_ROTL_CA2 = crate::Reg<u32, _CAU_ROTL_CA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ROTL_CA2;
#[doc = "`write(|w| ..)` method takes [cau_rotl_ca2::W](cau_rotl_ca2::W) writer structure"]
impl crate::Writable for CAU_ROTL_CA2 {}
#[doc = "General Purpose Register 2 - Rotate Left command"]
pub mod cau_rotl_ca2;
#[doc = "General Purpose Register 3 - Rotate Left command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_rotl_ca3](cau_rotl_ca3) module"]
pub type CAU_ROTL_CA3 = crate::Reg<u32, _CAU_ROTL_CA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ROTL_CA3;
#[doc = "`write(|w| ..)` method takes [cau_rotl_ca3::W](cau_rotl_ca3::W) writer structure"]
impl crate::Writable for CAU_ROTL_CA3 {}
#[doc = "General Purpose Register 3 - Rotate Left command"]
pub mod cau_rotl_ca3;
#[doc = "General Purpose Register 4 - Rotate Left command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_rotl_ca4](cau_rotl_ca4) module"]
pub type CAU_ROTL_CA4 = crate::Reg<u32, _CAU_ROTL_CA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ROTL_CA4;
#[doc = "`write(|w| ..)` method takes [cau_rotl_ca4::W](cau_rotl_ca4::W) writer structure"]
impl crate::Writable for CAU_ROTL_CA4 {}
#[doc = "General Purpose Register 4 - Rotate Left command"]
pub mod cau_rotl_ca4;
#[doc = "General Purpose Register 5 - Rotate Left command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_rotl_ca5](cau_rotl_ca5) module"]
pub type CAU_ROTL_CA5 = crate::Reg<u32, _CAU_ROTL_CA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ROTL_CA5;
#[doc = "`write(|w| ..)` method takes [cau_rotl_ca5::W](cau_rotl_ca5::W) writer structure"]
impl crate::Writable for CAU_ROTL_CA5 {}
#[doc = "General Purpose Register 5 - Rotate Left command"]
pub mod cau_rotl_ca5;
#[doc = "General Purpose Register 6 - Rotate Left command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_rotl_ca6](cau_rotl_ca6) module"]
pub type CAU_ROTL_CA6 = crate::Reg<u32, _CAU_ROTL_CA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ROTL_CA6;
#[doc = "`write(|w| ..)` method takes [cau_rotl_ca6::W](cau_rotl_ca6::W) writer structure"]
impl crate::Writable for CAU_ROTL_CA6 {}
#[doc = "General Purpose Register 6 - Rotate Left command"]
pub mod cau_rotl_ca6;
#[doc = "General Purpose Register 7 - Rotate Left command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_rotl_ca7](cau_rotl_ca7) module"]
pub type CAU_ROTL_CA7 = crate::Reg<u32, _CAU_ROTL_CA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ROTL_CA7;
#[doc = "`write(|w| ..)` method takes [cau_rotl_ca7::W](cau_rotl_ca7::W) writer structure"]
impl crate::Writable for CAU_ROTL_CA7 {}
#[doc = "General Purpose Register 7 - Rotate Left command"]
pub mod cau_rotl_ca7;
#[doc = "General Purpose Register 8 - Rotate Left command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_rotl_ca8](cau_rotl_ca8) module"]
pub type CAU_ROTL_CA8 = crate::Reg<u32, _CAU_ROTL_CA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_ROTL_CA8;
#[doc = "`write(|w| ..)` method takes [cau_rotl_ca8::W](cau_rotl_ca8::W) writer structure"]
impl crate::Writable for CAU_ROTL_CA8 {}
#[doc = "General Purpose Register 8 - Rotate Left command"]
pub mod cau_rotl_ca8;
#[doc = "Status register - AES Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesc_casr](cau_aesc_casr) module"]
pub type CAU_AESC_CASR = crate::Reg<u32, _CAU_AESC_CASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESC_CASR;
#[doc = "`write(|w| ..)` method takes [cau_aesc_casr::W](cau_aesc_casr::W) writer structure"]
impl crate::Writable for CAU_AESC_CASR {}
#[doc = "Status register - AES Column Operation command"]
pub mod cau_aesc_casr;
#[doc = "Accumulator register - AES Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesc_caa](cau_aesc_caa) module"]
pub type CAU_AESC_CAA = crate::Reg<u32, _CAU_AESC_CAA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESC_CAA;
#[doc = "`write(|w| ..)` method takes [cau_aesc_caa::W](cau_aesc_caa::W) writer structure"]
impl crate::Writable for CAU_AESC_CAA {}
#[doc = "Accumulator register - AES Column Operation command"]
pub mod cau_aesc_caa;
#[doc = "General Purpose Register 0 - AES Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesc_ca0](cau_aesc_ca0) module"]
pub type CAU_AESC_CA0 = crate::Reg<u32, _CAU_AESC_CA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESC_CA0;
#[doc = "`write(|w| ..)` method takes [cau_aesc_ca0::W](cau_aesc_ca0::W) writer structure"]
impl crate::Writable for CAU_AESC_CA0 {}
#[doc = "General Purpose Register 0 - AES Column Operation command"]
pub mod cau_aesc_ca0;
#[doc = "General Purpose Register 1 - AES Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesc_ca1](cau_aesc_ca1) module"]
pub type CAU_AESC_CA1 = crate::Reg<u32, _CAU_AESC_CA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESC_CA1;
#[doc = "`write(|w| ..)` method takes [cau_aesc_ca1::W](cau_aesc_ca1::W) writer structure"]
impl crate::Writable for CAU_AESC_CA1 {}
#[doc = "General Purpose Register 1 - AES Column Operation command"]
pub mod cau_aesc_ca1;
#[doc = "General Purpose Register 2 - AES Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesc_ca2](cau_aesc_ca2) module"]
pub type CAU_AESC_CA2 = crate::Reg<u32, _CAU_AESC_CA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESC_CA2;
#[doc = "`write(|w| ..)` method takes [cau_aesc_ca2::W](cau_aesc_ca2::W) writer structure"]
impl crate::Writable for CAU_AESC_CA2 {}
#[doc = "General Purpose Register 2 - AES Column Operation command"]
pub mod cau_aesc_ca2;
#[doc = "General Purpose Register 3 - AES Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesc_ca3](cau_aesc_ca3) module"]
pub type CAU_AESC_CA3 = crate::Reg<u32, _CAU_AESC_CA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESC_CA3;
#[doc = "`write(|w| ..)` method takes [cau_aesc_ca3::W](cau_aesc_ca3::W) writer structure"]
impl crate::Writable for CAU_AESC_CA3 {}
#[doc = "General Purpose Register 3 - AES Column Operation command"]
pub mod cau_aesc_ca3;
#[doc = "General Purpose Register 4 - AES Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesc_ca4](cau_aesc_ca4) module"]
pub type CAU_AESC_CA4 = crate::Reg<u32, _CAU_AESC_CA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESC_CA4;
#[doc = "`write(|w| ..)` method takes [cau_aesc_ca4::W](cau_aesc_ca4::W) writer structure"]
impl crate::Writable for CAU_AESC_CA4 {}
#[doc = "General Purpose Register 4 - AES Column Operation command"]
pub mod cau_aesc_ca4;
#[doc = "General Purpose Register 5 - AES Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesc_ca5](cau_aesc_ca5) module"]
pub type CAU_AESC_CA5 = crate::Reg<u32, _CAU_AESC_CA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESC_CA5;
#[doc = "`write(|w| ..)` method takes [cau_aesc_ca5::W](cau_aesc_ca5::W) writer structure"]
impl crate::Writable for CAU_AESC_CA5 {}
#[doc = "General Purpose Register 5 - AES Column Operation command"]
pub mod cau_aesc_ca5;
#[doc = "General Purpose Register 6 - AES Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesc_ca6](cau_aesc_ca6) module"]
pub type CAU_AESC_CA6 = crate::Reg<u32, _CAU_AESC_CA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESC_CA6;
#[doc = "`write(|w| ..)` method takes [cau_aesc_ca6::W](cau_aesc_ca6::W) writer structure"]
impl crate::Writable for CAU_AESC_CA6 {}
#[doc = "General Purpose Register 6 - AES Column Operation command"]
pub mod cau_aesc_ca6;
#[doc = "General Purpose Register 7 - AES Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesc_ca7](cau_aesc_ca7) module"]
pub type CAU_AESC_CA7 = crate::Reg<u32, _CAU_AESC_CA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESC_CA7;
#[doc = "`write(|w| ..)` method takes [cau_aesc_ca7::W](cau_aesc_ca7::W) writer structure"]
impl crate::Writable for CAU_AESC_CA7 {}
#[doc = "General Purpose Register 7 - AES Column Operation command"]
pub mod cau_aesc_ca7;
#[doc = "General Purpose Register 8 - AES Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesc_ca8](cau_aesc_ca8) module"]
pub type CAU_AESC_CA8 = crate::Reg<u32, _CAU_AESC_CA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESC_CA8;
#[doc = "`write(|w| ..)` method takes [cau_aesc_ca8::W](cau_aesc_ca8::W) writer structure"]
impl crate::Writable for CAU_AESC_CA8 {}
#[doc = "General Purpose Register 8 - AES Column Operation command"]
pub mod cau_aesc_ca8;
#[doc = "Status register - AES Inverse Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesic_casr](cau_aesic_casr) module"]
pub type CAU_AESIC_CASR = crate::Reg<u32, _CAU_AESIC_CASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESIC_CASR;
#[doc = "`write(|w| ..)` method takes [cau_aesic_casr::W](cau_aesic_casr::W) writer structure"]
impl crate::Writable for CAU_AESIC_CASR {}
#[doc = "Status register - AES Inverse Column Operation command"]
pub mod cau_aesic_casr;
#[doc = "Accumulator register - AES Inverse Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesic_caa](cau_aesic_caa) module"]
pub type CAU_AESIC_CAA = crate::Reg<u32, _CAU_AESIC_CAA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESIC_CAA;
#[doc = "`write(|w| ..)` method takes [cau_aesic_caa::W](cau_aesic_caa::W) writer structure"]
impl crate::Writable for CAU_AESIC_CAA {}
#[doc = "Accumulator register - AES Inverse Column Operation command"]
pub mod cau_aesic_caa;
#[doc = "General Purpose Register 0 - AES Inverse Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesic_ca0](cau_aesic_ca0) module"]
pub type CAU_AESIC_CA0 = crate::Reg<u32, _CAU_AESIC_CA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESIC_CA0;
#[doc = "`write(|w| ..)` method takes [cau_aesic_ca0::W](cau_aesic_ca0::W) writer structure"]
impl crate::Writable for CAU_AESIC_CA0 {}
#[doc = "General Purpose Register 0 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca0;
#[doc = "General Purpose Register 1 - AES Inverse Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesic_ca1](cau_aesic_ca1) module"]
pub type CAU_AESIC_CA1 = crate::Reg<u32, _CAU_AESIC_CA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESIC_CA1;
#[doc = "`write(|w| ..)` method takes [cau_aesic_ca1::W](cau_aesic_ca1::W) writer structure"]
impl crate::Writable for CAU_AESIC_CA1 {}
#[doc = "General Purpose Register 1 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca1;
#[doc = "General Purpose Register 2 - AES Inverse Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesic_ca2](cau_aesic_ca2) module"]
pub type CAU_AESIC_CA2 = crate::Reg<u32, _CAU_AESIC_CA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESIC_CA2;
#[doc = "`write(|w| ..)` method takes [cau_aesic_ca2::W](cau_aesic_ca2::W) writer structure"]
impl crate::Writable for CAU_AESIC_CA2 {}
#[doc = "General Purpose Register 2 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca2;
#[doc = "General Purpose Register 3 - AES Inverse Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesic_ca3](cau_aesic_ca3) module"]
pub type CAU_AESIC_CA3 = crate::Reg<u32, _CAU_AESIC_CA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESIC_CA3;
#[doc = "`write(|w| ..)` method takes [cau_aesic_ca3::W](cau_aesic_ca3::W) writer structure"]
impl crate::Writable for CAU_AESIC_CA3 {}
#[doc = "General Purpose Register 3 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca3;
#[doc = "General Purpose Register 4 - AES Inverse Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesic_ca4](cau_aesic_ca4) module"]
pub type CAU_AESIC_CA4 = crate::Reg<u32, _CAU_AESIC_CA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESIC_CA4;
#[doc = "`write(|w| ..)` method takes [cau_aesic_ca4::W](cau_aesic_ca4::W) writer structure"]
impl crate::Writable for CAU_AESIC_CA4 {}
#[doc = "General Purpose Register 4 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca4;
#[doc = "General Purpose Register 5 - AES Inverse Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesic_ca5](cau_aesic_ca5) module"]
pub type CAU_AESIC_CA5 = crate::Reg<u32, _CAU_AESIC_CA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESIC_CA5;
#[doc = "`write(|w| ..)` method takes [cau_aesic_ca5::W](cau_aesic_ca5::W) writer structure"]
impl crate::Writable for CAU_AESIC_CA5 {}
#[doc = "General Purpose Register 5 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca5;
#[doc = "General Purpose Register 6 - AES Inverse Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesic_ca6](cau_aesic_ca6) module"]
pub type CAU_AESIC_CA6 = crate::Reg<u32, _CAU_AESIC_CA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESIC_CA6;
#[doc = "`write(|w| ..)` method takes [cau_aesic_ca6::W](cau_aesic_ca6::W) writer structure"]
impl crate::Writable for CAU_AESIC_CA6 {}
#[doc = "General Purpose Register 6 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca6;
#[doc = "General Purpose Register 7 - AES Inverse Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesic_ca7](cau_aesic_ca7) module"]
pub type CAU_AESIC_CA7 = crate::Reg<u32, _CAU_AESIC_CA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESIC_CA7;
#[doc = "`write(|w| ..)` method takes [cau_aesic_ca7::W](cau_aesic_ca7::W) writer structure"]
impl crate::Writable for CAU_AESIC_CA7 {}
#[doc = "General Purpose Register 7 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca7;
#[doc = "General Purpose Register 8 - AES Inverse Column Operation command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cau_aesic_ca8](cau_aesic_ca8) module"]
pub type CAU_AESIC_CA8 = crate::Reg<u32, _CAU_AESIC_CA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAU_AESIC_CA8;
#[doc = "`write(|w| ..)` method takes [cau_aesic_ca8::W](cau_aesic_ca8::W) writer structure"]
impl crate::Writable for CAU_AESIC_CA8 {}
#[doc = "General Purpose Register 8 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca8;
