#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C2 {
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
pub struct ADR {
    bits: u8,
}
impl ADR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMENR {
    #[doc = "Range mode disabled. No address matching occurs for an address within the range of values of the A1 and RA registers."]
    _0,
    #[doc = "Range mode enabled. Address matching occurs when a slave receives an address within the range of values of the A1 and RA registers."]
    _1,
}
impl RMENR {
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
            RMENR::_0 => false,
            RMENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMENR {
        match value {
            false => RMENR::_0,
            true => RMENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RMENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RMENR::_1
    }
}
#[doc = "Possible values of the field `SBRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBRCR {
    #[doc = "The slave baud rate follows the master baud rate and clock stretching may occur"]
    _0,
    #[doc = "Slave baud rate is independent of the master baud rate"]
    _1,
}
impl SBRCR {
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
            SBRCR::_0 => false,
            SBRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBRCR {
        match value {
            false => SBRCR::_0,
            true => SBRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SBRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SBRCR::_1
    }
}
#[doc = "Possible values of the field `HDRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDRSR {
    #[doc = "Normal drive mode"]
    _0,
    #[doc = "High drive mode"]
    _1,
}
impl HDRSR {
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
            HDRSR::_0 => false,
            HDRSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HDRSR {
        match value {
            false => HDRSR::_0,
            true => HDRSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HDRSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HDRSR::_1
    }
}
#[doc = "Possible values of the field `ADEXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADEXTR {
    #[doc = "7-bit address scheme"]
    _0,
    #[doc = "10-bit address scheme"]
    _1,
}
impl ADEXTR {
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
            ADEXTR::_0 => false,
            ADEXTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADEXTR {
        match value {
            false => ADEXTR::_0,
            true => ADEXTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADEXTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADEXTR::_1
    }
}
#[doc = "Possible values of the field `GCAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCAENR {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl GCAENR {
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
            GCAENR::_0 => false,
            GCAENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GCAENR {
        match value {
            false => GCAENR::_0,
            true => GCAENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GCAENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GCAENR::_1
    }
}
#[doc = r" Proxy"]
pub struct _ADW<'a> {
    w: &'a mut W,
}
impl<'a> _ADW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RMEN`"]
pub enum RMENW {
    #[doc = "Range mode disabled. No address matching occurs for an address within the range of values of the A1 and RA registers."]
    _0,
    #[doc = "Range mode enabled. Address matching occurs when a slave receives an address within the range of values of the A1 and RA registers."]
    _1,
}
impl RMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMENW::_0 => false,
            RMENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMENW<'a> {
    w: &'a mut W,
}
impl<'a> _RMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Range mode disabled. No address matching occurs for an address within the range of values of the A1 and RA registers."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMENW::_0)
    }
    #[doc = "Range mode enabled. Address matching occurs when a slave receives an address within the range of values of the A1 and RA registers."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMENW::_1)
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
#[doc = "Values that can be written to the field `SBRC`"]
pub enum SBRCW {
    #[doc = "The slave baud rate follows the master baud rate and clock stretching may occur"]
    _0,
    #[doc = "Slave baud rate is independent of the master baud rate"]
    _1,
}
impl SBRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBRCW::_0 => false,
            SBRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SBRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The slave baud rate follows the master baud rate and clock stretching may occur"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBRCW::_0)
    }
    #[doc = "Slave baud rate is independent of the master baud rate"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBRCW::_1)
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
#[doc = "Values that can be written to the field `HDRS`"]
pub enum HDRSW {
    #[doc = "Normal drive mode"]
    _0,
    #[doc = "High drive mode"]
    _1,
}
impl HDRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HDRSW::_0 => false,
            HDRSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDRSW<'a> {
    w: &'a mut W,
}
impl<'a> _HDRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HDRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal drive mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HDRSW::_0)
    }
    #[doc = "High drive mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HDRSW::_1)
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
#[doc = "Values that can be written to the field `ADEXT`"]
pub enum ADEXTW {
    #[doc = "7-bit address scheme"]
    _0,
    #[doc = "10-bit address scheme"]
    _1,
}
impl ADEXTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADEXTW::_0 => false,
            ADEXTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADEXTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADEXTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADEXTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "7-bit address scheme"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADEXTW::_0)
    }
    #[doc = "10-bit address scheme"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADEXTW::_1)
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
#[doc = "Values that can be written to the field `GCAEN`"]
pub enum GCAENW {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl GCAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GCAENW::_0 => false,
            GCAENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GCAENW<'a> {
    w: &'a mut W,
}
impl<'a> _GCAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GCAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GCAENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GCAENW::_1)
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
    #[doc = "Bits 0:2 - Slave Address"]
    #[inline]
    pub fn ad(&self) -> ADR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        ADR { bits }
    }
    #[doc = "Bit 3 - Range Address Matching Enable"]
    #[inline]
    pub fn rmen(&self) -> RMENR {
        RMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Slave Baud Rate Control"]
    #[inline]
    pub fn sbrc(&self) -> SBRCR {
        SBRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - High Drive Select"]
    #[inline]
    pub fn hdrs(&self) -> HDRSR {
        HDRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Address Extension"]
    #[inline]
    pub fn adext(&self) -> ADEXTR {
        ADEXTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - General Call Address Enable"]
    #[inline]
    pub fn gcaen(&self) -> GCAENR {
        GCAENR::_from({
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
    #[doc = "Bits 0:2 - Slave Address"]
    #[inline]
    pub fn ad(&mut self) -> _ADW {
        _ADW { w: self }
    }
    #[doc = "Bit 3 - Range Address Matching Enable"]
    #[inline]
    pub fn rmen(&mut self) -> _RMENW {
        _RMENW { w: self }
    }
    #[doc = "Bit 4 - Slave Baud Rate Control"]
    #[inline]
    pub fn sbrc(&mut self) -> _SBRCW {
        _SBRCW { w: self }
    }
    #[doc = "Bit 5 - High Drive Select"]
    #[inline]
    pub fn hdrs(&mut self) -> _HDRSW {
        _HDRSW { w: self }
    }
    #[doc = "Bit 6 - Address Extension"]
    #[inline]
    pub fn adext(&mut self) -> _ADEXTW {
        _ADEXTW { w: self }
    }
    #[doc = "Bit 7 - General Call Address Enable"]
    #[inline]
    pub fn gcaen(&mut self) -> _GCAENW {
        _GCAENW { w: self }
    }
}
