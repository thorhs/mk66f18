#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PF3 {
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
#[doc = "Possible values of the field `WUF16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF16R {
    #[doc = "LLWU_P16 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P16 input was a wakeup source"]
    _1,
}
impl WUF16R {
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
            WUF16R::_0 => false,
            WUF16R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF16R {
        match value {
            false => WUF16R::_0,
            true => WUF16R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF16R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF16R::_1
    }
}
#[doc = "Possible values of the field `WUF17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF17R {
    #[doc = "LLWU_P17 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P17 input was a wakeup source"]
    _1,
}
impl WUF17R {
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
            WUF17R::_0 => false,
            WUF17R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF17R {
        match value {
            false => WUF17R::_0,
            true => WUF17R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF17R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF17R::_1
    }
}
#[doc = "Possible values of the field `WUF18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF18R {
    #[doc = "LLWU_P18 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P18 input was a wakeup source"]
    _1,
}
impl WUF18R {
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
            WUF18R::_0 => false,
            WUF18R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF18R {
        match value {
            false => WUF18R::_0,
            true => WUF18R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF18R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF18R::_1
    }
}
#[doc = "Possible values of the field `WUF19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF19R {
    #[doc = "LLWU_P19 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P19 input was a wakeup source"]
    _1,
}
impl WUF19R {
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
            WUF19R::_0 => false,
            WUF19R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF19R {
        match value {
            false => WUF19R::_0,
            true => WUF19R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF19R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF19R::_1
    }
}
#[doc = "Possible values of the field `WUF20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF20R {
    #[doc = "LLWU_P20 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P20 input was a wakeup source"]
    _1,
}
impl WUF20R {
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
            WUF20R::_0 => false,
            WUF20R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF20R {
        match value {
            false => WUF20R::_0,
            true => WUF20R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF20R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF20R::_1
    }
}
#[doc = "Possible values of the field `WUF21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF21R {
    #[doc = "LLWU_P21 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P21 input was a wakeup source"]
    _1,
}
impl WUF21R {
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
            WUF21R::_0 => false,
            WUF21R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF21R {
        match value {
            false => WUF21R::_0,
            true => WUF21R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF21R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF21R::_1
    }
}
#[doc = "Possible values of the field `WUF22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF22R {
    #[doc = "LLWU_P22 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P22 input was a wakeup source"]
    _1,
}
impl WUF22R {
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
            WUF22R::_0 => false,
            WUF22R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF22R {
        match value {
            false => WUF22R::_0,
            true => WUF22R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF22R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF22R::_1
    }
}
#[doc = "Possible values of the field `WUF23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF23R {
    #[doc = "LLWU_P23 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P23 input was a wakeup source"]
    _1,
}
impl WUF23R {
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
            WUF23R::_0 => false,
            WUF23R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF23R {
        match value {
            false => WUF23R::_0,
            true => WUF23R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF23R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF23R::_1
    }
}
#[doc = "Values that can be written to the field `WUF16`"]
pub enum WUF16W {
    #[doc = "LLWU_P16 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P16 input was a wakeup source"]
    _1,
}
impl WUF16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF16W::_0 => false,
            WUF16W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF16W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P16 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF16W::_0)
    }
    #[doc = "LLWU_P16 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF16W::_1)
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
#[doc = "Values that can be written to the field `WUF17`"]
pub enum WUF17W {
    #[doc = "LLWU_P17 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P17 input was a wakeup source"]
    _1,
}
impl WUF17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF17W::_0 => false,
            WUF17W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF17W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P17 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF17W::_0)
    }
    #[doc = "LLWU_P17 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF17W::_1)
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
#[doc = "Values that can be written to the field `WUF18`"]
pub enum WUF18W {
    #[doc = "LLWU_P18 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P18 input was a wakeup source"]
    _1,
}
impl WUF18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF18W::_0 => false,
            WUF18W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF18W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P18 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF18W::_0)
    }
    #[doc = "LLWU_P18 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF18W::_1)
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
#[doc = "Values that can be written to the field `WUF19`"]
pub enum WUF19W {
    #[doc = "LLWU_P19 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P19 input was a wakeup source"]
    _1,
}
impl WUF19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF19W::_0 => false,
            WUF19W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF19W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P19 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF19W::_0)
    }
    #[doc = "LLWU_P19 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF19W::_1)
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
#[doc = "Values that can be written to the field `WUF20`"]
pub enum WUF20W {
    #[doc = "LLWU_P20 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P20 input was a wakeup source"]
    _1,
}
impl WUF20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF20W::_0 => false,
            WUF20W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF20W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P20 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF20W::_0)
    }
    #[doc = "LLWU_P20 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF20W::_1)
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
#[doc = "Values that can be written to the field `WUF21`"]
pub enum WUF21W {
    #[doc = "LLWU_P21 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P21 input was a wakeup source"]
    _1,
}
impl WUF21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF21W::_0 => false,
            WUF21W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF21W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P21 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF21W::_0)
    }
    #[doc = "LLWU_P21 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF21W::_1)
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
#[doc = "Values that can be written to the field `WUF22`"]
pub enum WUF22W {
    #[doc = "LLWU_P22 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P22 input was a wakeup source"]
    _1,
}
impl WUF22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF22W::_0 => false,
            WUF22W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF22W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P22 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF22W::_0)
    }
    #[doc = "LLWU_P22 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF22W::_1)
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
#[doc = "Values that can be written to the field `WUF23`"]
pub enum WUF23W {
    #[doc = "LLWU_P23 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P23 input was a wakeup source"]
    _1,
}
impl WUF23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF23W::_0 => false,
            WUF23W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF23W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P23 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF23W::_0)
    }
    #[doc = "LLWU_P23 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF23W::_1)
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P16"]
    #[inline]
    pub fn wuf16(&self) -> WUF16R {
        WUF16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P17"]
    #[inline]
    pub fn wuf17(&self) -> WUF17R {
        WUF17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P18"]
    #[inline]
    pub fn wuf18(&self) -> WUF18R {
        WUF18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P19"]
    #[inline]
    pub fn wuf19(&self) -> WUF19R {
        WUF19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P20"]
    #[inline]
    pub fn wuf20(&self) -> WUF20R {
        WUF20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P21"]
    #[inline]
    pub fn wuf21(&self) -> WUF21R {
        WUF21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P22"]
    #[inline]
    pub fn wuf22(&self) -> WUF22R {
        WUF22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P23"]
    #[inline]
    pub fn wuf23(&self) -> WUF23R {
        WUF23R::_from({
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P16"]
    #[inline]
    pub fn wuf16(&mut self) -> _WUF16W {
        _WUF16W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P17"]
    #[inline]
    pub fn wuf17(&mut self) -> _WUF17W {
        _WUF17W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P18"]
    #[inline]
    pub fn wuf18(&mut self) -> _WUF18W {
        _WUF18W { w: self }
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P19"]
    #[inline]
    pub fn wuf19(&mut self) -> _WUF19W {
        _WUF19W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P20"]
    #[inline]
    pub fn wuf20(&mut self) -> _WUF20W {
        _WUF20W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P21"]
    #[inline]
    pub fn wuf21(&mut self) -> _WUF21W {
        _WUF21W { w: self }
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P22"]
    #[inline]
    pub fn wuf22(&mut self) -> _WUF22W {
        _WUF22W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P23"]
    #[inline]
    pub fn wuf23(&mut self) -> _WUF23W {
        _WUF23W { w: self }
    }
}
