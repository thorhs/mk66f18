#[doc = "Reader of register FDPROT"]
pub type R = crate::R<u8, super::FDPROT>;
#[doc = "Reader of field `DPROT`"]
pub type DPROT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - D-Flash Region Protect"]
    #[inline(always)]
    pub fn dprot(&self) -> DPROT_R {
        DPROT_R::new((self.bits & 0xff) as u8)
    }
}
