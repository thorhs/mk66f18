#[doc = "Reader of register ETBCNT"]
pub type R = crate::R<u32, super::ETBCNT>;
#[doc = "Reader of field `COUNTER`"]
pub type COUNTER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Byte Count Counter Value"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0x07ff) as u16)
    }
}
