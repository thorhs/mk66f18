#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MMCBOOT {
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
#[doc = "Possible values of the field `DTOCVACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOCVACKR {
    #[doc = "SDCLK x 2^8"]
    _0000,
    #[doc = "SDCLK x 2^9"]
    _0001,
    #[doc = "SDCLK x 2^10"]
    _0010,
    #[doc = "SDCLK x 2^11"]
    _0011,
    #[doc = "SDCLK x 2^12"]
    _0100,
    #[doc = "SDCLK x 2^13"]
    _0101,
    #[doc = "SDCLK x 2^14"]
    _0110,
    #[doc = "SDCLK x 2^15"]
    _0111,
    #[doc = "SDCLK x 2^22"]
    _1110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTOCVACKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTOCVACKR::_0000 => 0,
            DTOCVACKR::_0001 => 1,
            DTOCVACKR::_0010 => 2,
            DTOCVACKR::_0011 => 3,
            DTOCVACKR::_0100 => 4,
            DTOCVACKR::_0101 => 5,
            DTOCVACKR::_0110 => 6,
            DTOCVACKR::_0111 => 7,
            DTOCVACKR::_1110 => 14,
            DTOCVACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTOCVACKR {
        match value {
            0 => DTOCVACKR::_0000,
            1 => DTOCVACKR::_0001,
            2 => DTOCVACKR::_0010,
            3 => DTOCVACKR::_0011,
            4 => DTOCVACKR::_0100,
            5 => DTOCVACKR::_0101,
            6 => DTOCVACKR::_0110,
            7 => DTOCVACKR::_0111,
            14 => DTOCVACKR::_1110,
            i => DTOCVACKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == DTOCVACKR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == DTOCVACKR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == DTOCVACKR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == DTOCVACKR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == DTOCVACKR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == DTOCVACKR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == DTOCVACKR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == DTOCVACKR::_0111
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == DTOCVACKR::_1110
    }
}
#[doc = "Possible values of the field `BOOTACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTACKR {
    #[doc = "No ack."]
    _0,
    #[doc = "Ack."]
    _1,
}
impl BOOTACKR {
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
            BOOTACKR::_0 => false,
            BOOTACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOOTACKR {
        match value {
            false => BOOTACKR::_0,
            true => BOOTACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOOTACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BOOTACKR::_1
    }
}
#[doc = "Possible values of the field `BOOTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTMODER {
    #[doc = "Normal boot."]
    _0,
    #[doc = "Alternative boot."]
    _1,
}
impl BOOTMODER {
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
            BOOTMODER::_0 => false,
            BOOTMODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOOTMODER {
        match value {
            false => BOOTMODER::_0,
            true => BOOTMODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOOTMODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BOOTMODER::_1
    }
}
#[doc = "Possible values of the field `BOOTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOTENR {
    #[doc = "Fast boot disable."]
    _0,
    #[doc = "Fast boot enable."]
    _1,
}
impl BOOTENR {
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
            BOOTENR::_0 => false,
            BOOTENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOOTENR {
        match value {
            false => BOOTENR::_0,
            true => BOOTENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOOTENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BOOTENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct AUTOSABGENR {
    bits: bool,
}
impl AUTOSABGENR {
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
pub struct BOOTBLKCNTR {
    bits: u16,
}
impl BOOTBLKCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `DTOCVACK`"]
pub enum DTOCVACKW {
    #[doc = "SDCLK x 2^8"]
    _0000,
    #[doc = "SDCLK x 2^9"]
    _0001,
    #[doc = "SDCLK x 2^10"]
    _0010,
    #[doc = "SDCLK x 2^11"]
    _0011,
    #[doc = "SDCLK x 2^12"]
    _0100,
    #[doc = "SDCLK x 2^13"]
    _0101,
    #[doc = "SDCLK x 2^14"]
    _0110,
    #[doc = "SDCLK x 2^15"]
    _0111,
    #[doc = "SDCLK x 2^22"]
    _1110,
}
impl DTOCVACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTOCVACKW::_0000 => 0,
            DTOCVACKW::_0001 => 1,
            DTOCVACKW::_0010 => 2,
            DTOCVACKW::_0011 => 3,
            DTOCVACKW::_0100 => 4,
            DTOCVACKW::_0101 => 5,
            DTOCVACKW::_0110 => 6,
            DTOCVACKW::_0111 => 7,
            DTOCVACKW::_1110 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTOCVACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOCVACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTOCVACKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SDCLK x 2^8"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DTOCVACKW::_0000)
    }
    #[doc = "SDCLK x 2^9"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DTOCVACKW::_0001)
    }
    #[doc = "SDCLK x 2^10"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DTOCVACKW::_0010)
    }
    #[doc = "SDCLK x 2^11"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DTOCVACKW::_0011)
    }
    #[doc = "SDCLK x 2^12"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DTOCVACKW::_0100)
    }
    #[doc = "SDCLK x 2^13"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DTOCVACKW::_0101)
    }
    #[doc = "SDCLK x 2^14"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DTOCVACKW::_0110)
    }
    #[doc = "SDCLK x 2^15"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DTOCVACKW::_0111)
    }
    #[doc = "SDCLK x 2^22"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DTOCVACKW::_1110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOOTACK`"]
