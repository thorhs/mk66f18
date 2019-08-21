#[doc = "Reader of register CAU_STR_CA8"]
pub type R = crate::R<u32, super::CAU_STR_CA8>;
#[doc = "Reader of field `CA8`"]
pub type CA8_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CA8"]
    #[inline(always)]
    pub fn ca8(&self) -> CA8_R {
        CA8_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
