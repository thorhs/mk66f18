#[doc = "Reader of register STAT"]
pub type R = crate::R<u8, super::STAT>;
#[doc = "Reader of field `ODD`"]
pub type ODD_R = crate::R<bool, bool>;
#[doc = "Transmit Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_A {
    #[doc = "0: The most recent transaction was a receive operation."]
    _0,
    #[doc = "1: The most recent transaction was a transmit operation."]
    _1,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        match variant {
            TX_A::_0 => false,
            TX_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<bool, TX_A>;
impl TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            false => TX_A::_0,
            true => TX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TX_A::_1
    }
}
#[doc = "Reader of field `ENDP`"]
pub type ENDP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 2 - This bit is set if the last buffer descriptor updated was in the odd bank of the BDT."]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Indicator"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - This four-bit field encodes the endpoint address that received or transmitted the previous token"]
    #[inline(always)]
    pub fn endp(&self) -> ENDP_R {
        ENDP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
