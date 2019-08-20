#[doc = "Reader of register CAU_STR_CA4"]
pub type R = crate::R<u32, super::CAU_STR_CA4>;
#[doc = "Reader of field `CA4`"]
pub type CA4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CA4"]
    #[inline(always)]
    pub fn ca4(&self) -> CA4_R {
        CA4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
