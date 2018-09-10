#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PFAPR {
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
#[doc = "Possible values of the field `M0AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0APR {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M0APR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M0APR::_00 => 0,
            M0APR::_01 => 1,
            M0APR::_10 => 2,
            M0APR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M0APR {
        match value {
            0 => M0APR::_00,
            1 => M0APR::_01,
            2 => M0APR::_10,
            3 => M0APR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == M0APR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == M0APR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == M0APR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == M0APR::_11
    }
}
#[doc = "Possible values of the field `M1AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M1APR {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M1APR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M1APR::_00 => 0,
            M1APR::_01 => 1,
            M1APR::_10 => 2,
            M1APR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M1APR {
        match value {
            0 => M1APR::_00,
            1 => M1APR::_01,
            2 => M1APR::_10,
            3 => M1APR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == M1APR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == M1APR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == M1APR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == M1APR::_11
    }
}
#[doc = "Possible values of the field `M2AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M2APR {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M2APR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M2APR::_00 => 0,
            M2APR::_01 => 1,
            M2APR::_10 => 2,
            M2APR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M2APR {
        match value {
            0 => M2APR::_00,
            1 => M2APR::_01,
            2 => M2APR::_10,
            3 => M2APR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == M2APR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == M2APR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == M2APR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == M2APR::_11
    }
}
#[doc = "Possible values of the field `M3AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3APR {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M3APR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M3APR::_00 => 0,
            M3APR::_01 => 1,
            M3APR::_10 => 2,
            M3APR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M3APR {
        match value {
            0 => M3APR::_00,
            1 => M3APR::_01,
            2 => M3APR::_10,
            3 => M3APR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == M3APR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == M3APR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == M3APR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == M3APR::_11
    }
}
#[doc = "Possible values of the field `M4AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4APR {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M4APR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M4APR::_00 => 0,
            M4APR::_01 => 1,
            M4APR::_10 => 2,
            M4APR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M4APR {
        match value {
            0 => M4APR::_00,
            1 => M4APR::_01,
            2 => M4APR::_10,
            3 => M4APR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == M4APR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == M4APR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == M4APR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == M4APR::_11
    }
}
#[doc = "Possible values of the field `M5AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M5APR {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M5APR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M5APR::_00 => 0,
            M5APR::_01 => 1,
            M5APR::_10 => 2,
            M5APR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M5APR {
        match value {
            0 => M5APR::_00,
            1 => M5APR::_01,
            2 => M5APR::_10,
            3 => M5APR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == M5APR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == M5APR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == M5APR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == M5APR::_11
    }
}
#[doc = "Possible values of the field `M6AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M6APR {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M6APR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M6APR::_00 => 0,
            M6APR::_01 => 1,
            M6APR::_10 => 2,
            M6APR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M6APR {
        match value {
            0 => M6APR::_00,
            1 => M6APR::_01,
            2 => M6APR::_10,
            3 => M6APR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == M6APR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == M6APR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == M6APR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == M6APR::_11
    }
}
#[doc = "Possible values of the field `M7AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7APR {
    #[doc = "No access may be performed by this master."]
    _00,
    #[doc = "Only read accesses may be performed by this master."]
    _01,
    #[doc = "Only write accesses may be performed by this master."]
    _10,
    #[doc = "Both read and write accesses may be performed by this master."]
    _11,
}
impl M7APR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M7APR::_00 => 0,
            M7APR::_01 => 1,
            M7APR::_10 => 2,
            M7APR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M7APR {
        match value {
            0 => M7APR::_00,
            1 => M7APR::_01,
            2 => M7APR::_10,
            3 => M7APR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == M7APR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == M7APR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == M7APR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == M7APR::_11
    }
}
#[doc = "Possible values of the field `M0PFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0PFDR {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M0PFDR {
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
            M0PFDR::_0 => false,
            M0PFDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M0PFDR {
        match value {
            false => M0PFDR::_0,
            true => M0PFDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M0PFDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M0PFDR::_1
    }
}
#[doc = "Possible values of the field `M1PFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M1PFDR {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M1PFDR {
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
            M1PFDR::_0 => false,
            M1PFDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M1PFDR {
        match value {
            false => M1PFDR::_0,
            true => M1PFDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M1PFDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M1PFDR::_1
    }
}
#[doc = "Possible values of the field `M2PFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M2PFDR {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M2PFDR {
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
            M2PFDR::_0 => false,
            M2PFDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M2PFDR {
        match value {
            false => M2PFDR::_0,
            true => M2PFDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M2PFDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M2PFDR::_1
    }
}
#[doc = "Possible values of the field `M3PFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3PFDR {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M3PFDR {
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
            M3PFDR::_0 => false,
            M3PFDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M3PFDR {
        match value {
            false => M3PFDR::_0,
            true => M3PFDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M3PFDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M3PFDR::_1
    }
}
#[doc = "Possible values of the field `M4PFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4PFDR {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M4PFDR {
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
            M4PFDR::_0 => false,
            M4PFDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M4PFDR {
        match value {
            false => M4PFDR::_0,
            true => M4PFDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M4PFDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M4PFDR::_1
    }
}
#[doc = "Possible values of the field `M5PFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M5PFDR {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M5PFDR {
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
            M5PFDR::_0 => false,
            M5PFDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M5PFDR {
        match value {
            false => M5PFDR::_0,
            true => M5PFDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M5PFDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M5PFDR::_1
    }
}
#[doc = "Possible values of the field `M6PFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M6PFDR {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M6PFDR {
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
            M6PFDR::_0 => false,
            M6PFDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M6PFDR {
        match value {
            false => M6PFDR::_0,
            true => M6PFDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M6PFDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M6PFDR::_1
    }
}
#[doc = "Possible values of the field `M7PFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7PFDR {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M7PFDR {
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
            M7PFDR::_0 => false,
            M7PFDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M7PFDR {
        match value {
            false => M7PFDR::_0,
            true => M7PFDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M7PFDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M7PFDR::_1
    }
}
#[doc = "Values that can be written to the field `M0AP`"]
pub enum M0APW {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M0APW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M0APW::_00 => 0,
            M0APW::_01 => 1,
            M0APW::_10 => 2,
            M0APW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M0APW<'a> {
    w: &'a mut W,
}
impl<'a> _M0APW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M0APW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(M0APW::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(M0APW::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(M0APW::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(M0APW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M1AP`"]
pub enum M1APW {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M1APW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M1APW::_00 => 0,
            M1APW::_01 => 1,
            M1APW::_10 => 2,
            M1APW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M1APW<'a> {
    w: &'a mut W,
}
impl<'a> _M1APW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M1APW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(M1APW::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(M1APW::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(M1APW::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(M1APW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M2AP`"]
pub enum M2APW {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M2APW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M2APW::_00 => 0,
            M2APW::_01 => 1,
            M2APW::_10 => 2,
            M2APW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M2APW<'a> {
    w: &'a mut W,
}
impl<'a> _M2APW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M2APW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(M2APW::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(M2APW::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(M2APW::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(M2APW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M3AP`"]
pub enum M3APW {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M3APW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M3APW::_00 => 0,
            M3APW::_01 => 1,
            M3APW::_10 => 2,
            M3APW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M3APW<'a> {
    w: &'a mut W,
}
impl<'a> _M3APW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M3APW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(M3APW::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(M3APW::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(M3APW::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(M3APW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M4AP`"]
pub enum M4APW {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M4APW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M4APW::_00 => 0,
            M4APW::_01 => 1,
            M4APW::_10 => 2,
            M4APW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M4APW<'a> {
    w: &'a mut W,
}
impl<'a> _M4APW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M4APW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(M4APW::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(M4APW::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(M4APW::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(M4APW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M5AP`"]
pub enum M5APW {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M5APW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M5APW::_00 => 0,
            M5APW::_01 => 1,
            M5APW::_10 => 2,
            M5APW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M5APW<'a> {
    w: &'a mut W,
}
impl<'a> _M5APW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M5APW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(M5APW::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(M5APW::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(M5APW::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(M5APW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M6AP`"]
pub enum M6APW {
    #[doc = "No access may be performed by this master"]
    _00,
    #[doc = "Only read accesses may be performed by this master"]
    _01,
    #[doc = "Only write accesses may be performed by this master"]
    _10,
    #[doc = "Both read and write accesses may be performed by this master"]
    _11,
}
impl M6APW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M6APW::_00 => 0,
            M6APW::_01 => 1,
            M6APW::_10 => 2,
            M6APW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M6APW<'a> {
    w: &'a mut W,
}
impl<'a> _M6APW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M6APW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(M6APW::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(M6APW::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(M6APW::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(M6APW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M7AP`"]
pub enum M7APW {
    #[doc = "No access may be performed by this master."]
    _00,
    #[doc = "Only read accesses may be performed by this master."]
    _01,
    #[doc = "Only write accesses may be performed by this master."]
    _10,
    #[doc = "Both read and write accesses may be performed by this master."]
    _11,
}
impl M7APW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M7APW::_00 => 0,
            M7APW::_01 => 1,
            M7APW::_10 => 2,
            M7APW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M7APW<'a> {
    w: &'a mut W,
}
impl<'a> _M7APW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M7APW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No access may be performed by this master."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(M7APW::_00)
    }
    #[doc = "Only read accesses may be performed by this master."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(M7APW::_01)
    }
    #[doc = "Only write accesses may be performed by this master."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(M7APW::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(M7APW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M0PFD`"]
