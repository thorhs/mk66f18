#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: MCR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Transfer Count Register"]
    pub tcr: TCR,
    _reserved_2_ctar: [u8; 8usize],
    _reserved3: [u8; 24usize],
    #[doc = "0x2c - Status Register"]
    pub sr: SR,
    #[doc = "0x30 - DMA/Interrupt Request Select and Enable Register"]
    pub rser: RSER,
    _reserved_5_pushr: [u8; 4usize],
    #[doc = "0x38 - POP RX FIFO Register"]
    pub popr: POPR,
    #[doc = "0x3c - Transmit FIFO Registers"]
    pub txfr: [TXFR; 4],
    _reserved8: [u8; 48usize],
    #[doc = "0x7c - Receive FIFO Registers"]
    pub rxfr: [RXFR; 4],
}
impl RegisterBlock {
    #[doc = "0x0c - Clock and Transfer Attributes Register (In Slave Mode)"]
    #[inline(always)]
    pub fn ctar_slave(&self) -> &CTAR_SLAVE {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const CTAR_SLAVE) }
    }
    #[doc = "0x0c - Clock and Transfer Attributes Register (In Slave Mode)"]
    #[inline(always)]
    pub fn ctar_slave_mut(&self) -> &mut CTAR_SLAVE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut CTAR_SLAVE) }
    }
    #[doc = "0x0c - Clock and Transfer Attributes Register (In Master Mode)"]
    #[inline(always)]
    pub fn ctar(&self) -> &[CTAR; 2] {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const [CTAR; 2]) }
    }
    #[doc = "0x0c - Clock and Transfer Attributes Register (In Master Mode)"]
    #[inline(always)]
    pub fn ctar_mut(&self) -> &mut [CTAR; 2] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut [CTAR; 2]) }
    }
    #[doc = "0x34 - PUSH TX FIFO Register In Slave Mode"]
    #[inline(always)]
    pub fn pushr_slave(&self) -> &PUSHR_SLAVE {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const PUSHR_SLAVE) }
    }
    #[doc = "0x34 - PUSH TX FIFO Register In Slave Mode"]
    #[inline(always)]
    pub fn pushr_slave_mut(&self) -> &mut PUSHR_SLAVE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut PUSHR_SLAVE) }
    }
    #[doc = "0x34 - PUSH TX FIFO Register In Master Mode"]
    #[inline(always)]
    pub fn pushr(&self) -> &PUSHR {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const PUSHR) }
    }
    #[doc = "0x34 - PUSH TX FIFO Register In Master Mode"]
    #[inline(always)]
    pub fn pushr_mut(&self) -> &mut PUSHR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(52usize) as *mut PUSHR) }
    }
}
#[doc = "Module Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "Transfer Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "Transfer Count Register"]
pub mod tcr;
#[doc = "Clock and Transfer Attributes Register (In Master Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctar](ctar) module"]
pub type CTAR = crate::Reg<u32, _CTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTAR;
#[doc = "`read()` method returns [ctar::R](ctar::R) reader structure"]
impl crate::Readable for CTAR {}
#[doc = "`write(|w| ..)` method takes [ctar::W](ctar::W) writer structure"]
impl crate::Writable for CTAR {}
#[doc = "Clock and Transfer Attributes Register (In Master Mode)"]
pub mod ctar;
#[doc = "Clock and Transfer Attributes Register (In Slave Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctar_slave](ctar_slave) module"]
pub type CTAR_SLAVE = crate::Reg<u32, _CTAR_SLAVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTAR_SLAVE;
#[doc = "`read()` method returns [ctar_slave::R](ctar_slave::R) reader structure"]
impl crate::Readable for CTAR_SLAVE {}
#[doc = "`write(|w| ..)` method takes [ctar_slave::W](ctar_slave::W) writer structure"]
impl crate::Writable for CTAR_SLAVE {}
#[doc = "Clock and Transfer Attributes Register (In Slave Mode)"]
pub mod ctar_slave;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "DMA/Interrupt Request Select and Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rser](rser) module"]
pub type RSER = crate::Reg<u32, _RSER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSER;
#[doc = "`read()` method returns [rser::R](rser::R) reader structure"]
impl crate::Readable for RSER {}
#[doc = "`write(|w| ..)` method takes [rser::W](rser::W) writer structure"]
impl crate::Writable for RSER {}
#[doc = "DMA/Interrupt Request Select and Enable Register"]
pub mod rser;
#[doc = "PUSH TX FIFO Register In Master Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pushr](pushr) module"]
pub type PUSHR = crate::Reg<u32, _PUSHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUSHR;
#[doc = "`read()` method returns [pushr::R](pushr::R) reader structure"]
impl crate::Readable for PUSHR {}
#[doc = "`write(|w| ..)` method takes [pushr::W](pushr::W) writer structure"]
impl crate::Writable for PUSHR {}
#[doc = "PUSH TX FIFO Register In Master Mode"]
pub mod pushr;
#[doc = "PUSH TX FIFO Register In Slave Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pushr_slave](pushr_slave) module"]
pub type PUSHR_SLAVE = crate::Reg<u32, _PUSHR_SLAVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUSHR_SLAVE;
#[doc = "`read()` method returns [pushr_slave::R](pushr_slave::R) reader structure"]
impl crate::Readable for PUSHR_SLAVE {}
#[doc = "`write(|w| ..)` method takes [pushr_slave::W](pushr_slave::W) writer structure"]
impl crate::Writable for PUSHR_SLAVE {}
#[doc = "PUSH TX FIFO Register In Slave Mode"]
pub mod pushr_slave;
#[doc = "POP RX FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [popr](popr) module"]
pub type POPR = crate::Reg<u32, _POPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POPR;
#[doc = "`read()` method returns [popr::R](popr::R) reader structure"]
impl crate::Readable for POPR {}
#[doc = "POP RX FIFO Register"]
pub mod popr;
#[doc = "Transmit FIFO Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txfr](txfr) module"]
pub type TXFR = crate::Reg<u32, _TXFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFR;
#[doc = "`read()` method returns [txfr::R](txfr::R) reader structure"]
impl crate::Readable for TXFR {}
#[doc = "Transmit FIFO Registers"]
pub mod txfr;
#[doc = "Receive FIFO Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxfr](rxfr) module"]
pub type RXFR = crate::Reg<u32, _RXFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFR;
#[doc = "`read()` method returns [rxfr::R](rxfr::R) reader structure"]
impl crate::Readable for RXFR {}
#[doc = "Receive FIFO Registers"]
pub mod rxfr;
