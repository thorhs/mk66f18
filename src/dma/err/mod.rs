#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERR {
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
#[doc = "Possible values of the field `ERR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR0R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR0R {
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
            ERR0R::_0 => false,
            ERR0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR0R {
        match value {
            false => ERR0R::_0,
            true => ERR0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR0R::_1
    }
}
#[doc = "Possible values of the field `ERR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR1R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR1R {
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
            ERR1R::_0 => false,
            ERR1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR1R {
        match value {
            false => ERR1R::_0,
            true => ERR1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR1R::_1
    }
}
#[doc = "Possible values of the field `ERR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR2R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR2R {
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
            ERR2R::_0 => false,
            ERR2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR2R {
        match value {
            false => ERR2R::_0,
            true => ERR2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR2R::_1
    }
}
#[doc = "Possible values of the field `ERR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR3R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR3R {
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
            ERR3R::_0 => false,
            ERR3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR3R {
        match value {
            false => ERR3R::_0,
            true => ERR3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR3R::_1
    }
}
#[doc = "Possible values of the field `ERR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR4R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR4R {
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
            ERR4R::_0 => false,
            ERR4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR4R {
        match value {
            false => ERR4R::_0,
            true => ERR4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR4R::_1
    }
}
#[doc = "Possible values of the field `ERR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR5R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR5R {
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
            ERR5R::_0 => false,
            ERR5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR5R {
        match value {
            false => ERR5R::_0,
            true => ERR5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR5R::_1
    }
}
#[doc = "Possible values of the field `ERR6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR6R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR6R {
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
            ERR6R::_0 => false,
            ERR6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR6R {
        match value {
            false => ERR6R::_0,
            true => ERR6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR6R::_1
    }
}
#[doc = "Possible values of the field `ERR7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR7R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR7R {
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
            ERR7R::_0 => false,
            ERR7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR7R {
        match value {
            false => ERR7R::_0,
            true => ERR7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR7R::_1
    }
}
#[doc = "Possible values of the field `ERR8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR8R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR8R {
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
            ERR8R::_0 => false,
            ERR8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR8R {
        match value {
            false => ERR8R::_0,
            true => ERR8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR8R::_1
    }
}
#[doc = "Possible values of the field `ERR9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR9R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR9R {
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
            ERR9R::_0 => false,
            ERR9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR9R {
        match value {
            false => ERR9R::_0,
            true => ERR9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR9R::_1
    }
}
#[doc = "Possible values of the field `ERR10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR10R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR10R {
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
            ERR10R::_0 => false,
            ERR10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR10R {
        match value {
            false => ERR10R::_0,
            true => ERR10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR10R::_1
    }
}
#[doc = "Possible values of the field `ERR11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR11R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR11R {
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
            ERR11R::_0 => false,
            ERR11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR11R {
        match value {
            false => ERR11R::_0,
            true => ERR11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR11R::_1
    }
}
#[doc = "Possible values of the field `ERR12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR12R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR12R {
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
            ERR12R::_0 => false,
            ERR12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR12R {
        match value {
            false => ERR12R::_0,
            true => ERR12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR12R::_1
    }
}
#[doc = "Possible values of the field `ERR13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR13R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR13R {
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
            ERR13R::_0 => false,
            ERR13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR13R {
        match value {
            false => ERR13R::_0,
            true => ERR13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR13R::_1
    }
}
#[doc = "Possible values of the field `ERR14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR14R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR14R {
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
            ERR14R::_0 => false,
            ERR14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR14R {
        match value {
            false => ERR14R::_0,
            true => ERR14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR14R::_1
    }
}
#[doc = "Possible values of the field `ERR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR15R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR15R {
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
            ERR15R::_0 => false,
            ERR15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR15R {
        match value {
            false => ERR15R::_0,
            true => ERR15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR15R::_1
    }
}
#[doc = "Possible values of the field `ERR16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR16R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR16R {
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
            ERR16R::_0 => false,
            ERR16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR16R {
        match value {
            false => ERR16R::_0,
            true => ERR16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR16R::_1
    }
}
#[doc = "Possible values of the field `ERR17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR17R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR17R {
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
            ERR17R::_0 => false,
            ERR17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR17R {
        match value {
            false => ERR17R::_0,
            true => ERR17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR17R::_1
    }
}
#[doc = "Possible values of the field `ERR18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR18R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR18R {
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
            ERR18R::_0 => false,
            ERR18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR18R {
        match value {
            false => ERR18R::_0,
            true => ERR18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR18R::_1
    }
}
#[doc = "Possible values of the field `ERR19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR19R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR19R {
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
            ERR19R::_0 => false,
            ERR19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR19R {
        match value {
            false => ERR19R::_0,
            true => ERR19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR19R::_1
    }
}
#[doc = "Possible values of the field `ERR20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR20R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR20R {
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
            ERR20R::_0 => false,
            ERR20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR20R {
        match value {
            false => ERR20R::_0,
            true => ERR20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR20R::_1
    }
}
#[doc = "Possible values of the field `ERR21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR21R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR21R {
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
            ERR21R::_0 => false,
            ERR21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR21R {
        match value {
            false => ERR21R::_0,
            true => ERR21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR21R::_1
    }
}
#[doc = "Possible values of the field `ERR22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR22R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR22R {
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
            ERR22R::_0 => false,
            ERR22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR22R {
        match value {
            false => ERR22R::_0,
            true => ERR22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR22R::_1
    }
}
#[doc = "Possible values of the field `ERR23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR23R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR23R {
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
            ERR23R::_0 => false,
            ERR23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR23R {
        match value {
            false => ERR23R::_0,
            true => ERR23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR23R::_1
    }
}
#[doc = "Possible values of the field `ERR24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR24R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR24R {
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
            ERR24R::_0 => false,
            ERR24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR24R {
        match value {
            false => ERR24R::_0,
            true => ERR24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR24R::_1
    }
}
#[doc = "Possible values of the field `ERR25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR25R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR25R {
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
            ERR25R::_0 => false,
            ERR25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR25R {
        match value {
            false => ERR25R::_0,
            true => ERR25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR25R::_1
    }
}
#[doc = "Possible values of the field `ERR26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR26R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR26R {
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
            ERR26R::_0 => false,
            ERR26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR26R {
        match value {
            false => ERR26R::_0,
            true => ERR26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR26R::_1
    }
}
#[doc = "Possible values of the field `ERR27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR27R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR27R {
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
            ERR27R::_0 => false,
            ERR27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR27R {
        match value {
            false => ERR27R::_0,
            true => ERR27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR27R::_1
    }
}
#[doc = "Possible values of the field `ERR28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR28R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR28R {
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
            ERR28R::_0 => false,
            ERR28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR28R {
        match value {
            false => ERR28R::_0,
            true => ERR28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR28R::_1
    }
}
#[doc = "Possible values of the field `ERR29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR29R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR29R {
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
            ERR29R::_0 => false,
            ERR29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR29R {
        match value {
            false => ERR29R::_0,
            true => ERR29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR29R::_1
    }
}
#[doc = "Possible values of the field `ERR30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR30R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR30R {
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
            ERR30R::_0 => false,
            ERR30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR30R {
        match value {
            false => ERR30R::_0,
            true => ERR30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR30R::_1
    }
}
#[doc = "Possible values of the field `ERR31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR31R {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR31R {
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
            ERR31R::_0 => false,
            ERR31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERR31R {
        match value {
            false => ERR31R::_0,
            true => ERR31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERR31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERR31R::_1
    }
}
#[doc = "Values that can be written to the field `ERR0`"]
pub enum ERR0W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR0W::_0 => false,
            ERR0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR0W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR0W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR0W::_1)
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
#[doc = "Values that can be written to the field `ERR1`"]
pub enum ERR1W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR1W::_0 => false,
            ERR1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR1W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR1W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR1W::_1)
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
#[doc = "Values that can be written to the field `ERR2`"]
pub enum ERR2W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR2W::_0 => false,
            ERR2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR2W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR2W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR2W::_1)
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
#[doc = "Values that can be written to the field `ERR3`"]
pub enum ERR3W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR3W::_0 => false,
            ERR3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR3W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR3W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR3W::_1)
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
#[doc = "Values that can be written to the field `ERR4`"]
pub enum ERR4W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR4W::_0 => false,
            ERR4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR4W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR4W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR4W::_1)
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
#[doc = "Values that can be written to the field `ERR5`"]
pub enum ERR5W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR5W::_0 => false,
            ERR5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR5W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR5W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR5W::_1)
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
#[doc = "Values that can be written to the field `ERR6`"]
pub enum ERR6W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR6W::_0 => false,
            ERR6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR6W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR6W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR6W::_1)
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
#[doc = "Values that can be written to the field `ERR7`"]
pub enum ERR7W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR7W::_0 => false,
            ERR7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR7W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR7W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR7W::_1)
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
#[doc = "Values that can be written to the field `ERR8`"]
pub enum ERR8W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR8W::_0 => false,
            ERR8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR8W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR8W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR8W::_1)
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
#[doc = "Values that can be written to the field `ERR9`"]
pub enum ERR9W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR9W::_0 => false,
            ERR9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR9W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR9W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR9W::_1)
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
#[doc = "Values that can be written to the field `ERR10`"]
pub enum ERR10W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR10W::_0 => false,
            ERR10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR10W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR10W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR10W::_1)
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
#[doc = "Values that can be written to the field `ERR11`"]
pub enum ERR11W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR11W::_0 => false,
            ERR11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR11W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR11W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR11W::_1)
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
#[doc = "Values that can be written to the field `ERR12`"]
pub enum ERR12W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR12W::_0 => false,
            ERR12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR12W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR12W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR12W::_1)
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
#[doc = "Values that can be written to the field `ERR13`"]
pub enum ERR13W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR13W::_0 => false,
            ERR13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR13W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR13W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR13W::_1)
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
#[doc = "Values that can be written to the field `ERR14`"]
pub enum ERR14W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR14W::_0 => false,
            ERR14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR14W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR14W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR14W::_1)
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
#[doc = "Values that can be written to the field `ERR15`"]
pub enum ERR15W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR15W::_0 => false,
            ERR15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR15W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR15W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR15W::_1)
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
#[doc = "Values that can be written to the field `ERR16`"]
pub enum ERR16W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR16W::_0 => false,
            ERR16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR16W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR16W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR16W::_1)
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
#[doc = "Values that can be written to the field `ERR17`"]
pub enum ERR17W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR17W::_0 => false,
            ERR17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR17W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR17W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR17W::_1)
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
#[doc = "Values that can be written to the field `ERR18`"]
pub enum ERR18W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR18W::_0 => false,
            ERR18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR18W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR18W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR18W::_1)
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
#[doc = "Values that can be written to the field `ERR19`"]
pub enum ERR19W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR19W::_0 => false,
            ERR19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR19W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR19W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR19W::_1)
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
#[doc = "Values that can be written to the field `ERR20`"]
pub enum ERR20W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR20W::_0 => false,
            ERR20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR20W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR20W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR20W::_1)
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
#[doc = "Values that can be written to the field `ERR21`"]
pub enum ERR21W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR21W::_0 => false,
            ERR21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR21W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR21W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR21W::_1)
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
#[doc = "Values that can be written to the field `ERR22`"]
pub enum ERR22W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR22W::_0 => false,
            ERR22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR22W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR22W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR22W::_1)
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
#[doc = "Values that can be written to the field `ERR23`"]
pub enum ERR23W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR23W::_0 => false,
            ERR23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR23W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR23W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR23W::_1)
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
#[doc = "Values that can be written to the field `ERR24`"]
pub enum ERR24W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR24W::_0 => false,
            ERR24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR24W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR24W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR24W::_1)
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
#[doc = "Values that can be written to the field `ERR25`"]
pub enum ERR25W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR25W::_0 => false,
            ERR25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR25W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR25W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR25W::_1)
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
#[doc = "Values that can be written to the field `ERR26`"]
pub enum ERR26W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR26W::_0 => false,
            ERR26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR26W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR26W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR26W::_1)
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
#[doc = "Values that can be written to the field `ERR27`"]
pub enum ERR27W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR27W::_0 => false,
            ERR27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR27W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR27W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR27W::_1)
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
#[doc = "Values that can be written to the field `ERR28`"]
pub enum ERR28W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR28W::_0 => false,
            ERR28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR28W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR28W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR28W::_1)
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
#[doc = "Values that can be written to the field `ERR29`"]
pub enum ERR29W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR29W::_0 => false,
            ERR29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR29W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR29W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR29W::_1)
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
#[doc = "Values that can be written to the field `ERR30`"]
pub enum ERR30W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR30W::_0 => false,
            ERR30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR30W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR30W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR30W::_1)
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
#[doc = "Values that can be written to the field `ERR31`"]
pub enum ERR31W {
    #[doc = "An error in this channel has not occurred"]
    _0,
    #[doc = "An error in this channel has occurred"]
    _1,
}
impl ERR31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERR31W::_0 => false,
            ERR31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR31W<'a> {
    w: &'a mut W,
}
impl<'a> _ERR31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR31W::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR31W::_1)
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
    #[doc = "Bit 0 - Error In Channel 0"]
    #[inline]
    pub fn err0(&self) -> ERR0R {
        ERR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Error In Channel 1"]
    #[inline]
    pub fn err1(&self) -> ERR1R {
        ERR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Error In Channel 2"]
    #[inline]
    pub fn err2(&self) -> ERR2R {
        ERR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Error In Channel 3"]
    #[inline]
    pub fn err3(&self) -> ERR3R {
        ERR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Error In Channel 4"]
    #[inline]
    pub fn err4(&self) -> ERR4R {
        ERR4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Error In Channel 5"]
    #[inline]
    pub fn err5(&self) -> ERR5R {
        ERR5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Error In Channel 6"]
    #[inline]
    pub fn err6(&self) -> ERR6R {
        ERR6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Error In Channel 7"]
    #[inline]
    pub fn err7(&self) -> ERR7R {
        ERR7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Error In Channel 8"]
    #[inline]
    pub fn err8(&self) -> ERR8R {
        ERR8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Error In Channel 9"]
    #[inline]
    pub fn err9(&self) -> ERR9R {
        ERR9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Error In Channel 10"]
    #[inline]
    pub fn err10(&self) -> ERR10R {
        ERR10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Error In Channel 11"]
    #[inline]
    pub fn err11(&self) -> ERR11R {
        ERR11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Error In Channel 12"]
    #[inline]
    pub fn err12(&self) -> ERR12R {
        ERR12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Error In Channel 13"]
    #[inline]
    pub fn err13(&self) -> ERR13R {
        ERR13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Error In Channel 14"]
    #[inline]
    pub fn err14(&self) -> ERR14R {
        ERR14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Error In Channel 15"]
    #[inline]
    pub fn err15(&self) -> ERR15R {
        ERR15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Error In Channel 16"]
    #[inline]
    pub fn err16(&self) -> ERR16R {
        ERR16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Error In Channel 17"]
    #[inline]
    pub fn err17(&self) -> ERR17R {
        ERR17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Error In Channel 18"]
    #[inline]
    pub fn err18(&self) -> ERR18R {
        ERR18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Error In Channel 19"]
    #[inline]
    pub fn err19(&self) -> ERR19R {
        ERR19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Error In Channel 20"]
    #[inline]
    pub fn err20(&self) -> ERR20R {
        ERR20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Error In Channel 21"]
    #[inline]
    pub fn err21(&self) -> ERR21R {
        ERR21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Error In Channel 22"]
    #[inline]
    pub fn err22(&self) -> ERR22R {
        ERR22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Error In Channel 23"]
    #[inline]
    pub fn err23(&self) -> ERR23R {
        ERR23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Error In Channel 24"]
    #[inline]
    pub fn err24(&self) -> ERR24R {
        ERR24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Error In Channel 25"]
    #[inline]
    pub fn err25(&self) -> ERR25R {
        ERR25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Error In Channel 26"]
    #[inline]
    pub fn err26(&self) -> ERR26R {
        ERR26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Error In Channel 27"]
    #[inline]
    pub fn err27(&self) -> ERR27R {
        ERR27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Error In Channel 28"]
    #[inline]
    pub fn err28(&self) -> ERR28R {
        ERR28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Error In Channel 29"]
    #[inline]
    pub fn err29(&self) -> ERR29R {
        ERR29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Error In Channel 30"]
    #[inline]
    pub fn err30(&self) -> ERR30R {
        ERR30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Error In Channel 31"]
    #[inline]
    pub fn err31(&self) -> ERR31R {
        ERR31R::_from({
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
    #[doc = "Bit 0 - Error In Channel 0"]
    #[inline]
    pub fn err0(&mut self) -> _ERR0W {
        _ERR0W { w: self }
    }
    #[doc = "Bit 1 - Error In Channel 1"]
    #[inline]
    pub fn err1(&mut self) -> _ERR1W {
        _ERR1W { w: self }
    }
    #[doc = "Bit 2 - Error In Channel 2"]
    #[inline]
    pub fn err2(&mut self) -> _ERR2W {
        _ERR2W { w: self }
    }
    #[doc = "Bit 3 - Error In Channel 3"]
    #[inline]
    pub fn err3(&mut self) -> _ERR3W {
        _ERR3W { w: self }
    }
    #[doc = "Bit 4 - Error In Channel 4"]
    #[inline]
    pub fn err4(&mut self) -> _ERR4W {
        _ERR4W { w: self }
    }
    #[doc = "Bit 5 - Error In Channel 5"]
    #[inline]
    pub fn err5(&mut self) -> _ERR5W {
        _ERR5W { w: self }
    }
    #[doc = "Bit 6 - Error In Channel 6"]
    #[inline]
    pub fn err6(&mut self) -> _ERR6W {
        _ERR6W { w: self }
    }
    #[doc = "Bit 7 - Error In Channel 7"]
    #[inline]
    pub fn err7(&mut self) -> _ERR7W {
        _ERR7W { w: self }
    }
    #[doc = "Bit 8 - Error In Channel 8"]
    #[inline]
    pub fn err8(&mut self) -> _ERR8W {
        _ERR8W { w: self }
    }
    #[doc = "Bit 9 - Error In Channel 9"]
    #[inline]
    pub fn err9(&mut self) -> _ERR9W {
        _ERR9W { w: self }
    }
    #[doc = "Bit 10 - Error In Channel 10"]
    #[inline]
    pub fn err10(&mut self) -> _ERR10W {
        _ERR10W { w: self }
    }
    #[doc = "Bit 11 - Error In Channel 11"]
    #[inline]
    pub fn err11(&mut self) -> _ERR11W {
        _ERR11W { w: self }
    }
    #[doc = "Bit 12 - Error In Channel 12"]
    #[inline]
    pub fn err12(&mut self) -> _ERR12W {
        _ERR12W { w: self }
    }
    #[doc = "Bit 13 - Error In Channel 13"]
    #[inline]
    pub fn err13(&mut self) -> _ERR13W {
        _ERR13W { w: self }
    }
    #[doc = "Bit 14 - Error In Channel 14"]
    #[inline]
    pub fn err14(&mut self) -> _ERR14W {
        _ERR14W { w: self }
    }
    #[doc = "Bit 15 - Error In Channel 15"]
    #[inline]
    pub fn err15(&mut self) -> _ERR15W {
        _ERR15W { w: self }
    }
    #[doc = "Bit 16 - Error In Channel 16"]
    #[inline]
    pub fn err16(&mut self) -> _ERR16W {
        _ERR16W { w: self }
    }
    #[doc = "Bit 17 - Error In Channel 17"]
    #[inline]
    pub fn err17(&mut self) -> _ERR17W {
        _ERR17W { w: self }
    }
    #[doc = "Bit 18 - Error In Channel 18"]
    #[inline]
    pub fn err18(&mut self) -> _ERR18W {
        _ERR18W { w: self }
    }
    #[doc = "Bit 19 - Error In Channel 19"]
    #[inline]
    pub fn err19(&mut self) -> _ERR19W {
        _ERR19W { w: self }
    }
    #[doc = "Bit 20 - Error In Channel 20"]
    #[inline]
    pub fn err20(&mut self) -> _ERR20W {
        _ERR20W { w: self }
    }
    #[doc = "Bit 21 - Error In Channel 21"]
    #[inline]
    pub fn err21(&mut self) -> _ERR21W {
        _ERR21W { w: self }
    }
    #[doc = "Bit 22 - Error In Channel 22"]
    #[inline]
    pub fn err22(&mut self) -> _ERR22W {
        _ERR22W { w: self }
    }
    #[doc = "Bit 23 - Error In Channel 23"]
    #[inline]
    pub fn err23(&mut self) -> _ERR23W {
        _ERR23W { w: self }
    }
    #[doc = "Bit 24 - Error In Channel 24"]
    #[inline]
    pub fn err24(&mut self) -> _ERR24W {
        _ERR24W { w: self }
    }
    #[doc = "Bit 25 - Error In Channel 25"]
    #[inline]
    pub fn err25(&mut self) -> _ERR25W {
        _ERR25W { w: self }
    }
    #[doc = "Bit 26 - Error In Channel 26"]
    #[inline]
    pub fn err26(&mut self) -> _ERR26W {
        _ERR26W { w: self }
    }
    #[doc = "Bit 27 - Error In Channel 27"]
    #[inline]
    pub fn err27(&mut self) -> _ERR27W {
        _ERR27W { w: self }
    }
    #[doc = "Bit 28 - Error In Channel 28"]
    #[inline]
    pub fn err28(&mut self) -> _ERR28W {
        _ERR28W { w: self }
    }
    #[doc = "Bit 29 - Error In Channel 29"]
    #[inline]
    pub fn err29(&mut self) -> _ERR29W {
        _ERR29W { w: self }
    }
    #[doc = "Bit 30 - Error In Channel 30"]
    #[inline]
    pub fn err30(&mut self) -> _ERR30W {
        _ERR30W { w: self }
    }
    #[doc = "Bit 31 - Error In Channel 31"]
    #[inline]
    pub fn err31(&mut self) -> _ERR31W {
        _ERR31W { w: self }
    }
}
