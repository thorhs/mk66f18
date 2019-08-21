#[doc = "Reader of register RCFIFO"]
pub type R = crate::R<u8, super::RCFIFO>;
#[doc = "Reader of field `RXCOUNT`"]
pub type RXCOUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive Counter"]
    #[inline(always)]
    pub fn rxcount(&self) -> RXCOUNT_R {
        RXCOUNT_R::new((self.bits & 0xff) as u8)
    }
}
