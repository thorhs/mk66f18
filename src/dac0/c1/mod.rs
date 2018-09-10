#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C1 {
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
#[doc = "Possible values of the field `DACBFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFENR {
    #[doc = "Buffer read pointer is disabled. The converted data is always the first word of the buffer."]
    _0,
    #[doc = "Buffer read pointer is enabled. The converted data is the word that the read pointer points to. It means converted data can be from any word of the buffer."]
    _1,
}
impl DACBFENR {
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
            DACBFENR::_0 => false,
            DACBFENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACBFENR {
        match value {
            false => DACBFENR::_0,
            true => DACBFENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DACBFENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DACBFENR::_1
    }
}
#[doc = "Possible values of the field `DACBFMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFMDR {
    #[doc = "Normal mode"]
    _00,
    #[doc = "Swing mode"]
    _01,
    #[doc = "One-Time Scan mode"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DACBFMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DACBFMDR::_00 => 0,
            DACBFMDR::_01 => 1,
            DACBFMDR::_10 => 2,
            DACBFMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DACBFMDR {
        match value {
            0 => DACBFMDR::_00,
            1 => DACBFMDR::_01,
            2 => DACBFMDR::_10,
            i => DACBFMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DACBFMDR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DACBFMDR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DACBFMDR::_10
    }
}
#[doc = "Possible values of the field `DACBFWM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFWMR {
    #[doc = "1 word"]
    _00,
    #[doc = "2 words"]
    _01,
    #[doc = "3 words"]
    _10,
    #[doc = "4 words"]
    _11,
}
impl DACBFWMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DACBFWMR::_00 => 0,
            DACBFWMR::_01 => 1,
            DACBFWMR::_10 => 2,
            DACBFWMR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DACBFWMR {
        match value {
            0 => DACBFWMR::_00,
            1 => DACBFWMR::_01,
            2 => DACBFWMR::_10,
            3 => DACBFWMR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DACBFWMR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DACBFWMR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DACBFWMR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DACBFWMR::_11
    }
}
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "DMA is disabled."]
    _0,
    #[doc = "DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time."]
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
#[doc = "Values that can be written to the field `DACBFEN`"]
pub enum DACBFENW {
    #[doc = "Buffer read pointer is disabled. The converted data is always the first word of the buffer."]
    _0,
    #[doc = "Buffer read pointer is enabled. The converted data is the word that the read pointer points to. It means converted data can be from any word of the buffer."]
    _1,
}
impl DACBFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACBFENW::_0 => false,
            DACBFENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACBFENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACBFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACBFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Buffer read pointer is disabled. The converted data is always the first word of the buffer."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFENW::_0)
    }
    #[doc = "Buffer read pointer is enabled. The converted data is the word that the read pointer points to. It means converted data can be from any word of the buffer."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFENW::_1)
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
#[doc = "Values that can be written to the field `DACBFMD`"]
pub enum DACBFMDW {
    #[doc = "Normal mode"]
    _00,
    #[doc = "Swing mode"]
    _01,
    #[doc = "One-Time Scan mode"]
    _10,
}
impl DACBFMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DACBFMDW::_00 => 0,
            DACBFMDW::_01 => 1,
            DACBFMDW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACBFMDW<'a> {
    w: &'a mut W,
}
impl<'a> _DACBFMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACBFMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DACBFMDW::_00)
    }
    #[doc = "Swing mode"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DACBFMDW::_01)
    }
    #[doc = "One-Time Scan mode"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DACBFMDW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DACBFWM`"]
pub enum DACBFWMW {
    #[doc = "1 word"]
    _00,
    #[doc = "2 words"]
    _01,
    #[doc = "3 words"]
    _10,
    #[doc = "4 words"]
    _11,
}
impl DACBFWMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DACBFWMW::_00 => 0,
            DACBFWMW::_01 => 1,
            DACBFWMW::_10 => 2,
            DACBFWMW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACBFWMW<'a> {
    w: &'a mut W,
}
impl<'a> _DACBFWMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACBFWMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 word"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DACBFWMW::_00)
    }
    #[doc = "2 words"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DACBFWMW::_01)
    }
    #[doc = "3 words"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DACBFWMW::_10)
    }
    #[doc = "4 words"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DACBFWMW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAEN`"]
pub enum DMAENW {
    #[doc = "DMA is disabled."]
    _0,
    #[doc = "DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time."]
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
    #[doc = "DMA is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAENW::_0)
    }
    #[doc = "DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time."]
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
    #[doc = "Bit 0 - DAC Buffer Enable"]
    #[inline]
    pub fn dacbfen(&self) -> DACBFENR {
        DACBFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bits 1:2 - DAC Buffer Work Mode Select"]
    #[inline]
    pub fn dacbfmd(&self) -> DACBFMDR {
        DACBFMDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 3:4 - DAC Buffer Watermark Select"]
    #[inline]
    pub fn dacbfwm(&self) -> DACBFWMR {
        DACBFWMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 7 - DMA Enable Select"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
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
    #[doc = "Bit 0 - DAC Buffer Enable"]
    #[inline]
    pub fn dacbfen(&mut self) -> _DACBFENW {
        _DACBFENW { w: self }
    }
    #[doc = "Bits 1:2 - DAC Buffer Work Mode Select"]
    #[inline]
    pub fn dacbfmd(&mut self) -> _DACBFMDW {
        _DACBFMDW { w: self }
    }
    #[doc = "Bits 3:4 - DAC Buffer Watermark Select"]
    #[inline]
    pub fn dacbfwm(&mut self) -> _DACBFWMW {
        _DACBFWMW { w: self }
    }
    #[doc = "Bit 7 - DMA Enable Select"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
}
