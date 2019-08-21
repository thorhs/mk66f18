#[doc = "Reader of register HWTXBUF"]
pub type R = crate::R<u32, super::HWTXBUF>;
#[doc = "Reader of field `TXBURST`"]
pub type TXBURST_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXADD`"]
pub type TXADD_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXCHANADD`"]
pub type TXCHANADD_R = crate::R<u8, u8>;
#[doc = "Transmit local Context Registers\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLC_A {
    #[doc = "0: Store device transmit contexts in the TX FIFO"]
    _0,
    #[doc = "1: Store device transmit contexts in a register file"]
    _1,
}
impl From<TXLC_A> for bool {
    #[inline(always)]
    fn from(variant: TXLC_A) -> Self {
        match variant {
            TXLC_A::_0 => false,
            TXLC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXLC`"]
pub type TXLC_R = crate::R<bool, TXLC_A>;
impl TXLC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXLC_A {
        match self.bits {
            false => TXLC_A::_0,
            true => TXLC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXLC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXLC_A::_1
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit Burst."]
    #[inline(always)]
    pub fn txburst(&self) -> TXBURST_R {
        TXBURST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit Address."]
    #[inline(always)]
    pub fn txadd(&self) -> TXADD_R {
        TXADD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Channel Address"]
    #[inline(always)]
    pub fn txchanadd(&self) -> TXCHANADD_R {
        TXCHANADD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Transmit local Context Registers"]
    #[inline(always)]
    pub fn txlc(&self) -> TXLC_R {
        TXLC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
