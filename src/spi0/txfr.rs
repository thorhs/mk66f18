#[doc = "Reader of register TXFR%s"]
pub type R = crate::R<u32, super::TXFR>;
#[doc = "Reader of field `TXDATA`"]
pub type TXDATA_R = crate::R<u16, u16>;
#[doc = "Reader of field `TXCMD_TXDATA`"]
pub type TXCMD_TXDATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit Command or Transmit Data"]
    #[inline(always)]
    pub fn txcmd_txdata(&self) -> TXCMD_TXDATA_R {
        TXCMD_TXDATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
