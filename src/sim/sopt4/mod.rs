#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT4 {
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
#[doc = "Possible values of the field `FTM0FLT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0FLT0R {
    #[doc = "FTM0_FLT0 pin"]
    _0,
    #[doc = "CMP0 out"]
    _1,
}
impl FTM0FLT0R {
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
            FTM0FLT0R::_0 => false,
            FTM0FLT0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0FLT0R {
        match value {
            false => FTM0FLT0R::_0,
            true => FTM0FLT0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0FLT0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0FLT0R::_1
    }
}
#[doc = "Possible values of the field `FTM0FLT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0FLT1R {
    #[doc = "FTM0_FLT1 pin"]
    _0,
    #[doc = "CMP1 out"]
    _1,
}
impl FTM0FLT1R {
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
            FTM0FLT1R::_0 => false,
            FTM0FLT1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0FLT1R {
        match value {
            false => FTM0FLT1R::_0,
            true => FTM0FLT1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0FLT1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0FLT1R::_1
    }
}
#[doc = "Possible values of the field `FTM0FLT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0FLT2R {
    #[doc = "FTM0_FLT2 pin"]
    _0,
    #[doc = "CMP2 out"]
    _1,
}
impl FTM0FLT2R {
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
            FTM0FLT2R::_0 => false,
            FTM0FLT2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0FLT2R {
        match value {
            false => FTM0FLT2R::_0,
            true => FTM0FLT2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0FLT2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0FLT2R::_1
    }
}
#[doc = "Possible values of the field `FTM0FLT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0FLT3R {
    #[doc = "FTM0_FLT3 pin"]
    _0,
    #[doc = "CMP3 out"]
    _1,
}
impl FTM0FLT3R {
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
            FTM0FLT3R::_0 => false,
            FTM0FLT3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0FLT3R {
        match value {
            false => FTM0FLT3R::_0,
            true => FTM0FLT3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0FLT3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0FLT3R::_1
    }
}
#[doc = "Possible values of the field `FTM1FLT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1FLT0R {
    #[doc = "FTM1_FLT0 pin"]
    _0,
    #[doc = "CMP0 out"]
    _1,
}
impl FTM1FLT0R {
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
            FTM1FLT0R::_0 => false,
            FTM1FLT0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM1FLT0R {
        match value {
            false => FTM1FLT0R::_0,
            true => FTM1FLT0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM1FLT0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM1FLT0R::_1
    }
}
#[doc = "Possible values of the field `FTM2FLT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2FLT0R {
    #[doc = "FTM2_FLT0 pin"]
    _0,
    #[doc = "CMP0 out"]
    _1,
}
impl FTM2FLT0R {
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
            FTM2FLT0R::_0 => false,
            FTM2FLT0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM2FLT0R {
        match value {
            false => FTM2FLT0R::_0,
            true => FTM2FLT0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM2FLT0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM2FLT0R::_1
    }
}
#[doc = "Possible values of the field `FTM3FLT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3FLT0R {
    #[doc = "FTM3_FLT0 pin"]
    _0,
    #[doc = "CMP0 out"]
    _1,
}
impl FTM3FLT0R {
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
            FTM3FLT0R::_0 => false,
            FTM3FLT0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3FLT0R {
        match value {
            false => FTM3FLT0R::_0,
            true => FTM3FLT0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3FLT0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3FLT0R::_1
    }
}
#[doc = "Possible values of the field `FTM1CH0SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1CH0SRCR {
    #[doc = "FTM1_CH0 signal"]
    _00,
    #[doc = "CMP0 output"]
    _01,
    #[doc = "CMP1 output"]
    _10,
    #[doc = "USB start of frame pulse"]
    _11,
}
impl FTM1CH0SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM1CH0SRCR::_00 => 0,
            FTM1CH0SRCR::_01 => 1,
            FTM1CH0SRCR::_10 => 2,
            FTM1CH0SRCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM1CH0SRCR {
        match value {
            0 => FTM1CH0SRCR::_00,
            1 => FTM1CH0SRCR::_01,
            2 => FTM1CH0SRCR::_10,
            3 => FTM1CH0SRCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FTM1CH0SRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FTM1CH0SRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FTM1CH0SRCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FTM1CH0SRCR::_11
    }
}
#[doc = "Possible values of the field `FTM2CH0SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2CH0SRCR {
    #[doc = "FTM2_CH0 signal"]
    _00,
    #[doc = "CMP0 output"]
    _01,
    #[doc = "CMP1 output"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FTM2CH0SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FTM2CH0SRCR::_00 => 0,
            FTM2CH0SRCR::_01 => 1,
            FTM2CH0SRCR::_10 => 2,
            FTM2CH0SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FTM2CH0SRCR {
        match value {
            0 => FTM2CH0SRCR::_00,
            1 => FTM2CH0SRCR::_01,
            2 => FTM2CH0SRCR::_10,
            i => FTM2CH0SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FTM2CH0SRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FTM2CH0SRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FTM2CH0SRCR::_10
    }
}
#[doc = "Possible values of the field `FTM2CH1SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2CH1SRCR {
    #[doc = "FTM2_CH1 signal"]
    _0,
    #[doc = "Exclusive OR of FTM2_CH1, FTM2_CH0 and FTM1_CH1."]
    _1,
}
impl FTM2CH1SRCR {
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
            FTM2CH1SRCR::_0 => false,
            FTM2CH1SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM2CH1SRCR {
        match value {
            false => FTM2CH1SRCR::_0,
            true => FTM2CH1SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM2CH1SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM2CH1SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM0CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0CLKSELR {
    #[doc = "FTM_CLK0 pin"]
    _0,
    #[doc = "FTM_CLK1 pin"]
    _1,
}
impl FTM0CLKSELR {
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
            FTM0CLKSELR::_0 => false,
            FTM0CLKSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0CLKSELR {
        match value {
            false => FTM0CLKSELR::_0,
            true => FTM0CLKSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0CLKSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0CLKSELR::_1
    }
}
#[doc = "Possible values of the field `FTM1CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1CLKSELR {
    #[doc = "FTM_CLK0 pin"]
    _0,
    #[doc = "FTM_CLK1 pin"]
    _1,
}
impl FTM1CLKSELR {
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
            FTM1CLKSELR::_0 => false,
            FTM1CLKSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM1CLKSELR {
        match value {
            false => FTM1CLKSELR::_0,
            true => FTM1CLKSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM1CLKSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM1CLKSELR::_1
    }
}
#[doc = "Possible values of the field `FTM2CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2CLKSELR {
    #[doc = "FTM2 external clock driven by FTM_CLK0 pin."]
    _0,
    #[doc = "FTM2 external clock driven by FTM_CLK1 pin."]
    _1,
}
impl FTM2CLKSELR {
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
            FTM2CLKSELR::_0 => false,
            FTM2CLKSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM2CLKSELR {
        match value {
            false => FTM2CLKSELR::_0,
            true => FTM2CLKSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM2CLKSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM2CLKSELR::_1
    }
}
#[doc = "Possible values of the field `FTM3CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3CLKSELR {
    #[doc = "FTM3 external clock driven by FTM_CLK0 pin."]
    _0,
    #[doc = "FTM3 external clock driven by FTM_CLK1 pin."]
    _1,
}
impl FTM3CLKSELR {
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
            FTM3CLKSELR::_0 => false,
            FTM3CLKSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3CLKSELR {
        match value {
            false => FTM3CLKSELR::_0,
            true => FTM3CLKSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3CLKSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3CLKSELR::_1
    }
}
#[doc = "Possible values of the field `FTM0TRG0SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0TRG0SRCR {
    #[doc = "HSCMP0 output drives FTM0 hardware trigger 0"]
    _0,
    #[doc = "FTM1 channel match drives FTM0 hardware trigger 0"]
    _1,
}
impl FTM0TRG0SRCR {
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
            FTM0TRG0SRCR::_0 => false,
            FTM0TRG0SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0TRG0SRCR {
        match value {
            false => FTM0TRG0SRCR::_0,
            true => FTM0TRG0SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0TRG0SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0TRG0SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM0TRG1SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0TRG1SRCR {
    #[doc = "PDB output trigger 1 drives FTM0 hardware trigger 1"]
    _0,
    #[doc = "FTM2 channel match drives FTM0 hardware trigger 1"]
    _1,
}
impl FTM0TRG1SRCR {
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
            FTM0TRG1SRCR::_0 => false,
            FTM0TRG1SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0TRG1SRCR {
        match value {
            false => FTM0TRG1SRCR::_0,
            true => FTM0TRG1SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0TRG1SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0TRG1SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM3TRG0SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3TRG0SRCR {
    #[doc = "FTM1 channel match drives FTM3 hardware trigger 0"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FTM3TRG0SRCR {
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
            FTM3TRG0SRCR::_1 => true,
            FTM3TRG0SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3TRG0SRCR {
        match value {
            true => FTM3TRG0SRCR::_1,
            i => FTM3TRG0SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3TRG0SRCR::_1
    }
}
#[doc = "Possible values of the field `FTM3TRG1SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3TRG1SRCR {
    #[doc = "FTM2 channel match drives FTM3 hardware trigger 1"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FTM3TRG1SRCR {
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
            FTM3TRG1SRCR::_1 => true,
            FTM3TRG1SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3TRG1SRCR {
        match value {
            true => FTM3TRG1SRCR::_1,
            i => FTM3TRG1SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3TRG1SRCR::_1
    }
}
#[doc = "Values that can be written to the field `FTM0FLT0`"]
pub enum FTM0FLT0W {
    #[doc = "FTM0_FLT0 pin"]
    _0,
    #[doc = "CMP0 out"]
    _1,
}
impl FTM0FLT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0FLT0W::_0 => false,
            FTM0FLT0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0FLT0W<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0FLT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0FLT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM0_FLT0 pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0FLT0W::_0)
    }
    #[doc = "CMP0 out"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0FLT0W::_1)
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
#[doc = "Values that can be written to the field `FTM0FLT1`"]
pub enum FTM0FLT1W {
    #[doc = "FTM0_FLT1 pin"]
    _0,
    #[doc = "CMP1 out"]
    _1,
}
impl FTM0FLT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0FLT1W::_0 => false,
            FTM0FLT1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0FLT1W<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0FLT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0FLT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM0_FLT1 pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0FLT1W::_0)
    }
    #[doc = "CMP1 out"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0FLT1W::_1)
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
#[doc = "Values that can be written to the field `FTM0FLT2`"]
pub enum FTM0FLT2W {
    #[doc = "FTM0_FLT2 pin"]
    _0,
    #[doc = "CMP2 out"]
    _1,
}
impl FTM0FLT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0FLT2W::_0 => false,
            FTM0FLT2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0FLT2W<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0FLT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0FLT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM0_FLT2 pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0FLT2W::_0)
    }
    #[doc = "CMP2 out"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0FLT2W::_1)
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
#[doc = "Values that can be written to the field `FTM0FLT3`"]
pub enum FTM0FLT3W {
    #[doc = "FTM0_FLT3 pin"]
    _0,
    #[doc = "CMP3 out"]
    _1,
}
impl FTM0FLT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0FLT3W::_0 => false,
            FTM0FLT3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0FLT3W<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0FLT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0FLT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM0_FLT3 pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0FLT3W::_0)
    }
    #[doc = "CMP3 out"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0FLT3W::_1)
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
#[doc = "Values that can be written to the field `FTM1FLT0`"]
pub enum FTM1FLT0W {
    #[doc = "FTM1_FLT0 pin"]
    _0,
    #[doc = "CMP0 out"]
    _1,
}
impl FTM1FLT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM1FLT0W::_0 => false,
            FTM1FLT0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM1FLT0W<'a> {
    w: &'a mut W,
}
impl<'a> _FTM1FLT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM1FLT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM1_FLT0 pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1FLT0W::_0)
    }
    #[doc = "CMP0 out"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1FLT0W::_1)
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
#[doc = "Values that can be written to the field `FTM2FLT0`"]
pub enum FTM2FLT0W {
    #[doc = "FTM2_FLT0 pin"]
    _0,
    #[doc = "CMP0 out"]
    _1,
}
impl FTM2FLT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM2FLT0W::_0 => false,
            FTM2FLT0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM2FLT0W<'a> {
    w: &'a mut W,
}
impl<'a> _FTM2FLT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM2FLT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM2_FLT0 pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2FLT0W::_0)
    }
    #[doc = "CMP0 out"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2FLT0W::_1)
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
#[doc = "Values that can be written to the field `FTM3FLT0`"]
pub enum FTM3FLT0W {
    #[doc = "FTM3_FLT0 pin"]
    _0,
    #[doc = "CMP0 out"]
    _1,
}
impl FTM3FLT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3FLT0W::_0 => false,
            FTM3FLT0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3FLT0W<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3FLT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3FLT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM3_FLT0 pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3FLT0W::_0)
    }
    #[doc = "CMP0 out"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3FLT0W::_1)
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
#[doc = "Values that can be written to the field `FTM1CH0SRC`"]
pub enum FTM1CH0SRCW {
    #[doc = "FTM1_CH0 signal"]
    _00,
    #[doc = "CMP0 output"]
    _01,
    #[doc = "CMP1 output"]
    _10,
    #[doc = "USB start of frame pulse"]
    _11,
}
impl FTM1CH0SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM1CH0SRCW::_00 => 0,
            FTM1CH0SRCW::_01 => 1,
            FTM1CH0SRCW::_10 => 2,
            FTM1CH0SRCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM1CH0SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM1CH0SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM1CH0SRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FTM1_CH0 signal"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM1CH0SRCW::_00)
    }
    #[doc = "CMP0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM1CH0SRCW::_01)
    }
    #[doc = "CMP1 output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM1CH0SRCW::_10)
    }
    #[doc = "USB start of frame pulse"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM1CH0SRCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM2CH0SRC`"]
