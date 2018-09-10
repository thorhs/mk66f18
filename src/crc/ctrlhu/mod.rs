#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CTRLHU {
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
#[doc = "Possible values of the field `TCRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCRCR {
    #[doc = "16-bit CRC protocol."]
    _0,
    #[doc = "32-bit CRC protocol."]
    _1,
}
impl TCRCR {
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
            TCRCR::_0 => false,
            TCRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCRCR {
        match value {
            false => TCRCR::_0,
            true => TCRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCRCR::_1
    }
}
#[doc = "Possible values of the field `WAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WASR {
    #[doc = "Writes to CRC data register are data values."]
    _0,
    #[doc = "Writes to CRC data reguster are seed values."]
    _1,
}
impl WASR {
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
            WASR::_0 => false,
            WASR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WASR {
        match value {
            false => WASR::_0,
            true => WASR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WASR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WASR::_1
    }
}
#[doc = "Possible values of the field `FXOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FXORR {
    #[doc = "No XOR on reading."]
    _0,
    #[doc = "Invert or complement the read value of CRC data register."]
    _1,
}
impl FXORR {
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
            FXORR::_0 => false,
            FXORR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FXORR {
        match value {
            false => FXORR::_0,
            true => FXORR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FXORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FXORR::_1
    }
}
#[doc = "Possible values of the field `TOTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOTRR {
    #[doc = "No Transposition."]
    _00,
    #[doc = "Bits in bytes are transposed, bytes are not transposed."]
    _01,
    #[doc = "Both bits in bytes and bytes are transposed."]
    _10,
    #[doc = "Only bytes are transposed; no bits in a byte are transposed."]
    _11,
}
impl TOTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TOTRR::_00 => 0,
            TOTRR::_01 => 1,
            TOTRR::_10 => 2,
            TOTRR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TOTRR {
        match value {
            0 => TOTRR::_00,
            1 => TOTRR::_01,
            2 => TOTRR::_10,
            3 => TOTRR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TOTRR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TOTRR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TOTRR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TOTRR::_11
    }
}
#[doc = "Possible values of the field `TOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOTR {
    #[doc = "No Transposition."]
    _00,
    #[doc = "Bits in bytes are transposed, bytes are not transposed."]
    _01,
    #[doc = "Both bits in bytes and bytes are transposed."]
    _10,
    #[doc = "Only bytes are transposed; no bits in a byte are transposed."]
    _11,
}
impl TOTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TOTR::_00 => 0,
            TOTR::_01 => 1,
            TOTR::_10 => 2,
            TOTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TOTR {
        match value {
            0 => TOTR::_00,
            1 => TOTR::_01,
            2 => TOTR::_10,
            3 => TOTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TOTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TOTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TOTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TOTR::_11
    }
}
#[doc = "Values that can be written to the field `TCRC`"]
pub enum TCRCW {
    #[doc = "16-bit CRC protocol."]
    _0,
    #[doc = "32-bit CRC protocol."]
    _1,
}
impl TCRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCRCW::_0 => false,
            TCRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TCRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "16-bit CRC protocol."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCRCW::_0)
    }
    #[doc = "32-bit CRC protocol."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCRCW::_1)
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
#[doc = "Values that can be written to the field `WAS`"]
pub enum WASW {
    #[doc = "Writes to CRC data register are data values."]
    _0,
    #[doc = "Writes to CRC data reguster are seed values."]
    _1,
}
impl WASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WASW::_0 => false,
            WASW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WASW<'a> {
    w: &'a mut W,
}
impl<'a> _WASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WASW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to CRC data register are data values."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WASW::_0)
    }
    #[doc = "Writes to CRC data reguster are seed values."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WASW::_1)
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
#[doc = "Values that can be written to the field `FXOR`"]
pub enum FXORW {
    #[doc = "No XOR on reading."]
    _0,
    #[doc = "Invert or complement the read value of CRC data register."]
    _1,
}
impl FXORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FXORW::_0 => false,
            FXORW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FXORW<'a> {
    w: &'a mut W,
}
impl<'a> _FXORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FXORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No XOR on reading."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FXORW::_0)
    }
    #[doc = "Invert or complement the read value of CRC data register."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FXORW::_1)
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
#[doc = "Values that can be written to the field `TOTR`"]
pub enum TOTRW {
    #[doc = "No Transposition."]
    _00,
    #[doc = "Bits in bytes are transposed, bytes are not transposed."]
    _01,
    #[doc = "Both bits in bytes and bytes are transposed."]
    _10,
    #[doc = "Only bytes are transposed; no bits in a byte are transposed."]
    _11,
}
impl TOTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TOTRW::_00 => 0,
            TOTRW::_01 => 1,
            TOTRW::_10 => 2,
            TOTRW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOTRW<'a> {
    w: &'a mut W,
}
impl<'a> _TOTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Transposition."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TOTRW::_00)
    }
    #[doc = "Bits in bytes are transposed, bytes are not transposed."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TOTRW::_01)
    }
    #[doc = "Both bits in bytes and bytes are transposed."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TOTRW::_10)
    }
    #[doc = "Only bytes are transposed; no bits in a byte are transposed."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TOTRW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOT`"]
pub enum TOTW {
    #[doc = "No Transposition."]
    _00,
    #[doc = "Bits in bytes are transposed, bytes are not transposed."]
    _01,
    #[doc = "Both bits in bytes and bytes are transposed."]
    _10,
    #[doc = "Only bytes are transposed; no bits in a byte are transposed."]
    _11,
}
impl TOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TOTW::_00 => 0,
            TOTW::_01 => 1,
            TOTW::_10 => 2,
            TOTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOTW<'a> {
    w: &'a mut W,
}
impl<'a> _TOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Transposition."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TOTW::_00)
    }
    #[doc = "Bits in bytes are transposed, bytes are not transposed."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TOTW::_01)
    }
    #[doc = "Both bits in bytes and bytes are transposed."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TOTW::_10)
    }
    #[doc = "Only bytes are transposed; no bits in a byte are transposed."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TOTW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn tcrc(&self) -> TCRCR {
        TCRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn was(&self) -> WASR {
        WASR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - no description available"]
    #[inline]
    pub fn fxor(&self) -> FXORR {
        FXORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 4:5 - no description available"]
    #[inline]
    pub fn totr(&self) -> TOTRR {
        TOTRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 6:7 - no description available"]
    #[inline]
    pub fn tot(&self) -> TOTR {
        TOTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    #[doc = "Bit 0 - no description available"]
    #[inline]
    pub fn tcrc(&mut self) -> _TCRCW {
        _TCRCW { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline]
    pub fn was(&mut self) -> _WASW {
        _WASW { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline]
    pub fn fxor(&mut self) -> _FXORW {
        _FXORW { w: self }
    }
    #[doc = "Bits 4:5 - no description available"]
    #[inline]
    pub fn totr(&mut self) -> _TOTRW {
        _TOTRW { w: self }
    }
    #[doc = "Bits 6:7 - no description available"]
    #[inline]
    pub fn tot(&mut self) -> _TOTW {
        _TOTW { w: self }
    }
}
