#[doc = "Reader of register HWDEVICE"]
pub type R = crate::R<u32, super::HWDEVICE>;
#[doc = "Reader of field `DC`"]
pub type DC_R = crate::R<bool, bool>;
#[doc = "Reader of field `DEVEP`"]
pub type DEVEP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Device Capable"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:5 - Device endpoints."]
    #[inline(always)]
    pub fn devep(&self) -> DEVEP_R {
        DEVEP_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
