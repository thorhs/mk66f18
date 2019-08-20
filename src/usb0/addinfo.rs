#[doc = "Reader of register ADDINFO"]
pub type R = crate::R<u8, super::ADDINFO>;
#[doc = "Reader of field `IEHOST`"]
pub type IEHOST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - This bit is set if host mode is enabled."]
    #[inline(always)]
    pub fn iehost(&self) -> IEHOST_R {
        IEHOST_R::new((self.bits & 0x01) != 0)
    }
}
