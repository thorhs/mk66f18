#[doc = "Reader of register TCFIFO"]
pub type R = crate::R<u8, super::TCFIFO>;
#[doc = "Reader of field `TXCOUNT`"]
pub type TXCOUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Counter"]
    #[inline(always)]
    pub fn txcount(&self) -> TXCOUNT_R {
        TXCOUNT_R::new((self.bits & 0xff) as u8)
    }
}
