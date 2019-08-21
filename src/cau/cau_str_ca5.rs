#[doc = "Reader of register CAU_STR_CA5"]
pub type R = crate::R<u32, super::CAU_STR_CA5>;
#[doc = "Reader of field `CA5`"]
pub type CA5_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CA5"]
    #[inline(always)]
    pub fn ca5(&self) -> CA5_R {
        CA5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
