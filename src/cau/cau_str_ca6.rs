#[doc = "Reader of register CAU_STR_CA6"]
pub type R = crate::R<u32, super::CAU_STR_CA6>;
#[doc = "Reader of field `CA6`"]
pub type CA6_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CA6"]
    #[inline(always)]
    pub fn ca6(&self) -> CA6_R {
        CA6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
