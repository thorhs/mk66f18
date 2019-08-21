#[doc = "Reader of register EPSR"]
pub type R = crate::R<u32, super::EPSR>;
#[doc = "Reader of field `ERBR`"]
pub type ERBR_R = crate::R<u8, u8>;
#[doc = "Reader of field `ETBR`"]
pub type ETBR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Endpoint Receive Buffer Ready"]
    #[inline(always)]
    pub fn erbr(&self) -> ERBR_R {
        ERBR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Endpoint Transmit Buffer Ready"]
    #[inline(always)]
    pub fn etbr(&self) -> ETBR_R {
        ETBR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
