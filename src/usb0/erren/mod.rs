#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::ERREN {
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
#[doc = "Possible values of the field `PIDERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDERRENR {
    #[doc = "Disables the PIDERR interrupt."]
    _0,
    #[doc = "Enters the PIDERR interrupt."]
    _1,
}
impl PIDERRENR {
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
            PIDERRENR::_0 => false,
            PIDERRENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIDERRENR {
        match value {
            false => PIDERRENR::_0,
            true => PIDERRENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PIDERRENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PIDERRENR::_1
    }
}
#[doc = "Possible values of the field `CRC5EOFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC5EOFENR {
    #[doc = "Disables the CRC5/EOF interrupt."]
    _0,
    #[doc = "Enables the CRC5/EOF interrupt."]
    _1,
}
impl CRC5EOFENR {
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
            CRC5EOFENR::_0 => false,
            CRC5EOFENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC5EOFENR {
        match value {
            false => CRC5EOFENR::_0,
            true => CRC5EOFENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRC5EOFENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRC5EOFENR::_1
    }
}
#[doc = "Possible values of the field `CRC16EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC16ENR {
    #[doc = "Disables the CRC16 interrupt."]
    _0,
    #[doc = "Enables the CRC16 interrupt."]
    _1,
}
impl CRC16ENR {
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
            CRC16ENR::_0 => false,
            CRC16ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRC16ENR {
        match value {
            false => CRC16ENR::_0,
            true => CRC16ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRC16ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRC16ENR::_1
    }
}
#[doc = "Possible values of the field `DFN8EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFN8ENR {
    #[doc = "Disables the DFN8 interrupt."]
    _0,
    #[doc = "Enables the DFN8 interrupt."]
    _1,
}
impl DFN8ENR {
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
            DFN8ENR::_0 => false,
            DFN8ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFN8ENR {
        match value {
            false => DFN8ENR::_0,
            true => DFN8ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFN8ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFN8ENR::_1
    }
}
#[doc = "Possible values of the field `BTOERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTOERRENR {
    #[doc = "Disables the BTOERR interrupt."]
    _0,
    #[doc = "Enables the BTOERR interrupt."]
    _1,
}
impl BTOERRENR {
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
            BTOERRENR::_0 => false,
            BTOERRENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BTOERRENR {
        match value {
            false => BTOERRENR::_0,
            true => BTOERRENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BTOERRENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BTOERRENR::_1
    }
}
#[doc = "Possible values of the field `DMAERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAERRENR {
    #[doc = "Disables the DMAERR interrupt."]
    _0,
    #[doc = "Enables the DMAERR interrupt."]
    _1,
}
impl DMAERRENR {
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
            DMAERRENR::_0 => false,
            DMAERRENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAERRENR {
        match value {
            false => DMAERRENR::_0,
            true => DMAERRENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAERRENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAERRENR::_1
    }
}
#[doc = "Possible values of the field `BTSERREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTSERRENR {
    #[doc = "Disables the BTSERR interrupt."]
    _0,
    #[doc = "Enables the BTSERR interrupt."]
    _1,
}
impl BTSERRENR {
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
            BTSERRENR::_0 => false,
            BTSERRENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BTSERRENR {
        match value {
            false => BTSERRENR::_0,
            true => BTSERRENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BTSERRENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BTSERRENR::_1
    }
}
#[doc = "Values that can be written to the field `PIDERREN`"]
pub enum PIDERRENW {
    #[doc = "Disables the PIDERR interrupt."]
    _0,
    #[doc = "Enters the PIDERR interrupt."]
    _1,
}
impl PIDERRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIDERRENW::_0 => false,
            PIDERRENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIDERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _PIDERRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIDERRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the PIDERR interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIDERRENW::_0)
    }
    #[doc = "Enters the PIDERR interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIDERRENW::_1)
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
#[doc = "Values that can be written to the field `CRC5EOFEN`"]
pub enum CRC5EOFENW {
    #[doc = "Disables the CRC5/EOF interrupt."]
    _0,
    #[doc = "Enables the CRC5/EOF interrupt."]
    _1,
}
impl CRC5EOFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC5EOFENW::_0 => false,
            CRC5EOFENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC5EOFENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC5EOFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC5EOFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the CRC5/EOF interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC5EOFENW::_0)
    }
    #[doc = "Enables the CRC5/EOF interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC5EOFENW::_1)
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
#[doc = "Values that can be written to the field `CRC16EN`"]
pub enum CRC16ENW {
    #[doc = "Disables the CRC16 interrupt."]
    _0,
    #[doc = "Enables the CRC16 interrupt."]
    _1,
}
impl CRC16ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRC16ENW::_0 => false,
            CRC16ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRC16ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC16ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC16ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the CRC16 interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC16ENW::_0)
    }
    #[doc = "Enables the CRC16 interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC16ENW::_1)
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
#[doc = "Values that can be written to the field `DFN8EN`"]
pub enum DFN8ENW {
    #[doc = "Disables the DFN8 interrupt."]
    _0,
    #[doc = "Enables the DFN8 interrupt."]
    _1,
}
impl DFN8ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFN8ENW::_0 => false,
            DFN8ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFN8ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DFN8ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFN8ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the DFN8 interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFN8ENW::_0)
    }
    #[doc = "Enables the DFN8 interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFN8ENW::_1)
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
#[doc = "Values that can be written to the field `BTOERREN`"]
pub enum BTOERRENW {
    #[doc = "Disables the BTOERR interrupt."]
    _0,
    #[doc = "Enables the BTOERR interrupt."]
    _1,
}
impl BTOERRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BTOERRENW::_0 => false,
            BTOERRENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BTOERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _BTOERRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BTOERRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the BTOERR interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BTOERRENW::_0)
    }
    #[doc = "Enables the BTOERR interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BTOERRENW::_1)
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
#[doc = "Values that can be written to the field `DMAERREN`"]
pub enum DMAERRENW {
    #[doc = "Disables the DMAERR interrupt."]
    _0,
    #[doc = "Enables the DMAERR interrupt."]
    _1,
}
impl DMAERRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAERRENW::_0 => false,
            DMAERRENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAERRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAERRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the DMAERR interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAERRENW::_0)
    }
    #[doc = "Enables the DMAERR interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAERRENW::_1)
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
#[doc = "Values that can be written to the field `BTSERREN`"]
pub enum BTSERRENW {
    #[doc = "Disables the BTSERR interrupt."]
    _0,
    #[doc = "Enables the BTSERR interrupt."]
    _1,
}
impl BTSERRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BTSERRENW::_0 => false,
            BTSERRENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BTSERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _BTSERRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BTSERRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables the BTSERR interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BTSERRENW::_0)
    }
    #[doc = "Enables the BTSERR interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BTSERRENW::_1)
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
    #[doc = "Bit 0 - PIDERR Interrupt Enable"]
    #[inline]
    pub fn piderren(&self) -> PIDERRENR {
        PIDERRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - CRC5/EOF Interrupt Enable"]
    #[inline]
    pub fn crc5eofen(&self) -> CRC5EOFENR {
        CRC5EOFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - CRC16 Interrupt Enable"]
    #[inline]
    pub fn crc16en(&self) -> CRC16ENR {
        CRC16ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - DFN8 Interrupt Enable"]
    #[inline]
    pub fn dfn8en(&self) -> DFN8ENR {
        DFN8ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - BTOERR Interrupt Enable"]
    #[inline]
    pub fn btoerren(&self) -> BTOERRENR {
        BTOERRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - DMAERR Interrupt Enable"]
    #[inline]
    pub fn dmaerren(&self) -> DMAERRENR {
        DMAERRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - BTSERR Interrupt Enable"]
    #[inline]
    pub fn btserren(&self) -> BTSERRENR {
        BTSERRENR::_from({
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
    #[doc = "Bit 0 - PIDERR Interrupt Enable"]
    #[inline]
    pub fn piderren(&mut self) -> _PIDERRENW {
        _PIDERRENW { w: self }
    }
    #[doc = "Bit 1 - CRC5/EOF Interrupt Enable"]
    #[inline]
    pub fn crc5eofen(&mut self) -> _CRC5EOFENW {
        _CRC5EOFENW { w: self }
    }
    #[doc = "Bit 2 - CRC16 Interrupt Enable"]
    #[inline]
    pub fn crc16en(&mut self) -> _CRC16ENW {
        _CRC16ENW { w: self }
    }
    #[doc = "Bit 3 - DFN8 Interrupt Enable"]
    #[inline]
    pub fn dfn8en(&mut self) -> _DFN8ENW {
        _DFN8ENW { w: self }
    }
    #[doc = "Bit 4 - BTOERR Interrupt Enable"]
    #[inline]
    pub fn btoerren(&mut self) -> _BTOERRENW {
        _BTOERRENW { w: self }
    }
    #[doc = "Bit 5 - DMAERR Interrupt Enable"]
    #[inline]
    pub fn dmaerren(&mut self) -> _DMAERRENW {
        _DMAERRENW { w: self }
    }
    #[doc = "Bit 7 - BTSERR Interrupt Enable"]
    #[inline]
    pub fn btserren(&mut self) -> _BTSERRENW {
        _BTSERRENW { w: self }
    }
}
