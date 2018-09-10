#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Access Protection Register"]
    pub pfapr: PFAPR,
    #[doc = "0x04 - Flash Bank 0-1 Control Register"]
    pub pfb01cr: PFB01CR,
    #[doc = "0x08 - Flash Bank 2-3 Control Register"]
    pub pfb23cr: PFB23CR,
    _reserved0: [u8; 244usize],
    #[doc = "0x100 - Cache Tag Storage"]
    pub tagvdw0s: [TAGVDW0S; 4],
    #[doc = "0x110 - Cache Tag Storage"]
    pub tagvdw1s: [TAGVDW1S; 4],
    #[doc = "0x120 - Cache Tag Storage"]
    pub tagvdw2s: [TAGVDW2S; 4],
    #[doc = "0x130 - Cache Tag Storage"]
    pub tagvdw3s: [TAGVDW3S; 4],
    _reserved1: [u8; 192usize],
    #[doc = "0x200 - Cache Data Storage (uppermost word)"]
    pub dataw0s0um: DATAW0SUM,
    #[doc = "0x204 - Cache Data Storage (mid-upper word)"]
    pub dataw0s0mu: DATAW0SMU,
    #[doc = "0x208 - Cache Data Storage (mid-lower word)"]
    pub dataw0s0ml: DATAW0SML,
    #[doc = "0x20c - Cache Data Storage (lowermost word)"]
    pub dataw0s0lm: DATAW0SLM,
    #[doc = "0x210 - Cache Data Storage (uppermost word)"]
    pub dataw0s1um: DATAW0SUM,
    #[doc = "0x214 - Cache Data Storage (mid-upper word)"]
    pub dataw0s1mu: DATAW0SMU,
    #[doc = "0x218 - Cache Data Storage (mid-lower word)"]
    pub dataw0s1ml: DATAW0SML,
    #[doc = "0x21c - Cache Data Storage (lowermost word)"]
    pub dataw0s1lm: DATAW0SLM,
    #[doc = "0x220 - Cache Data Storage (uppermost word)"]
    pub dataw0s2um: DATAW0SUM,
    #[doc = "0x224 - Cache Data Storage (mid-upper word)"]
    pub dataw0s2mu: DATAW0SMU,
    #[doc = "0x228 - Cache Data Storage (mid-lower word)"]
    pub dataw0s2ml: DATAW0SML,
    #[doc = "0x22c - Cache Data Storage (lowermost word)"]
    pub dataw0s2lm: DATAW0SLM,
    #[doc = "0x230 - Cache Data Storage (uppermost word)"]
    pub dataw0s3um: DATAW0SUM,
    #[doc = "0x234 - Cache Data Storage (mid-upper word)"]
    pub dataw0s3mu: DATAW0SMU,
    #[doc = "0x238 - Cache Data Storage (mid-lower word)"]
    pub dataw0s3ml: DATAW0SML,
    #[doc = "0x23c - Cache Data Storage (lowermost word)"]
    pub dataw0s3lm: DATAW0SLM,
    #[doc = "0x240 - Cache Data Storage (uppermost word)"]
    pub dataw1s0um: DATAW1SUM,
    #[doc = "0x244 - Cache Data Storage (mid-upper word)"]
    pub dataw1s0mu: DATAW1SMU,
    #[doc = "0x248 - Cache Data Storage (mid-lower word)"]
    pub dataw1s0ml: DATAW1SML,
    #[doc = "0x24c - Cache Data Storage (lowermost word)"]
    pub dataw1s0lm: DATAW1SLM,
    #[doc = "0x250 - Cache Data Storage (uppermost word)"]
    pub dataw1s1um: DATAW1SUM,
    #[doc = "0x254 - Cache Data Storage (mid-upper word)"]
    pub dataw1s1mu: DATAW1SMU,
    #[doc = "0x258 - Cache Data Storage (mid-lower word)"]
    pub dataw1s1ml: DATAW1SML,
    #[doc = "0x25c - Cache Data Storage (lowermost word)"]
    pub dataw1s1lm: DATAW1SLM,
    #[doc = "0x260 - Cache Data Storage (uppermost word)"]
    pub dataw1s2um: DATAW1SUM,
    #[doc = "0x264 - Cache Data Storage (mid-upper word)"]
    pub dataw1s2mu: DATAW1SMU,
    #[doc = "0x268 - Cache Data Storage (mid-lower word)"]
    pub dataw1s2ml: DATAW1SML,
    #[doc = "0x26c - Cache Data Storage (lowermost word)"]
    pub dataw1s2lm: DATAW1SLM,
    #[doc = "0x270 - Cache Data Storage (uppermost word)"]
    pub dataw1s3um: DATAW1SUM,
    #[doc = "0x274 - Cache Data Storage (mid-upper word)"]
    pub dataw1s3mu: DATAW1SMU,
    #[doc = "0x278 - Cache Data Storage (mid-lower word)"]
    pub dataw1s3ml: DATAW1SML,
    #[doc = "0x27c - Cache Data Storage (lowermost word)"]
    pub dataw1s3lm: DATAW1SLM,
    #[doc = "0x280 - Cache Data Storage (uppermost word)"]
    pub dataw2s0um: DATAW2SUM,
    #[doc = "0x284 - Cache Data Storage (mid-upper word)"]
    pub dataw2s0mu: DATAW2SMU,
    #[doc = "0x288 - Cache Data Storage (mid-lower word)"]
    pub dataw2s0ml: DATAW2SML,
    #[doc = "0x28c - Cache Data Storage (lowermost word)"]
    pub dataw2s0lm: DATAW2SLM,
    #[doc = "0x290 - Cache Data Storage (uppermost word)"]
    pub dataw2s1um: DATAW2SUM,
    #[doc = "0x294 - Cache Data Storage (mid-upper word)"]
    pub dataw2s1mu: DATAW2SMU,
    #[doc = "0x298 - Cache Data Storage (mid-lower word)"]
    pub dataw2s1ml: DATAW2SML,
    #[doc = "0x29c - Cache Data Storage (lowermost word)"]
    pub dataw2s1lm: DATAW2SLM,
    #[doc = "0x2a0 - Cache Data Storage (uppermost word)"]
    pub dataw2s2um: DATAW2SUM,
    #[doc = "0x2a4 - Cache Data Storage (mid-upper word)"]
    pub dataw2s2mu: DATAW2SMU,
    #[doc = "0x2a8 - Cache Data Storage (mid-lower word)"]
    pub dataw2s2ml: DATAW2SML,
    #[doc = "0x2ac - Cache Data Storage (lowermost word)"]
    pub dataw2s2lm: DATAW2SLM,
    #[doc = "0x2b0 - Cache Data Storage (uppermost word)"]
    pub dataw2s3um: DATAW2SUM,
    #[doc = "0x2b4 - Cache Data Storage (mid-upper word)"]
    pub dataw2s3mu: DATAW2SMU,
    #[doc = "0x2b8 - Cache Data Storage (mid-lower word)"]
    pub dataw2s3ml: DATAW2SML,
    #[doc = "0x2bc - Cache Data Storage (lowermost word)"]
    pub dataw2s3lm: DATAW2SLM,
    #[doc = "0x2c0 - Cache Data Storage (uppermost word)"]
    pub dataw3s0um: DATAW3SUM,
    #[doc = "0x2c4 - Cache Data Storage (mid-upper word)"]
    pub dataw3s0mu: DATAW3SMU,
    #[doc = "0x2c8 - Cache Data Storage (mid-lower word)"]
    pub dataw3s0ml: DATAW3SML,
    #[doc = "0x2cc - Cache Data Storage (lowermost word)"]
    pub dataw3s0lm: DATAW3SLM,
    #[doc = "0x2d0 - Cache Data Storage (uppermost word)"]
    pub dataw3s1um: DATAW3SUM,
    #[doc = "0x2d4 - Cache Data Storage (mid-upper word)"]
    pub dataw3s1mu: DATAW3SMU,
    #[doc = "0x2d8 - Cache Data Storage (mid-lower word)"]
    pub dataw3s1ml: DATAW3SML,
    #[doc = "0x2dc - Cache Data Storage (lowermost word)"]
    pub dataw3s1lm: DATAW3SLM,
    #[doc = "0x2e0 - Cache Data Storage (uppermost word)"]
    pub dataw3s2um: DATAW3SUM,
    #[doc = "0x2e4 - Cache Data Storage (mid-upper word)"]
    pub dataw3s2mu: DATAW3SMU,
    #[doc = "0x2e8 - Cache Data Storage (mid-lower word)"]
    pub dataw3s2ml: DATAW3SML,
    #[doc = "0x2ec - Cache Data Storage (lowermost word)"]
    pub dataw3s2lm: DATAW3SLM,
    #[doc = "0x2f0 - Cache Data Storage (uppermost word)"]
    pub dataw3s3um: DATAW3SUM,
    #[doc = "0x2f4 - Cache Data Storage (mid-upper word)"]
    pub dataw3s3mu: DATAW3SMU,
    #[doc = "0x2f8 - Cache Data Storage (mid-lower word)"]
    pub dataw3s3ml: DATAW3SML,
    #[doc = "0x2fc - Cache Data Storage (lowermost word)"]
    pub dataw3s3lm: DATAW3SLM,
}
#[doc = "Flash Access Protection Register"]
pub struct PFAPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Access Protection Register"]
pub mod pfapr;
#[doc = "Flash Bank 0-1 Control Register"]
pub struct PFB01CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 0-1 Control Register"]
pub mod pfb01cr;
#[doc = "Flash Bank 2-3 Control Register"]
pub struct PFB23CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 2-3 Control Register"]
pub mod pfb23cr;
#[doc = "Cache Tag Storage"]
pub struct TAGVDW0S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Tag Storage"]
pub mod tagvdw0s;
#[doc = "Cache Tag Storage"]
pub struct TAGVDW1S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Tag Storage"]
pub mod tagvdw1s;
#[doc = "Cache Tag Storage"]
pub struct TAGVDW2S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Tag Storage"]
pub mod tagvdw2s;
#[doc = "Cache Tag Storage"]
pub struct TAGVDW3S {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Tag Storage"]
pub mod tagvdw3s;
#[doc = "Cache Data Storage (uppermost word)"]
pub struct DATAW0SUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (uppermost word)"]
pub mod dataw0sum;
#[doc = "Cache Data Storage (mid-upper word)"]
pub struct DATAW0SMU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (mid-upper word)"]
pub mod dataw0smu;
#[doc = "Cache Data Storage (mid-lower word)"]
pub struct DATAW0SML {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (mid-lower word)"]
pub mod dataw0sml;
#[doc = "Cache Data Storage (lowermost word)"]
pub struct DATAW0SLM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (lowermost word)"]
pub mod dataw0slm;
#[doc = "Cache Data Storage (uppermost word)"]
pub struct DATAW1SUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (uppermost word)"]
pub mod dataw1sum;
#[doc = "Cache Data Storage (mid-upper word)"]
pub struct DATAW1SMU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (mid-upper word)"]
pub mod dataw1smu;
#[doc = "Cache Data Storage (mid-lower word)"]
pub struct DATAW1SML {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (mid-lower word)"]
pub mod dataw1sml;
#[doc = "Cache Data Storage (lowermost word)"]
pub struct DATAW1SLM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (lowermost word)"]
pub mod dataw1slm;
#[doc = "Cache Data Storage (uppermost word)"]
pub struct DATAW2SUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (uppermost word)"]
pub mod dataw2sum;
#[doc = "Cache Data Storage (mid-upper word)"]
pub struct DATAW2SMU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (mid-upper word)"]
pub mod dataw2smu;
#[doc = "Cache Data Storage (mid-lower word)"]
pub struct DATAW2SML {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (mid-lower word)"]
pub mod dataw2sml;
#[doc = "Cache Data Storage (lowermost word)"]
pub struct DATAW2SLM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (lowermost word)"]
pub mod dataw2slm;
#[doc = "Cache Data Storage (uppermost word)"]
pub struct DATAW3SUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (uppermost word)"]
pub mod dataw3sum;
#[doc = "Cache Data Storage (mid-upper word)"]
pub struct DATAW3SMU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (mid-upper word)"]
pub mod dataw3smu;
#[doc = "Cache Data Storage (mid-lower word)"]
pub struct DATAW3SML {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (mid-lower word)"]
pub mod dataw3sml;
#[doc = "Cache Data Storage (lowermost word)"]
pub struct DATAW3SLM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Data Storage (lowermost word)"]
pub mod dataw3slm;
