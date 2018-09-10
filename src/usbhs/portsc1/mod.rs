#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PORTSC1 {
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
#[doc = "Possible values of the field `CCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCSR {
    #[doc = "No device present (host mode) or attached (device mode)"]
    _0,
    #[doc = "Device is present (host mode) or attached (device mode)"]
    _1,
}
impl CCSR {
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
            CCSR::_0 => false,
            CCSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCSR {
        match value {
            false => CCSR::_0,
            true => CCSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCSR::_1
    }
}
#[doc = "Possible values of the field `CSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSCR {
    #[doc = "No change"]
    _0,
    #[doc = "Connect status has changed"]
    _1,
}
impl CSCR {
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
            CSCR::_0 => false,
            CSCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSCR {
        match value {
            false => CSCR::_0,
            true => CSCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CSCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CSCR::_1
    }
}
#[doc = r" Value of the field"]
pub struct PER {
    bits: bool,
}
impl PER {
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
#[doc = "Possible values of the field `PEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECR {
    #[doc = "No change"]
    _0,
    #[doc = "Port disabled"]
    _1,
}
impl PECR {
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
            PECR::_0 => false,
            PECR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PECR {
        match value {
            false => PECR::_0,
            true => PECR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PECR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PECR::_1
    }
}
#[doc = "Possible values of the field `OCA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCAR {
    #[doc = "Port not in over-current condition"]
    _0,
    #[doc = "Port currently in over-current condition"]
    _1,
}
impl OCAR {
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
            OCAR::_0 => false,
            OCAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCAR {
        match value {
            false => OCAR::_0,
            true => OCAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OCAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OCAR::_1
    }
}
#[doc = "Possible values of the field `OCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCCR {
    #[doc = "No over-current"]
    _0,
    #[doc = "Over-current detect"]
    _1,
}
impl OCCR {
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
            OCCR::_0 => false,
            OCCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCCR {
        match value {
            false => OCCR::_0,
            true => OCCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OCCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OCCR::_1
    }
}
#[doc = "Possible values of the field `FPR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPRR {
    #[doc = "No resume (K-state) detected/driven on port"]
    _0,
    #[doc = "Resume detected/driven on port"]
    _1,
}
impl FPRR {
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
            FPRR::_0 => false,
            FPRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FPRR {
        match value {
            false => FPRR::_0,
            true => FPRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FPRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FPRR::_1
    }
}
#[doc = "Possible values of the field `SUSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPR {
    #[doc = "Port not in suspend state"]
    _0,
    #[doc = "Port in suspend state"]
    _1,
}
impl SUSPR {
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
            SUSPR::_0 => false,
            SUSPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUSPR {
        match value {
            false => SUSPR::_0,
            true => SUSPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SUSPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SUSPR::_1
    }
}
#[doc = "Possible values of the field `PR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRR {
    #[doc = "Port is not in reset"]
    _0,
    #[doc = "Port is in reset"]
    _1,
}
impl PRR {
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
            PRR::_0 => false,
            PRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRR {
        match value {
            false => PRR::_0,
            true => PRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PRR::_1
    }
}
#[doc = "Possible values of the field `HSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSPR {
    #[doc = "FS or LS"]
    _0,
    #[doc = "HS"]
    _1,
}
impl HSPR {
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
            HSPR::_0 => false,
            HSPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSPR {
        match value {
            false => HSPR::_0,
            true => HSPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HSPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HSPR::_1
    }
}
#[doc = "Possible values of the field `LS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSR {
    #[doc = "SE0"]
    _00,
    #[doc = "J-state"]
    _01,
    #[doc = "K-state"]
    _10,
    #[doc = "Undefined"]
    _11,
}
impl LSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LSR::_00 => 0,
            LSR::_01 => 1,
            LSR::_10 => 2,
            LSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LSR {
        match value {
            0 => LSR::_00,
            1 => LSR::_01,
            2 => LSR::_10,
            3 => LSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == LSR::_11
    }
}
#[doc = r" Value of the field"]
pub struct PPR {
    bits: bool,
}
impl PPR {
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
#[doc = r" Value of the field"]
pub struct POR {
    bits: bool,
}
impl POR {
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
#[doc = r" Value of the field"]
pub struct PICR {
    bits: u8,
}
impl PICR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTCR {
    #[doc = "Not enabled"]
    _0000,
    #[doc = "J_STATE"]
    _0001,
    #[doc = "K_STATE"]
    _0010,
    #[doc = "SE0_NAK"]
    _0011,
    #[doc = "Packet"]
    _0100,
    #[doc = "FORCE_ENABLE_HS"]
    _0101,
    #[doc = "FORCE_ENABLE_FS"]
    _0110,
    #[doc = "FORCE_ENABLE_LS"]
    _0111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PTCR::_0000 => 0,
            PTCR::_0001 => 1,
            PTCR::_0010 => 2,
            PTCR::_0011 => 3,
            PTCR::_0100 => 4,
            PTCR::_0101 => 5,
            PTCR::_0110 => 6,
            PTCR::_0111 => 7,
            PTCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PTCR {
        match value {
            0 => PTCR::_0000,
            1 => PTCR::_0001,
            2 => PTCR::_0010,
            3 => PTCR::_0011,
            4 => PTCR::_0100,
            5 => PTCR::_0101,
            6 => PTCR::_0110,
            7 => PTCR::_0111,
            i => PTCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == PTCR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == PTCR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == PTCR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == PTCR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == PTCR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == PTCR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == PTCR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == PTCR::_0111
    }
}
#[doc = r" Value of the field"]
pub struct WKCNR {
    bits: bool,
}
impl WKCNR {
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
#[doc = r" Value of the field"]
pub struct WKDSR {
    bits: bool,
}
impl WKDSR {
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
#[doc = r" Value of the field"]
pub struct WKOCR {
    bits: bool,
}
impl WKOCR {
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
#[doc = r" Value of the field"]
pub struct PHCDR {
    bits: bool,
}
impl PHCDR {
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
#[doc = "Possible values of the field `PFSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFSCR {
    #[doc = "Allow the port to identify itself as high speed"]
    _0,
    #[doc = "Force the port to only connect at full speed"]
    _1,
}
impl PFSCR {
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
            PFSCR::_0 => false,
            PFSCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFSCR {
        match value {
            false => PFSCR::_0,
            true => PFSCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PFSCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PFSCR::_1
    }
}
#[doc = r" Value of the field"]
pub struct PTS2R {
    bits: bool,
}
impl PTS2R {
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
#[doc = "Possible values of the field `PSPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSPDR {
    #[doc = "Full speed"]
    _00,
    #[doc = "Low speed"]
    _01,
    #[doc = "High speed"]
    _10,
    #[doc = "Undefined"]
    _11,
}
impl PSPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSPDR::_00 => 0,
            PSPDR::_01 => 1,
            PSPDR::_10 => 2,
            PSPDR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSPDR {
        match value {
            0 => PSPDR::_00,
            1 => PSPDR::_01,
            2 => PSPDR::_10,
            3 => PSPDR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PSPDR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PSPDR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PSPDR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PSPDR::_11
    }
}
#[doc = "Possible values of the field `PTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTSR {
    #[doc = "Use UTMI transceiver interface."]
    _000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PTSR::_000 => 0,
            PTSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PTSR {
        match value {
            0 => PTSR::_000,
            i => PTSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PTSR::_000
    }
}
#[doc = "Values that can be written to the field `CSC`"]
pub enum CSCW {
    #[doc = "No change"]
    _0,
    #[doc = "Connect status has changed"]
    _1,
}
impl CSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSCW::_0 => false,
            CSCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CSCW::_0)
    }
    #[doc = "Connect status has changed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSCW::_1)
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
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
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
#[doc = "Values that can be written to the field `PEC`"]
pub enum PECW {
    #[doc = "No change"]
    _0,
    #[doc = "Port disabled"]
    _1,
}
impl PECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PECW::_0 => false,
            PECW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PECW<'a> {
    w: &'a mut W,
}
impl<'a> _PECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PECW::_0)
    }
    #[doc = "Port disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PECW::_1)
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
#[doc = "Values that can be written to the field `OCC`"]
pub enum OCCW {
    #[doc = "No over-current"]
    _0,
    #[doc = "Over-current detect"]
    _1,
}
impl OCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OCCW::_0 => false,
            OCCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCCW<'a> {
    w: &'a mut W,
}
impl<'a> _OCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No over-current"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OCCW::_0)
    }
    #[doc = "Over-current detect"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OCCW::_1)
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
#[doc = "Values that can be written to the field `FPR`"]
pub enum FPRW {
    #[doc = "No resume (K-state) detected/driven on port"]
    _0,
    #[doc = "Resume detected/driven on port"]
    _1,
}
impl FPRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FPRW::_0 => false,
            FPRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPRW<'a> {
    w: &'a mut W,
}
impl<'a> _FPRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No resume (K-state) detected/driven on port"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPRW::_0)
    }
    #[doc = "Resume detected/driven on port"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPRW::_1)
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
#[doc = "Values that can be written to the field `SUSP`"]
pub enum SUSPW {
    #[doc = "Port not in suspend state"]
    _0,
    #[doc = "Port in suspend state"]
    _1,
}
impl SUSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUSPW::_0 => false,
            SUSPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port not in suspend state"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUSPW::_0)
    }
    #[doc = "Port in suspend state"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUSPW::_1)
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
#[doc = "Values that can be written to the field `PR`"]
pub enum PRW {
    #[doc = "Port is not in reset"]
    _0,
    #[doc = "Port is in reset"]
    _1,
}
impl PRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRW::_0 => false,
            PRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Port is not in reset"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRW::_0)
    }
    #[doc = "Port is in reset"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRW::_1)
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
#[doc = r" Proxy"]
pub struct _PPW<'a> {
    w: &'a mut W,
}
impl<'a> _PPW<'a> {
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
#[doc = r" Proxy"]
pub struct _POW<'a> {
    w: &'a mut W,
}
impl<'a> _POW<'a> {
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
#[doc = r" Proxy"]
pub struct _PICW<'a> {
    w: &'a mut W,
}
impl<'a> _PICW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PTC`"]
pub enum PTCW {
    #[doc = "Not enabled"]
    _0000,
    #[doc = "J_STATE"]
    _0001,
    #[doc = "K_STATE"]
    _0010,
    #[doc = "SE0_NAK"]
    _0011,
    #[doc = "Packet"]
    _0100,
    #[doc = "FORCE_ENABLE_HS"]
    _0101,
    #[doc = "FORCE_ENABLE_FS"]
    _0110,
    #[doc = "FORCE_ENABLE_LS"]
    _0111,
}
impl PTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PTCW::_0000 => 0,
            PTCW::_0001 => 1,
            PTCW::_0010 => 2,
            PTCW::_0011 => 3,
            PTCW::_0100 => 4,
            PTCW::_0101 => 5,
            PTCW::_0110 => 6,
            PTCW::_0111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTCW<'a> {
    w: &'a mut W,
}
impl<'a> _PTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Not enabled"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PTCW::_0000)
    }
    #[doc = "J_STATE"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(PTCW::_0001)
    }
    #[doc = "K_STATE"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(PTCW::_0010)
    }
    #[doc = "SE0_NAK"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(PTCW::_0011)
    }
    #[doc = "Packet"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(PTCW::_0100)
    }
    #[doc = "FORCE_ENABLE_HS"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(PTCW::_0101)
    }
    #[doc = "FORCE_ENABLE_FS"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(PTCW::_0110)
    }
    #[doc = "FORCE_ENABLE_LS"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(PTCW::_0111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WKCNW<'a> {
    w: &'a mut W,
}
impl<'a> _WKCNW<'a> {
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
#[doc = r" Proxy"]
pub struct _WKDSW<'a> {
    w: &'a mut W,
}
impl<'a> _WKDSW<'a> {
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
#[doc = r" Proxy"]
pub struct _WKOCW<'a> {
    w: &'a mut W,
}
impl<'a> _WKOCW<'a> {
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
#[doc = r" Proxy"]
pub struct _PHCDW<'a> {
    w: &'a mut W,
}
impl<'a> _PHCDW<'a> {
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
#[doc = "Values that can be written to the field `PFSC`"]
pub enum PFSCW {
    #[doc = "Allow the port to identify itself as high speed"]
    _0,
    #[doc = "Force the port to only connect at full speed"]
    _1,
}
impl PFSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PFSCW::_0 => false,
            PFSCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PFSCW<'a> {
    w: &'a mut W,
}
impl<'a> _PFSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PFSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow the port to identify itself as high speed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PFSCW::_0)
    }
    #[doc = "Force the port to only connect at full speed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PFSCW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Current Connect Status"]
    #[inline]
    pub fn ccs(&self) -> CCSR {
        CCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Connect Change Status"]
    #[inline]
    pub fn csc(&self) -> CSCR {
        CSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port Enabled/disabled"]
    #[inline]
    pub fn pe(&self) -> PER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PER { bits }
    }
    #[doc = "Bit 3 - Port Enable/disable Change"]
    #[inline]
    pub fn pec(&self) -> PECR {
        PECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Over-current active"]
    #[inline]
    pub fn oca(&self) -> OCAR {
        OCAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Over-Current Change"]
    #[inline]
    pub fn occ(&self) -> OCCR {
        OCCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Force Port Resume"]
    #[inline]
    pub fn fpr(&self) -> FPRR {
        FPRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Suspend"]
    #[inline]
    pub fn susp(&self) -> SUSPR {
        SUSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline]
    pub fn pr(&self) -> PRR {
        PRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - High Speed Port."]
    #[inline]
    pub fn hsp(&self) -> HSPR {
        HSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:11 - Line Status"]
    #[inline]
    pub fn ls(&self) -> LSR {
        LSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline]
    pub fn pp(&self) -> PPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PPR { bits }
    }
    #[doc = "Bit 13 - Port Owner"]
    #[inline]
    pub fn po(&self) -> POR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POR { bits }
    }
    #[doc = "Bits 14:15 - Port Indicator Control"]
    #[inline]
    pub fn pic(&self) -> PICR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PICR { bits }
    }
    #[doc = "Bits 16:19 - Port Test Control"]
    #[inline]
    pub fn ptc(&self) -> PTCR {
        PTCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Wake on Connect enable"]
    #[inline]
    pub fn wkcn(&self) -> WKCNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WKCNR { bits }
    }
    #[doc = "Bit 21 - Wake on Disconnect enable"]
    #[inline]
    pub fn wkds(&self) -> WKDSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WKDSR { bits }
    }
    #[doc = "Bit 22 - Wake on Over-Current enable"]
    #[inline]
    pub fn wkoc(&self) -> WKOCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WKOCR { bits }
    }
    #[doc = "Bit 23 - PHY low power suspend"]
    #[inline]
    pub fn phcd(&self) -> PHCDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PHCDR { bits }
    }
    #[doc = "Bit 24 - Port force Full-Speed Connect"]
    #[inline]
    pub fn pfsc(&self) -> PFSCR {
        PFSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Port Transceiver Select \\[2\\]"]
    #[inline]
    pub fn pts2(&self) -> PTS2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PTS2R { bits }
    }
    #[doc = "Bits 26:27 - Port Speed"]
    #[inline]
    pub fn pspd(&self) -> PSPDR {
        PSPDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Port Transceiver Select \\[1:0\\]"]
    #[inline]
    pub fn pts(&self) -> PTSR {
        PTSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 1 - Connect Change Status"]
    #[inline]
    pub fn csc(&mut self) -> _CSCW {
        _CSCW { w: self }
    }
    #[doc = "Bit 2 - Port Enabled/disabled"]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bit 3 - Port Enable/disable Change"]
    #[inline]
    pub fn pec(&mut self) -> _PECW {
        _PECW { w: self }
    }
    #[doc = "Bit 5 - Over-Current Change"]
    #[inline]
    pub fn occ(&mut self) -> _OCCW {
        _OCCW { w: self }
    }
    #[doc = "Bit 6 - Force Port Resume"]
    #[inline]
    pub fn fpr(&mut self) -> _FPRW {
        _FPRW { w: self }
    }
    #[doc = "Bit 7 - Suspend"]
    #[inline]
    pub fn susp(&mut self) -> _SUSPW {
        _SUSPW { w: self }
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline]
    pub fn pr(&mut self) -> _PRW {
        _PRW { w: self }
    }
    #[doc = "Bit 12 - Port Power"]
    #[inline]
    pub fn pp(&mut self) -> _PPW {
        _PPW { w: self }
    }
    #[doc = "Bit 13 - Port Owner"]
    #[inline]
    pub fn po(&mut self) -> _POW {
        _POW { w: self }
    }
    #[doc = "Bits 14:15 - Port Indicator Control"]
    #[inline]
    pub fn pic(&mut self) -> _PICW {
        _PICW { w: self }
    }
    #[doc = "Bits 16:19 - Port Test Control"]
    #[inline]
    pub fn ptc(&mut self) -> _PTCW {
        _PTCW { w: self }
    }
    #[doc = "Bit 20 - Wake on Connect enable"]
    #[inline]
    pub fn wkcn(&mut self) -> _WKCNW {
        _WKCNW { w: self }
    }
    #[doc = "Bit 21 - Wake on Disconnect enable"]
    #[inline]
    pub fn wkds(&mut self) -> _WKDSW {
        _WKDSW { w: self }
    }
    #[doc = "Bit 22 - Wake on Over-Current enable"]
    #[inline]
    pub fn wkoc(&mut self) -> _WKOCW {
        _WKOCW { w: self }
    }
    #[doc = "Bit 23 - PHY low power suspend"]
    #[inline]
    pub fn phcd(&mut self) -> _PHCDW {
        _PHCDW { w: self }
    }
    #[doc = "Bit 24 - Port force Full-Speed Connect"]
    #[inline]
    pub fn pfsc(&mut self) -> _PFSCW {
        _PFSCW { w: self }
    }
}
