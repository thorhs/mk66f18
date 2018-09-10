#[doc = r" Register block"]
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
    _reserved0: [u8; 2048usize],
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
    _reserved1: [u8; 20usize],
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
    _reserved2: [u8; 20usize],
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
    _reserved3: [u8; 20usize],
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
    _reserved4: [u8; 84usize],
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
    _reserved5: [u8; 20usize],
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
    _reserved6: [u8; 276usize],
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
    _reserved7: [u8; 20usize],
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
#[doc = "Direct access register 0"]
pub struct CAU_DIRECT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 0"]
pub mod cau_direct0;
#[doc = "Direct access register 1"]
pub struct CAU_DIRECT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 1"]
pub mod cau_direct1;
#[doc = "Direct access register 2"]
pub struct CAU_DIRECT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 2"]
pub mod cau_direct2;
#[doc = "Direct access register 3"]
pub struct CAU_DIRECT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 3"]
pub mod cau_direct3;
#[doc = "Direct access register 4"]
pub struct CAU_DIRECT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 4"]
pub mod cau_direct4;
#[doc = "Direct access register 5"]
pub struct CAU_DIRECT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 5"]
pub mod cau_direct5;
#[doc = "Direct access register 6"]
pub struct CAU_DIRECT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 6"]
pub mod cau_direct6;
#[doc = "Direct access register 7"]
pub struct CAU_DIRECT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 7"]
pub mod cau_direct7;
#[doc = "Direct access register 8"]
pub struct CAU_DIRECT8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 8"]
pub mod cau_direct8;
#[doc = "Direct access register 9"]
pub struct CAU_DIRECT9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 9"]
pub mod cau_direct9;
#[doc = "Direct access register 10"]
pub struct CAU_DIRECT10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 10"]
pub mod cau_direct10;
#[doc = "Direct access register 11"]
pub struct CAU_DIRECT11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 11"]
pub mod cau_direct11;
#[doc = "Direct access register 12"]
pub struct CAU_DIRECT12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 12"]
pub mod cau_direct12;
#[doc = "Direct access register 13"]
pub struct CAU_DIRECT13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 13"]
pub mod cau_direct13;
#[doc = "Direct access register 14"]
pub struct CAU_DIRECT14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 14"]
pub mod cau_direct14;
#[doc = "Direct access register 15"]
pub struct CAU_DIRECT15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direct access register 15"]
pub mod cau_direct15;
#[doc = "Status register - Load Register command"]
pub struct CAU_LDR_CASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register - Load Register command"]
pub mod cau_ldr_casr;
#[doc = "Accumulator register - Load Register command"]
pub struct CAU_LDR_CAA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator register - Load Register command"]
pub mod cau_ldr_caa;
#[doc = "General Purpose Register 0 - Load Register command"]
pub struct CAU_LDR_CA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 0 - Load Register command"]
pub mod cau_ldr_ca0;
#[doc = "General Purpose Register 1 - Load Register command"]
pub struct CAU_LDR_CA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 1 - Load Register command"]
pub mod cau_ldr_ca1;
#[doc = "General Purpose Register 2 - Load Register command"]
pub struct CAU_LDR_CA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 2 - Load Register command"]
pub mod cau_ldr_ca2;
#[doc = "General Purpose Register 3 - Load Register command"]
pub struct CAU_LDR_CA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 3 - Load Register command"]
pub mod cau_ldr_ca3;
#[doc = "General Purpose Register 4 - Load Register command"]
pub struct CAU_LDR_CA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 4 - Load Register command"]
pub mod cau_ldr_ca4;
#[doc = "General Purpose Register 5 - Load Register command"]
pub struct CAU_LDR_CA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 5 - Load Register command"]
pub mod cau_ldr_ca5;
#[doc = "General Purpose Register 6 - Load Register command"]
pub struct CAU_LDR_CA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 6 - Load Register command"]
pub mod cau_ldr_ca6;
#[doc = "General Purpose Register 7 - Load Register command"]
pub struct CAU_LDR_CA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 7 - Load Register command"]
pub mod cau_ldr_ca7;
#[doc = "General Purpose Register 8 - Load Register command"]
pub struct CAU_LDR_CA8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 8 - Load Register command"]
pub mod cau_ldr_ca8;
#[doc = "Status register - Store Register command"]
pub struct CAU_STR_CASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register - Store Register command"]
pub mod cau_str_casr;
#[doc = "Accumulator register - Store Register command"]
pub struct CAU_STR_CAA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator register - Store Register command"]
pub mod cau_str_caa;
#[doc = "General Purpose Register 0 - Store Register command"]
pub struct CAU_STR_CA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 0 - Store Register command"]
pub mod cau_str_ca0;
#[doc = "General Purpose Register 1 - Store Register command"]
pub struct CAU_STR_CA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 1 - Store Register command"]
pub mod cau_str_ca1;
#[doc = "General Purpose Register 2 - Store Register command"]
pub struct CAU_STR_CA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 2 - Store Register command"]
pub mod cau_str_ca2;
#[doc = "General Purpose Register 3 - Store Register command"]
pub struct CAU_STR_CA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 3 - Store Register command"]
pub mod cau_str_ca3;
#[doc = "General Purpose Register 4 - Store Register command"]
pub struct CAU_STR_CA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 4 - Store Register command"]
pub mod cau_str_ca4;
#[doc = "General Purpose Register 5 - Store Register command"]
pub struct CAU_STR_CA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 5 - Store Register command"]
pub mod cau_str_ca5;
#[doc = "General Purpose Register 6 - Store Register command"]
pub struct CAU_STR_CA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 6 - Store Register command"]
pub mod cau_str_ca6;
#[doc = "General Purpose Register 7 - Store Register command"]
pub struct CAU_STR_CA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 7 - Store Register command"]
pub mod cau_str_ca7;
#[doc = "General Purpose Register 8 - Store Register command"]
pub struct CAU_STR_CA8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 8 - Store Register command"]
pub mod cau_str_ca8;
#[doc = "Status register - Add Register command"]
pub struct CAU_ADR_CASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register - Add Register command"]
pub mod cau_adr_casr;
#[doc = "Accumulator register - Add to register command"]
pub struct CAU_ADR_CAA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator register - Add to register command"]
pub mod cau_adr_caa;
#[doc = "General Purpose Register 0 - Add to register command"]
pub struct CAU_ADR_CA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 0 - Add to register command"]
pub mod cau_adr_ca0;
#[doc = "General Purpose Register 1 - Add to register command"]
pub struct CAU_ADR_CA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 1 - Add to register command"]
pub mod cau_adr_ca1;
#[doc = "General Purpose Register 2 - Add to register command"]
pub struct CAU_ADR_CA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 2 - Add to register command"]
pub mod cau_adr_ca2;
#[doc = "General Purpose Register 3 - Add to register command"]
pub struct CAU_ADR_CA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 3 - Add to register command"]
pub mod cau_adr_ca3;
#[doc = "General Purpose Register 4 - Add to register command"]
pub struct CAU_ADR_CA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 4 - Add to register command"]
pub mod cau_adr_ca4;
#[doc = "General Purpose Register 5 - Add to register command"]
pub struct CAU_ADR_CA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 5 - Add to register command"]
pub mod cau_adr_ca5;
#[doc = "General Purpose Register 6 - Add to register command"]
pub struct CAU_ADR_CA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 6 - Add to register command"]
pub mod cau_adr_ca6;
#[doc = "General Purpose Register 7 - Add to register command"]
pub struct CAU_ADR_CA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 7 - Add to register command"]
pub mod cau_adr_ca7;
#[doc = "General Purpose Register 8 - Add to register command"]
pub struct CAU_ADR_CA8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 8 - Add to register command"]
pub mod cau_adr_ca8;
#[doc = "Status register - Reverse and Add to Register command"]
pub struct CAU_RADR_CASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register - Reverse and Add to Register command"]
pub mod cau_radr_casr;
#[doc = "Accumulator register - Reverse and Add to Register command"]
pub struct CAU_RADR_CAA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator register - Reverse and Add to Register command"]
pub mod cau_radr_caa;
#[doc = "General Purpose Register 0 - Reverse and Add to Register command"]
pub struct CAU_RADR_CA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 0 - Reverse and Add to Register command"]
pub mod cau_radr_ca0;
#[doc = "General Purpose Register 1 - Reverse and Add to Register command"]
pub struct CAU_RADR_CA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 1 - Reverse and Add to Register command"]
pub mod cau_radr_ca1;
#[doc = "General Purpose Register 2 - Reverse and Add to Register command"]
pub struct CAU_RADR_CA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 2 - Reverse and Add to Register command"]
pub mod cau_radr_ca2;
#[doc = "General Purpose Register 3 - Reverse and Add to Register command"]
pub struct CAU_RADR_CA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 3 - Reverse and Add to Register command"]
pub mod cau_radr_ca3;
#[doc = "General Purpose Register 4 - Reverse and Add to Register command"]
pub struct CAU_RADR_CA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 4 - Reverse and Add to Register command"]
pub mod cau_radr_ca4;
#[doc = "General Purpose Register 5 - Reverse and Add to Register command"]
pub struct CAU_RADR_CA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 5 - Reverse and Add to Register command"]
pub mod cau_radr_ca5;
#[doc = "General Purpose Register 6 - Reverse and Add to Register command"]
pub struct CAU_RADR_CA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 6 - Reverse and Add to Register command"]
pub mod cau_radr_ca6;
#[doc = "General Purpose Register 7 - Reverse and Add to Register command"]
pub struct CAU_RADR_CA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 7 - Reverse and Add to Register command"]
pub mod cau_radr_ca7;
#[doc = "General Purpose Register 8 - Reverse and Add to Register command"]
pub struct CAU_RADR_CA8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 8 - Reverse and Add to Register command"]
pub mod cau_radr_ca8;
#[doc = "Status register - Exclusive Or command"]
pub struct CAU_XOR_CASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register - Exclusive Or command"]
pub mod cau_xor_casr;
#[doc = "Accumulator register - Exclusive Or command"]
pub struct CAU_XOR_CAA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator register - Exclusive Or command"]
pub mod cau_xor_caa;
#[doc = "General Purpose Register 0 - Exclusive Or command"]
pub struct CAU_XOR_CA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 0 - Exclusive Or command"]
pub mod cau_xor_ca0;
#[doc = "General Purpose Register 1 - Exclusive Or command"]
pub struct CAU_XOR_CA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 1 - Exclusive Or command"]
pub mod cau_xor_ca1;
#[doc = "General Purpose Register 2 - Exclusive Or command"]
pub struct CAU_XOR_CA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 2 - Exclusive Or command"]
pub mod cau_xor_ca2;
#[doc = "General Purpose Register 3 - Exclusive Or command"]
pub struct CAU_XOR_CA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 3 - Exclusive Or command"]
pub mod cau_xor_ca3;
#[doc = "General Purpose Register 4 - Exclusive Or command"]
pub struct CAU_XOR_CA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 4 - Exclusive Or command"]
pub mod cau_xor_ca4;
#[doc = "General Purpose Register 5 - Exclusive Or command"]
pub struct CAU_XOR_CA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 5 - Exclusive Or command"]
pub mod cau_xor_ca5;
#[doc = "General Purpose Register 6 - Exclusive Or command"]
pub struct CAU_XOR_CA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 6 - Exclusive Or command"]
pub mod cau_xor_ca6;
#[doc = "General Purpose Register 7 - Exclusive Or command"]
pub struct CAU_XOR_CA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 7 - Exclusive Or command"]
pub mod cau_xor_ca7;
#[doc = "General Purpose Register 8 - Exclusive Or command"]
pub struct CAU_XOR_CA8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 8 - Exclusive Or command"]
pub mod cau_xor_ca8;
#[doc = "Status register - Rotate Left command"]
pub struct CAU_ROTL_CASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register - Rotate Left command"]
pub mod cau_rotl_casr;
#[doc = "Accumulator register - Rotate Left command"]
pub struct CAU_ROTL_CAA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator register - Rotate Left command"]
pub mod cau_rotl_caa;
#[doc = "General Purpose Register 0 - Rotate Left command"]
pub struct CAU_ROTL_CA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 0 - Rotate Left command"]
pub mod cau_rotl_ca0;
#[doc = "General Purpose Register 1 - Rotate Left command"]
pub struct CAU_ROTL_CA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 1 - Rotate Left command"]
pub mod cau_rotl_ca1;
#[doc = "General Purpose Register 2 - Rotate Left command"]
pub struct CAU_ROTL_CA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 2 - Rotate Left command"]
pub mod cau_rotl_ca2;
#[doc = "General Purpose Register 3 - Rotate Left command"]
pub struct CAU_ROTL_CA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 3 - Rotate Left command"]
pub mod cau_rotl_ca3;
#[doc = "General Purpose Register 4 - Rotate Left command"]
pub struct CAU_ROTL_CA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 4 - Rotate Left command"]
pub mod cau_rotl_ca4;
#[doc = "General Purpose Register 5 - Rotate Left command"]
pub struct CAU_ROTL_CA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 5 - Rotate Left command"]
pub mod cau_rotl_ca5;
#[doc = "General Purpose Register 6 - Rotate Left command"]
pub struct CAU_ROTL_CA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 6 - Rotate Left command"]
pub mod cau_rotl_ca6;
#[doc = "General Purpose Register 7 - Rotate Left command"]
pub struct CAU_ROTL_CA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 7 - Rotate Left command"]
pub mod cau_rotl_ca7;
#[doc = "General Purpose Register 8 - Rotate Left command"]
pub struct CAU_ROTL_CA8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 8 - Rotate Left command"]
pub mod cau_rotl_ca8;
#[doc = "Status register - AES Column Operation command"]
pub struct CAU_AESC_CASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register - AES Column Operation command"]
pub mod cau_aesc_casr;
#[doc = "Accumulator register - AES Column Operation command"]
pub struct CAU_AESC_CAA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator register - AES Column Operation command"]
pub mod cau_aesc_caa;
#[doc = "General Purpose Register 0 - AES Column Operation command"]
pub struct CAU_AESC_CA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 0 - AES Column Operation command"]
pub mod cau_aesc_ca0;
#[doc = "General Purpose Register 1 - AES Column Operation command"]
pub struct CAU_AESC_CA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 1 - AES Column Operation command"]
pub mod cau_aesc_ca1;
#[doc = "General Purpose Register 2 - AES Column Operation command"]
pub struct CAU_AESC_CA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 2 - AES Column Operation command"]
pub mod cau_aesc_ca2;
#[doc = "General Purpose Register 3 - AES Column Operation command"]
pub struct CAU_AESC_CA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 3 - AES Column Operation command"]
pub mod cau_aesc_ca3;
#[doc = "General Purpose Register 4 - AES Column Operation command"]
pub struct CAU_AESC_CA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 4 - AES Column Operation command"]
pub mod cau_aesc_ca4;
#[doc = "General Purpose Register 5 - AES Column Operation command"]
pub struct CAU_AESC_CA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 5 - AES Column Operation command"]
pub mod cau_aesc_ca5;
#[doc = "General Purpose Register 6 - AES Column Operation command"]
pub struct CAU_AESC_CA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 6 - AES Column Operation command"]
pub mod cau_aesc_ca6;
#[doc = "General Purpose Register 7 - AES Column Operation command"]
pub struct CAU_AESC_CA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 7 - AES Column Operation command"]
pub mod cau_aesc_ca7;
#[doc = "General Purpose Register 8 - AES Column Operation command"]
pub struct CAU_AESC_CA8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 8 - AES Column Operation command"]
pub mod cau_aesc_ca8;
#[doc = "Status register - AES Inverse Column Operation command"]
pub struct CAU_AESIC_CASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register - AES Inverse Column Operation command"]
pub mod cau_aesic_casr;
#[doc = "Accumulator register - AES Inverse Column Operation command"]
pub struct CAU_AESIC_CAA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Accumulator register - AES Inverse Column Operation command"]
pub mod cau_aesic_caa;
#[doc = "General Purpose Register 0 - AES Inverse Column Operation command"]
pub struct CAU_AESIC_CA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 0 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca0;
#[doc = "General Purpose Register 1 - AES Inverse Column Operation command"]
pub struct CAU_AESIC_CA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 1 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca1;
#[doc = "General Purpose Register 2 - AES Inverse Column Operation command"]
pub struct CAU_AESIC_CA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 2 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca2;
#[doc = "General Purpose Register 3 - AES Inverse Column Operation command"]
pub struct CAU_AESIC_CA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 3 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca3;
#[doc = "General Purpose Register 4 - AES Inverse Column Operation command"]
pub struct CAU_AESIC_CA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 4 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca4;
#[doc = "General Purpose Register 5 - AES Inverse Column Operation command"]
pub struct CAU_AESIC_CA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 5 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca5;
#[doc = "General Purpose Register 6 - AES Inverse Column Operation command"]
pub struct CAU_AESIC_CA6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 6 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca6;
#[doc = "General Purpose Register 7 - AES Inverse Column Operation command"]
pub struct CAU_AESIC_CA7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 7 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca7;
#[doc = "General Purpose Register 8 - AES Inverse Column Operation command"]
pub struct CAU_AESIC_CA8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 8 - AES Inverse Column Operation command"]
pub mod cau_aesic_ca8;
