#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MPRA {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `MPL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL6R {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL6R {
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
            MPL6R::_0 => false,
            MPL6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPL6R {
        match value {
            false => MPL6R::_0,
            true => MPL6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MPL6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MPL6R::_1
    }
}
#[doc = "Possible values of the field `MTW6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW6R {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW6R {
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
            MTW6R::_0 => false,
            MTW6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTW6R {
        match value {
            false => MTW6R::_0,
            true => MTW6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTW6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTW6R::_1
    }
}
#[doc = "Possible values of the field `MTR6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR6R {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR6R {
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
            MTR6R::_0 => false,
            MTR6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTR6R {
        match value {
            false => MTR6R::_0,
            true => MTR6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTR6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTR6R::_1
    }
}
#[doc = "Possible values of the field `MPL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL5R {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL5R {
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
            MPL5R::_0 => false,
            MPL5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPL5R {
        match value {
            false => MPL5R::_0,
            true => MPL5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MPL5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MPL5R::_1
    }
}
#[doc = "Possible values of the field `MTW5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW5R {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW5R {
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
            MTW5R::_0 => false,
            MTW5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTW5R {
        match value {
            false => MTW5R::_0,
            true => MTW5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTW5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTW5R::_1
    }
}
#[doc = "Possible values of the field `MTR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR5R {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR5R {
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
            MTR5R::_0 => false,
            MTR5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTR5R {
        match value {
            false => MTR5R::_0,
            true => MTR5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTR5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTR5R::_1
    }
}
#[doc = "Possible values of the field `MPL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL4R {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL4R {
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
            MPL4R::_0 => false,
            MPL4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPL4R {
        match value {
            false => MPL4R::_0,
            true => MPL4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MPL4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MPL4R::_1
    }
}
#[doc = "Possible values of the field `MTW4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW4R {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW4R {
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
            MTW4R::_0 => false,
            MTW4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTW4R {
        match value {
            false => MTW4R::_0,
            true => MTW4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTW4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTW4R::_1
    }
}
#[doc = "Possible values of the field `MTR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR4R {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR4R {
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
            MTR4R::_0 => false,
            MTR4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTR4R {
        match value {
            false => MTR4R::_0,
            true => MTR4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTR4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTR4R::_1
    }
}
#[doc = "Possible values of the field `MPL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL3R {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL3R {
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
            MPL3R::_0 => false,
            MPL3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPL3R {
        match value {
            false => MPL3R::_0,
            true => MPL3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MPL3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MPL3R::_1
    }
}
#[doc = "Possible values of the field `MTW3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW3R {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW3R {
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
            MTW3R::_0 => false,
            MTW3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTW3R {
        match value {
            false => MTW3R::_0,
            true => MTW3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTW3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTW3R::_1
    }
}
#[doc = "Possible values of the field `MTR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR3R {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR3R {
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
            MTR3R::_0 => false,
            MTR3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTR3R {
        match value {
            false => MTR3R::_0,
            true => MTR3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTR3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTR3R::_1
    }
}
#[doc = "Possible values of the field `MPL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL2R {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL2R {
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
            MPL2R::_0 => false,
            MPL2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPL2R {
        match value {
            false => MPL2R::_0,
            true => MPL2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MPL2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MPL2R::_1
    }
}
#[doc = "Possible values of the field `MTW2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW2R {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW2R {
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
            MTW2R::_0 => false,
            MTW2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTW2R {
        match value {
            false => MTW2R::_0,
            true => MTW2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTW2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTW2R::_1
    }
}
#[doc = "Possible values of the field `MTR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR2R {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR2R {
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
            MTR2R::_0 => false,
            MTR2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTR2R {
        match value {
            false => MTR2R::_0,
            true => MTR2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTR2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTR2R::_1
    }
}
#[doc = "Possible values of the field `MPL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL1R {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL1R {
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
            MPL1R::_0 => false,
            MPL1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPL1R {
        match value {
            false => MPL1R::_0,
            true => MPL1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MPL1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MPL1R::_1
    }
}
#[doc = "Possible values of the field `MTW1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW1R {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW1R {
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
            MTW1R::_0 => false,
            MTW1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTW1R {
        match value {
            false => MTW1R::_0,
            true => MTW1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTW1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTW1R::_1
    }
}
#[doc = "Possible values of the field `MTR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR1R {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR1R {
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
            MTR1R::_0 => false,
            MTR1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTR1R {
        match value {
            false => MTR1R::_0,
            true => MTR1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTR1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTR1R::_1
    }
}
#[doc = "Possible values of the field `MPL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL0R {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL0R {
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
            MPL0R::_0 => false,
            MPL0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPL0R {
        match value {
            false => MPL0R::_0,
            true => MPL0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MPL0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MPL0R::_1
    }
}
#[doc = "Possible values of the field `MTW0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW0R {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW0R {
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
            MTW0R::_0 => false,
            MTW0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTW0R {
        match value {
            false => MTW0R::_0,
            true => MTW0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTW0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTW0R::_1
    }
}
#[doc = "Possible values of the field `MTR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR0R {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR0R {
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
            MTR0R::_0 => false,
            MTR0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTR0R {
        match value {
            false => MTR0R::_0,
            true => MTR0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTR0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTR0R::_1
    }
}
#[doc = "Values that can be written to the field `MPL6`"]
pub enum MPL6W {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MPL6W::_0 => false,
            MPL6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPL6W<'a> {
    w: &'a mut W,
}
impl<'a> _MPL6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPL6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL6W::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL6W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTW6`"]
pub enum MTW6W {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTW6W::_0 => false,
            MTW6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTW6W<'a> {
    w: &'a mut W,
}
impl<'a> _MTW6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTW6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW6W::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW6W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTR6`"]
pub enum MTR6W {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTR6W::_0 => false,
            MTR6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTR6W<'a> {
    w: &'a mut W,
}
impl<'a> _MTR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTR6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR6W::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR6W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MPL5`"]
pub enum MPL5W {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MPL5W::_0 => false,
            MPL5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPL5W<'a> {
    w: &'a mut W,
}
impl<'a> _MPL5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPL5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL5W::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL5W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTW5`"]
pub enum MTW5W {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTW5W::_0 => false,
            MTW5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTW5W<'a> {
    w: &'a mut W,
}
impl<'a> _MTW5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTW5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW5W::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW5W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTR5`"]
pub enum MTR5W {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTR5W::_0 => false,
            MTR5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTR5W<'a> {
    w: &'a mut W,
}
impl<'a> _MTR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTR5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR5W::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR5W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MPL4`"]
pub enum MPL4W {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MPL4W::_0 => false,
            MPL4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPL4W<'a> {
    w: &'a mut W,
}
impl<'a> _MPL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPL4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL4W::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL4W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTW4`"]
pub enum MTW4W {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTW4W::_0 => false,
            MTW4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTW4W<'a> {
    w: &'a mut W,
}
impl<'a> _MTW4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTW4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW4W::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW4W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTR4`"]
pub enum MTR4W {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTR4W::_0 => false,
            MTR4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTR4W<'a> {
    w: &'a mut W,
}
impl<'a> _MTR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTR4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR4W::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR4W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MPL3`"]
pub enum MPL3W {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MPL3W::_0 => false,
            MPL3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPL3W<'a> {
    w: &'a mut W,
}
impl<'a> _MPL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPL3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL3W::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL3W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTW3`"]
pub enum MTW3W {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTW3W::_0 => false,
            MTW3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTW3W<'a> {
    w: &'a mut W,
}
impl<'a> _MTW3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTW3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW3W::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW3W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTR3`"]
pub enum MTR3W {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTR3W::_0 => false,
            MTR3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTR3W<'a> {
    w: &'a mut W,
}
impl<'a> _MTR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR3W::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR3W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MPL2`"]
pub enum MPL2W {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MPL2W::_0 => false,
            MPL2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPL2W<'a> {
    w: &'a mut W,
}
impl<'a> _MPL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPL2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL2W::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL2W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTW2`"]
pub enum MTW2W {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTW2W::_0 => false,
            MTW2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTW2W<'a> {
    w: &'a mut W,
}
impl<'a> _MTW2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTW2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW2W::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW2W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTR2`"]
pub enum MTR2W {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTR2W::_0 => false,
            MTR2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTR2W<'a> {
    w: &'a mut W,
}
impl<'a> _MTR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR2W::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR2W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MPL1`"]
pub enum MPL1W {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MPL1W::_0 => false,
            MPL1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPL1W<'a> {
    w: &'a mut W,
}
impl<'a> _MPL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL1W::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL1W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTW1`"]
pub enum MTW1W {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTW1W::_0 => false,
            MTW1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTW1W<'a> {
    w: &'a mut W,
}
impl<'a> _MTW1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTW1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW1W::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW1W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTR1`"]
pub enum MTR1W {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTR1W::_0 => false,
            MTR1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTR1W<'a> {
    w: &'a mut W,
}
impl<'a> _MTR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR1W::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR1W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MPL0`"]
pub enum MPL0W {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MPL0W::_0 => false,
            MPL0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPL0W<'a> {
    w: &'a mut W,
}
impl<'a> _MPL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPL0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL0W::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL0W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTW0`"]
pub enum MTW0W {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTW0W::_0 => false,
            MTW0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTW0W<'a> {
    w: &'a mut W,
}
impl<'a> _MTW0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTW0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW0W::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW0W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTR0`"]
pub enum MTR0W {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTR0W::_0 => false,
            MTR0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTR0W<'a> {
    w: &'a mut W,
}
impl<'a> _MTR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR0W::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR0W::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 4 - Master 6 Privilege Level"]
    #[inline]
    pub fn mpl6(&self) -> MPL6R {
        MPL6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Master 6 Trusted for Writes"]
    #[inline]
    pub fn mtw6(&self) -> MTW6R {
        MTW6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Master 6 Trusted for Read"]
    #[inline]
    pub fn mtr6(&self) -> MTR6R {
        MTR6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Master 5 Privilege Level"]
    #[inline]
    pub fn mpl5(&self) -> MPL5R {
        MPL5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Master 5 Trusted For Writes"]
    #[inline]
    pub fn mtw5(&self) -> MTW5R {
        MTW5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Master 5 Trusted For Read"]
    #[inline]
    pub fn mtr5(&self) -> MTR5R {
        MTR5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Master 4 Privilege Level"]
    #[inline]
    pub fn mpl4(&self) -> MPL4R {
        MPL4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Master 4 Trusted For Writes"]
    #[inline]
    pub fn mtw4(&self) -> MTW4R {
        MTW4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Master 4 Trusted For Read"]
    #[inline]
    pub fn mtr4(&self) -> MTR4R {
        MTR4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Master 3 Privilege Level"]
    #[inline]
    pub fn mpl3(&self) -> MPL3R {
        MPL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Master 3 Trusted For Writes"]
    #[inline]
    pub fn mtw3(&self) -> MTW3R {
        MTW3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Master 3 Trusted For Read"]
    #[inline]
    pub fn mtr3(&self) -> MTR3R {
        MTR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Master 2 Privilege Level"]
    #[inline]
    pub fn mpl2(&self) -> MPL2R {
        MPL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Master 2 Trusted For Writes"]
    #[inline]
    pub fn mtw2(&self) -> MTW2R {
        MTW2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Master 2 Trusted For Read"]
    #[inline]
    pub fn mtr2(&self) -> MTR2R {
        MTR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Master 1 Privilege Level"]
    #[inline]
    pub fn mpl1(&self) -> MPL1R {
        MPL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Master 1 Trusted for Writes"]
    #[inline]
    pub fn mtw1(&self) -> MTW1R {
        MTW1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Master 1 Trusted for Read"]
    #[inline]
    pub fn mtr1(&self) -> MTR1R {
        MTR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Master 0 Privilege Level"]
    #[inline]
    pub fn mpl0(&self) -> MPL0R {
        MPL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Master 0 Trusted For Writes"]
    #[inline]
    pub fn mtw0(&self) -> MTW0R {
        MTW0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Master 0 Trusted For Read"]
    #[inline]
    pub fn mtr0(&self) -> MTR0R {
        MTR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2003828736 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Master 6 Privilege Level"]
    #[inline]
    pub fn mpl6(&mut self) -> _MPL6W {
        _MPL6W { w: self }
    }
    #[doc = "Bit 5 - Master 6 Trusted for Writes"]
    #[inline]
    pub fn mtw6(&mut self) -> _MTW6W {
        _MTW6W { w: self }
    }
    #[doc = "Bit 6 - Master 6 Trusted for Read"]
    #[inline]
    pub fn mtr6(&mut self) -> _MTR6W {
        _MTR6W { w: self }
    }
    #[doc = "Bit 8 - Master 5 Privilege Level"]
    #[inline]
    pub fn mpl5(&mut self) -> _MPL5W {
        _MPL5W { w: self }
    }
    #[doc = "Bit 9 - Master 5 Trusted For Writes"]
    #[inline]
    pub fn mtw5(&mut self) -> _MTW5W {
        _MTW5W { w: self }
    }
    #[doc = "Bit 10 - Master 5 Trusted For Read"]
    #[inline]
    pub fn mtr5(&mut self) -> _MTR5W {
        _MTR5W { w: self }
    }
    #[doc = "Bit 12 - Master 4 Privilege Level"]
    #[inline]
    pub fn mpl4(&mut self) -> _MPL4W {
        _MPL4W { w: self }
    }
    #[doc = "Bit 13 - Master 4 Trusted For Writes"]
    #[inline]
    pub fn mtw4(&mut self) -> _MTW4W {
        _MTW4W { w: self }
    }
    #[doc = "Bit 14 - Master 4 Trusted For Read"]
    #[inline]
    pub fn mtr4(&mut self) -> _MTR4W {
        _MTR4W { w: self }
    }
    #[doc = "Bit 16 - Master 3 Privilege Level"]
    #[inline]
    pub fn mpl3(&mut self) -> _MPL3W {
        _MPL3W { w: self }
    }
    #[doc = "Bit 17 - Master 3 Trusted For Writes"]
    #[inline]
    pub fn mtw3(&mut self) -> _MTW3W {
        _MTW3W { w: self }
    }
    #[doc = "Bit 18 - Master 3 Trusted For Read"]
    #[inline]
    pub fn mtr3(&mut self) -> _MTR3W {
        _MTR3W { w: self }
    }
    #[doc = "Bit 20 - Master 2 Privilege Level"]
    #[inline]
    pub fn mpl2(&mut self) -> _MPL2W {
        _MPL2W { w: self }
    }
    #[doc = "Bit 21 - Master 2 Trusted For Writes"]
    #[inline]
    pub fn mtw2(&mut self) -> _MTW2W {
        _MTW2W { w: self }
    }
    #[doc = "Bit 22 - Master 2 Trusted For Read"]
    #[inline]
    pub fn mtr2(&mut self) -> _MTR2W {
        _MTR2W { w: self }
    }
    #[doc = "Bit 24 - Master 1 Privilege Level"]
    #[inline]
    pub fn mpl1(&mut self) -> _MPL1W {
        _MPL1W { w: self }
    }
    #[doc = "Bit 25 - Master 1 Trusted for Writes"]
    #[inline]
    pub fn mtw1(&mut self) -> _MTW1W {
        _MTW1W { w: self }
    }
    #[doc = "Bit 26 - Master 1 Trusted for Read"]
    #[inline]
    pub fn mtr1(&mut self) -> _MTR1W {
        _MTR1W { w: self }
    }
    #[doc = "Bit 28 - Master 0 Privilege Level"]
    #[inline]
    pub fn mpl0(&mut self) -> _MPL0W {
        _MPL0W { w: self }
    }
    #[doc = "Bit 29 - Master 0 Trusted For Writes"]
    #[inline]
    pub fn mtw0(&mut self) -> _MTW0W {
        _MTW0W { w: self }
    }
    #[doc = "Bit 30 - Master 0 Trusted For Read"]
    #[inline]
    pub fn mtr0(&mut self) -> _MTR0W {
        _MTR0W { w: self }
    }
}
