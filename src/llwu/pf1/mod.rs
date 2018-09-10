#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PF1 {
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
#[doc = "Possible values of the field `WUF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF0R {
    #[doc = "LLWU_P0 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P0 input was a wakeup source"]
    _1,
}
impl WUF0R {
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
            WUF0R::_0 => false,
            WUF0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF0R {
        match value {
            false => WUF0R::_0,
            true => WUF0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF0R::_1
    }
}
#[doc = "Possible values of the field `WUF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF1R {
    #[doc = "LLWU_P1 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P1 input was a wakeup source"]
    _1,
}
impl WUF1R {
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
            WUF1R::_0 => false,
            WUF1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF1R {
        match value {
            false => WUF1R::_0,
            true => WUF1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF1R::_1
    }
}
#[doc = "Possible values of the field `WUF2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF2R {
    #[doc = "LLWU_P2 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P2 input was a wakeup source"]
    _1,
}
impl WUF2R {
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
            WUF2R::_0 => false,
            WUF2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF2R {
        match value {
            false => WUF2R::_0,
            true => WUF2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF2R::_1
    }
}
#[doc = "Possible values of the field `WUF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF3R {
    #[doc = "LLWU_P3 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P3 input was a wakeup source"]
    _1,
}
impl WUF3R {
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
            WUF3R::_0 => false,
            WUF3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF3R {
        match value {
            false => WUF3R::_0,
            true => WUF3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF3R::_1
    }
}
#[doc = "Possible values of the field `WUF4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF4R {
    #[doc = "LLWU_P4 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P4 input was a wakeup source"]
    _1,
}
impl WUF4R {
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
            WUF4R::_0 => false,
            WUF4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF4R {
        match value {
            false => WUF4R::_0,
            true => WUF4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF4R::_1
    }
}
#[doc = "Possible values of the field `WUF5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF5R {
    #[doc = "LLWU_P5 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P5 input was a wakeup source"]
    _1,
}
impl WUF5R {
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
            WUF5R::_0 => false,
            WUF5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF5R {
        match value {
            false => WUF5R::_0,
            true => WUF5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF5R::_1
    }
}
#[doc = "Possible values of the field `WUF6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF6R {
    #[doc = "LLWU_P6 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P6 input was a wakeup source"]
    _1,
}
impl WUF6R {
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
            WUF6R::_0 => false,
            WUF6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF6R {
        match value {
            false => WUF6R::_0,
            true => WUF6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF6R::_1
    }
}
#[doc = "Possible values of the field `WUF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF7R {
    #[doc = "LLWU_P7 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P7 input was a wakeup source"]
    _1,
}
impl WUF7R {
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
            WUF7R::_0 => false,
            WUF7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF7R {
        match value {
            false => WUF7R::_0,
            true => WUF7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF7R::_1
    }
}
#[doc = "Values that can be written to the field `WUF0`"]
pub enum WUF0W {
    #[doc = "LLWU_P0 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P0 input was a wakeup source"]
    _1,
}
impl WUF0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF0W::_0 => false,
            WUF0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF0W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P0 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF0W::_0)
    }
    #[doc = "LLWU_P0 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF0W::_1)
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
#[doc = "Values that can be written to the field `WUF1`"]
pub enum WUF1W {
    #[doc = "LLWU_P1 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P1 input was a wakeup source"]
    _1,
}
impl WUF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF1W::_0 => false,
            WUF1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF1W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P1 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF1W::_0)
    }
    #[doc = "LLWU_P1 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF1W::_1)
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
#[doc = "Values that can be written to the field `WUF2`"]
pub enum WUF2W {
    #[doc = "LLWU_P2 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P2 input was a wakeup source"]
    _1,
}
impl WUF2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF2W::_0 => false,
            WUF2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF2W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P2 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF2W::_0)
    }
    #[doc = "LLWU_P2 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF2W::_1)
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
#[doc = "Values that can be written to the field `WUF3`"]
pub enum WUF3W {
    #[doc = "LLWU_P3 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P3 input was a wakeup source"]
    _1,
}
impl WUF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF3W::_0 => false,
            WUF3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF3W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P3 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF3W::_0)
    }
    #[doc = "LLWU_P3 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF3W::_1)
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
#[doc = "Values that can be written to the field `WUF4`"]
pub enum WUF4W {
    #[doc = "LLWU_P4 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P4 input was a wakeup source"]
    _1,
}
impl WUF4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF4W::_0 => false,
            WUF4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF4W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P4 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF4W::_0)
    }
    #[doc = "LLWU_P4 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF4W::_1)
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
#[doc = "Values that can be written to the field `WUF5`"]
pub enum WUF5W {
    #[doc = "LLWU_P5 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P5 input was a wakeup source"]
    _1,
}
impl WUF5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF5W::_0 => false,
            WUF5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF5W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P5 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF5W::_0)
    }
    #[doc = "LLWU_P5 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF5W::_1)
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
#[doc = "Values that can be written to the field `WUF6`"]
pub enum WUF6W {
    #[doc = "LLWU_P6 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P6 input was a wakeup source"]
    _1,
}
impl WUF6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF6W::_0 => false,
            WUF6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF6W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P6 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF6W::_0)
    }
    #[doc = "LLWU_P6 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF6W::_1)
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
#[doc = "Values that can be written to the field `WUF7`"]
pub enum WUF7W {
    #[doc = "LLWU_P7 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P7 input was a wakeup source"]
    _1,
}
impl WUF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF7W::_0 => false,
            WUF7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF7W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P7 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF7W::_0)
    }
    #[doc = "LLWU_P7 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF7W::_1)
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P0"]
    #[inline]
    pub fn wuf0(&self) -> WUF0R {
        WUF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P1"]
    #[inline]
    pub fn wuf1(&self) -> WUF1R {
        WUF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P2"]
    #[inline]
    pub fn wuf2(&self) -> WUF2R {
        WUF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P3"]
    #[inline]
    pub fn wuf3(&self) -> WUF3R {
        WUF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P4"]
    #[inline]
    pub fn wuf4(&self) -> WUF4R {
        WUF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P5"]
    #[inline]
    pub fn wuf5(&self) -> WUF5R {
        WUF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P6"]
    #[inline]
    pub fn wuf6(&self) -> WUF6R {
        WUF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P7"]
    #[inline]
    pub fn wuf7(&self) -> WUF7R {
        WUF7R::_from({
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P0"]
    #[inline]
    pub fn wuf0(&mut self) -> _WUF0W {
        _WUF0W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P1"]
    #[inline]
    pub fn wuf1(&mut self) -> _WUF1W {
        _WUF1W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P2"]
    #[inline]
    pub fn wuf2(&mut self) -> _WUF2W {
        _WUF2W { w: self }
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P3"]
    #[inline]
    pub fn wuf3(&mut self) -> _WUF3W {
        _WUF3W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P4"]
    #[inline]
    pub fn wuf4(&mut self) -> _WUF4W {
        _WUF4W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P5"]
    #[inline]
    pub fn wuf5(&mut self) -> _WUF5W {
        _WUF5W { w: self }
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P6"]
    #[inline]
    pub fn wuf6(&mut self) -> _WUF6W {
        _WUF6W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P7"]
    #[inline]
    pub fn wuf7(&mut self) -> _WUF7W {
        _WUF7W { w: self }
    }
}
