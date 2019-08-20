#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Baud Rate Registers: High"]
    pub bdh: BDH,
    #[doc = "0x01 - UART Baud Rate Registers: Low"]
    pub bdl: BDL,
    #[doc = "0x02 - UART Control Register 1"]
    pub c1: C1,
    #[doc = "0x03 - UART Control Register 2"]
    pub c2: C2,
    #[doc = "0x04 - UART Status Register 1"]
    pub s1: S1,
    #[doc = "0x05 - UART Status Register 2"]
    pub s2: S2,
    #[doc = "0x06 - UART Control Register 3"]
    pub c3: C3,
    #[doc = "0x07 - UART Data Register"]
    pub d: D,
    #[doc = "0x08 - UART Match Address Registers 1"]
    pub ma1: MA1,
    #[doc = "0x09 - UART Match Address Registers 2"]
    pub ma2: MA2,
    #[doc = "0x0a - UART Control Register 4"]
    pub c4: C4,
    #[doc = "0x0b - UART Control Register 5"]
    pub c5: C5,
    #[doc = "0x0c - UART Extended Data Register"]
    pub ed: ED,
    #[doc = "0x0d - UART Modem Register"]
    pub modem: MODEM,
    #[doc = "0x0e - UART Infrared Register"]
    pub ir: IR,
    _reserved15: [u8; 1usize],
    #[doc = "0x10 - UART FIFO Parameters"]
    pub pfifo: PFIFO,
    #[doc = "0x11 - UART FIFO Control Register"]
    pub cfifo: CFIFO,
    #[doc = "0x12 - UART FIFO Status Register"]
    pub sfifo: SFIFO,
    #[doc = "0x13 - UART FIFO Transmit Watermark"]
    pub twfifo: TWFIFO,
    #[doc = "0x14 - UART FIFO Transmit Count"]
    pub tcfifo: TCFIFO,
    #[doc = "0x15 - UART FIFO Receive Watermark"]
    pub rwfifo: RWFIFO,
    #[doc = "0x16 - UART FIFO Receive Count"]
    pub rcfifo: RCFIFO,
    _reserved22: [u8; 1usize],
    #[doc = "0x18 - UART 7816 Control Register"]
    pub c7816: C7816,
    #[doc = "0x19 - UART 7816 Interrupt Enable Register"]
    pub ie7816: IE7816,
    #[doc = "0x1a - UART 7816 Interrupt Status Register"]
    pub is7816: IS7816,
    #[doc = "0x1b - UART 7816 Wait Parameter Register"]
    pub wp7816: WP7816,
    #[doc = "0x1c - UART 7816 Wait N Register"]
    pub wn7816: WN7816,
    #[doc = "0x1d - UART 7816 Wait FD Register"]
    pub wf7816: WF7816,
    #[doc = "0x1e - UART 7816 Error Threshold Register"]
    pub et7816: ET7816,
    #[doc = "0x1f - UART 7816 Transmit Length Register"]
    pub tl7816: TL7816,
    _reserved30: [u8; 26usize],
    #[doc = "0x3a - UART 7816 ATR Duration Timer Register A"]
    pub ap7816a_t0: AP7816A_T0,
    #[doc = "0x3b - UART 7816 ATR Duration Timer Register B"]
    pub ap7816b_t0: AP7816B_T0,
    _reserved_32_wp7816a_t: [u8; 1usize],
    _reserved_33_wp7816b_t: [u8; 1usize],
    #[doc = "0x3e - UART 7816 Wait and Guard Parameter Register"]
    pub wgp7816_t1: WGP7816_T1,
    #[doc = "0x3f - UART 7816 Wait Parameter Register C"]
    pub wp7816c_t1: WP7816C_T1,
}
impl RegisterBlock {
    #[doc = "0x3c - UART 7816 Wait Parameter Register A"]
    #[inline(always)]
    pub fn wp7816a_t1(&self) -> &WP7816A_T1 {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const WP7816A_T1) }
    }
    #[doc = "0x3c - UART 7816 Wait Parameter Register A"]
    #[inline(always)]
    pub fn wp7816a_t1_mut(&self) -> &mut WP7816A_T1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut WP7816A_T1) }
    }
    #[doc = "0x3c - UART 7816 Wait Parameter Register A"]
    #[inline(always)]
    pub fn wp7816a_t0(&self) -> &WP7816A_T0 {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const WP7816A_T0) }
    }
    #[doc = "0x3c - UART 7816 Wait Parameter Register A"]
    #[inline(always)]
    pub fn wp7816a_t0_mut(&self) -> &mut WP7816A_T0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut WP7816A_T0) }
    }
    #[doc = "0x3d - UART 7816 Wait Parameter Register B"]
    #[inline(always)]
    pub fn wp7816b_t1(&self) -> &WP7816B_T1 {
        unsafe { &*(((self as *const Self) as *const u8).add(61usize) as *const WP7816B_T1) }
    }
    #[doc = "0x3d - UART 7816 Wait Parameter Register B"]
    #[inline(always)]
    pub fn wp7816b_t1_mut(&self) -> &mut WP7816B_T1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(61usize) as *mut WP7816B_T1) }
    }
    #[doc = "0x3d - UART 7816 Wait Parameter Register B"]
    #[inline(always)]
    pub fn wp7816b_t0(&self) -> &WP7816B_T0 {
        unsafe { &*(((self as *const Self) as *const u8).add(61usize) as *const WP7816B_T0) }
    }
    #[doc = "0x3d - UART 7816 Wait Parameter Register B"]
    #[inline(always)]
    pub fn wp7816b_t0_mut(&self) -> &mut WP7816B_T0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(61usize) as *mut WP7816B_T0) }
    }
}
#[doc = "UART Baud Rate Registers: High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bdh](bdh) module"]
pub type BDH = crate::Reg<u8, _BDH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDH;
#[doc = "`read()` method returns [bdh::R](bdh::R) reader structure"]
impl crate::Readable for BDH {}
#[doc = "`write(|w| ..)` method takes [bdh::W](bdh::W) writer structure"]
impl crate::Writable for BDH {}
#[doc = "UART Baud Rate Registers: High"]
pub mod bdh;
#[doc = "UART Baud Rate Registers: Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bdl](bdl) module"]
pub type BDL = crate::Reg<u8, _BDL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDL;
#[doc = "`read()` method returns [bdl::R](bdl::R) reader structure"]
impl crate::Readable for BDL {}
#[doc = "`write(|w| ..)` method takes [bdl::W](bdl::W) writer structure"]
impl crate::Writable for BDL {}
#[doc = "UART Baud Rate Registers: Low"]
pub mod bdl;
#[doc = "UART Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c1](c1) module"]
pub type C1 = crate::Reg<u8, _C1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1;
#[doc = "`read()` method returns [c1::R](c1::R) reader structure"]
impl crate::Readable for C1 {}
#[doc = "`write(|w| ..)` method takes [c1::W](c1::W) writer structure"]
impl crate::Writable for C1 {}
#[doc = "UART Control Register 1"]
pub mod c1;
#[doc = "UART Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c2](c2) module"]
pub type C2 = crate::Reg<u8, _C2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2;
#[doc = "`read()` method returns [c2::R](c2::R) reader structure"]
impl crate::Readable for C2 {}
#[doc = "`write(|w| ..)` method takes [c2::W](c2::W) writer structure"]
impl crate::Writable for C2 {}
#[doc = "UART Control Register 2"]
pub mod c2;
#[doc = "UART Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [s1](s1) module"]
pub type S1 = crate::Reg<u8, _S1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S1;
#[doc = "`read()` method returns [s1::R](s1::R) reader structure"]
impl crate::Readable for S1 {}
#[doc = "UART Status Register 1"]
pub mod s1;
#[doc = "UART Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [s2](s2) module"]
pub type S2 = crate::Reg<u8, _S2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _S2;
#[doc = "`read()` method returns [s2::R](s2::R) reader structure"]
impl crate::Readable for S2 {}
#[doc = "`write(|w| ..)` method takes [s2::W](s2::W) writer structure"]
impl crate::Writable for S2 {}
#[doc = "UART Status Register 2"]
pub mod s2;
#[doc = "UART Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c3](c3) module"]
pub type C3 = crate::Reg<u8, _C3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3;
#[doc = "`read()` method returns [c3::R](c3::R) reader structure"]
impl crate::Readable for C3 {}
#[doc = "`write(|w| ..)` method takes [c3::W](c3::W) writer structure"]
impl crate::Writable for C3 {}
#[doc = "UART Control Register 3"]
pub mod c3;
#[doc = "UART Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [d](d) module"]
pub type D = crate::Reg<u8, _D>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _D;
#[doc = "`read()` method returns [d::R](d::R) reader structure"]
impl crate::Readable for D {}
#[doc = "`write(|w| ..)` method takes [d::W](d::W) writer structure"]
impl crate::Writable for D {}
#[doc = "UART Data Register"]
pub mod d;
#[doc = "UART Match Address Registers 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ma1](ma1) module"]
pub type MA1 = crate::Reg<u8, _MA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MA1;
#[doc = "`read()` method returns [ma1::R](ma1::R) reader structure"]
impl crate::Readable for MA1 {}
#[doc = "`write(|w| ..)` method takes [ma1::W](ma1::W) writer structure"]
impl crate::Writable for MA1 {}
#[doc = "UART Match Address Registers 1"]
pub mod ma1;
#[doc = "UART Match Address Registers 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ma2](ma2) module"]
pub type MA2 = crate::Reg<u8, _MA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MA2;
#[doc = "`read()` method returns [ma2::R](ma2::R) reader structure"]
impl crate::Readable for MA2 {}
#[doc = "`write(|w| ..)` method takes [ma2::W](ma2::W) writer structure"]
impl crate::Writable for MA2 {}
#[doc = "UART Match Address Registers 2"]
pub mod ma2;
#[doc = "UART Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c4](c4) module"]
pub type C4 = crate::Reg<u8, _C4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4;
#[doc = "`read()` method returns [c4::R](c4::R) reader structure"]
impl crate::Readable for C4 {}
#[doc = "`write(|w| ..)` method takes [c4::W](c4::W) writer structure"]
impl crate::Writable for C4 {}
#[doc = "UART Control Register 4"]
pub mod c4;
#[doc = "UART Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c5](c5) module"]
pub type C5 = crate::Reg<u8, _C5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5;
#[doc = "`read()` method returns [c5::R](c5::R) reader structure"]
impl crate::Readable for C5 {}
#[doc = "`write(|w| ..)` method takes [c5::W](c5::W) writer structure"]
impl crate::Writable for C5 {}
#[doc = "UART Control Register 5"]
pub mod c5;
#[doc = "UART Extended Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ed](ed) module"]
pub type ED = crate::Reg<u8, _ED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ED;
#[doc = "`read()` method returns [ed::R](ed::R) reader structure"]
impl crate::Readable for ED {}
#[doc = "UART Extended Data Register"]
pub mod ed;
#[doc = "UART Modem Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [modem](modem) module"]
pub type MODEM = crate::Reg<u8, _MODEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODEM;
#[doc = "`read()` method returns [modem::R](modem::R) reader structure"]
impl crate::Readable for MODEM {}
#[doc = "`write(|w| ..)` method takes [modem::W](modem::W) writer structure"]
impl crate::Writable for MODEM {}
#[doc = "UART Modem Register"]
pub mod modem;
#[doc = "UART Infrared Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ir](ir) module"]
pub type IR = crate::Reg<u8, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
#[doc = "`read()` method returns [ir::R](ir::R) reader structure"]
impl crate::Readable for IR {}
#[doc = "`write(|w| ..)` method takes [ir::W](ir::W) writer structure"]
impl crate::Writable for IR {}
#[doc = "UART Infrared Register"]
pub mod ir;
#[doc = "UART FIFO Parameters\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pfifo](pfifo) module"]
pub type PFIFO = crate::Reg<u8, _PFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFIFO;
#[doc = "`read()` method returns [pfifo::R](pfifo::R) reader structure"]
impl crate::Readable for PFIFO {}
#[doc = "`write(|w| ..)` method takes [pfifo::W](pfifo::W) writer structure"]
impl crate::Writable for PFIFO {}
#[doc = "UART FIFO Parameters"]
pub mod pfifo;
#[doc = "UART FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfifo](cfifo) module"]
pub type CFIFO = crate::Reg<u8, _CFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFIFO;
#[doc = "`read()` method returns [cfifo::R](cfifo::R) reader structure"]
impl crate::Readable for CFIFO {}
#[doc = "`write(|w| ..)` method takes [cfifo::W](cfifo::W) writer structure"]
impl crate::Writable for CFIFO {}
#[doc = "UART FIFO Control Register"]
pub mod cfifo;
#[doc = "UART FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sfifo](sfifo) module"]
pub type SFIFO = crate::Reg<u8, _SFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFIFO;
#[doc = "`read()` method returns [sfifo::R](sfifo::R) reader structure"]
impl crate::Readable for SFIFO {}
#[doc = "`write(|w| ..)` method takes [sfifo::W](sfifo::W) writer structure"]
impl crate::Writable for SFIFO {}
#[doc = "UART FIFO Status Register"]
pub mod sfifo;
#[doc = "UART FIFO Transmit Watermark\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [twfifo](twfifo) module"]
pub type TWFIFO = crate::Reg<u8, _TWFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWFIFO;
#[doc = "`read()` method returns [twfifo::R](twfifo::R) reader structure"]
impl crate::Readable for TWFIFO {}
#[doc = "`write(|w| ..)` method takes [twfifo::W](twfifo::W) writer structure"]
impl crate::Writable for TWFIFO {}
#[doc = "UART FIFO Transmit Watermark"]
pub mod twfifo;
#[doc = "UART FIFO Transmit Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcfifo](tcfifo) module"]
pub type TCFIFO = crate::Reg<u8, _TCFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCFIFO;
#[doc = "`read()` method returns [tcfifo::R](tcfifo::R) reader structure"]
impl crate::Readable for TCFIFO {}
#[doc = "UART FIFO Transmit Count"]
pub mod tcfifo;
#[doc = "UART FIFO Receive Watermark\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rwfifo](rwfifo) module"]
pub type RWFIFO = crate::Reg<u8, _RWFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RWFIFO;
#[doc = "`read()` method returns [rwfifo::R](rwfifo::R) reader structure"]
impl crate::Readable for RWFIFO {}
#[doc = "`write(|w| ..)` method takes [rwfifo::W](rwfifo::W) writer structure"]
impl crate::Writable for RWFIFO {}
#[doc = "UART FIFO Receive Watermark"]
pub mod rwfifo;
#[doc = "UART FIFO Receive Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcfifo](rcfifo) module"]
pub type RCFIFO = crate::Reg<u8, _RCFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCFIFO;
#[doc = "`read()` method returns [rcfifo::R](rcfifo::R) reader structure"]
impl crate::Readable for RCFIFO {}
#[doc = "UART FIFO Receive Count"]
pub mod rcfifo;
#[doc = "UART 7816 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [c7816](c7816) module"]
pub type C7816 = crate::Reg<u8, _C7816>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7816;
#[doc = "`read()` method returns [c7816::R](c7816::R) reader structure"]
impl crate::Readable for C7816 {}
#[doc = "`write(|w| ..)` method takes [c7816::W](c7816::W) writer structure"]
impl crate::Writable for C7816 {}
#[doc = "UART 7816 Control Register"]
pub mod c7816;
#[doc = "UART 7816 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ie7816](ie7816) module"]
pub type IE7816 = crate::Reg<u8, _IE7816>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE7816;
#[doc = "`read()` method returns [ie7816::R](ie7816::R) reader structure"]
impl crate::Readable for IE7816 {}
#[doc = "`write(|w| ..)` method takes [ie7816::W](ie7816::W) writer structure"]
impl crate::Writable for IE7816 {}
#[doc = "UART 7816 Interrupt Enable Register"]
pub mod ie7816;
#[doc = "UART 7816 Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [is7816](is7816) module"]
pub type IS7816 = crate::Reg<u8, _IS7816>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IS7816;
#[doc = "`read()` method returns [is7816::R](is7816::R) reader structure"]
impl crate::Readable for IS7816 {}
#[doc = "`write(|w| ..)` method takes [is7816::W](is7816::W) writer structure"]
impl crate::Writable for IS7816 {}
#[doc = "UART 7816 Interrupt Status Register"]
pub mod is7816;
#[doc = "UART 7816 Wait Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wp7816](wp7816) module"]
pub type WP7816 = crate::Reg<u8, _WP7816>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WP7816;
#[doc = "`read()` method returns [wp7816::R](wp7816::R) reader structure"]
impl crate::Readable for WP7816 {}
#[doc = "`write(|w| ..)` method takes [wp7816::W](wp7816::W) writer structure"]
impl crate::Writable for WP7816 {}
#[doc = "UART 7816 Wait Parameter Register"]
pub mod wp7816;
#[doc = "UART 7816 Wait N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wn7816](wn7816) module"]
pub type WN7816 = crate::Reg<u8, _WN7816>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WN7816;
#[doc = "`read()` method returns [wn7816::R](wn7816::R) reader structure"]
impl crate::Readable for WN7816 {}
#[doc = "`write(|w| ..)` method takes [wn7816::W](wn7816::W) writer structure"]
impl crate::Writable for WN7816 {}
#[doc = "UART 7816 Wait N Register"]
pub mod wn7816;
#[doc = "UART 7816 Wait FD Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wf7816](wf7816) module"]
pub type WF7816 = crate::Reg<u8, _WF7816>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WF7816;
#[doc = "`read()` method returns [wf7816::R](wf7816::R) reader structure"]
impl crate::Readable for WF7816 {}
#[doc = "`write(|w| ..)` method takes [wf7816::W](wf7816::W) writer structure"]
impl crate::Writable for WF7816 {}
#[doc = "UART 7816 Wait FD Register"]
pub mod wf7816;
#[doc = "UART 7816 Error Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [et7816](et7816) module"]
pub type ET7816 = crate::Reg<u8, _ET7816>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ET7816;
#[doc = "`read()` method returns [et7816::R](et7816::R) reader structure"]
impl crate::Readable for ET7816 {}
#[doc = "`write(|w| ..)` method takes [et7816::W](et7816::W) writer structure"]
impl crate::Writable for ET7816 {}
#[doc = "UART 7816 Error Threshold Register"]
pub mod et7816;
#[doc = "UART 7816 Transmit Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tl7816](tl7816) module"]
pub type TL7816 = crate::Reg<u8, _TL7816>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TL7816;
#[doc = "`read()` method returns [tl7816::R](tl7816::R) reader structure"]
impl crate::Readable for TL7816 {}
#[doc = "`write(|w| ..)` method takes [tl7816::W](tl7816::W) writer structure"]
impl crate::Writable for TL7816 {}
#[doc = "UART 7816 Transmit Length Register"]
pub mod tl7816;
#[doc = "UART 7816 ATR Duration Timer Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ap7816a_t0](ap7816a_t0) module"]
pub type AP7816A_T0 = crate::Reg<u8, _AP7816A_T0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AP7816A_T0;
#[doc = "`read()` method returns [ap7816a_t0::R](ap7816a_t0::R) reader structure"]
impl crate::Readable for AP7816A_T0 {}
#[doc = "`write(|w| ..)` method takes [ap7816a_t0::W](ap7816a_t0::W) writer structure"]
impl crate::Writable for AP7816A_T0 {}
#[doc = "UART 7816 ATR Duration Timer Register A"]
pub mod ap7816a_t0;
#[doc = "UART 7816 ATR Duration Timer Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ap7816b_t0](ap7816b_t0) module"]
pub type AP7816B_T0 = crate::Reg<u8, _AP7816B_T0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AP7816B_T0;
#[doc = "`read()` method returns [ap7816b_t0::R](ap7816b_t0::R) reader structure"]
impl crate::Readable for AP7816B_T0 {}
#[doc = "`write(|w| ..)` method takes [ap7816b_t0::W](ap7816b_t0::W) writer structure"]
impl crate::Writable for AP7816B_T0 {}
#[doc = "UART 7816 ATR Duration Timer Register B"]
pub mod ap7816b_t0;
#[doc = "UART 7816 Wait Parameter Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wp7816a_t0](wp7816a_t0) module"]
pub type WP7816A_T0 = crate::Reg<u8, _WP7816A_T0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WP7816A_T0;
#[doc = "`read()` method returns [wp7816a_t0::R](wp7816a_t0::R) reader structure"]
impl crate::Readable for WP7816A_T0 {}
#[doc = "`write(|w| ..)` method takes [wp7816a_t0::W](wp7816a_t0::W) writer structure"]
impl crate::Writable for WP7816A_T0 {}
#[doc = "UART 7816 Wait Parameter Register A"]
pub mod wp7816a_t0;
#[doc = "UART 7816 Wait Parameter Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wp7816a_t1](wp7816a_t1) module"]
pub type WP7816A_T1 = crate::Reg<u8, _WP7816A_T1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WP7816A_T1;
#[doc = "`read()` method returns [wp7816a_t1::R](wp7816a_t1::R) reader structure"]
impl crate::Readable for WP7816A_T1 {}
#[doc = "`write(|w| ..)` method takes [wp7816a_t1::W](wp7816a_t1::W) writer structure"]
impl crate::Writable for WP7816A_T1 {}
#[doc = "UART 7816 Wait Parameter Register A"]
pub mod wp7816a_t1;
#[doc = "UART 7816 Wait Parameter Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wp7816b_t0](wp7816b_t0) module"]
pub type WP7816B_T0 = crate::Reg<u8, _WP7816B_T0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WP7816B_T0;
#[doc = "`read()` method returns [wp7816b_t0::R](wp7816b_t0::R) reader structure"]
impl crate::Readable for WP7816B_T0 {}
#[doc = "`write(|w| ..)` method takes [wp7816b_t0::W](wp7816b_t0::W) writer structure"]
impl crate::Writable for WP7816B_T0 {}
#[doc = "UART 7816 Wait Parameter Register B"]
pub mod wp7816b_t0;
#[doc = "UART 7816 Wait Parameter Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wp7816b_t1](wp7816b_t1) module"]
pub type WP7816B_T1 = crate::Reg<u8, _WP7816B_T1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WP7816B_T1;
#[doc = "`read()` method returns [wp7816b_t1::R](wp7816b_t1::R) reader structure"]
impl crate::Readable for WP7816B_T1 {}
#[doc = "`write(|w| ..)` method takes [wp7816b_t1::W](wp7816b_t1::W) writer structure"]
impl crate::Writable for WP7816B_T1 {}
#[doc = "UART 7816 Wait Parameter Register B"]
pub mod wp7816b_t1;
#[doc = "UART 7816 Wait and Guard Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wgp7816_t1](wgp7816_t1) module"]
pub type WGP7816_T1 = crate::Reg<u8, _WGP7816_T1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WGP7816_T1;
#[doc = "`read()` method returns [wgp7816_t1::R](wgp7816_t1::R) reader structure"]
impl crate::Readable for WGP7816_T1 {}
#[doc = "`write(|w| ..)` method takes [wgp7816_t1::W](wgp7816_t1::W) writer structure"]
impl crate::Writable for WGP7816_T1 {}
#[doc = "UART 7816 Wait and Guard Parameter Register"]
pub mod wgp7816_t1;
#[doc = "UART 7816 Wait Parameter Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wp7816c_t1](wp7816c_t1) module"]
pub type WP7816C_T1 = crate::Reg<u8, _WP7816C_T1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WP7816C_T1;
#[doc = "`read()` method returns [wp7816c_t1::R](wp7816c_t1::R) reader structure"]
impl crate::Readable for WP7816C_T1 {}
#[doc = "`write(|w| ..)` method takes [wp7816c_t1::W](wp7816c_t1::W) writer structure"]
impl crate::Writable for WP7816C_T1 {}
#[doc = "UART 7816 Wait Parameter Register C"]
pub mod wp7816c_t1;
