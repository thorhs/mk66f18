#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PE7 {
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
#[doc = "Possible values of the field `WUPE24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE24R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE24R::_00 => 0,
            WUPE24R::_01 => 1,
            WUPE24R::_10 => 2,
            WUPE24R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE24R {
        match value {
            0 => WUPE24R::_00,
            1 => WUPE24R::_01,
            2 => WUPE24R::_10,
            3 => WUPE24R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE24R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE24R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE24R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE24R::_11
    }
}
#[doc = "Possible values of the field `WUPE25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE25R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE25R::_00 => 0,
            WUPE25R::_01 => 1,
            WUPE25R::_10 => 2,
            WUPE25R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE25R {
        match value {
            0 => WUPE25R::_00,
            1 => WUPE25R::_01,
            2 => WUPE25R::_10,
            3 => WUPE25R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE25R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE25R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE25R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE25R::_11
    }
}
#[doc = "Possible values of the field `WUPE26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE26R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE26R::_00 => 0,
            WUPE26R::_01 => 1,
            WUPE26R::_10 => 2,
            WUPE26R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE26R {
        match value {
            0 => WUPE26R::_00,
            1 => WUPE26R::_01,
            2 => WUPE26R::_10,
            3 => WUPE26R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE26R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE26R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE26R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE26R::_11
    }
}
#[doc = "Possible values of the field `WUPE27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE27R {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE27R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUPE27R::_00 => 0,
            WUPE27R::_01 => 1,
            WUPE27R::_10 => 2,
            WUPE27R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUPE27R {
        match value {
            0 => WUPE27R::_00,
            1 => WUPE27R::_01,
            2 => WUPE27R::_10,
            3 => WUPE27R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WUPE27R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WUPE27R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WUPE27R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == WUPE27R::_11
    }
}
#[doc = "Values that can be written to the field `WUPE24`"]
pub enum WUPE24W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE24W::_00 => 0,
            WUPE24W::_01 => 1,
            WUPE24W::_10 => 2,
            WUPE24W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE24W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE24W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE24W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE24W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE24W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE24W::_11)
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
#[doc = "Values that can be written to the field `WUPE25`"]
pub enum WUPE25W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE25W::_00 => 0,
            WUPE25W::_01 => 1,
            WUPE25W::_10 => 2,
            WUPE25W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE25W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE25W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE25W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE25W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE25W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE25W::_11)
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
#[doc = "Values that can be written to the field `WUPE26`"]
pub enum WUPE26W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE26W::_00 => 0,
            WUPE26W::_01 => 1,
            WUPE26W::_10 => 2,
            WUPE26W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE26W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE26W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE26W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE26W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE26W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE26W::_11)
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
#[doc = "Values that can be written to the field `WUPE27`"]
pub enum WUPE27W {
    #[doc = "External input pin disabled as wakeup input"]
    _00,
    #[doc = "External input pin enabled with rising edge detection"]
    _01,
    #[doc = "External input pin enabled with falling edge detection"]
    _10,
    #[doc = "External input pin enabled with any change detection"]
    _11,
}
impl WUPE27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WUPE27W::_00 => 0,
            WUPE27W::_01 => 1,
            WUPE27W::_10 => 2,
            WUPE27W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPE27W<'a> {
    w: &'a mut W,
}
impl<'a> _WUPE27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPE27W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE27W::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE27W::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE27W::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE27W::_11)
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
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P24"]
    #[inline]
    pub fn wupe24(&self) -> WUPE24R {
        WUPE24R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P25"]
    #[inline]
    pub fn wupe25(&self) -> WUPE25R {
        WUPE25R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P26"]
    #[inline]
    pub fn wupe26(&self) -> WUPE26R {
        WUPE26R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P27"]
    #[inline]
    pub fn wupe27(&self) -> WUPE27R {
        WUPE27R::_from({
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
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P24"]
    #[inline]
    pub fn wupe24(&mut self) -> _WUPE24W {
        _WUPE24W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P25"]
    #[inline]
    pub fn wupe25(&mut self) -> _WUPE25W {
        _WUPE25W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P26"]
    #[inline]
    pub fn wupe26(&mut self) -> _WUPE26W {
        _WUPE26W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P27"]
    #[inline]
    pub fn wupe27(&mut self) -> _WUPE27W {
        _WUPE27W { w: self }
    }
}
