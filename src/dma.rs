#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Error Status Register"]
    pub es: ES,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: ERQ,
    _reserved3: [u8; 4usize],
    #[doc = "0x14 - Enable Error Interrupt Register"]
    pub eei: EEI,
    #[doc = "0x18 - Clear Enable Error Interrupt Register"]
    pub ceei: CEEI,
    #[doc = "0x19 - Set Enable Error Interrupt Register"]
    pub seei: SEEI,
    #[doc = "0x1a - Clear Enable Request Register"]
    pub cerq: CERQ,
    #[doc = "0x1b - Set Enable Request Register"]
    pub serq: SERQ,
    #[doc = "0x1c - Clear DONE Status Bit Register"]
    pub cdne: CDNE,
    #[doc = "0x1d - Set START Bit Register"]
    pub ssrt: SSRT,
    #[doc = "0x1e - Clear Error Register"]
    pub cerr: CERR,
    #[doc = "0x1f - Clear Interrupt Request Register"]
    pub cint: CINT,
    _reserved12: [u8; 4usize],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: INT,
    _reserved13: [u8; 4usize],
    #[doc = "0x2c - Error Register"]
    pub err: ERR,
    _reserved14: [u8; 4usize],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: HRS,
    _reserved15: [u8; 12usize],
    #[doc = "0x44 - Enable Asynchronous Request in Stop Register"]
    pub ears: EARS,
    _reserved16: [u8; 184usize],
    #[doc = "0x100 - Channel n Priority Register"]
    pub dchpri3: DCHPRI,
    #[doc = "0x101 - Channel n Priority Register"]
    pub dchpri2: DCHPRI,
    #[doc = "0x102 - Channel n Priority Register"]
    pub dchpri1: DCHPRI,
    #[doc = "0x103 - Channel n Priority Register"]
    pub dchpri0: DCHPRI,
    #[doc = "0x104 - Channel n Priority Register"]
    pub dchpri7: DCHPRI,
    #[doc = "0x105 - Channel n Priority Register"]
    pub dchpri6: DCHPRI,
    #[doc = "0x106 - Channel n Priority Register"]
    pub dchpri5: DCHPRI,
    #[doc = "0x107 - Channel n Priority Register"]
    pub dchpri4: DCHPRI,
    #[doc = "0x108 - Channel n Priority Register"]
    pub dchpri11: DCHPRI,
    #[doc = "0x109 - Channel n Priority Register"]
    pub dchpri10: DCHPRI,
    #[doc = "0x10a - Channel n Priority Register"]
    pub dchpri9: DCHPRI,
    #[doc = "0x10b - Channel n Priority Register"]
    pub dchpri8: DCHPRI,
    #[doc = "0x10c - Channel n Priority Register"]
    pub dchpri15: DCHPRI,
    #[doc = "0x10d - Channel n Priority Register"]
    pub dchpri14: DCHPRI,
    #[doc = "0x10e - Channel n Priority Register"]
    pub dchpri13: DCHPRI,
    #[doc = "0x10f - Channel n Priority Register"]
    pub dchpri12: DCHPRI,
    #[doc = "0x110 - Channel n Priority Register"]
    pub dchpri19: DCHPRI,
    #[doc = "0x111 - Channel n Priority Register"]
    pub dchpri18: DCHPRI,
    #[doc = "0x112 - Channel n Priority Register"]
    pub dchpri17: DCHPRI,
    #[doc = "0x113 - Channel n Priority Register"]
    pub dchpri16: DCHPRI,
    #[doc = "0x114 - Channel n Priority Register"]
    pub dchpri23: DCHPRI,
    #[doc = "0x115 - Channel n Priority Register"]
    pub dchpri22: DCHPRI,
    #[doc = "0x116 - Channel n Priority Register"]
    pub dchpri21: DCHPRI,
    #[doc = "0x117 - Channel n Priority Register"]
    pub dchpri20: DCHPRI,
    #[doc = "0x118 - Channel n Priority Register"]
    pub dchpri27: DCHPRI,
    #[doc = "0x119 - Channel n Priority Register"]
    pub dchpri26: DCHPRI,
    #[doc = "0x11a - Channel n Priority Register"]
    pub dchpri25: DCHPRI,
    #[doc = "0x11b - Channel n Priority Register"]
    pub dchpri24: DCHPRI,
    #[doc = "0x11c - Channel n Priority Register"]
    pub dchpri31: DCHPRI,
    #[doc = "0x11d - Channel n Priority Register"]
    pub dchpri30: DCHPRI,
    #[doc = "0x11e - Channel n Priority Register"]
    pub dchpri29: DCHPRI,
    #[doc = "0x11f - Channel n Priority Register"]
    pub dchpri28: DCHPRI,
    _reserved48: [u8; 3808usize],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: TCD_SADDR,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: TCD_SOFF,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: TCD_ATTR,
    _reserved_51_tcd0_nbytes: [u8; 4usize],
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: TCD_SLAST,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: TCD_DADDR,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: TCD_DOFF,
    _reserved_55_tcd0_citer: [u8; 2usize],
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: TCD_DLASTSGA,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: TCD_CSR,
    _reserved_58_tcd0_biter: [u8; 2usize],
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: TCD_SADDR,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: TCD_SOFF,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: TCD_ATTR,
    _reserved_62_tcd1_nbytes: [u8; 4usize],
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: TCD_SLAST,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: TCD_DADDR,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: TCD_DOFF,
    _reserved_66_tcd1_citer: [u8; 2usize],
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: TCD_DLASTSGA,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: TCD_CSR,
    _reserved_69_tcd1_biter: [u8; 2usize],
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: TCD_SADDR,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: TCD_SOFF,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: TCD_ATTR,
    _reserved_73_tcd2_nbytes: [u8; 4usize],
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: TCD_SLAST,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: TCD_DADDR,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: TCD_DOFF,
    _reserved_77_tcd2_citer: [u8; 2usize],
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: TCD_DLASTSGA,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: TCD_CSR,
    _reserved_80_tcd2_biter: [u8; 2usize],
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: TCD_SADDR,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: TCD_SOFF,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: TCD_ATTR,
    _reserved_84_tcd3_nbytes: [u8; 4usize],
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: TCD_SLAST,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: TCD_DADDR,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: TCD_DOFF,
    _reserved_88_tcd3_citer: [u8; 2usize],
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: TCD_DLASTSGA,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: TCD_CSR,
    _reserved_91_tcd3_biter: [u8; 2usize],
    #[doc = "0x1080 - TCD Source Address"]
    pub tcd4_saddr: TCD_SADDR,
    #[doc = "0x1084 - TCD Signed Source Address Offset"]
    pub tcd4_soff: TCD_SOFF,
    #[doc = "0x1086 - TCD Transfer Attributes"]
    pub tcd4_attr: TCD_ATTR,
    _reserved_95_tcd4_nbytes: [u8; 4usize],
    #[doc = "0x108c - TCD Last Source Address Adjustment"]
    pub tcd4_slast: TCD_SLAST,
    #[doc = "0x1090 - TCD Destination Address"]
    pub tcd4_daddr: TCD_DADDR,
    #[doc = "0x1094 - TCD Signed Destination Address Offset"]
    pub tcd4_doff: TCD_DOFF,
    _reserved_99_tcd4_citer: [u8; 2usize],
    #[doc = "0x1098 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd4_dlastsga: TCD_DLASTSGA,
    #[doc = "0x109c - TCD Control and Status"]
    pub tcd4_csr: TCD_CSR,
    _reserved_102_tcd4_biter: [u8; 2usize],
    #[doc = "0x10a0 - TCD Source Address"]
    pub tcd5_saddr: TCD_SADDR,
    #[doc = "0x10a4 - TCD Signed Source Address Offset"]
    pub tcd5_soff: TCD_SOFF,
    #[doc = "0x10a6 - TCD Transfer Attributes"]
    pub tcd5_attr: TCD_ATTR,
    _reserved_106_tcd5_nbytes: [u8; 4usize],
    #[doc = "0x10ac - TCD Last Source Address Adjustment"]
    pub tcd5_slast: TCD_SLAST,
    #[doc = "0x10b0 - TCD Destination Address"]
    pub tcd5_daddr: TCD_DADDR,
    #[doc = "0x10b4 - TCD Signed Destination Address Offset"]
    pub tcd5_doff: TCD_DOFF,
    _reserved_110_tcd5_citer: [u8; 2usize],
    #[doc = "0x10b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd5_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10bc - TCD Control and Status"]
    pub tcd5_csr: TCD_CSR,
    _reserved_113_tcd5_biter: [u8; 2usize],
    #[doc = "0x10c0 - TCD Source Address"]
    pub tcd6_saddr: TCD_SADDR,
    #[doc = "0x10c4 - TCD Signed Source Address Offset"]
    pub tcd6_soff: TCD_SOFF,
    #[doc = "0x10c6 - TCD Transfer Attributes"]
    pub tcd6_attr: TCD_ATTR,
    _reserved_117_tcd6_nbytes: [u8; 4usize],
    #[doc = "0x10cc - TCD Last Source Address Adjustment"]
    pub tcd6_slast: TCD_SLAST,
    #[doc = "0x10d0 - TCD Destination Address"]
    pub tcd6_daddr: TCD_DADDR,
    #[doc = "0x10d4 - TCD Signed Destination Address Offset"]
    pub tcd6_doff: TCD_DOFF,
    _reserved_121_tcd6_citer: [u8; 2usize],
    #[doc = "0x10d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd6_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10dc - TCD Control and Status"]
    pub tcd6_csr: TCD_CSR,
    _reserved_124_tcd6_biter: [u8; 2usize],
    #[doc = "0x10e0 - TCD Source Address"]
    pub tcd7_saddr: TCD_SADDR,
    #[doc = "0x10e4 - TCD Signed Source Address Offset"]
    pub tcd7_soff: TCD_SOFF,
    #[doc = "0x10e6 - TCD Transfer Attributes"]
    pub tcd7_attr: TCD_ATTR,
    _reserved_128_tcd7_nbytes: [u8; 4usize],
    #[doc = "0x10ec - TCD Last Source Address Adjustment"]
    pub tcd7_slast: TCD_SLAST,
    #[doc = "0x10f0 - TCD Destination Address"]
    pub tcd7_daddr: TCD_DADDR,
    #[doc = "0x10f4 - TCD Signed Destination Address Offset"]
    pub tcd7_doff: TCD_DOFF,
    _reserved_132_tcd7_citer: [u8; 2usize],
    #[doc = "0x10f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd7_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10fc - TCD Control and Status"]
    pub tcd7_csr: TCD_CSR,
    _reserved_135_tcd7_biter: [u8; 2usize],
    #[doc = "0x1100 - TCD Source Address"]
    pub tcd8_saddr: TCD_SADDR,
    #[doc = "0x1104 - TCD Signed Source Address Offset"]
    pub tcd8_soff: TCD_SOFF,
    #[doc = "0x1106 - TCD Transfer Attributes"]
    pub tcd8_attr: TCD_ATTR,
    _reserved_139_tcd8_nbytes: [u8; 4usize],
    #[doc = "0x110c - TCD Last Source Address Adjustment"]
    pub tcd8_slast: TCD_SLAST,
    #[doc = "0x1110 - TCD Destination Address"]
    pub tcd8_daddr: TCD_DADDR,
    #[doc = "0x1114 - TCD Signed Destination Address Offset"]
    pub tcd8_doff: TCD_DOFF,
    _reserved_143_tcd8_citer: [u8; 2usize],
    #[doc = "0x1118 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd8_dlastsga: TCD_DLASTSGA,
    #[doc = "0x111c - TCD Control and Status"]
    pub tcd8_csr: TCD_CSR,
    _reserved_146_tcd8_biter: [u8; 2usize],
    #[doc = "0x1120 - TCD Source Address"]
    pub tcd9_saddr: TCD_SADDR,
    #[doc = "0x1124 - TCD Signed Source Address Offset"]
    pub tcd9_soff: TCD_SOFF,
    #[doc = "0x1126 - TCD Transfer Attributes"]
    pub tcd9_attr: TCD_ATTR,
    _reserved_150_tcd9_nbytes: [u8; 4usize],
    #[doc = "0x112c - TCD Last Source Address Adjustment"]
    pub tcd9_slast: TCD_SLAST,
    #[doc = "0x1130 - TCD Destination Address"]
    pub tcd9_daddr: TCD_DADDR,
    #[doc = "0x1134 - TCD Signed Destination Address Offset"]
    pub tcd9_doff: TCD_DOFF,
    _reserved_154_tcd9_citer: [u8; 2usize],
    #[doc = "0x1138 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd9_dlastsga: TCD_DLASTSGA,
    #[doc = "0x113c - TCD Control and Status"]
    pub tcd9_csr: TCD_CSR,
    _reserved_157_tcd9_biter: [u8; 2usize],
    #[doc = "0x1140 - TCD Source Address"]
    pub tcd10_saddr: TCD_SADDR,
    #[doc = "0x1144 - TCD Signed Source Address Offset"]
    pub tcd10_soff: TCD_SOFF,
    #[doc = "0x1146 - TCD Transfer Attributes"]
    pub tcd10_attr: TCD_ATTR,
    _reserved_161_tcd10_nbytes: [u8; 4usize],
    #[doc = "0x114c - TCD Last Source Address Adjustment"]
    pub tcd10_slast: TCD_SLAST,
    #[doc = "0x1150 - TCD Destination Address"]
    pub tcd10_daddr: TCD_DADDR,
    #[doc = "0x1154 - TCD Signed Destination Address Offset"]
    pub tcd10_doff: TCD_DOFF,
    _reserved_165_tcd10_citer: [u8; 2usize],
    #[doc = "0x1158 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd10_dlastsga: TCD_DLASTSGA,
    #[doc = "0x115c - TCD Control and Status"]
    pub tcd10_csr: TCD_CSR,
    _reserved_168_tcd10_biter: [u8; 2usize],
    #[doc = "0x1160 - TCD Source Address"]
    pub tcd11_saddr: TCD_SADDR,
    #[doc = "0x1164 - TCD Signed Source Address Offset"]
    pub tcd11_soff: TCD_SOFF,
    #[doc = "0x1166 - TCD Transfer Attributes"]
    pub tcd11_attr: TCD_ATTR,
    _reserved_172_tcd11_nbytes: [u8; 4usize],
    #[doc = "0x116c - TCD Last Source Address Adjustment"]
    pub tcd11_slast: TCD_SLAST,
    #[doc = "0x1170 - TCD Destination Address"]
    pub tcd11_daddr: TCD_DADDR,
    #[doc = "0x1174 - TCD Signed Destination Address Offset"]
    pub tcd11_doff: TCD_DOFF,
    _reserved_176_tcd11_citer: [u8; 2usize],
    #[doc = "0x1178 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd11_dlastsga: TCD_DLASTSGA,
    #[doc = "0x117c - TCD Control and Status"]
    pub tcd11_csr: TCD_CSR,
    _reserved_179_tcd11_biter: [u8; 2usize],
    #[doc = "0x1180 - TCD Source Address"]
    pub tcd12_saddr: TCD_SADDR,
    #[doc = "0x1184 - TCD Signed Source Address Offset"]
    pub tcd12_soff: TCD_SOFF,
    #[doc = "0x1186 - TCD Transfer Attributes"]
    pub tcd12_attr: TCD_ATTR,
    _reserved_183_tcd12_nbytes: [u8; 4usize],
    #[doc = "0x118c - TCD Last Source Address Adjustment"]
    pub tcd12_slast: TCD_SLAST,
    #[doc = "0x1190 - TCD Destination Address"]
    pub tcd12_daddr: TCD_DADDR,
    #[doc = "0x1194 - TCD Signed Destination Address Offset"]
    pub tcd12_doff: TCD_DOFF,
    _reserved_187_tcd12_citer: [u8; 2usize],
    #[doc = "0x1198 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd12_dlastsga: TCD_DLASTSGA,
    #[doc = "0x119c - TCD Control and Status"]
    pub tcd12_csr: TCD_CSR,
    _reserved_190_tcd12_biter: [u8; 2usize],
    #[doc = "0x11a0 - TCD Source Address"]
    pub tcd13_saddr: TCD_SADDR,
    #[doc = "0x11a4 - TCD Signed Source Address Offset"]
    pub tcd13_soff: TCD_SOFF,
    #[doc = "0x11a6 - TCD Transfer Attributes"]
    pub tcd13_attr: TCD_ATTR,
    _reserved_194_tcd13_nbytes: [u8; 4usize],
    #[doc = "0x11ac - TCD Last Source Address Adjustment"]
    pub tcd13_slast: TCD_SLAST,
    #[doc = "0x11b0 - TCD Destination Address"]
    pub tcd13_daddr: TCD_DADDR,
    #[doc = "0x11b4 - TCD Signed Destination Address Offset"]
    pub tcd13_doff: TCD_DOFF,
    _reserved_198_tcd13_citer: [u8; 2usize],
    #[doc = "0x11b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd13_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11bc - TCD Control and Status"]
    pub tcd13_csr: TCD_CSR,
    _reserved_201_tcd13_biter: [u8; 2usize],
    #[doc = "0x11c0 - TCD Source Address"]
    pub tcd14_saddr: TCD_SADDR,
    #[doc = "0x11c4 - TCD Signed Source Address Offset"]
    pub tcd14_soff: TCD_SOFF,
    #[doc = "0x11c6 - TCD Transfer Attributes"]
    pub tcd14_attr: TCD_ATTR,
    _reserved_205_tcd14_nbytes: [u8; 4usize],
    #[doc = "0x11cc - TCD Last Source Address Adjustment"]
    pub tcd14_slast: TCD_SLAST,
    #[doc = "0x11d0 - TCD Destination Address"]
    pub tcd14_daddr: TCD_DADDR,
    #[doc = "0x11d4 - TCD Signed Destination Address Offset"]
    pub tcd14_doff: TCD_DOFF,
    _reserved_209_tcd14_citer: [u8; 2usize],
    #[doc = "0x11d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd14_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11dc - TCD Control and Status"]
    pub tcd14_csr: TCD_CSR,
    _reserved_212_tcd14_biter: [u8; 2usize],
    #[doc = "0x11e0 - TCD Source Address"]
    pub tcd15_saddr: TCD_SADDR,
    #[doc = "0x11e4 - TCD Signed Source Address Offset"]
    pub tcd15_soff: TCD_SOFF,
    #[doc = "0x11e6 - TCD Transfer Attributes"]
    pub tcd15_attr: TCD_ATTR,
    _reserved_216_tcd15_nbytes: [u8; 4usize],
    #[doc = "0x11ec - TCD Last Source Address Adjustment"]
    pub tcd15_slast: TCD_SLAST,
    #[doc = "0x11f0 - TCD Destination Address"]
    pub tcd15_daddr: TCD_DADDR,
    #[doc = "0x11f4 - TCD Signed Destination Address Offset"]
    pub tcd15_doff: TCD_DOFF,
    _reserved_220_tcd15_citer: [u8; 2usize],
    #[doc = "0x11f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd15_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11fc - TCD Control and Status"]
    pub tcd15_csr: TCD_CSR,
    _reserved_223_tcd15_biter: [u8; 2usize],
    #[doc = "0x1200 - TCD Source Address"]
    pub tcd16_saddr: TCD_SADDR,
    #[doc = "0x1204 - TCD Signed Source Address Offset"]
    pub tcd16_soff: TCD_SOFF,
    #[doc = "0x1206 - TCD Transfer Attributes"]
    pub tcd16_attr: TCD_ATTR,
    _reserved_227_tcd16_nbytes: [u8; 4usize],
    #[doc = "0x120c - TCD Last Source Address Adjustment"]
    pub tcd16_slast: TCD_SLAST,
    #[doc = "0x1210 - TCD Destination Address"]
    pub tcd16_daddr: TCD_DADDR,
    #[doc = "0x1214 - TCD Signed Destination Address Offset"]
    pub tcd16_doff: TCD_DOFF,
    _reserved_231_tcd16_citer: [u8; 2usize],
    #[doc = "0x1218 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd16_dlastsga: TCD_DLASTSGA,
    #[doc = "0x121c - TCD Control and Status"]
    pub tcd16_csr: TCD_CSR,
    _reserved_234_tcd16_biter: [u8; 2usize],
    #[doc = "0x1220 - TCD Source Address"]
    pub tcd17_saddr: TCD_SADDR,
    #[doc = "0x1224 - TCD Signed Source Address Offset"]
    pub tcd17_soff: TCD_SOFF,
    #[doc = "0x1226 - TCD Transfer Attributes"]
    pub tcd17_attr: TCD_ATTR,
    _reserved_238_tcd17_nbytes: [u8; 4usize],
    #[doc = "0x122c - TCD Last Source Address Adjustment"]
    pub tcd17_slast: TCD_SLAST,
    #[doc = "0x1230 - TCD Destination Address"]
    pub tcd17_daddr: TCD_DADDR,
    #[doc = "0x1234 - TCD Signed Destination Address Offset"]
    pub tcd17_doff: TCD_DOFF,
    _reserved_242_tcd17_citer: [u8; 2usize],
    #[doc = "0x1238 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd17_dlastsga: TCD_DLASTSGA,
    #[doc = "0x123c - TCD Control and Status"]
    pub tcd17_csr: TCD_CSR,
    _reserved_245_tcd17_biter: [u8; 2usize],
    #[doc = "0x1240 - TCD Source Address"]
    pub tcd18_saddr: TCD_SADDR,
    #[doc = "0x1244 - TCD Signed Source Address Offset"]
    pub tcd18_soff: TCD_SOFF,
    #[doc = "0x1246 - TCD Transfer Attributes"]
    pub tcd18_attr: TCD_ATTR,
    _reserved_249_tcd18_nbytes: [u8; 4usize],
    #[doc = "0x124c - TCD Last Source Address Adjustment"]
    pub tcd18_slast: TCD_SLAST,
    #[doc = "0x1250 - TCD Destination Address"]
    pub tcd18_daddr: TCD_DADDR,
    #[doc = "0x1254 - TCD Signed Destination Address Offset"]
    pub tcd18_doff: TCD_DOFF,
    _reserved_253_tcd18_citer: [u8; 2usize],
    #[doc = "0x1258 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd18_dlastsga: TCD_DLASTSGA,
    #[doc = "0x125c - TCD Control and Status"]
    pub tcd18_csr: TCD_CSR,
    _reserved_256_tcd18_biter: [u8; 2usize],
    #[doc = "0x1260 - TCD Source Address"]
    pub tcd19_saddr: TCD_SADDR,
    #[doc = "0x1264 - TCD Signed Source Address Offset"]
    pub tcd19_soff: TCD_SOFF,
    #[doc = "0x1266 - TCD Transfer Attributes"]
    pub tcd19_attr: TCD_ATTR,
    _reserved_260_tcd19_nbytes: [u8; 4usize],
    #[doc = "0x126c - TCD Last Source Address Adjustment"]
    pub tcd19_slast: TCD_SLAST,
    #[doc = "0x1270 - TCD Destination Address"]
    pub tcd19_daddr: TCD_DADDR,
    #[doc = "0x1274 - TCD Signed Destination Address Offset"]
    pub tcd19_doff: TCD_DOFF,
    _reserved_264_tcd19_citer: [u8; 2usize],
    #[doc = "0x1278 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd19_dlastsga: TCD_DLASTSGA,
    #[doc = "0x127c - TCD Control and Status"]
    pub tcd19_csr: TCD_CSR,
    _reserved_267_tcd19_biter: [u8; 2usize],
    #[doc = "0x1280 - TCD Source Address"]
    pub tcd20_saddr: TCD_SADDR,
    #[doc = "0x1284 - TCD Signed Source Address Offset"]
    pub tcd20_soff: TCD_SOFF,
    #[doc = "0x1286 - TCD Transfer Attributes"]
    pub tcd20_attr: TCD_ATTR,
    _reserved_271_tcd20_nbytes: [u8; 4usize],
    #[doc = "0x128c - TCD Last Source Address Adjustment"]
    pub tcd20_slast: TCD_SLAST,
    #[doc = "0x1290 - TCD Destination Address"]
    pub tcd20_daddr: TCD_DADDR,
    #[doc = "0x1294 - TCD Signed Destination Address Offset"]
    pub tcd20_doff: TCD_DOFF,
    _reserved_275_tcd20_citer: [u8; 2usize],
    #[doc = "0x1298 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd20_dlastsga: TCD_DLASTSGA,
    #[doc = "0x129c - TCD Control and Status"]
    pub tcd20_csr: TCD_CSR,
    _reserved_278_tcd20_biter: [u8; 2usize],
    #[doc = "0x12a0 - TCD Source Address"]
    pub tcd21_saddr: TCD_SADDR,
    #[doc = "0x12a4 - TCD Signed Source Address Offset"]
    pub tcd21_soff: TCD_SOFF,
    #[doc = "0x12a6 - TCD Transfer Attributes"]
    pub tcd21_attr: TCD_ATTR,
    _reserved_282_tcd21_nbytes: [u8; 4usize],
    #[doc = "0x12ac - TCD Last Source Address Adjustment"]
    pub tcd21_slast: TCD_SLAST,
    #[doc = "0x12b0 - TCD Destination Address"]
    pub tcd21_daddr: TCD_DADDR,
    #[doc = "0x12b4 - TCD Signed Destination Address Offset"]
    pub tcd21_doff: TCD_DOFF,
    _reserved_286_tcd21_citer: [u8; 2usize],
    #[doc = "0x12b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd21_dlastsga: TCD_DLASTSGA,
    #[doc = "0x12bc - TCD Control and Status"]
    pub tcd21_csr: TCD_CSR,
    _reserved_289_tcd21_biter: [u8; 2usize],
    #[doc = "0x12c0 - TCD Source Address"]
    pub tcd22_saddr: TCD_SADDR,
    #[doc = "0x12c4 - TCD Signed Source Address Offset"]
    pub tcd22_soff: TCD_SOFF,
    #[doc = "0x12c6 - TCD Transfer Attributes"]
    pub tcd22_attr: TCD_ATTR,
    _reserved_293_tcd22_nbytes: [u8; 4usize],
    #[doc = "0x12cc - TCD Last Source Address Adjustment"]
    pub tcd22_slast: TCD_SLAST,
    #[doc = "0x12d0 - TCD Destination Address"]
    pub tcd22_daddr: TCD_DADDR,
    #[doc = "0x12d4 - TCD Signed Destination Address Offset"]
    pub tcd22_doff: TCD_DOFF,
    _reserved_297_tcd22_citer: [u8; 2usize],
    #[doc = "0x12d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd22_dlastsga: TCD_DLASTSGA,
    #[doc = "0x12dc - TCD Control and Status"]
    pub tcd22_csr: TCD_CSR,
    _reserved_300_tcd22_biter: [u8; 2usize],
    #[doc = "0x12e0 - TCD Source Address"]
    pub tcd23_saddr: TCD_SADDR,
    #[doc = "0x12e4 - TCD Signed Source Address Offset"]
    pub tcd23_soff: TCD_SOFF,
    #[doc = "0x12e6 - TCD Transfer Attributes"]
    pub tcd23_attr: TCD_ATTR,
    _reserved_304_tcd23_nbytes: [u8; 4usize],
    #[doc = "0x12ec - TCD Last Source Address Adjustment"]
    pub tcd23_slast: TCD_SLAST,
    #[doc = "0x12f0 - TCD Destination Address"]
    pub tcd23_daddr: TCD_DADDR,
    #[doc = "0x12f4 - TCD Signed Destination Address Offset"]
    pub tcd23_doff: TCD_DOFF,
    _reserved_308_tcd23_citer: [u8; 2usize],
    #[doc = "0x12f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd23_dlastsga: TCD_DLASTSGA,
    #[doc = "0x12fc - TCD Control and Status"]
    pub tcd23_csr: TCD_CSR,
    _reserved_311_tcd23_biter: [u8; 2usize],
    #[doc = "0x1300 - TCD Source Address"]
    pub tcd24_saddr: TCD_SADDR,
    #[doc = "0x1304 - TCD Signed Source Address Offset"]
    pub tcd24_soff: TCD_SOFF,
    #[doc = "0x1306 - TCD Transfer Attributes"]
    pub tcd24_attr: TCD_ATTR,
    _reserved_315_tcd24_nbytes: [u8; 4usize],
    #[doc = "0x130c - TCD Last Source Address Adjustment"]
    pub tcd24_slast: TCD_SLAST,
    #[doc = "0x1310 - TCD Destination Address"]
    pub tcd24_daddr: TCD_DADDR,
    #[doc = "0x1314 - TCD Signed Destination Address Offset"]
    pub tcd24_doff: TCD_DOFF,
    _reserved_319_tcd24_citer: [u8; 2usize],
    #[doc = "0x1318 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd24_dlastsga: TCD_DLASTSGA,
    #[doc = "0x131c - TCD Control and Status"]
    pub tcd24_csr: TCD_CSR,
    _reserved_322_tcd24_biter: [u8; 2usize],
    #[doc = "0x1320 - TCD Source Address"]
    pub tcd25_saddr: TCD_SADDR,
    #[doc = "0x1324 - TCD Signed Source Address Offset"]
    pub tcd25_soff: TCD_SOFF,
    #[doc = "0x1326 - TCD Transfer Attributes"]
    pub tcd25_attr: TCD_ATTR,
    _reserved_326_tcd25_nbytes: [u8; 4usize],
    #[doc = "0x132c - TCD Last Source Address Adjustment"]
    pub tcd25_slast: TCD_SLAST,
    #[doc = "0x1330 - TCD Destination Address"]
    pub tcd25_daddr: TCD_DADDR,
    #[doc = "0x1334 - TCD Signed Destination Address Offset"]
    pub tcd25_doff: TCD_DOFF,
    _reserved_330_tcd25_citer: [u8; 2usize],
    #[doc = "0x1338 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd25_dlastsga: TCD_DLASTSGA,
    #[doc = "0x133c - TCD Control and Status"]
    pub tcd25_csr: TCD_CSR,
    _reserved_333_tcd25_biter: [u8; 2usize],
    #[doc = "0x1340 - TCD Source Address"]
    pub tcd26_saddr: TCD_SADDR,
    #[doc = "0x1344 - TCD Signed Source Address Offset"]
    pub tcd26_soff: TCD_SOFF,
    #[doc = "0x1346 - TCD Transfer Attributes"]
    pub tcd26_attr: TCD_ATTR,
    _reserved_337_tcd26_nbytes: [u8; 4usize],
    #[doc = "0x134c - TCD Last Source Address Adjustment"]
    pub tcd26_slast: TCD_SLAST,
    #[doc = "0x1350 - TCD Destination Address"]
    pub tcd26_daddr: TCD_DADDR,
    #[doc = "0x1354 - TCD Signed Destination Address Offset"]
    pub tcd26_doff: TCD_DOFF,
    _reserved_341_tcd26_citer: [u8; 2usize],
    #[doc = "0x1358 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd26_dlastsga: TCD_DLASTSGA,
    #[doc = "0x135c - TCD Control and Status"]
    pub tcd26_csr: TCD_CSR,
    _reserved_344_tcd26_biter: [u8; 2usize],
    #[doc = "0x1360 - TCD Source Address"]
    pub tcd27_saddr: TCD_SADDR,
    #[doc = "0x1364 - TCD Signed Source Address Offset"]
    pub tcd27_soff: TCD_SOFF,
    #[doc = "0x1366 - TCD Transfer Attributes"]
    pub tcd27_attr: TCD_ATTR,
    _reserved_348_tcd27_nbytes: [u8; 4usize],
    #[doc = "0x136c - TCD Last Source Address Adjustment"]
    pub tcd27_slast: TCD_SLAST,
    #[doc = "0x1370 - TCD Destination Address"]
    pub tcd27_daddr: TCD_DADDR,
    #[doc = "0x1374 - TCD Signed Destination Address Offset"]
    pub tcd27_doff: TCD_DOFF,
    _reserved_352_tcd27_citer: [u8; 2usize],
    #[doc = "0x1378 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd27_dlastsga: TCD_DLASTSGA,
    #[doc = "0x137c - TCD Control and Status"]
    pub tcd27_csr: TCD_CSR,
    _reserved_355_tcd27_biter: [u8; 2usize],
    #[doc = "0x1380 - TCD Source Address"]
    pub tcd28_saddr: TCD_SADDR,
    #[doc = "0x1384 - TCD Signed Source Address Offset"]
    pub tcd28_soff: TCD_SOFF,
    #[doc = "0x1386 - TCD Transfer Attributes"]
    pub tcd28_attr: TCD_ATTR,
    _reserved_359_tcd28_nbytes: [u8; 4usize],
    #[doc = "0x138c - TCD Last Source Address Adjustment"]
    pub tcd28_slast: TCD_SLAST,
    #[doc = "0x1390 - TCD Destination Address"]
    pub tcd28_daddr: TCD_DADDR,
    #[doc = "0x1394 - TCD Signed Destination Address Offset"]
    pub tcd28_doff: TCD_DOFF,
    _reserved_363_tcd28_citer: [u8; 2usize],
    #[doc = "0x1398 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd28_dlastsga: TCD_DLASTSGA,
    #[doc = "0x139c - TCD Control and Status"]
    pub tcd28_csr: TCD_CSR,
    _reserved_366_tcd28_biter: [u8; 2usize],
    #[doc = "0x13a0 - TCD Source Address"]
    pub tcd29_saddr: TCD_SADDR,
    #[doc = "0x13a4 - TCD Signed Source Address Offset"]
    pub tcd29_soff: TCD_SOFF,
    #[doc = "0x13a6 - TCD Transfer Attributes"]
    pub tcd29_attr: TCD_ATTR,
    _reserved_370_tcd29_nbytes: [u8; 4usize],
    #[doc = "0x13ac - TCD Last Source Address Adjustment"]
    pub tcd29_slast: TCD_SLAST,
    #[doc = "0x13b0 - TCD Destination Address"]
    pub tcd29_daddr: TCD_DADDR,
    #[doc = "0x13b4 - TCD Signed Destination Address Offset"]
    pub tcd29_doff: TCD_DOFF,
    _reserved_374_tcd29_citer: [u8; 2usize],
    #[doc = "0x13b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd29_dlastsga: TCD_DLASTSGA,
    #[doc = "0x13bc - TCD Control and Status"]
    pub tcd29_csr: TCD_CSR,
    _reserved_377_tcd29_biter: [u8; 2usize],
    #[doc = "0x13c0 - TCD Source Address"]
    pub tcd30_saddr: TCD_SADDR,
    #[doc = "0x13c4 - TCD Signed Source Address Offset"]
    pub tcd30_soff: TCD_SOFF,
    #[doc = "0x13c6 - TCD Transfer Attributes"]
    pub tcd30_attr: TCD_ATTR,
    _reserved_381_tcd30_nbytes: [u8; 4usize],
    #[doc = "0x13cc - TCD Last Source Address Adjustment"]
    pub tcd30_slast: TCD_SLAST,
    #[doc = "0x13d0 - TCD Destination Address"]
    pub tcd30_daddr: TCD_DADDR,
    #[doc = "0x13d4 - TCD Signed Destination Address Offset"]
    pub tcd30_doff: TCD_DOFF,
    _reserved_385_tcd30_citer: [u8; 2usize],
    #[doc = "0x13d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd30_dlastsga: TCD_DLASTSGA,
    #[doc = "0x13dc - TCD Control and Status"]
    pub tcd30_csr: TCD_CSR,
    _reserved_388_tcd30_biter: [u8; 2usize],
    #[doc = "0x13e0 - TCD Source Address"]
    pub tcd31_saddr: TCD_SADDR,
    #[doc = "0x13e4 - TCD Signed Source Address Offset"]
    pub tcd31_soff: TCD_SOFF,
    #[doc = "0x13e6 - TCD Transfer Attributes"]
    pub tcd31_attr: TCD_ATTR,
    _reserved_392_tcd31_nbytes: [u8; 4usize],
    #[doc = "0x13ec - TCD Last Source Address Adjustment"]
    pub tcd31_slast: TCD_SLAST,
    #[doc = "0x13f0 - TCD Destination Address"]
    pub tcd31_daddr: TCD_DADDR,
    #[doc = "0x13f4 - TCD Signed Destination Address Offset"]
    pub tcd31_doff: TCD_DOFF,
    _reserved_396_tcd31_citer: [u8; 2usize],
    #[doc = "0x13f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd31_dlastsga: TCD_DLASTSGA,
    #[doc = "0x13fc - TCD Control and Status"]
    pub tcd31_csr: TCD_CSR,
    _reserved_399_tcd31_biter: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4104usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4104usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4104usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4104usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4104usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4104usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4118usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4118usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4118usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4118usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4126usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4126usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4126usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4126usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4136usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4136usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4136usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4136usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4136usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4136usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4150usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4150usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4150usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4150usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4158usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4158usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4158usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4158usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4168usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4168usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4168usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4168usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4168usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4168usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4182usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4182usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4182usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4182usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4190usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4190usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4190usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4190usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4200usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4200usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4200usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4200usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4200usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4200usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4214usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4214usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4214usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4214usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4222usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4222usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4222usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4222usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4232usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4232usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4232usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4232usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4232usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4232usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4246usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4246usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4246usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4246usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4254usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4254usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4254usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4254usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4264usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4264usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4264usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4264usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4264usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4264usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4278usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4278usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4278usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4278usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4286usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4286usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4286usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4286usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4296usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4296usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4296usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4296usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4296usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4296usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4310usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4310usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4310usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4310usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4318usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4318usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4318usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4318usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4328usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4328usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4328usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4328usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4328usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4328usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4342usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4342usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4342usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4342usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4350usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4350usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4350usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4350usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4360usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4360usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4360usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4360usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1108 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4360usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1108 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4360usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd8_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4374usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd8_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4374usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd8_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4374usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd8_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4374usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd8_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4382usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd8_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4382usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd8_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4382usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd8_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4382usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4392usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4392usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4392usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4392usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1128 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4392usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1128 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4392usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd9_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4406usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd9_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4406usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd9_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4406usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd9_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4406usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd9_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4414usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd9_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4414usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd9_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4414usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd9_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4414usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4424usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4424usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4424usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4424usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1148 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4424usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1148 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4424usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd10_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4438usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd10_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4438usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd10_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4438usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd10_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4438usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd10_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4446usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd10_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4446usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd10_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4446usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd10_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4446usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4456usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4456usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4456usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4456usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1168 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4456usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1168 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4456usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd11_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4470usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd11_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4470usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd11_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4470usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd11_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4470usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd11_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4478usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd11_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4478usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd11_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4478usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd11_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4478usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4488usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4488usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4488usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4488usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1188 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4488usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1188 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4488usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd12_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4502usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd12_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4502usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd12_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4502usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd12_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4502usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd12_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4510usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd12_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4510usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd12_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4510usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd12_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4510usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4520usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4520usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4520usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4520usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x11a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4520usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x11a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4520usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd13_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4534usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd13_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4534usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd13_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4534usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd13_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4534usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd13_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4542usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd13_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4542usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd13_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4542usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd13_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4542usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4552usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4552usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4552usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4552usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x11c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4552usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x11c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4552usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd14_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4566usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd14_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4566usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd14_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4566usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd14_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4566usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd14_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4574usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd14_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4574usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd14_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4574usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd14_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4574usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4584usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4584usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4584usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4584usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x11e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4584usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x11e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4584usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd15_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4598usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd15_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4598usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd15_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4598usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd15_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4598usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd15_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4606usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd15_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4606usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd15_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4606usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd15_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4606usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1208 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd16_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4616usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1208 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd16_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4616usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1208 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd16_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4616usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1208 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd16_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4616usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1208 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd16_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4616usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1208 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd16_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4616usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1216 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd16_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4630usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1216 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd16_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4630usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1216 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd16_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4630usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1216 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd16_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4630usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x121e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd16_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4638usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x121e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd16_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4638usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x121e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd16_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4638usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x121e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd16_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4638usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1228 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd17_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4648usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1228 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd17_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4648usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1228 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd17_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4648usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1228 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd17_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4648usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1228 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd17_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4648usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1228 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd17_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4648usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1236 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd17_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4662usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1236 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd17_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4662usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1236 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd17_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4662usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1236 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd17_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4662usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x123e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd17_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4670usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x123e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd17_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4670usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x123e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd17_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4670usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x123e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd17_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4670usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1248 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd18_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4680usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1248 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd18_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4680usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1248 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd18_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4680usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1248 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd18_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4680usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1248 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd18_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4680usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1248 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd18_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4680usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1256 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd18_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4694usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1256 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd18_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4694usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1256 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd18_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4694usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1256 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd18_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4694usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x125e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd18_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4702usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x125e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd18_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4702usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x125e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd18_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4702usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x125e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd18_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4702usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1268 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd19_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4712usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1268 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd19_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4712usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1268 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd19_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4712usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1268 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd19_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4712usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1268 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd19_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4712usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1268 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd19_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4712usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1276 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd19_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4726usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1276 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd19_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4726usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1276 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd19_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4726usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1276 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd19_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4726usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x127e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd19_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4734usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x127e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd19_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4734usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x127e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd19_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4734usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x127e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd19_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4734usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1288 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd20_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4744usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1288 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd20_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4744usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1288 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd20_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4744usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1288 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd20_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4744usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1288 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd20_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4744usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1288 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd20_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4744usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1296 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd20_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4758usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1296 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd20_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4758usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1296 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd20_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4758usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1296 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd20_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4758usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x129e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd20_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4766usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x129e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd20_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4766usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x129e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd20_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4766usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x129e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd20_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4766usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x12a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd21_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4776usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x12a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd21_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4776usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x12a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd21_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4776usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x12a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd21_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4776usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x12a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd21_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4776usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x12a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd21_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4776usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x12b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd21_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4790usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x12b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd21_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4790usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x12b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd21_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4790usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x12b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd21_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4790usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x12be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd21_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4798usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x12be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd21_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4798usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x12be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd21_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4798usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x12be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd21_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4798usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x12c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd22_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4808usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x12c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd22_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4808usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x12c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd22_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4808usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x12c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd22_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4808usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x12c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd22_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4808usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x12c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd22_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4808usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x12d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd22_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4822usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x12d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd22_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4822usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x12d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd22_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4822usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x12d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd22_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4822usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x12de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd22_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4830usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x12de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd22_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4830usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x12de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd22_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4830usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x12de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd22_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4830usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x12e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd23_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4840usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x12e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd23_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4840usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x12e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd23_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4840usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x12e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd23_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4840usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x12e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd23_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4840usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x12e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd23_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4840usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x12f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd23_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4854usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x12f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd23_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4854usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x12f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd23_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4854usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x12f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd23_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4854usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x12fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd23_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4862usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x12fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd23_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4862usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x12fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd23_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4862usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x12fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd23_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4862usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1308 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd24_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4872usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1308 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd24_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4872usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1308 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd24_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4872usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1308 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd24_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4872usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1308 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd24_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4872usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1308 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd24_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4872usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1316 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd24_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4886usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1316 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd24_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4886usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1316 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd24_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4886usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1316 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd24_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4886usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x131e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd24_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4894usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x131e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd24_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4894usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x131e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd24_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4894usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x131e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd24_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4894usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1328 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd25_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4904usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1328 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd25_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4904usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1328 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd25_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4904usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1328 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd25_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4904usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1328 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd25_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4904usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1328 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd25_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4904usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1336 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd25_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4918usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1336 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd25_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4918usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1336 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd25_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4918usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1336 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd25_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4918usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x133e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd25_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4926usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x133e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd25_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4926usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x133e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd25_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4926usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x133e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd25_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4926usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1348 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd26_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4936usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1348 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd26_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4936usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1348 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd26_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4936usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1348 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd26_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4936usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1348 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd26_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4936usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1348 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd26_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4936usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1356 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd26_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4950usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1356 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd26_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4950usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1356 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd26_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4950usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1356 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd26_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4950usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x135e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd26_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4958usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x135e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd26_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4958usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x135e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd26_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4958usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x135e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd26_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4958usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1368 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd27_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4968usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1368 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd27_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4968usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1368 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd27_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4968usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1368 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd27_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4968usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1368 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd27_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4968usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1368 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd27_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4968usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1376 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd27_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4982usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1376 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd27_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4982usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1376 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd27_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4982usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1376 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd27_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4982usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x137e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd27_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4990usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x137e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd27_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4990usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x137e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd27_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4990usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x137e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd27_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4990usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1388 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd28_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5000usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1388 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd28_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5000usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1388 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd28_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5000usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1388 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd28_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5000usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1388 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd28_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(5000usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1388 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd28_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(5000usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1396 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd28_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5014usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1396 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd28_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5014usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1396 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd28_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5014usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1396 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd28_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5014usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x139e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd28_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5022usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x139e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd28_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5022usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x139e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd28_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5022usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x139e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd28_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5022usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x13a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd29_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5032usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x13a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd29_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5032usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x13a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd29_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5032usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x13a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd29_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5032usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x13a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd29_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(5032usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x13a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd29_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(5032usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x13b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd29_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5046usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x13b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd29_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5046usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x13b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd29_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5046usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x13b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd29_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5046usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x13be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd29_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5054usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x13be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd29_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5054usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x13be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd29_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5054usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x13be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd29_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5054usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x13c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd30_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5064usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x13c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd30_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5064usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x13c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd30_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5064usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x13c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd30_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5064usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x13c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd30_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(5064usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x13c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd30_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(5064usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x13d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd30_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5078usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x13d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd30_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5078usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x13d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd30_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5078usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x13d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd30_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5078usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x13de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd30_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5086usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x13de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd30_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5086usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x13de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd30_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5086usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x13de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd30_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5086usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x13e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd31_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5096usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x13e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd31_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5096usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x13e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd31_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5096usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x13e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd31_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5096usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x13e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd31_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(5096usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x13e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd31_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(5096usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x13f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd31_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5110usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x13f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd31_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5110usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x13f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd31_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5110usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x13f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd31_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5110usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x13fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd31_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5118usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x13fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd31_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5118usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x13fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd31_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(5118usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x13fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd31_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(5118usize) as *mut TCD_BITER_ELINKNO)
        }
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [es](es) module"]
pub type ES = crate::Reg<u32, _ES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ES;
#[doc = "`read()` method returns [es::R](es::R) reader structure"]
impl crate::Readable for ES {}
#[doc = "Error Status Register"]
pub mod es;
#[doc = "Enable Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [erq](erq) module"]
pub type ERQ = crate::Reg<u32, _ERQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERQ;
#[doc = "`read()` method returns [erq::R](erq::R) reader structure"]
impl crate::Readable for ERQ {}
#[doc = "`write(|w| ..)` method takes [erq::W](erq::W) writer structure"]
impl crate::Writable for ERQ {}
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "Enable Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eei](eei) module"]
pub type EEI = crate::Reg<u32, _EEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEI;
#[doc = "`read()` method returns [eei::R](eei::R) reader structure"]
impl crate::Readable for EEI {}
#[doc = "`write(|w| ..)` method takes [eei::W](eei::W) writer structure"]
impl crate::Writable for EEI {}
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "Clear Enable Error Interrupt Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ceei](ceei) module"]
pub type CEEI = crate::Reg<u8, _CEEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEEI;
#[doc = "`write(|w| ..)` method takes [ceei::W](ceei::W) writer structure"]
impl crate::Writable for CEEI {}
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "Set Enable Error Interrupt Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seei](seei) module"]
pub type SEEI = crate::Reg<u8, _SEEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEEI;
#[doc = "`write(|w| ..)` method takes [seei::W](seei::W) writer structure"]
impl crate::Writable for SEEI {}
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "Clear Enable Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cerq](cerq) module"]
pub type CERQ = crate::Reg<u8, _CERQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CERQ;
#[doc = "`write(|w| ..)` method takes [cerq::W](cerq::W) writer structure"]
impl crate::Writable for CERQ {}
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "Set Enable Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [serq](serq) module"]
pub type SERQ = crate::Reg<u8, _SERQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SERQ;
#[doc = "`write(|w| ..)` method takes [serq::W](serq::W) writer structure"]
impl crate::Writable for SERQ {}
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "Clear DONE Status Bit Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cdne](cdne) module"]
pub type CDNE = crate::Reg<u8, _CDNE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDNE;
#[doc = "`write(|w| ..)` method takes [cdne::W](cdne::W) writer structure"]
impl crate::Writable for CDNE {}
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "Set START Bit Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssrt](ssrt) module"]
pub type SSRT = crate::Reg<u8, _SSRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSRT;
#[doc = "`write(|w| ..)` method takes [ssrt::W](ssrt::W) writer structure"]
impl crate::Writable for SSRT {}
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "Clear Error Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cerr](cerr) module"]
pub type CERR = crate::Reg<u8, _CERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CERR;
#[doc = "`write(|w| ..)` method takes [cerr::W](cerr::W) writer structure"]
impl crate::Writable for CERR {}
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "Clear Interrupt Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cint](cint) module"]
pub type CINT = crate::Reg<u8, _CINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CINT;
#[doc = "`write(|w| ..)` method takes [cint::W](cint::W) writer structure"]
impl crate::Writable for CINT {}
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "Interrupt Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int](int) module"]
pub type INT = crate::Reg<u32, _INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT;
#[doc = "`read()` method returns [int::R](int::R) reader structure"]
impl crate::Readable for INT {}
#[doc = "`write(|w| ..)` method takes [int::W](int::W) writer structure"]
impl crate::Writable for INT {}
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [err](err) module"]
pub type ERR = crate::Reg<u32, _ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERR;
#[doc = "`read()` method returns [err::R](err::R) reader structure"]
impl crate::Readable for ERR {}
#[doc = "`write(|w| ..)` method takes [err::W](err::W) writer structure"]
impl crate::Writable for ERR {}
#[doc = "Error Register"]
pub mod err;
#[doc = "Hardware Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hrs](hrs) module"]
pub type HRS = crate::Reg<u32, _HRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRS;
#[doc = "`read()` method returns [hrs::R](hrs::R) reader structure"]
impl crate::Readable for HRS {}
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "Enable Asynchronous Request in Stop Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ears](ears) module"]
pub type EARS = crate::Reg<u32, _EARS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EARS;
#[doc = "`read()` method returns [ears::R](ears::R) reader structure"]
impl crate::Readable for EARS {}
#[doc = "`write(|w| ..)` method takes [ears::W](ears::W) writer structure"]
impl crate::Writable for EARS {}
#[doc = "Enable Asynchronous Request in Stop Register"]
pub mod ears;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dchpri](dchpri) module"]
pub type DCHPRI = crate::Reg<u8, _DCHPRI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI;
#[doc = "`read()` method returns [dchpri::R](dchpri::R) reader structure"]
impl crate::Readable for DCHPRI {}
#[doc = "`write(|w| ..)` method takes [dchpri::W](dchpri::W) writer structure"]
impl crate::Writable for DCHPRI {}
#[doc = "Channel n Priority Register"]
pub mod dchpri;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_saddr](tcd_saddr) module"]
pub type TCD_SADDR = crate::Reg<u32, _TCD_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_SADDR;
#[doc = "`read()` method returns [tcd_saddr::R](tcd_saddr::R) reader structure"]
impl crate::Readable for TCD_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd_saddr::W](tcd_saddr::W) writer structure"]
impl crate::Writable for TCD_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_soff](tcd_soff) module"]
pub type TCD_SOFF = crate::Reg<u16, _TCD_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_SOFF;
#[doc = "`read()` method returns [tcd_soff::R](tcd_soff::R) reader structure"]
impl crate::Readable for TCD_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd_soff::W](tcd_soff::W) writer structure"]
impl crate::Writable for TCD_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_attr](tcd_attr) module"]
pub type TCD_ATTR = crate::Reg<u16, _TCD_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_ATTR;
#[doc = "`read()` method returns [tcd_attr::R](tcd_attr::R) reader structure"]
impl crate::Readable for TCD_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd_attr::W](tcd_attr::W) writer structure"]
impl crate::Writable for TCD_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_nbytes_mlno](tcd_nbytes_mlno) module"]
pub type TCD_NBYTES_MLNO = crate::Reg<u32, _TCD_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd_nbytes_mlno::R](tcd_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd_nbytes_mlno::W](tcd_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_nbytes_mloffno](tcd_nbytes_mloffno) module"]
pub type TCD_NBYTES_MLOFFNO = crate::Reg<u32, _TCD_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd_nbytes_mloffno::R](tcd_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd_nbytes_mloffno::W](tcd_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_nbytes_mloffyes](tcd_nbytes_mloffyes) module"]
pub type TCD_NBYTES_MLOFFYES = crate::Reg<u32, _TCD_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd_nbytes_mloffyes::R](tcd_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd_nbytes_mloffyes::W](tcd_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_slast](tcd_slast) module"]
pub type TCD_SLAST = crate::Reg<u32, _TCD_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_SLAST;
#[doc = "`read()` method returns [tcd_slast::R](tcd_slast::R) reader structure"]
impl crate::Readable for TCD_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd_slast::W](tcd_slast::W) writer structure"]
impl crate::Writable for TCD_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_daddr](tcd_daddr) module"]
pub type TCD_DADDR = crate::Reg<u32, _TCD_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_DADDR;
#[doc = "`read()` method returns [tcd_daddr::R](tcd_daddr::R) reader structure"]
impl crate::Readable for TCD_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd_daddr::W](tcd_daddr::W) writer structure"]
impl crate::Writable for TCD_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_doff](tcd_doff) module"]
pub type TCD_DOFF = crate::Reg<u16, _TCD_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_DOFF;
#[doc = "`read()` method returns [tcd_doff::R](tcd_doff::R) reader structure"]
impl crate::Readable for TCD_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd_doff::W](tcd_doff::W) writer structure"]
impl crate::Writable for TCD_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_citer_elinkno](tcd_citer_elinkno) module"]
pub type TCD_CITER_ELINKNO = crate::Reg<u16, _TCD_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd_citer_elinkno::R](tcd_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd_citer_elinkno::W](tcd_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_citer_elinkyes](tcd_citer_elinkyes) module"]
pub type TCD_CITER_ELINKYES = crate::Reg<u16, _TCD_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd_citer_elinkyes::R](tcd_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd_citer_elinkyes::W](tcd_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_dlastsga](tcd_dlastsga) module"]
pub type TCD_DLASTSGA = crate::Reg<u32, _TCD_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_DLASTSGA;
#[doc = "`read()` method returns [tcd_dlastsga::R](tcd_dlastsga::R) reader structure"]
impl crate::Readable for TCD_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd_dlastsga::W](tcd_dlastsga::W) writer structure"]
impl crate::Writable for TCD_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_csr](tcd_csr) module"]
pub type TCD_CSR = crate::Reg<u16, _TCD_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_CSR;
#[doc = "`read()` method returns [tcd_csr::R](tcd_csr::R) reader structure"]
impl crate::Readable for TCD_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd_csr::W](tcd_csr::W) writer structure"]
impl crate::Writable for TCD_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_biter_elinkno](tcd_biter_elinkno) module"]
pub type TCD_BITER_ELINKNO = crate::Reg<u16, _TCD_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd_biter_elinkno::R](tcd_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd_biter_elinkno::W](tcd_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcd_biter_elinkyes](tcd_biter_elinkyes) module"]
pub type TCD_BITER_ELINKYES = crate::Reg<u16, _TCD_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd_biter_elinkyes::R](tcd_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd_biter_elinkyes::W](tcd_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elinkyes;
