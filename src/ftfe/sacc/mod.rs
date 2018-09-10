#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::SACC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAR {
    #[doc = "Associated segment is accessible in supervisor mode only"]
    _0,
    #[doc = "Associated segment is accessible in user or supervisor mode"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAR::_0 => 0,
            SAR::_1 => 1,
            SAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAR {
        match value {
            0 => SAR::_0,
            1 => SAR::_1,
            i => SAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SAR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:7 - Supervisor-only access control"]
    #[inline]
    pub fn sa(&self) -> SAR {
        SAR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
}
