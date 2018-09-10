#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EEI {
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
#[doc = "Possible values of the field `EEI0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI0R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI0R {
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
            EEI0R::_0 => false,
            EEI0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI0R {
        match value {
            false => EEI0R::_0,
            true => EEI0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI0R::_1
    }
}
#[doc = "Possible values of the field `EEI1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI1R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI1R {
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
            EEI1R::_0 => false,
            EEI1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI1R {
        match value {
            false => EEI1R::_0,
            true => EEI1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI1R::_1
    }
}
#[doc = "Possible values of the field `EEI2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI2R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI2R {
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
            EEI2R::_0 => false,
            EEI2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI2R {
        match value {
            false => EEI2R::_0,
            true => EEI2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI2R::_1
    }
}
#[doc = "Possible values of the field `EEI3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI3R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI3R {
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
            EEI3R::_0 => false,
            EEI3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI3R {
        match value {
            false => EEI3R::_0,
            true => EEI3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI3R::_1
    }
}
#[doc = "Possible values of the field `EEI4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI4R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI4R {
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
            EEI4R::_0 => false,
            EEI4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI4R {
        match value {
            false => EEI4R::_0,
            true => EEI4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI4R::_1
    }
}
#[doc = "Possible values of the field `EEI5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI5R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI5R {
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
            EEI5R::_0 => false,
            EEI5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI5R {
        match value {
            false => EEI5R::_0,
            true => EEI5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI5R::_1
    }
}
#[doc = "Possible values of the field `EEI6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI6R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI6R {
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
            EEI6R::_0 => false,
            EEI6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI6R {
        match value {
            false => EEI6R::_0,
            true => EEI6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI6R::_1
    }
}
#[doc = "Possible values of the field `EEI7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI7R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI7R {
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
            EEI7R::_0 => false,
            EEI7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI7R {
        match value {
            false => EEI7R::_0,
            true => EEI7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI7R::_1
    }
}
#[doc = "Possible values of the field `EEI8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI8R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI8R {
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
            EEI8R::_0 => false,
            EEI8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI8R {
        match value {
            false => EEI8R::_0,
            true => EEI8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI8R::_1
    }
}
#[doc = "Possible values of the field `EEI9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI9R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI9R {
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
            EEI9R::_0 => false,
            EEI9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI9R {
        match value {
            false => EEI9R::_0,
            true => EEI9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI9R::_1
    }
}
#[doc = "Possible values of the field `EEI10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI10R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI10R {
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
            EEI10R::_0 => false,
            EEI10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI10R {
        match value {
            false => EEI10R::_0,
            true => EEI10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI10R::_1
    }
}
#[doc = "Possible values of the field `EEI11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI11R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI11R {
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
            EEI11R::_0 => false,
            EEI11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI11R {
        match value {
            false => EEI11R::_0,
            true => EEI11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI11R::_1
    }
}
#[doc = "Possible values of the field `EEI12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI12R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI12R {
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
            EEI12R::_0 => false,
            EEI12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI12R {
        match value {
            false => EEI12R::_0,
            true => EEI12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI12R::_1
    }
}
#[doc = "Possible values of the field `EEI13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI13R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI13R {
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
            EEI13R::_0 => false,
            EEI13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI13R {
        match value {
            false => EEI13R::_0,
            true => EEI13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI13R::_1
    }
}
#[doc = "Possible values of the field `EEI14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI14R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI14R {
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
            EEI14R::_0 => false,
            EEI14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI14R {
        match value {
            false => EEI14R::_0,
            true => EEI14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI14R::_1
    }
}
#[doc = "Possible values of the field `EEI15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI15R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI15R {
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
            EEI15R::_0 => false,
            EEI15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI15R {
        match value {
            false => EEI15R::_0,
            true => EEI15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI15R::_1
    }
}
#[doc = "Possible values of the field `EEI16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI16R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI16R {
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
            EEI16R::_0 => false,
            EEI16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI16R {
        match value {
            false => EEI16R::_0,
            true => EEI16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI16R::_1
    }
}
#[doc = "Possible values of the field `EEI17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI17R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI17R {
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
            EEI17R::_0 => false,
            EEI17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI17R {
        match value {
            false => EEI17R::_0,
            true => EEI17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI17R::_1
    }
}
#[doc = "Possible values of the field `EEI18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI18R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI18R {
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
            EEI18R::_0 => false,
            EEI18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI18R {
        match value {
            false => EEI18R::_0,
            true => EEI18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI18R::_1
    }
}
#[doc = "Possible values of the field `EEI19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI19R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI19R {
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
            EEI19R::_0 => false,
            EEI19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI19R {
        match value {
            false => EEI19R::_0,
            true => EEI19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI19R::_1
    }
}
#[doc = "Possible values of the field `EEI20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI20R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI20R {
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
            EEI20R::_0 => false,
            EEI20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI20R {
        match value {
            false => EEI20R::_0,
            true => EEI20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI20R::_1
    }
}
#[doc = "Possible values of the field `EEI21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI21R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI21R {
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
            EEI21R::_0 => false,
            EEI21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI21R {
        match value {
            false => EEI21R::_0,
            true => EEI21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI21R::_1
    }
}
#[doc = "Possible values of the field `EEI22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI22R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI22R {
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
            EEI22R::_0 => false,
            EEI22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI22R {
        match value {
            false => EEI22R::_0,
            true => EEI22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI22R::_1
    }
}
#[doc = "Possible values of the field `EEI23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI23R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI23R {
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
            EEI23R::_0 => false,
            EEI23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI23R {
        match value {
            false => EEI23R::_0,
            true => EEI23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI23R::_1
    }
}
#[doc = "Possible values of the field `EEI24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI24R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI24R {
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
            EEI24R::_0 => false,
            EEI24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI24R {
        match value {
            false => EEI24R::_0,
            true => EEI24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI24R::_1
    }
}
#[doc = "Possible values of the field `EEI25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI25R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI25R {
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
            EEI25R::_0 => false,
            EEI25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI25R {
        match value {
            false => EEI25R::_0,
            true => EEI25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI25R::_1
    }
}
#[doc = "Possible values of the field `EEI26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI26R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI26R {
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
            EEI26R::_0 => false,
            EEI26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI26R {
        match value {
            false => EEI26R::_0,
            true => EEI26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI26R::_1
    }
}
#[doc = "Possible values of the field `EEI27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI27R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI27R {
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
            EEI27R::_0 => false,
            EEI27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI27R {
        match value {
            false => EEI27R::_0,
            true => EEI27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI27R::_1
    }
}
#[doc = "Possible values of the field `EEI28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI28R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI28R {
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
            EEI28R::_0 => false,
            EEI28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI28R {
        match value {
            false => EEI28R::_0,
            true => EEI28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI28R::_1
    }
}
#[doc = "Possible values of the field `EEI29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI29R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI29R {
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
            EEI29R::_0 => false,
            EEI29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI29R {
        match value {
            false => EEI29R::_0,
            true => EEI29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI29R::_1
    }
}
#[doc = "Possible values of the field `EEI30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI30R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI30R {
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
            EEI30R::_0 => false,
            EEI30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI30R {
        match value {
            false => EEI30R::_0,
            true => EEI30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI30R::_1
    }
}
#[doc = "Possible values of the field `EEI31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI31R {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI31R {
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
            EEI31R::_0 => false,
            EEI31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EEI31R {
        match value {
            false => EEI31R::_0,
            true => EEI31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EEI31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EEI31R::_1
    }
}
#[doc = "Values that can be written to the field `EEI0`"]
pub enum EEI0W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI0W::_0 => false,
            EEI0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI0W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI0W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI0W::_1)
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
#[doc = "Values that can be written to the field `EEI1`"]
pub enum EEI1W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI1W::_0 => false,
            EEI1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI1W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI1W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI1W::_1)
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
#[doc = "Values that can be written to the field `EEI2`"]
pub enum EEI2W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI2W::_0 => false,
            EEI2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI2W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI2W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI2W::_1)
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
#[doc = "Values that can be written to the field `EEI3`"]
pub enum EEI3W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI3W::_0 => false,
            EEI3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI3W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI3W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI3W::_1)
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
#[doc = "Values that can be written to the field `EEI4`"]
pub enum EEI4W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI4W::_0 => false,
            EEI4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI4W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI4W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI4W::_1)
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
#[doc = "Values that can be written to the field `EEI5`"]
pub enum EEI5W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI5W::_0 => false,
            EEI5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI5W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI5W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI5W::_1)
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
#[doc = "Values that can be written to the field `EEI6`"]
pub enum EEI6W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI6W::_0 => false,
            EEI6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI6W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI6W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI6W::_1)
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
#[doc = "Values that can be written to the field `EEI7`"]
pub enum EEI7W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI7W::_0 => false,
            EEI7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI7W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI7W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI7W::_1)
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
#[doc = "Values that can be written to the field `EEI8`"]
pub enum EEI8W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI8W::_0 => false,
            EEI8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI8W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI8W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI8W::_1)
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
#[doc = "Values that can be written to the field `EEI9`"]
pub enum EEI9W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI9W::_0 => false,
            EEI9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI9W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI9W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI9W::_1)
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
#[doc = "Values that can be written to the field `EEI10`"]
pub enum EEI10W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI10W::_0 => false,
            EEI10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI10W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI10W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI10W::_1)
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
#[doc = "Values that can be written to the field `EEI11`"]
pub enum EEI11W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI11W::_0 => false,
            EEI11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI11W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI11W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI11W::_1)
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
#[doc = "Values that can be written to the field `EEI12`"]
pub enum EEI12W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI12W::_0 => false,
            EEI12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI12W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI12W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI12W::_1)
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
#[doc = "Values that can be written to the field `EEI13`"]
pub enum EEI13W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI13W::_0 => false,
            EEI13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI13W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI13W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI13W::_1)
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
#[doc = "Values that can be written to the field `EEI14`"]
pub enum EEI14W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI14W::_0 => false,
            EEI14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI14W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI14W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI14W::_1)
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
#[doc = "Values that can be written to the field `EEI15`"]
pub enum EEI15W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI15W::_0 => false,
            EEI15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI15W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI15W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI15W::_1)
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
#[doc = "Values that can be written to the field `EEI16`"]
pub enum EEI16W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI16W::_0 => false,
            EEI16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI16W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI16W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI16W::_1)
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
#[doc = "Values that can be written to the field `EEI17`"]
pub enum EEI17W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI17W::_0 => false,
            EEI17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI17W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI17W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI17W::_1)
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
#[doc = "Values that can be written to the field `EEI18`"]
pub enum EEI18W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI18W::_0 => false,
            EEI18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI18W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI18W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI18W::_1)
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
#[doc = "Values that can be written to the field `EEI19`"]
pub enum EEI19W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI19W::_0 => false,
            EEI19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI19W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI19W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI19W::_1)
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
#[doc = "Values that can be written to the field `EEI20`"]
pub enum EEI20W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI20W::_0 => false,
            EEI20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI20W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI20W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI20W::_1)
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
#[doc = "Values that can be written to the field `EEI21`"]
pub enum EEI21W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI21W::_0 => false,
            EEI21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI21W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI21W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI21W::_1)
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
#[doc = "Values that can be written to the field `EEI22`"]
pub enum EEI22W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI22W::_0 => false,
            EEI22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI22W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI22W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI22W::_1)
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
#[doc = "Values that can be written to the field `EEI23`"]
pub enum EEI23W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI23W::_0 => false,
            EEI23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI23W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI23W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI23W::_1)
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
#[doc = "Values that can be written to the field `EEI24`"]
pub enum EEI24W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI24W::_0 => false,
            EEI24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI24W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI24W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI24W::_1)
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
#[doc = "Values that can be written to the field `EEI25`"]
pub enum EEI25W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI25W::_0 => false,
            EEI25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI25W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI25W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI25W::_1)
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
#[doc = "Values that can be written to the field `EEI26`"]
pub enum EEI26W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI26W::_0 => false,
            EEI26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI26W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI26W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI26W::_1)
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
#[doc = "Values that can be written to the field `EEI27`"]
pub enum EEI27W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI27W::_0 => false,
            EEI27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI27W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI27W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI27W::_1)
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
#[doc = "Values that can be written to the field `EEI28`"]
pub enum EEI28W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI28W::_0 => false,
            EEI28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI28W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI28W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI28W::_1)
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
#[doc = "Values that can be written to the field `EEI29`"]
pub enum EEI29W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI29W::_0 => false,
            EEI29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI29W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI29W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI29W::_1)
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
#[doc = "Values that can be written to the field `EEI30`"]
pub enum EEI30W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI30W::_0 => false,
            EEI30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI30W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI30W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI30W::_1)
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
#[doc = "Values that can be written to the field `EEI31`"]
pub enum EEI31W {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    _0,
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1,
}
impl EEI31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EEI31W::_0 => false,
            EEI31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEI31W<'a> {
    w: &'a mut W,
}
impl<'a> _EEI31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEI31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI31W::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI31W::_1)
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
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline]
    pub fn eei0(&self) -> EEI0R {
        EEI0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline]
    pub fn eei1(&self) -> EEI1R {
        EEI1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline]
    pub fn eei2(&self) -> EEI2R {
        EEI2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline]
    pub fn eei3(&self) -> EEI3R {
        EEI3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable Error Interrupt 4"]
    #[inline]
    pub fn eei4(&self) -> EEI4R {
        EEI4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable Error Interrupt 5"]
    #[inline]
    pub fn eei5(&self) -> EEI5R {
        EEI5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable Error Interrupt 6"]
    #[inline]
    pub fn eei6(&self) -> EEI6R {
        EEI6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable Error Interrupt 7"]
    #[inline]
    pub fn eei7(&self) -> EEI7R {
        EEI7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable Error Interrupt 8"]
    #[inline]
    pub fn eei8(&self) -> EEI8R {
        EEI8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable Error Interrupt 9"]
    #[inline]
    pub fn eei9(&self) -> EEI9R {
        EEI9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable Error Interrupt 10"]
    #[inline]
    pub fn eei10(&self) -> EEI10R {
        EEI10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable Error Interrupt 11"]
    #[inline]
    pub fn eei11(&self) -> EEI11R {
        EEI11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable Error Interrupt 12"]
    #[inline]
    pub fn eei12(&self) -> EEI12R {
        EEI12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable Error Interrupt 13"]
    #[inline]
    pub fn eei13(&self) -> EEI13R {
        EEI13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enable Error Interrupt 14"]
    #[inline]
    pub fn eei14(&self) -> EEI14R {
        EEI14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enable Error Interrupt 15"]
    #[inline]
    pub fn eei15(&self) -> EEI15R {
        EEI15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable Error Interrupt 16"]
    #[inline]
    pub fn eei16(&self) -> EEI16R {
        EEI16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enable Error Interrupt 17"]
    #[inline]
    pub fn eei17(&self) -> EEI17R {
        EEI17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enable Error Interrupt 18"]
    #[inline]
    pub fn eei18(&self) -> EEI18R {
        EEI18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enable Error Interrupt 19"]
    #[inline]
    pub fn eei19(&self) -> EEI19R {
        EEI19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enable Error Interrupt 20"]
    #[inline]
    pub fn eei20(&self) -> EEI20R {
        EEI20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Enable Error Interrupt 21"]
    #[inline]
    pub fn eei21(&self) -> EEI21R {
        EEI21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Enable Error Interrupt 22"]
    #[inline]
    pub fn eei22(&self) -> EEI22R {
        EEI22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enable Error Interrupt 23"]
    #[inline]
    pub fn eei23(&self) -> EEI23R {
        EEI23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable Error Interrupt 24"]
    #[inline]
    pub fn eei24(&self) -> EEI24R {
        EEI24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enable Error Interrupt 25"]
    #[inline]
    pub fn eei25(&self) -> EEI25R {
        EEI25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enable Error Interrupt 26"]
    #[inline]
    pub fn eei26(&self) -> EEI26R {
        EEI26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enable Error Interrupt 27"]
    #[inline]
    pub fn eei27(&self) -> EEI27R {
        EEI27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Enable Error Interrupt 28"]
    #[inline]
    pub fn eei28(&self) -> EEI28R {
        EEI28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Enable Error Interrupt 29"]
    #[inline]
    pub fn eei29(&self) -> EEI29R {
        EEI29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Enable Error Interrupt 30"]
    #[inline]
    pub fn eei30(&self) -> EEI30R {
        EEI30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enable Error Interrupt 31"]
    #[inline]
    pub fn eei31(&self) -> EEI31R {
        EEI31R::_from({
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
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline]
    pub fn eei0(&mut self) -> _EEI0W {
        _EEI0W { w: self }
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline]
    pub fn eei1(&mut self) -> _EEI1W {
        _EEI1W { w: self }
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline]
    pub fn eei2(&mut self) -> _EEI2W {
        _EEI2W { w: self }
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline]
    pub fn eei3(&mut self) -> _EEI3W {
        _EEI3W { w: self }
    }
    #[doc = "Bit 4 - Enable Error Interrupt 4"]
    #[inline]
    pub fn eei4(&mut self) -> _EEI4W {
        _EEI4W { w: self }
    }
    #[doc = "Bit 5 - Enable Error Interrupt 5"]
    #[inline]
    pub fn eei5(&mut self) -> _EEI5W {
        _EEI5W { w: self }
    }
    #[doc = "Bit 6 - Enable Error Interrupt 6"]
    #[inline]
    pub fn eei6(&mut self) -> _EEI6W {
        _EEI6W { w: self }
    }
    #[doc = "Bit 7 - Enable Error Interrupt 7"]
    #[inline]
    pub fn eei7(&mut self) -> _EEI7W {
        _EEI7W { w: self }
    }
    #[doc = "Bit 8 - Enable Error Interrupt 8"]
    #[inline]
    pub fn eei8(&mut self) -> _EEI8W {
        _EEI8W { w: self }
    }
    #[doc = "Bit 9 - Enable Error Interrupt 9"]
    #[inline]
    pub fn eei9(&mut self) -> _EEI9W {
        _EEI9W { w: self }
    }
    #[doc = "Bit 10 - Enable Error Interrupt 10"]
    #[inline]
    pub fn eei10(&mut self) -> _EEI10W {
        _EEI10W { w: self }
    }
    #[doc = "Bit 11 - Enable Error Interrupt 11"]
    #[inline]
    pub fn eei11(&mut self) -> _EEI11W {
        _EEI11W { w: self }
    }
    #[doc = "Bit 12 - Enable Error Interrupt 12"]
    #[inline]
    pub fn eei12(&mut self) -> _EEI12W {
        _EEI12W { w: self }
    }
    #[doc = "Bit 13 - Enable Error Interrupt 13"]
    #[inline]
    pub fn eei13(&mut self) -> _EEI13W {
        _EEI13W { w: self }
    }
    #[doc = "Bit 14 - Enable Error Interrupt 14"]
    #[inline]
    pub fn eei14(&mut self) -> _EEI14W {
        _EEI14W { w: self }
    }
    #[doc = "Bit 15 - Enable Error Interrupt 15"]
    #[inline]
    pub fn eei15(&mut self) -> _EEI15W {
        _EEI15W { w: self }
    }
    #[doc = "Bit 16 - Enable Error Interrupt 16"]
    #[inline]
    pub fn eei16(&mut self) -> _EEI16W {
        _EEI16W { w: self }
    }
    #[doc = "Bit 17 - Enable Error Interrupt 17"]
    #[inline]
    pub fn eei17(&mut self) -> _EEI17W {
        _EEI17W { w: self }
    }
    #[doc = "Bit 18 - Enable Error Interrupt 18"]
    #[inline]
    pub fn eei18(&mut self) -> _EEI18W {
        _EEI18W { w: self }
    }
    #[doc = "Bit 19 - Enable Error Interrupt 19"]
    #[inline]
    pub fn eei19(&mut self) -> _EEI19W {
        _EEI19W { w: self }
    }
    #[doc = "Bit 20 - Enable Error Interrupt 20"]
    #[inline]
    pub fn eei20(&mut self) -> _EEI20W {
        _EEI20W { w: self }
    }
    #[doc = "Bit 21 - Enable Error Interrupt 21"]
    #[inline]
    pub fn eei21(&mut self) -> _EEI21W {
        _EEI21W { w: self }
    }
    #[doc = "Bit 22 - Enable Error Interrupt 22"]
    #[inline]
    pub fn eei22(&mut self) -> _EEI22W {
        _EEI22W { w: self }
    }
    #[doc = "Bit 23 - Enable Error Interrupt 23"]
    #[inline]
    pub fn eei23(&mut self) -> _EEI23W {
        _EEI23W { w: self }
    }
    #[doc = "Bit 24 - Enable Error Interrupt 24"]
    #[inline]
    pub fn eei24(&mut self) -> _EEI24W {
        _EEI24W { w: self }
    }
    #[doc = "Bit 25 - Enable Error Interrupt 25"]
    #[inline]
    pub fn eei25(&mut self) -> _EEI25W {
        _EEI25W { w: self }
    }
    #[doc = "Bit 26 - Enable Error Interrupt 26"]
    #[inline]
    pub fn eei26(&mut self) -> _EEI26W {
        _EEI26W { w: self }
    }
    #[doc = "Bit 27 - Enable Error Interrupt 27"]
    #[inline]
    pub fn eei27(&mut self) -> _EEI27W {
        _EEI27W { w: self }
    }
    #[doc = "Bit 28 - Enable Error Interrupt 28"]
    #[inline]
    pub fn eei28(&mut self) -> _EEI28W {
        _EEI28W { w: self }
    }
    #[doc = "Bit 29 - Enable Error Interrupt 29"]
    #[inline]
    pub fn eei29(&mut self) -> _EEI29W {
        _EEI29W { w: self }
    }
    #[doc = "Bit 30 - Enable Error Interrupt 30"]
    #[inline]
    pub fn eei30(&mut self) -> _EEI30W {
        _EEI30W { w: self }
    }
    #[doc = "Bit 31 - Enable Error Interrupt 31"]
    #[inline]
    pub fn eei31(&mut self) -> _EEI31W {
        _EEI31W { w: self }
    }
}
