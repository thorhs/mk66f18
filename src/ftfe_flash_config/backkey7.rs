#[doc = "Reader of register BACKKEY7"]
pub type R = crate::R<u8, super::BACKKEY7>;
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Backdoor Comparison Key."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xff) as u8)
    }
}
