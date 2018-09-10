#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::ME {
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
#[doc = "Possible values of the field `WUME0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME0R {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME0R {
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
            WUME0R::_0 => false,
            WUME0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUME0R {
        match value {
            false => WUME0R::_0,
            true => WUME0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUME0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUME0R::_1
    }
}
#[doc = "Possible values of the field `WUME1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME1R {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME1R {
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
            WUME1R::_0 => false,
            WUME1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUME1R {
        match value {
            false => WUME1R::_0,
            true => WUME1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUME1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUME1R::_1
    }
}
#[doc = "Possible values of the field `WUME2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME2R {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME2R {
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
            WUME2R::_0 => false,
            WUME2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUME2R {
        match value {
            false => WUME2R::_0,
            true => WUME2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUME2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUME2R::_1
    }
}
#[doc = "Possible values of the field `WUME3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME3R {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME3R {
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
            WUME3R::_0 => false,
            WUME3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUME3R {
        match value {
            false => WUME3R::_0,
            true => WUME3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUME3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUME3R::_1
    }
}
#[doc = "Possible values of the field `WUME4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME4R {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME4R {
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
            WUME4R::_0 => false,
            WUME4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUME4R {
        match value {
            false => WUME4R::_0,
            true => WUME4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUME4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUME4R::_1
    }
}
#[doc = "Possible values of the field `WUME5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME5R {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME5R {
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
            WUME5R::_0 => false,
            WUME5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUME5R {
        match value {
            false => WUME5R::_0,
            true => WUME5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUME5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUME5R::_1
    }
}
#[doc = "Possible values of the field `WUME6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME6R {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME6R {
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
            WUME6R::_0 => false,
            WUME6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUME6R {
        match value {
            false => WUME6R::_0,
            true => WUME6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUME6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUME6R::_1
    }
}
#[doc = "Possible values of the field `WUME7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUME7R {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME7R {
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
            WUME7R::_0 => false,
            WUME7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUME7R {
        match value {
            false => WUME7R::_0,
            true => WUME7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUME7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUME7R::_1
    }
}
#[doc = "Values that can be written to the field `WUME0`"]
pub enum WUME0W {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUME0W::_0 => false,
            WUME0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUME0W<'a> {
    w: &'a mut W,
}
impl<'a> _WUME0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUME0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME0W::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME0W::_1)
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
#[doc = "Values that can be written to the field `WUME1`"]
pub enum WUME1W {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUME1W::_0 => false,
            WUME1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUME1W<'a> {
    w: &'a mut W,
}
impl<'a> _WUME1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUME1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME1W::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME1W::_1)
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
#[doc = "Values that can be written to the field `WUME2`"]
pub enum WUME2W {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUME2W::_0 => false,
            WUME2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUME2W<'a> {
    w: &'a mut W,
}
impl<'a> _WUME2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUME2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME2W::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME2W::_1)
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
#[doc = "Values that can be written to the field `WUME3`"]
pub enum WUME3W {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUME3W::_0 => false,
            WUME3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUME3W<'a> {
    w: &'a mut W,
}
impl<'a> _WUME3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUME3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME3W::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME3W::_1)
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
#[doc = "Values that can be written to the field `WUME4`"]
pub enum WUME4W {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUME4W::_0 => false,
            WUME4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUME4W<'a> {
    w: &'a mut W,
}
impl<'a> _WUME4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUME4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME4W::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME4W::_1)
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
#[doc = "Values that can be written to the field `WUME5`"]
pub enum WUME5W {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUME5W::_0 => false,
            WUME5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUME5W<'a> {
    w: &'a mut W,
}
impl<'a> _WUME5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUME5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME5W::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME5W::_1)
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
#[doc = "Values that can be written to the field `WUME6`"]
pub enum WUME6W {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUME6W::_0 => false,
            WUME6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUME6W<'a> {
    w: &'a mut W,
}
impl<'a> _WUME6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUME6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME6W::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME6W::_1)
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
#[doc = "Values that can be written to the field `WUME7`"]
pub enum WUME7W {
    #[doc = "Internal module flag not used as wakeup source"]
    _0,
    #[doc = "Internal module flag used as wakeup source"]
    _1,
}
impl WUME7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUME7W::_0 => false,
            WUME7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUME7W<'a> {
    w: &'a mut W,
}
impl<'a> _WUME7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUME7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal module flag not used as wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUME7W::_0)
    }
    #[doc = "Internal module flag used as wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUME7W::_1)
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
    #[doc = "Bit 0 - Wakeup Module Enable For Module 0"]
    #[inline]
    pub fn wume0(&self) -> WUME0R {
        WUME0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Wakeup Module Enable for Module 1"]
    #[inline]
    pub fn wume1(&self) -> WUME1R {
        WUME1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Wakeup Module Enable For Module 2"]
    #[inline]
    pub fn wume2(&self) -> WUME2R {
        WUME2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Wakeup Module Enable For Module 3"]
    #[inline]
    pub fn wume3(&self) -> WUME3R {
        WUME3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Wakeup Module Enable For Module 4"]
    #[inline]
    pub fn wume4(&self) -> WUME4R {
        WUME4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Wakeup Module Enable For Module 5"]
    #[inline]
    pub fn wume5(&self) -> WUME5R {
        WUME5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Wakeup Module Enable For Module 6"]
    #[inline]
    pub fn wume6(&self) -> WUME6R {
        WUME6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Wakeup Module Enable For Module 7"]
    #[inline]
    pub fn wume7(&self) -> WUME7R {
        WUME7R::_from({
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
    #[doc = "Bit 0 - Wakeup Module Enable For Module 0"]
    #[inline]
    pub fn wume0(&mut self) -> _WUME0W {
        _WUME0W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Module Enable for Module 1"]
    #[inline]
    pub fn wume1(&mut self) -> _WUME1W {
        _WUME1W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Module Enable For Module 2"]
    #[inline]
    pub fn wume2(&mut self) -> _WUME2W {
        _WUME2W { w: self }
    }
    #[doc = "Bit 3 - Wakeup Module Enable For Module 3"]
    #[inline]
    pub fn wume3(&mut self) -> _WUME3W {
        _WUME3W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Module Enable For Module 4"]
    #[inline]
    pub fn wume4(&mut self) -> _WUME4W {
        _WUME4W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Module Enable For Module 5"]
    #[inline]
    pub fn wume5(&mut self) -> _WUME5W {
        _WUME5W { w: self }
    }
    #[doc = "Bit 6 - Wakeup Module Enable For Module 6"]
    #[inline]
    pub fn wume6(&mut self) -> _WUME6W {
        _WUME6W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Module Enable For Module 7"]
    #[inline]
    pub fn wume7(&mut self) -> _WUME7W {
        _WUME7W { w: self }
    }
}
