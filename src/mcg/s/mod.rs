#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::S {
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
#[doc = "Possible values of the field `IRCST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRCSTR {
    #[doc = "Source of internal reference clock is the slow clock (32 kHz IRC)."]
    _0,
    #[doc = "Source of internal reference clock is the fast clock (4 MHz IRC)."]
    _1,
}
impl IRCSTR {
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
            IRCSTR::_0 => false,
            IRCSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRCSTR {
        match value {
            false => IRCSTR::_0,
            true => IRCSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IRCSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IRCSTR::_1
    }
}
#[doc = r" Value of the field"]
pub struct OSCINIT0R {
    bits: bool,
}
impl OSCINIT0R {
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
#[doc = "Possible values of the field `CLKST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSTR {
    #[doc = "Encoding 0 - Output of the FLL is selected (reset default)."]
    _00,
    #[doc = "Encoding 1 - Internal reference clock is selected."]
    _01,
    #[doc = "Encoding 2 - External reference clock is selected."]
    _10,
    #[doc = "Encoding 3 - Output of the PLL is selected."]
    _11,
}
impl CLKSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSTR::_00 => 0,
            CLKSTR::_01 => 1,
            CLKSTR::_10 => 2,
            CLKSTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSTR {
        match value {
            0 => CLKSTR::_00,
            1 => CLKSTR::_01,
            2 => CLKSTR::_10,
            3 => CLKSTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == CLKSTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == CLKSTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == CLKSTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == CLKSTR::_11
    }
}
#[doc = "Possible values of the field `IREFST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREFSTR {
    #[doc = "Source of FLL reference clock is the external reference clock."]
    _0,
    #[doc = "Source of FLL reference clock is the internal reference clock."]
    _1,
}
impl IREFSTR {
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
            IREFSTR::_0 => false,
            IREFSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IREFSTR {
        match value {
            false => IREFSTR::_0,
            true => IREFSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IREFSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IREFSTR::_1
    }
}
#[doc = "Possible values of the field `PLLST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSTR {
    #[doc = "Source of PLLS clock is FLL clock."]
    _0,
    #[doc = "Source of PLLS clock is PLLCS output clock."]
    _1,
}
impl PLLSTR {
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
            PLLSTR::_0 => false,
            PLLSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLSTR {
        match value {
            false => PLLSTR::_0,
            true => PLLSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLLSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLLSTR::_1
    }
}
#[doc = "Possible values of the field `LOCK0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK0R {
    #[doc = "PLL is currently unlocked."]
    _0,
    #[doc = "PLL is currently locked."]
    _1,
}
impl LOCK0R {
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
            LOCK0R::_0 => false,
            LOCK0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCK0R {
        match value {
            false => LOCK0R::_0,
            true => LOCK0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCK0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCK0R::_1
    }
}
#[doc = "Possible values of the field `LOLS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLS0R {
    #[doc = "PLL has not lost lock since LOLS 0 was last cleared."]
    _0,
    #[doc = "PLL has lost lock since LOLS 0 was last cleared."]
    _1,
}
impl LOLS0R {
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
            LOLS0R::_0 => false,
            LOLS0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOLS0R {
        match value {
            false => LOLS0R::_0,
            true => LOLS0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOLS0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOLS0R::_1
    }
}
#[doc = "Values that can be written to the field `LOLS0`"]
pub enum LOLS0W {
    #[doc = "PLL has not lost lock since LOLS 0 was last cleared."]
    _0,
    #[doc = "PLL has lost lock since LOLS 0 was last cleared."]
    _1,
}
impl LOLS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOLS0W::_0 => false,
            LOLS0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOLS0W<'a> {
    w: &'a mut W,
}
impl<'a> _LOLS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOLS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL has not lost lock since LOLS 0 was last cleared."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLS0W::_0)
    }
    #[doc = "PLL has lost lock since LOLS 0 was last cleared."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLS0W::_1)
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
    #[doc = "Bit 0 - Internal Reference Clock Status"]
    #[inline]
    pub fn ircst(&self) -> IRCSTR {
        IRCSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - OSC Initialization"]
    #[inline]
    pub fn oscinit0(&self) -> OSCINIT0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        OSCINIT0R { bits }
    }
    #[doc = "Bits 2:3 - Clock Mode Status"]
    #[inline]
    pub fn clkst(&self) -> CLKSTR {
        CLKSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 4 - Internal Reference Status"]
    #[inline]
    pub fn irefst(&self) -> IREFSTR {
        IREFSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - PLL Select Status"]
    #[inline]
    pub fn pllst(&self) -> PLLSTR {
        PLLSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Lock Status"]
    #[inline]
    pub fn lock0(&self) -> LOCK0R {
        LOCK0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Loss of Lock Status"]
    #[inline]
    pub fn lols0(&self) -> LOLS0R {
        LOLS0R::_from({
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
        W { bits: 16 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - Loss of Lock Status"]
    #[inline]
    pub fn lols0(&mut self) -> _LOLS0W {
        _LOLS0W { w: self }
    }
}
