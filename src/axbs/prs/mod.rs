#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRS {
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
#[doc = "Possible values of the field `M0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0R {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M0R::_000 => 0,
            M0R::_001 => 1,
            M0R::_010 => 2,
            M0R::_011 => 3,
            M0R::_100 => 4,
            M0R::_101 => 5,
            M0R::_110 => 6,
            M0R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M0R {
        match value {
            0 => M0R::_000,
            1 => M0R::_001,
            2 => M0R::_010,
            3 => M0R::_011,
            4 => M0R::_100,
            5 => M0R::_101,
            6 => M0R::_110,
            7 => M0R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == M0R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == M0R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == M0R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == M0R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == M0R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == M0R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == M0R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == M0R::_111
    }
}
#[doc = "Possible values of the field `M1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M1R {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M1R::_000 => 0,
            M1R::_001 => 1,
            M1R::_010 => 2,
            M1R::_011 => 3,
            M1R::_100 => 4,
            M1R::_101 => 5,
            M1R::_110 => 6,
            M1R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M1R {
        match value {
            0 => M1R::_000,
            1 => M1R::_001,
            2 => M1R::_010,
            3 => M1R::_011,
            4 => M1R::_100,
            5 => M1R::_101,
            6 => M1R::_110,
            7 => M1R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == M1R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == M1R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == M1R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == M1R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == M1R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == M1R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == M1R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == M1R::_111
    }
}
#[doc = "Possible values of the field `M2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M2R {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M2R::_000 => 0,
            M2R::_001 => 1,
            M2R::_010 => 2,
            M2R::_011 => 3,
            M2R::_100 => 4,
            M2R::_101 => 5,
            M2R::_110 => 6,
            M2R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M2R {
        match value {
            0 => M2R::_000,
            1 => M2R::_001,
            2 => M2R::_010,
            3 => M2R::_011,
            4 => M2R::_100,
            5 => M2R::_101,
            6 => M2R::_110,
            7 => M2R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == M2R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == M2R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == M2R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == M2R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == M2R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == M2R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == M2R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == M2R::_111
    }
}
#[doc = "Possible values of the field `M3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3R {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M3R::_000 => 0,
            M3R::_001 => 1,
            M3R::_010 => 2,
            M3R::_011 => 3,
            M3R::_100 => 4,
            M3R::_101 => 5,
            M3R::_110 => 6,
            M3R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M3R {
        match value {
            0 => M3R::_000,
            1 => M3R::_001,
            2 => M3R::_010,
            3 => M3R::_011,
            4 => M3R::_100,
            5 => M3R::_101,
            6 => M3R::_110,
            7 => M3R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == M3R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == M3R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == M3R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == M3R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == M3R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == M3R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == M3R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == M3R::_111
    }
}
#[doc = "Possible values of the field `M4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4R {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M4R::_000 => 0,
            M4R::_001 => 1,
            M4R::_010 => 2,
            M4R::_011 => 3,
            M4R::_100 => 4,
            M4R::_101 => 5,
            M4R::_110 => 6,
            M4R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M4R {
        match value {
            0 => M4R::_000,
            1 => M4R::_001,
            2 => M4R::_010,
            3 => M4R::_011,
            4 => M4R::_100,
            5 => M4R::_101,
            6 => M4R::_110,
            7 => M4R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == M4R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == M4R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == M4R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == M4R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == M4R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == M4R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == M4R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == M4R::_111
    }
}
#[doc = "Possible values of the field `M5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M5R {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M5R::_000 => 0,
            M5R::_001 => 1,
            M5R::_010 => 2,
            M5R::_011 => 3,
            M5R::_100 => 4,
            M5R::_101 => 5,
            M5R::_110 => 6,
            M5R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M5R {
        match value {
            0 => M5R::_000,
            1 => M5R::_001,
            2 => M5R::_010,
            3 => M5R::_011,
            4 => M5R::_100,
            5 => M5R::_101,
            6 => M5R::_110,
            7 => M5R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == M5R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == M5R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == M5R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == M5R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == M5R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == M5R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == M5R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == M5R::_111
    }
}
#[doc = "Possible values of the field `M6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M6R {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            M6R::_000 => 0,
            M6R::_001 => 1,
            M6R::_010 => 2,
            M6R::_011 => 3,
            M6R::_100 => 4,
            M6R::_101 => 5,
            M6R::_110 => 6,
            M6R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> M6R {
        match value {
            0 => M6R::_000,
            1 => M6R::_001,
            2 => M6R::_010,
            3 => M6R::_011,
            4 => M6R::_100,
            5 => M6R::_101,
            6 => M6R::_110,
            7 => M6R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == M6R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == M6R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == M6R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == M6R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == M6R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == M6R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == M6R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == M6R::_111
    }
}
#[doc = "Values that can be written to the field `M0`"]
pub enum M0W {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M0W::_000 => 0,
            M0W::_001 => 1,
            M0W::_010 => 2,
            M0W::_011 => 3,
            M0W::_100 => 4,
            M0W::_101 => 5,
            M0W::_110 => 6,
            M0W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M0W<'a> {
    w: &'a mut W,
}
impl<'a> _M0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(M0W::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(M0W::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(M0W::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(M0W::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(M0W::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(M0W::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(M0W::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(M0W::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M1`"]
pub enum M1W {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M1W::_000 => 0,
            M1W::_001 => 1,
            M1W::_010 => 2,
            M1W::_011 => 3,
            M1W::_100 => 4,
            M1W::_101 => 5,
            M1W::_110 => 6,
            M1W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M1W<'a> {
    w: &'a mut W,
}
impl<'a> _M1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(M1W::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(M1W::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(M1W::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(M1W::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(M1W::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(M1W::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(M1W::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(M1W::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M2`"]
pub enum M2W {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M2W::_000 => 0,
            M2W::_001 => 1,
            M2W::_010 => 2,
            M2W::_011 => 3,
            M2W::_100 => 4,
            M2W::_101 => 5,
            M2W::_110 => 6,
            M2W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M2W<'a> {
    w: &'a mut W,
}
impl<'a> _M2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(M2W::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(M2W::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(M2W::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(M2W::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(M2W::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(M2W::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(M2W::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(M2W::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M3`"]
pub enum M3W {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M3W::_000 => 0,
            M3W::_001 => 1,
            M3W::_010 => 2,
            M3W::_011 => 3,
            M3W::_100 => 4,
            M3W::_101 => 5,
            M3W::_110 => 6,
            M3W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M3W<'a> {
    w: &'a mut W,
}
impl<'a> _M3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(M3W::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(M3W::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(M3W::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(M3W::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(M3W::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(M3W::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(M3W::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(M3W::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M4`"]
pub enum M4W {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M4W::_000 => 0,
            M4W::_001 => 1,
            M4W::_010 => 2,
            M4W::_011 => 3,
            M4W::_100 => 4,
            M4W::_101 => 5,
            M4W::_110 => 6,
            M4W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M4W<'a> {
    w: &'a mut W,
}
impl<'a> _M4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(M4W::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(M4W::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(M4W::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(M4W::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(M4W::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(M4W::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(M4W::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(M4W::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M5`"]
pub enum M5W {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M5W::_000 => 0,
            M5W::_001 => 1,
            M5W::_010 => 2,
            M5W::_011 => 3,
            M5W::_100 => 4,
            M5W::_101 => 5,
            M5W::_110 => 6,
            M5W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M5W<'a> {
    w: &'a mut W,
}
impl<'a> _M5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(M5W::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(M5W::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(M5W::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(M5W::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(M5W::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(M5W::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(M5W::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(M5W::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M6`"]
pub enum M6W {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl M6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            M6W::_000 => 0,
            M6W::_001 => 1,
            M6W::_010 => 2,
            M6W::_011 => 3,
            M6W::_100 => 4,
            M6W::_101 => 5,
            M6W::_110 => 6,
            M6W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M6W<'a> {
    w: &'a mut W,
}
impl<'a> _M6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(M6W::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(M6W::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(M6W::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(M6W::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(M6W::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(M6W::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(M6W::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(M6W::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:2 - Master 0 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m0(&self) -> M0R {
        M0R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Master 1 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m1(&self) -> M1R {
        M1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Master 2 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m2(&self) -> M2R {
        M2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Master 3 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m3(&self) -> M3R {
        M3R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Master 4 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m4(&self) -> M4R {
        M4R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - Master 5 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m5(&self) -> M5R {
        M5R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Master 6 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m6(&self) -> M6R {
        M6R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 106181136 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Master 0 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m0(&mut self) -> _M0W {
        _M0W { w: self }
    }
    #[doc = "Bits 4:6 - Master 1 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m1(&mut self) -> _M1W {
        _M1W { w: self }
    }
    #[doc = "Bits 8:10 - Master 2 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m2(&mut self) -> _M2W {
        _M2W { w: self }
    }
    #[doc = "Bits 12:14 - Master 3 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m3(&mut self) -> _M3W {
        _M3W { w: self }
    }
    #[doc = "Bits 16:18 - Master 4 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m4(&mut self) -> _M4W {
        _M4W { w: self }
    }
    #[doc = "Bits 20:22 - Master 5 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m5(&mut self) -> _M5W {
        _M5W { w: self }
    }
    #[doc = "Bits 24:26 - Master 6 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline]
    pub fn m6(&mut self) -> _M6W {
        _M6W { w: self }
    }
}
