#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EARS {
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
#[doc = "Possible values of the field `EDREQ_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_0R {
    #[doc = "Disable asynchronous DMA request for channel 0."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 0."]
    _1,
}
impl EDREQ_0R {
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
            EDREQ_0R::_0 => false,
            EDREQ_0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_0R {
        match value {
            false => EDREQ_0R::_0,
            true => EDREQ_0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_0R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_1R {
    #[doc = "Disable asynchronous DMA request for channel 1"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 1."]
    _1,
}
impl EDREQ_1R {
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
            EDREQ_1R::_0 => false,
            EDREQ_1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_1R {
        match value {
            false => EDREQ_1R::_0,
            true => EDREQ_1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_1R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_2R {
    #[doc = "Disable asynchronous DMA request for channel 2."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 2."]
    _1,
}
impl EDREQ_2R {
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
            EDREQ_2R::_0 => false,
            EDREQ_2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_2R {
        match value {
            false => EDREQ_2R::_0,
            true => EDREQ_2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_2R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_3R {
    #[doc = "Disable asynchronous DMA request for channel 3."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 3."]
    _1,
}
impl EDREQ_3R {
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
            EDREQ_3R::_0 => false,
            EDREQ_3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_3R {
        match value {
            false => EDREQ_3R::_0,
            true => EDREQ_3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_3R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_4R {
    #[doc = "Disable asynchronous DMA request for channel 4."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 4."]
    _1,
}
impl EDREQ_4R {
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
            EDREQ_4R::_0 => false,
            EDREQ_4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_4R {
        match value {
            false => EDREQ_4R::_0,
            true => EDREQ_4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_4R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_5R {
    #[doc = "Disable asynchronous DMA request for channel 5."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 5."]
    _1,
}
impl EDREQ_5R {
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
            EDREQ_5R::_0 => false,
            EDREQ_5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_5R {
        match value {
            false => EDREQ_5R::_0,
            true => EDREQ_5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_5R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_6R {
    #[doc = "Disable asynchronous DMA request for channel 6."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 6."]
    _1,
}
impl EDREQ_6R {
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
            EDREQ_6R::_0 => false,
            EDREQ_6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_6R {
        match value {
            false => EDREQ_6R::_0,
            true => EDREQ_6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_6R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_7R {
    #[doc = "Disable asynchronous DMA request for channel 7."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 7."]
    _1,
}
impl EDREQ_7R {
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
            EDREQ_7R::_0 => false,
            EDREQ_7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_7R {
        match value {
            false => EDREQ_7R::_0,
            true => EDREQ_7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_7R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_8R {
    #[doc = "Disable asynchronous DMA request for channel 8."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 8."]
    _1,
}
impl EDREQ_8R {
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
            EDREQ_8R::_0 => false,
            EDREQ_8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_8R {
        match value {
            false => EDREQ_8R::_0,
            true => EDREQ_8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_8R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_9R {
    #[doc = "Disable asynchronous DMA request for channel 9."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 9."]
    _1,
}
impl EDREQ_9R {
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
            EDREQ_9R::_0 => false,
            EDREQ_9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_9R {
        match value {
            false => EDREQ_9R::_0,
            true => EDREQ_9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_9R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_10R {
    #[doc = "Disable asynchronous DMA request for channel 10."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 10."]
    _1,
}
impl EDREQ_10R {
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
            EDREQ_10R::_0 => false,
            EDREQ_10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_10R {
        match value {
            false => EDREQ_10R::_0,
            true => EDREQ_10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_10R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_11R {
    #[doc = "Disable asynchronous DMA request for channel 11."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 11."]
    _1,
}
impl EDREQ_11R {
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
            EDREQ_11R::_0 => false,
            EDREQ_11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_11R {
        match value {
            false => EDREQ_11R::_0,
            true => EDREQ_11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_11R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_12R {
    #[doc = "Disable asynchronous DMA request for channel 12."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 12."]
    _1,
}
impl EDREQ_12R {
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
            EDREQ_12R::_0 => false,
            EDREQ_12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_12R {
        match value {
            false => EDREQ_12R::_0,
            true => EDREQ_12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_12R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_13R {
    #[doc = "Disable asynchronous DMA request for channel 13."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 13."]
    _1,
}
impl EDREQ_13R {
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
            EDREQ_13R::_0 => false,
            EDREQ_13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_13R {
        match value {
            false => EDREQ_13R::_0,
            true => EDREQ_13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_13R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_14R {
    #[doc = "Disable asynchronous DMA request for channel 14."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 14."]
    _1,
}
impl EDREQ_14R {
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
            EDREQ_14R::_0 => false,
            EDREQ_14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_14R {
        match value {
            false => EDREQ_14R::_0,
            true => EDREQ_14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_14R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_15R {
    #[doc = "Disable asynchronous DMA request for channel 15."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 15."]
    _1,
}
impl EDREQ_15R {
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
            EDREQ_15R::_0 => false,
            EDREQ_15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_15R {
        match value {
            false => EDREQ_15R::_0,
            true => EDREQ_15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_15R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_16R {
    #[doc = "Disable asynchronous DMA request for channel 16"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 16"]
    _1,
}
impl EDREQ_16R {
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
            EDREQ_16R::_0 => false,
            EDREQ_16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_16R {
        match value {
            false => EDREQ_16R::_0,
            true => EDREQ_16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_16R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_17R {
    #[doc = "Disable asynchronous DMA request for channel 17"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 17"]
    _1,
}
impl EDREQ_17R {
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
            EDREQ_17R::_0 => false,
            EDREQ_17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_17R {
        match value {
            false => EDREQ_17R::_0,
            true => EDREQ_17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_17R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_18R {
    #[doc = "Disable asynchronous DMA request for channel 18"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 18"]
    _1,
}
impl EDREQ_18R {
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
            EDREQ_18R::_0 => false,
            EDREQ_18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_18R {
        match value {
            false => EDREQ_18R::_0,
            true => EDREQ_18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_18R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_19R {
    #[doc = "Disable asynchronous DMA request for channel 19"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 19"]
    _1,
}
impl EDREQ_19R {
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
            EDREQ_19R::_0 => false,
            EDREQ_19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_19R {
        match value {
            false => EDREQ_19R::_0,
            true => EDREQ_19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_19R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_20R {
    #[doc = "Disable asynchronous DMA request for channel 20"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 20"]
    _1,
}
impl EDREQ_20R {
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
            EDREQ_20R::_0 => false,
            EDREQ_20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_20R {
        match value {
            false => EDREQ_20R::_0,
            true => EDREQ_20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_20R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_21R {
    #[doc = "Disable asynchronous DMA request for channel 21"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 21"]
    _1,
}
impl EDREQ_21R {
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
            EDREQ_21R::_0 => false,
            EDREQ_21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_21R {
        match value {
            false => EDREQ_21R::_0,
            true => EDREQ_21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_21R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_22R {
    #[doc = "Disable asynchronous DMA request for channel 22"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 22"]
    _1,
}
impl EDREQ_22R {
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
            EDREQ_22R::_0 => false,
            EDREQ_22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_22R {
        match value {
            false => EDREQ_22R::_0,
            true => EDREQ_22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_22R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_23R {
    #[doc = "Disable asynchronous DMA request for channel 23"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 23"]
    _1,
}
impl EDREQ_23R {
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
            EDREQ_23R::_0 => false,
            EDREQ_23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_23R {
        match value {
            false => EDREQ_23R::_0,
            true => EDREQ_23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_23R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_24R {
    #[doc = "Disable asynchronous DMA request for channel 24"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 24"]
    _1,
}
impl EDREQ_24R {
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
            EDREQ_24R::_0 => false,
            EDREQ_24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_24R {
        match value {
            false => EDREQ_24R::_0,
            true => EDREQ_24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_24R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_25R {
    #[doc = "Disable asynchronous DMA request for channel 25"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 25"]
    _1,
}
impl EDREQ_25R {
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
            EDREQ_25R::_0 => false,
            EDREQ_25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_25R {
        match value {
            false => EDREQ_25R::_0,
            true => EDREQ_25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_25R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_26R {
    #[doc = "Disable asynchronous DMA request for channel 26"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 26"]
    _1,
}
impl EDREQ_26R {
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
            EDREQ_26R::_0 => false,
            EDREQ_26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_26R {
        match value {
            false => EDREQ_26R::_0,
            true => EDREQ_26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_26R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_27R {
    #[doc = "Disable asynchronous DMA request for channel 27"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 27"]
    _1,
}
impl EDREQ_27R {
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
            EDREQ_27R::_0 => false,
            EDREQ_27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_27R {
        match value {
            false => EDREQ_27R::_0,
            true => EDREQ_27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_27R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_28R {
    #[doc = "Disable asynchronous DMA request for channel 28"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 28"]
    _1,
}
impl EDREQ_28R {
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
            EDREQ_28R::_0 => false,
            EDREQ_28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_28R {
        match value {
            false => EDREQ_28R::_0,
            true => EDREQ_28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_28R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_29R {
    #[doc = "Disable asynchronous DMA request for channel 29"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 29"]
    _1,
}
impl EDREQ_29R {
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
            EDREQ_29R::_0 => false,
            EDREQ_29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_29R {
        match value {
            false => EDREQ_29R::_0,
            true => EDREQ_29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_29R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_30R {
    #[doc = "Disable asynchronous DMA request for channel 30"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 30"]
    _1,
}
impl EDREQ_30R {
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
            EDREQ_30R::_0 => false,
            EDREQ_30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_30R {
        match value {
            false => EDREQ_30R::_0,
            true => EDREQ_30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_30R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_31R {
    #[doc = "Disable asynchronous DMA request for channel 31"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 31"]
    _1,
}
impl EDREQ_31R {
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
            EDREQ_31R::_0 => false,
            EDREQ_31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_31R {
        match value {
            false => EDREQ_31R::_0,
            true => EDREQ_31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_31R::_1
    }
}
#[doc = "Values that can be written to the field `EDREQ_0`"]
pub enum EDREQ_0W {
    #[doc = "Disable asynchronous DMA request for channel 0."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 0."]
    _1,
}
impl EDREQ_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_0W::_0 => false,
            EDREQ_0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_0W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_0W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_0W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_1`"]
pub enum EDREQ_1W {
    #[doc = "Disable asynchronous DMA request for channel 1"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 1."]
    _1,
}
impl EDREQ_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_1W::_0 => false,
            EDREQ_1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_1W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_1W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_1W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_2`"]
pub enum EDREQ_2W {
    #[doc = "Disable asynchronous DMA request for channel 2."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 2."]
    _1,
}
impl EDREQ_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_2W::_0 => false,
            EDREQ_2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_2W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 2."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_2W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 2."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_2W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_3`"]
pub enum EDREQ_3W {
    #[doc = "Disable asynchronous DMA request for channel 3."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 3."]
    _1,
}
impl EDREQ_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_3W::_0 => false,
            EDREQ_3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_3W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 3."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_3W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 3."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_3W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_4`"]
pub enum EDREQ_4W {
    #[doc = "Disable asynchronous DMA request for channel 4."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 4."]
    _1,
}
impl EDREQ_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_4W::_0 => false,
            EDREQ_4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_4W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 4."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_4W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 4."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_4W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_5`"]
pub enum EDREQ_5W {
    #[doc = "Disable asynchronous DMA request for channel 5."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 5."]
    _1,
}
impl EDREQ_5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_5W::_0 => false,
            EDREQ_5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_5W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 5."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_5W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 5."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_5W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_6`"]
pub enum EDREQ_6W {
    #[doc = "Disable asynchronous DMA request for channel 6."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 6."]
    _1,
}
impl EDREQ_6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_6W::_0 => false,
            EDREQ_6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_6W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 6."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_6W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 6."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_6W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_7`"]
pub enum EDREQ_7W {
    #[doc = "Disable asynchronous DMA request for channel 7."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 7."]
    _1,
}
impl EDREQ_7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_7W::_0 => false,
            EDREQ_7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_7W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 7."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_7W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 7."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_7W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_8`"]
pub enum EDREQ_8W {
    #[doc = "Disable asynchronous DMA request for channel 8."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 8."]
    _1,
}
impl EDREQ_8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_8W::_0 => false,
            EDREQ_8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_8W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 8."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_8W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 8."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_8W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_9`"]
pub enum EDREQ_9W {
    #[doc = "Disable asynchronous DMA request for channel 9."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 9."]
    _1,
}
impl EDREQ_9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_9W::_0 => false,
            EDREQ_9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_9W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 9."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_9W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 9."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_9W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_10`"]
pub enum EDREQ_10W {
    #[doc = "Disable asynchronous DMA request for channel 10."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 10."]
    _1,
}
impl EDREQ_10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_10W::_0 => false,
            EDREQ_10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_10W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 10."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_10W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 10."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_10W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_11`"]
pub enum EDREQ_11W {
    #[doc = "Disable asynchronous DMA request for channel 11."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 11."]
    _1,
}
impl EDREQ_11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_11W::_0 => false,
            EDREQ_11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_11W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 11."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_11W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 11."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_11W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_12`"]
pub enum EDREQ_12W {
    #[doc = "Disable asynchronous DMA request for channel 12."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 12."]
    _1,
}
impl EDREQ_12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_12W::_0 => false,
            EDREQ_12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_12W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 12."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_12W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 12."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_12W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_13`"]
pub enum EDREQ_13W {
    #[doc = "Disable asynchronous DMA request for channel 13."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 13."]
    _1,
}
impl EDREQ_13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_13W::_0 => false,
            EDREQ_13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_13W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 13."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_13W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 13."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_13W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_14`"]
pub enum EDREQ_14W {
    #[doc = "Disable asynchronous DMA request for channel 14."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 14."]
    _1,
}
impl EDREQ_14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_14W::_0 => false,
            EDREQ_14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_14W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 14."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_14W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 14."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_14W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_15`"]
pub enum EDREQ_15W {
    #[doc = "Disable asynchronous DMA request for channel 15."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 15."]
    _1,
}
impl EDREQ_15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_15W::_0 => false,
            EDREQ_15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_15W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 15."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_15W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 15."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_15W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_16`"]
pub enum EDREQ_16W {
    #[doc = "Disable asynchronous DMA request for channel 16"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 16"]
    _1,
}
impl EDREQ_16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_16W::_0 => false,
            EDREQ_16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_16W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 16"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_16W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 16"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_16W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_17`"]
pub enum EDREQ_17W {
    #[doc = "Disable asynchronous DMA request for channel 17"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 17"]
    _1,
}
impl EDREQ_17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_17W::_0 => false,
            EDREQ_17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_17W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 17"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_17W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 17"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_17W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_18`"]
pub enum EDREQ_18W {
    #[doc = "Disable asynchronous DMA request for channel 18"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 18"]
    _1,
}
impl EDREQ_18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_18W::_0 => false,
            EDREQ_18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_18W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 18"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_18W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 18"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_18W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_19`"]
pub enum EDREQ_19W {
    #[doc = "Disable asynchronous DMA request for channel 19"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 19"]
    _1,
}
impl EDREQ_19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_19W::_0 => false,
            EDREQ_19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_19W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 19"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_19W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 19"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_19W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_20`"]
pub enum EDREQ_20W {
    #[doc = "Disable asynchronous DMA request for channel 20"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 20"]
    _1,
}
impl EDREQ_20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_20W::_0 => false,
            EDREQ_20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_20W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 20"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_20W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 20"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_20W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_21`"]
pub enum EDREQ_21W {
    #[doc = "Disable asynchronous DMA request for channel 21"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 21"]
    _1,
}
impl EDREQ_21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_21W::_0 => false,
            EDREQ_21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_21W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 21"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_21W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 21"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_21W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_22`"]
pub enum EDREQ_22W {
    #[doc = "Disable asynchronous DMA request for channel 22"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 22"]
    _1,
}
impl EDREQ_22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_22W::_0 => false,
            EDREQ_22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_22W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 22"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_22W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 22"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_22W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_23`"]
pub enum EDREQ_23W {
    #[doc = "Disable asynchronous DMA request for channel 23"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 23"]
    _1,
}
impl EDREQ_23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_23W::_0 => false,
            EDREQ_23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_23W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 23"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_23W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 23"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_23W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_24`"]
pub enum EDREQ_24W {
    #[doc = "Disable asynchronous DMA request for channel 24"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 24"]
    _1,
}
impl EDREQ_24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_24W::_0 => false,
            EDREQ_24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_24W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 24"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_24W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 24"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_24W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_25`"]
pub enum EDREQ_25W {
    #[doc = "Disable asynchronous DMA request for channel 25"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 25"]
    _1,
}
impl EDREQ_25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_25W::_0 => false,
            EDREQ_25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_25W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 25"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_25W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 25"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_25W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_26`"]
pub enum EDREQ_26W {
    #[doc = "Disable asynchronous DMA request for channel 26"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 26"]
    _1,
}
impl EDREQ_26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_26W::_0 => false,
            EDREQ_26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_26W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 26"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_26W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 26"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_26W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_27`"]
pub enum EDREQ_27W {
    #[doc = "Disable asynchronous DMA request for channel 27"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 27"]
    _1,
}
impl EDREQ_27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_27W::_0 => false,
            EDREQ_27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_27W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 27"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_27W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 27"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_27W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_28`"]
pub enum EDREQ_28W {
    #[doc = "Disable asynchronous DMA request for channel 28"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 28"]
    _1,
}
impl EDREQ_28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_28W::_0 => false,
            EDREQ_28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_28W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 28"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_28W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 28"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_28W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_29`"]
pub enum EDREQ_29W {
    #[doc = "Disable asynchronous DMA request for channel 29"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 29"]
    _1,
}
impl EDREQ_29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_29W::_0 => false,
            EDREQ_29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_29W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 29"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_29W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 29"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_29W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_30`"]
pub enum EDREQ_30W {
    #[doc = "Disable asynchronous DMA request for channel 30"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 30"]
    _1,
}
impl EDREQ_30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_30W::_0 => false,
            EDREQ_30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_30W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 30"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_30W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 30"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_30W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_31`"]
pub enum EDREQ_31W {
    #[doc = "Disable asynchronous DMA request for channel 31"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 31"]
    _1,
}
impl EDREQ_31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_31W::_0 => false,
            EDREQ_31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_31W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 31"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_31W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 31"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_31W::_1)
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
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline]
    pub fn edreq_0(&self) -> EDREQ_0R {
        EDREQ_0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline]
    pub fn edreq_1(&self) -> EDREQ_1R {
        EDREQ_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline]
    pub fn edreq_2(&self) -> EDREQ_2R {
        EDREQ_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline]
    pub fn edreq_3(&self) -> EDREQ_3R {
        EDREQ_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable asynchronous DMA request in stop mode for channel 4"]
    #[inline]
    pub fn edreq_4(&self) -> EDREQ_4R {
        EDREQ_4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable asynchronous DMA request in stop mode for channel 5"]
    #[inline]
    pub fn edreq_5(&self) -> EDREQ_5R {
        EDREQ_5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable asynchronous DMA request in stop mode for channel 6"]
    #[inline]
    pub fn edreq_6(&self) -> EDREQ_6R {
        EDREQ_6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable asynchronous DMA request in stop mode for channel 7"]
    #[inline]
    pub fn edreq_7(&self) -> EDREQ_7R {
        EDREQ_7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable asynchronous DMA request in stop mode for channel 8"]
    #[inline]
    pub fn edreq_8(&self) -> EDREQ_8R {
        EDREQ_8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable asynchronous DMA request in stop mode for channel 9"]
    #[inline]
    pub fn edreq_9(&self) -> EDREQ_9R {
        EDREQ_9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable asynchronous DMA request in stop mode for channel 10"]
    #[inline]
    pub fn edreq_10(&self) -> EDREQ_10R {
        EDREQ_10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable asynchronous DMA request in stop mode for channel 11"]
    #[inline]
    pub fn edreq_11(&self) -> EDREQ_11R {
        EDREQ_11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable asynchronous DMA request in stop mode for channel 12"]
    #[inline]
    pub fn edreq_12(&self) -> EDREQ_12R {
        EDREQ_12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable asynchronous DMA request in stop mode for channel 13"]
    #[inline]
    pub fn edreq_13(&self) -> EDREQ_13R {
        EDREQ_13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enable asynchronous DMA request in stop mode for channel 14"]
    #[inline]
    pub fn edreq_14(&self) -> EDREQ_14R {
        EDREQ_14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enable asynchronous DMA request in stop mode for channel 15"]
    #[inline]
    pub fn edreq_15(&self) -> EDREQ_15R {
        EDREQ_15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable asynchronous DMA request in stop mode for channel 16"]
    #[inline]
    pub fn edreq_16(&self) -> EDREQ_16R {
        EDREQ_16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enable asynchronous DMA request in stop mode for channel 17"]
    #[inline]
    pub fn edreq_17(&self) -> EDREQ_17R {
        EDREQ_17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Enable asynchronous DMA request in stop mode for channel 18"]
    #[inline]
    pub fn edreq_18(&self) -> EDREQ_18R {
        EDREQ_18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enable asynchronous DMA request in stop mode for channel 19"]
    #[inline]
    pub fn edreq_19(&self) -> EDREQ_19R {
        EDREQ_19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enable asynchronous DMA request in stop mode for channel 20"]
    #[inline]
    pub fn edreq_20(&self) -> EDREQ_20R {
        EDREQ_20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Enable asynchronous DMA request in stop mode for channel 21"]
    #[inline]
    pub fn edreq_21(&self) -> EDREQ_21R {
        EDREQ_21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Enable asynchronous DMA request in stop mode for channel 22"]
    #[inline]
    pub fn edreq_22(&self) -> EDREQ_22R {
        EDREQ_22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enable asynchronous DMA request in stop mode for channel 23"]
    #[inline]
    pub fn edreq_23(&self) -> EDREQ_23R {
        EDREQ_23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable asynchronous DMA request in stop mode for channel 24"]
    #[inline]
    pub fn edreq_24(&self) -> EDREQ_24R {
        EDREQ_24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enable asynchronous DMA request in stop mode for channel 25"]
    #[inline]
    pub fn edreq_25(&self) -> EDREQ_25R {
        EDREQ_25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enable asynchronous DMA request in stop mode for channel 26"]
    #[inline]
    pub fn edreq_26(&self) -> EDREQ_26R {
        EDREQ_26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enable asynchronous DMA request in stop mode for channel 27"]
    #[inline]
    pub fn edreq_27(&self) -> EDREQ_27R {
        EDREQ_27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Enable asynchronous DMA request in stop mode for channel 28"]
    #[inline]
    pub fn edreq_28(&self) -> EDREQ_28R {
        EDREQ_28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Enable asynchronous DMA request in stop mode for channel 29"]
    #[inline]
    pub fn edreq_29(&self) -> EDREQ_29R {
        EDREQ_29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Enable asynchronous DMA request in stop mode for channel 30"]
    #[inline]
    pub fn edreq_30(&self) -> EDREQ_30R {
        EDREQ_30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Enable asynchronous DMA request in stop mode for channel 31"]
    #[inline]
    pub fn edreq_31(&self) -> EDREQ_31R {
        EDREQ_31R::_from({
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
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline]
    pub fn edreq_0(&mut self) -> _EDREQ_0W {
        _EDREQ_0W { w: self }
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline]
    pub fn edreq_1(&mut self) -> _EDREQ_1W {
        _EDREQ_1W { w: self }
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline]
    pub fn edreq_2(&mut self) -> _EDREQ_2W {
        _EDREQ_2W { w: self }
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline]
    pub fn edreq_3(&mut self) -> _EDREQ_3W {
        _EDREQ_3W { w: self }
    }
    #[doc = "Bit 4 - Enable asynchronous DMA request in stop mode for channel 4"]
    #[inline]
    pub fn edreq_4(&mut self) -> _EDREQ_4W {
        _EDREQ_4W { w: self }
    }
    #[doc = "Bit 5 - Enable asynchronous DMA request in stop mode for channel 5"]
    #[inline]
    pub fn edreq_5(&mut self) -> _EDREQ_5W {
        _EDREQ_5W { w: self }
    }
    #[doc = "Bit 6 - Enable asynchronous DMA request in stop mode for channel 6"]
    #[inline]
    pub fn edreq_6(&mut self) -> _EDREQ_6W {
        _EDREQ_6W { w: self }
    }
    #[doc = "Bit 7 - Enable asynchronous DMA request in stop mode for channel 7"]
    #[inline]
    pub fn edreq_7(&mut self) -> _EDREQ_7W {
        _EDREQ_7W { w: self }
    }
    #[doc = "Bit 8 - Enable asynchronous DMA request in stop mode for channel 8"]
    #[inline]
    pub fn edreq_8(&mut self) -> _EDREQ_8W {
        _EDREQ_8W { w: self }
    }
    #[doc = "Bit 9 - Enable asynchronous DMA request in stop mode for channel 9"]
    #[inline]
    pub fn edreq_9(&mut self) -> _EDREQ_9W {
        _EDREQ_9W { w: self }
    }
    #[doc = "Bit 10 - Enable asynchronous DMA request in stop mode for channel 10"]
    #[inline]
    pub fn edreq_10(&mut self) -> _EDREQ_10W {
        _EDREQ_10W { w: self }
    }
    #[doc = "Bit 11 - Enable asynchronous DMA request in stop mode for channel 11"]
    #[inline]
    pub fn edreq_11(&mut self) -> _EDREQ_11W {
        _EDREQ_11W { w: self }
    }
    #[doc = "Bit 12 - Enable asynchronous DMA request in stop mode for channel 12"]
    #[inline]
    pub fn edreq_12(&mut self) -> _EDREQ_12W {
        _EDREQ_12W { w: self }
    }
    #[doc = "Bit 13 - Enable asynchronous DMA request in stop mode for channel 13"]
    #[inline]
    pub fn edreq_13(&mut self) -> _EDREQ_13W {
        _EDREQ_13W { w: self }
    }
    #[doc = "Bit 14 - Enable asynchronous DMA request in stop mode for channel 14"]
    #[inline]
    pub fn edreq_14(&mut self) -> _EDREQ_14W {
        _EDREQ_14W { w: self }
    }
    #[doc = "Bit 15 - Enable asynchronous DMA request in stop mode for channel 15"]
    #[inline]
    pub fn edreq_15(&mut self) -> _EDREQ_15W {
        _EDREQ_15W { w: self }
    }
    #[doc = "Bit 16 - Enable asynchronous DMA request in stop mode for channel 16"]
    #[inline]
    pub fn edreq_16(&mut self) -> _EDREQ_16W {
        _EDREQ_16W { w: self }
    }
    #[doc = "Bit 17 - Enable asynchronous DMA request in stop mode for channel 17"]
    #[inline]
    pub fn edreq_17(&mut self) -> _EDREQ_17W {
        _EDREQ_17W { w: self }
    }
    #[doc = "Bit 18 - Enable asynchronous DMA request in stop mode for channel 18"]
    #[inline]
    pub fn edreq_18(&mut self) -> _EDREQ_18W {
        _EDREQ_18W { w: self }
    }
    #[doc = "Bit 19 - Enable asynchronous DMA request in stop mode for channel 19"]
    #[inline]
    pub fn edreq_19(&mut self) -> _EDREQ_19W {
        _EDREQ_19W { w: self }
    }
    #[doc = "Bit 20 - Enable asynchronous DMA request in stop mode for channel 20"]
    #[inline]
    pub fn edreq_20(&mut self) -> _EDREQ_20W {
        _EDREQ_20W { w: self }
    }
    #[doc = "Bit 21 - Enable asynchronous DMA request in stop mode for channel 21"]
    #[inline]
    pub fn edreq_21(&mut self) -> _EDREQ_21W {
        _EDREQ_21W { w: self }
    }
    #[doc = "Bit 22 - Enable asynchronous DMA request in stop mode for channel 22"]
    #[inline]
    pub fn edreq_22(&mut self) -> _EDREQ_22W {
        _EDREQ_22W { w: self }
    }
    #[doc = "Bit 23 - Enable asynchronous DMA request in stop mode for channel 23"]
    #[inline]
    pub fn edreq_23(&mut self) -> _EDREQ_23W {
        _EDREQ_23W { w: self }
    }
    #[doc = "Bit 24 - Enable asynchronous DMA request in stop mode for channel 24"]
    #[inline]
    pub fn edreq_24(&mut self) -> _EDREQ_24W {
        _EDREQ_24W { w: self }
    }
    #[doc = "Bit 25 - Enable asynchronous DMA request in stop mode for channel 25"]
    #[inline]
    pub fn edreq_25(&mut self) -> _EDREQ_25W {
        _EDREQ_25W { w: self }
    }
    #[doc = "Bit 26 - Enable asynchronous DMA request in stop mode for channel 26"]
    #[inline]
    pub fn edreq_26(&mut self) -> _EDREQ_26W {
        _EDREQ_26W { w: self }
    }
    #[doc = "Bit 27 - Enable asynchronous DMA request in stop mode for channel 27"]
    #[inline]
    pub fn edreq_27(&mut self) -> _EDREQ_27W {
        _EDREQ_27W { w: self }
    }
    #[doc = "Bit 28 - Enable asynchronous DMA request in stop mode for channel 28"]
    #[inline]
    pub fn edreq_28(&mut self) -> _EDREQ_28W {
        _EDREQ_28W { w: self }
    }
    #[doc = "Bit 29 - Enable asynchronous DMA request in stop mode for channel 29"]
    #[inline]
    pub fn edreq_29(&mut self) -> _EDREQ_29W {
        _EDREQ_29W { w: self }
    }
    #[doc = "Bit 30 - Enable asynchronous DMA request in stop mode for channel 30"]
    #[inline]
    pub fn edreq_30(&mut self) -> _EDREQ_30W {
        _EDREQ_30W { w: self }
    }
    #[doc = "Bit 31 - Enable asynchronous DMA request in stop mode for channel 31"]
    #[inline]
    pub fn edreq_31(&mut self) -> _EDREQ_31W {
        _EDREQ_31W { w: self }
    }
}
