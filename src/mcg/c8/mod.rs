#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C8 {
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
#[doc = "Possible values of the field `LOCS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCS1R {
    #[doc = "Loss of RTC has not occur."]
    _0,
    #[doc = "Loss of RTC has occur"]
    _1,
}
impl LOCS1R {
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
            LOCS1R::_0 => false,
            LOCS1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCS1R {
        match value {
            false => LOCS1R::_0,
            true => LOCS1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCS1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCS1R::_1
    }
}
#[doc = "Possible values of the field `CME1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CME1R {
    #[doc = "External clock monitor is disabled for RTC clock."]
    _0,
    #[doc = "External clock monitor is enabled for RTC clock."]
    _1,
}
impl CME1R {
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
            CME1R::_0 => false,
            CME1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CME1R {
        match value {
            false => CME1R::_0,
            true => CME1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CME1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CME1R::_1
    }
}
#[doc = "Possible values of the field `LOLRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLRER {
    #[doc = "Interrupt request is generated on a PLL loss of lock indication. The PLL loss of lock interrupt enable bit must also be set to generate the interrupt request."]
    _0,
    #[doc = "Generate a reset request on a PLL loss of lock indication."]
    _1,
}
impl LOLRER {
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
            LOLRER::_0 => false,
            LOLRER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOLRER {
        match value {
            false => LOLRER::_0,
            true => LOLRER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOLRER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOLRER::_1
    }
}
#[doc = "Possible values of the field `LOCRE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCRE1R {
    #[doc = "Interrupt request is generated on a loss of RTC external reference clock."]
    _0,
    #[doc = "Generate a reset request on a loss of RTC external reference clock"]
    _1,
}
impl LOCRE1R {
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
            LOCRE1R::_0 => false,
            LOCRE1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCRE1R {
        match value {
            false => LOCRE1R::_0,
            true => LOCRE1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCRE1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCRE1R::_1
    }
}
#[doc = "Values that can be written to the field `LOCS1`"]
pub enum LOCS1W {
    #[doc = "Loss of RTC has not occur."]
    _0,
    #[doc = "Loss of RTC has occur"]
    _1,
}
impl LOCS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCS1W::_0 => false,
            LOCS1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCS1W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Loss of RTC has not occur."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCS1W::_0)
    }
    #[doc = "Loss of RTC has occur"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCS1W::_1)
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
#[doc = "Values that can be written to the field `CME1`"]
pub enum CME1W {
    #[doc = "External clock monitor is disabled for RTC clock."]
    _0,
    #[doc = "External clock monitor is enabled for RTC clock."]
    _1,
}
impl CME1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CME1W::_0 => false,
            CME1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CME1W<'a> {
    w: &'a mut W,
}
impl<'a> _CME1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CME1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External clock monitor is disabled for RTC clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CME1W::_0)
    }
    #[doc = "External clock monitor is enabled for RTC clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CME1W::_1)
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
#[doc = "Values that can be written to the field `LOLRE`"]
pub enum LOLREW {
    #[doc = "Interrupt request is generated on a PLL loss of lock indication. The PLL loss of lock interrupt enable bit must also be set to generate the interrupt request."]
    _0,
    #[doc = "Generate a reset request on a PLL loss of lock indication."]
    _1,
}
impl LOLREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOLREW::_0 => false,
            LOLREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOLREW<'a> {
    w: &'a mut W,
}
impl<'a> _LOLREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOLREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request is generated on a PLL loss of lock indication. The PLL loss of lock interrupt enable bit must also be set to generate the interrupt request."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLREW::_0)
    }
    #[doc = "Generate a reset request on a PLL loss of lock indication."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLREW::_1)
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
#[doc = "Values that can be written to the field `LOCRE1`"]
pub enum LOCRE1W {
    #[doc = "Interrupt request is generated on a loss of RTC external reference clock."]
    _0,
    #[doc = "Generate a reset request on a loss of RTC external reference clock"]
    _1,
}
impl LOCRE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCRE1W::_0 => false,
            LOCRE1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCRE1W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCRE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCRE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request is generated on a loss of RTC external reference clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCRE1W::_0)
    }
    #[doc = "Generate a reset request on a loss of RTC external reference clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCRE1W::_1)
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
    #[doc = "Bit 0 - RTC Loss of Clock Status"]
    #[inline]
    pub fn locs1(&self) -> LOCS1R {
        LOCS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Clock Monitor Enable1"]
    #[inline]
    pub fn cme1(&self) -> CME1R {
        CME1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - PLL Loss of Lock Reset Enable"]
    #[inline]
    pub fn lolre(&self) -> LOLRER {
        LOLRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline]
    pub fn locre1(&self) -> LOCRE1R {
        LOCRE1R::_from({
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
        W { bits: 128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RTC Loss of Clock Status"]
    #[inline]
    pub fn locs1(&mut self) -> _LOCS1W {
        _LOCS1W { w: self }
    }
    #[doc = "Bit 5 - Clock Monitor Enable1"]
    #[inline]
    pub fn cme1(&mut self) -> _CME1W {
        _CME1W { w: self }
    }
    #[doc = "Bit 6 - PLL Loss of Lock Reset Enable"]
    #[inline]
    pub fn lolre(&mut self) -> _LOLREW {
        _LOLREW { w: self }
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline]
    pub fn locre1(&mut self) -> _LOCRE1W {
        _LOCRE1W { w: self }
    }
}
