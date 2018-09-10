#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT8 {
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
#[doc = "Possible values of the field `FTM0SYNCBIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0SYNCBITR {
    #[doc = "No effect"]
    _0,
    #[doc = "Write 1 to assert the TRIG0 input to FTM0, software must clear this bit to allow other trigger sources to assert."]
    _1,
}
impl FTM0SYNCBITR {
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
            FTM0SYNCBITR::_0 => false,
            FTM0SYNCBITR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0SYNCBITR {
        match value {
            false => FTM0SYNCBITR::_0,
            true => FTM0SYNCBITR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0SYNCBITR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0SYNCBITR::_1
    }
}
#[doc = "Possible values of the field `FTM1SYNCBIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1SYNCBITR {
    #[doc = "No effect."]
    _0,
    #[doc = "Write 1 to assert the TRIG0 input to FTM1, software must clear this bit to allow other trigger sources to assert."]
    _1,
}
impl FTM1SYNCBITR {
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
            FTM1SYNCBITR::_0 => false,
            FTM1SYNCBITR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM1SYNCBITR {
        match value {
            false => FTM1SYNCBITR::_0,
            true => FTM1SYNCBITR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM1SYNCBITR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM1SYNCBITR::_1
    }
}
#[doc = "Possible values of the field `FTM2SYNCBIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2SYNCBITR {
    #[doc = "No effect."]
    _0,
    #[doc = "Write 1 to assert the TRIG0 input to FTM2, software must clear this bit to allow other trigger sources to assert."]
    _1,
}
impl FTM2SYNCBITR {
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
            FTM2SYNCBITR::_0 => false,
            FTM2SYNCBITR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM2SYNCBITR {
        match value {
            false => FTM2SYNCBITR::_0,
            true => FTM2SYNCBITR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM2SYNCBITR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM2SYNCBITR::_1
    }
}
#[doc = "Possible values of the field `FTM3SYNCBIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3SYNCBITR {
    #[doc = "No effect."]
    _0,
    #[doc = "Write 1 to assert the TRIG0 input to FTM3, software must clear this bit to allow other trigger sources to assert."]
    _1,
}
impl FTM3SYNCBITR {
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
            FTM3SYNCBITR::_0 => false,
            FTM3SYNCBITR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3SYNCBITR {
        match value {
            false => FTM3SYNCBITR::_0,
            true => FTM3SYNCBITR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3SYNCBITR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3SYNCBITR::_1
    }
}
#[doc = "Possible values of the field `FTM0OCH0SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH0SRCR {
    #[doc = "FTM0_CH0 pin is output of FTM0 channel 0 output"]
    _0,
    #[doc = "FTM0_CH0 pin is output of FTM0 channel 0 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH0SRCR {
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
            FTM0OCH0SRCR::_0 => false,
            FTM0OCH0SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0OCH0SRCR {
        match value {
            false => FTM0OCH0SRCR::_0,
            true => FTM0OCH0SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH0SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH0SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM0OCH1SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH1SRCR {
    #[doc = "FTM0_CH1 pin is output of FTM0 channel 1 output"]
    _0,
    #[doc = "FTM0_CH1 pin is output of FTM0 channel 1 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH1SRCR {
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
            FTM0OCH1SRCR::_0 => false,
            FTM0OCH1SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0OCH1SRCR {
        match value {
            false => FTM0OCH1SRCR::_0,
            true => FTM0OCH1SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH1SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH1SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM0OCH2SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH2SRCR {
    #[doc = "FTM0_CH2 pin is output of FTM0 channel 2 output"]
    _0,
    #[doc = "FTM0_CH2 pin is output of FTM0 channel 2 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH2SRCR {
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
            FTM0OCH2SRCR::_0 => false,
            FTM0OCH2SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0OCH2SRCR {
        match value {
            false => FTM0OCH2SRCR::_0,
            true => FTM0OCH2SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH2SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH2SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM0OCH3SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH3SRCR {
    #[doc = "FTM0_CH3 pin is output of FTM0 channel 3 output"]
    _0,
    #[doc = "FTM0_CH3 pin is output of FTM0 channel 3 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH3SRCR {
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
            FTM0OCH3SRCR::_0 => false,
            FTM0OCH3SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0OCH3SRCR {
        match value {
            false => FTM0OCH3SRCR::_0,
            true => FTM0OCH3SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH3SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH3SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM0OCH4SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH4SRCR {
    #[doc = "FTM0_CH4 pin is output of FTM0 channel 4 output"]
    _0,
    #[doc = "FTM0_CH4 pin is output of FTM0 channel 4 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH4SRCR {
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
            FTM0OCH4SRCR::_0 => false,
            FTM0OCH4SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0OCH4SRCR {
        match value {
            false => FTM0OCH4SRCR::_0,
            true => FTM0OCH4SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH4SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH4SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM0OCH5SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH5SRCR {
    #[doc = "FTM0_CH5 pin is output of FTM0 channel 5 output"]
    _0,
    #[doc = "FTM0_CH5 pin is output of FTM0 channel 5 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH5SRCR {
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
            FTM0OCH5SRCR::_0 => false,
            FTM0OCH5SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0OCH5SRCR {
        match value {
            false => FTM0OCH5SRCR::_0,
            true => FTM0OCH5SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH5SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH5SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM0OCH6SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH6SRCR {
    #[doc = "FTM0_CH6 pin is output of FTM0 channel 6 output"]
    _0,
    #[doc = "FTM0_CH6 pin is output of FTM0 channel 6 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH6SRCR {
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
            FTM0OCH6SRCR::_0 => false,
            FTM0OCH6SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0OCH6SRCR {
        match value {
            false => FTM0OCH6SRCR::_0,
            true => FTM0OCH6SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH6SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH6SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM0OCH7SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0OCH7SRCR {
    #[doc = "FTM0_CH7 pin is output of FTM0 channel 7 output"]
    _0,
    #[doc = "FTM0_CH7 pin is output of FTM0 channel 7 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH7SRCR {
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
            FTM0OCH7SRCR::_0 => false,
            FTM0OCH7SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0OCH7SRCR {
        match value {
            false => FTM0OCH7SRCR::_0,
            true => FTM0OCH7SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH7SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH7SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM3OCH0SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH0SRCR {
    #[doc = "FTM3_CH0 pin is output of FTM3 channel 0 output"]
    _0,
    #[doc = "FTM3_CH0 pin is output of FTM3 channel 0 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH0SRCR {
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
            FTM3OCH0SRCR::_0 => false,
            FTM3OCH0SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3OCH0SRCR {
        match value {
            false => FTM3OCH0SRCR::_0,
            true => FTM3OCH0SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH0SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH0SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM3OCH1SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH1SRCR {
    #[doc = "FTM3_CH1 pin is output of FTM3 channel 1 output"]
    _0,
    #[doc = "FTM3_CH1 pin is output of FTM3 channel 1 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH1SRCR {
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
            FTM3OCH1SRCR::_0 => false,
            FTM3OCH1SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3OCH1SRCR {
        match value {
            false => FTM3OCH1SRCR::_0,
            true => FTM3OCH1SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH1SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH1SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM3OCH2SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH2SRCR {
    #[doc = "FTM3_CH2 pin is output of FTM3 channel 2 output"]
    _0,
    #[doc = "FTM3_CH2 pin is output of FTM3 channel 2 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH2SRCR {
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
            FTM3OCH2SRCR::_0 => false,
            FTM3OCH2SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3OCH2SRCR {
        match value {
            false => FTM3OCH2SRCR::_0,
            true => FTM3OCH2SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH2SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH2SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM3OCH3SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH3SRCR {
    #[doc = "FTM3_CH3 pin is output of FTM3 channel 3 output"]
    _0,
    #[doc = "FTM3_CH3 pin is output of FTM3 channel 3 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH3SRCR {
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
            FTM3OCH3SRCR::_0 => false,
            FTM3OCH3SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3OCH3SRCR {
        match value {
            false => FTM3OCH3SRCR::_0,
            true => FTM3OCH3SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH3SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH3SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM3OCH4SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH4SRCR {
    #[doc = "FTM3_CH4 pin is output of FTM3 channel 4 output"]
    _0,
    #[doc = "FTM3_CH4 pin is output of FTM3 channel 4 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH4SRCR {
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
            FTM3OCH4SRCR::_0 => false,
            FTM3OCH4SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3OCH4SRCR {
        match value {
            false => FTM3OCH4SRCR::_0,
            true => FTM3OCH4SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH4SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH4SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM3OCH5SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH5SRCR {
    #[doc = "FTM3_CH5 pin is output of FTM3 channel 5 output"]
    _0,
    #[doc = "FTM3_CH5 pin is output of FTM3 channel 5 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH5SRCR {
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
            FTM3OCH5SRCR::_0 => false,
            FTM3OCH5SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3OCH5SRCR {
        match value {
            false => FTM3OCH5SRCR::_0,
            true => FTM3OCH5SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH5SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH5SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM3OCH6SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH6SRCR {
    #[doc = "FTM3_CH6 pin is output of FTM3 channel 6 output"]
    _0,
    #[doc = "FTM3_CH6 pin is output of FTM3 channel 6 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH6SRCR {
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
            FTM3OCH6SRCR::_0 => false,
            FTM3OCH6SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3OCH6SRCR {
        match value {
            false => FTM3OCH6SRCR::_0,
            true => FTM3OCH6SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH6SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH6SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM3OCH7SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3OCH7SRCR {
    #[doc = "FTM3_CH7 pin is output of FTM3 channel 7 output"]
    _0,
    #[doc = "FTM3_CH7 pin is output of FTM3 channel 7 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH7SRCR {
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
            FTM3OCH7SRCR::_0 => false,
            FTM3OCH7SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3OCH7SRCR {
        match value {
            false => FTM3OCH7SRCR::_0,
            true => FTM3OCH7SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3OCH7SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3OCH7SRCR::_1
    }
}
#[doc = "Values that can be written to the field `FTM0SYNCBIT`"]
pub enum FTM0SYNCBITW {
    #[doc = "No effect"]
    _0,
    #[doc = "Write 1 to assert the TRIG0 input to FTM0, software must clear this bit to allow other trigger sources to assert."]
    _1,
}
impl FTM0SYNCBITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0SYNCBITW::_0 => false,
            FTM0SYNCBITW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0SYNCBITW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0SYNCBITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0SYNCBITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0SYNCBITW::_0)
    }
    #[doc = "Write 1 to assert the TRIG0 input to FTM0, software must clear this bit to allow other trigger sources to assert."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0SYNCBITW::_1)
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
#[doc = "Values that can be written to the field `FTM1SYNCBIT`"]
pub enum FTM1SYNCBITW {
    #[doc = "No effect."]
    _0,
    #[doc = "Write 1 to assert the TRIG0 input to FTM1, software must clear this bit to allow other trigger sources to assert."]
    _1,
}
impl FTM1SYNCBITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM1SYNCBITW::_0 => false,
            FTM1SYNCBITW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM1SYNCBITW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM1SYNCBITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM1SYNCBITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1SYNCBITW::_0)
    }
    #[doc = "Write 1 to assert the TRIG0 input to FTM1, software must clear this bit to allow other trigger sources to assert."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1SYNCBITW::_1)
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
#[doc = "Values that can be written to the field `FTM2SYNCBIT`"]
pub enum FTM2SYNCBITW {
    #[doc = "No effect."]
    _0,
    #[doc = "Write 1 to assert the TRIG0 input to FTM2, software must clear this bit to allow other trigger sources to assert."]
    _1,
}
impl FTM2SYNCBITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM2SYNCBITW::_0 => false,
            FTM2SYNCBITW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM2SYNCBITW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM2SYNCBITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM2SYNCBITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2SYNCBITW::_0)
    }
    #[doc = "Write 1 to assert the TRIG0 input to FTM2, software must clear this bit to allow other trigger sources to assert."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2SYNCBITW::_1)
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
#[doc = "Values that can be written to the field `FTM3SYNCBIT`"]
pub enum FTM3SYNCBITW {
    #[doc = "No effect."]
    _0,
    #[doc = "Write 1 to assert the TRIG0 input to FTM3, software must clear this bit to allow other trigger sources to assert."]
    _1,
}
impl FTM3SYNCBITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3SYNCBITW::_0 => false,
            FTM3SYNCBITW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3SYNCBITW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3SYNCBITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3SYNCBITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3SYNCBITW::_0)
    }
    #[doc = "Write 1 to assert the TRIG0 input to FTM3, software must clear this bit to allow other trigger sources to assert."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3SYNCBITW::_1)
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
#[doc = "Values that can be written to the field `FTM0OCH0SRC`"]
pub enum FTM0OCH0SRCW {
    #[doc = "FTM0_CH0 pin is output of FTM0 channel 0 output"]
    _0,
    #[doc = "FTM0_CH0 pin is output of FTM0 channel 0 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH0SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0OCH0SRCW::_0 => false,
            FTM0OCH0SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0OCH0SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0OCH0SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0OCH0SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM0_CH0 pin is output of FTM0 channel 0 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH0SRCW::_0)
    }
    #[doc = "FTM0_CH0 pin is output of FTM0 channel 0 output, modulated by FTM1 channel 1 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH0SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM0OCH1SRC`"]
pub enum FTM0OCH1SRCW {
    #[doc = "FTM0_CH1 pin is output of FTM0 channel 1 output"]
    _0,
    #[doc = "FTM0_CH1 pin is output of FTM0 channel 1 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH1SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0OCH1SRCW::_0 => false,
            FTM0OCH1SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0OCH1SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0OCH1SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0OCH1SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM0_CH1 pin is output of FTM0 channel 1 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH1SRCW::_0)
    }
    #[doc = "FTM0_CH1 pin is output of FTM0 channel 1 output, modulated by FTM1 channel 1 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH1SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM0OCH2SRC`"]
pub enum FTM0OCH2SRCW {
    #[doc = "FTM0_CH2 pin is output of FTM0 channel 2 output"]
    _0,
    #[doc = "FTM0_CH2 pin is output of FTM0 channel 2 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH2SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0OCH2SRCW::_0 => false,
            FTM0OCH2SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0OCH2SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0OCH2SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0OCH2SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM0_CH2 pin is output of FTM0 channel 2 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH2SRCW::_0)
    }
    #[doc = "FTM0_CH2 pin is output of FTM0 channel 2 output, modulated by FTM1 channel 1 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH2SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM0OCH3SRC`"]
pub enum FTM0OCH3SRCW {
    #[doc = "FTM0_CH3 pin is output of FTM0 channel 3 output"]
    _0,
    #[doc = "FTM0_CH3 pin is output of FTM0 channel 3 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH3SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0OCH3SRCW::_0 => false,
            FTM0OCH3SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0OCH3SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0OCH3SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0OCH3SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM0_CH3 pin is output of FTM0 channel 3 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH3SRCW::_0)
    }
    #[doc = "FTM0_CH3 pin is output of FTM0 channel 3 output, modulated by FTM1 channel 1 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH3SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM0OCH4SRC`"]
pub enum FTM0OCH4SRCW {
    #[doc = "FTM0_CH4 pin is output of FTM0 channel 4 output"]
    _0,
    #[doc = "FTM0_CH4 pin is output of FTM0 channel 4 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH4SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0OCH4SRCW::_0 => false,
            FTM0OCH4SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0OCH4SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0OCH4SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0OCH4SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM0_CH4 pin is output of FTM0 channel 4 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH4SRCW::_0)
    }
    #[doc = "FTM0_CH4 pin is output of FTM0 channel 4 output, modulated by FTM1 channel 1 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH4SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM0OCH5SRC`"]
pub enum FTM0OCH5SRCW {
    #[doc = "FTM0_CH5 pin is output of FTM0 channel 5 output"]
    _0,
    #[doc = "FTM0_CH5 pin is output of FTM0 channel 5 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH5SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0OCH5SRCW::_0 => false,
            FTM0OCH5SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0OCH5SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0OCH5SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0OCH5SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM0_CH5 pin is output of FTM0 channel 5 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH5SRCW::_0)
    }
    #[doc = "FTM0_CH5 pin is output of FTM0 channel 5 output, modulated by FTM1 channel 1 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH5SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM0OCH6SRC`"]
pub enum FTM0OCH6SRCW {
    #[doc = "FTM0_CH6 pin is output of FTM0 channel 6 output"]
    _0,
    #[doc = "FTM0_CH6 pin is output of FTM0 channel 6 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH6SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0OCH6SRCW::_0 => false,
            FTM0OCH6SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0OCH6SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0OCH6SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0OCH6SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM0_CH6 pin is output of FTM0 channel 6 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH6SRCW::_0)
    }
    #[doc = "FTM0_CH6 pin is output of FTM0 channel 6 output, modulated by FTM1 channel 1 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH6SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM0OCH7SRC`"]
pub enum FTM0OCH7SRCW {
    #[doc = "FTM0_CH7 pin is output of FTM0 channel 7 output"]
    _0,
    #[doc = "FTM0_CH7 pin is output of FTM0 channel 7 output, modulated by FTM1 channel 1 output"]
    _1,
}
impl FTM0OCH7SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0OCH7SRCW::_0 => false,
            FTM0OCH7SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0OCH7SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0OCH7SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0OCH7SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM0_CH7 pin is output of FTM0 channel 7 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH7SRCW::_0)
    }
    #[doc = "FTM0_CH7 pin is output of FTM0 channel 7 output, modulated by FTM1 channel 1 output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH7SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM3OCH0SRC`"]
pub enum FTM3OCH0SRCW {
    #[doc = "FTM3_CH0 pin is output of FTM3 channel 0 output"]
    _0,
    #[doc = "FTM3_CH0 pin is output of FTM3 channel 0 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH0SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3OCH0SRCW::_0 => false,
            FTM3OCH0SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3OCH0SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3OCH0SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3OCH0SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM3_CH0 pin is output of FTM3 channel 0 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH0SRCW::_0)
    }
    #[doc = "FTM3_CH0 pin is output of FTM3 channel 0 output modulated by FTM2 channel 1 output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH0SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM3OCH1SRC`"]
pub enum FTM3OCH1SRCW {
    #[doc = "FTM3_CH1 pin is output of FTM3 channel 1 output"]
    _0,
    #[doc = "FTM3_CH1 pin is output of FTM3 channel 1 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH1SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3OCH1SRCW::_0 => false,
            FTM3OCH1SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3OCH1SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3OCH1SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3OCH1SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM3_CH1 pin is output of FTM3 channel 1 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH1SRCW::_0)
    }
    #[doc = "FTM3_CH1 pin is output of FTM3 channel 1 output modulated by FTM2 channel 1 output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH1SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM3OCH2SRC`"]
pub enum FTM3OCH2SRCW {
    #[doc = "FTM3_CH2 pin is output of FTM3 channel 2 output"]
    _0,
    #[doc = "FTM3_CH2 pin is output of FTM3 channel 2 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH2SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3OCH2SRCW::_0 => false,
            FTM3OCH2SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3OCH2SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3OCH2SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3OCH2SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM3_CH2 pin is output of FTM3 channel 2 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH2SRCW::_0)
    }
    #[doc = "FTM3_CH2 pin is output of FTM3 channel 2 output modulated by FTM2 channel 1 output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH2SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM3OCH3SRC`"]
pub enum FTM3OCH3SRCW {
    #[doc = "FTM3_CH3 pin is output of FTM3 channel 3 output"]
    _0,
    #[doc = "FTM3_CH3 pin is output of FTM3 channel 3 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH3SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3OCH3SRCW::_0 => false,
            FTM3OCH3SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3OCH3SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3OCH3SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3OCH3SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM3_CH3 pin is output of FTM3 channel 3 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH3SRCW::_0)
    }
    #[doc = "FTM3_CH3 pin is output of FTM3 channel 3 output modulated by FTM2 channel 1 output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH3SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM3OCH4SRC`"]
pub enum FTM3OCH4SRCW {
    #[doc = "FTM3_CH4 pin is output of FTM3 channel 4 output"]
    _0,
    #[doc = "FTM3_CH4 pin is output of FTM3 channel 4 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH4SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3OCH4SRCW::_0 => false,
            FTM3OCH4SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3OCH4SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3OCH4SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3OCH4SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM3_CH4 pin is output of FTM3 channel 4 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH4SRCW::_0)
    }
    #[doc = "FTM3_CH4 pin is output of FTM3 channel 4 output modulated by FTM2 channel 1 output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH4SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM3OCH5SRC`"]
pub enum FTM3OCH5SRCW {
    #[doc = "FTM3_CH5 pin is output of FTM3 channel 5 output"]
    _0,
    #[doc = "FTM3_CH5 pin is output of FTM3 channel 5 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH5SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3OCH5SRCW::_0 => false,
            FTM3OCH5SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3OCH5SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3OCH5SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3OCH5SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM3_CH5 pin is output of FTM3 channel 5 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH5SRCW::_0)
    }
    #[doc = "FTM3_CH5 pin is output of FTM3 channel 5 output modulated by FTM2 channel 1 output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH5SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM3OCH6SRC`"]
pub enum FTM3OCH6SRCW {
    #[doc = "FTM3_CH6 pin is output of FTM3 channel 6 output"]
    _0,
    #[doc = "FTM3_CH6 pin is output of FTM3 channel 6 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH6SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3OCH6SRCW::_0 => false,
            FTM3OCH6SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3OCH6SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3OCH6SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3OCH6SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM3_CH6 pin is output of FTM3 channel 6 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH6SRCW::_0)
    }
    #[doc = "FTM3_CH6 pin is output of FTM3 channel 6 output modulated by FTM2 channel 1 output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH6SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM3OCH7SRC`"]
pub enum FTM3OCH7SRCW {
    #[doc = "FTM3_CH7 pin is output of FTM3 channel 7 output"]
    _0,
    #[doc = "FTM3_CH7 pin is output of FTM3 channel 7 output modulated by FTM2 channel 1 output."]
    _1,
}
impl FTM3OCH7SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3OCH7SRCW::_0 => false,
            FTM3OCH7SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3OCH7SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3OCH7SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3OCH7SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM3_CH7 pin is output of FTM3 channel 7 output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3OCH7SRCW::_0)
    }
    #[doc = "FTM3_CH7 pin is output of FTM3 channel 7 output modulated by FTM2 channel 1 output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3OCH7SRCW::_1)
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
    #[doc = "Bit 0 - FTM0 Hardware Trigger 0 Software Synchronization"]
    #[inline]
    pub fn ftm0syncbit(&self) -> FTM0SYNCBITR {
        FTM0SYNCBITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - FTM1 Hardware Trigger 0 Software Synchronization"]
    #[inline]
    pub fn ftm1syncbit(&self) -> FTM1SYNCBITR {
        FTM1SYNCBITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - FTM2 Hardware Trigger 0 Software Synchronization"]
    #[inline]
    pub fn ftm2syncbit(&self) -> FTM2SYNCBITR {
        FTM2SYNCBITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - FTM3 Hardware Trigger 0 Software Synchronization"]
    #[inline]
    pub fn ftm3syncbit(&self) -> FTM3SYNCBITR {
        FTM3SYNCBITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - FTM0 channel 0 output source"]
    #[inline]
    pub fn ftm0och0src(&self) -> FTM0OCH0SRCR {
        FTM0OCH0SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - FTM0 channel 1 output source"]
    #[inline]
    pub fn ftm0och1src(&self) -> FTM0OCH1SRCR {
        FTM0OCH1SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - FTM0 channel 2 output source"]
    #[inline]
    pub fn ftm0och2src(&self) -> FTM0OCH2SRCR {
        FTM0OCH2SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - FTM0 channel 3 output source"]
    #[inline]
    pub fn ftm0och3src(&self) -> FTM0OCH3SRCR {
        FTM0OCH3SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - FTM0 channel 4 output source"]
    #[inline]
    pub fn ftm0och4src(&self) -> FTM0OCH4SRCR {
        FTM0OCH4SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - FTM0 channel 5 output source"]
    #[inline]
    pub fn ftm0och5src(&self) -> FTM0OCH5SRCR {
        FTM0OCH5SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - FTM0 channel 6 output source"]
    #[inline]
    pub fn ftm0och6src(&self) -> FTM0OCH6SRCR {
        FTM0OCH6SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - FTM0 channel 7 output source"]
    #[inline]
    pub fn ftm0och7src(&self) -> FTM0OCH7SRCR {
        FTM0OCH7SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - FTM3 channel 0 output source"]
    #[inline]
    pub fn ftm3och0src(&self) -> FTM3OCH0SRCR {
        FTM3OCH0SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - FTM3 channel 1 output source"]
    #[inline]
    pub fn ftm3och1src(&self) -> FTM3OCH1SRCR {
        FTM3OCH1SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - FTM3 channel 2 output source"]
    #[inline]
    pub fn ftm3och2src(&self) -> FTM3OCH2SRCR {
        FTM3OCH2SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - FTM3 channel 3 output source"]
    #[inline]
    pub fn ftm3och3src(&self) -> FTM3OCH3SRCR {
        FTM3OCH3SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - FTM3 channel 4 output source"]
    #[inline]
    pub fn ftm3och4src(&self) -> FTM3OCH4SRCR {
        FTM3OCH4SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - FTM3 channel 5 output source"]
    #[inline]
    pub fn ftm3och5src(&self) -> FTM3OCH5SRCR {
        FTM3OCH5SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - FTM3 channel 6 output source"]
    #[inline]
    pub fn ftm3och6src(&self) -> FTM3OCH6SRCR {
        FTM3OCH6SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - FTM3 channel 7 output source"]
    #[inline]
    pub fn ftm3och7src(&self) -> FTM3OCH7SRCR {
        FTM3OCH7SRCR::_from({
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
    #[doc = "Bit 0 - FTM0 Hardware Trigger 0 Software Synchronization"]
    #[inline]
    pub fn ftm0syncbit(&mut self) -> _FTM0SYNCBITW {
        _FTM0SYNCBITW { w: self }
    }
    #[doc = "Bit 1 - FTM1 Hardware Trigger 0 Software Synchronization"]
    #[inline]
    pub fn ftm1syncbit(&mut self) -> _FTM1SYNCBITW {
        _FTM1SYNCBITW { w: self }
    }
    #[doc = "Bit 2 - FTM2 Hardware Trigger 0 Software Synchronization"]
    #[inline]
    pub fn ftm2syncbit(&mut self) -> _FTM2SYNCBITW {
        _FTM2SYNCBITW { w: self }
    }
    #[doc = "Bit 3 - FTM3 Hardware Trigger 0 Software Synchronization"]
    #[inline]
    pub fn ftm3syncbit(&mut self) -> _FTM3SYNCBITW {
        _FTM3SYNCBITW { w: self }
    }
    #[doc = "Bit 16 - FTM0 channel 0 output source"]
    #[inline]
    pub fn ftm0och0src(&mut self) -> _FTM0OCH0SRCW {
        _FTM0OCH0SRCW { w: self }
    }
    #[doc = "Bit 17 - FTM0 channel 1 output source"]
    #[inline]
    pub fn ftm0och1src(&mut self) -> _FTM0OCH1SRCW {
        _FTM0OCH1SRCW { w: self }
    }
    #[doc = "Bit 18 - FTM0 channel 2 output source"]
    #[inline]
    pub fn ftm0och2src(&mut self) -> _FTM0OCH2SRCW {
        _FTM0OCH2SRCW { w: self }
    }
    #[doc = "Bit 19 - FTM0 channel 3 output source"]
    #[inline]
    pub fn ftm0och3src(&mut self) -> _FTM0OCH3SRCW {
        _FTM0OCH3SRCW { w: self }
    }
    #[doc = "Bit 20 - FTM0 channel 4 output source"]
    #[inline]
    pub fn ftm0och4src(&mut self) -> _FTM0OCH4SRCW {
        _FTM0OCH4SRCW { w: self }
    }
    #[doc = "Bit 21 - FTM0 channel 5 output source"]
    #[inline]
    pub fn ftm0och5src(&mut self) -> _FTM0OCH5SRCW {
        _FTM0OCH5SRCW { w: self }
    }
    #[doc = "Bit 22 - FTM0 channel 6 output source"]
    #[inline]
    pub fn ftm0och6src(&mut self) -> _FTM0OCH6SRCW {
        _FTM0OCH6SRCW { w: self }
    }
    #[doc = "Bit 23 - FTM0 channel 7 output source"]
    #[inline]
    pub fn ftm0och7src(&mut self) -> _FTM0OCH7SRCW {
        _FTM0OCH7SRCW { w: self }
    }
    #[doc = "Bit 24 - FTM3 channel 0 output source"]
    #[inline]
    pub fn ftm3och0src(&mut self) -> _FTM3OCH0SRCW {
        _FTM3OCH0SRCW { w: self }
    }
    #[doc = "Bit 25 - FTM3 channel 1 output source"]
    #[inline]
    pub fn ftm3och1src(&mut self) -> _FTM3OCH1SRCW {
        _FTM3OCH1SRCW { w: self }
    }
    #[doc = "Bit 26 - FTM3 channel 2 output source"]
    #[inline]
    pub fn ftm3och2src(&mut self) -> _FTM3OCH2SRCW {
        _FTM3OCH2SRCW { w: self }
    }
    #[doc = "Bit 27 - FTM3 channel 3 output source"]
    #[inline]
    pub fn ftm3och3src(&mut self) -> _FTM3OCH3SRCW {
        _FTM3OCH3SRCW { w: self }
    }
    #[doc = "Bit 28 - FTM3 channel 4 output source"]
    #[inline]
    pub fn ftm3och4src(&mut self) -> _FTM3OCH4SRCW {
        _FTM3OCH4SRCW { w: self }
    }
    #[doc = "Bit 29 - FTM3 channel 5 output source"]
    #[inline]
    pub fn ftm3och5src(&mut self) -> _FTM3OCH5SRCW {
        _FTM3OCH5SRCW { w: self }
    }
    #[doc = "Bit 30 - FTM3 channel 6 output source"]
    #[inline]
    pub fn ftm3och6src(&mut self) -> _FTM3OCH6SRCW {
        _FTM3OCH6SRCW { w: self }
    }
    #[doc = "Bit 31 - FTM3 channel 7 output source"]
    #[inline]
    pub fn ftm3och7src(&mut self) -> _FTM3OCH7SRCW {
        _FTM3OCH7SRCW { w: self }
    }
}
