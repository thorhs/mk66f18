#[doc = "Reader of register CAU_STR_CA7"]
pub type R = crate::R<u32, super::CAU_STR_CA7>;
#[doc = "Reader of field `CA7`"]
pub type CA7_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CA7"]
    #[inline(always)]
    pub fn ca7(&self) -> CA7_R {
        CA7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
