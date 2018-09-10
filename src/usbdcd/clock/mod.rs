#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLOCK {
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
#[doc = "Possible values of the field `CLOCK_UNIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCK_UNITR {
    #[doc = "kHz Speed (between 1 kHz and 1023 kHz)"]
    _0,
    #[doc = "MHz Speed (between 1 MHz and 1023 MHz)"]
    _1,
}
impl CLOCK_UNITR {
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
            CLOCK_UNITR::_0 => false,
            CLOCK_UNITR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLOCK_UNITR {
        match value {
            false => CLOCK_UNITR::_0,
            true => CLOCK_UNITR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CLOCK_UNITR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CLOCK_UNITR::_1
    }
}
#[doc = r" Value of the field"]
pub struct CLOCK_SPEEDR {
    bits: u16,
}
impl CLOCK_SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `CLOCK_UNIT`"]
pub enum CLOCK_UNITW {
    #[doc = "kHz Speed (between 1 kHz and 1023 kHz)"]
    _0,
    #[doc = "MHz Speed (between 1 MHz and 1023 MHz)"]
    _1,
}
impl CLOCK_UNITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLOCK_UNITW::_0 => false,
            CLOCK_UNITW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLOCK_UNITW<'a> {
    w: &'a mut W,
}
impl<'a> _CLOCK_UNITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLOCK_UNITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "kHz Speed (between 1 kHz and 1023 kHz)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLOCK_UNITW::_0)
    }
    #[doc = "MHz Speed (between 1 MHz and 1023 MHz)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLOCK_UNITW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLOCK_SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _CLOCK_SPEEDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Unit of Measurement Encoding for Clock Speed"]
    #[inline]
    pub fn clock_unit(&self) -> CLOCK_UNITR {
        CLOCK_UNITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:11 - Numerical Value of Clock Speed in Binary"]
    #[inline]
    pub fn clock_speed(&self) -> CLOCK_SPEEDR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CLOCK_SPEEDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 193 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Unit of Measurement Encoding for Clock Speed"]
    #[inline]
    pub fn clock_unit(&mut self) -> _CLOCK_UNITW {
        _CLOCK_UNITW { w: self }
    }
    #[doc = "Bits 2:11 - Numerical Value of Clock Speed in Binary"]
    #[inline]
    pub fn clock_speed(&mut self) -> _CLOCK_SPEEDW {
        _CLOCK_SPEEDW { w: self }
    }
}
