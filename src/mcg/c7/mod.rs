#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C7 {
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
#[doc = "Possible values of the field `OSCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSELR {
    #[doc = "Selects Oscillator (OSCCLK0)."]
    _00,
    #[doc = "Selects 32 kHz RTC Oscillator."]
    _01,
    #[doc = "Selects Oscillator (OSCCLK1)."]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSCSELR::_00 => 0,
            OSCSELR::_01 => 1,
            OSCSELR::_10 => 2,
            OSCSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSCSELR {
        match value {
            0 => OSCSELR::_00,
            1 => OSCSELR::_01,
            2 => OSCSELR::_10,
            i => OSCSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == OSCSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == OSCSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == OSCSELR::_10
    }
}
#[doc = "Values that can be written to the field `OSCSEL`"]
pub enum OSCSELW {
    #[doc = "Selects Oscillator (OSCCLK0)."]
    _00,
    #[doc = "Selects 32 kHz RTC Oscillator."]
    _01,
    #[doc = "Selects Oscillator (OSCCLK1)."]
    _10,
}
impl OSCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSCSELW::_00 => 0,
            OSCSELW::_01 => 1,
            OSCSELW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selects Oscillator (OSCCLK0)."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSCSELW::_00)
    }
    #[doc = "Selects 32 kHz RTC Oscillator."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(OSCSELW::_01)
    }
    #[doc = "Selects Oscillator (OSCCLK1)."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSCSELW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - MCG OSC Clock Select"]
    #[inline]
    pub fn oscsel(&self) -> OSCSELR {
        OSCSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - MCG OSC Clock Select"]
    #[inline]
    pub fn oscsel(&mut self) -> _OSCSELW {
        _OSCSELW { w: self }
    }
}
