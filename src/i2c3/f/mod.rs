#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::F {
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
pub struct ICRR {
    bits: u8,
}
impl ICRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MULT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTR {
    #[doc = "mul = 1"]
    _00,
    #[doc = "mul = 2"]
    _01,
    #[doc = "mul = 4"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MULTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MULTR::_00 => 0,
            MULTR::_01 => 1,
            MULTR::_10 => 2,
            MULTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MULTR {
        match value {
            0 => MULTR::_00,
            1 => MULTR::_01,
            2 => MULTR::_10,
            i => MULTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == MULTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == MULTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == MULTR::_10
    }
}
#[doc = r" Proxy"]
pub struct _ICRW<'a> {
    w: &'a mut W,
}
impl<'a> _ICRW<'a> {
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
#[doc = "Values that can be written to the field `MULT`"]
pub enum MULTW {
    #[doc = "mul = 1"]
    _00,
    #[doc = "mul = 2"]
    _01,
    #[doc = "mul = 4"]
    _10,
}
impl MULTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MULTW::_00 => 0,
            MULTW::_01 => 1,
            MULTW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MULTW<'a> {
    w: &'a mut W,
}
impl<'a> _MULTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MULTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "mul = 1"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(MULTW::_00)
    }
    #[doc = "mul = 2"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(MULTW::_01)
    }
    #[doc = "mul = 4"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(MULTW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:5 - ClockRate"]
    #[inline]
    pub fn icr(&self) -> ICRR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        ICRR { bits }
    }
    #[doc = "Bits 6:7 - Multiplier Factor"]
    #[inline]
    pub fn mult(&self) -> MULTR {
        MULTR::_from({
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
    #[doc = "Bits 0:5 - ClockRate"]
    #[inline]
    pub fn icr(&mut self) -> _ICRW {
        _ICRW { w: self }
    }
    #[doc = "Bits 6:7 - Multiplier Factor"]
    #[inline]
    pub fn mult(&mut self) -> _MULTW {
        _MULTW { w: self }
    }
}
