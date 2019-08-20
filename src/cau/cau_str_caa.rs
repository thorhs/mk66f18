#[doc = "Reader of register CAU_STR_CAA"]
pub type R = crate::R<u32, super::CAU_STR_CAA>;
#[doc = "Reader of field `ACC`"]
pub type ACC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ACC"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
