#[doc = "Reader of register CAU_STR_CA3"]
pub type R = crate::R<u32, super::CAU_STR_CA3>;
#[doc = "Reader of field `CA3`"]
pub type CA3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CA3"]
    #[inline(always)]
    pub fn ca3(&self) -> CA3_R {
        CA3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
