#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::OBSERVE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DMPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMPDR {
    #[doc = "D- pulldown disabled."]
    _0,
    #[doc = "D- pulldown enabled."]
    _1,
}
impl DMPDR {
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
            DMPDR::_0 => false,
            DMPDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMPDR {
        match value {
            false => DMPDR::_0,
            true => DMPDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMPDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMPDR::_1
    }
}
#[doc = "Possible values of the field `DPPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPDR {
    #[doc = "D+ pulldown disabled."]
    _0,
    #[doc = "D+ pulldown enabled."]
    _1,
}
impl DPPDR {
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
            DPPDR::_0 => false,
            DPPDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPPDR {
        match value {
            false => DPPDR::_0,
            true => DPPDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DPPDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DPPDR::_1
    }
}
#[doc = "Possible values of the field `DPPU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPUR {
    #[doc = "D+ pullup disabled."]
    _0,
    #[doc = "D+ pullup enabled."]
    _1,
}
impl DPPUR {
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
            DPPUR::_0 => false,
            DPPUR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPPUR {
        match value {
            false => DPPUR::_0,
            true => DPPUR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DPPUR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DPPUR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 4 - Provides observability of the D- Pulldown enable at the USB transceiver."]
    #[inline]
    pub fn dmpd(&self) -> DMPDR {
        DMPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Provides observability of the D+ Pulldown enable at the USB transceiver."]
    #[inline]
    pub fn dppd(&self) -> DPPDR {
        DPPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Provides observability of the D+ Pullup enable at the USB transceiver."]
    #[inline]
    pub fn dppu(&self) -> DPPUR {
        DPPUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
