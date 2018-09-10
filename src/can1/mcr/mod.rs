#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = r" Value of the field"]
pub struct MAXMBR {
    bits: u8,
}
impl MAXMBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `IDAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDAMR {
    #[doc = "Format A: One full ID (standard and extended) per ID Filter Table element."]
    _00,
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    _01,
    #[doc = "Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    _10,
    #[doc = "Format D: All frames rejected."]
    _11,
}
impl IDAMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDAMR::_00 => 0,
            IDAMR::_01 => 1,
            IDAMR::_10 => 2,
            IDAMR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDAMR {
        match value {
            0 => IDAMR::_00,
            1 => IDAMR::_01,
            2 => IDAMR::_10,
            3 => IDAMR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == IDAMR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == IDAMR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == IDAMR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == IDAMR::_11
    }
}
#[doc = "Possible values of the field `AEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AENR {
    #[doc = "Abort disabled."]
    _0,
    #[doc = "Abort enabled."]
    _1,
}
impl AENR {
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
            AENR::_0 => false,
            AENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AENR {
        match value {
            false => AENR::_0,
            true => AENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AENR::_1
    }
}
#[doc = "Possible values of the field `LPRIOEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPRIOENR {
    #[doc = "Local Priority disabled."]
    _0,
    #[doc = "Local Priority enabled."]
    _1,
}
impl LPRIOENR {
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
            LPRIOENR::_0 => false,
            LPRIOENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPRIOENR {
        match value {
            false => LPRIOENR::_0,
            true => LPRIOENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPRIOENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPRIOENR::_1
    }
}
#[doc = "Possible values of the field `IRMQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRMQR {
    #[doc = "Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    _0,
    #[doc = "Individual Rx masking and queue feature are enabled."]
    _1,
}
impl IRMQR {
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
            IRMQR::_0 => false,
            IRMQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRMQR {
        match value {
            false => IRMQR::_0,
            true => IRMQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IRMQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IRMQR::_1
    }
}
#[doc = "Possible values of the field `SRXDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRXDISR {
    #[doc = "Self reception enabled."]
    _0,
    #[doc = "Self reception disabled."]
    _1,
}
impl SRXDISR {
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
            SRXDISR::_0 => false,
            SRXDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRXDISR {
        match value {
            false => SRXDISR::_0,
            true => SRXDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SRXDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SRXDISR::_1
    }
}
#[doc = "Possible values of the field `WAKSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKSRCR {
    #[doc = "FlexCAN uses the unfiltered Rx input to detect recessive to dominant edges on the CAN bus."]
    _0,
    #[doc = "FlexCAN uses the filtered Rx input to detect recessive to dominant edges on the CAN bus."]
    _1,
}
impl WAKSRCR {
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
            WAKSRCR::_0 => false,
            WAKSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKSRCR {
        match value {
            false => WAKSRCR::_0,
            true => WAKSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WAKSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WAKSRCR::_1
    }
}
#[doc = "Possible values of the field `LPMACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMACKR {
    #[doc = "FlexCAN is not in a low-power mode."]
    _0,
    #[doc = "FlexCAN is in a low-power mode."]
    _1,
}
impl LPMACKR {
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
            LPMACKR::_0 => false,
            LPMACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPMACKR {
        match value {
            false => LPMACKR::_0,
            true => LPMACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPMACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPMACKR::_1
    }
}
#[doc = "Possible values of the field `WRNEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRNENR {
    #[doc = "TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    _0,
    #[doc = "TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    _1,
}
impl WRNENR {
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
            WRNENR::_0 => false,
            WRNENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRNENR {
        match value {
            false => WRNENR::_0,
            true => WRNENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WRNENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WRNENR::_1
    }
}
#[doc = "Possible values of the field `SLFWAK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLFWAKR {
    #[doc = "FlexCAN Self Wake Up feature is disabled."]
    _0,
    #[doc = "FlexCAN Self Wake Up feature is enabled."]
    _1,
}
impl SLFWAKR {
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
            SLFWAKR::_0 => false,
            SLFWAKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLFWAKR {
        match value {
            false => SLFWAKR::_0,
            true => SLFWAKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLFWAKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLFWAKR::_1
    }
}
#[doc = "Possible values of the field `SUPV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUPVR {
    #[doc = "FlexCAN is in User mode. Affected registers allow both Supervisor and Unrestricted accesses ."]
    _0,
    #[doc = "FlexCAN is in Supervisor mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location ."]
    _1,
}
impl SUPVR {
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
            SUPVR::_0 => false,
            SUPVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUPVR {
        match value {
            false => SUPVR::_0,
            true => SUPVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SUPVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SUPVR::_1
    }
}
#[doc = "Possible values of the field `FRZACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZACKR {
    #[doc = "FlexCAN not in Freeze mode, prescaler running."]
    _0,
    #[doc = "FlexCAN in Freeze mode, prescaler stopped."]
    _1,
}
impl FRZACKR {
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
            FRZACKR::_0 => false,
            FRZACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRZACKR {
        match value {
            false => FRZACKR::_0,
            true => FRZACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FRZACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FRZACKR::_1
    }
}
#[doc = "Possible values of the field `SOFTRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTRSTR {
    #[doc = "No reset request."]
    _0,
    #[doc = "Resets the registers affected by soft reset."]
    _1,
}
impl SOFTRSTR {
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
            SOFTRSTR::_0 => false,
            SOFTRSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFTRSTR {
        match value {
            false => SOFTRSTR::_0,
            true => SOFTRSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SOFTRSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SOFTRSTR::_1
    }
}
#[doc = "Possible values of the field `WAKMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKMSKR {
    #[doc = "Wake Up Interrupt is disabled."]
    _0,
    #[doc = "Wake Up Interrupt is enabled."]
    _1,
}
impl WAKMSKR {
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
            WAKMSKR::_0 => false,
            WAKMSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKMSKR {
        match value {
            false => WAKMSKR::_0,
            true => WAKMSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WAKMSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WAKMSKR::_1
    }
}
#[doc = "Possible values of the field `NOTRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRDYR {
    #[doc = "FlexCAN module is either in Normal mode, Listen-Only mode or Loop-Back mode."]
    _0,
    #[doc = "FlexCAN module is either in Disable mode , Stop mode or Freeze mode."]
    _1,
}
impl NOTRDYR {
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
            NOTRDYR::_0 => false,
            NOTRDYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOTRDYR {
        match value {
            false => NOTRDYR::_0,
            true => NOTRDYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NOTRDYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NOTRDYR::_1
    }
}
#[doc = "Possible values of the field `HALT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALTR {
    #[doc = "No Freeze mode request."]
    _0,
    #[doc = "Enters Freeze mode if the FRZ bit is asserted."]
    _1,
}
impl HALTR {
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
            HALTR::_0 => false,
            HALTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HALTR {
        match value {
            false => HALTR::_0,
            true => HALTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HALTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HALTR::_1
    }
}
#[doc = "Possible values of the field `RFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFENR {
    #[doc = "Rx FIFO not enabled."]
    _0,
    #[doc = "Rx FIFO enabled."]
    _1,
}
impl RFENR {
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
            RFENR::_0 => false,
            RFENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFENR {
        match value {
            false => RFENR::_0,
            true => RFENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RFENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RFENR::_1
    }
}
#[doc = "Possible values of the field `FRZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZR {
    #[doc = "Not enabled to enter Freeze mode."]
    _0,
    #[doc = "Enabled to enter Freeze mode."]
    _1,
}
impl FRZR {
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
            FRZR::_0 => false,
            FRZR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRZR {
        match value {
            false => FRZR::_0,
            true => FRZR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FRZR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FRZR::_1
    }
}
#[doc = "Possible values of the field `MDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDISR {
    #[doc = "Enable the FlexCAN module."]
    _0,
    #[doc = "Disable the FlexCAN module."]
    _1,
}
impl MDISR {
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
            MDISR::_0 => false,
            MDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDISR {
        match value {
            false => MDISR::_0,
            true => MDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MDISR::_1
    }
}
#[doc = r" Proxy"]
pub struct _MAXMBW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXMBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDAM`"]
pub enum IDAMW {
    #[doc = "Format A: One full ID (standard and extended) per ID Filter Table element."]
    _00,
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    _01,
    #[doc = "Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    _10,
    #[doc = "Format D: All frames rejected."]
    _11,
}
impl IDAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDAMW::_00 => 0,
            IDAMW::_01 => 1,
            IDAMW::_10 => 2,
            IDAMW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDAMW<'a> {
    w: &'a mut W,
}
impl<'a> _IDAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDAMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Format A: One full ID (standard and extended) per ID Filter Table element."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(IDAMW::_00)
    }
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(IDAMW::_01)
    }
    #[doc = "Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(IDAMW::_10)
    }
    #[doc = "Format D: All frames rejected."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(IDAMW::_11)
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
#[doc = "Values that can be written to the field `AEN`"]
pub enum AENW {
    #[doc = "Abort disabled."]
    _0,
    #[doc = "Abort enabled."]
    _1,
}
impl AENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AENW::_0 => false,
            AENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AENW<'a> {
    w: &'a mut W,
}
impl<'a> _AENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Abort disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AENW::_0)
    }
    #[doc = "Abort enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AENW::_1)
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
#[doc = "Values that can be written to the field `LPRIOEN`"]
pub enum LPRIOENW {
    #[doc = "Local Priority disabled."]
    _0,
    #[doc = "Local Priority enabled."]
    _1,
}
impl LPRIOENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPRIOENW::_0 => false,
            LPRIOENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPRIOENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPRIOENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPRIOENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Local Priority disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPRIOENW::_0)
    }
    #[doc = "Local Priority enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPRIOENW::_1)
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
#[doc = "Values that can be written to the field `IRMQ`"]
pub enum IRMQW {
    #[doc = "Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    _0,
    #[doc = "Individual Rx masking and queue feature are enabled."]
    _1,
}
impl IRMQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRMQW::_0 => false,
            IRMQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRMQW<'a> {
    w: &'a mut W,
}
impl<'a> _IRMQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRMQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRMQW::_0)
    }
    #[doc = "Individual Rx masking and queue feature are enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRMQW::_1)
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
#[doc = "Values that can be written to the field `SRXDIS`"]
pub enum SRXDISW {
    #[doc = "Self reception enabled."]
    _0,
    #[doc = "Self reception disabled."]
    _1,
}
impl SRXDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRXDISW::_0 => false,
            SRXDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRXDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SRXDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRXDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Self reception enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRXDISW::_0)
    }
    #[doc = "Self reception disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRXDISW::_1)
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
#[doc = "Values that can be written to the field `WAKSRC`"]
pub enum WAKSRCW {
    #[doc = "FlexCAN uses the unfiltered Rx input to detect recessive to dominant edges on the CAN bus."]
    _0,
    #[doc = "FlexCAN uses the filtered Rx input to detect recessive to dominant edges on the CAN bus."]
    _1,
}
impl WAKSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKSRCW::_0 => false,
            WAKSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexCAN uses the unfiltered Rx input to detect recessive to dominant edges on the CAN bus."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKSRCW::_0)
    }
    #[doc = "FlexCAN uses the filtered Rx input to detect recessive to dominant edges on the CAN bus."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKSRCW::_1)
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
#[doc = "Values that can be written to the field `WRNEN`"]
pub enum WRNENW {
    #[doc = "TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    _0,
    #[doc = "TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    _1,
}
impl WRNENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WRNENW::_0 => false,
            WRNENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRNENW<'a> {
    w: &'a mut W,
}
impl<'a> _WRNENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRNENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WRNENW::_0)
    }
    #[doc = "TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WRNENW::_1)
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
#[doc = "Values that can be written to the field `SLFWAK`"]
pub enum SLFWAKW {
    #[doc = "FlexCAN Self Wake Up feature is disabled."]
    _0,
    #[doc = "FlexCAN Self Wake Up feature is enabled."]
    _1,
}
impl SLFWAKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLFWAKW::_0 => false,
            SLFWAKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLFWAKW<'a> {
    w: &'a mut W,
}
impl<'a> _SLFWAKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLFWAKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexCAN Self Wake Up feature is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLFWAKW::_0)
    }
    #[doc = "FlexCAN Self Wake Up feature is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLFWAKW::_1)
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
#[doc = "Values that can be written to the field `SUPV`"]
pub enum SUPVW {
    #[doc = "FlexCAN is in User mode. Affected registers allow both Supervisor and Unrestricted accesses ."]
    _0,
    #[doc = "FlexCAN is in Supervisor mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location ."]
    _1,
}
impl SUPVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUPVW::_0 => false,
            SUPVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUPVW<'a> {
    w: &'a mut W,
}
impl<'a> _SUPVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUPVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexCAN is in User mode. Affected registers allow both Supervisor and Unrestricted accesses ."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUPVW::_0)
    }
    #[doc = "FlexCAN is in Supervisor mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location ."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUPVW::_1)
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
#[doc = "Values that can be written to the field `SOFTRST`"]
pub enum SOFTRSTW {
    #[doc = "No reset request."]
    _0,
    #[doc = "Resets the registers affected by soft reset."]
    _1,
}
impl SOFTRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOFTRSTW::_0 => false,
            SOFTRSTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOFTRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFTRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOFTRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No reset request."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFTRSTW::_0)
    }
    #[doc = "Resets the registers affected by soft reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFTRSTW::_1)
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
#[doc = "Values that can be written to the field `WAKMSK`"]
pub enum WAKMSKW {
    #[doc = "Wake Up Interrupt is disabled."]
    _0,
    #[doc = "Wake Up Interrupt is enabled."]
    _1,
}
impl WAKMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKMSKW::_0 => false,
            WAKMSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake Up Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKMSKW::_0)
    }
    #[doc = "Wake Up Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKMSKW::_1)
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
#[doc = "Values that can be written to the field `HALT`"]
pub enum HALTW {
    #[doc = "No Freeze mode request."]
    _0,
    #[doc = "Enters Freeze mode if the FRZ bit is asserted."]
    _1,
}
impl HALTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HALTW::_0 => false,
            HALTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HALTW<'a> {
    w: &'a mut W,
}
impl<'a> _HALTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HALTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Freeze mode request."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALTW::_0)
    }
    #[doc = "Enters Freeze mode if the FRZ bit is asserted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HALTW::_1)
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
#[doc = "Values that can be written to the field `RFEN`"]
pub enum RFENW {
    #[doc = "Rx FIFO not enabled."]
    _0,
    #[doc = "Rx FIFO enabled."]
    _1,
}
impl RFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFENW::_0 => false,
            RFENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFENW<'a> {
    w: &'a mut W,
}
impl<'a> _RFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx FIFO not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFENW::_0)
    }
    #[doc = "Rx FIFO enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFENW::_1)
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
#[doc = "Values that can be written to the field `FRZ`"]
pub enum FRZW {
    #[doc = "Not enabled to enter Freeze mode."]
    _0,
    #[doc = "Enabled to enter Freeze mode."]
    _1,
}
impl FRZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRZW::_0 => false,
            FRZW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRZW<'a> {
    w: &'a mut W,
}
impl<'a> _FRZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not enabled to enter Freeze mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRZW::_0)
    }
    #[doc = "Enabled to enter Freeze mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRZW::_1)
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
#[doc = "Values that can be written to the field `MDIS`"]
pub enum MDISW {
    #[doc = "Enable the FlexCAN module."]
    _0,
    #[doc = "Disable the FlexCAN module."]
    _1,
}
impl MDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDISW::_0 => false,
            MDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable the FlexCAN module."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDISW::_0)
    }
    #[doc = "Disable the FlexCAN module."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDISW::_1)
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
    #[doc = "Bits 0:6 - Number Of The Last Message Buffer"]
    #[inline]
    pub fn maxmb(&self) -> MAXMBR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXMBR { bits }
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline]
    pub fn idam(&self) -> IDAMR {
        IDAMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline]
    pub fn aen(&self) -> AENR {
        AENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline]
    pub fn lprioen(&self) -> LPRIOENR {
        LPRIOENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Individual Rx Masking And Queue Enable"]
    #[inline]
    pub fn irmq(&self) -> IRMQR {
        IRMQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline]
    pub fn srxdis(&self) -> SRXDISR {
        SRXDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Wake Up Source"]
    #[inline]
    pub fn waksrc(&self) -> WAKSRCR {
        WAKSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Low-Power Mode Acknowledge"]
    #[inline]
    pub fn lpmack(&self) -> LPMACKR {
        LPMACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline]
    pub fn wrnen(&self) -> WRNENR {
        WRNENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Self Wake Up"]
    #[inline]
    pub fn slfwak(&self) -> SLFWAKR {
        SLFWAKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline]
    pub fn supv(&self) -> SUPVR {
        SUPVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Freeze Mode Acknowledge"]
    #[inline]
    pub fn frzack(&self) -> FRZACKR {
        FRZACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline]
    pub fn softrst(&self) -> SOFTRSTR {
        SOFTRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Wake Up Interrupt Mask"]
    #[inline]
    pub fn wakmsk(&self) -> WAKMSKR {
        WAKMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - FlexCAN Not Ready"]
    #[inline]
    pub fn notrdy(&self) -> NOTRDYR {
        NOTRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline]
    pub fn halt(&self) -> HALTR {
        HALTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline]
    pub fn rfen(&self) -> RFENR {
        RFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline]
    pub fn frz(&self) -> FRZR {
        FRZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline]
    pub fn mdis(&self) -> MDISR {
        MDISR::_from({
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
        W { bits: 3633315855 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Number Of The Last Message Buffer"]
    #[inline]
    pub fn maxmb(&mut self) -> _MAXMBW {
        _MAXMBW { w: self }
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline]
    pub fn idam(&mut self) -> _IDAMW {
        _IDAMW { w: self }
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline]
    pub fn aen(&mut self) -> _AENW {
        _AENW { w: self }
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline]
    pub fn lprioen(&mut self) -> _LPRIOENW {
        _LPRIOENW { w: self }
    }
    #[doc = "Bit 16 - Individual Rx Masking And Queue Enable"]
    #[inline]
    pub fn irmq(&mut self) -> _IRMQW {
        _IRMQW { w: self }
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline]
    pub fn srxdis(&mut self) -> _SRXDISW {
        _SRXDISW { w: self }
    }
    #[doc = "Bit 19 - Wake Up Source"]
    #[inline]
    pub fn waksrc(&mut self) -> _WAKSRCW {
        _WAKSRCW { w: self }
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline]
    pub fn wrnen(&mut self) -> _WRNENW {
        _WRNENW { w: self }
    }
    #[doc = "Bit 22 - Self Wake Up"]
    #[inline]
    pub fn slfwak(&mut self) -> _SLFWAKW {
        _SLFWAKW { w: self }
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline]
    pub fn supv(&mut self) -> _SUPVW {
        _SUPVW { w: self }
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline]
    pub fn softrst(&mut self) -> _SOFTRSTW {
        _SOFTRSTW { w: self }
    }
    #[doc = "Bit 26 - Wake Up Interrupt Mask"]
    #[inline]
    pub fn wakmsk(&mut self) -> _WAKMSKW {
        _WAKMSKW { w: self }
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline]
    pub fn halt(&mut self) -> _HALTW {
        _HALTW { w: self }
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline]
    pub fn rfen(&mut self) -> _RFENW {
        _RFENW { w: self }
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline]
    pub fn frz(&mut self) -> _FRZW {
        _FRZW { w: self }
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline]
    pub fn mdis(&mut self) -> _MDISW {
        _MDISW { w: self }
    }
}
