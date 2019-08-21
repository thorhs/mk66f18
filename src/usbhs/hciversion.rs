#[doc = "Reader of register HCIVERSION"]
pub type R = crate::R<u32, super::HCIVERSION>;
#[doc = "Reader of field `CAPLENGTH`"]
pub type CAPLENGTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `HCIVERSION`"]
pub type HCIVERSION_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Capability registers length"]
    #[inline(always)]
    pub fn caplength(&self) -> CAPLENGTH_R {
        CAPLENGTH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - EHCI revision number"]
    #[inline(always)]
    pub fn hciversion(&self) -> HCIVERSION_R {
        HCIVERSION_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
