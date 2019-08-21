#[doc = "Reader of register DCCPARAMS"]
pub type R = crate::R<u32, super::DCCPARAMS>;
#[doc = "Reader of field `DEN`"]
pub type DEN_R = crate::R<u8, u8>;
#[doc = "Reader of field `DC`"]
pub type DC_R = crate::R<bool, bool>;
#[doc = "Reader of field `HC`"]
pub type HC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:4 - Device Endpoint Number"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Device Capable"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Host Capable"]
    #[inline(always)]
    pub fn hc(&self) -> HC_R {
        HC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
