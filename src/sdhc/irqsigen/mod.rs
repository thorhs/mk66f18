#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IRQSIGEN {
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
#[doc = "Possible values of the field `CCIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CCIENR {
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
            CCIENR::_0 => false,
            CCIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCIENR {
        match value {
            false => CCIENR::_0,
            true => CCIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCIENR::_1
    }
}
#[doc = "Possible values of the field `TCIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl TCIENR {
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
            TCIENR::_0 => false,
            TCIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIENR {
        match value {
            false => TCIENR::_0,
            true => TCIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCIENR::_1
    }
}
#[doc = "Possible values of the field `BGEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGEIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl BGEIENR {
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
            BGEIENR::_0 => false,
            BGEIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BGEIENR {
        match value {
            false => BGEIENR::_0,
            true => BGEIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BGEIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BGEIENR::_1
    }
}
#[doc = "Possible values of the field `DINTIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINTIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DINTIENR {
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
            DINTIENR::_0 => false,
            DINTIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DINTIENR {
        match value {
            false => DINTIENR::_0,
            true => DINTIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DINTIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DINTIENR::_1
    }
}
#[doc = "Possible values of the field `BWRIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWRIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl BWRIENR {
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
            BWRIENR::_0 => false,
            BWRIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWRIENR {
        match value {
            false => BWRIENR::_0,
            true => BWRIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BWRIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BWRIENR::_1
    }
}
#[doc = "Possible values of the field `BRRIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRRIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl BRRIENR {
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
            BRRIENR::_0 => false,
            BRRIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRRIENR {
        match value {
            false => BRRIENR::_0,
            true => BRRIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BRRIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BRRIENR::_1
    }
}
#[doc = "Possible values of the field `CINSIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CINSIENR {
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
            CINSIENR::_0 => false,
            CINSIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINSIENR {
        match value {
            false => CINSIENR::_0,
            true => CINSIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CINSIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CINSIENR::_1
    }
}
#[doc = "Possible values of the field `CRMIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRMIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CRMIENR {
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
            CRMIENR::_0 => false,
            CRMIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRMIENR {
        match value {
            false => CRMIENR::_0,
            true => CRMIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRMIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRMIENR::_1
    }
}
#[doc = "Possible values of the field `CINTIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINTIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CINTIENR {
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
            CINTIENR::_0 => false,
            CINTIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CINTIENR {
        match value {
            false => CINTIENR::_0,
            true => CINTIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CINTIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CINTIENR::_1
    }
}
#[doc = "Possible values of the field `CTOEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTOEIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CTOEIENR {
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
            CTOEIENR::_0 => false,
            CTOEIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTOEIENR {
        match value {
            false => CTOEIENR::_0,
            true => CTOEIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CTOEIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CTOEIENR::_1
    }
}
#[doc = "Possible values of the field `CCEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCEIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CCEIENR {
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
            CCEIENR::_0 => false,
            CCEIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCEIENR {
        match value {
            false => CCEIENR::_0,
            true => CCEIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCEIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCEIENR::_1
    }
}
#[doc = "Possible values of the field `CEBEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEBEIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CEBEIENR {
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
            CEBEIENR::_0 => false,
            CEBEIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CEBEIENR {
        match value {
            false => CEBEIENR::_0,
            true => CEBEIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CEBEIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CEBEIENR::_1
    }
}
#[doc = "Possible values of the field `CIEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIEIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CIEIENR {
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
            CIEIENR::_0 => false,
            CIEIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIEIENR {
        match value {
            false => CIEIENR::_0,
            true => CIEIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CIEIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CIEIENR::_1
    }
}
#[doc = "Possible values of the field `DTOEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOEIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DTOEIENR {
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
            DTOEIENR::_0 => false,
            DTOEIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTOEIENR {
        match value {
            false => DTOEIENR::_0,
            true => DTOEIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DTOEIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DTOEIENR::_1
    }
}
#[doc = "Possible values of the field `DCEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCEIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DCEIENR {
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
            DCEIENR::_0 => false,
            DCEIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCEIENR {
        match value {
            false => DCEIENR::_0,
            true => DCEIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DCEIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DCEIENR::_1
    }
}
#[doc = "Possible values of the field `DEBEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBEIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DEBEIENR {
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
            DEBEIENR::_0 => false,
            DEBEIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DEBEIENR {
        match value {
            false => DEBEIENR::_0,
            true => DEBEIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DEBEIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DEBEIENR::_1
    }
}
#[doc = "Possible values of the field `AC12EIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12EIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl AC12EIENR {
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
            AC12EIENR::_0 => false,
            AC12EIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12EIENR {
        match value {
            false => AC12EIENR::_0,
            true => AC12EIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AC12EIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AC12EIENR::_1
    }
}
#[doc = "Possible values of the field `DMAEIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEIENR {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DMAEIENR {
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
            DMAEIENR::_0 => false,
            DMAEIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAEIENR {
        match value {
            false => DMAEIENR::_0,
            true => DMAEIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAEIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAEIENR::_1
    }
}
#[doc = "Values that can be written to the field `CCIEN`"]
pub enum CCIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CCIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCIENW::_0 => false,
            CCIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCIENW::_1)
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
#[doc = "Values that can be written to the field `TCIEN`"]
pub enum TCIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl TCIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIENW::_0 => false,
            TCIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIENW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCIENW::_1)
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
#[doc = "Values that can be written to the field `BGEIEN`"]
pub enum BGEIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl BGEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BGEIENW::_0 => false,
            BGEIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BGEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _BGEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BGEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGEIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGEIENW::_1)
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
#[doc = "Values that can be written to the field `DINTIEN`"]
pub enum DINTIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DINTIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DINTIENW::_0 => false,
            DINTIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DINTIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DINTIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DINTIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINTIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINTIENW::_1)
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
#[doc = "Values that can be written to the field `BWRIEN`"]
pub enum BWRIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl BWRIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWRIENW::_0 => false,
            BWRIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWRIENW<'a> {
    w: &'a mut W,
}
impl<'a> _BWRIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWRIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWRIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWRIENW::_1)
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
#[doc = "Values that can be written to the field `BRRIEN`"]
pub enum BRRIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl BRRIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRRIENW::_0 => false,
            BRRIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRRIENW<'a> {
    w: &'a mut W,
}
impl<'a> _BRRIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRRIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRRIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRRIENW::_1)
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
#[doc = "Values that can be written to the field `CINSIEN`"]
pub enum CINSIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CINSIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINSIENW::_0 => false,
            CINSIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINSIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CINSIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINSIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINSIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINSIENW::_1)
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
#[doc = "Values that can be written to the field `CRMIEN`"]
pub enum CRMIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CRMIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRMIENW::_0 => false,
            CRMIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRMIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRMIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRMIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRMIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRMIENW::_1)
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
#[doc = "Values that can be written to the field `CINTIEN`"]
pub enum CINTIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CINTIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CINTIENW::_0 => false,
            CINTIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CINTIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CINTIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CINTIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINTIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINTIENW::_1)
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
#[doc = "Values that can be written to the field `CTOEIEN`"]
pub enum CTOEIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CTOEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTOEIENW::_0 => false,
            CTOEIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTOEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CTOEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTOEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTOEIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTOEIENW::_1)
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
#[doc = "Values that can be written to the field `CCEIEN`"]
pub enum CCEIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CCEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCEIENW::_0 => false,
            CCEIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCEIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCEIENW::_1)
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
#[doc = "Values that can be written to the field `CEBEIEN`"]
pub enum CEBEIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CEBEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CEBEIENW::_0 => false,
            CEBEIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CEBEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CEBEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CEBEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEBEIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEBEIENW::_1)
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
#[doc = "Values that can be written to the field `CIEIEN`"]
pub enum CIEIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl CIEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIEIENW::_0 => false,
            CIEIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _CIEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CIEIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CIEIENW::_1)
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
#[doc = "Values that can be written to the field `DTOEIEN`"]
pub enum DTOEIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DTOEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTOEIENW::_0 => false,
            DTOEIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTOEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTOEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTOEIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOEIENW::_1)
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
#[doc = "Values that can be written to the field `DCEIEN`"]
pub enum DCEIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DCEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCEIENW::_0 => false,
            DCEIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCEIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCEIENW::_1)
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
#[doc = "Values that can be written to the field `DEBEIEN`"]
pub enum DEBEIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DEBEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DEBEIENW::_0 => false,
            DEBEIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEBEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEBEIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEBEIENW::_1)
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
#[doc = "Values that can be written to the field `AC12EIEN`"]
pub enum AC12EIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl AC12EIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AC12EIENW::_0 => false,
            AC12EIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AC12EIENW<'a> {
    w: &'a mut W,
}
impl<'a> _AC12EIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AC12EIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AC12EIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AC12EIENW::_1)
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
#[doc = "Values that can be written to the field `DMAEIEN`"]
pub enum DMAEIENW {
    #[doc = "Masked"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl DMAEIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAEIENW::_0 => false,
            DMAEIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAEIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAEIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAEIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Masked"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEIENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEIENW::_1)
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
    #[doc = "Bit 0 - Command Complete Interrupt Enable"]
    #[inline]
    pub fn ccien(&self) -> CCIENR {
        CCIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
    #[inline]
    pub fn tcien(&self) -> TCIENR {
        TCIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Block Gap Event Interrupt Enable"]
    #[inline]
    pub fn bgeien(&self) -> BGEIENR {
        BGEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline]
    pub fn dintien(&self) -> DINTIENR {
        DINTIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Buffer Write Ready Interrupt Enable"]
    #[inline]
    pub fn bwrien(&self) -> BWRIENR {
        BWRIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Buffer Read Ready Interrupt Enable"]
    #[inline]
    pub fn brrien(&self) -> BRRIENR {
        BRRIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Card Insertion Interrupt Enable"]
    #[inline]
    pub fn cinsien(&self) -> CINSIENR {
        CINSIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Card Removal Interrupt Enable"]
    #[inline]
    pub fn crmien(&self) -> CRMIENR {
        CRMIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Card Interrupt Enable"]
    #[inline]
    pub fn cintien(&self) -> CINTIENR {
        CINTIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Command Timeout Error Interrupt Enable"]
    #[inline]
    pub fn ctoeien(&self) -> CTOEIENR {
        CTOEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Command CRC Error Interrupt Enable"]
    #[inline]
    pub fn cceien(&self) -> CCEIENR {
        CCEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Command End Bit Error Interrupt Enable"]
    #[inline]
    pub fn cebeien(&self) -> CEBEIENR {
        CEBEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Command Index Error Interrupt Enable"]
    #[inline]
    pub fn cieien(&self) -> CIEIENR {
        CIEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline]
    pub fn dtoeien(&self) -> DTOEIENR {
        DTOEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline]
    pub fn dceien(&self) -> DCEIENR {
        DCEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Data End Bit Error Interrupt Enable"]
    #[inline]
    pub fn debeien(&self) -> DEBEIENR {
        DEBEIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Auto CMD12 Error Interrupt Enable"]
    #[inline]
    pub fn ac12eien(&self) -> AC12EIENR {
        AC12EIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - DMA Error Interrupt Enable"]
    #[inline]
    pub fn dmaeien(&self) -> DMAEIENR {
        DMAEIENR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Command Complete Interrupt Enable"]
    #[inline]
    pub fn ccien(&mut self) -> _CCIENW {
        _CCIENW { w: self }
    }
    #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
    #[inline]
    pub fn tcien(&mut self) -> _TCIENW {
        _TCIENW { w: self }
    }
    #[doc = "Bit 2 - Block Gap Event Interrupt Enable"]
    #[inline]
    pub fn bgeien(&mut self) -> _BGEIENW {
        _BGEIENW { w: self }
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline]
    pub fn dintien(&mut self) -> _DINTIENW {
        _DINTIENW { w: self }
    }
    #[doc = "Bit 4 - Buffer Write Ready Interrupt Enable"]
    #[inline]
    pub fn bwrien(&mut self) -> _BWRIENW {
        _BWRIENW { w: self }
    }
    #[doc = "Bit 5 - Buffer Read Ready Interrupt Enable"]
    #[inline]
    pub fn brrien(&mut self) -> _BRRIENW {
        _BRRIENW { w: self }
    }
    #[doc = "Bit 6 - Card Insertion Interrupt Enable"]
    #[inline]
    pub fn cinsien(&mut self) -> _CINSIENW {
        _CINSIENW { w: self }
    }
    #[doc = "Bit 7 - Card Removal Interrupt Enable"]
    #[inline]
    pub fn crmien(&mut self) -> _CRMIENW {
        _CRMIENW { w: self }
    }
    #[doc = "Bit 8 - Card Interrupt Enable"]
    #[inline]
    pub fn cintien(&mut self) -> _CINTIENW {
        _CINTIENW { w: self }
    }
    #[doc = "Bit 16 - Command Timeout Error Interrupt Enable"]
    #[inline]
    pub fn ctoeien(&mut self) -> _CTOEIENW {
        _CTOEIENW { w: self }
    }
    #[doc = "Bit 17 - Command CRC Error Interrupt Enable"]
    #[inline]
    pub fn cceien(&mut self) -> _CCEIENW {
        _CCEIENW { w: self }
    }
    #[doc = "Bit 18 - Command End Bit Error Interrupt Enable"]
    #[inline]
    pub fn cebeien(&mut self) -> _CEBEIENW {
        _CEBEIENW { w: self }
    }
    #[doc = "Bit 19 - Command Index Error Interrupt Enable"]
    #[inline]
    pub fn cieien(&mut self) -> _CIEIENW {
        _CIEIENW { w: self }
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline]
    pub fn dtoeien(&mut self) -> _DTOEIENW {
        _DTOEIENW { w: self }
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline]
    pub fn dceien(&mut self) -> _DCEIENW {
        _DCEIENW { w: self }
    }
    #[doc = "Bit 22 - Data End Bit Error Interrupt Enable"]
    #[inline]
    pub fn debeien(&mut self) -> _DEBEIENW {
        _DEBEIENW { w: self }
    }
    #[doc = "Bit 24 - Auto CMD12 Error Interrupt Enable"]
    #[inline]
    pub fn ac12eien(&mut self) -> _AC12EIENW {
        _AC12EIENW { w: self }
    }
    #[doc = "Bit 28 - DMA Error Interrupt Enable"]
    #[inline]
    pub fn dmaeien(&mut self) -> _DMAEIENW {
        _DMAEIENW { w: self }
    }
}
