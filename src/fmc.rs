#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Access Protection Register"]
    pub pfapr: PFAPR,
    #[doc = "0x04 - Flash Bank 0-1 Control Register"]
    pub pfb01cr: PFB01CR,
    #[doc = "0x08 - Flash Bank 2-3 Control Register"]
    pub pfb23cr: PFB23CR,
    _reserved3: [u8; 244usize],
    #[doc = "0x100 - Cache Tag Storage"]
    pub tagvdw0s: [TAGVDW0S; 4],
    #[doc = "0x110 - Cache Tag Storage"]
    pub tagvdw1s: [TAGVDW1S; 4],
    #[doc = "0x120 - Cache Tag Storage"]
    pub tagvdw2s: [TAGVDW2S; 4],
    #[doc = "0x130 - Cache Tag Storage"]
    pub tagvdw3s: [TAGVDW3S; 4],
    _reserved7: [u8; 192usize],
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
#[doc = "Flash Access Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfapr](pfapr) module"]
pub type PFAPR = crate::Reg<u32, _PFAPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFAPR;
#[doc = "`read()` method returns [pfapr::R](pfapr::R) reader structure"]
impl crate::Readable for PFAPR {}
#[doc = "`write(|w| ..)` method takes [pfapr::W](pfapr::W) writer structure"]
impl crate::Writable for PFAPR {}
#[doc = "Flash Access Protection Register"]
pub mod pfapr;
#[doc = "Flash Bank 0-1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfb01cr](pfb01cr) module"]
pub type PFB01CR = crate::Reg<u32, _PFB01CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFB01CR;
#[doc = "`read()` method returns [pfb01cr::R](pfb01cr::R) reader structure"]
impl crate::Readable for PFB01CR {}
#[doc = "`write(|w| ..)` method takes [pfb01cr::W](pfb01cr::W) writer structure"]
impl crate::Writable for PFB01CR {}
#[doc = "Flash Bank 0-1 Control Register"]
pub mod pfb01cr;
#[doc = "Flash Bank 2-3 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfb23cr](pfb23cr) module"]
pub type PFB23CR = crate::Reg<u32, _PFB23CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFB23CR;
#[doc = "`read()` method returns [pfb23cr::R](pfb23cr::R) reader structure"]
impl crate::Readable for PFB23CR {}
#[doc = "`write(|w| ..)` method takes [pfb23cr::W](pfb23cr::W) writer structure"]
impl crate::Writable for PFB23CR {}
#[doc = "Flash Bank 2-3 Control Register"]
pub mod pfb23cr;
#[doc = "Cache Tag Storage\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tagvdw0s](tagvdw0s) module"]
pub type TAGVDW0S = crate::Reg<u32, _TAGVDW0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAGVDW0S;
#[doc = "`read()` method returns [tagvdw0s::R](tagvdw0s::R) reader structure"]
impl crate::Readable for TAGVDW0S {}
#[doc = "`write(|w| ..)` method takes [tagvdw0s::W](tagvdw0s::W) writer structure"]
impl crate::Writable for TAGVDW0S {}
#[doc = "Cache Tag Storage"]
pub mod tagvdw0s;
#[doc = "Cache Tag Storage\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tagvdw1s](tagvdw1s) module"]
pub type TAGVDW1S = crate::Reg<u32, _TAGVDW1S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAGVDW1S;
#[doc = "`read()` method returns [tagvdw1s::R](tagvdw1s::R) reader structure"]
impl crate::Readable for TAGVDW1S {}
#[doc = "`write(|w| ..)` method takes [tagvdw1s::W](tagvdw1s::W) writer structure"]
impl crate::Writable for TAGVDW1S {}
#[doc = "Cache Tag Storage"]
pub mod tagvdw1s;
#[doc = "Cache Tag Storage\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tagvdw2s](tagvdw2s) module"]
pub type TAGVDW2S = crate::Reg<u32, _TAGVDW2S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAGVDW2S;
#[doc = "`read()` method returns [tagvdw2s::R](tagvdw2s::R) reader structure"]
impl crate::Readable for TAGVDW2S {}
#[doc = "`write(|w| ..)` method takes [tagvdw2s::W](tagvdw2s::W) writer structure"]
impl crate::Writable for TAGVDW2S {}
#[doc = "Cache Tag Storage"]
pub mod tagvdw2s;
#[doc = "Cache Tag Storage\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tagvdw3s](tagvdw3s) module"]
pub type TAGVDW3S = crate::Reg<u32, _TAGVDW3S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAGVDW3S;
#[doc = "`read()` method returns [tagvdw3s::R](tagvdw3s::R) reader structure"]
impl crate::Readable for TAGVDW3S {}
#[doc = "`write(|w| ..)` method takes [tagvdw3s::W](tagvdw3s::W) writer structure"]
impl crate::Writable for TAGVDW3S {}
#[doc = "Cache Tag Storage"]
pub mod tagvdw3s;
#[doc = "Cache Data Storage (uppermost word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw0sum](dataw0sum) module"]
pub type DATAW0SUM = crate::Reg<u32, _DATAW0SUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW0SUM;
#[doc = "`read()` method returns [dataw0sum::R](dataw0sum::R) reader structure"]
impl crate::Readable for DATAW0SUM {}
#[doc = "`write(|w| ..)` method takes [dataw0sum::W](dataw0sum::W) writer structure"]
impl crate::Writable for DATAW0SUM {}
#[doc = "Cache Data Storage (uppermost word)"]
pub mod dataw0sum;
#[doc = "Cache Data Storage (mid-upper word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw0smu](dataw0smu) module"]
pub type DATAW0SMU = crate::Reg<u32, _DATAW0SMU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW0SMU;
#[doc = "`read()` method returns [dataw0smu::R](dataw0smu::R) reader structure"]
impl crate::Readable for DATAW0SMU {}
#[doc = "`write(|w| ..)` method takes [dataw0smu::W](dataw0smu::W) writer structure"]
impl crate::Writable for DATAW0SMU {}
#[doc = "Cache Data Storage (mid-upper word)"]
pub mod dataw0smu;
#[doc = "Cache Data Storage (mid-lower word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw0sml](dataw0sml) module"]
pub type DATAW0SML = crate::Reg<u32, _DATAW0SML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW0SML;
#[doc = "`read()` method returns [dataw0sml::R](dataw0sml::R) reader structure"]
impl crate::Readable for DATAW0SML {}
#[doc = "`write(|w| ..)` method takes [dataw0sml::W](dataw0sml::W) writer structure"]
impl crate::Writable for DATAW0SML {}
#[doc = "Cache Data Storage (mid-lower word)"]
pub mod dataw0sml;
#[doc = "Cache Data Storage (lowermost word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw0slm](dataw0slm) module"]
pub type DATAW0SLM = crate::Reg<u32, _DATAW0SLM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW0SLM;
#[doc = "`read()` method returns [dataw0slm::R](dataw0slm::R) reader structure"]
impl crate::Readable for DATAW0SLM {}
#[doc = "`write(|w| ..)` method takes [dataw0slm::W](dataw0slm::W) writer structure"]
impl crate::Writable for DATAW0SLM {}
#[doc = "Cache Data Storage (lowermost word)"]
pub mod dataw0slm;
#[doc = "Cache Data Storage (uppermost word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw1sum](dataw1sum) module"]
pub type DATAW1SUM = crate::Reg<u32, _DATAW1SUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW1SUM;
#[doc = "`read()` method returns [dataw1sum::R](dataw1sum::R) reader structure"]
impl crate::Readable for DATAW1SUM {}
#[doc = "`write(|w| ..)` method takes [dataw1sum::W](dataw1sum::W) writer structure"]
impl crate::Writable for DATAW1SUM {}
#[doc = "Cache Data Storage (uppermost word)"]
pub mod dataw1sum;
#[doc = "Cache Data Storage (mid-upper word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw1smu](dataw1smu) module"]
pub type DATAW1SMU = crate::Reg<u32, _DATAW1SMU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW1SMU;
#[doc = "`read()` method returns [dataw1smu::R](dataw1smu::R) reader structure"]
impl crate::Readable for DATAW1SMU {}
#[doc = "`write(|w| ..)` method takes [dataw1smu::W](dataw1smu::W) writer structure"]
impl crate::Writable for DATAW1SMU {}
#[doc = "Cache Data Storage (mid-upper word)"]
pub mod dataw1smu;
#[doc = "Cache Data Storage (mid-lower word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw1sml](dataw1sml) module"]
pub type DATAW1SML = crate::Reg<u32, _DATAW1SML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW1SML;
#[doc = "`read()` method returns [dataw1sml::R](dataw1sml::R) reader structure"]
impl crate::Readable for DATAW1SML {}
#[doc = "`write(|w| ..)` method takes [dataw1sml::W](dataw1sml::W) writer structure"]
impl crate::Writable for DATAW1SML {}
#[doc = "Cache Data Storage (mid-lower word)"]
pub mod dataw1sml;
#[doc = "Cache Data Storage (lowermost word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw1slm](dataw1slm) module"]
pub type DATAW1SLM = crate::Reg<u32, _DATAW1SLM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW1SLM;
#[doc = "`read()` method returns [dataw1slm::R](dataw1slm::R) reader structure"]
impl crate::Readable for DATAW1SLM {}
#[doc = "`write(|w| ..)` method takes [dataw1slm::W](dataw1slm::W) writer structure"]
impl crate::Writable for DATAW1SLM {}
#[doc = "Cache Data Storage (lowermost word)"]
pub mod dataw1slm;
#[doc = "Cache Data Storage (uppermost word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw2sum](dataw2sum) module"]
pub type DATAW2SUM = crate::Reg<u32, _DATAW2SUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW2SUM;
#[doc = "`read()` method returns [dataw2sum::R](dataw2sum::R) reader structure"]
impl crate::Readable for DATAW2SUM {}
#[doc = "`write(|w| ..)` method takes [dataw2sum::W](dataw2sum::W) writer structure"]
impl crate::Writable for DATAW2SUM {}
#[doc = "Cache Data Storage (uppermost word)"]
pub mod dataw2sum;
#[doc = "Cache Data Storage (mid-upper word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw2smu](dataw2smu) module"]
pub type DATAW2SMU = crate::Reg<u32, _DATAW2SMU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW2SMU;
#[doc = "`read()` method returns [dataw2smu::R](dataw2smu::R) reader structure"]
impl crate::Readable for DATAW2SMU {}
#[doc = "`write(|w| ..)` method takes [dataw2smu::W](dataw2smu::W) writer structure"]
impl crate::Writable for DATAW2SMU {}
#[doc = "Cache Data Storage (mid-upper word)"]
pub mod dataw2smu;
#[doc = "Cache Data Storage (mid-lower word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw2sml](dataw2sml) module"]
pub type DATAW2SML = crate::Reg<u32, _DATAW2SML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW2SML;
#[doc = "`read()` method returns [dataw2sml::R](dataw2sml::R) reader structure"]
impl crate::Readable for DATAW2SML {}
#[doc = "`write(|w| ..)` method takes [dataw2sml::W](dataw2sml::W) writer structure"]
impl crate::Writable for DATAW2SML {}
#[doc = "Cache Data Storage (mid-lower word)"]
pub mod dataw2sml;
#[doc = "Cache Data Storage (lowermost word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw2slm](dataw2slm) module"]
pub type DATAW2SLM = crate::Reg<u32, _DATAW2SLM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW2SLM;
#[doc = "`read()` method returns [dataw2slm::R](dataw2slm::R) reader structure"]
impl crate::Readable for DATAW2SLM {}
#[doc = "`write(|w| ..)` method takes [dataw2slm::W](dataw2slm::W) writer structure"]
impl crate::Writable for DATAW2SLM {}
#[doc = "Cache Data Storage (lowermost word)"]
pub mod dataw2slm;
#[doc = "Cache Data Storage (uppermost word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw3sum](dataw3sum) module"]
pub type DATAW3SUM = crate::Reg<u32, _DATAW3SUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW3SUM;
#[doc = "`read()` method returns [dataw3sum::R](dataw3sum::R) reader structure"]
impl crate::Readable for DATAW3SUM {}
#[doc = "`write(|w| ..)` method takes [dataw3sum::W](dataw3sum::W) writer structure"]
impl crate::Writable for DATAW3SUM {}
#[doc = "Cache Data Storage (uppermost word)"]
pub mod dataw3sum;
#[doc = "Cache Data Storage (mid-upper word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw3smu](dataw3smu) module"]
pub type DATAW3SMU = crate::Reg<u32, _DATAW3SMU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW3SMU;
#[doc = "`read()` method returns [dataw3smu::R](dataw3smu::R) reader structure"]
impl crate::Readable for DATAW3SMU {}
#[doc = "`write(|w| ..)` method takes [dataw3smu::W](dataw3smu::W) writer structure"]
impl crate::Writable for DATAW3SMU {}
#[doc = "Cache Data Storage (mid-upper word)"]
pub mod dataw3smu;
#[doc = "Cache Data Storage (mid-lower word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw3sml](dataw3sml) module"]
pub type DATAW3SML = crate::Reg<u32, _DATAW3SML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW3SML;
#[doc = "`read()` method returns [dataw3sml::R](dataw3sml::R) reader structure"]
impl crate::Readable for DATAW3SML {}
#[doc = "`write(|w| ..)` method takes [dataw3sml::W](dataw3sml::W) writer structure"]
impl crate::Writable for DATAW3SML {}
#[doc = "Cache Data Storage (mid-lower word)"]
pub mod dataw3sml;
#[doc = "Cache Data Storage (lowermost word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dataw3slm](dataw3slm) module"]
pub type DATAW3SLM = crate::Reg<u32, _DATAW3SLM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAW3SLM;
#[doc = "`read()` method returns [dataw3slm::R](dataw3slm::R) reader structure"]
impl crate::Readable for DATAW3SLM {}
#[doc = "`write(|w| ..)` method takes [dataw3slm::W](dataw3slm::W) writer structure"]
impl crate::Writable for DATAW3SLM {}
#[doc = "Cache Data Storage (lowermost word)"]
pub mod dataw3slm;
