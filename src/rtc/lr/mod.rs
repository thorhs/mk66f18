#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LR {
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
#[doc = "Possible values of the field `TCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCLR {
    #[doc = "Time Compensation Register is locked and writes are ignored."]
    _0,
    #[doc = "Time Compensation Register is not locked and writes complete as normal."]
    _1,
}
impl TCLR {
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
            TCLR::_0 => false,
            TCLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCLR {
        match value {
            false => TCLR::_0,
            true => TCLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCLR::_1
    }
}
#[doc = "Possible values of the field `CRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRLR {
    #[doc = "Control Register is locked and writes are ignored."]
    _0,
    #[doc = "Control Register is not locked and writes complete as normal."]
    _1,
}
impl CRLR {
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
            CRLR::_0 => false,
            CRLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRLR {
        match value {
            false => CRLR::_0,
            true => CRLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRLR::_1
    }
}
#[doc = "Possible values of the field `SRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRLR {
    #[doc = "Status Register is locked and writes are ignored."]
    _0,
    #[doc = "Status Register is not locked and writes complete as normal."]
    _1,
}
impl SRLR {
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
            SRLR::_0 => false,
            SRLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRLR {
        match value {
            false => SRLR::_0,
            true => SRLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SRLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SRLR::_1
    }
}
#[doc = "Possible values of the field `LRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRLR {
    #[doc = "Lock Register is locked and writes are ignored."]
    _0,
    #[doc = "Lock Register is not locked and writes complete as normal."]
    _1,
}
impl LRLR {
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
            LRLR::_0 => false,
            LRLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRLR {
        match value {
            false => LRLR::_0,
            true => LRLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LRLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LRLR::_1
    }
}
#[doc = "Possible values of the field `TTSL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TTSLR {
    #[doc = "Tamper Time Seconds Register is locked and writes are ignored."]
    _0,
    #[doc = "Tamper Time Seconds Register is not locked and writes complete as normal."]
    _1,
}
impl TTSLR {
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
            TTSLR::_0 => false,
            TTSLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TTSLR {
        match value {
            false => TTSLR::_0,
            true => TTSLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TTSLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TTSLR::_1
    }
}
#[doc = "Possible values of the field `MEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MELR {
    #[doc = "Monotonic Enable Register is locked and writes are ignored."]
    _0,
    #[doc = "Monotonic Enable Register is not locked and writes complete as normal."]
    _1,
}
impl MELR {
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
            MELR::_0 => false,
            MELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MELR {
        match value {
            false => MELR::_0,
            true => MELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MELR::_1
    }
}
#[doc = "Possible values of the field `MCLL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLLR {
    #[doc = "Monotonic Counter Low Register is locked and writes are ignored."]
    _0,
    #[doc = "Monotonic Counter Low Register is not locked and writes complete as normal."]
    _1,
}
impl MCLLR {
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
            MCLLR::_0 => false,
            MCLLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCLLR {
        match value {
            false => MCLLR::_0,
            true => MCLLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MCLLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MCLLR::_1
    }
}
#[doc = "Possible values of the field `MCHL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCHLR {
    #[doc = "Monotonic Counter High Register is locked and writes are ignored."]
    _0,
    #[doc = "Monotonic Counter High Register is not locked and writes complete as normal."]
    _1,
}
impl MCHLR {
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
            MCHLR::_0 => false,
            MCHLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCHLR {
        match value {
            false => MCHLR::_0,
            true => MCHLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MCHLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MCHLR::_1
    }
}
#[doc = "Values that can be written to the field `TCL`"]
pub enum TCLW {
    #[doc = "Time Compensation Register is locked and writes are ignored."]
    _0,
    #[doc = "Time Compensation Register is not locked and writes complete as normal."]
    _1,
}
impl TCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCLW::_0 => false,
            TCLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCLW<'a> {
    w: &'a mut W,
}
impl<'a> _TCLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Time Compensation Register is locked and writes are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCLW::_0)
    }
    #[doc = "Time Compensation Register is not locked and writes complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCLW::_1)
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
#[doc = "Values that can be written to the field `CRL`"]
pub enum CRLW {
    #[doc = "Control Register is locked and writes are ignored."]
    _0,
    #[doc = "Control Register is not locked and writes complete as normal."]
    _1,
}
impl CRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRLW::_0 => false,
            CRLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Control Register is locked and writes are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRLW::_0)
    }
    #[doc = "Control Register is not locked and writes complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRLW::_1)
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
#[doc = "Values that can be written to the field `SRL`"]
pub enum SRLW {
    #[doc = "Status Register is locked and writes are ignored."]
    _0,
    #[doc = "Status Register is not locked and writes complete as normal."]
    _1,
}
impl SRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRLW::_0 => false,
            SRLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRLW<'a> {
    w: &'a mut W,
}
impl<'a> _SRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Status Register is locked and writes are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRLW::_0)
    }
    #[doc = "Status Register is not locked and writes complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRLW::_1)
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
#[doc = "Values that can be written to the field `LRL`"]
pub enum LRLW {
    #[doc = "Lock Register is locked and writes are ignored."]
    _0,
    #[doc = "Lock Register is not locked and writes complete as normal."]
    _1,
}
impl LRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LRLW::_0 => false,
            LRLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LRLW<'a> {
    w: &'a mut W,
}
impl<'a> _LRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lock Register is locked and writes are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LRLW::_0)
    }
    #[doc = "Lock Register is not locked and writes complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LRLW::_1)
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
#[doc = "Values that can be written to the field `TTSL`"]
pub enum TTSLW {
    #[doc = "Tamper Time Seconds Register is locked and writes are ignored."]
    _0,
    #[doc = "Tamper Time Seconds Register is not locked and writes complete as normal."]
    _1,
}
impl TTSLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TTSLW::_0 => false,
            TTSLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TTSLW<'a> {
    w: &'a mut W,
}
impl<'a> _TTSLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TTSLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Tamper Time Seconds Register is locked and writes are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TTSLW::_0)
    }
    #[doc = "Tamper Time Seconds Register is not locked and writes complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TTSLW::_1)
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
#[doc = "Values that can be written to the field `MEL`"]
pub enum MELW {
    #[doc = "Monotonic Enable Register is locked and writes are ignored."]
    _0,
    #[doc = "Monotonic Enable Register is not locked and writes complete as normal."]
    _1,
}
impl MELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MELW::_0 => false,
            MELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MELW<'a> {
    w: &'a mut W,
}
impl<'a> _MELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Monotonic Enable Register is locked and writes are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MELW::_0)
    }
    #[doc = "Monotonic Enable Register is not locked and writes complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MELW::_1)
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
#[doc = "Values that can be written to the field `MCLL`"]
pub enum MCLLW {
    #[doc = "Monotonic Counter Low Register is locked and writes are ignored."]
    _0,
    #[doc = "Monotonic Counter Low Register is not locked and writes complete as normal."]
    _1,
}
impl MCLLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCLLW::_0 => false,
            MCLLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCLLW<'a> {
    w: &'a mut W,
}
impl<'a> _MCLLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCLLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Monotonic Counter Low Register is locked and writes are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCLLW::_0)
    }
    #[doc = "Monotonic Counter Low Register is not locked and writes complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCLLW::_1)
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
#[doc = "Values that can be written to the field `MCHL`"]
pub enum MCHLW {
    #[doc = "Monotonic Counter High Register is locked and writes are ignored."]
    _0,
    #[doc = "Monotonic Counter High Register is not locked and writes complete as normal."]
    _1,
}
impl MCHLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCHLW::_0 => false,
            MCHLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCHLW<'a> {
    w: &'a mut W,
}
impl<'a> _MCHLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCHLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Monotonic Counter High Register is locked and writes are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCHLW::_0)
    }
    #[doc = "Monotonic Counter High Register is not locked and writes complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCHLW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 3 - Time Compensation Lock"]
    #[inline]
    pub fn tcl(&self) -> TCLR {
        TCLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Control Register Lock"]
    #[inline]
    pub fn crl(&self) -> CRLR {
        CRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Status Register Lock"]
    #[inline]
    pub fn srl(&self) -> SRLR {
        SRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Lock Register Lock"]
    #[inline]
    pub fn lrl(&self) -> LRLR {
        LRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Tamper Time Seconds Lock"]
    #[inline]
    pub fn ttsl(&self) -> TTSLR {
        TTSLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Monotonic Enable Lock"]
    #[inline]
    pub fn mel(&self) -> MELR {
        MELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Monotonic Counter Low Lock"]
    #[inline]
    pub fn mcll(&self) -> MCLLR {
        MCLLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Monotonic Counter High Lock"]
    #[inline]
    pub fn mchl(&self) -> MCHLR {
        MCHLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - Time Compensation Lock"]
    #[inline]
    pub fn tcl(&mut self) -> _TCLW {
        _TCLW { w: self }
    }
    #[doc = "Bit 4 - Control Register Lock"]
    #[inline]
    pub fn crl(&mut self) -> _CRLW {
        _CRLW { w: self }
    }
    #[doc = "Bit 5 - Status Register Lock"]
    #[inline]
    pub fn srl(&mut self) -> _SRLW {
        _SRLW { w: self }
    }
    #[doc = "Bit 6 - Lock Register Lock"]
    #[inline]
    pub fn lrl(&mut self) -> _LRLW {
        _LRLW { w: self }
    }
    #[doc = "Bit 8 - Tamper Time Seconds Lock"]
    #[inline]
    pub fn ttsl(&mut self) -> _TTSLW {
        _TTSLW { w: self }
    }
    #[doc = "Bit 9 - Monotonic Enable Lock"]
    #[inline]
    pub fn mel(&mut self) -> _MELW {
        _MELW { w: self }
    }
    #[doc = "Bit 10 - Monotonic Counter Low Lock"]
    #[inline]
    pub fn mcll(&mut self) -> _MCLLW {
        _MCLLW { w: self }
    }
    #[doc = "Bit 11 - Monotonic Counter High Lock"]
    #[inline]
    pub fn mchl(&mut self) -> _MCHLW {
        _MCHLW { w: self }
    }
}
