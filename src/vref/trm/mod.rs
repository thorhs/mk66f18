#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TRM {
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
#[doc = "Possible values of the field `TRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIMR {
    #[doc = "Min"]
    _000000,
    #[doc = "Max"]
    _111111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIMR::_000000 => 0,
            TRIMR::_111111 => 63,
            TRIMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIMR {
        match value {
            0 => TRIMR::_000000,
            63 => TRIMR::_111111,
            i => TRIMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline]
    pub fn is_000000(&self) -> bool {
        *self == TRIMR::_000000
    }
    #[doc = "Checks if the value of the field is `_111111`"]
    #[inline]
    pub fn is_111111(&self) -> bool {
        *self == TRIMR::_111111
    }
}
#[doc = "Possible values of the field `CHOPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHOPENR {
    #[doc = "Chop oscillator is disabled."]
    _0,
    #[doc = "Chop oscillator is enabled."]
    _1,
}
impl CHOPENR {
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
            CHOPENR::_0 => false,
            CHOPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHOPENR {
        match value {
            false => CHOPENR::_0,
            true => CHOPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CHOPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CHOPENR::_1
    }
}
#[doc = "Values that can be written to the field `TRIM`"]
pub enum TRIMW {
    #[doc = "Min"]
    _000000,
    #[doc = "Max"]
    _111111,
}
impl TRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIMW::_000000 => 0,
            TRIMW::_111111 => 63,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Min"]
    #[inline]
    pub fn _000000(self) -> &'a mut W {
        self.variant(TRIMW::_000000)
    }
    #[doc = "Max"]
    #[inline]
    pub fn _111111(self) -> &'a mut W {
        self.variant(TRIMW::_111111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHOPEN`"]
pub enum CHOPENW {
    #[doc = "Chop oscillator is disabled."]
    _0,
    #[doc = "Chop oscillator is enabled."]
    _1,
}
impl CHOPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHOPENW::_0 => false,
            CHOPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHOPENW<'a> {
    w: &'a mut W,
}
impl<'a> _CHOPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHOPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Chop oscillator is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHOPENW::_0)
    }
    #[doc = "Chop oscillator is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHOPENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:5 - Trim bits"]
    #[inline]
    pub fn trim(&self) -> TRIMR {
        TRIMR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 6 - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
    #[inline]
    pub fn chopen(&self) -> CHOPENR {
        CHOPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:5 - Trim bits"]
    #[inline]
    pub fn trim(&mut self) -> _TRIMW {
        _TRIMW { w: self }
    }
    #[doc = "Bit 6 - Chop oscillator enable. When set, internal chopping operation is enabled and the internal analog offset will be minimized."]
    #[inline]
    pub fn chopen(&mut self) -> _CHOPENW {
        _CHOPENW { w: self }
    }
}