pub enum FTM2CH0SRCW {
    #[doc = "FTM2_CH0 signal"]
    _00,
    #[doc = "CMP0 output"]
    _01,
    #[doc = "CMP1 output"]
    _10,
}
impl FTM2CH0SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FTM2CH0SRCW::_00 => 0,
            FTM2CH0SRCW::_01 => 1,
            FTM2CH0SRCW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM2CH0SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM2CH0SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM2CH0SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FTM2_CH0 signal"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2CH0SRCW::_00)
    }
    #[doc = "CMP0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2CH0SRCW::_01)
    }
    #[doc = "CMP1 output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM2CH0SRCW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM2CH1SRC`"]
pub enum FTM2CH1SRCW {
    #[doc = "FTM2_CH1 signal"]
    _0,
    #[doc = "Exclusive OR of FTM2_CH1, FTM2_CH0 and FTM1_CH1."]
    _1,
}
impl FTM2CH1SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM2CH1SRCW::_0 => false,
            FTM2CH1SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM2CH1SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM2CH1SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM2CH1SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM2_CH1 signal"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2CH1SRCW::_0)
    }
    #[doc = "Exclusive OR of FTM2_CH1, FTM2_CH0 and FTM1_CH1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2CH1SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM0CLKSEL`"]
pub enum FTM0CLKSELW {
    #[doc = "FTM_CLK0 pin"]
    _0,
    #[doc = "FTM_CLK1 pin"]
    _1,
}
impl FTM0CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0CLKSELW::_0 => false,
            FTM0CLKSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0CLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM_CLK0 pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0CLKSELW::_0)
    }
    #[doc = "FTM_CLK1 pin"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0CLKSELW::_1)
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
#[doc = "Values that can be written to the field `FTM1CLKSEL`"]
pub enum FTM1CLKSELW {
    #[doc = "FTM_CLK0 pin"]
    _0,
    #[doc = "FTM_CLK1 pin"]
    _1,
}
impl FTM1CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM1CLKSELW::_0 => false,
            FTM1CLKSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM1CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM1CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM1CLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM_CLK0 pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1CLKSELW::_0)
    }
    #[doc = "FTM_CLK1 pin"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1CLKSELW::_1)
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
#[doc = "Values that can be written to the field `FTM2CLKSEL`"]
pub enum FTM2CLKSELW {
    #[doc = "FTM2 external clock driven by FTM_CLK0 pin."]
    _0,
    #[doc = "FTM2 external clock driven by FTM_CLK1 pin."]
    _1,
}
impl FTM2CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM2CLKSELW::_0 => false,
            FTM2CLKSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM2CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM2CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM2CLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM2 external clock driven by FTM_CLK0 pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2CLKSELW::_0)
    }
    #[doc = "FTM2 external clock driven by FTM_CLK1 pin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2CLKSELW::_1)
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
#[doc = "Values that can be written to the field `FTM3CLKSEL`"]
pub enum FTM3CLKSELW {
    #[doc = "FTM3 external clock driven by FTM_CLK0 pin."]
    _0,
    #[doc = "FTM3 external clock driven by FTM_CLK1 pin."]
    _1,
}
impl FTM3CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3CLKSELW::_0 => false,
            FTM3CLKSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3CLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM3 external clock driven by FTM_CLK0 pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3CLKSELW::_0)
    }
    #[doc = "FTM3 external clock driven by FTM_CLK1 pin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3CLKSELW::_1)
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
#[doc = "Values that can be written to the field `FTM0TRG0SRC`"]
pub enum FTM0TRG0SRCW {
    #[doc = "HSCMP0 output drives FTM0 hardware trigger 0"]
    _0,
    #[doc = "FTM1 channel match drives FTM0 hardware trigger 0"]
    _1,
}
impl FTM0TRG0SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0TRG0SRCW::_0 => false,
            FTM0TRG0SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0TRG0SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0TRG0SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0TRG0SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSCMP0 output drives FTM0 hardware trigger 0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0TRG0SRCW::_0)
    }
    #[doc = "FTM1 channel match drives FTM0 hardware trigger 0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0TRG0SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM0TRG1SRC`"]
