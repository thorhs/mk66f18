#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HWTXBUF {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TXBURSTR {
    bits: u8,
}
impl TXBURSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXADDR {
    bits: u8,
}
impl TXADDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXCHANADDR {
    bits: u8,
}
impl TXCHANADDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TXLC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLCR {
    #[doc = "Store device transmit contexts in the TX FIFO"]
    _0,
    #[doc = "Store device transmit contexts in a register file"]
    _1,
}
impl TXLCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TXLCR::_0 => false,
            TXLCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXLCR {
        match value {
            false => TXLCR::_0,
            true => TXLCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXLCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXLCR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Transmit Burst."]
    #[inline]
    pub fn txburst(&self) -> TXBURSTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXBURSTR { bits }
    }
    #[doc = "Bits 8:15 - Transmit Address."]
    #[inline]
    pub fn txadd(&self) -> TXADDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXADDR { bits }
    }
    #[doc = "Bits 16:23 - Transmit Channel Address"]
    #[inline]
    pub fn txchanadd(&self) -> TXCHANADDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXCHANADDR { bits }
    }
    #[doc = "Bit 31 - Transmit local Context Registers"]
    #[inline]
    pub fn txlc(&self) -> TXLCR {
        TXLCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
