#[doc = "Reader of register EAR%s"]
pub type R = crate::R<u32, super::EAR>;
#[doc = "Reader of field `EADDR`"]
pub type EADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Error Address"]
    #[inline(always)]
    pub fn eaddr(&self) -> EADDR_R {
        EADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
