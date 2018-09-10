#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PDIR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PDI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDIR {
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    _0,
    #[doc = "Pin logic level is logic 1."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl PDIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            PDIR::_0 => 0,
            PDIR::_1 => 1,
            PDIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> PDIR {
        match value {
            0 => PDIR::_0,
            1 => PDIR::_1,
            i => PDIR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDIR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Port Data Input"]
    #[inline]
    pub fn pdi(&self) -> PDIR {
        PDIR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
