#[doc = "Reader of register FADR"]
pub type R = crate::R<u32, super::FADR>;
#[doc = "Reader of field `ADDRESS`"]
pub type ADDRESS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fault address"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
