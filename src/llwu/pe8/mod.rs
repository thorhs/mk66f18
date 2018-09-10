#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PE8 {
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
#[doc = "Possible values of the field `WUPE28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE28R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE28R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE28R::_00 => 0,
            WUPE28R::_01 => 1,
            WUPE28R::_10 => 2,
            WUPE28R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE28R {
        match value {
            0 => WUPE28R::_00,
            1 => WUPE28R::_01,
            2 => WUPE28R::_10,
            3 => WUPE28R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE28R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE28R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE28R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE28R::_11
    }
}
#[doc = "Possible values of the field `WUPE29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE29R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE29R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE29R::_00 => 0,
            WUPE29R::_01 => 1,
            WUPE29R::_10 => 2,
            WUPE29R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE29R {
        match value {
            0 => WUPE29R::_00,
            1 => WUPE29R::_01,
            2 => WUPE29R::_10,
            3 => WUPE29R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE29R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE29R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE29R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE29R::_11
    }
}
#[doc = "Possible values of the field `WUPE30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE30R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE30R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE30R::_00 => 0,
            WUPE30R::_01 => 1,
            WUPE30R::_10 => 2,
            WUPE30R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE30R {
        match value {
            0 => WUPE30R::_00,
            1 => WUPE30R::_01,
            2 => WUPE30R::_10,
            3 => WUPE30R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE30R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE30R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE30R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE30R::_11
    }
}
#[doc = "Possible values of the field `WUPE31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE31R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE31R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE31R::_00 => 0,
            WUPE31R::_01 => 1,
            WUPE31R::_10 => 2,
            WUPE31R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE31R {
        match value {
            0 => WUPE31R::_00,
            1 => WUPE31R::_01,
            2 => WUPE31R::_10,
            3 => WUPE31R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE31R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE31R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE31R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE31R::_11
    }
}
#[doc = "Values that can be written to the field `WUPE28`"]
pub enum WUPE28W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE28W::_00 => 0,
            WUPE28W::_01 => 1,
            WUPE28W::_10 => 2,
            WUPE28W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE28W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE28W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE28W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE28W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE28W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE28W::_11)
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
#[doc = "Values that can be written to the field `WUPE29`"]
pub enum WUPE29W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE29W::_00 => 0,
            WUPE29W::_01 => 1,
            WUPE29W::_10 => 2,
            WUPE29W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE29W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE29W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE29W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE29W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE29W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE29W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WUPE30`"]
pub enum WUPE30W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE30W::_00 => 0,
            WUPE30W::_01 => 1,
            WUPE30W::_10 => 2,
            WUPE30W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE30W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE30W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE30W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE30W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE30W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE30W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WUPE31`"]
pub enum WUPE31W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE31W::_00 => 0,
            WUPE31W::_01 => 1,
            WUPE31W::_10 => 2,
            WUPE31W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE31W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE31W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE31W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE31W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE31W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE31W::_11)
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
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P28"]
    #[inline]
    pub fn wupe28(&self) -> WUPE28R {
        WUPE28R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P29"]
    #[inline]
    pub fn wupe29(&self) -> WUPE29R {
        WUPE29R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P30"]
    #[inline]
    pub fn wupe30(&self) -> WUPE30R {
        WUPE30R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P31"]
    #[inline]
    pub fn wupe31(&self) -> WUPE31R {
        WUPE31R::_from({
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
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P28"]
    #[inline]
    pub fn wupe28(&mut self) -> _WUPE28W {
        _WUPE28W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P29"]
    #[inline]
    pub fn wupe29(&mut self) -> _WUPE29W {
        _WUPE29W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P30"]
    #[inline]
    pub fn wupe30(&mut self) -> _WUPE30W {
        _WUPE30W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P31"]
    #[inline]
    pub fn wupe31(&mut self) -> _WUPE31W {
        _WUPE31W { w: self }
    }
}
