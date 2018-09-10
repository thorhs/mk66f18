#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQSTATEN {
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
#[doc = "Possible values of the field `CCSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCSENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CCSENR {
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
            CCSENR::_0 => false,
            CCSENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCSENR {
        match value {
            false => CCSENR::_0,
            true => CCSENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCSENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCSENR::_1
    }
}
#[doc = "Possible values of the field `TCSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCSENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl TCSENR {
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
            TCSENR::_0 => false,
            TCSENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCSENR {
        match value {
            false => TCSENR::_0,
            true => TCSENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCSENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCSENR::_1
    }
}
#[doc = "Possible values of the field `BGESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGESENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl BGESENR {
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
            BGESENR::_0 => false,
            BGESENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGESENR {
        match value {
            false => BGESENR::_0,
            true => BGESENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BGESENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BGESENR::_1
    }
}
#[doc = "Possible values of the field `DINTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINTSENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DINTSENR {
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
            DINTSENR::_0 => false,
            DINTSENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DINTSENR {
        match value {
            false => DINTSENR::_0,
            true => DINTSENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DINTSENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DINTSENR::_1
    }
}
#[doc = "Possible values of the field `BWRSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWRSENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl BWRSENR {
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
            BWRSENR::_0 => false,
            BWRSENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWRSENR {
        match value {
            false => BWRSENR::_0,
            true => BWRSENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BWRSENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BWRSENR::_1
    }
}
#[doc = "Possible values of the field `BRRSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRRSENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl BRRSENR {
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
            BRRSENR::_0 => false,
            BRRSENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRRSENR {
        match value {
            false => BRRSENR::_0,
            true => BRRSENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BRRSENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BRRSENR::_1
    }
}
#[doc = "Possible values of the field `CINSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CINSENR {
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
            CINSENR::_0 => false,
            CINSENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINSENR {
        match value {
            false => CINSENR::_0,
            true => CINSENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CINSENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CINSENR::_1
    }
}
#[doc = "Possible values of the field `CRMSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRMSENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CRMSENR {
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
            CRMSENR::_0 => false,
            CRMSENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRMSENR {
        match value {
            false => CRMSENR::_0,
            true => CRMSENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRMSENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRMSENR::_1
    }
}
#[doc = "Possible values of the field `CINTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINTSENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CINTSENR {
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
            CINTSENR::_0 => false,
            CINTSENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINTSENR {
        match value {
            false => CINTSENR::_0,
            true => CINTSENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CINTSENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CINTSENR::_1
    }
}
#[doc = "Possible values of the field `CTOESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTOESENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CTOESENR {
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
            CTOESENR::_0 => false,
            CTOESENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTOESENR {
        match value {
            false => CTOESENR::_0,
            true => CTOESENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CTOESENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CTOESENR::_1
    }
}
#[doc = "Possible values of the field `CCESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCESENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CCESENR {
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
            CCESENR::_0 => false,
            CCESENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCESENR {
        match value {
            false => CCESENR::_0,
            true => CCESENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCESENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCESENR::_1
    }
}
#[doc = "Possible values of the field `CEBESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEBESENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CEBESENR {
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
            CEBESENR::_0 => false,
            CEBESENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEBESENR {
        match value {
            false => CEBESENR::_0,
            true => CEBESENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CEBESENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CEBESENR::_1
    }
}
#[doc = "Possible values of the field `CIESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIESENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CIESENR {
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
            CIESENR::_0 => false,
            CIESENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIESENR {
        match value {
            false => CIESENR::_0,
            true => CIESENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CIESENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CIESENR::_1
    }
}
#[doc = "Possible values of the field `DTOESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOESENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DTOESENR {
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
            DTOESENR::_0 => false,
            DTOESENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTOESENR {
        match value {
            false => DTOESENR::_0,
            true => DTOESENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DTOESENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DTOESENR::_1
    }
}
#[doc = "Possible values of the field `DCESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCESENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DCESENR {
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
            DCESENR::_0 => false,
            DCESENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCESENR {
        match value {
            false => DCESENR::_0,
            true => DCESENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCESENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCESENR::_1
    }
}
#[doc = "Possible values of the field `DEBESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBESENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DEBESENR {
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
            DEBESENR::_0 => false,
            DEBESENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEBESENR {
        match value {
            false => DEBESENR::_0,
            true => DEBESENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DEBESENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DEBESENR::_1
    }
}
#[doc = "Possible values of the field `AC12ESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12ESENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl AC12ESENR {
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
            AC12ESENR::_0 => false,
            AC12ESENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12ESENR {
        match value {
            false => AC12ESENR::_0,
            true => AC12ESENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AC12ESENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AC12ESENR::_1
    }
}
#[doc = "Possible values of the field `DMAESEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAESENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DMAESENR {
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
            DMAESENR::_0 => false,
            DMAESENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAESENR {
        match value {
            false => DMAESENR::_0,
            true => DMAESENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAESENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAESENR::_1
    }
}
#[doc = "Values that can be written to the field `CCSEN`"]
pub enum CCSENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CCSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCSENW::_0 => false,
            CCSENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCSENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCSENW::_1)
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
#[doc = "Values that can be written to the field `TCSEN`"]
pub enum TCSENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl TCSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCSENW::_0 => false,
            TCSENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCSENW<'a> {
    w: &'a mut W,
}
impl<'a> _TCSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCSENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCSENW::_1)
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
#[doc = "Values that can be written to the field `BGESEN`"]
pub enum BGESENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl BGESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGESENW::_0 => false,
            BGESENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGESENW<'a> {
    w: &'a mut W,
}
impl<'a> _BGESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGESENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGESENW::_1)
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
#[doc = "Values that can be written to the field `DINTSEN`"]
pub enum DINTSENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DINTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DINTSENW::_0 => false,
            DINTSENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DINTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _DINTSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DINTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINTSENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINTSENW::_1)
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
#[doc = "Values that can be written to the field `BWRSEN`"]
pub enum BWRSENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl BWRSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWRSENW::_0 => false,
            BWRSENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWRSENW<'a> {
    w: &'a mut W,
}
impl<'a> _BWRSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWRSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWRSENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWRSENW::_1)
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
#[doc = "Values that can be written to the field `BRRSEN`"]
pub enum BRRSENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl BRRSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRRSENW::_0 => false,
            BRRSENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRRSENW<'a> {
    w: &'a mut W,
}
impl<'a> _BRRSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRRSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRRSENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRRSENW::_1)
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
#[doc = "Values that can be written to the field `CINSEN`"]
pub enum CINSENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CINSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINSENW::_0 => false,
            CINSENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CINSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINSENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINSENW::_1)
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
#[doc = "Values that can be written to the field `CRMSEN`"]
pub enum CRMSENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CRMSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRMSENW::_0 => false,
            CRMSENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRMSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRMSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRMSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRMSENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRMSENW::_1)
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
#[doc = "Values that can be written to the field `CINTSEN`"]
pub enum CINTSENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CINTSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINTSENW::_0 => false,
            CINTSENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _CINTSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINTSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINTSENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINTSENW::_1)
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
#[doc = "Values that can be written to the field `CTOESEN`"]
pub enum CTOESENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CTOESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTOESENW::_0 => false,
            CTOESENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTOESENW<'a> {
    w: &'a mut W,
}
impl<'a> _CTOESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTOESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTOESENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTOESENW::_1)
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
#[doc = "Values that can be written to the field `CCESEN`"]
pub enum CCESENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CCESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCESENW::_0 => false,
            CCESENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCESENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCESENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCESENW::_1)
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
#[doc = "Values that can be written to the field `CEBESEN`"]
pub enum CEBESENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CEBESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEBESENW::_0 => false,
            CEBESENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEBESENW<'a> {
    w: &'a mut W,
}
impl<'a> _CEBESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEBESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEBESENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEBESENW::_1)
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
#[doc = "Values that can be written to the field `CIESEN`"]
pub enum CIESENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CIESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIESENW::_0 => false,
            CIESENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIESENW<'a> {
    w: &'a mut W,
}
impl<'a> _CIESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CIESENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CIESENW::_1)
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
#[doc = "Values that can be written to the field `DTOESEN`"]
pub enum DTOESENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DTOESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTOESENW::_0 => false,
            DTOESENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTOESENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTOESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTOESENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOESENW::_1)
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
#[doc = "Values that can be written to the field `DCESEN`"]
pub enum DCESENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DCESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCESENW::_0 => false,
            DCESENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCESENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCESENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCESENW::_1)
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
#[doc = "Values that can be written to the field `DEBESEN`"]
pub enum DEBESENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DEBESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEBESENW::_0 => false,
            DEBESENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEBESENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEBESENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEBESENW::_1)
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
#[doc = "Values that can be written to the field `AC12ESEN`"]
pub enum AC12ESENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl AC12ESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AC12ESENW::_0 => false,
            AC12ESENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AC12ESENW<'a> {
    w: &'a mut W,
}
impl<'a> _AC12ESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AC12ESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AC12ESENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AC12ESENW::_1)
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
#[doc = "Values that can be written to the field `DMAESEN`"]
pub enum DMAESENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DMAESENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAESENW::_0 => false,
            DMAESENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAESENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAESENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAESENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAESENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAESENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline]
    pub fn ccsen(&self) -> CCSENR {
        CCSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline]
    pub fn tcsen(&self) -> TCSENR {
        TCSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline]
    pub fn bgesen(&self) -> BGESENR {
        BGESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline]
    pub fn dintsen(&self) -> DINTSENR {
        DINTSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline]
    pub fn bwrsen(&self) -> BWRSENR {
        BWRSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline]
    pub fn brrsen(&self) -> BRRSENR {
        BRRSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline]
    pub fn cinsen(&self) -> CINSENR {
        CINSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline]
    pub fn crmsen(&self) -> CRMSENR {
        CRMSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline]
    pub fn cintsen(&self) -> CINTSENR {
        CINTSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Command Timeout Error Status Enable"]
    #[inline]
    pub fn ctoesen(&self) -> CTOESENR {
        CTOESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline]
    pub fn ccesen(&self) -> CCESENR {
        CCESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline]
    pub fn cebesen(&self) -> CEBESENR {
        CEBESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline]
    pub fn ciesen(&self) -> CIESENR {
        CIESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline]
    pub fn dtoesen(&self) -> DTOESENR {
        DTOESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline]
    pub fn dcesen(&self) -> DCESENR {
        DCESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline]
    pub fn debesen(&self) -> DEBESENR {
        DEBESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline]
    pub fn ac12esen(&self) -> AC12ESENR {
        AC12ESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - DMA Error Status Enable"]
    #[inline]
    pub fn dmaesen(&self) -> DMAESENR {
        DMAESENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 293536063 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline]
    pub fn ccsen(&mut self) -> _CCSENW {
        _CCSENW { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline]
    pub fn tcsen(&mut self) -> _TCSENW {
        _TCSENW { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline]
    pub fn bgesen(&mut self) -> _BGESENW {
        _BGESENW { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline]
    pub fn dintsen(&mut self) -> _DINTSENW {
        _DINTSENW { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline]
    pub fn bwrsen(&mut self) -> _BWRSENW {
        _BWRSENW { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline]
    pub fn brrsen(&mut self) -> _BRRSENW {
        _BRRSENW { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline]
    pub fn cinsen(&mut self) -> _CINSENW {
        _CINSENW { w: self }
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline]
    pub fn crmsen(&mut self) -> _CRMSENW {
        _CRMSENW { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline]
    pub fn cintsen(&mut self) -> _CINTSENW {
        _CINTSENW { w: self }
    }
    #[doc = "Bit 16 - Command Timeout Error Status Enable"]
    #[inline]
    pub fn ctoesen(&mut self) -> _CTOESENW {
        _CTOESENW { w: self }
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline]
    pub fn ccesen(&mut self) -> _CCESENW {
        _CCESENW { w: self }
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline]
    pub fn cebesen(&mut self) -> _CEBESENW {
        _CEBESENW { w: self }
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline]
    pub fn ciesen(&mut self) -> _CIESENW {
        _CIESENW { w: self }
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline]
    pub fn dtoesen(&mut self) -> _DTOESENW {
        _DTOESENW { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline]
    pub fn dcesen(&mut self) -> _DCESENW {
        _DCESENW { w: self }
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline]
    pub fn debesen(&mut self) -> _DEBESENW {
        _DEBESENW { w: self }
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline]
    pub fn ac12esen(&mut self) -> _AC12ESENW {
        _AC12ESENW { w: self }
    }
    #[doc = "Bit 28 - DMA Error Status Enable"]
    #[inline]
    pub fn dmaesen(&mut self) -> _DMAESENW {
        _DMAESENW { w: self }
    }
}
