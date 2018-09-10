#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C7816 {
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
#[doc = "Possible values of the field `ISO_7816E`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISO_7816ER {
    #[doc = "ISO-7816 functionality is turned off/not enabled."]
    _0,
    #[doc = "ISO-7816 functionality is turned on/enabled."]
    _1,
}
impl ISO_7816ER {
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
            ISO_7816ER::_0 => false,
            ISO_7816ER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISO_7816ER {
        match value {
            false => ISO_7816ER::_0,
            true => ISO_7816ER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ISO_7816ER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ISO_7816ER::_1
    }
}
#[doc = "Possible values of the field `TTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TTYPER {
    #[doc = "T = 0 per the ISO-7816 specification."]
    _0,
    #[doc = "T = 1 per the ISO-7816 specification."]
    _1,
}
impl TTYPER {
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
            TTYPER::_0 => false,
            TTYPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TTYPER {
        match value {
            false => TTYPER::_0,
            true => TTYPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TTYPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TTYPER::_1
    }
}
#[doc = "Possible values of the field `INIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITR {
    #[doc = "Normal operating mode. Receiver does not seek to identify initial character."]
    _0,
    #[doc = "Receiver searches for initial character."]
    _1,
}
impl INITR {
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
            INITR::_0 => false,
            INITR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INITR {
        match value {
            false => INITR::_0,
            true => INITR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INITR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INITR::_1
    }
}
#[doc = "Possible values of the field `ANACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANACKR {
    #[doc = "No NACK is automatically generated."]
    _0,
    #[doc = "A NACK is automatically generated if a parity error is detected or if an invalid initial character is detected."]
    _1,
}
impl ANACKR {
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
            ANACKR::_0 => false,
            ANACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANACKR {
        match value {
            false => ANACKR::_0,
            true => ANACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ANACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ANACKR::_1
    }
}
#[doc = "Possible values of the field `ONACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONACKR {
    #[doc = "The received data does not generate a NACK when the receipt of the data results in an overflow event."]
    _0,
    #[doc = "If the receiver buffer overflows, a NACK is automatically sent on a received character."]
    _1,
}
impl ONACKR {
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
            ONACKR::_0 => false,
            ONACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONACKR {
        match value {
            false => ONACKR::_0,
            true => ONACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ONACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ONACKR::_1
    }
}
#[doc = "Values that can be written to the field `ISO_7816E`"]
pub enum ISO_7816EW {
    #[doc = "ISO-7816 functionality is turned off/not enabled."]
    _0,
    #[doc = "ISO-7816 functionality is turned on/enabled."]
    _1,
}
impl ISO_7816EW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISO_7816EW::_0 => false,
            ISO_7816EW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISO_7816EW<'a> {
    w: &'a mut W,
}
impl<'a> _ISO_7816EW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISO_7816EW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ISO-7816 functionality is turned off/not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISO_7816EW::_0)
    }
    #[doc = "ISO-7816 functionality is turned on/enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISO_7816EW::_1)
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
#[doc = "Values that can be written to the field `TTYPE`"]
pub enum TTYPEW {
    #[doc = "T = 0 per the ISO-7816 specification."]
    _0,
    #[doc = "T = 1 per the ISO-7816 specification."]
    _1,
}
impl TTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TTYPEW::_0 => false,
            TTYPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TTYPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "T = 0 per the ISO-7816 specification."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TTYPEW::_0)
    }
    #[doc = "T = 1 per the ISO-7816 specification."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TTYPEW::_1)
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
#[doc = "Values that can be written to the field `INIT`"]
pub enum INITW {
    #[doc = "Normal operating mode. Receiver does not seek to identify initial character."]
    _0,
    #[doc = "Receiver searches for initial character."]
    _1,
}
impl INITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INITW::_0 => false,
            INITW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INITW<'a> {
    w: &'a mut W,
}
impl<'a> _INITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operating mode. Receiver does not seek to identify initial character."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INITW::_0)
    }
    #[doc = "Receiver searches for initial character."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INITW::_1)
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
#[doc = "Values that can be written to the field `ANACK`"]
pub enum ANACKW {
    #[doc = "No NACK is automatically generated."]
    _0,
    #[doc = "A NACK is automatically generated if a parity error is detected or if an invalid initial character is detected."]
    _1,
}
impl ANACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANACKW::_0 => false,
            ANACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANACKW<'a> {
    w: &'a mut W,
}
impl<'a> _ANACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No NACK is automatically generated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANACKW::_0)
    }
    #[doc = "A NACK is automatically generated if a parity error is detected or if an invalid initial character is detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANACKW::_1)
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
#[doc = "Values that can be written to the field `ONACK`"]
pub enum ONACKW {
    #[doc = "The received data does not generate a NACK when the receipt of the data results in an overflow event."]
    _0,
    #[doc = "If the receiver buffer overflows, a NACK is automatically sent on a received character."]
    _1,
}
impl ONACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONACKW::_0 => false,
            ONACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONACKW<'a> {
    w: &'a mut W,
}
impl<'a> _ONACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The received data does not generate a NACK when the receipt of the data results in an overflow event."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONACKW::_0)
    }
    #[doc = "If the receiver buffer overflows, a NACK is automatically sent on a received character."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONACKW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - ISO-7816 Functionality Enabled"]
    #[inline]
    pub fn iso_7816e(&self) -> ISO_7816ER {
        ISO_7816ER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Transfer Type"]
    #[inline]
    pub fn ttype(&self) -> TTYPER {
        TTYPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Detect Initial Character"]
    #[inline]
    pub fn init(&self) -> INITR {
        INITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Generate NACK on Error"]
    #[inline]
    pub fn anack(&self) -> ANACKR {
        ANACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Generate NACK on Overflow"]
    #[inline]
    pub fn onack(&self) -> ONACKR {
        ONACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - ISO-7816 Functionality Enabled"]
    #[inline]
    pub fn iso_7816e(&mut self) -> _ISO_7816EW {
        _ISO_7816EW { w: self }
    }
    #[doc = "Bit 1 - Transfer Type"]
    #[inline]
    pub fn ttype(&mut self) -> _TTYPEW {
        _TTYPEW { w: self }
    }
    #[doc = "Bit 2 - Detect Initial Character"]
    #[inline]
    pub fn init(&mut self) -> _INITW {
        _INITW { w: self }
    }
    #[doc = "Bit 3 - Generate NACK on Error"]
    #[inline]
    pub fn anack(&mut self) -> _ANACKW {
        _ANACKW { w: self }
    }
    #[doc = "Bit 4 - Generate NACK on Overflow"]
    #[inline]
    pub fn onack(&mut self) -> _ONACKW {
        _ONACKW { w: self }
    }
}
