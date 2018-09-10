#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
impl super::MF5 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `MWUF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF0R {
    #[doc = "Module 0 input was not a wakeup source"]
    _0,
    #[doc = "Module 0 input was a wakeup source"]
    _1,
}
impl MWUF0R {
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
            MWUF0R::_0 => false,
            MWUF0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MWUF0R {
        match value {
            false => MWUF0R::_0,
            true => MWUF0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MWUF0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MWUF0R::_1
    }
}
#[doc = "Possible values of the field `MWUF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF1R {
    #[doc = "Module 1 input was not a wakeup source"]
    _0,
    #[doc = "Module 1 input was a wakeup source"]
    _1,
}
impl MWUF1R {
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
            MWUF1R::_0 => false,
            MWUF1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MWUF1R {
        match value {
            false => MWUF1R::_0,
            true => MWUF1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MWUF1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MWUF1R::_1
    }
}
#[doc = "Possible values of the field `MWUF2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF2R {
    #[doc = "Module 2 input was not a wakeup source"]
    _0,
    #[doc = "Module 2 input was a wakeup source"]
    _1,
}
impl MWUF2R {
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
            MWUF2R::_0 => false,
            MWUF2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MWUF2R {
        match value {
            false => MWUF2R::_0,
            true => MWUF2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MWUF2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MWUF2R::_1
    }
}
#[doc = "Possible values of the field `MWUF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF3R {
    #[doc = "Module 3 input was not a wakeup source"]
    _0,
    #[doc = "Module 3 input was a wakeup source"]
    _1,
}
impl MWUF3R {
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
            MWUF3R::_0 => false,
            MWUF3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MWUF3R {
        match value {
            false => MWUF3R::_0,
            true => MWUF3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MWUF3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MWUF3R::_1
    }
}
#[doc = "Possible values of the field `MWUF4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF4R {
    #[doc = "Module 4 input was not a wakeup source"]
    _0,
    #[doc = "Module 4 input was a wakeup source"]
    _1,
}
impl MWUF4R {
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
            MWUF4R::_0 => false,
            MWUF4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MWUF4R {
        match value {
            false => MWUF4R::_0,
            true => MWUF4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MWUF4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MWUF4R::_1
    }
}
#[doc = "Possible values of the field `MWUF5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF5R {
    #[doc = "Module 5 input was not a wakeup source"]
    _0,
    #[doc = "Module 5 input was a wakeup source"]
    _1,
}
impl MWUF5R {
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
            MWUF5R::_0 => false,
            MWUF5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MWUF5R {
        match value {
            false => MWUF5R::_0,
            true => MWUF5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MWUF5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MWUF5R::_1
    }
}
#[doc = "Possible values of the field `MWUF6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF6R {
    #[doc = "Module 6 input was not a wakeup source"]
    _0,
    #[doc = "Module 6 input was a wakeup source"]
    _1,
}
impl MWUF6R {
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
            MWUF6R::_0 => false,
            MWUF6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MWUF6R {
        match value {
            false => MWUF6R::_0,
            true => MWUF6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MWUF6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MWUF6R::_1
    }
}
#[doc = "Possible values of the field `MWUF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MWUF7R {
    #[doc = "Module 7 input was not a wakeup source"]
    _0,
    #[doc = "Module 7 input was a wakeup source"]
    _1,
}
impl MWUF7R {
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
            MWUF7R::_0 => false,
            MWUF7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MWUF7R {
        match value {
            false => MWUF7R::_0,
            true => MWUF7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MWUF7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MWUF7R::_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Wakeup flag For module 0"]
    #[inline]
    pub fn mwuf0(&self) -> MWUF0R {
        MWUF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Wakeup flag For module 1"]
    #[inline]
    pub fn mwuf1(&self) -> MWUF1R {
        MWUF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Wakeup flag For module 2"]
    #[inline]
    pub fn mwuf2(&self) -> MWUF2R {
        MWUF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Wakeup flag For module 3"]
    #[inline]
    pub fn mwuf3(&self) -> MWUF3R {
        MWUF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Wakeup flag For module 4"]
    #[inline]
    pub fn mwuf4(&self) -> MWUF4R {
        MWUF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Wakeup flag For module 5"]
    #[inline]
    pub fn mwuf5(&self) -> MWUF5R {
        MWUF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Wakeup flag For module 6"]
    #[inline]
    pub fn mwuf6(&self) -> MWUF6R {
        MWUF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Wakeup flag For module 7"]
    #[inline]
    pub fn mwuf7(&self) -> MWUF7R {
        MWUF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
