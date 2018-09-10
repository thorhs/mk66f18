#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::FEPROT {
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
#[doc = "Possible values of the field `EPROT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPROTR {
    #[doc = "For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is protected"]
    _0,
    #[doc = "For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is not protected"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EPROTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPROTR::_0 => 0,
            EPROTR::_1 => 1,
            EPROTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPROTR {
        match value {
            0 => EPROTR::_0,
            1 => EPROTR::_1,
            i => EPROTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EPROTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EPROTR::_1
    }
}
#[doc = "Values that can be written to the field `EPROT`"]
pub enum EPROTW {
    #[doc = "For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is protected"]
    _0,
    #[doc = "For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is not protected"]
    _1,
}
impl EPROTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPROTW::_0 => 0,
            EPROTW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPROTW<'a> {
    w: &'a mut W,
}
impl<'a> _EPROTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPROTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is protected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPROTW::_0)
    }
    #[doc = "For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is not protected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPROTW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - EEPROM Region Protect"]
    #[inline]
    pub fn eprot(&self) -> EPROTR {
        EPROTR::_from({
            const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - EEPROM Region Protect"]
    #[inline]
    pub fn eprot(&mut self) -> _EPROTW {
        _EPROTW { w: self }
    }
}
