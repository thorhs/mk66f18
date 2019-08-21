#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Address Register 1"]
    pub a1: A1,
    #[doc = "0x01 - I2C Frequency Divider register"]
    pub f: F,
    #[doc = "0x02 - I2C Control Register 1"]
    pub c1: C1,
    #[doc = "0x03 - I2C Status register"]
    pub s: S,
    #[doc = "0x04 - I2C Data I/O register"]
    pub d: D,
    #[doc = "0x05 - I2C Control Register 2"]
    pub c2: C2,
    #[doc = "0x06 - I2C Programmable Input Glitch Filter Register"]
    pub flt: FLT,
    #[doc = "0x07 - I2C Range Address register"]
    pub ra: RA,
    #[doc = "0x08 - I2C SMBus Control and Status register"]
    pub smb: SMB,
    #[doc = "0x09 - I2C Address Register 2"]
    pub a2: A2,
    #[doc = "0x0a - I2C SCL Low Timeout Register High"]
    pub slth: SLTH,
    #[doc = "0x0b - I2C SCL Low Timeout Register Low"]
    pub sltl: SLTL,
}
#[doc = "I2C Address Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [a1](a1) module"]
pub type A1 = crate::Reg<u8, _A1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A1;
#[doc = "`read()` method returns [a1::R](a1::R) reader structure"]
impl crate::Readable for A1 {}
#[doc = "`write(|w| ..)` method takes [a1::W](a1::W) writer structure"]
impl crate::Writable for A1 {}
#[doc = "I2C Address Register 1"]
pub mod a1;
#[doc = "I2C Frequency Divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [f](f) module"]
pub type F = crate::Reg<u8, _F>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F;
#[doc = "`read()` method returns [f::R](f::R) reader structure"]
impl crate::Readable for F {}
#[doc = "`write(|w| ..)` method takes [f::W](f::W) writer structure"]
impl crate::Writable for F {}
#[doc = "I2C Frequency Divider register"]
pub mod f;
#[doc = "I2C Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c1](c1) module"]
pub type C1 = crate::Reg<u8, _C1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1;
#[doc = "`read()` method returns [c1::R](c1::R) reader structure"]
impl crate::Readable for C1 {}
#[doc = "`write(|w| ..)` method takes [c1::W](c1::W) writer structure"]
impl crate::Writable for C1 {}
#[doc = "I2C Control Register 1"]
pub mod c1;
#[doc = "I2C Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [s](s) module"]
pub type S = crate::Reg<u8, _S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S;
#[doc = "`read()` method returns [s::R](s::R) reader structure"]
impl crate::Readable for S {}
#[doc = "`write(|w| ..)` method takes [s::W](s::W) writer structure"]
impl crate::Writable for S {}
#[doc = "I2C Status register"]
pub mod s;
#[doc = "I2C Data I/O register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [d](d) module"]
pub type D = crate::Reg<u8, _D>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D;
#[doc = "`read()` method returns [d::R](d::R) reader structure"]
impl crate::Readable for D {}
#[doc = "`write(|w| ..)` method takes [d::W](d::W) writer structure"]
impl crate::Writable for D {}
#[doc = "I2C Data I/O register"]
pub mod d;
#[doc = "I2C Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c2](c2) module"]
pub type C2 = crate::Reg<u8, _C2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2;
#[doc = "`read()` method returns [c2::R](c2::R) reader structure"]
impl crate::Readable for C2 {}
#[doc = "`write(|w| ..)` method takes [c2::W](c2::W) writer structure"]
impl crate::Writable for C2 {}
#[doc = "I2C Control Register 2"]
pub mod c2;
#[doc = "I2C Programmable Input Glitch Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flt](flt) module"]
pub type FLT = crate::Reg<u8, _FLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLT;
#[doc = "`read()` method returns [flt::R](flt::R) reader structure"]
impl crate::Readable for FLT {}
#[doc = "`write(|w| ..)` method takes [flt::W](flt::W) writer structure"]
impl crate::Writable for FLT {}
#[doc = "I2C Programmable Input Glitch Filter Register"]
pub mod flt;
#[doc = "I2C Range Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ra](ra) module"]
pub type RA = crate::Reg<u8, _RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RA;
#[doc = "`read()` method returns [ra::R](ra::R) reader structure"]
impl crate::Readable for RA {}
#[doc = "`write(|w| ..)` method takes [ra::W](ra::W) writer structure"]
impl crate::Writable for RA {}
#[doc = "I2C Range Address register"]
pub mod ra;
#[doc = "I2C SMBus Control and Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [smb](smb) module"]
pub type SMB = crate::Reg<u8, _SMB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMB;
#[doc = "`read()` method returns [smb::R](smb::R) reader structure"]
impl crate::Readable for SMB {}
#[doc = "`write(|w| ..)` method takes [smb::W](smb::W) writer structure"]
impl crate::Writable for SMB {}
#[doc = "I2C SMBus Control and Status register"]
pub mod smb;
#[doc = "I2C Address Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [a2](a2) module"]
pub type A2 = crate::Reg<u8, _A2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A2;
#[doc = "`read()` method returns [a2::R](a2::R) reader structure"]
impl crate::Readable for A2 {}
#[doc = "`write(|w| ..)` method takes [a2::W](a2::W) writer structure"]
impl crate::Writable for A2 {}
#[doc = "I2C Address Register 2"]
pub mod a2;
#[doc = "I2C SCL Low Timeout Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slth](slth) module"]
pub type SLTH = crate::Reg<u8, _SLTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLTH;
#[doc = "`read()` method returns [slth::R](slth::R) reader structure"]
impl crate::Readable for SLTH {}
#[doc = "`write(|w| ..)` method takes [slth::W](slth::W) writer structure"]
impl crate::Writable for SLTH {}
#[doc = "I2C SCL Low Timeout Register High"]
pub mod slth;
#[doc = "I2C SCL Low Timeout Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sltl](sltl) module"]
pub type SLTL = crate::Reg<u8, _SLTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLTL;
#[doc = "`read()` method returns [sltl::R](sltl::R) reader structure"]
impl crate::Readable for SLTL {}
#[doc = "`write(|w| ..)` method takes [sltl::W](sltl::W) writer structure"]
impl crate::Writable for SLTL {}
#[doc = "I2C SCL Low Timeout Register Low"]
pub mod sltl;
