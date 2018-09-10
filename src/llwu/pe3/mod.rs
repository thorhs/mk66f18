#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PE3 {
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
#[doc = "Possible values of the field `WUPE8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE8R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE8R::_00 => 0,
            WUPE8R::_01 => 1,
            WUPE8R::_10 => 2,
            WUPE8R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE8R {
        match value {
            0 => WUPE8R::_00,
            1 => WUPE8R::_01,
            2 => WUPE8R::_10,
            3 => WUPE8R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE8R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE8R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE8R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE8R::_11
    }
}
#[doc = "Possible values of the field `WUPE9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE9R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE9R::_00 => 0,
            WUPE9R::_01 => 1,
            WUPE9R::_10 => 2,
            WUPE9R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE9R {
        match value {
            0 => WUPE9R::_00,
            1 => WUPE9R::_01,
            2 => WUPE9R::_10,
            3 => WUPE9R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE9R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE9R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE9R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE9R::_11
    }
}
#[doc = "Possible values of the field `WUPE10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE10R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE10R::_00 => 0,
            WUPE10R::_01 => 1,
            WUPE10R::_10 => 2,
            WUPE10R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE10R {
        match value {
            0 => WUPE10R::_00,
            1 => WUPE10R::_01,
            2 => WUPE10R::_10,
            3 => WUPE10R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE10R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE10R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE10R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE10R::_11
    }
}
#[doc = "Possible values of the field `WUPE11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE11R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE11R::_00 => 0,
            WUPE11R::_01 => 1,
            WUPE11R::_10 => 2,
            WUPE11R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE11R {
        match value {
            0 => WUPE11R::_00,
            1 => WUPE11R::_01,
            2 => WUPE11R::_10,
            3 => WUPE11R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE11R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE11R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE11R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE11R::_11
    }
}
#[doc = "Values that can be written to the field `WUPE8`"]
pub enum WUPE8W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE8W::_00 => 0,
            WUPE8W::_01 => 1,
            WUPE8W::_10 => 2,
            WUPE8W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE8W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE8W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE8W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE8W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE8W::_11)
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
#[doc = "Values that can be written to the field `WUPE9`"]
pub enum WUPE9W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE9W::_00 => 0,
            WUPE9W::_01 => 1,
            WUPE9W::_10 => 2,
            WUPE9W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE9W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE9W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE9W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE9W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE9W::_11)
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
#[doc = "Values that can be written to the field `WUPE10`"]
pub enum WUPE10W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE10W::_00 => 0,
            WUPE10W::_01 => 1,
            WUPE10W::_10 => 2,
            WUPE10W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE10W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE10W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE10W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE10W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE10W::_11)
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
#[doc = "Values that can be written to the field `WUPE11`"]
pub enum WUPE11W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE11W::_00 => 0,
            WUPE11W::_01 => 1,
            WUPE11W::_10 => 2,
            WUPE11W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE11W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE11W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE11W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE11W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE11W::_11)
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
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P8"]
    #[inline]
    pub fn wupe8(&self) -> WUPE8R {
        WUPE8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P9"]
    #[inline]
    pub fn wupe9(&self) -> WUPE9R {
        WUPE9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P10"]
    #[inline]
    pub fn wupe10(&self) -> WUPE10R {
        WUPE10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P11"]
    #[inline]
    pub fn wupe11(&self) -> WUPE11R {
        WUPE11R::_from({
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
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P8"]
    #[inline]
    pub fn wupe8(&mut self) -> _WUPE8W {
        _WUPE8W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P9"]
    #[inline]
    pub fn wupe9(&mut self) -> _WUPE9W {
        _WUPE9W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P10"]
    #[inline]
    pub fn wupe10(&mut self) -> _WUPE10W {
        _WUPE10W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P11"]
    #[inline]
    pub fn wupe11(&mut self) -> _WUPE11W {
        _WUPE11W { w: self }
    }
}
