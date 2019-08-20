#[doc = "Reader of register TFR%s"]
pub type R = crate::R<u32, super::TFR>;
#[doc = "Reader of field `RFP`"]
pub type RFP_R = crate::R<u8, u8>;
#[doc = "Reader of field `WFP`"]
pub type WFP_R = crate::R<u8, u8>;
#[doc = "Write Channel Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCP_A {
    #[doc = "0: No effect."]
    _0,
    #[doc = "1: FIFO combine is enabled for FIFO writes and this FIFO will be written on the next FIFO write."]
    _1,
}
impl From<WCP_A> for bool {
    #[inline(always)]
    fn from(variant: WCP_A) -> Self {
        match variant {
            WCP_A::_0 => false,
            WCP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WCP`"]
pub type WCP_R = crate::R<bool, WCP_A>;
impl WCP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCP_A {
        match self.bits {
            false => WCP_A::_0,
            true => WCP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WCP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WCP_A::_1
    }
}
impl R {
    #[doc = "Bits 0:3 - Read FIFO Pointer"]
    #[inline(always)]
    pub fn rfp(&self) -> RFP_R {
        RFP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Write FIFO Pointer"]
    #[inline(always)]
    pub fn wfp(&self) -> WFP_R {
        WFP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Write Channel Pointer"]
    #[inline(always)]
    pub fn wcp(&self) -> WCP_R {
        WCP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
