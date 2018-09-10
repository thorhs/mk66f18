#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::IS7816 {
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
#[doc = "Possible values of the field `RXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTR {
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is less than or equal to the value in ET7816\\[RXTHRESHOLD\\]."]
    _0,
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is greater than the value in ET7816\\[RXTHRESHOLD\\]."]
    _1,
}
impl RXTR {
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
            RXTR::_0 => false,
            RXTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTR {
        match value {
            false => RXTR::_0,
            true => RXTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXTR::_1
    }
}
#[doc = "Possible values of the field `TXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTR {
    #[doc = "The number of retries and corresponding NACKS does not exceed the value in ET7816\\[TXTHRESHOLD\\]."]
    _0,
    #[doc = "The number of retries and corresponding NACKS exceeds the value in ET7816\\[TXTHRESHOLD\\]."]
    _1,
}
impl TXTR {
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
            TXTR::_0 => false,
            TXTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXTR {
        match value {
            false => TXTR::_0,
            true => TXTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXTR::_1
    }
}
#[doc = "Possible values of the field `GTV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTVR {
    #[doc = "A guard time (GT, CGT, or BGT) has not been violated."]
    _0,
    #[doc = "A guard time (GT, CGT, or BGT) has been violated."]
    _1,
}
impl GTVR {
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
            GTVR::_0 => false,
            GTVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GTVR {
        match value {
            false => GTVR::_0,
            true => GTVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GTVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GTVR::_1
    }
}
#[doc = "Possible values of the field `ADT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADTR {
    #[doc = "ATR Duration time (ADT) has not been violated."]
    _0,
    #[doc = "ATR Duration time (ADT) has been violated."]
    _1,
}
impl ADTR {
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
            ADTR::_0 => false,
            ADTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADTR {
        match value {
            false => ADTR::_0,
            true => ADTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADTR::_1
    }
}
#[doc = "Possible values of the field `INITD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITDR {
    #[doc = "A valid initial character has not been received."]
    _0,
    #[doc = "A valid initial character has been received."]
    _1,
}
impl INITDR {
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
            INITDR::_0 => false,
            INITDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INITDR {
        match value {
            false => INITDR::_0,
            true => INITDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INITDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INITDR::_1
    }
}
#[doc = "Possible values of the field `BWT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWTR {
    #[doc = "Block wait time (BWT) has not been violated."]
    _0,
    #[doc = "Block wait time (BWT) has been violated."]
    _1,
}
impl BWTR {
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
            BWTR::_0 => false,
            BWTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWTR {
        match value {
            false => BWTR::_0,
            true => BWTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BWTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BWTR::_1
    }
}
#[doc = "Possible values of the field `CWT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWTR {
    #[doc = "Character wait time (CWT) has not been violated."]
    _0,
    #[doc = "Character wait time (CWT) has been violated."]
    _1,
}
impl CWTR {
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
            CWTR::_0 => false,
            CWTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CWTR {
        match value {
            false => CWTR::_0,
            true => CWTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CWTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CWTR::_1
    }
}
#[doc = "Possible values of the field `WT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTR {
    #[doc = "Wait time (WT) has not been violated."]
    _0,
    #[doc = "Wait time (WT) has been violated."]
    _1,
}
impl WTR {
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
            WTR::_0 => false,
            WTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WTR {
        match value {
            false => WTR::_0,
            true => WTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WTR::_1
    }
}
#[doc = "Values that can be written to the field `RXT`"]
pub enum RXTW {
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is less than or equal to the value in ET7816\\[RXTHRESHOLD\\]."]
    _0,
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is greater than the value in ET7816\\[RXTHRESHOLD\\]."]
    _1,
}
impl RXTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTW::_0 => false,
            RXTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is less than or equal to the value in ET7816\\[RXTHRESHOLD\\]."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTW::_0)
    }
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is greater than the value in ET7816\\[RXTHRESHOLD\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXT`"]
pub enum TXTW {
    #[doc = "The number of retries and corresponding NACKS does not exceed the value in ET7816\\[TXTHRESHOLD\\]."]
    _0,
    #[doc = "The number of retries and corresponding NACKS exceeds the value in ET7816\\[TXTHRESHOLD\\]."]
    _1,
}
impl TXTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXTW::_0 => false,
            TXTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The number of retries and corresponding NACKS does not exceed the value in ET7816\\[TXTHRESHOLD\\]."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXTW::_0)
    }
    #[doc = "The number of retries and corresponding NACKS exceeds the value in ET7816\\[TXTHRESHOLD\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXTW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GTV`"]
pub enum GTVW {
    #[doc = "A guard time (GT, CGT, or BGT) has not been violated."]
    _0,
    #[doc = "A guard time (GT, CGT, or BGT) has been violated."]
    _1,
}
impl GTVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GTVW::_0 => false,
            GTVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GTVW<'a> {
    w: &'a mut W,
}
impl<'a> _GTVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GTVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A guard time (GT, CGT, or BGT) has not been violated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTVW::_0)
    }
    #[doc = "A guard time (GT, CGT, or BGT) has been violated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTVW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADT`"]
