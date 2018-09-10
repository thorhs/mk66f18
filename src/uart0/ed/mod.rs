#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::ED {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PARITYE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYER {
    #[doc = "The dataword was received without a parity error."]
    _0,
    #[doc = "The dataword was received with a parity error."]
    _1,
}
impl PARITYER {
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
            PARITYER::_0 => false,
            PARITYER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PARITYER {
        match value {
            false => PARITYER::_0,
            true => PARITYER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PARITYER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PARITYER::_1
    }
}
#[doc = "Possible values of the field `NOISY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOISYR {
    #[doc = "The dataword was received without noise."]
    _0,
    #[doc = "The data was received with noise."]
    _1,
}
impl NOISYR {
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
            NOISYR::_0 => false,
            NOISYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOISYR {
        match value {
            false => NOISYR::_0,
            true => NOISYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NOISYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NOISYR::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 6 - The current received dataword contained in D and C3\\[R8\\] was received with a parity error."]
    #[inline]
    pub fn paritye(&self) -> PARITYER {
        PARITYER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - The current received dataword contained in D and C3\\[R8\\] was received with noise."]
    #[inline]
    pub fn noisy(&self) -> NOISYR {
        NOISYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
