#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DIV {
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
#[doc = "Possible values of the field `ERPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERPSR {
    #[doc = "The divisor ratio is 1."]
    _00,
    #[doc = "The divisor ratio is 2."]
    _01,
    #[doc = "The divisor ratio is 4."]
    _10,
    #[doc = "The divisor ratio is 8."]
    _11,
}
impl ERPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ERPSR::_00 => 0,
            ERPSR::_01 => 1,
            ERPSR::_10 => 2,
            ERPSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ERPSR {
        match value {
            0 => ERPSR::_00,
            1 => ERPSR::_01,
            2 => ERPSR::_10,
            3 => ERPSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ERPSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ERPSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ERPSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == ERPSR::_11
    }
}
#[doc = "Values that can be written to the field `ERPS`"]
pub enum ERPSW {
    #[doc = "The divisor ratio is 1."]
    _00,
    #[doc = "The divisor ratio is 2."]
    _01,
    #[doc = "The divisor ratio is 4."]
    _10,
    #[doc = "The divisor ratio is 8."]
    _11,
}
impl ERPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ERPSW::_00 => 0,
            ERPSW::_01 => 1,
            ERPSW::_10 => 2,
            ERPSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERPSW<'a> {
    w: &'a mut W,
}
impl<'a> _ERPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERPSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The divisor ratio is 1."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ERPSW::_00)
    }
    #[doc = "The divisor ratio is 2."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ERPSW::_01)
    }
    #[doc = "The divisor ratio is 4."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ERPSW::_10)
    }
    #[doc = "The divisor ratio is 8."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(ERPSW::_11)
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
    #[doc = "Bits 6:7 - ERCLK prescaler"]
    #[inline]
    pub fn erps(&self) -> ERPSR {
        ERPSR::_from({
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
    #[doc = "Bits 6:7 - ERCLK prescaler"]
    #[inline]
    pub fn erps(&mut self) -> _ERPSW {
        _ERPSW { w: self }
    }
}