pub enum FTM0TRG1SRCW {
    #[doc = "PDB output trigger 1 drives FTM0 hardware trigger 1"]
    _0,
    #[doc = "FTM2 channel match drives FTM0 hardware trigger 1"]
    _1,
}
impl FTM0TRG1SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0TRG1SRCW::_0 => false,
            FTM0TRG1SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0TRG1SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0TRG1SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0TRG1SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB output trigger 1 drives FTM0 hardware trigger 1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0TRG1SRCW::_0)
    }
    #[doc = "FTM2 channel match drives FTM0 hardware trigger 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0TRG1SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM3TRG0SRC`"]
pub enum FTM3TRG0SRCW {
    #[doc = "FTM1 channel match drives FTM3 hardware trigger 0"]
    _1,
}
impl FTM3TRG0SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3TRG0SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3TRG0SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3TRG0SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3TRG0SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM1 channel match drives FTM3 hardware trigger 0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3TRG0SRCW::_1)
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
#[doc = "Values that can be written to the field `FTM3TRG1SRC`"]
pub enum FTM3TRG1SRCW {
    #[doc = "FTM2 channel match drives FTM3 hardware trigger 1"]
    _1,
}
impl FTM3TRG1SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3TRG1SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3TRG1SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3TRG1SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3TRG1SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM2 channel match drives FTM3 hardware trigger 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3TRG1SRCW::_1)
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
    #[doc = "Bit 0 - FTM0 Fault 0 Select"]
    #[inline]
    pub fn ftm0flt0(&self) -> FTM0FLT0R {
        FTM0FLT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - FTM0 Fault 1 Select"]
    #[inline]
    pub fn ftm0flt1(&self) -> FTM0FLT1R {
        FTM0FLT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - FTM0 Fault 2 Select"]
    #[inline]
    pub fn ftm0flt2(&self) -> FTM0FLT2R {
        FTM0FLT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - FTM0 Fault 3 Select"]
    #[inline]
    pub fn ftm0flt3(&self) -> FTM0FLT3R {
        FTM0FLT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - FTM1 Fault 0 Select"]
    #[inline]
    pub fn ftm1flt0(&self) -> FTM1FLT0R {
        FTM1FLT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - FTM2 Fault 0 Select"]
    #[inline]
    pub fn ftm2flt0(&self) -> FTM2FLT0R {
        FTM2FLT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - FTM3 Fault 0 Select"]
    #[inline]
    pub fn ftm3flt0(&self) -> FTM3FLT0R {
        FTM3FLT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:19 - FTM1 channel 0 input capture source select"]
    #[inline]
    pub fn ftm1ch0src(&self) -> FTM1CH0SRCR {
        FTM1CH0SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - FTM2 channel 0 input capture source select"]
    #[inline]
    pub fn ftm2ch0src(&self) -> FTM2CH0SRCR {
        FTM2CH0SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - FTM2 channel 1 input capture source select"]
    #[inline]
    pub fn ftm2ch1src(&self) -> FTM2CH1SRCR {
        FTM2CH1SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - FlexTimer 0 External Clock Pin Select"]
    #[inline]
    pub fn ftm0clksel(&self) -> FTM0CLKSELR {
        FTM0CLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - FTM1 External Clock Pin Select"]
    #[inline]
    pub fn ftm1clksel(&self) -> FTM1CLKSELR {
        FTM1CLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - FlexTimer 2 External Clock Pin Select"]
    #[inline]
    pub fn ftm2clksel(&self) -> FTM2CLKSELR {
        FTM2CLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - FlexTimer 3 External Clock Pin Select"]
    #[inline]
    pub fn ftm3clksel(&self) -> FTM3CLKSELR {
        FTM3CLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - FlexTimer 0 Hardware Trigger 0 Source Select"]
    #[inline]
    pub fn ftm0trg0src(&self) -> FTM0TRG0SRCR {
        FTM0TRG0SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - FlexTimer 0 Hardware Trigger 1 Source Select"]
    #[inline]
    pub fn ftm0trg1src(&self) -> FTM0TRG1SRCR {
        FTM0TRG1SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - FlexTimer 3 Hardware Trigger 0 Source Select"]
    #[inline]
    pub fn ftm3trg0src(&self) -> FTM3TRG0SRCR {
        FTM3TRG0SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - FlexTimer 3 Hardware Trigger 1 Source Select"]
    #[inline]
    pub fn ftm3trg1src(&self) -> FTM3TRG1SRCR {
        FTM3TRG1SRCR::_from({
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
    #[doc = "Bit 0 - FTM0 Fault 0 Select"]
    #[inline]
    pub fn ftm0flt0(&mut self) -> _FTM0FLT0W {
        _FTM0FLT0W { w: self }
    }
    #[doc = "Bit 1 - FTM0 Fault 1 Select"]
    #[inline]
    pub fn ftm0flt1(&mut self) -> _FTM0FLT1W {
        _FTM0FLT1W { w: self }
    }
    #[doc = "Bit 2 - FTM0 Fault 2 Select"]
    #[inline]
    pub fn ftm0flt2(&mut self) -> _FTM0FLT2W {
        _FTM0FLT2W { w: self }
    }
    #[doc = "Bit 3 - FTM0 Fault 3 Select"]
    #[inline]
    pub fn ftm0flt3(&mut self) -> _FTM0FLT3W {
        _FTM0FLT3W { w: self }
    }
    #[doc = "Bit 4 - FTM1 Fault 0 Select"]
    #[inline]
    pub fn ftm1flt0(&mut self) -> _FTM1FLT0W {
        _FTM1FLT0W { w: self }
    }
    #[doc = "Bit 8 - FTM2 Fault 0 Select"]
    #[inline]
    pub fn ftm2flt0(&mut self) -> _FTM2FLT0W {
        _FTM2FLT0W { w: self }
    }
    #[doc = "Bit 12 - FTM3 Fault 0 Select"]
    #[inline]
    pub fn ftm3flt0(&mut self) -> _FTM3FLT0W {
        _FTM3FLT0W { w: self }
    }
    #[doc = "Bits 18:19 - FTM1 channel 0 input capture source select"]
    #[inline]
    pub fn ftm1ch0src(&mut self) -> _FTM1CH0SRCW {
        _FTM1CH0SRCW { w: self }
    }
    #[doc = "Bits 20:21 - FTM2 channel 0 input capture source select"]
    #[inline]
    pub fn ftm2ch0src(&mut self) -> _FTM2CH0SRCW {
        _FTM2CH0SRCW { w: self }
    }
    #[doc = "Bit 22 - FTM2 channel 1 input capture source select"]
    #[inline]
    pub fn ftm2ch1src(&mut self) -> _FTM2CH1SRCW {
        _FTM2CH1SRCW { w: self }
    }
    #[doc = "Bit 24 - FlexTimer 0 External Clock Pin Select"]
    #[inline]
    pub fn ftm0clksel(&mut self) -> _FTM0CLKSELW {
        _FTM0CLKSELW { w: self }
    }
    #[doc = "Bit 25 - FTM1 External Clock Pin Select"]
    #[inline]
    pub fn ftm1clksel(&mut self) -> _FTM1CLKSELW {
        _FTM1CLKSELW { w: self }
    }
    #[doc = "Bit 26 - FlexTimer 2 External Clock Pin Select"]
    #[inline]
    pub fn ftm2clksel(&mut self) -> _FTM2CLKSELW {
        _FTM2CLKSELW { w: self }
    }
    #[doc = "Bit 27 - FlexTimer 3 External Clock Pin Select"]
    #[inline]
    pub fn ftm3clksel(&mut self) -> _FTM3CLKSELW {
        _FTM3CLKSELW { w: self }
    }
    #[doc = "Bit 28 - FlexTimer 0 Hardware Trigger 0 Source Select"]
    #[inline]
    pub fn ftm0trg0src(&mut self) -> _FTM0TRG0SRCW {
        _FTM0TRG0SRCW { w: self }
    }
    #[doc = "Bit 29 - FlexTimer 0 Hardware Trigger 1 Source Select"]
    #[inline]
    pub fn ftm0trg1src(&mut self) -> _FTM0TRG1SRCW {
        _FTM0TRG1SRCW { w: self }
    }
    #[doc = "Bit 30 - FlexTimer 3 Hardware Trigger 0 Source Select"]
    #[inline]
    pub fn ftm3trg0src(&mut self) -> _FTM3TRG0SRCW {
        _FTM3TRG0SRCW { w: self }
    }
    #[doc = "Bit 31 - FlexTimer 3 Hardware Trigger 1 Source Select"]
    #[inline]
    pub fn ftm3trg1src(&mut self) -> _FTM3TRG1SRCW {
        _FTM3TRG1SRCW { w: self }
    }
}
