#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GENCS {
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
#[doc = "Possible values of the field `EOSDMEO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSDMEOR {
    #[doc = "Do not enable the End-of-Scan DMA transfer request only. Depending on ESOR state, either Out-of-Range or End-of-Scan can trigger a DMA transfer request and interrupt."]
    _0,
    #[doc = "Only the End-of-Scan event can trigger a DMA transfer request. The Out-of-Range event only and always triggers an interrupt if TSIIE is set."]
    _1,
}
impl EOSDMEOR {
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
            EOSDMEOR::_0 => false,
            EOSDMEOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOSDMEOR {
        match value {
            false => EOSDMEOR::_0,
            true => EOSDMEOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EOSDMEOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EOSDMEOR::_1
    }
}
#[doc = "Possible values of the field `CURSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURSWR {
    #[doc = "The current source pair are not swapped."]
    _0,
    #[doc = "The current source pair are swapped."]
    _1,
}
impl CURSWR {
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
            CURSWR::_0 => false,
            CURSWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CURSWR {
        match value {
            false => CURSWR::_0,
            true => CURSWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CURSWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CURSWR::_1
    }
}
#[doc = "Possible values of the field `EOSF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOSFR {
    #[doc = "Scan not complete."]
    _0,
    #[doc = "Scan complete."]
    _1,
}
impl EOSFR {
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
            EOSFR::_0 => false,
            EOSFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOSFR {
        match value {
            false => EOSFR::_0,
            true => EOSFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EOSFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EOSFR::_1
    }
}
#[doc = "Possible values of the field `SCNIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCNIPR {
    #[doc = "No scan in progress."]
    _0,
    #[doc = "Scan in progress."]
    _1,
}
impl SCNIPR {
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
            SCNIPR::_0 => false,
            SCNIPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCNIPR {
        match value {
            false => SCNIPR::_0,
            true => SCNIPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SCNIPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SCNIPR::_1
    }
}
#[doc = "Possible values of the field `STM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STMR {
    #[doc = "Software trigger scan."]
    _0,
    #[doc = "Hardware trigger scan."]
    _1,
}
impl STMR {
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
            STMR::_0 => false,
            STMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STMR {
        match value {
            false => STMR::_0,
            true => STMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STMR::_1
    }
}
#[doc = "Possible values of the field `STPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPER {
    #[doc = "TSI is disabled when MCU goes into low power mode."]
    _0,
    #[doc = "Allows TSI to continue running in all low power modes."]
    _1,
}
impl STPER {
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
            STPER::_0 => false,
            STPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STPER {
        match value {
            false => STPER::_0,
            true => STPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STPER::_1
    }
}
#[doc = "Possible values of the field `TSIIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIIENR {
    #[doc = "TSI interrupt is disabled."]
    _0,
    #[doc = "TSI interrupt is enabled."]
    _1,
}
impl TSIIENR {
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
            TSIIENR::_0 => false,
            TSIIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSIIENR {
        match value {
            false => TSIIENR::_0,
            true => TSIIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSIIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSIIENR::_1
    }
}
#[doc = "Possible values of the field `TSIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIENR {
    #[doc = "TSI module disabled."]
    _0,
    #[doc = "TSI module enabled."]
    _1,
}
impl TSIENR {
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
            TSIENR::_0 => false,
            TSIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSIENR {
        match value {
            false => TSIENR::_0,
            true => TSIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSIENR::_1
    }
}
#[doc = "Possible values of the field `NSCN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSCNR {
    #[doc = "Once per electrode"]
    _00000,
    #[doc = "Twice per electrode"]
    _00001,
    #[doc = "3 times per electrode"]
    _00010,
    #[doc = "4 times per electrode"]
    _00011,
    #[doc = "5 times per electrode"]
    _00100,
    #[doc = "6 times per electrode"]
    _00101,
    #[doc = "7 times per electrode"]
    _00110,
    #[doc = "8 times per electrode"]
    _00111,
    #[doc = "9 times per electrode"]
    _01000,
    #[doc = "10 times per electrode"]
    _01001,
    #[doc = "11 times per electrode"]
    _01010,
    #[doc = "12 times per electrode"]
    _01011,
    #[doc = "13 times per electrode"]
    _01100,
    #[doc = "14 times per electrode"]
    _01101,
    #[doc = "15 times per electrode"]
    _01110,
    #[doc = "16 times per electrode"]
    _01111,
    #[doc = "17 times per electrode"]
    _10000,
    #[doc = "18 times per electrode"]
    _10001,
    #[doc = "19 times per electrode"]
    _10010,
    #[doc = "20 times per electrode"]
    _10011,
    #[doc = "21 times per electrode"]
    _10100,
    #[doc = "22 times per electrode"]
    _10101,
    #[doc = "23 times per electrode"]
    _10110,
    #[doc = "24 times per electrode"]
    _10111,
    #[doc = "25 times per electrode"]
    _11000,
    #[doc = "26 times per electrode"]
    _11001,
    #[doc = "27 times per electrode"]
    _11010,
    #[doc = "28 times per electrode"]
    _11011,
    #[doc = "29 times per electrode"]
    _11100,
    #[doc = "30 times per electrode"]
    _11101,
    #[doc = "31 times per electrode"]
    _11110,
    #[doc = "32 times per electrode"]
    _11111,
}
impl NSCNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NSCNR::_00000 => 0,
            NSCNR::_00001 => 1,
            NSCNR::_00010 => 2,
            NSCNR::_00011 => 3,
            NSCNR::_00100 => 4,
            NSCNR::_00101 => 5,
            NSCNR::_00110 => 6,
            NSCNR::_00111 => 7,
            NSCNR::_01000 => 8,
            NSCNR::_01001 => 9,
            NSCNR::_01010 => 10,
            NSCNR::_01011 => 11,
            NSCNR::_01100 => 12,
            NSCNR::_01101 => 13,
            NSCNR::_01110 => 14,
            NSCNR::_01111 => 15,
            NSCNR::_10000 => 16,
            NSCNR::_10001 => 17,
            NSCNR::_10010 => 18,
            NSCNR::_10011 => 19,
            NSCNR::_10100 => 20,
            NSCNR::_10101 => 21,
            NSCNR::_10110 => 22,
            NSCNR::_10111 => 23,
            NSCNR::_11000 => 24,
            NSCNR::_11001 => 25,
            NSCNR::_11010 => 26,
            NSCNR::_11011 => 27,
            NSCNR::_11100 => 28,
            NSCNR::_11101 => 29,
            NSCNR::_11110 => 30,
            NSCNR::_11111 => 31,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NSCNR {
        match value {
            0 => NSCNR::_00000,
            1 => NSCNR::_00001,
            2 => NSCNR::_00010,
            3 => NSCNR::_00011,
            4 => NSCNR::_00100,
            5 => NSCNR::_00101,
            6 => NSCNR::_00110,
            7 => NSCNR::_00111,
            8 => NSCNR::_01000,
            9 => NSCNR::_01001,
            10 => NSCNR::_01010,
            11 => NSCNR::_01011,
            12 => NSCNR::_01100,
            13 => NSCNR::_01101,
            14 => NSCNR::_01110,
            15 => NSCNR::_01111,
            16 => NSCNR::_10000,
            17 => NSCNR::_10001,
            18 => NSCNR::_10010,
            19 => NSCNR::_10011,
            20 => NSCNR::_10100,
            21 => NSCNR::_10101,
            22 => NSCNR::_10110,
            23 => NSCNR::_10111,
            24 => NSCNR::_11000,
            25 => NSCNR::_11001,
            26 => NSCNR::_11010,
            27 => NSCNR::_11011,
            28 => NSCNR::_11100,
            29 => NSCNR::_11101,
            30 => NSCNR::_11110,
            31 => NSCNR::_11111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline]
    pub fn is_00000(&self) -> bool {
        *self == NSCNR::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline]
    pub fn is_00001(&self) -> bool {
        *self == NSCNR::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline]
    pub fn is_00010(&self) -> bool {
        *self == NSCNR::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline]
    pub fn is_00011(&self) -> bool {
        *self == NSCNR::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline]
    pub fn is_00100(&self) -> bool {
        *self == NSCNR::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline]
    pub fn is_00101(&self) -> bool {
        *self == NSCNR::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline]
    pub fn is_00110(&self) -> bool {
        *self == NSCNR::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline]
    pub fn is_00111(&self) -> bool {
        *self == NSCNR::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline]
    pub fn is_01000(&self) -> bool {
        *self == NSCNR::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline]
    pub fn is_01001(&self) -> bool {
        *self == NSCNR::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline]
    pub fn is_01010(&self) -> bool {
        *self == NSCNR::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline]
    pub fn is_01011(&self) -> bool {
        *self == NSCNR::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline]
    pub fn is_01100(&self) -> bool {
        *self == NSCNR::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline]
    pub fn is_01101(&self) -> bool {
        *self == NSCNR::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline]
    pub fn is_01110(&self) -> bool {
        *self == NSCNR::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline]
    pub fn is_01111(&self) -> bool {
        *self == NSCNR::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline]
    pub fn is_10000(&self) -> bool {
        *self == NSCNR::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline]
    pub fn is_10001(&self) -> bool {
        *self == NSCNR::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline]
    pub fn is_10010(&self) -> bool {
        *self == NSCNR::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline]
    pub fn is_10011(&self) -> bool {
        *self == NSCNR::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline]
    pub fn is_10100(&self) -> bool {
        *self == NSCNR::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline]
    pub fn is_10101(&self) -> bool {
        *self == NSCNR::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline]
    pub fn is_10110(&self) -> bool {
        *self == NSCNR::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline]
    pub fn is_10111(&self) -> bool {
        *self == NSCNR::_10111
    }
    #[doc = "Checks if the value of the field is `_11000`"]
    #[inline]
    pub fn is_11000(&self) -> bool {
        *self == NSCNR::_11000
    }
    #[doc = "Checks if the value of the field is `_11001`"]
    #[inline]
    pub fn is_11001(&self) -> bool {
        *self == NSCNR::_11001
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline]
    pub fn is_11010(&self) -> bool {
        *self == NSCNR::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline]
    pub fn is_11011(&self) -> bool {
        *self == NSCNR::_11011
    }
    #[doc = "Checks if the value of the field is `_11100`"]
    #[inline]
    pub fn is_11100(&self) -> bool {
        *self == NSCNR::_11100
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline]
    pub fn is_11101(&self) -> bool {
        *self == NSCNR::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline]
    pub fn is_11110(&self) -> bool {
        *self == NSCNR::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline]
    pub fn is_11111(&self) -> bool {
        *self == NSCNR::_11111
    }
}
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "Electrode Oscillator Frequency divided by 1"]
    _000,
    #[doc = "Electrode Oscillator Frequency divided by 2"]
    _001,
    #[doc = "Electrode Oscillator Frequency divided by 4"]
    _010,
    #[doc = "Electrode Oscillator Frequency divided by 8"]
    _011,
    #[doc = "Electrode Oscillator Frequency divided by 16"]
    _100,
    #[doc = "Electrode Oscillator Frequency divided by 32"]
    _101,
    #[doc = "Electrode Oscillator Frequency divided by 64"]
    _110,
    #[doc = "Electrode Oscillator Frequency divided by 128"]
    _111,
}
impl PSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSR::_000 => 0,
            PSR::_001 => 1,
            PSR::_010 => 2,
            PSR::_011 => 3,
            PSR::_100 => 4,
            PSR::_101 => 5,
            PSR::_110 => 6,
            PSR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSR {
        match value {
            0 => PSR::_000,
            1 => PSR::_001,
            2 => PSR::_010,
            3 => PSR::_011,
            4 => PSR::_100,
            5 => PSR::_101,
            6 => PSR::_110,
            7 => PSR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PSR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PSR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PSR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PSR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PSR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PSR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PSR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PSR::_111
    }
}
#[doc = "Possible values of the field `EXTCHRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTCHRGR {
    #[doc = "500 nA."]
    _000,
    #[doc = "1 uA."]
    _001,
    #[doc = "2 uA."]
    _010,
    #[doc = "4 uA."]
    _011,
    #[doc = "8 uA."]
    _100,
    #[doc = "16 uA."]
    _101,
    #[doc = "32 uA."]
    _110,
    #[doc = "64 uA."]
    _111,
}
impl EXTCHRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTCHRGR::_000 => 0,
            EXTCHRGR::_001 => 1,
            EXTCHRGR::_010 => 2,
            EXTCHRGR::_011 => 3,
            EXTCHRGR::_100 => 4,
            EXTCHRGR::_101 => 5,
            EXTCHRGR::_110 => 6,
            EXTCHRGR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTCHRGR {
        match value {
            0 => EXTCHRGR::_000,
            1 => EXTCHRGR::_001,
            2 => EXTCHRGR::_010,
            3 => EXTCHRGR::_011,
            4 => EXTCHRGR::_100,
            5 => EXTCHRGR::_101,
            6 => EXTCHRGR::_110,
            7 => EXTCHRGR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == EXTCHRGR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == EXTCHRGR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == EXTCHRGR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == EXTCHRGR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == EXTCHRGR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == EXTCHRGR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == EXTCHRGR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == EXTCHRGR::_111
    }
}
#[doc = "Possible values of the field `DVOLT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DVOLTR {
    #[doc = "DV = 1.026 V; VP = 1.328 V; Vm = 0.302 V."]
    _00,
    #[doc = "DV = 0.592 V; VP = 1.111 V; Vm = 0.519 V."]
    _01,
    #[doc = "DV = 0.342 V; VP = 0.986 V; Vm = 0.644 V."]
    _10,
    #[doc = "DV = 0.197 V; VP = 0.914 V; Vm = 0.716 V."]
    _11,
}
impl DVOLTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DVOLTR::_00 => 0,
            DVOLTR::_01 => 1,
            DVOLTR::_10 => 2,
            DVOLTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DVOLTR {
        match value {
            0 => DVOLTR::_00,
            1 => DVOLTR::_01,
            2 => DVOLTR::_10,
            3 => DVOLTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DVOLTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DVOLTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DVOLTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DVOLTR::_11
    }
}
#[doc = "Possible values of the field `REFCHRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFCHRGR {
    #[doc = "500 nA."]
    _000,
    #[doc = "1 uA."]
    _001,
    #[doc = "2 uA."]
    _010,
    #[doc = "4 uA."]
    _011,
    #[doc = "8 uA."]
    _100,
    #[doc = "16 uA."]
    _101,
    #[doc = "32 uA."]
    _110,
    #[doc = "64 uA."]
    _111,
}
impl REFCHRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFCHRGR::_000 => 0,
            REFCHRGR::_001 => 1,
            REFCHRGR::_010 => 2,
            REFCHRGR::_011 => 3,
            REFCHRGR::_100 => 4,
            REFCHRGR::_101 => 5,
            REFCHRGR::_110 => 6,
            REFCHRGR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFCHRGR {
        match value {
            0 => REFCHRGR::_000,
            1 => REFCHRGR::_001,
            2 => REFCHRGR::_010,
            3 => REFCHRGR::_011,
            4 => REFCHRGR::_100,
            5 => REFCHRGR::_101,
            6 => REFCHRGR::_110,
            7 => REFCHRGR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == REFCHRGR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == REFCHRGR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == REFCHRGR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == REFCHRGR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == REFCHRGR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == REFCHRGR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == REFCHRGR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == REFCHRGR::_111
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Set TSI in capacitive sensing(non-noise detection) mode."]
    _0000,
    #[doc = "Set TSI analog to work in single threshold noise detection mode and the frequency limitation circuit is disabled."]
    _0100,
    #[doc = "Set TSI analog to work in single threshold noise detection mode and the frequency limitation circuit is enabled to work in higher frequencies operations."]
    _1000,
    #[doc = "Set TSI analog to work in automatic noise detection mode."]
    _1100,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::_0000 => 0,
            MODER::_0100 => 4,
            MODER::_1000 => 8,
            MODER::_1100 => 12,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::_0000,
            4 => MODER::_0100,
            8 => MODER::_1000,
            12 => MODER::_1100,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == MODER::_0000
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == MODER::_0100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == MODER::_1000
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == MODER::_1100
    }
}
#[doc = "Possible values of the field `ESOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESORR {
    #[doc = "Out-of-range interrupt is allowed."]
    _0,
    #[doc = "End-of-scan interrupt is allowed."]
    _1,
}
impl ESORR {
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
            ESORR::_0 => false,
            ESORR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESORR {
        match value {
            false => ESORR::_0,
            true => ESORR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ESORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ESORR::_1
    }
}
#[doc = r" Value of the field"]
pub struct OUTRGFR {
    bits: bool,
}
impl OUTRGFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `EOSDMEO`"]
pub enum EOSDMEOW {
    #[doc = "Do not enable the End-of-Scan DMA transfer request only. Depending on ESOR state, either Out-of-Range or End-of-Scan can trigger a DMA transfer request and interrupt."]
    _0,
    #[doc = "Only the End-of-Scan event can trigger a DMA transfer request. The Out-of-Range event only and always triggers an interrupt if TSIIE is set."]
    _1,
}
impl EOSDMEOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOSDMEOW::_0 => false,
            EOSDMEOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOSDMEOW<'a> {
    w: &'a mut W,
}
impl<'a> _EOSDMEOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOSDMEOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not enable the End-of-Scan DMA transfer request only. Depending on ESOR state, either Out-of-Range or End-of-Scan can trigger a DMA transfer request and interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSDMEOW::_0)
    }
    #[doc = "Only the End-of-Scan event can trigger a DMA transfer request. The Out-of-Range event only and always triggers an interrupt if TSIIE is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSDMEOW::_1)
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
#[doc = "Values that can be written to the field `CURSW`"]
pub enum CURSWW {
    #[doc = "The current source pair are not swapped."]
    _0,
    #[doc = "The current source pair are swapped."]
    _1,
}
impl CURSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CURSWW::_0 => false,
            CURSWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CURSWW<'a> {
    w: &'a mut W,
}
impl<'a> _CURSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CURSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The current source pair are not swapped."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CURSWW::_0)
    }
    #[doc = "The current source pair are swapped."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CURSWW::_1)
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
#[doc = "Values that can be written to the field `EOSF`"]
pub enum EOSFW {
    #[doc = "Scan not complete."]
    _0,
    #[doc = "Scan complete."]
    _1,
}
impl EOSFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOSFW::_0 => false,
            EOSFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOSFW<'a> {
    w: &'a mut W,
}
impl<'a> _EOSFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOSFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Scan not complete."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOSFW::_0)
    }
    #[doc = "Scan complete."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOSFW::_1)
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
#[doc = "Values that can be written to the field `STM`"]
pub enum STMW {
    #[doc = "Software trigger scan."]
    _0,
    #[doc = "Hardware trigger scan."]
    _1,
}
impl STMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STMW::_0 => false,
            STMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STMW<'a> {
    w: &'a mut W,
}
impl<'a> _STMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software trigger scan."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STMW::_0)
    }
    #[doc = "Hardware trigger scan."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STMW::_1)
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
#[doc = "Values that can be written to the field `STPE`"]
pub enum STPEW {
    #[doc = "TSI is disabled when MCU goes into low power mode."]
    _0,
    #[doc = "Allows TSI to continue running in all low power modes."]
    _1,
}
impl STPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STPEW::_0 => false,
            STPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STPEW<'a> {
    w: &'a mut W,
}
impl<'a> _STPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TSI is disabled when MCU goes into low power mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STPEW::_0)
    }
    #[doc = "Allows TSI to continue running in all low power modes."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STPEW::_1)
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
#[doc = "Values that can be written to the field `TSIIEN`"]
pub enum TSIIENW {
    #[doc = "TSI interrupt is disabled."]
    _0,
    #[doc = "TSI interrupt is enabled."]
    _1,
}
impl TSIIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSIIENW::_0 => false,
            TSIIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSIIENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSIIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TSI interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIIENW::_0)
    }
    #[doc = "TSI interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIIENW::_1)
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
#[doc = "Values that can be written to the field `TSIEN`"]
pub enum TSIENW {
    #[doc = "TSI module disabled."]
    _0,
    #[doc = "TSI module enabled."]
    _1,
}
impl TSIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSIENW::_0 => false,
            TSIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSIENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TSI module disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIENW::_0)
    }
    #[doc = "TSI module enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIENW::_1)
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
#[doc = "Values that can be written to the field `NSCN`"]
pub enum NSCNW {
    #[doc = "Once per electrode"]
    _00000,
    #[doc = "Twice per electrode"]
    _00001,
    #[doc = "3 times per electrode"]
    _00010,
    #[doc = "4 times per electrode"]
    _00011,
    #[doc = "5 times per electrode"]
    _00100,
    #[doc = "6 times per electrode"]
    _00101,
    #[doc = "7 times per electrode"]
    _00110,
    #[doc = "8 times per electrode"]
    _00111,
    #[doc = "9 times per electrode"]
    _01000,
    #[doc = "10 times per electrode"]
    _01001,
    #[doc = "11 times per electrode"]
    _01010,
    #[doc = "12 times per electrode"]
    _01011,
    #[doc = "13 times per electrode"]
    _01100,
    #[doc = "14 times per electrode"]
    _01101,
    #[doc = "15 times per electrode"]
    _01110,
    #[doc = "16 times per electrode"]
    _01111,
    #[doc = "17 times per electrode"]
    _10000,
    #[doc = "18 times per electrode"]
    _10001,
    #[doc = "19 times per electrode"]
    _10010,
    #[doc = "20 times per electrode"]
    _10011,
    #[doc = "21 times per electrode"]
    _10100,
    #[doc = "22 times per electrode"]
    _10101,
    #[doc = "23 times per electrode"]
    _10110,
    #[doc = "24 times per electrode"]
    _10111,
    #[doc = "25 times per electrode"]
    _11000,
    #[doc = "26 times per electrode"]
    _11001,
    #[doc = "27 times per electrode"]
    _11010,
    #[doc = "28 times per electrode"]
    _11011,
    #[doc = "29 times per electrode"]
    _11100,
    #[doc = "30 times per electrode"]
    _11101,
    #[doc = "31 times per electrode"]
    _11110,
    #[doc = "32 times per electrode"]
    _11111,
}
impl NSCNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NSCNW::_00000 => 0,
            NSCNW::_00001 => 1,
            NSCNW::_00010 => 2,
            NSCNW::_00011 => 3,
            NSCNW::_00100 => 4,
            NSCNW::_00101 => 5,
            NSCNW::_00110 => 6,
            NSCNW::_00111 => 7,
            NSCNW::_01000 => 8,
            NSCNW::_01001 => 9,
            NSCNW::_01010 => 10,
            NSCNW::_01011 => 11,
            NSCNW::_01100 => 12,
            NSCNW::_01101 => 13,
            NSCNW::_01110 => 14,
            NSCNW::_01111 => 15,
            NSCNW::_10000 => 16,
            NSCNW::_10001 => 17,
            NSCNW::_10010 => 18,
            NSCNW::_10011 => 19,
            NSCNW::_10100 => 20,
            NSCNW::_10101 => 21,
            NSCNW::_10110 => 22,
            NSCNW::_10111 => 23,
            NSCNW::_11000 => 24,
            NSCNW::_11001 => 25,
            NSCNW::_11010 => 26,
            NSCNW::_11011 => 27,
            NSCNW::_11100 => 28,
            NSCNW::_11101 => 29,
            NSCNW::_11110 => 30,
            NSCNW::_11111 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSCNW<'a> {
    w: &'a mut W,
}
impl<'a> _NSCNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSCNW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Once per electrode"]
    #[inline]
    pub fn _00000(self) -> &'a mut W {
        self.variant(NSCNW::_00000)
    }
    #[doc = "Twice per electrode"]
    #[inline]
    pub fn _00001(self) -> &'a mut W {
        self.variant(NSCNW::_00001)
    }
    #[doc = "3 times per electrode"]
    #[inline]
    pub fn _00010(self) -> &'a mut W {
        self.variant(NSCNW::_00010)
    }
    #[doc = "4 times per electrode"]
    #[inline]
    pub fn _00011(self) -> &'a mut W {
        self.variant(NSCNW::_00011)
    }
    #[doc = "5 times per electrode"]
    #[inline]
    pub fn _00100(self) -> &'a mut W {
        self.variant(NSCNW::_00100)
    }
    #[doc = "6 times per electrode"]
    #[inline]
    pub fn _00101(self) -> &'a mut W {
        self.variant(NSCNW::_00101)
    }
    #[doc = "7 times per electrode"]
    #[inline]
    pub fn _00110(self) -> &'a mut W {
        self.variant(NSCNW::_00110)
    }
    #[doc = "8 times per electrode"]
    #[inline]
    pub fn _00111(self) -> &'a mut W {
        self.variant(NSCNW::_00111)
    }
    #[doc = "9 times per electrode"]
    #[inline]
    pub fn _01000(self) -> &'a mut W {
        self.variant(NSCNW::_01000)
    }
    #[doc = "10 times per electrode"]
    #[inline]
    pub fn _01001(self) -> &'a mut W {
        self.variant(NSCNW::_01001)
    }
    #[doc = "11 times per electrode"]
    #[inline]
    pub fn _01010(self) -> &'a mut W {
        self.variant(NSCNW::_01010)
    }
    #[doc = "12 times per electrode"]
    #[inline]
    pub fn _01011(self) -> &'a mut W {
        self.variant(NSCNW::_01011)
    }
    #[doc = "13 times per electrode"]
    #[inline]
    pub fn _01100(self) -> &'a mut W {
        self.variant(NSCNW::_01100)
    }
    #[doc = "14 times per electrode"]
    #[inline]
    pub fn _01101(self) -> &'a mut W {
        self.variant(NSCNW::_01101)
    }
    #[doc = "15 times per electrode"]
    #[inline]
    pub fn _01110(self) -> &'a mut W {
        self.variant(NSCNW::_01110)
    }
    #[doc = "16 times per electrode"]
    #[inline]
    pub fn _01111(self) -> &'a mut W {
        self.variant(NSCNW::_01111)
    }
    #[doc = "17 times per electrode"]
    #[inline]
    pub fn _10000(self) -> &'a mut W {
        self.variant(NSCNW::_10000)
    }
    #[doc = "18 times per electrode"]
    #[inline]
    pub fn _10001(self) -> &'a mut W {
        self.variant(NSCNW::_10001)
    }
    #[doc = "19 times per electrode"]
    #[inline]
    pub fn _10010(self) -> &'a mut W {
        self.variant(NSCNW::_10010)
    }
    #[doc = "20 times per electrode"]
    #[inline]
    pub fn _10011(self) -> &'a mut W {
        self.variant(NSCNW::_10011)
    }
    #[doc = "21 times per electrode"]
    #[inline]
    pub fn _10100(self) -> &'a mut W {
        self.variant(NSCNW::_10100)
    }
    #[doc = "22 times per electrode"]
    #[inline]
    pub fn _10101(self) -> &'a mut W {
        self.variant(NSCNW::_10101)
    }
    #[doc = "23 times per electrode"]
    #[inline]
    pub fn _10110(self) -> &'a mut W {
        self.variant(NSCNW::_10110)
    }
    #[doc = "24 times per electrode"]
    #[inline]
    pub fn _10111(self) -> &'a mut W {
        self.variant(NSCNW::_10111)
    }
    #[doc = "25 times per electrode"]
    #[inline]
    pub fn _11000(self) -> &'a mut W {
        self.variant(NSCNW::_11000)
    }
    #[doc = "26 times per electrode"]
    #[inline]
    pub fn _11001(self) -> &'a mut W {
        self.variant(NSCNW::_11001)
    }
    #[doc = "27 times per electrode"]
    #[inline]
    pub fn _11010(self) -> &'a mut W {
        self.variant(NSCNW::_11010)
    }
    #[doc = "28 times per electrode"]
    #[inline]
    pub fn _11011(self) -> &'a mut W {
        self.variant(NSCNW::_11011)
    }
    #[doc = "29 times per electrode"]
    #[inline]
    pub fn _11100(self) -> &'a mut W {
        self.variant(NSCNW::_11100)
    }
    #[doc = "30 times per electrode"]
    #[inline]
    pub fn _11101(self) -> &'a mut W {
        self.variant(NSCNW::_11101)
    }
    #[doc = "31 times per electrode"]
    #[inline]
    pub fn _11110(self) -> &'a mut W {
        self.variant(NSCNW::_11110)
    }
    #[doc = "32 times per electrode"]
    #[inline]
    pub fn _11111(self) -> &'a mut W {
        self.variant(NSCNW::_11111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PS`"]
pub enum PSW {
    #[doc = "Electrode Oscillator Frequency divided by 1"]
    _000,
    #[doc = "Electrode Oscillator Frequency divided by 2"]
    _001,
    #[doc = "Electrode Oscillator Frequency divided by 4"]
    _010,
    #[doc = "Electrode Oscillator Frequency divided by 8"]
    _011,
    #[doc = "Electrode Oscillator Frequency divided by 16"]
    _100,
    #[doc = "Electrode Oscillator Frequency divided by 32"]
    _101,
    #[doc = "Electrode Oscillator Frequency divided by 64"]
    _110,
    #[doc = "Electrode Oscillator Frequency divided by 128"]
    _111,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSW::_000 => 0,
            PSW::_001 => 1,
            PSW::_010 => 2,
            PSW::_011 => 3,
            PSW::_100 => 4,
            PSW::_101 => 5,
            PSW::_110 => 6,
            PSW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Electrode Oscillator Frequency divided by 1"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PSW::_000)
    }
    #[doc = "Electrode Oscillator Frequency divided by 2"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PSW::_001)
    }
    #[doc = "Electrode Oscillator Frequency divided by 4"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PSW::_010)
    }
    #[doc = "Electrode Oscillator Frequency divided by 8"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PSW::_011)
    }
    #[doc = "Electrode Oscillator Frequency divided by 16"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PSW::_100)
    }
    #[doc = "Electrode Oscillator Frequency divided by 32"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PSW::_101)
    }
    #[doc = "Electrode Oscillator Frequency divided by 64"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PSW::_110)
    }
    #[doc = "Electrode Oscillator Frequency divided by 128"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PSW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTCHRG`"]
pub enum EXTCHRGW {
    #[doc = "500 nA."]
    _000,
    #[doc = "1 uA."]
    _001,
    #[doc = "2 uA."]
    _010,
    #[doc = "4 uA."]
    _011,
    #[doc = "8 uA."]
    _100,
    #[doc = "16 uA."]
    _101,
    #[doc = "32 uA."]
    _110,
    #[doc = "64 uA."]
    _111,
}
impl EXTCHRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTCHRGW::_000 => 0,
            EXTCHRGW::_001 => 1,
            EXTCHRGW::_010 => 2,
            EXTCHRGW::_011 => 3,
            EXTCHRGW::_100 => 4,
            EXTCHRGW::_101 => 5,
            EXTCHRGW::_110 => 6,
            EXTCHRGW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTCHRGW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTCHRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTCHRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "500 nA."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(EXTCHRGW::_000)
    }
    #[doc = "1 uA."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(EXTCHRGW::_001)
    }
    #[doc = "2 uA."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(EXTCHRGW::_010)
    }
    #[doc = "4 uA."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(EXTCHRGW::_011)
    }
    #[doc = "8 uA."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(EXTCHRGW::_100)
    }
    #[doc = "16 uA."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(EXTCHRGW::_101)
    }
    #[doc = "32 uA."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(EXTCHRGW::_110)
    }
    #[doc = "64 uA."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(EXTCHRGW::_111)
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
#[doc = "Values that can be written to the field `DVOLT`"]
pub enum DVOLTW {
    #[doc = "DV = 1.026 V; VP = 1.328 V; Vm = 0.302 V."]
    _00,
    #[doc = "DV = 0.592 V; VP = 1.111 V; Vm = 0.519 V."]
    _01,
    #[doc = "DV = 0.342 V; VP = 0.986 V; Vm = 0.644 V."]
    _10,
    #[doc = "DV = 0.197 V; VP = 0.914 V; Vm = 0.716 V."]
    _11,
}
impl DVOLTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DVOLTW::_00 => 0,
            DVOLTW::_01 => 1,
            DVOLTW::_10 => 2,
            DVOLTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DVOLTW<'a> {
    w: &'a mut W,
}
impl<'a> _DVOLTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DVOLTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DV = 1.026 V; VP = 1.328 V; Vm = 0.302 V."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DVOLTW::_00)
    }
    #[doc = "DV = 0.592 V; VP = 1.111 V; Vm = 0.519 V."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DVOLTW::_01)
    }
    #[doc = "DV = 0.342 V; VP = 0.986 V; Vm = 0.644 V."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DVOLTW::_10)
    }
    #[doc = "DV = 0.197 V; VP = 0.914 V; Vm = 0.716 V."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DVOLTW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REFCHRG`"]
