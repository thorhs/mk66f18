#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::XACC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `XA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XAR {
    #[doc = "Associated segment is accessible in execute mode only (as an instruction fetch)"]
    _0,
    #[doc = "Associated segment is accessible as data or in execute mode"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl XAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            XAR::_0 => 0,
            XAR::_1 => 1,
            XAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> XAR {
        match value {
            0 => XAR::_0,
            1 => XAR::_1,
            i => XAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == XAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == XAR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:7 - Execute-only access control"]
    #[inline]
    pub fn xa(&self) -> XAR {
        XAR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
