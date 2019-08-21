#[doc = "Reader of register POPR"]
pub type R = crate::R<u32, super::POPR>;
#[doc = "Reader of field `RXDATA`"]
pub type RXDATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Received Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
