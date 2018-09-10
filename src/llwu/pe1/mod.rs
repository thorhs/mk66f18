#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PE1 {
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
#[doc = "Possible values of the field `WUPE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE0R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE0R::_00 => 0,
            WUPE0R::_01 => 1,
            WUPE0R::_10 => 2,
            WUPE0R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE0R {
        match value {
            0 => WUPE0R::_00,
            1 => WUPE0R::_01,
            2 => WUPE0R::_10,
            3 => WUPE0R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE0R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE0R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE0R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE0R::_11
    }
}
#[doc = "Possible values of the field `WUPE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE1R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE1R::_00 => 0,
            WUPE1R::_01 => 1,
            WUPE1R::_10 => 2,
            WUPE1R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE1R {
        match value {
            0 => WUPE1R::_00,
            1 => WUPE1R::_01,
            2 => WUPE1R::_10,
            3 => WUPE1R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE1R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE1R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE1R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE1R::_11
    }
}
#[doc = "Possible values of the field `WUPE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE2R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE2R::_00 => 0,
            WUPE2R::_01 => 1,
            WUPE2R::_10 => 2,
            WUPE2R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE2R {
        match value {
            0 => WUPE2R::_00,
            1 => WUPE2R::_01,
            2 => WUPE2R::_10,
            3 => WUPE2R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE2R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE2R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE2R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE2R::_11
    }
}
#[doc = "Possible values of the field `WUPE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE3R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE3R::_00 => 0,
            WUPE3R::_01 => 1,
            WUPE3R::_10 => 2,
            WUPE3R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE3R {
        match value {
            0 => WUPE3R::_00,
            1 => WUPE3R::_01,
            2 => WUPE3R::_10,
            3 => WUPE3R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE3R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE3R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE3R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE3R::_11
    }
}
#[doc = "Values that can be written to the field `WUPE0`"]
pub enum WUPE0W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE0W::_00 => 0,
            WUPE0W::_01 => 1,
            WUPE0W::_10 => 2,
            WUPE0W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE0W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE0W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE0W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE0W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE0W::_11)
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
#[doc = "Values that can be written to the field `WUPE1`"]
pub enum WUPE1W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE1W::_00 => 0,
            WUPE1W::_01 => 1,
            WUPE1W::_10 => 2,
            WUPE1W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE1W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE1W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE1W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE1W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE1W::_11)
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
#[doc = "Values that can be written to the field `WUPE2`"]
pub enum WUPE2W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE2W::_00 => 0,
            WUPE2W::_01 => 1,
            WUPE2W::_10 => 2,
            WUPE2W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE2W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE2W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE2W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE2W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE2W::_11)
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
#[doc = "Values that can be written to the field `WUPE3`"]
pub enum WUPE3W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE3W::_00 => 0,
            WUPE3W::_01 => 1,
            WUPE3W::_10 => 2,
            WUPE3W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE3W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE3W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE3W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE3W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE3W::_11)
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
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P0"]
    #[inline]
    pub fn wupe0(&self) -> WUPE0R {
        WUPE0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P1"]
    #[inline]
    pub fn wupe1(&self) -> WUPE1R {
        WUPE1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P2"]
    #[inline]
    pub fn wupe2(&self) -> WUPE2R {
        WUPE2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P3"]
    #[inline]
    pub fn wupe3(&self) -> WUPE3R {
        WUPE3R::_from({
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
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P0"]
    #[inline]
    pub fn wupe0(&mut self) -> _WUPE0W {
        _WUPE0W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P1"]
    #[inline]
    pub fn wupe1(&mut self) -> _WUPE1W {
        _WUPE1W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P2"]
    #[inline]
    pub fn wupe2(&mut self) -> _WUPE2W {
        _WUPE2W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P3"]
    #[inline]
    pub fn wupe3(&mut self) -> _WUPE3W {
        _WUPE3W { w: self }
    }
}
