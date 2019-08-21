#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMT Carrier Generator High Data Register 1"]
    pub cgh1: CGH1,
    #[doc = "0x01 - CMT Carrier Generator Low Data Register 1"]
    pub cgl1: CGL1,
    #[doc = "0x02 - CMT Carrier Generator High Data Register 2"]
    pub cgh2: CGH2,
    #[doc = "0x03 - CMT Carrier Generator Low Data Register 2"]
    pub cgl2: CGL2,
    #[doc = "0x04 - CMT Output Control Register"]
    pub oc: OC,
    #[doc = "0x05 - CMT Modulator Status and Control Register"]
    pub msc: MSC,
    #[doc = "0x06 - CMT Modulator Data Register Mark High"]
    pub cmd1: CMD1,
    #[doc = "0x07 - CMT Modulator Data Register Mark Low"]
    pub cmd2: CMD2,
    #[doc = "0x08 - CMT Modulator Data Register Space High"]
    pub cmd3: CMD3,
    #[doc = "0x09 - CMT Modulator Data Register Space Low"]
    pub cmd4: CMD4,
    #[doc = "0x0a - CMT Primary Prescaler Register"]
    pub pps: PPS,
    #[doc = "0x0b - CMT Direct Memory Access Register"]
    pub dma: DMA,
}
#[doc = "CMT Carrier Generator High Data Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cgh1](cgh1) module"]
pub type CGH1 = crate::Reg<u8, _CGH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGH1;
#[doc = "`read()` method returns [cgh1::R](cgh1::R) reader structure"]
impl crate::Readable for CGH1 {}
#[doc = "`write(|w| ..)` method takes [cgh1::W](cgh1::W) writer structure"]
impl crate::Writable for CGH1 {}
#[doc = "CMT Carrier Generator High Data Register 1"]
pub mod cgh1;
#[doc = "CMT Carrier Generator Low Data Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cgl1](cgl1) module"]
pub type CGL1 = crate::Reg<u8, _CGL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGL1;
#[doc = "`read()` method returns [cgl1::R](cgl1::R) reader structure"]
impl crate::Readable for CGL1 {}
#[doc = "`write(|w| ..)` method takes [cgl1::W](cgl1::W) writer structure"]
impl crate::Writable for CGL1 {}
#[doc = "CMT Carrier Generator Low Data Register 1"]
pub mod cgl1;
#[doc = "CMT Carrier Generator High Data Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cgh2](cgh2) module"]
pub type CGH2 = crate::Reg<u8, _CGH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGH2;
#[doc = "`read()` method returns [cgh2::R](cgh2::R) reader structure"]
impl crate::Readable for CGH2 {}
#[doc = "`write(|w| ..)` method takes [cgh2::W](cgh2::W) writer structure"]
impl crate::Writable for CGH2 {}
#[doc = "CMT Carrier Generator High Data Register 2"]
pub mod cgh2;
#[doc = "CMT Carrier Generator Low Data Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cgl2](cgl2) module"]
pub type CGL2 = crate::Reg<u8, _CGL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CGL2;
#[doc = "`read()` method returns [cgl2::R](cgl2::R) reader structure"]
impl crate::Readable for CGL2 {}
#[doc = "`write(|w| ..)` method takes [cgl2::W](cgl2::W) writer structure"]
impl crate::Writable for CGL2 {}
#[doc = "CMT Carrier Generator Low Data Register 2"]
pub mod cgl2;
#[doc = "CMT Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [oc](oc) module"]
pub type OC = crate::Reg<u8, _OC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC;
#[doc = "`read()` method returns [oc::R](oc::R) reader structure"]
impl crate::Readable for OC {}
#[doc = "`write(|w| ..)` method takes [oc::W](oc::W) writer structure"]
impl crate::Writable for OC {}
#[doc = "CMT Output Control Register"]
pub mod oc;
#[doc = "CMT Modulator Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msc](msc) module"]
pub type MSC = crate::Reg<u8, _MSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSC;
#[doc = "`read()` method returns [msc::R](msc::R) reader structure"]
impl crate::Readable for MSC {}
#[doc = "`write(|w| ..)` method takes [msc::W](msc::W) writer structure"]
impl crate::Writable for MSC {}
#[doc = "CMT Modulator Status and Control Register"]
pub mod msc;
#[doc = "CMT Modulator Data Register Mark High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd1](cmd1) module"]
pub type CMD1 = crate::Reg<u8, _CMD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD1;
#[doc = "`read()` method returns [cmd1::R](cmd1::R) reader structure"]
impl crate::Readable for CMD1 {}
#[doc = "`write(|w| ..)` method takes [cmd1::W](cmd1::W) writer structure"]
impl crate::Writable for CMD1 {}
#[doc = "CMT Modulator Data Register Mark High"]
pub mod cmd1;
#[doc = "CMT Modulator Data Register Mark Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd2](cmd2) module"]
pub type CMD2 = crate::Reg<u8, _CMD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD2;
#[doc = "`read()` method returns [cmd2::R](cmd2::R) reader structure"]
impl crate::Readable for CMD2 {}
#[doc = "`write(|w| ..)` method takes [cmd2::W](cmd2::W) writer structure"]
impl crate::Writable for CMD2 {}
#[doc = "CMT Modulator Data Register Mark Low"]
pub mod cmd2;
#[doc = "CMT Modulator Data Register Space High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd3](cmd3) module"]
pub type CMD3 = crate::Reg<u8, _CMD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD3;
#[doc = "`read()` method returns [cmd3::R](cmd3::R) reader structure"]
impl crate::Readable for CMD3 {}
#[doc = "`write(|w| ..)` method takes [cmd3::W](cmd3::W) writer structure"]
impl crate::Writable for CMD3 {}
#[doc = "CMT Modulator Data Register Space High"]
pub mod cmd3;
#[doc = "CMT Modulator Data Register Space Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd4](cmd4) module"]
pub type CMD4 = crate::Reg<u8, _CMD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD4;
#[doc = "`read()` method returns [cmd4::R](cmd4::R) reader structure"]
impl crate::Readable for CMD4 {}
#[doc = "`write(|w| ..)` method takes [cmd4::W](cmd4::W) writer structure"]
impl crate::Writable for CMD4 {}
#[doc = "CMT Modulator Data Register Space Low"]
pub mod cmd4;
#[doc = "CMT Primary Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pps](pps) module"]
pub type PPS = crate::Reg<u8, _PPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPS;
#[doc = "`read()` method returns [pps::R](pps::R) reader structure"]
impl crate::Readable for PPS {}
#[doc = "`write(|w| ..)` method takes [pps::W](pps::W) writer structure"]
impl crate::Writable for PPS {}
#[doc = "CMT Primary Prescaler Register"]
pub mod pps;
#[doc = "CMT Direct Memory Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma](dma) module"]
pub type DMA = crate::Reg<u8, _DMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA;
#[doc = "`read()` method returns [dma::R](dma::R) reader structure"]
impl crate::Readable for DMA {}
#[doc = "`write(|w| ..)` method takes [dma::W](dma::W) writer structure"]
impl crate::Writable for DMA {}
#[doc = "CMT Direct Memory Access Register"]
pub mod dma;
