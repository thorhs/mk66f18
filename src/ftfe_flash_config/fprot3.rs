#[doc = "Reader of register FPROT3"]
pub type R = crate::R<u8, super::FPROT3>;
#[doc = "Reader of field `PROT`"]
pub type PROT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - P-Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new((self.bits & 0xff) as u8)
    }
}
