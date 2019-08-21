#[doc = "Reader of register TTCTRL"]
pub type R = crate::R<u32, super::TTCTRL>;
#[doc = "Reader of field `TTHA`"]
pub type TTHA_R = crate::R<u8, u8>;
#[doc = "Reader of field `Reerved`"]
pub type REERVED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 24:30 - TT Hub Address"]
    #[inline(always)]
    pub fn ttha(&self) -> TTHA_R {
        TTHA_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn reerved(&self) -> REERVED_R {
        REERVED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
