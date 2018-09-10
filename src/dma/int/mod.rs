#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INT {
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
#[doc = "Possible values of the field `INT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT0R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT0R {
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
            INT0R::_0 => false,
            INT0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT0R {
        match value {
            false => INT0R::_0,
            true => INT0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT0R::_1
    }
}
#[doc = "Possible values of the field `INT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT1R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT1R {
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
            INT1R::_0 => false,
            INT1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT1R {
        match value {
            false => INT1R::_0,
            true => INT1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT1R::_1
    }
}
#[doc = "Possible values of the field `INT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT2R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT2R {
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
            INT2R::_0 => false,
            INT2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT2R {
        match value {
            false => INT2R::_0,
            true => INT2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT2R::_1
    }
}
#[doc = "Possible values of the field `INT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT3R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT3R {
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
            INT3R::_0 => false,
            INT3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT3R {
        match value {
            false => INT3R::_0,
            true => INT3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT3R::_1
    }
}
#[doc = "Possible values of the field `INT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT4R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT4R {
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
            INT4R::_0 => false,
            INT4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT4R {
        match value {
            false => INT4R::_0,
            true => INT4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT4R::_1
    }
}
#[doc = "Possible values of the field `INT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT5R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT5R {
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
            INT5R::_0 => false,
            INT5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT5R {
        match value {
            false => INT5R::_0,
            true => INT5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT5R::_1
    }
}
#[doc = "Possible values of the field `INT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT6R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT6R {
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
            INT6R::_0 => false,
            INT6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT6R {
        match value {
            false => INT6R::_0,
            true => INT6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT6R::_1
    }
}
#[doc = "Possible values of the field `INT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT7R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT7R {
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
            INT7R::_0 => false,
            INT7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT7R {
        match value {
            false => INT7R::_0,
            true => INT7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT7R::_1
    }
}
#[doc = "Possible values of the field `INT8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT8R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT8R {
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
            INT8R::_0 => false,
            INT8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT8R {
        match value {
            false => INT8R::_0,
            true => INT8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT8R::_1
    }
}
#[doc = "Possible values of the field `INT9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT9R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT9R {
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
            INT9R::_0 => false,
            INT9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT9R {
        match value {
            false => INT9R::_0,
            true => INT9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT9R::_1
    }
}
#[doc = "Possible values of the field `INT10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT10R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT10R {
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
            INT10R::_0 => false,
            INT10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT10R {
        match value {
            false => INT10R::_0,
            true => INT10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT10R::_1
    }
}
#[doc = "Possible values of the field `INT11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT11R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT11R {
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
            INT11R::_0 => false,
            INT11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT11R {
        match value {
            false => INT11R::_0,
            true => INT11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT11R::_1
    }
}
#[doc = "Possible values of the field `INT12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT12R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT12R {
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
            INT12R::_0 => false,
            INT12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT12R {
        match value {
            false => INT12R::_0,
            true => INT12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT12R::_1
    }
}
#[doc = "Possible values of the field `INT13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT13R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT13R {
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
            INT13R::_0 => false,
            INT13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT13R {
        match value {
            false => INT13R::_0,
            true => INT13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT13R::_1
    }
}
#[doc = "Possible values of the field `INT14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT14R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT14R {
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
            INT14R::_0 => false,
            INT14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT14R {
        match value {
            false => INT14R::_0,
            true => INT14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT14R::_1
    }
}
#[doc = "Possible values of the field `INT15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT15R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT15R {
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
            INT15R::_0 => false,
            INT15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT15R {
        match value {
            false => INT15R::_0,
            true => INT15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT15R::_1
    }
}
#[doc = "Possible values of the field `INT16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT16R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT16R {
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
            INT16R::_0 => false,
            INT16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT16R {
        match value {
            false => INT16R::_0,
            true => INT16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT16R::_1
    }
}
#[doc = "Possible values of the field `INT17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT17R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT17R {
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
            INT17R::_0 => false,
            INT17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT17R {
        match value {
            false => INT17R::_0,
            true => INT17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT17R::_1
    }
}
#[doc = "Possible values of the field `INT18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT18R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT18R {
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
            INT18R::_0 => false,
            INT18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT18R {
        match value {
            false => INT18R::_0,
            true => INT18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT18R::_1
    }
}
#[doc = "Possible values of the field `INT19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT19R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT19R {
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
            INT19R::_0 => false,
            INT19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT19R {
        match value {
            false => INT19R::_0,
            true => INT19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT19R::_1
    }
}
#[doc = "Possible values of the field `INT20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT20R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT20R {
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
            INT20R::_0 => false,
            INT20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT20R {
        match value {
            false => INT20R::_0,
            true => INT20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT20R::_1
    }
}
#[doc = "Possible values of the field `INT21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT21R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT21R {
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
            INT21R::_0 => false,
            INT21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT21R {
        match value {
            false => INT21R::_0,
            true => INT21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT21R::_1
    }
}
#[doc = "Possible values of the field `INT22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT22R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT22R {
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
            INT22R::_0 => false,
            INT22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT22R {
        match value {
            false => INT22R::_0,
            true => INT22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT22R::_1
    }
}
#[doc = "Possible values of the field `INT23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT23R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT23R {
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
            INT23R::_0 => false,
            INT23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT23R {
        match value {
            false => INT23R::_0,
            true => INT23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT23R::_1
    }
}
#[doc = "Possible values of the field `INT24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT24R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT24R {
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
            INT24R::_0 => false,
            INT24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT24R {
        match value {
            false => INT24R::_0,
            true => INT24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT24R::_1
    }
}
#[doc = "Possible values of the field `INT25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT25R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT25R {
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
            INT25R::_0 => false,
            INT25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT25R {
        match value {
            false => INT25R::_0,
            true => INT25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT25R::_1
    }
}
#[doc = "Possible values of the field `INT26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT26R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT26R {
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
            INT26R::_0 => false,
            INT26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT26R {
        match value {
            false => INT26R::_0,
            true => INT26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT26R::_1
    }
}
#[doc = "Possible values of the field `INT27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT27R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT27R {
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
            INT27R::_0 => false,
            INT27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT27R {
        match value {
            false => INT27R::_0,
            true => INT27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT27R::_1
    }
}
#[doc = "Possible values of the field `INT28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT28R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT28R {
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
            INT28R::_0 => false,
            INT28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT28R {
        match value {
            false => INT28R::_0,
            true => INT28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT28R::_1
    }
}
#[doc = "Possible values of the field `INT29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT29R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT29R {
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
            INT29R::_0 => false,
            INT29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT29R {
        match value {
            false => INT29R::_0,
            true => INT29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT29R::_1
    }
}
#[doc = "Possible values of the field `INT30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT30R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT30R {
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
            INT30R::_0 => false,
            INT30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT30R {
        match value {
            false => INT30R::_0,
            true => INT30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT30R::_1
    }
}
#[doc = "Possible values of the field `INT31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT31R {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT31R {
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
            INT31R::_0 => false,
            INT31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT31R {
        match value {
            false => INT31R::_0,
            true => INT31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INT31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INT31R::_1
    }
}
#[doc = "Values that can be written to the field `INT0`"]
pub enum INT0W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT0W::_0 => false,
            INT0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT0W<'a> {
    w: &'a mut W,
}
impl<'a> _INT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT0W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT0W::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT1`"]
pub enum INT1W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT1W::_0 => false,
            INT1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT1W<'a> {
    w: &'a mut W,
}
impl<'a> _INT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT1W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT1W::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT2`"]
pub enum INT2W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT2W::_0 => false,
            INT2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT2W<'a> {
    w: &'a mut W,
}
impl<'a> _INT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT2W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT2W::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT3`"]
pub enum INT3W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT3W::_0 => false,
            INT3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT3W<'a> {
    w: &'a mut W,
}
impl<'a> _INT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT3W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT3W::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT4`"]
pub enum INT4W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT4W::_0 => false,
            INT4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT4W<'a> {
    w: &'a mut W,
}
impl<'a> _INT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT4W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT4W::_1)
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
#[doc = "Values that can be written to the field `INT5`"]
pub enum INT5W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT5W::_0 => false,
            INT5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT5W<'a> {
    w: &'a mut W,
}
impl<'a> _INT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT5W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT5W::_1)
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
#[doc = "Values that can be written to the field `INT6`"]
pub enum INT6W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT6W::_0 => false,
            INT6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT6W<'a> {
    w: &'a mut W,
}
impl<'a> _INT6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT6W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT6W::_1)
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
#[doc = "Values that can be written to the field `INT7`"]
pub enum INT7W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT7W::_0 => false,
            INT7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT7W<'a> {
    w: &'a mut W,
}
impl<'a> _INT7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT7W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT7W::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT8`"]
pub enum INT8W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT8W::_0 => false,
            INT8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT8W<'a> {
    w: &'a mut W,
}
impl<'a> _INT8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT8W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT8W::_1)
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
#[doc = "Values that can be written to the field `INT9`"]
pub enum INT9W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT9W::_0 => false,
            INT9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT9W<'a> {
    w: &'a mut W,
}
impl<'a> _INT9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT9W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT9W::_1)
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
#[doc = "Values that can be written to the field `INT10`"]
pub enum INT10W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT10W::_0 => false,
            INT10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT10W<'a> {
    w: &'a mut W,
}
impl<'a> _INT10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT10W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT10W::_1)
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
#[doc = "Values that can be written to the field `INT11`"]
pub enum INT11W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT11W::_0 => false,
            INT11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT11W<'a> {
    w: &'a mut W,
}
impl<'a> _INT11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT11W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT11W::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT12`"]
pub enum INT12W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT12W::_0 => false,
            INT12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT12W<'a> {
    w: &'a mut W,
}
impl<'a> _INT12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT12W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT12W::_1)
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
#[doc = "Values that can be written to the field `INT13`"]
pub enum INT13W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT13W::_0 => false,
            INT13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT13W<'a> {
    w: &'a mut W,
}
impl<'a> _INT13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT13W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT13W::_1)
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
#[doc = "Values that can be written to the field `INT14`"]
pub enum INT14W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT14W::_0 => false,
            INT14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT14W<'a> {
    w: &'a mut W,
}
impl<'a> _INT14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT14W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT14W::_1)
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
#[doc = "Values that can be written to the field `INT15`"]
pub enum INT15W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT15W::_0 => false,
            INT15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT15W<'a> {
    w: &'a mut W,
}
impl<'a> _INT15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT15W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT15W::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT16`"]
pub enum INT16W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT16W::_0 => false,
            INT16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT16W<'a> {
    w: &'a mut W,
}
impl<'a> _INT16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT16W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT16W::_1)
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
#[doc = "Values that can be written to the field `INT17`"]
pub enum INT17W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT17W::_0 => false,
            INT17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT17W<'a> {
    w: &'a mut W,
}
impl<'a> _INT17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT17W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT17W::_1)
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
#[doc = "Values that can be written to the field `INT18`"]
pub enum INT18W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT18W::_0 => false,
            INT18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT18W<'a> {
    w: &'a mut W,
}
impl<'a> _INT18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT18W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT18W::_1)
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
#[doc = "Values that can be written to the field `INT19`"]
pub enum INT19W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT19W::_0 => false,
            INT19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT19W<'a> {
    w: &'a mut W,
}
impl<'a> _INT19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT19W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT19W::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT20`"]
pub enum INT20W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT20W::_0 => false,
            INT20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT20W<'a> {
    w: &'a mut W,
}
impl<'a> _INT20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT20W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT20W::_1)
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
#[doc = "Values that can be written to the field `INT21`"]
pub enum INT21W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT21W::_0 => false,
            INT21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT21W<'a> {
    w: &'a mut W,
}
impl<'a> _INT21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT21W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT21W::_1)
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
#[doc = "Values that can be written to the field `INT22`"]
pub enum INT22W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT22W::_0 => false,
            INT22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT22W<'a> {
    w: &'a mut W,
}
impl<'a> _INT22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT22W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT22W::_1)
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
#[doc = "Values that can be written to the field `INT23`"]
pub enum INT23W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT23W::_0 => false,
            INT23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT23W<'a> {
    w: &'a mut W,
}
impl<'a> _INT23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT23W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT23W::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT24`"]
pub enum INT24W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT24W::_0 => false,
            INT24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT24W<'a> {
    w: &'a mut W,
}
impl<'a> _INT24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT24W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT24W::_1)
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
#[doc = "Values that can be written to the field `INT25`"]
pub enum INT25W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT25W::_0 => false,
            INT25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT25W<'a> {
    w: &'a mut W,
}
impl<'a> _INT25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT25W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT25W::_1)
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
#[doc = "Values that can be written to the field `INT26`"]
pub enum INT26W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT26W::_0 => false,
            INT26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT26W<'a> {
    w: &'a mut W,
}
impl<'a> _INT26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT26W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT26W::_1)
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
#[doc = "Values that can be written to the field `INT27`"]
pub enum INT27W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT27W::_0 => false,
            INT27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT27W<'a> {
    w: &'a mut W,
}
impl<'a> _INT27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT27W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT27W::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INT28`"]
pub enum INT28W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT28W::_0 => false,
            INT28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT28W<'a> {
    w: &'a mut W,
}
impl<'a> _INT28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT28W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT28W::_1)
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
#[doc = "Values that can be written to the field `INT29`"]
pub enum INT29W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT29W::_0 => false,
            INT29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT29W<'a> {
    w: &'a mut W,
}
impl<'a> _INT29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT29W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT29W::_1)
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
#[doc = "Values that can be written to the field `INT30`"]
pub enum INT30W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT30W::_0 => false,
            INT30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT30W<'a> {
    w: &'a mut W,
}
impl<'a> _INT30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT30W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT30W::_1)
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
#[doc = "Values that can be written to the field `INT31`"]
pub enum INT31W {
    #[doc = "The interrupt request for corresponding channel is cleared"]
    _0,
    #[doc = "The interrupt request for corresponding channel is active"]
    _1,
}
impl INT31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INT31W::_0 => false,
            INT31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT31W<'a> {
    w: &'a mut W,
}
impl<'a> _INT31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT31W::_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT31W::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline]
    pub fn int0(&self) -> INT0R {
        INT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline]
    pub fn int1(&self) -> INT1R {
        INT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline]
    pub fn int2(&self) -> INT2R {
        INT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline]
    pub fn int3(&self) -> INT3R {
        INT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline]
    pub fn int4(&self) -> INT4R {
        INT4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline]
    pub fn int5(&self) -> INT5R {
        INT5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline]
    pub fn int6(&self) -> INT6R {
        INT6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline]
    pub fn int7(&self) -> INT7R {
        INT7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Interrupt Request 8"]
    #[inline]
    pub fn int8(&self) -> INT8R {
        INT8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Interrupt Request 9"]
    #[inline]
    pub fn int9(&self) -> INT9R {
        INT9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Interrupt Request 10"]
    #[inline]
    pub fn int10(&self) -> INT10R {
        INT10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Interrupt Request 11"]
    #[inline]
    pub fn int11(&self) -> INT11R {
        INT11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Interrupt Request 12"]
    #[inline]
    pub fn int12(&self) -> INT12R {
        INT12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Interrupt Request 13"]
    #[inline]
    pub fn int13(&self) -> INT13R {
        INT13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Interrupt Request 14"]
    #[inline]
    pub fn int14(&self) -> INT14R {
        INT14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Interrupt Request 15"]
    #[inline]
    pub fn int15(&self) -> INT15R {
        INT15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Interrupt Request 16"]
    #[inline]
    pub fn int16(&self) -> INT16R {
        INT16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Interrupt Request 17"]
    #[inline]
    pub fn int17(&self) -> INT17R {
        INT17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Interrupt Request 18"]
    #[inline]
    pub fn int18(&self) -> INT18R {
        INT18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Interrupt Request 19"]
    #[inline]
    pub fn int19(&self) -> INT19R {
        INT19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Interrupt Request 20"]
    #[inline]
    pub fn int20(&self) -> INT20R {
        INT20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Interrupt Request 21"]
    #[inline]
    pub fn int21(&self) -> INT21R {
        INT21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Interrupt Request 22"]
    #[inline]
    pub fn int22(&self) -> INT22R {
        INT22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Interrupt Request 23"]
    #[inline]
    pub fn int23(&self) -> INT23R {
        INT23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Interrupt Request 24"]
    #[inline]
    pub fn int24(&self) -> INT24R {
        INT24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Interrupt Request 25"]
    #[inline]
    pub fn int25(&self) -> INT25R {
        INT25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Interrupt Request 26"]
    #[inline]
    pub fn int26(&self) -> INT26R {
        INT26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Interrupt Request 27"]
    #[inline]
    pub fn int27(&self) -> INT27R {
        INT27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Interrupt Request 28"]
    #[inline]
    pub fn int28(&self) -> INT28R {
        INT28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Interrupt Request 29"]
    #[inline]
    pub fn int29(&self) -> INT29R {
        INT29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Interrupt Request 30"]
    #[inline]
    pub fn int30(&self) -> INT30R {
        INT30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Interrupt Request 31"]
    #[inline]
    pub fn int31(&self) -> INT31R {
        INT31R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline]
    pub fn int0(&mut self) -> _INT0W {
        _INT0W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline]
    pub fn int1(&mut self) -> _INT1W {
        _INT1W { w: self }
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline]
    pub fn int2(&mut self) -> _INT2W {
        _INT2W { w: self }
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline]
    pub fn int3(&mut self) -> _INT3W {
        _INT3W { w: self }
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline]
    pub fn int4(&mut self) -> _INT4W {
        _INT4W { w: self }
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline]
    pub fn int5(&mut self) -> _INT5W {
        _INT5W { w: self }
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline]
    pub fn int6(&mut self) -> _INT6W {
        _INT6W { w: self }
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline]
    pub fn int7(&mut self) -> _INT7W {
        _INT7W { w: self }
    }
    #[doc = "Bit 8 - Interrupt Request 8"]
    #[inline]
    pub fn int8(&mut self) -> _INT8W {
        _INT8W { w: self }
    }
    #[doc = "Bit 9 - Interrupt Request 9"]
    #[inline]
    pub fn int9(&mut self) -> _INT9W {
        _INT9W { w: self }
    }
    #[doc = "Bit 10 - Interrupt Request 10"]
    #[inline]
    pub fn int10(&mut self) -> _INT10W {
        _INT10W { w: self }
    }
    #[doc = "Bit 11 - Interrupt Request 11"]
    #[inline]
    pub fn int11(&mut self) -> _INT11W {
        _INT11W { w: self }
    }
    #[doc = "Bit 12 - Interrupt Request 12"]
    #[inline]
    pub fn int12(&mut self) -> _INT12W {
        _INT12W { w: self }
    }
    #[doc = "Bit 13 - Interrupt Request 13"]
    #[inline]
    pub fn int13(&mut self) -> _INT13W {
        _INT13W { w: self }
    }
    #[doc = "Bit 14 - Interrupt Request 14"]
    #[inline]
    pub fn int14(&mut self) -> _INT14W {
        _INT14W { w: self }
    }
    #[doc = "Bit 15 - Interrupt Request 15"]
    #[inline]
    pub fn int15(&mut self) -> _INT15W {
        _INT15W { w: self }
    }
    #[doc = "Bit 16 - Interrupt Request 16"]
    #[inline]
    pub fn int16(&mut self) -> _INT16W {
        _INT16W { w: self }
    }
    #[doc = "Bit 17 - Interrupt Request 17"]
    #[inline]
    pub fn int17(&mut self) -> _INT17W {
        _INT17W { w: self }
    }
    #[doc = "Bit 18 - Interrupt Request 18"]
    #[inline]
    pub fn int18(&mut self) -> _INT18W {
        _INT18W { w: self }
    }
    #[doc = "Bit 19 - Interrupt Request 19"]
    #[inline]
    pub fn int19(&mut self) -> _INT19W {
        _INT19W { w: self }
    }
    #[doc = "Bit 20 - Interrupt Request 20"]
    #[inline]
    pub fn int20(&mut self) -> _INT20W {
        _INT20W { w: self }
    }
    #[doc = "Bit 21 - Interrupt Request 21"]
    #[inline]
    pub fn int21(&mut self) -> _INT21W {
        _INT21W { w: self }
    }
    #[doc = "Bit 22 - Interrupt Request 22"]
    #[inline]
    pub fn int22(&mut self) -> _INT22W {
        _INT22W { w: self }
    }
    #[doc = "Bit 23 - Interrupt Request 23"]
    #[inline]
    pub fn int23(&mut self) -> _INT23W {
        _INT23W { w: self }
    }
    #[doc = "Bit 24 - Interrupt Request 24"]
    #[inline]
    pub fn int24(&mut self) -> _INT24W {
        _INT24W { w: self }
    }
    #[doc = "Bit 25 - Interrupt Request 25"]
    #[inline]
    pub fn int25(&mut self) -> _INT25W {
        _INT25W { w: self }
    }
    #[doc = "Bit 26 - Interrupt Request 26"]
    #[inline]
    pub fn int26(&mut self) -> _INT26W {
        _INT26W { w: self }
    }
    #[doc = "Bit 27 - Interrupt Request 27"]
    #[inline]
    pub fn int27(&mut self) -> _INT27W {
        _INT27W { w: self }
    }
    #[doc = "Bit 28 - Interrupt Request 28"]
    #[inline]
    pub fn int28(&mut self) -> _INT28W {
        _INT28W { w: self }
    }
    #[doc = "Bit 29 - Interrupt Request 29"]
    #[inline]
    pub fn int29(&mut self) -> _INT29W {
        _INT29W { w: self }
    }
    #[doc = "Bit 30 - Interrupt Request 30"]
    #[inline]
    pub fn int30(&mut self) -> _INT30W {
        _INT30W { w: self }
    }
    #[doc = "Bit 31 - Interrupt Request 31"]
    #[inline]
    pub fn int31(&mut self) -> _INT31W {
        _INT31W { w: self }
    }
}