pub enum REFCHRGW {
    #[doc = "500 nA."]
    _000,
    #[doc = "1 uA."]
    _001,
    #[doc = "2 uA."]
    _010,
    #[doc = "4 uA."]
    _011,
    #[doc = "8 uA."]
    _100,
    #[doc = "16 uA."]
    _101,
    #[doc = "32 uA."]
    _110,
    #[doc = "64 uA."]
    _111,
}
impl REFCHRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFCHRGW::_000 => 0,
            REFCHRGW::_001 => 1,
            REFCHRGW::_010 => 2,
            REFCHRGW::_011 => 3,
            REFCHRGW::_100 => 4,
            REFCHRGW::_101 => 5,
            REFCHRGW::_110 => 6,
            REFCHRGW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFCHRGW<'a> {
    w: &'a mut W,
}
impl<'a> _REFCHRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFCHRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "500 nA."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(REFCHRGW::_000)
    }
    #[doc = "1 uA."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(REFCHRGW::_001)
    }
    #[doc = "2 uA."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(REFCHRGW::_010)
    }
    #[doc = "4 uA."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(REFCHRGW::_011)
    }
    #[doc = "8 uA."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(REFCHRGW::_100)
    }
    #[doc = "16 uA."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(REFCHRGW::_101)
    }
    #[doc = "32 uA."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(REFCHRGW::_110)
    }
    #[doc = "64 uA."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(REFCHRGW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Set TSI in capacitive sensing(non-noise detection) mode."]
    _0000,
    #[doc = "Set TSI analog to work in single threshold noise detection mode and the frequency limitation circuit is disabled."]
    _0100,
    #[doc = "Set TSI analog to work in single threshold noise detection mode and the frequency limitation circuit is enabled to work in higher frequencies operations."]
    _1000,
    #[doc = "Set TSI analog to work in automatic noise detection mode."]
    _1100,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::_0000 => 0,
            MODEW::_0100 => 4,
            MODEW::_1000 => 8,
            MODEW::_1100 => 12,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set TSI in capacitive sensing(non-noise detection) mode."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(MODEW::_0000)
    }
    #[doc = "Set TSI analog to work in single threshold noise detection mode and the frequency limitation circuit is disabled."]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(MODEW::_0100)
    }
    #[doc = "Set TSI analog to work in single threshold noise detection mode and the frequency limitation circuit is enabled to work in higher frequencies operations."]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(MODEW::_1000)
    }
    #[doc = "Set TSI analog to work in automatic noise detection mode."]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(MODEW::_1100)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ESOR`"]
