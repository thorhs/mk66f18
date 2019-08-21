#[doc = "Reader of register HWHOST"]
pub type R = crate::R<u32, super::HWHOST>;
#[doc = "Reader of field `HC`"]
pub type HC_R = crate::R<bool, bool>;
#[doc = "Reader of field `NPORT`"]
pub type NPORT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TTASY`"]
pub type TTASY_R = crate::R<u8, u8>;
#[doc = "Reader of field `TTPER`"]
pub type TTPER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Host Capable"]
    #[inline(always)]
    pub fn hc(&self) -> HC_R {
        HC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Number of Ports"]
    #[inline(always)]
    pub fn nport(&self) -> NPORT_R {
        NPORT_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - Transaction translator contexts."]
    #[inline(always)]
    pub fn ttasy(&self) -> TTASY_R {
        TTASY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transaction translator periodic contexts."]
    #[inline(always)]
    pub fn ttper(&self) -> TTPER_R {
        TTPER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
