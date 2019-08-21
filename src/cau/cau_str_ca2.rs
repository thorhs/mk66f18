#[doc = "Reader of register CAU_STR_CA2"]
pub type R = crate::R<u32, super::CAU_STR_CA2>;
#[doc = "Reader of field `CA2`"]
pub type CA2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CA2"]
    #[inline(always)]
    pub fn ca2(&self) -> CA2_R {
        CA2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