pub enum BOOTACKW {
    #[doc = "No ack."]
    _0,
    #[doc = "Ack."]
    _1,
}
impl BOOTACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOOTACKW::_0 => false,
            BOOTACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOTACKW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOTACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOTACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No ack."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOOTACKW::_0)
    }
    #[doc = "Ack."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOOTACKW::_1)
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
#[doc = "Values that can be written to the field `BOOTMODE`"]
pub enum BOOTMODEW {
    #[doc = "Normal boot."]
    _0,
    #[doc = "Alternative boot."]
    _1,
}
impl BOOTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOOTMODEW::_0 => false,
            BOOTMODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOTMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOTMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal boot."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOOTMODEW::_0)
    }
    #[doc = "Alternative boot."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOOTMODEW::_1)
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
#[doc = "Values that can be written to the field `BOOTEN`"]
pub enum BOOTENW {
    #[doc = "Fast boot disable."]
    _0,
    #[doc = "Fast boot enable."]
    _1,
}
impl BOOTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOOTENW::_0 => false,
            BOOTENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOOTENW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOOTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fast boot disable."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOOTENW::_0)
    }
    #[doc = "Fast boot enable."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOOTENW::_1)
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
#[doc = r" Proxy"]
pub struct _AUTOSABGENW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOSABGENW<'a> {
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
#[doc = r" Proxy"]
pub struct _BOOTBLKCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOTBLKCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:3 - Boot ACK Time Out Counter Value"]
    #[inline]
    pub fn dtocvack(&self) -> DTOCVACKR {
        DTOCVACKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Boot Ack Mode Select"]
    #[inline]
    pub fn bootack(&self) -> BOOTACKR {
        BOOTACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Boot Mode Select"]
    #[inline]
    pub fn bootmode(&self) -> BOOTMODER {
        BOOTMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Boot Mode Enable"]
    #[inline]
    pub fn booten(&self) -> BOOTENR {
        BOOTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - When boot, enable auto stop at block gap function"]
    #[inline]
    pub fn autosabgen(&self) -> AUTOSABGENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOSABGENR { bits }
    }
    #[doc = "Bits 16:31 - Defines the stop at block gap value of automatic mode"]
    #[inline]
    pub fn bootblkcnt(&self) -> BOOTBLKCNTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BOOTBLKCNTR { bits }
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
    #[doc = "Bits 0:3 - Boot ACK Time Out Counter Value"]
    #[inline]
    pub fn dtocvack(&mut self) -> _DTOCVACKW {
        _DTOCVACKW { w: self }
    }
    #[doc = "Bit 4 - Boot Ack Mode Select"]
    #[inline]
    pub fn bootack(&mut self) -> _BOOTACKW {
        _BOOTACKW { w: self }
    }
    #[doc = "Bit 5 - Boot Mode Select"]
    #[inline]
    pub fn bootmode(&mut self) -> _BOOTMODEW {
        _BOOTMODEW { w: self }
    }
    #[doc = "Bit 6 - Boot Mode Enable"]
    #[inline]
    pub fn booten(&mut self) -> _BOOTENW {
        _BOOTENW { w: self }
    }
    #[doc = "Bit 7 - When boot, enable auto stop at block gap function"]
    #[inline]
    pub fn autosabgen(&mut self) -> _AUTOSABGENW {
        _AUTOSABGENW { w: self }
    }
    #[doc = "Bits 16:31 - Defines the stop at block gap value of automatic mode"]
    #[inline]
    pub fn bootblkcnt(&mut self) -> _BOOTBLKCNTW {
        _BOOTBLKCNTW { w: self }
    }
}
