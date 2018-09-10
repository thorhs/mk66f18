#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PF2 {
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
#[doc = "Possible values of the field `WUF8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF8R {
    #[doc = "LLWU_P8 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P8 input was a wakeup source"]
    _1,
}
impl WUF8R {
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
            WUF8R::_0 => false,
            WUF8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF8R {
        match value {
            false => WUF8R::_0,
            true => WUF8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF8R::_1
    }
}
#[doc = "Possible values of the field `WUF9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF9R {
    #[doc = "LLWU_P9 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P9 input was a wakeup source"]
    _1,
}
impl WUF9R {
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
            WUF9R::_0 => false,
            WUF9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF9R {
        match value {
            false => WUF9R::_0,
            true => WUF9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF9R::_1
    }
}
#[doc = "Possible values of the field `WUF10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF10R {
    #[doc = "LLWU_P10 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P10 input was a wakeup source"]
    _1,
}
impl WUF10R {
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
            WUF10R::_0 => false,
            WUF10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF10R {
        match value {
            false => WUF10R::_0,
            true => WUF10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF10R::_1
    }
}
#[doc = "Possible values of the field `WUF11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF11R {
    #[doc = "LLWU_P11 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P11 input was a wakeup source"]
    _1,
}
impl WUF11R {
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
            WUF11R::_0 => false,
            WUF11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF11R {
        match value {
            false => WUF11R::_0,
            true => WUF11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF11R::_1
    }
}
#[doc = "Possible values of the field `WUF12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF12R {
    #[doc = "LLWU_P12 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P12 input was a wakeup source"]
    _1,
}
impl WUF12R {
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
            WUF12R::_0 => false,
            WUF12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF12R {
        match value {
            false => WUF12R::_0,
            true => WUF12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF12R::_1
    }
}
#[doc = "Possible values of the field `WUF13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF13R {
    #[doc = "LLWU_P13 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P13 input was a wakeup source"]
    _1,
}
impl WUF13R {
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
            WUF13R::_0 => false,
            WUF13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF13R {
        match value {
            false => WUF13R::_0,
            true => WUF13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF13R::_1
    }
}
#[doc = "Possible values of the field `WUF14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF14R {
    #[doc = "LLWU_P14 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P14 input was a wakeup source"]
    _1,
}
impl WUF14R {
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
            WUF14R::_0 => false,
            WUF14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF14R {
        match value {
            false => WUF14R::_0,
            true => WUF14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF14R::_1
    }
}
#[doc = "Possible values of the field `WUF15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF15R {
    #[doc = "LLWU_P15 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P15 input was a wakeup source"]
    _1,
}
impl WUF15R {
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
            WUF15R::_0 => false,
            WUF15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF15R {
        match value {
            false => WUF15R::_0,
            true => WUF15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF15R::_1
    }
}
#[doc = "Values that can be written to the field `WUF8`"]
pub enum WUF8W {
    #[doc = "LLWU_P8 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P8 input was a wakeup source"]
    _1,
}
impl WUF8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF8W::_0 => false,
            WUF8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF8W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P8 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF8W::_0)
    }
    #[doc = "LLWU_P8 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF8W::_1)
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
#[doc = "Values that can be written to the field `WUF9`"]
pub enum WUF9W {
    #[doc = "LLWU_P9 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P9 input was a wakeup source"]
    _1,
}
impl WUF9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF9W::_0 => false,
            WUF9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF9W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P9 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF9W::_0)
    }
    #[doc = "LLWU_P9 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF9W::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WUF10`"]
pub enum WUF10W {
    #[doc = "LLWU_P10 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P10 input was a wakeup source"]
    _1,
}
impl WUF10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF10W::_0 => false,
            WUF10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF10W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P10 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF10W::_0)
    }
    #[doc = "LLWU_P10 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF10W::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WUF11`"]
pub enum WUF11W {
    #[doc = "LLWU_P11 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P11 input was a wakeup source"]
    _1,
}
impl WUF11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF11W::_0 => false,
            WUF11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF11W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P11 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF11W::_0)
    }
    #[doc = "LLWU_P11 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF11W::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WUF12`"]
pub enum WUF12W {
    #[doc = "LLWU_P12 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P12 input was a wakeup source"]
    _1,
}
impl WUF12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF12W::_0 => false,
            WUF12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF12W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P12 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF12W::_0)
    }
    #[doc = "LLWU_P12 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF12W::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WUF13`"]
pub enum WUF13W {
    #[doc = "LLWU_P13 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P13 input was a wakeup source"]
    _1,
}
impl WUF13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF13W::_0 => false,
            WUF13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF13W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P13 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF13W::_0)
    }
    #[doc = "LLWU_P13 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF13W::_1)
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
#[doc = "Values that can be written to the field `WUF14`"]
pub enum WUF14W {
    #[doc = "LLWU_P14 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P14 input was a wakeup source"]
    _1,
}
impl WUF14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF14W::_0 => false,
            WUF14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF14W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P14 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF14W::_0)
    }
    #[doc = "LLWU_P14 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF14W::_1)
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
#[doc = "Values that can be written to the field `WUF15`"]
pub enum WUF15W {
    #[doc = "LLWU_P15 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P15 input was a wakeup source"]
    _1,
}
impl WUF15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF15W::_0 => false,
            WUF15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF15W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P15 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF15W::_0)
    }
    #[doc = "LLWU_P15 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF15W::_1)
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P8"]
    #[inline]
    pub fn wuf8(&self) -> WUF8R {
        WUF8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P9"]
    #[inline]
    pub fn wuf9(&self) -> WUF9R {
        WUF9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P10"]
    #[inline]
    pub fn wuf10(&self) -> WUF10R {
        WUF10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P11"]
    #[inline]
    pub fn wuf11(&self) -> WUF11R {
        WUF11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P12"]
    #[inline]
    pub fn wuf12(&self) -> WUF12R {
        WUF12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P13"]
    #[inline]
    pub fn wuf13(&self) -> WUF13R {
        WUF13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P14"]
    #[inline]
    pub fn wuf14(&self) -> WUF14R {
        WUF14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P15"]
    #[inline]
    pub fn wuf15(&self) -> WUF15R {
        WUF15R::_from({
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P8"]
    #[inline]
    pub fn wuf8(&mut self) -> _WUF8W {
        _WUF8W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P9"]
    #[inline]
    pub fn wuf9(&mut self) -> _WUF9W {
        _WUF9W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P10"]
    #[inline]
    pub fn wuf10(&mut self) -> _WUF10W {
        _WUF10W { w: self }
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P11"]
    #[inline]
    pub fn wuf11(&mut self) -> _WUF11W {
        _WUF11W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P12"]
    #[inline]
    pub fn wuf12(&mut self) -> _WUF12W {
        _WUF12W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P13"]
    #[inline]
    pub fn wuf13(&mut self) -> _WUF13W {
        _WUF13W { w: self }
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P14"]
    #[inline]
    pub fn wuf14(&mut self) -> _WUF14W {
        _WUF14W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P15"]
    #[inline]
    pub fn wuf15(&mut self) -> _WUF15W {
        _WUF15W { w: self }
    }
}