pub enum ESORW {
    #[doc = "Out-of-range interrupt is allowed."]
    _0,
    #[doc = "End-of-scan interrupt is allowed."]
    _1,
}
impl ESORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESORW::_0 => false,
            ESORW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESORW<'a> {
    w: &'a mut W,
}
impl<'a> _ESORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Out-of-range interrupt is allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESORW::_0)
    }
    #[doc = "End-of-scan interrupt is allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESORW::_1)
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
#[doc = r" Proxy"]
pub struct _OUTRGFW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTRGFW<'a> {
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
    #[doc = "Bit 0 - End-of-Scan DMA Transfer Request Enable Only"]
    #[inline]
    pub fn eosdmeo(&self) -> EOSDMEOR {
        EOSDMEOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CURSW"]
    #[inline]
    pub fn cursw(&self) -> CURSWR {
        CURSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - End of Scan Flag"]
    #[inline]
    pub fn eosf(&self) -> EOSFR {
        EOSFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Scan In Progress Status"]
    #[inline]
    pub fn scnip(&self) -> SCNIPR {
        SCNIPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Scan Trigger Mode"]
    #[inline]
    pub fn stm(&self) -> STMR {
        STMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - TSI STOP Enable"]
    #[inline]
    pub fn stpe(&self) -> STPER {
        STPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Touch Sensing Input Interrupt Enable"]
    #[inline]
    pub fn tsiien(&self) -> TSIIENR {
        TSIIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Touch Sensing Input Module Enable"]
    #[inline]
    pub fn tsien(&self) -> TSIENR {
        TSIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - NSCN"]
    #[inline]
    pub fn nscn(&self) -> NSCNR {
        NSCNR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 13:15 - PS"]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - EXTCHRG"]
    #[inline]
    pub fn extchrg(&self) -> EXTCHRGR {
        EXTCHRGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:20 - DVOLT"]
    #[inline]
    pub fn dvolt(&self) -> DVOLTR {
        DVOLTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:23 - REFCHRG"]
    #[inline]
    pub fn refchrg(&self) -> REFCHRGR {
        REFCHRGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - TSI analog modes setup and status bits."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - End-of-scan or Out-of-Range Interrupt Selection"]
    #[inline]
    pub fn esor(&self) -> ESORR {
        ESORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Out of Range Flag."]
    #[inline]
    pub fn outrgf(&self) -> OUTRGFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OUTRGFR { bits }
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
    #[doc = "Bit 0 - End-of-Scan DMA Transfer Request Enable Only"]
    #[inline]
    pub fn eosdmeo(&mut self) -> _EOSDMEOW {
        _EOSDMEOW { w: self }
    }
    #[doc = "Bit 1 - CURSW"]
    #[inline]
    pub fn cursw(&mut self) -> _CURSWW {
        _CURSWW { w: self }
    }
    #[doc = "Bit 2 - End of Scan Flag"]
    #[inline]
    pub fn eosf(&mut self) -> _EOSFW {
        _EOSFW { w: self }
    }
    #[doc = "Bit 4 - Scan Trigger Mode"]
    #[inline]
    pub fn stm(&mut self) -> _STMW {
        _STMW { w: self }
    }
    #[doc = "Bit 5 - TSI STOP Enable"]
    #[inline]
    pub fn stpe(&mut self) -> _STPEW {
        _STPEW { w: self }
    }
    #[doc = "Bit 6 - Touch Sensing Input Interrupt Enable"]
    #[inline]
    pub fn tsiien(&mut self) -> _TSIIENW {
        _TSIIENW { w: self }
    }
    #[doc = "Bit 7 - Touch Sensing Input Module Enable"]
    #[inline]
    pub fn tsien(&mut self) -> _TSIENW {
        _TSIENW { w: self }
    }
    #[doc = "Bits 8:12 - NSCN"]
    #[inline]
    pub fn nscn(&mut self) -> _NSCNW {
        _NSCNW { w: self }
    }
    #[doc = "Bits 13:15 - PS"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bits 16:18 - EXTCHRG"]
    #[inline]
    pub fn extchrg(&mut self) -> _EXTCHRGW {
        _EXTCHRGW { w: self }
    }
    #[doc = "Bits 19:20 - DVOLT"]
    #[inline]
    pub fn dvolt(&mut self) -> _DVOLTW {
        _DVOLTW { w: self }
    }
    #[doc = "Bits 21:23 - REFCHRG"]
    #[inline]
    pub fn refchrg(&mut self) -> _REFCHRGW {
        _REFCHRGW { w: self }
    }
    #[doc = "Bits 24:27 - TSI analog modes setup and status bits."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 28 - End-of-scan or Out-of-Range Interrupt Selection"]
    #[inline]
    pub fn esor(&mut self) -> _ESORW {
        _ESORW { w: self }
    }
    #[doc = "Bit 31 - Out of Range Flag."]
    #[inline]
    pub fn outrgf(&mut self) -> _OUTRGFW {
        _OUTRGFW { w: self }
    }
}
