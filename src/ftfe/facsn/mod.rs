#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::FACSN {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `NUMSG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUMSGR {
    #[doc = "Program flash memory is divided into 48 segments (768 Kbytes, 1.5 Mbytes)"]
    _110000,
    #[doc = "Program flash memory is divided into 64 segments (512 Kbytes, 1 Mbyte, 2 Mbytes)"]
    _1000000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NUMSGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NUMSGR::_110000 => 48,
            NUMSGR::_1000000 => 64,
            NUMSGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NUMSGR {
        match value {
            48 => NUMSGR::_110000,
            64 => NUMSGR::_1000000,
            i => NUMSGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_110000`"]
    #[inline]
    pub fn is_110000(&self) -> bool {
        *self == NUMSGR::_110000
    }
    #[doc = "Checks if the value of the field is `_1000000`"]
    #[inline]
    pub fn is_1000000(&self) -> bool {
        *self == NUMSGR::_1000000
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:7 - Number of Segments Indicator"]
    #[inline]
    pub fn numsg(&self) -> NUMSGR {
        NUMSGR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
