#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CAU_STR_CASR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `IC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICR {
    #[doc = "No illegal commands issued"]
    _0,
    #[doc = "Illegal command issued"]
    _1,
}
impl ICR {
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
            ICR::_0 => false,
            ICR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICR {
        match value {
            false => ICR::_0,
            true => ICR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ICR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ICR::_1
    }
}
#[doc = "Possible values of the field `DPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPER {
    #[doc = "No error detected"]
    _0,
    #[doc = "DES key parity error detected"]
    _1,
}
impl DPER {
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
            DPER::_0 => false,
            DPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPER {
        match value {
            false => DPER::_0,
            true => DPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DPER::_1
    }
}
#[doc = "Possible values of the field `VER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VERR {
    #[doc = "Initial CAU version"]
    _0001,
    #[doc = "Second version, added support for SHA-256 algorithm.(This is the value on this device)"]
    _0010,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VERR::_0001 => 1,
            VERR::_0010 => 2,
            VERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VERR {
        match value {
            1 => VERR::_0001,
            2 => VERR::_0010,
            i => VERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == VERR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == VERR::_0010
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn ic(&self) -> ICR {
        ICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn dpe(&self) -> DPER {
        DPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:31 - CAU version"]
    #[inline]
    pub fn ver(&self) -> VERR {
        VERR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
