#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HCCPARAMS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ADCR {
    bits: bool,
}
impl ADCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PFLR {
    bits: bool,
}
impl PFLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `ASP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASPR {
    #[doc = "Park not supported."]
    _0,
    #[doc = "Park supported."]
    _1,
}
impl ASPR {
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
            ASPR::_0 => false,
            ASPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASPR {
        match value {
            false => ASPR::_0,
            true => ASPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ASPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ASPR::_1
    }
}
#[doc = "Possible values of the field `IST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISTR {
    #[doc = "The value of the least significant 3 bits indicates the number of microframes a host controller can hold a set of isochronous data structures (one or more) before flushing the state"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ISTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ISTR::_0 => 0,
            ISTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ISTR {
        match value {
            0 => ISTR::_0,
            i => ISTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ISTR::_0
    }
}
#[doc = "Possible values of the field `EECP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EECPR {
    #[doc = "No extended capabilities are implemented"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EECPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EECPR::_0 => 0,
            EECPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EECPR {
        match value {
            0 => EECPR::_0,
            i => EECPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EECPR::_0
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - 64-bit addressing capability."]
    #[inline]
    pub fn adc(&self) -> ADCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADCR { bits }
    }
    #[doc = "Bit 1 - Programmable Frame List flag"]
    #[inline]
    pub fn pfl(&self) -> PFLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFLR { bits }
    }
    #[doc = "Bit 2 - Asynchronous Schedule Park capability"]
    #[inline]
    pub fn asp(&self) -> ASPR {
        ASPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:7 - Isochronous Scheduling Threshold"]
    #[inline]
    pub fn ist(&self) -> ISTR {
        ISTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - EHCI Extended Capabilities Pointer"]
    #[inline]
    pub fn eecp(&self) -> EECPR {
        EECPR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
