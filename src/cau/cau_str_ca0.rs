#[doc = "Reader of register CAU_STR_CA0"]
pub type R = crate::R<u32, super::CAU_STR_CA0>;
#[doc = "Reader of field `CA0`"]
pub type CA0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CA0"]
    #[inline(always)]
    pub fn ca0(&self) -> CA0_R {
        CA0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
