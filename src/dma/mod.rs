#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Error Status Register"]
    pub es: ES,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: ERQ,
    _reserved1: [u8; 4usize],
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
    _reserved2: [u8; 4usize],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: INT,
    _reserved3: [u8; 4usize],
    #[doc = "0x2c - Error Register"]
    pub err: ERR,
    _reserved4: [u8; 4usize],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: HRS,
    _reserved5: [u8; 12usize],
    #[doc = "0x44 - Enable Asynchronous Request in Stop Register"]
    pub ears: EARS,
    _reserved6: [u8; 184usize],
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
    _reserved7: [u8; 3808usize],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: TCD_SADDR,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: TCD_SOFF,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: TCD_ATTR,
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd0_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: TCD_SLAST,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: TCD_DADDR,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: TCD_DOFF,
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd0_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: TCD_DLASTSGA,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: TCD_CSR,
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd0_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: TCD_SADDR,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: TCD_SOFF,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: TCD_ATTR,
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd1_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: TCD_SLAST,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: TCD_DADDR,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: TCD_DOFF,
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd1_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: TCD_DLASTSGA,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: TCD_CSR,
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd1_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: TCD_SADDR,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: TCD_SOFF,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: TCD_ATTR,
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd2_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: TCD_SLAST,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: TCD_DADDR,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: TCD_DOFF,
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd2_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: TCD_DLASTSGA,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: TCD_CSR,
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd2_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: TCD_SADDR,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: TCD_SOFF,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: TCD_ATTR,
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd3_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: TCD_SLAST,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: TCD_DADDR,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: TCD_DOFF,
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd3_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: TCD_DLASTSGA,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: TCD_CSR,
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd3_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1080 - TCD Source Address"]
    pub tcd4_saddr: TCD_SADDR,
    #[doc = "0x1084 - TCD Signed Source Address Offset"]
    pub tcd4_soff: TCD_SOFF,
    #[doc = "0x1086 - TCD Transfer Attributes"]
    pub tcd4_attr: TCD_ATTR,
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd4_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x108c - TCD Last Source Address Adjustment"]
    pub tcd4_slast: TCD_SLAST,
    #[doc = "0x1090 - TCD Destination Address"]
    pub tcd4_daddr: TCD_DADDR,
    #[doc = "0x1094 - TCD Signed Destination Address Offset"]
    pub tcd4_doff: TCD_DOFF,
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd4_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1098 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd4_dlastsga: TCD_DLASTSGA,
    #[doc = "0x109c - TCD Control and Status"]
    pub tcd4_csr: TCD_CSR,
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd4_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x10a0 - TCD Source Address"]
    pub tcd5_saddr: TCD_SADDR,
    #[doc = "0x10a4 - TCD Signed Source Address Offset"]
    pub tcd5_soff: TCD_SOFF,
    #[doc = "0x10a6 - TCD Transfer Attributes"]
    pub tcd5_attr: TCD_ATTR,
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd5_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x10ac - TCD Last Source Address Adjustment"]
    pub tcd5_slast: TCD_SLAST,
    #[doc = "0x10b0 - TCD Destination Address"]
    pub tcd5_daddr: TCD_DADDR,
    #[doc = "0x10b4 - TCD Signed Destination Address Offset"]
    pub tcd5_doff: TCD_DOFF,
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd5_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x10b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd5_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10bc - TCD Control and Status"]
    pub tcd5_csr: TCD_CSR,
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd5_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x10c0 - TCD Source Address"]
    pub tcd6_saddr: TCD_SADDR,
    #[doc = "0x10c4 - TCD Signed Source Address Offset"]
    pub tcd6_soff: TCD_SOFF,
    #[doc = "0x10c6 - TCD Transfer Attributes"]
    pub tcd6_attr: TCD_ATTR,
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd6_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x10cc - TCD Last Source Address Adjustment"]
    pub tcd6_slast: TCD_SLAST,
    #[doc = "0x10d0 - TCD Destination Address"]
    pub tcd6_daddr: TCD_DADDR,
    #[doc = "0x10d4 - TCD Signed Destination Address Offset"]
    pub tcd6_doff: TCD_DOFF,
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd6_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x10d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd6_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10dc - TCD Control and Status"]
    pub tcd6_csr: TCD_CSR,
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd6_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x10e0 - TCD Source Address"]
    pub tcd7_saddr: TCD_SADDR,
    #[doc = "0x10e4 - TCD Signed Source Address Offset"]
    pub tcd7_soff: TCD_SOFF,
    #[doc = "0x10e6 - TCD Transfer Attributes"]
    pub tcd7_attr: TCD_ATTR,
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd7_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x10ec - TCD Last Source Address Adjustment"]
    pub tcd7_slast: TCD_SLAST,
    #[doc = "0x10f0 - TCD Destination Address"]
    pub tcd7_daddr: TCD_DADDR,
    #[doc = "0x10f4 - TCD Signed Destination Address Offset"]
    pub tcd7_doff: TCD_DOFF,
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd7_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x10f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd7_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10fc - TCD Control and Status"]
    pub tcd7_csr: TCD_CSR,
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd7_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1100 - TCD Source Address"]
    pub tcd8_saddr: TCD_SADDR,
    #[doc = "0x1104 - TCD Signed Source Address Offset"]
    pub tcd8_soff: TCD_SOFF,
    #[doc = "0x1106 - TCD Transfer Attributes"]
    pub tcd8_attr: TCD_ATTR,
    #[doc = "0x1108 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd8_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x110c - TCD Last Source Address Adjustment"]
    pub tcd8_slast: TCD_SLAST,
    #[doc = "0x1110 - TCD Destination Address"]
    pub tcd8_daddr: TCD_DADDR,
    #[doc = "0x1114 - TCD Signed Destination Address Offset"]
    pub tcd8_doff: TCD_DOFF,
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd8_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1118 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd8_dlastsga: TCD_DLASTSGA,
    #[doc = "0x111c - TCD Control and Status"]
    pub tcd8_csr: TCD_CSR,
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd8_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1120 - TCD Source Address"]
    pub tcd9_saddr: TCD_SADDR,
    #[doc = "0x1124 - TCD Signed Source Address Offset"]
    pub tcd9_soff: TCD_SOFF,
    #[doc = "0x1126 - TCD Transfer Attributes"]
    pub tcd9_attr: TCD_ATTR,
    #[doc = "0x1128 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd9_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x112c - TCD Last Source Address Adjustment"]
    pub tcd9_slast: TCD_SLAST,
    #[doc = "0x1130 - TCD Destination Address"]
    pub tcd9_daddr: TCD_DADDR,
    #[doc = "0x1134 - TCD Signed Destination Address Offset"]
    pub tcd9_doff: TCD_DOFF,
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd9_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1138 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd9_dlastsga: TCD_DLASTSGA,
    #[doc = "0x113c - TCD Control and Status"]
    pub tcd9_csr: TCD_CSR,
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd9_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1140 - TCD Source Address"]
    pub tcd10_saddr: TCD_SADDR,
    #[doc = "0x1144 - TCD Signed Source Address Offset"]
    pub tcd10_soff: TCD_SOFF,
    #[doc = "0x1146 - TCD Transfer Attributes"]
    pub tcd10_attr: TCD_ATTR,
    #[doc = "0x1148 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd10_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x114c - TCD Last Source Address Adjustment"]
    pub tcd10_slast: TCD_SLAST,
    #[doc = "0x1150 - TCD Destination Address"]
    pub tcd10_daddr: TCD_DADDR,
    #[doc = "0x1154 - TCD Signed Destination Address Offset"]
    pub tcd10_doff: TCD_DOFF,
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd10_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1158 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd10_dlastsga: TCD_DLASTSGA,
    #[doc = "0x115c - TCD Control and Status"]
    pub tcd10_csr: TCD_CSR,
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd10_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1160 - TCD Source Address"]
    pub tcd11_saddr: TCD_SADDR,
    #[doc = "0x1164 - TCD Signed Source Address Offset"]
    pub tcd11_soff: TCD_SOFF,
    #[doc = "0x1166 - TCD Transfer Attributes"]
    pub tcd11_attr: TCD_ATTR,
    #[doc = "0x1168 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd11_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x116c - TCD Last Source Address Adjustment"]
    pub tcd11_slast: TCD_SLAST,
    #[doc = "0x1170 - TCD Destination Address"]
    pub tcd11_daddr: TCD_DADDR,
    #[doc = "0x1174 - TCD Signed Destination Address Offset"]
    pub tcd11_doff: TCD_DOFF,
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd11_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1178 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd11_dlastsga: TCD_DLASTSGA,
    #[doc = "0x117c - TCD Control and Status"]
    pub tcd11_csr: TCD_CSR,
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd11_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1180 - TCD Source Address"]
    pub tcd12_saddr: TCD_SADDR,
    #[doc = "0x1184 - TCD Signed Source Address Offset"]
    pub tcd12_soff: TCD_SOFF,
    #[doc = "0x1186 - TCD Transfer Attributes"]
    pub tcd12_attr: TCD_ATTR,
    #[doc = "0x1188 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd12_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x118c - TCD Last Source Address Adjustment"]
    pub tcd12_slast: TCD_SLAST,
    #[doc = "0x1190 - TCD Destination Address"]
    pub tcd12_daddr: TCD_DADDR,
    #[doc = "0x1194 - TCD Signed Destination Address Offset"]
    pub tcd12_doff: TCD_DOFF,
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd12_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1198 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd12_dlastsga: TCD_DLASTSGA,
    #[doc = "0x119c - TCD Control and Status"]
    pub tcd12_csr: TCD_CSR,
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd12_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x11a0 - TCD Source Address"]
    pub tcd13_saddr: TCD_SADDR,
    #[doc = "0x11a4 - TCD Signed Source Address Offset"]
    pub tcd13_soff: TCD_SOFF,
    #[doc = "0x11a6 - TCD Transfer Attributes"]
    pub tcd13_attr: TCD_ATTR,
    #[doc = "0x11a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd13_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x11ac - TCD Last Source Address Adjustment"]
    pub tcd13_slast: TCD_SLAST,
    #[doc = "0x11b0 - TCD Destination Address"]
    pub tcd13_daddr: TCD_DADDR,
    #[doc = "0x11b4 - TCD Signed Destination Address Offset"]
    pub tcd13_doff: TCD_DOFF,
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd13_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x11b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd13_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11bc - TCD Control and Status"]
    pub tcd13_csr: TCD_CSR,
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd13_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x11c0 - TCD Source Address"]
    pub tcd14_saddr: TCD_SADDR,
    #[doc = "0x11c4 - TCD Signed Source Address Offset"]
    pub tcd14_soff: TCD_SOFF,
    #[doc = "0x11c6 - TCD Transfer Attributes"]
    pub tcd14_attr: TCD_ATTR,
    #[doc = "0x11c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd14_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x11cc - TCD Last Source Address Adjustment"]
    pub tcd14_slast: TCD_SLAST,
    #[doc = "0x11d0 - TCD Destination Address"]
    pub tcd14_daddr: TCD_DADDR,
    #[doc = "0x11d4 - TCD Signed Destination Address Offset"]
    pub tcd14_doff: TCD_DOFF,
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd14_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x11d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd14_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11dc - TCD Control and Status"]
    pub tcd14_csr: TCD_CSR,
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd14_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x11e0 - TCD Source Address"]
    pub tcd15_saddr: TCD_SADDR,
    #[doc = "0x11e4 - TCD Signed Source Address Offset"]
    pub tcd15_soff: TCD_SOFF,
    #[doc = "0x11e6 - TCD Transfer Attributes"]
    pub tcd15_attr: TCD_ATTR,
    #[doc = "0x11e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd15_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x11ec - TCD Last Source Address Adjustment"]
    pub tcd15_slast: TCD_SLAST,
    #[doc = "0x11f0 - TCD Destination Address"]
    pub tcd15_daddr: TCD_DADDR,
    #[doc = "0x11f4 - TCD Signed Destination Address Offset"]
    pub tcd15_doff: TCD_DOFF,
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd15_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x11f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd15_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11fc - TCD Control and Status"]
    pub tcd15_csr: TCD_CSR,
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd15_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1200 - TCD Source Address"]
    pub tcd16_saddr: TCD_SADDR,
    #[doc = "0x1204 - TCD Signed Source Address Offset"]
    pub tcd16_soff: TCD_SOFF,
    #[doc = "0x1206 - TCD Transfer Attributes"]
    pub tcd16_attr: TCD_ATTR,
    #[doc = "0x1208 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd16_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x120c - TCD Last Source Address Adjustment"]
    pub tcd16_slast: TCD_SLAST,
    #[doc = "0x1210 - TCD Destination Address"]
    pub tcd16_daddr: TCD_DADDR,
    #[doc = "0x1214 - TCD Signed Destination Address Offset"]
    pub tcd16_doff: TCD_DOFF,
    #[doc = "0x1216 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd16_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1218 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd16_dlastsga: TCD_DLASTSGA,
    #[doc = "0x121c - TCD Control and Status"]
    pub tcd16_csr: TCD_CSR,
    #[doc = "0x121e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd16_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1220 - TCD Source Address"]
    pub tcd17_saddr: TCD_SADDR,
    #[doc = "0x1224 - TCD Signed Source Address Offset"]
    pub tcd17_soff: TCD_SOFF,
    #[doc = "0x1226 - TCD Transfer Attributes"]
    pub tcd17_attr: TCD_ATTR,
    #[doc = "0x1228 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd17_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x122c - TCD Last Source Address Adjustment"]
    pub tcd17_slast: TCD_SLAST,
    #[doc = "0x1230 - TCD Destination Address"]
    pub tcd17_daddr: TCD_DADDR,
    #[doc = "0x1234 - TCD Signed Destination Address Offset"]
    pub tcd17_doff: TCD_DOFF,
    #[doc = "0x1236 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd17_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1238 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd17_dlastsga: TCD_DLASTSGA,
    #[doc = "0x123c - TCD Control and Status"]
    pub tcd17_csr: TCD_CSR,
    #[doc = "0x123e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd17_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1240 - TCD Source Address"]
    pub tcd18_saddr: TCD_SADDR,
    #[doc = "0x1244 - TCD Signed Source Address Offset"]
    pub tcd18_soff: TCD_SOFF,
    #[doc = "0x1246 - TCD Transfer Attributes"]
    pub tcd18_attr: TCD_ATTR,
    #[doc = "0x1248 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd18_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x124c - TCD Last Source Address Adjustment"]
    pub tcd18_slast: TCD_SLAST,
    #[doc = "0x1250 - TCD Destination Address"]
    pub tcd18_daddr: TCD_DADDR,
    #[doc = "0x1254 - TCD Signed Destination Address Offset"]
    pub tcd18_doff: TCD_DOFF,
    #[doc = "0x1256 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd18_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1258 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd18_dlastsga: TCD_DLASTSGA,
    #[doc = "0x125c - TCD Control and Status"]
    pub tcd18_csr: TCD_CSR,
    #[doc = "0x125e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd18_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1260 - TCD Source Address"]
    pub tcd19_saddr: TCD_SADDR,
    #[doc = "0x1264 - TCD Signed Source Address Offset"]
    pub tcd19_soff: TCD_SOFF,
    #[doc = "0x1266 - TCD Transfer Attributes"]
    pub tcd19_attr: TCD_ATTR,
    #[doc = "0x1268 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd19_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x126c - TCD Last Source Address Adjustment"]
    pub tcd19_slast: TCD_SLAST,
    #[doc = "0x1270 - TCD Destination Address"]
    pub tcd19_daddr: TCD_DADDR,
    #[doc = "0x1274 - TCD Signed Destination Address Offset"]
    pub tcd19_doff: TCD_DOFF,
    #[doc = "0x1276 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd19_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1278 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd19_dlastsga: TCD_DLASTSGA,
    #[doc = "0x127c - TCD Control and Status"]
    pub tcd19_csr: TCD_CSR,
    #[doc = "0x127e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd19_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1280 - TCD Source Address"]
    pub tcd20_saddr: TCD_SADDR,
    #[doc = "0x1284 - TCD Signed Source Address Offset"]
    pub tcd20_soff: TCD_SOFF,
    #[doc = "0x1286 - TCD Transfer Attributes"]
    pub tcd20_attr: TCD_ATTR,
    #[doc = "0x1288 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd20_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x128c - TCD Last Source Address Adjustment"]
    pub tcd20_slast: TCD_SLAST,
    #[doc = "0x1290 - TCD Destination Address"]
    pub tcd20_daddr: TCD_DADDR,
    #[doc = "0x1294 - TCD Signed Destination Address Offset"]
    pub tcd20_doff: TCD_DOFF,
    #[doc = "0x1296 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd20_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1298 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd20_dlastsga: TCD_DLASTSGA,
    #[doc = "0x129c - TCD Control and Status"]
    pub tcd20_csr: TCD_CSR,
    #[doc = "0x129e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd20_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x12a0 - TCD Source Address"]
    pub tcd21_saddr: TCD_SADDR,
    #[doc = "0x12a4 - TCD Signed Source Address Offset"]
    pub tcd21_soff: TCD_SOFF,
    #[doc = "0x12a6 - TCD Transfer Attributes"]
    pub tcd21_attr: TCD_ATTR,
    #[doc = "0x12a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd21_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x12ac - TCD Last Source Address Adjustment"]
    pub tcd21_slast: TCD_SLAST,
    #[doc = "0x12b0 - TCD Destination Address"]
    pub tcd21_daddr: TCD_DADDR,
    #[doc = "0x12b4 - TCD Signed Destination Address Offset"]
    pub tcd21_doff: TCD_DOFF,
    #[doc = "0x12b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd21_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x12b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd21_dlastsga: TCD_DLASTSGA,
    #[doc = "0x12bc - TCD Control and Status"]
    pub tcd21_csr: TCD_CSR,
    #[doc = "0x12be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd21_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x12c0 - TCD Source Address"]
    pub tcd22_saddr: TCD_SADDR,
    #[doc = "0x12c4 - TCD Signed Source Address Offset"]
    pub tcd22_soff: TCD_SOFF,
    #[doc = "0x12c6 - TCD Transfer Attributes"]
    pub tcd22_attr: TCD_ATTR,
    #[doc = "0x12c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd22_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x12cc - TCD Last Source Address Adjustment"]
    pub tcd22_slast: TCD_SLAST,
    #[doc = "0x12d0 - TCD Destination Address"]
    pub tcd22_daddr: TCD_DADDR,
    #[doc = "0x12d4 - TCD Signed Destination Address Offset"]
    pub tcd22_doff: TCD_DOFF,
    #[doc = "0x12d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd22_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x12d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd22_dlastsga: TCD_DLASTSGA,
    #[doc = "0x12dc - TCD Control and Status"]
    pub tcd22_csr: TCD_CSR,
    #[doc = "0x12de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd22_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x12e0 - TCD Source Address"]
    pub tcd23_saddr: TCD_SADDR,
    #[doc = "0x12e4 - TCD Signed Source Address Offset"]
    pub tcd23_soff: TCD_SOFF,
    #[doc = "0x12e6 - TCD Transfer Attributes"]
    pub tcd23_attr: TCD_ATTR,
    #[doc = "0x12e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd23_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x12ec - TCD Last Source Address Adjustment"]
    pub tcd23_slast: TCD_SLAST,
    #[doc = "0x12f0 - TCD Destination Address"]
    pub tcd23_daddr: TCD_DADDR,
    #[doc = "0x12f4 - TCD Signed Destination Address Offset"]
    pub tcd23_doff: TCD_DOFF,
    #[doc = "0x12f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd23_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x12f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd23_dlastsga: TCD_DLASTSGA,
    #[doc = "0x12fc - TCD Control and Status"]
    pub tcd23_csr: TCD_CSR,
    #[doc = "0x12fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd23_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1300 - TCD Source Address"]
    pub tcd24_saddr: TCD_SADDR,
    #[doc = "0x1304 - TCD Signed Source Address Offset"]
    pub tcd24_soff: TCD_SOFF,
    #[doc = "0x1306 - TCD Transfer Attributes"]
    pub tcd24_attr: TCD_ATTR,
    #[doc = "0x1308 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd24_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x130c - TCD Last Source Address Adjustment"]
    pub tcd24_slast: TCD_SLAST,
    #[doc = "0x1310 - TCD Destination Address"]
    pub tcd24_daddr: TCD_DADDR,
    #[doc = "0x1314 - TCD Signed Destination Address Offset"]
    pub tcd24_doff: TCD_DOFF,
    #[doc = "0x1316 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd24_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1318 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd24_dlastsga: TCD_DLASTSGA,
    #[doc = "0x131c - TCD Control and Status"]
    pub tcd24_csr: TCD_CSR,
    #[doc = "0x131e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd24_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1320 - TCD Source Address"]
    pub tcd25_saddr: TCD_SADDR,
    #[doc = "0x1324 - TCD Signed Source Address Offset"]
    pub tcd25_soff: TCD_SOFF,
    #[doc = "0x1326 - TCD Transfer Attributes"]
    pub tcd25_attr: TCD_ATTR,
    #[doc = "0x1328 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd25_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x132c - TCD Last Source Address Adjustment"]
    pub tcd25_slast: TCD_SLAST,
    #[doc = "0x1330 - TCD Destination Address"]
    pub tcd25_daddr: TCD_DADDR,
    #[doc = "0x1334 - TCD Signed Destination Address Offset"]
    pub tcd25_doff: TCD_DOFF,
    #[doc = "0x1336 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd25_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1338 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd25_dlastsga: TCD_DLASTSGA,
    #[doc = "0x133c - TCD Control and Status"]
    pub tcd25_csr: TCD_CSR,
    #[doc = "0x133e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd25_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1340 - TCD Source Address"]
    pub tcd26_saddr: TCD_SADDR,
    #[doc = "0x1344 - TCD Signed Source Address Offset"]
    pub tcd26_soff: TCD_SOFF,
    #[doc = "0x1346 - TCD Transfer Attributes"]
    pub tcd26_attr: TCD_ATTR,
    #[doc = "0x1348 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd26_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x134c - TCD Last Source Address Adjustment"]
    pub tcd26_slast: TCD_SLAST,
    #[doc = "0x1350 - TCD Destination Address"]
    pub tcd26_daddr: TCD_DADDR,
    #[doc = "0x1354 - TCD Signed Destination Address Offset"]
    pub tcd26_doff: TCD_DOFF,
    #[doc = "0x1356 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd26_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1358 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd26_dlastsga: TCD_DLASTSGA,
    #[doc = "0x135c - TCD Control and Status"]
    pub tcd26_csr: TCD_CSR,
    #[doc = "0x135e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd26_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1360 - TCD Source Address"]
    pub tcd27_saddr: TCD_SADDR,
    #[doc = "0x1364 - TCD Signed Source Address Offset"]
    pub tcd27_soff: TCD_SOFF,
    #[doc = "0x1366 - TCD Transfer Attributes"]
    pub tcd27_attr: TCD_ATTR,
    #[doc = "0x1368 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd27_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x136c - TCD Last Source Address Adjustment"]
    pub tcd27_slast: TCD_SLAST,
    #[doc = "0x1370 - TCD Destination Address"]
    pub tcd27_daddr: TCD_DADDR,
    #[doc = "0x1374 - TCD Signed Destination Address Offset"]
    pub tcd27_doff: TCD_DOFF,
    #[doc = "0x1376 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd27_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1378 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd27_dlastsga: TCD_DLASTSGA,
    #[doc = "0x137c - TCD Control and Status"]
    pub tcd27_csr: TCD_CSR,
    #[doc = "0x137e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd27_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x1380 - TCD Source Address"]
    pub tcd28_saddr: TCD_SADDR,
    #[doc = "0x1384 - TCD Signed Source Address Offset"]
    pub tcd28_soff: TCD_SOFF,
    #[doc = "0x1386 - TCD Transfer Attributes"]
    pub tcd28_attr: TCD_ATTR,
    #[doc = "0x1388 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd28_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x138c - TCD Last Source Address Adjustment"]
    pub tcd28_slast: TCD_SLAST,
    #[doc = "0x1390 - TCD Destination Address"]
    pub tcd28_daddr: TCD_DADDR,
    #[doc = "0x1394 - TCD Signed Destination Address Offset"]
    pub tcd28_doff: TCD_DOFF,
    #[doc = "0x1396 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd28_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x1398 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd28_dlastsga: TCD_DLASTSGA,
    #[doc = "0x139c - TCD Control and Status"]
    pub tcd28_csr: TCD_CSR,
    #[doc = "0x139e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd28_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x13a0 - TCD Source Address"]
    pub tcd29_saddr: TCD_SADDR,
    #[doc = "0x13a4 - TCD Signed Source Address Offset"]
    pub tcd29_soff: TCD_SOFF,
    #[doc = "0x13a6 - TCD Transfer Attributes"]
    pub tcd29_attr: TCD_ATTR,
    #[doc = "0x13a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd29_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x13ac - TCD Last Source Address Adjustment"]
    pub tcd29_slast: TCD_SLAST,
    #[doc = "0x13b0 - TCD Destination Address"]
    pub tcd29_daddr: TCD_DADDR,
    #[doc = "0x13b4 - TCD Signed Destination Address Offset"]
    pub tcd29_doff: TCD_DOFF,
    #[doc = "0x13b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd29_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x13b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd29_dlastsga: TCD_DLASTSGA,
    #[doc = "0x13bc - TCD Control and Status"]
    pub tcd29_csr: TCD_CSR,
    #[doc = "0x13be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd29_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x13c0 - TCD Source Address"]
    pub tcd30_saddr: TCD_SADDR,
    #[doc = "0x13c4 - TCD Signed Source Address Offset"]
    pub tcd30_soff: TCD_SOFF,
    #[doc = "0x13c6 - TCD Transfer Attributes"]
    pub tcd30_attr: TCD_ATTR,
    #[doc = "0x13c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd30_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x13cc - TCD Last Source Address Adjustment"]
    pub tcd30_slast: TCD_SLAST,
    #[doc = "0x13d0 - TCD Destination Address"]
    pub tcd30_daddr: TCD_DADDR,
    #[doc = "0x13d4 - TCD Signed Destination Address Offset"]
    pub tcd30_doff: TCD_DOFF,
    #[doc = "0x13d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd30_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x13d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd30_dlastsga: TCD_DLASTSGA,
    #[doc = "0x13dc - TCD Control and Status"]
    pub tcd30_csr: TCD_CSR,
    #[doc = "0x13de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd30_biter_elinkno: TCD_BITER_ELINKNO,
    #[doc = "0x13e0 - TCD Source Address"]
    pub tcd31_saddr: TCD_SADDR,
    #[doc = "0x13e4 - TCD Signed Source Address Offset"]
    pub tcd31_soff: TCD_SOFF,
    #[doc = "0x13e6 - TCD Transfer Attributes"]
    pub tcd31_attr: TCD_ATTR,
    #[doc = "0x13e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd31_nbytes_mlno: TCD_NBYTES_MLNO,
    #[doc = "0x13ec - TCD Last Source Address Adjustment"]
    pub tcd31_slast: TCD_SLAST,
    #[doc = "0x13f0 - TCD Destination Address"]
    pub tcd31_daddr: TCD_DADDR,
    #[doc = "0x13f4 - TCD Signed Destination Address Offset"]
    pub tcd31_doff: TCD_DOFF,
    #[doc = "0x13f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd31_citer_elinkno: TCD_CITER_ELINKNO,
    #[doc = "0x13f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd31_dlastsga: TCD_DLASTSGA,
    #[doc = "0x13fc - TCD Control and Status"]
    pub tcd31_csr: TCD_CSR,
    #[doc = "0x13fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd31_biter_elinkno: TCD_BITER_ELINKNO,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Error Status Register"]
pub struct ES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Status Register"]
pub mod es;
#[doc = "Enable Request Register"]
pub struct ERQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "Enable Error Interrupt Register"]
pub struct EEI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "Clear Enable Error Interrupt Register"]
pub struct CEEI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "Set Enable Error Interrupt Register"]
pub struct SEEI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "Clear Enable Request Register"]
pub struct CERQ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "Set Enable Request Register"]
pub struct SERQ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "Clear DONE Status Bit Register"]
pub struct CDNE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "Set START Bit Register"]
pub struct SSRT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "Clear Error Register"]
pub struct CERR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "Clear Interrupt Request Register"]
pub struct CINT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "Interrupt Request Register"]
pub struct INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "Error Register"]
pub struct ERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Register"]
pub mod err;
#[doc = "Hardware Request Status Register"]
pub struct HRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "Enable Asynchronous Request in Stop Register"]
pub struct EARS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Asynchronous Request in Stop Register"]
pub mod ears;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri;
#[doc = "TCD Source Address"]
pub struct TCD_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd_slast;
#[doc = "TCD Destination Address"]
pub struct TCD_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elinkyes;