pub enum M0PFDW {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M0PFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M0PFDW::_0 => false,
            M0PFDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M0PFDW<'a> {
    w: &'a mut W,
}
impl<'a> _M0PFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M0PFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M0PFDW::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M0PFDW::_1)
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
#[doc = "Values that can be written to the field `M1PFD`"]
pub enum M1PFDW {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M1PFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M1PFDW::_0 => false,
            M1PFDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M1PFDW<'a> {
    w: &'a mut W,
}
impl<'a> _M1PFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M1PFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M1PFDW::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M1PFDW::_1)
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
#[doc = "Values that can be written to the field `M2PFD`"]
pub enum M2PFDW {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M2PFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M2PFDW::_0 => false,
            M2PFDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M2PFDW<'a> {
    w: &'a mut W,
}
impl<'a> _M2PFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M2PFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M2PFDW::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M2PFDW::_1)
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
#[doc = "Values that can be written to the field `M3PFD`"]
pub enum M3PFDW {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M3PFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M3PFDW::_0 => false,
            M3PFDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M3PFDW<'a> {
    w: &'a mut W,
}
impl<'a> _M3PFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M3PFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M3PFDW::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M3PFDW::_1)
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
#[doc = "Values that can be written to the field `M4PFD`"]
pub enum M4PFDW {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M4PFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M4PFDW::_0 => false,
            M4PFDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M4PFDW<'a> {
    w: &'a mut W,
}
impl<'a> _M4PFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M4PFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M4PFDW::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M4PFDW::_1)
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
#[doc = "Values that can be written to the field `M5PFD`"]
pub enum M5PFDW {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M5PFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M5PFDW::_0 => false,
            M5PFDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M5PFDW<'a> {
    w: &'a mut W,
}
impl<'a> _M5PFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M5PFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M5PFDW::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M5PFDW::_1)
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
#[doc = "Values that can be written to the field `M6PFD`"]
pub enum M6PFDW {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M6PFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M6PFDW::_0 => false,
            M6PFDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M6PFDW<'a> {
    w: &'a mut W,
}
impl<'a> _M6PFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M6PFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M6PFDW::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M6PFDW::_1)
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
#[doc = "Values that can be written to the field `M7PFD`"]
pub enum M7PFDW {
    #[doc = "Prefetching for this master is enabled."]
    _0,
    #[doc = "Prefetching for this master is disabled."]
    _1,
}
impl M7PFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M7PFDW::_0 => false,
            M7PFDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M7PFDW<'a> {
    w: &'a mut W,
}
impl<'a> _M7PFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M7PFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M7PFDW::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M7PFDW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Master 0 Access Protection"]
    #[inline]
    pub fn m0ap(&self) -> M0APR {
        M0APR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Master 1 Access Protection"]
    #[inline]
    pub fn m1ap(&self) -> M1APR {
        M1APR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Master 2 Access Protection"]
    #[inline]
    pub fn m2ap(&self) -> M2APR {
        M2APR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Master 3 Access Protection"]
    #[inline]
    pub fn m3ap(&self) -> M3APR {
        M3APR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Master 4 Access Protection"]
    #[inline]
    pub fn m4ap(&self) -> M4APR {
        M4APR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Master 5 Access Protection"]
    #[inline]
    pub fn m5ap(&self) -> M5APR {
        M5APR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Master 6 Access Protection"]
    #[inline]
    pub fn m6ap(&self) -> M6APR {
        M6APR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Master 7 Access Protection"]
    #[inline]
    pub fn m7ap(&self) -> M7APR {
        M7APR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Master 0 Prefetch Disable"]
    #[inline]
    pub fn m0pfd(&self) -> M0PFDR {
        M0PFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Master 1 Prefetch Disable"]
    #[inline]
    pub fn m1pfd(&self) -> M1PFDR {
        M1PFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Master 2 Prefetch Disable"]
    #[inline]
    pub fn m2pfd(&self) -> M2PFDR {
        M2PFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Master 3 Prefetch Disable"]
    #[inline]
    pub fn m3pfd(&self) -> M3PFDR {
        M3PFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Master 4 Prefetch Disable"]
    #[inline]
    pub fn m4pfd(&self) -> M4PFDR {
        M4PFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Master 5 Prefetch Disable"]
    #[inline]
    pub fn m5pfd(&self) -> M5PFDR {
        M5PFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Master 6 Prefetch Disable"]
    #[inline]
    pub fn m6pfd(&self) -> M6PFDR {
        M6PFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Master 7 Prefetch Disable"]
    #[inline]
    pub fn m7pfd(&self) -> M7PFDR {
        M7PFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16252991 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Master 0 Access Protection"]
    #[inline]
    pub fn m0ap(&mut self) -> _M0APW {
        _M0APW { w: self }
    }
    #[doc = "Bits 2:3 - Master 1 Access Protection"]
    #[inline]
    pub fn m1ap(&mut self) -> _M1APW {
        _M1APW { w: self }
    }
    #[doc = "Bits 4:5 - Master 2 Access Protection"]
    #[inline]
    pub fn m2ap(&mut self) -> _M2APW {
        _M2APW { w: self }
    }
    #[doc = "Bits 6:7 - Master 3 Access Protection"]
    #[inline]
    pub fn m3ap(&mut self) -> _M3APW {
        _M3APW { w: self }
    }
    #[doc = "Bits 8:9 - Master 4 Access Protection"]
    #[inline]
    pub fn m4ap(&mut self) -> _M4APW {
        _M4APW { w: self }
    }
    #[doc = "Bits 10:11 - Master 5 Access Protection"]
    #[inline]
    pub fn m5ap(&mut self) -> _M5APW {
        _M5APW { w: self }
    }
    #[doc = "Bits 12:13 - Master 6 Access Protection"]
    #[inline]
    pub fn m6ap(&mut self) -> _M6APW {
        _M6APW { w: self }
    }
    #[doc = "Bits 14:15 - Master 7 Access Protection"]
    #[inline]
    pub fn m7ap(&mut self) -> _M7APW {
        _M7APW { w: self }
    }
    #[doc = "Bit 16 - Master 0 Prefetch Disable"]
    #[inline]
    pub fn m0pfd(&mut self) -> _M0PFDW {
        _M0PFDW { w: self }
    }
    #[doc = "Bit 17 - Master 1 Prefetch Disable"]
    #[inline]
    pub fn m1pfd(&mut self) -> _M1PFDW {
        _M1PFDW { w: self }
    }
    #[doc = "Bit 18 - Master 2 Prefetch Disable"]
    #[inline]
    pub fn m2pfd(&mut self) -> _M2PFDW {
        _M2PFDW { w: self }
    }
    #[doc = "Bit 19 - Master 3 Prefetch Disable"]
    #[inline]
    pub fn m3pfd(&mut self) -> _M3PFDW {
        _M3PFDW { w: self }
    }
    #[doc = "Bit 20 - Master 4 Prefetch Disable"]
    #[inline]
    pub fn m4pfd(&mut self) -> _M4PFDW {
        _M4PFDW { w: self }
    }
    #[doc = "Bit 21 - Master 5 Prefetch Disable"]
    #[inline]
    pub fn m5pfd(&mut self) -> _M5PFDW {
        _M5PFDW { w: self }
    }
    #[doc = "Bit 22 - Master 6 Prefetch Disable"]
    #[inline]
    pub fn m6pfd(&mut self) -> _M6PFDW {
        _M6PFDW { w: self }
    }
    #[doc = "Bit 23 - Master 7 Prefetch Disable"]
    #[inline]
    pub fn m7pfd(&mut self) -> _M7PFDW {
        _M7PFDW { w: self }
    }
}
