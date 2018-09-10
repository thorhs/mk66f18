#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::LVDSC2 {
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
#[doc = "Possible values of the field `LVWV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVWVR {
    #[doc = "Low trip point selected (VLVW = VLVW1)"]
    _00,
    #[doc = "Mid 1 trip point selected (VLVW = VLVW2)"]
    _01,
    #[doc = "Mid 2 trip point selected (VLVW = VLVW3)"]
    _10,
    #[doc = "High trip point selected (VLVW = VLVW4)"]
    _11,
}
impl LVWVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LVWVR::_00 => 0,
            LVWVR::_01 => 1,
            LVWVR::_10 => 2,
            LVWVR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LVWVR {
        match value {
            0 => LVWVR::_00,
            1 => LVWVR::_01,
            2 => LVWVR::_10,
            3 => LVWVR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LVWVR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LVWVR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LVWVR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == LVWVR::_11
    }
}
#[doc = "Possible values of the field `LVWIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVWIER {
    #[doc = "Hardware interrupt disabled (use polling)"]
    _0,
    #[doc = "Request a hardware interrupt when LVWF = 1"]
    _1,
}
impl LVWIER {
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
            LVWIER::_0 => false,
            LVWIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LVWIER {
        match value {
            false => LVWIER::_0,
            true => LVWIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LVWIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LVWIER::_1
    }
}
#[doc = "Possible values of the field `LVWF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVWFR {
    #[doc = "Low-voltage warning event not detected"]
    _0,
    #[doc = "Low-voltage warning event detected"]
    _1,
}
impl LVWFR {
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
            LVWFR::_0 => false,
            LVWFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LVWFR {
        match value {
            false => LVWFR::_0,
            true => LVWFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LVWFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LVWFR::_1
    }
}
#[doc = "Values that can be written to the field `LVWV`"]
pub enum LVWVW {
    #[doc = "Low trip point selected (VLVW = VLVW1)"]
    _00,
    #[doc = "Mid 1 trip point selected (VLVW = VLVW2)"]
    _01,
    #[doc = "Mid 2 trip point selected (VLVW = VLVW3)"]
    _10,
    #[doc = "High trip point selected (VLVW = VLVW4)"]
    _11,
}
impl LVWVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LVWVW::_00 => 0,
            LVWVW::_01 => 1,
            LVWVW::_10 => 2,
            LVWVW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LVWVW<'a> {
    w: &'a mut W,
}
impl<'a> _LVWVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LVWVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low trip point selected (VLVW = VLVW1)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LVWVW::_00)
    }
    #[doc = "Mid 1 trip point selected (VLVW = VLVW2)"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LVWVW::_01)
    }
    #[doc = "Mid 2 trip point selected (VLVW = VLVW3)"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LVWVW::_10)
    }
    #[doc = "High trip point selected (VLVW = VLVW4)"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(LVWVW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LVWIE`"]
pub enum LVWIEW {
    #[doc = "Hardware interrupt disabled (use polling)"]
    _0,
    #[doc = "Request a hardware interrupt when LVWF = 1"]
    _1,
}
impl LVWIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LVWIEW::_0 => false,
            LVWIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LVWIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LVWIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LVWIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVWIEW::_0)
    }
    #[doc = "Request a hardware interrupt when LVWF = 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVWIEW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LVWACKW<'a> {
    w: &'a mut W,
}
impl<'a> _LVWACKW<'a> {
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
    #[doc = "Bits 0:1 - Low-Voltage Warning Voltage Select"]
    #[inline]
    pub fn lvwv(&self) -> LVWVR {
        LVWVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline]
    pub fn lvwie(&self) -> LVWIER {
        LVWIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Low-Voltage Warning Flag"]
    #[inline]
    pub fn lvwf(&self) -> LVWFR {
        LVWFR::_from({
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
    #[doc = "Bits 0:1 - Low-Voltage Warning Voltage Select"]
    #[inline]
    pub fn lvwv(&mut self) -> _LVWVW {
        _LVWVW { w: self }
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline]
    pub fn lvwie(&mut self) -> _LVWIEW {
        _LVWIEW { w: self }
    }
    #[doc = "Bit 6 - Low-Voltage Warning Acknowledge"]
    #[inline]
    pub fn lvwack(&mut self) -> _LVWACKW {
        _LVWACKW { w: self }
    }
}
