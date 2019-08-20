#[doc = "Reader of register FEPROT"]
pub type R = crate::R<u8, super::FEPROT>;
#[doc = "Reader of field `EPROT`"]
pub type EPROT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn eprot(&self) -> EPROT_R {
        EPROT_R::new((self.bits & 0xff) as u8)
    }
}
