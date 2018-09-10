#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ADMAES {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ADMAESR {
    bits: u8,
}
impl ADMAESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ADMALME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMALMER {
    #[doc = "No error."]
    _0,
    #[doc = "Error."]
    _1,
}
impl ADMALMER {
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
            ADMALMER::_0 => false,
            ADMALMER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADMALMER {
        match value {
            false => ADMALMER::_0,
            true => ADMALMER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADMALMER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADMALMER::_1
    }
}
#[doc = "Possible values of the field `ADMADCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMADCER {
    #[doc = "No error."]
    _0,
    #[doc = "Error."]
    _1,
}
impl ADMADCER {
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
            ADMADCER::_0 => false,
            ADMADCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADMADCER {
        match value {
            false => ADMADCER::_0,
            true => ADMADCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADMADCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADMADCER::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - ADMA Error State (When ADMA Error Is Occurred.)"]
    #[inline]
    pub fn admaes(&self) -> ADMAESR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADMAESR { bits }
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline]
    pub fn admalme(&self) -> ADMALMER {
        ADMALMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ADMA Descriptor Error"]
    #[inline]
    pub fn admadce(&self) -> ADMADCER {
        ADMADCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
