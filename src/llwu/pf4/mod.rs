#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PF4 {
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
#[doc = "Possible values of the field `WUF24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF24R {
    #[doc = "LLWU_P24 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P24 input was a wakeup source"]
    _1,
}
impl WUF24R {
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
            WUF24R::_0 => false,
            WUF24R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF24R {
        match value {
            false => WUF24R::_0,
            true => WUF24R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF24R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF24R::_1
    }
}
#[doc = "Possible values of the field `WUF25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF25R {
    #[doc = "LLWU_P25 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P25 input was a wakeup source"]
    _1,
}
impl WUF25R {
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
            WUF25R::_0 => false,
            WUF25R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF25R {
        match value {
            false => WUF25R::_0,
            true => WUF25R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF25R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF25R::_1
    }
}
#[doc = "Possible values of the field `WUF26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF26R {
    #[doc = "LLWU_P26 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P26 input was a wakeup source"]
    _1,
}
impl WUF26R {
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
            WUF26R::_0 => false,
            WUF26R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF26R {
        match value {
            false => WUF26R::_0,
            true => WUF26R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF26R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF26R::_1
    }
}
#[doc = "Possible values of the field `WUF27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF27R {
    #[doc = "LLWU_P27 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P27 input was a wakeup source"]
    _1,
}
impl WUF27R {
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
            WUF27R::_0 => false,
            WUF27R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF27R {
        match value {
            false => WUF27R::_0,
            true => WUF27R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF27R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF27R::_1
    }
}
#[doc = "Possible values of the field `WUF28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF28R {
    #[doc = "LLWU_P28 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P28 input was a wakeup source"]
    _1,
}
impl WUF28R {
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
            WUF28R::_0 => false,
            WUF28R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF28R {
        match value {
            false => WUF28R::_0,
            true => WUF28R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF28R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF28R::_1
    }
}
#[doc = "Possible values of the field `WUF29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF29R {
    #[doc = "LLWU_P29 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P29 input was a wakeup source"]
    _1,
}
impl WUF29R {
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
            WUF29R::_0 => false,
            WUF29R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF29R {
        match value {
            false => WUF29R::_0,
            true => WUF29R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF29R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF29R::_1
    }
}
#[doc = "Possible values of the field `WUF30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF30R {
    #[doc = "LLWU_P30 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P30 input was a wakeup source"]
    _1,
}
impl WUF30R {
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
            WUF30R::_0 => false,
            WUF30R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF30R {
        match value {
            false => WUF30R::_0,
            true => WUF30R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF30R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF30R::_1
    }
}
#[doc = "Possible values of the field `WUF31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF31R {
    #[doc = "LLWU_P31 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P31 input was a wakeup source"]
    _1,
}
impl WUF31R {
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
            WUF31R::_0 => false,
            WUF31R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUF31R {
        match value {
            false => WUF31R::_0,
            true => WUF31R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUF31R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUF31R::_1
    }
}
#[doc = "Values that can be written to the field `WUF24`"]
pub enum WUF24W {
    #[doc = "LLWU_P24 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P24 input was a wakeup source"]
    _1,
}
impl WUF24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF24W::_0 => false,
            WUF24W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF24W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P24 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF24W::_0)
    }
    #[doc = "LLWU_P24 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF24W::_1)
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
#[doc = "Values that can be written to the field `WUF25`"]
pub enum WUF25W {
    #[doc = "LLWU_P25 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P25 input was a wakeup source"]
    _1,
}
impl WUF25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF25W::_0 => false,
            WUF25W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF25W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P25 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF25W::_0)
    }
    #[doc = "LLWU_P25 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF25W::_1)
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
#[doc = "Values that can be written to the field `WUF26`"]
pub enum WUF26W {
    #[doc = "LLWU_P26 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P26 input was a wakeup source"]
    _1,
}
impl WUF26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF26W::_0 => false,
            WUF26W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF26W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P26 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF26W::_0)
    }
    #[doc = "LLWU_P26 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF26W::_1)
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
#[doc = "Values that can be written to the field `WUF27`"]
pub enum WUF27W {
    #[doc = "LLWU_P27 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P27 input was a wakeup source"]
    _1,
}
impl WUF27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF27W::_0 => false,
            WUF27W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF27W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P27 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF27W::_0)
    }
    #[doc = "LLWU_P27 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF27W::_1)
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
#[doc = "Values that can be written to the field `WUF28`"]
pub enum WUF28W {
    #[doc = "LLWU_P28 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P28 input was a wakeup source"]
    _1,
}
impl WUF28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF28W::_0 => false,
            WUF28W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF28W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P28 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF28W::_0)
    }
    #[doc = "LLWU_P28 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF28W::_1)
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
#[doc = "Values that can be written to the field `WUF29`"]
pub enum WUF29W {
    #[doc = "LLWU_P29 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P29 input was a wakeup source"]
    _1,
}
impl WUF29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF29W::_0 => false,
            WUF29W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF29W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P29 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF29W::_0)
    }
    #[doc = "LLWU_P29 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF29W::_1)
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
#[doc = "Values that can be written to the field `WUF30`"]
pub enum WUF30W {
    #[doc = "LLWU_P30 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P30 input was a wakeup source"]
    _1,
}
impl WUF30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF30W::_0 => false,
            WUF30W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF30W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P30 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF30W::_0)
    }
    #[doc = "LLWU_P30 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF30W::_1)
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
#[doc = "Values that can be written to the field `WUF31`"]
pub enum WUF31W {
    #[doc = "LLWU_P31 input was not a wakeup source"]
    _0,
    #[doc = "LLWU_P31 input was a wakeup source"]
    _1,
}
impl WUF31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUF31W::_0 => false,
            WUF31W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUF31W<'a> {
    w: &'a mut W,
}
impl<'a> _WUF31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUF31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LLWU_P31 input was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF31W::_0)
    }
    #[doc = "LLWU_P31 input was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF31W::_1)
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P24"]
    #[inline]
    pub fn wuf24(&self) -> WUF24R {
        WUF24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P25"]
    #[inline]
    pub fn wuf25(&self) -> WUF25R {
        WUF25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P26"]
    #[inline]
    pub fn wuf26(&self) -> WUF26R {
        WUF26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P27"]
    #[inline]
    pub fn wuf27(&self) -> WUF27R {
        WUF27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P28"]
    #[inline]
    pub fn wuf28(&self) -> WUF28R {
        WUF28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P29"]
    #[inline]
    pub fn wuf29(&self) -> WUF29R {
        WUF29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P30"]
    #[inline]
    pub fn wuf30(&self) -> WUF30R {
        WUF30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P31"]
    #[inline]
    pub fn wuf31(&self) -> WUF31R {
        WUF31R::_from({
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P24"]
    #[inline]
    pub fn wuf24(&mut self) -> _WUF24W {
        _WUF24W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P25"]
    #[inline]
    pub fn wuf25(&mut self) -> _WUF25W {
        _WUF25W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P26"]
    #[inline]
    pub fn wuf26(&mut self) -> _WUF26W {
        _WUF26W { w: self }
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P27"]
    #[inline]
    pub fn wuf27(&mut self) -> _WUF27W {
        _WUF27W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P28"]
    #[inline]
    pub fn wuf28(&mut self) -> _WUF28W {
        _WUF28W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P29"]
    #[inline]
    pub fn wuf29(&mut self) -> _WUF29W {
        _WUF29W { w: self }
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P30"]
    #[inline]
    pub fn wuf30(&mut self) -> _WUF30W {
        _WUF30W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P31"]
    #[inline]
    pub fn wuf31(&mut self) -> _WUF31W {
        _WUF31W { w: self }
    }
}
