#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PE6 {
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
#[doc = "Possible values of the field `WUPE20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE20R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE20R::_00 => 0,
            WUPE20R::_01 => 1,
            WUPE20R::_10 => 2,
            WUPE20R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE20R {
        match value {
            0 => WUPE20R::_00,
            1 => WUPE20R::_01,
            2 => WUPE20R::_10,
            3 => WUPE20R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE20R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE20R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE20R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE20R::_11
    }
}
#[doc = "Possible values of the field `WUPE21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE21R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE21R::_00 => 0,
            WUPE21R::_01 => 1,
            WUPE21R::_10 => 2,
            WUPE21R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE21R {
        match value {
            0 => WUPE21R::_00,
            1 => WUPE21R::_01,
            2 => WUPE21R::_10,
            3 => WUPE21R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE21R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE21R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE21R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE21R::_11
    }
}
#[doc = "Possible values of the field `WUPE22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE22R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE22R::_00 => 0,
            WUPE22R::_01 => 1,
            WUPE22R::_10 => 2,
            WUPE22R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE22R {
        match value {
            0 => WUPE22R::_00,
            1 => WUPE22R::_01,
            2 => WUPE22R::_10,
            3 => WUPE22R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE22R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE22R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE22R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE22R::_11
    }
}
#[doc = "Possible values of the field `WUPE23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE23R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE23R::_00 => 0,
            WUPE23R::_01 => 1,
            WUPE23R::_10 => 2,
            WUPE23R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE23R {
        match value {
            0 => WUPE23R::_00,
            1 => WUPE23R::_01,
            2 => WUPE23R::_10,
            3 => WUPE23R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE23R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE23R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE23R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE23R::_11
    }
}
#[doc = "Values that can be written to the field `WUPE20`"]
pub enum WUPE20W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE20W::_00 => 0,
            WUPE20W::_01 => 1,
            WUPE20W::_10 => 2,
            WUPE20W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE20W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE20W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE20W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE20W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE20W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE20W::_11)
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
#[doc = "Values that can be written to the field `WUPE21`"]
pub enum WUPE21W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE21W::_00 => 0,
            WUPE21W::_01 => 1,
            WUPE21W::_10 => 2,
            WUPE21W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE21W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE21W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE21W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE21W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE21W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE21W::_11)
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
#[doc = "Values that can be written to the field `WUPE22`"]
pub enum WUPE22W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE22W::_00 => 0,
            WUPE22W::_01 => 1,
            WUPE22W::_10 => 2,
            WUPE22W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE22W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE22W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE22W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE22W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE22W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE22W::_11)
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
#[doc = "Values that can be written to the field `WUPE23`"]
pub enum WUPE23W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE23W::_00 => 0,
            WUPE23W::_01 => 1,
            WUPE23W::_10 => 2,
            WUPE23W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE23W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE23W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE23W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE23W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE23W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE23W::_11)
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
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P20"]
    #[inline]
    pub fn wupe20(&self) -> WUPE20R {
        WUPE20R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P21"]
    #[inline]
    pub fn wupe21(&self) -> WUPE21R {
        WUPE21R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P22"]
    #[inline]
    pub fn wupe22(&self) -> WUPE22R {
        WUPE22R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P23"]
    #[inline]
    pub fn wupe23(&self) -> WUPE23R {
        WUPE23R::_from({
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
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P20"]
    #[inline]
    pub fn wupe20(&mut self) -> _WUPE20W {
        _WUPE20W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P21"]
    #[inline]
    pub fn wupe21(&mut self) -> _WUPE21W {
        _WUPE21W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P22"]
    #[inline]
    pub fn wupe22(&mut self) -> _WUPE22W {
        _WUPE22W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P23"]
    #[inline]
    pub fn wupe23(&mut self) -> _WUPE23W {
        _WUPE23W { w: self }
    }
}
