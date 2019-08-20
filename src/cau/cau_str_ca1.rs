#[doc = "Reader of register CAU_STR_CA1"]
pub type R = crate::R<u32, super::CAU_STR_CA1>;
#[doc = "Reader of field `CA1`"]
pub type CA1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CA1"]
    #[inline(always)]
    pub fn ca1(&self) -> CA1_R {
        CA1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
