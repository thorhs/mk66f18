#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DATA {
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
pub struct TSICNTR {
    bits: u16,
}
impl TSICNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "Interrupt is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    _0,
    #[doc = "DMA transfer request is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    _1,
}
impl DMAENR {
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
            DMAENR::_0 => false,
            DMAENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAENR {
        match value {
            false => DMAENR::_0,
            true => DMAENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAENR::_1
    }
}
#[doc = "Possible values of the field `TSICH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSICHR {
    #[doc = "Channel 0."]
    _0000,
    #[doc = "Channel 1."]
    _0001,
    #[doc = "Channel 2."]
    _0010,
    #[doc = "Channel 3."]
    _0011,
    #[doc = "Channel 4."]
    _0100,
    #[doc = "Channel 5."]
    _0101,
    #[doc = "Channel 6."]
    _0110,
    #[doc = "Channel 7."]
    _0111,
    #[doc = "Channel 8."]
    _1000,
    #[doc = "Channel 9."]
    _1001,
    #[doc = "Channel 10."]
    _1010,
    #[doc = "Channel 11."]
    _1011,
    #[doc = "Channel 12."]
    _1100,
    #[doc = "Channel 13."]
    _1101,
    #[doc = "Channel 14."]
    _1110,
    #[doc = "Channel 15."]
    _1111,
}
impl TSICHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSICHR::_0000 => 0,
            TSICHR::_0001 => 1,
            TSICHR::_0010 => 2,
            TSICHR::_0011 => 3,
            TSICHR::_0100 => 4,
            TSICHR::_0101 => 5,
            TSICHR::_0110 => 6,
            TSICHR::_0111 => 7,
            TSICHR::_1000 => 8,
            TSICHR::_1001 => 9,
            TSICHR::_1010 => 10,
            TSICHR::_1011 => 11,
            TSICHR::_1100 => 12,
            TSICHR::_1101 => 13,
            TSICHR::_1110 => 14,
            TSICHR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSICHR {
        match value {
            0 => TSICHR::_0000,
            1 => TSICHR::_0001,
            2 => TSICHR::_0010,
            3 => TSICHR::_0011,
            4 => TSICHR::_0100,
            5 => TSICHR::_0101,
            6 => TSICHR::_0110,
            7 => TSICHR::_0111,
            8 => TSICHR::_1000,
            9 => TSICHR::_1001,
            10 => TSICHR::_1010,
            11 => TSICHR::_1011,
            12 => TSICHR::_1100,
            13 => TSICHR::_1101,
            14 => TSICHR::_1110,
            15 => TSICHR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == TSICHR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == TSICHR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == TSICHR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == TSICHR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == TSICHR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == TSICHR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == TSICHR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == TSICHR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == TSICHR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == TSICHR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == TSICHR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == TSICHR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == TSICHR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == TSICHR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == TSICHR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == TSICHR::_1111
    }
}
#[doc = "Values that can be written to the field `SWTS`"]
pub enum SWTSW {
    #[doc = "No effect."]
    _0,
    #[doc = "Start a scan to determine which channel is specified by TSI_DATA\\[TSICH\\]."]
    _1,
}
impl SWTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWTSW::_0 => false,
            SWTSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWTSW<'a> {
    w: &'a mut W,
}
impl<'a> _SWTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWTSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWTSW::_0)
    }
    #[doc = "Start a scan to determine which channel is specified by TSI_DATA\\[TSICH\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWTSW::_1)
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
#[doc = "Values that can be written to the field `DMAEN`"]
pub enum DMAENW {
    #[doc = "Interrupt is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    _0,
    #[doc = "DMA transfer request is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    _1,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENW::_0 => false,
            DMAENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAENW::_0)
    }
    #[doc = "DMA transfer request is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAENW::_1)
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
#[doc = "Values that can be written to the field `TSICH`"]
pub enum TSICHW {
    #[doc = "Channel 0."]
    _0000,
    #[doc = "Channel 1."]
    _0001,
    #[doc = "Channel 2."]
    _0010,
    #[doc = "Channel 3."]
    _0011,
    #[doc = "Channel 4."]
    _0100,
    #[doc = "Channel 5."]
    _0101,
    #[doc = "Channel 6."]
    _0110,
    #[doc = "Channel 7."]
    _0111,
    #[doc = "Channel 8."]
    _1000,
    #[doc = "Channel 9."]
    _1001,
    #[doc = "Channel 10."]
    _1010,
    #[doc = "Channel 11."]
    _1011,
    #[doc = "Channel 12."]
    _1100,
    #[doc = "Channel 13."]
    _1101,
    #[doc = "Channel 14."]
    _1110,
    #[doc = "Channel 15."]
    _1111,
}
impl TSICHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSICHW::_0000 => 0,
            TSICHW::_0001 => 1,
            TSICHW::_0010 => 2,
            TSICHW::_0011 => 3,
            TSICHW::_0100 => 4,
            TSICHW::_0101 => 5,
            TSICHW::_0110 => 6,
            TSICHW::_0111 => 7,
            TSICHW::_1000 => 8,
            TSICHW::_1001 => 9,
            TSICHW::_1010 => 10,
            TSICHW::_1011 => 11,
            TSICHW::_1100 => 12,
            TSICHW::_1101 => 13,
            TSICHW::_1110 => 14,
            TSICHW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSICHW<'a> {
    w: &'a mut W,
}
impl<'a> _TSICHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSICHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Channel 0."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TSICHW::_0000)
    }
    #[doc = "Channel 1."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TSICHW::_0001)
    }
    #[doc = "Channel 2."]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TSICHW::_0010)
    }
    #[doc = "Channel 3."]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TSICHW::_0011)
    }
    #[doc = "Channel 4."]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(TSICHW::_0100)
    }
    #[doc = "Channel 5."]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(TSICHW::_0101)
    }
    #[doc = "Channel 6."]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(TSICHW::_0110)
    }
    #[doc = "Channel 7."]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(TSICHW::_0111)
    }
    #[doc = "Channel 8."]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(TSICHW::_1000)
    }
    #[doc = "Channel 9."]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(TSICHW::_1001)
    }
    #[doc = "Channel 10."]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(TSICHW::_1010)
    }
    #[doc = "Channel 11."]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(TSICHW::_1011)
    }
    #[doc = "Channel 12."]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(TSICHW::_1100)
    }
    #[doc = "Channel 13."]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(TSICHW::_1101)
    }
    #[doc = "Channel 14."]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(TSICHW::_1110)
    }
    #[doc = "Channel 15."]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TSICHW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:15 - TSI Conversion Counter Value"]
    #[inline]
    pub fn tsicnt(&self) -> TSICNTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TSICNTR { bits }
    }
    #[doc = "Bit 23 - DMA Transfer Enabled"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:31 - TSICH"]
    #[inline]
    pub fn tsich(&self) -> TSICHR {
        TSICHR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 22 - Software Trigger Start"]
    #[inline]
    pub fn swts(&mut self) -> _SWTSW {
        _SWTSW { w: self }
    }
    #[doc = "Bit 23 - DMA Transfer Enabled"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bits 28:31 - TSICH"]
    #[inline]
    pub fn tsich(&mut self) -> _TSICHW {
        _TSICHW { w: self }
    }
}