pub enum ADTW {
    #[doc = "ATR Duration time (ADT) has not been violated."]
    _0,
    #[doc = "ATR Duration time (ADT) has been violated."]
    _1,
}
impl ADTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADTW::_0 => false,
            ADTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ATR Duration time (ADT) has not been violated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTW::_0)
    }
    #[doc = "ATR Duration time (ADT) has been violated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INITD`"]
pub enum INITDW {
    #[doc = "A valid initial character has not been received."]
    _0,
    #[doc = "A valid initial character has been received."]
    _1,
}
impl INITDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INITDW::_0 => false,
            INITDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INITDW<'a> {
    w: &'a mut W,
}
impl<'a> _INITDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INITDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A valid initial character has not been received."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INITDW::_0)
    }
    #[doc = "A valid initial character has been received."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INITDW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BWT`"]
pub enum BWTW {
    #[doc = "Block wait time (BWT) has not been violated."]
    _0,
    #[doc = "Block wait time (BWT) has been violated."]
    _1,
}
impl BWTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWTW::_0 => false,
            BWTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWTW<'a> {
    w: &'a mut W,
}
impl<'a> _BWTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Block wait time (BWT) has not been violated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWTW::_0)
    }
    #[doc = "Block wait time (BWT) has been violated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWTW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CWT`"]
pub enum CWTW {
    #[doc = "Character wait time (CWT) has not been violated."]
    _0,
    #[doc = "Character wait time (CWT) has been violated."]
    _1,
}
impl CWTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CWTW::_0 => false,
            CWTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CWTW<'a> {
    w: &'a mut W,
}
impl<'a> _CWTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CWTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Character wait time (CWT) has not been violated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CWTW::_0)
    }
    #[doc = "Character wait time (CWT) has been violated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CWTW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WT`"]
pub enum WTW {
    #[doc = "Wait time (WT) has not been violated."]
    _0,
    #[doc = "Wait time (WT) has been violated."]
    _1,
}
impl WTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WTW::_0 => false,
            WTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WTW<'a> {
    w: &'a mut W,
}
impl<'a> _WTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wait time (WT) has not been violated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WTW::_0)
    }
    #[doc = "Wait time (WT) has been violated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WTW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt"]
    #[inline]
    pub fn rxt(&self) -> RXTR {
        RXTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt"]
    #[inline]
    pub fn txt(&self) -> TXTR {
        TXTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt"]
    #[inline]
    pub fn gtv(&self) -> GTVR {
        GTVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - ATR Duration Time Interrupt"]
    #[inline]
    pub fn adt(&self) -> ADTR {
        ADTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt"]
    #[inline]
    pub fn initd(&self) -> INITDR {
        INITDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt"]
    #[inline]
    pub fn bwt(&self) -> BWTR {
        BWTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt"]
    #[inline]
    pub fn cwt(&self) -> CWTR {
        CWTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Wait Timer Interrupt"]
    #[inline]
    pub fn wt(&self) -> WTR {
        WTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt"]
    #[inline]
    pub fn rxt(&mut self) -> _RXTW {
        _RXTW { w: self }
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt"]
    #[inline]
    pub fn txt(&mut self) -> _TXTW {
        _TXTW { w: self }
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt"]
    #[inline]
    pub fn gtv(&mut self) -> _GTVW {
        _GTVW { w: self }
    }
    #[doc = "Bit 3 - ATR Duration Time Interrupt"]
    #[inline]
    pub fn adt(&mut self) -> _ADTW {
        _ADTW { w: self }
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt"]
    #[inline]
    pub fn initd(&mut self) -> _INITDW {
        _INITDW { w: self }
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt"]
    #[inline]
    pub fn bwt(&mut self) -> _BWTW {
        _BWTW { w: self }
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt"]
    #[inline]
    pub fn cwt(&mut self) -> _CWTW {
        _CWTW { w: self }
    }
    #[doc = "Bit 7 - Wait Timer Interrupt"]
    #[inline]
    pub fn wt(&mut self) -> _WTW {
        _WTW { w: self }
    }
}
