#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_data: [u8; 4usize],
    _reserved_1_gpoly: [u8; 4usize],
    _reserved_2_ctrl: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x00 - CRC_DATALL register."]
    #[inline(always)]
    pub fn datall(&self) -> &DATALL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DATALL) }
    }
    #[doc = "0x00 - CRC_DATALL register."]
    #[inline(always)]
    pub fn datall_mut(&self) -> &mut DATALL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DATALL) }
    }
    #[doc = "0x00 - CRC_DATAL register."]
    #[inline(always)]
    pub fn datal(&self) -> &DATAL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DATAL) }
    }
    #[doc = "0x00 - CRC_DATAL register."]
    #[inline(always)]
    pub fn datal_mut(&self) -> &mut DATAL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DATAL) }
    }
    #[doc = "0x00 - CRC Data register"]
    #[inline(always)]
    pub fn data(&self) -> &DATA {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DATA) }
    }
    #[doc = "0x00 - CRC Data register"]
    #[inline(always)]
    pub fn data_mut(&self) -> &mut DATA {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DATA) }
    }
    #[doc = "0x01 - CRC_DATALU register."]
    #[inline(always)]
    pub fn datalu(&self) -> &DATALU {
        unsafe { &*(((self as *const Self) as *const u8).add(1usize) as *const DATALU) }
    }
    #[doc = "0x01 - CRC_DATALU register."]
    #[inline(always)]
    pub fn datalu_mut(&self) -> &mut DATALU {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1usize) as *mut DATALU) }
    }
    #[doc = "0x02 - CRC_DATAHL register."]
    #[inline(always)]
    pub fn datahl(&self) -> &DATAHL {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const DATAHL) }
    }
    #[doc = "0x02 - CRC_DATAHL register."]
    #[inline(always)]
    pub fn datahl_mut(&self) -> &mut DATAHL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2usize) as *mut DATAHL) }
    }
    #[doc = "0x02 - CRC_DATAH register."]
    #[inline(always)]
    pub fn datah(&self) -> &DATAH {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const DATAH) }
    }
    #[doc = "0x02 - CRC_DATAH register."]
    #[inline(always)]
    pub fn datah_mut(&self) -> &mut DATAH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2usize) as *mut DATAH) }
    }
    #[doc = "0x03 - CRC_DATAHU register."]
    #[inline(always)]
    pub fn datahu(&self) -> &DATAHU {
        unsafe { &*(((self as *const Self) as *const u8).add(3usize) as *const DATAHU) }
    }
    #[doc = "0x03 - CRC_DATAHU register."]
    #[inline(always)]
    pub fn datahu_mut(&self) -> &mut DATAHU {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3usize) as *mut DATAHU) }
    }
    #[doc = "0x04 - CRC_GPOLYLL register."]
    #[inline(always)]
    pub fn gpolyll(&self) -> &GPOLYLL {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const GPOLYLL) }
    }
    #[doc = "0x04 - CRC_GPOLYLL register."]
    #[inline(always)]
    pub fn gpolyll_mut(&self) -> &mut GPOLYLL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut GPOLYLL) }
    }
    #[doc = "0x04 - CRC_GPOLYL register."]
    #[inline(always)]
    pub fn gpolyl(&self) -> &GPOLYL {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const GPOLYL) }
    }
    #[doc = "0x04 - CRC_GPOLYL register."]
    #[inline(always)]
    pub fn gpolyl_mut(&self) -> &mut GPOLYL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut GPOLYL) }
    }
    #[doc = "0x04 - CRC Polynomial register"]
    #[inline(always)]
    pub fn gpoly(&self) -> &GPOLY {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const GPOLY) }
    }
    #[doc = "0x04 - CRC Polynomial register"]
    #[inline(always)]
    pub fn gpoly_mut(&self) -> &mut GPOLY {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut GPOLY) }
    }
    #[doc = "0x05 - CRC_GPOLYLU register."]
    #[inline(always)]
    pub fn gpolylu(&self) -> &GPOLYLU {
        unsafe { &*(((self as *const Self) as *const u8).add(5usize) as *const GPOLYLU) }
    }
    #[doc = "0x05 - CRC_GPOLYLU register."]
    #[inline(always)]
    pub fn gpolylu_mut(&self) -> &mut GPOLYLU {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(5usize) as *mut GPOLYLU) }
    }
    #[doc = "0x06 - CRC_GPOLYHL register."]
    #[inline(always)]
    pub fn gpolyhl(&self) -> &GPOLYHL {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const GPOLYHL) }
    }
    #[doc = "0x06 - CRC_GPOLYHL register."]
    #[inline(always)]
    pub fn gpolyhl_mut(&self) -> &mut GPOLYHL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut GPOLYHL) }
    }
    #[doc = "0x06 - CRC_GPOLYH register."]
    #[inline(always)]
    pub fn gpolyh(&self) -> &GPOLYH {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const GPOLYH) }
    }
    #[doc = "0x06 - CRC_GPOLYH register."]
    #[inline(always)]
    pub fn gpolyh_mut(&self) -> &mut GPOLYH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut GPOLYH) }
    }
    #[doc = "0x07 - CRC_GPOLYHU register."]
    #[inline(always)]
    pub fn gpolyhu(&self) -> &GPOLYHU {
        unsafe { &*(((self as *const Self) as *const u8).add(7usize) as *const GPOLYHU) }
    }
    #[doc = "0x07 - CRC_GPOLYHU register."]
    #[inline(always)]
    pub fn gpolyhu_mut(&self) -> &mut GPOLYHU {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(7usize) as *mut GPOLYHU) }
    }
    #[doc = "0x08 - CRC Control register"]
    #[inline(always)]
    pub fn ctrl(&self) -> &CTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const CTRL) }
    }
    #[doc = "0x08 - CRC Control register"]
    #[inline(always)]
    pub fn ctrl_mut(&self) -> &mut CTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut CTRL) }
    }
    #[doc = "0x0b - CRC_CTRLHU register."]
    #[inline(always)]
    pub fn ctrlhu(&self) -> &CTRLHU {
        unsafe { &*(((self as *const Self) as *const u8).add(11usize) as *const CTRLHU) }
    }
    #[doc = "0x0b - CRC_CTRLHU register."]
    #[inline(always)]
    pub fn ctrlhu_mut(&self) -> &mut CTRLHU {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(11usize) as *mut CTRLHU) }
    }
}
#[doc = "CRC Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "CRC Data register"]
pub mod data;
#[doc = "CRC_DATAL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [datal](datal) module"]
pub type DATAL = crate::Reg<u16, _DATAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAL;
#[doc = "`read()` method returns [datal::R](datal::R) reader structure"]
impl crate::Readable for DATAL {}
#[doc = "`write(|w| ..)` method takes [datal::W](datal::W) writer structure"]
impl crate::Writable for DATAL {}
#[doc = "CRC_DATAL register."]
pub mod datal;
#[doc = "CRC_DATALL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [datall](datall) module"]
pub type DATALL = crate::Reg<u8, _DATALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATALL;
#[doc = "`read()` method returns [datall::R](datall::R) reader structure"]
impl crate::Readable for DATALL {}
#[doc = "`write(|w| ..)` method takes [datall::W](datall::W) writer structure"]
impl crate::Writable for DATALL {}
#[doc = "CRC_DATALL register."]
pub mod datall;
#[doc = "CRC_DATALU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [datalu](datalu) module"]
pub type DATALU = crate::Reg<u8, _DATALU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATALU;
#[doc = "`read()` method returns [datalu::R](datalu::R) reader structure"]
impl crate::Readable for DATALU {}
#[doc = "`write(|w| ..)` method takes [datalu::W](datalu::W) writer structure"]
impl crate::Writable for DATALU {}
#[doc = "CRC_DATALU register."]
pub mod datalu;
#[doc = "CRC_DATAH register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [datah](datah) module"]
pub type DATAH = crate::Reg<u16, _DATAH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAH;
#[doc = "`read()` method returns [datah::R](datah::R) reader structure"]
impl crate::Readable for DATAH {}
#[doc = "`write(|w| ..)` method takes [datah::W](datah::W) writer structure"]
impl crate::Writable for DATAH {}
#[doc = "CRC_DATAH register."]
pub mod datah;
#[doc = "CRC_DATAHL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [datahl](datahl) module"]
pub type DATAHL = crate::Reg<u8, _DATAHL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAHL;
#[doc = "`read()` method returns [datahl::R](datahl::R) reader structure"]
impl crate::Readable for DATAHL {}
#[doc = "`write(|w| ..)` method takes [datahl::W](datahl::W) writer structure"]
impl crate::Writable for DATAHL {}
#[doc = "CRC_DATAHL register."]
pub mod datahl;
#[doc = "CRC_DATAHU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [datahu](datahu) module"]
pub type DATAHU = crate::Reg<u8, _DATAHU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAHU;
#[doc = "`read()` method returns [datahu::R](datahu::R) reader structure"]
impl crate::Readable for DATAHU {}
#[doc = "`write(|w| ..)` method takes [datahu::W](datahu::W) writer structure"]
impl crate::Writable for DATAHU {}
#[doc = "CRC_DATAHU register."]
pub mod datahu;
#[doc = "CRC Polynomial register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpoly](gpoly) module"]
pub type GPOLY = crate::Reg<u32, _GPOLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLY;
#[doc = "`read()` method returns [gpoly::R](gpoly::R) reader structure"]
impl crate::Readable for GPOLY {}
#[doc = "`write(|w| ..)` method takes [gpoly::W](gpoly::W) writer structure"]
impl crate::Writable for GPOLY {}
#[doc = "CRC Polynomial register"]
pub mod gpoly;
#[doc = "CRC_GPOLYL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpolyl](gpolyl) module"]
pub type GPOLYL = crate::Reg<u16, _GPOLYL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLYL;
#[doc = "`read()` method returns [gpolyl::R](gpolyl::R) reader structure"]
impl crate::Readable for GPOLYL {}
#[doc = "`write(|w| ..)` method takes [gpolyl::W](gpolyl::W) writer structure"]
impl crate::Writable for GPOLYL {}
#[doc = "CRC_GPOLYL register."]
pub mod gpolyl;
#[doc = "CRC_GPOLYLL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpolyll](gpolyll) module"]
pub type GPOLYLL = crate::Reg<u8, _GPOLYLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLYLL;
#[doc = "`read()` method returns [gpolyll::R](gpolyll::R) reader structure"]
impl crate::Readable for GPOLYLL {}
#[doc = "`write(|w| ..)` method takes [gpolyll::W](gpolyll::W) writer structure"]
impl crate::Writable for GPOLYLL {}
#[doc = "CRC_GPOLYLL register."]
pub mod gpolyll;
#[doc = "CRC_GPOLYLU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpolylu](gpolylu) module"]
pub type GPOLYLU = crate::Reg<u8, _GPOLYLU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLYLU;
#[doc = "`read()` method returns [gpolylu::R](gpolylu::R) reader structure"]
impl crate::Readable for GPOLYLU {}
#[doc = "`write(|w| ..)` method takes [gpolylu::W](gpolylu::W) writer structure"]
impl crate::Writable for GPOLYLU {}
#[doc = "CRC_GPOLYLU register."]
pub mod gpolylu;
#[doc = "CRC_GPOLYH register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpolyh](gpolyh) module"]
pub type GPOLYH = crate::Reg<u16, _GPOLYH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLYH;
#[doc = "`read()` method returns [gpolyh::R](gpolyh::R) reader structure"]
impl crate::Readable for GPOLYH {}
#[doc = "`write(|w| ..)` method takes [gpolyh::W](gpolyh::W) writer structure"]
impl crate::Writable for GPOLYH {}
#[doc = "CRC_GPOLYH register."]
pub mod gpolyh;
#[doc = "CRC_GPOLYHL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpolyhl](gpolyhl) module"]
pub type GPOLYHL = crate::Reg<u8, _GPOLYHL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLYHL;
#[doc = "`read()` method returns [gpolyhl::R](gpolyhl::R) reader structure"]
impl crate::Readable for GPOLYHL {}
#[doc = "`write(|w| ..)` method takes [gpolyhl::W](gpolyhl::W) writer structure"]
impl crate::Writable for GPOLYHL {}
#[doc = "CRC_GPOLYHL register."]
pub mod gpolyhl;
#[doc = "CRC_GPOLYHU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpolyhu](gpolyhu) module"]
pub type GPOLYHU = crate::Reg<u8, _GPOLYHU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLYHU;
#[doc = "`read()` method returns [gpolyhu::R](gpolyhu::R) reader structure"]
impl crate::Readable for GPOLYHU {}
#[doc = "`write(|w| ..)` method takes [gpolyhu::W](gpolyhu::W) writer structure"]
impl crate::Writable for GPOLYHU {}
#[doc = "CRC_GPOLYHU register."]
pub mod gpolyhu;
#[doc = "CRC Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "CRC Control register"]
pub mod ctrl;
#[doc = "CRC_CTRLHU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlhu](ctrlhu) module"]
pub type CTRLHU = crate::Reg<u8, _CTRLHU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLHU;
#[doc = "`read()` method returns [ctrlhu::R](ctrlhu::R) reader structure"]
impl crate::Readable for CTRLHU {}
#[doc = "`write(|w| ..)` method takes [ctrlhu::W](ctrlhu::W) writer structure"]
impl crate::Writable for CTRLHU {}
#[doc = "CRC_CTRLHU register."]
pub mod ctrlhu;
