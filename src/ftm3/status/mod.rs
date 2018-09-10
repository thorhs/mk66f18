#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATUS {
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
#[doc = "Possible values of the field `CH0F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0FR {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH0FR {
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
            CH0FR::_0 => false,
            CH0FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0FR {
        match value {
            false => CH0FR::_0,
            true => CH0FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH0FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH0FR::_1
    }
}
#[doc = "Possible values of the field `CH1F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1FR {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH1FR {
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
            CH1FR::_0 => false,
            CH1FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1FR {
        match value {
            false => CH1FR::_0,
            true => CH1FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH1FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH1FR::_1
    }
}
#[doc = "Possible values of the field `CH2F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2FR {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH2FR {
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
            CH2FR::_0 => false,
            CH2FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2FR {
        match value {
            false => CH2FR::_0,
            true => CH2FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH2FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH2FR::_1
    }
}
#[doc = "Possible values of the field `CH3F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3FR {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH3FR {
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
            CH3FR::_0 => false,
            CH3FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3FR {
        match value {
            false => CH3FR::_0,
            true => CH3FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH3FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH3FR::_1
    }
}
#[doc = "Possible values of the field `CH4F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4FR {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH4FR {
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
            CH4FR::_0 => false,
            CH4FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4FR {
        match value {
            false => CH4FR::_0,
            true => CH4FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH4FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH4FR::_1
    }
}
#[doc = "Possible values of the field `CH5F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5FR {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH5FR {
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
            CH5FR::_0 => false,
            CH5FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5FR {
        match value {
            false => CH5FR::_0,
            true => CH5FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH5FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH5FR::_1
    }
}
#[doc = "Possible values of the field `CH6F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6FR {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH6FR {
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
            CH6FR::_0 => false,
            CH6FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6FR {
        match value {
            false => CH6FR::_0,
            true => CH6FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH6FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH6FR::_1
    }
}
#[doc = "Possible values of the field `CH7F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7FR {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH7FR {
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
            CH7FR::_0 => false,
            CH7FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7FR {
        match value {
            false => CH7FR::_0,
            true => CH7FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH7FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH7FR::_1
    }
}
#[doc = "Values that can be written to the field `CH0F`"]
pub enum CH0FW {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH0FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0FW::_0 => false,
            CH0FW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0FW::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0FW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH1F`"]
pub enum CH1FW {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH1FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1FW::_0 => false,
            CH1FW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1FW::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1FW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH2F`"]
pub enum CH2FW {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH2FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2FW::_0 => false,
            CH2FW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2FW::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2FW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH3F`"]
pub enum CH3FW {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH3FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3FW::_0 => false,
            CH3FW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3FW::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3FW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH4F`"]
pub enum CH4FW {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH4FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4FW::_0 => false,
            CH4FW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4FW::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4FW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH5F`"]
pub enum CH5FW {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH5FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5FW::_0 => false,
            CH5FW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5FW::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5FW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH6F`"]
pub enum CH6FW {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH6FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6FW::_0 => false,
            CH6FW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6FW::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6FW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH7F`"]
pub enum CH7FW {
    #[doc = "No channel event has occurred."]
    _0,
    #[doc = "A channel event has occurred."]
    _1,
}
impl CH7FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7FW::_0 => false,
            CH7FW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7FW::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7FW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel 0 Flag"]
    #[inline]
    pub fn ch0f(&self) -> CH0FR {
        CH0FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Flag"]
    #[inline]
    pub fn ch1f(&self) -> CH1FR {
        CH1FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Flag"]
    #[inline]
    pub fn ch2f(&self) -> CH2FR {
        CH2FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Flag"]
    #[inline]
    pub fn ch3f(&self) -> CH3FR {
        CH3FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 4 Flag"]
    #[inline]
    pub fn ch4f(&self) -> CH4FR {
        CH4FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 5 Flag"]
    #[inline]
    pub fn ch5f(&self) -> CH5FR {
        CH5FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel 6 Flag"]
    #[inline]
    pub fn ch6f(&self) -> CH6FR {
        CH6FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel 7 Flag"]
    #[inline]
    pub fn ch7f(&self) -> CH7FR {
        CH7FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Channel 0 Flag"]
    #[inline]
    pub fn ch0f(&mut self) -> _CH0FW {
        _CH0FW { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Flag"]
    #[inline]
    pub fn ch1f(&mut self) -> _CH1FW {
        _CH1FW { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Flag"]
    #[inline]
    pub fn ch2f(&mut self) -> _CH2FW {
        _CH2FW { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Flag"]
    #[inline]
    pub fn ch3f(&mut self) -> _CH3FW {
        _CH3FW { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Flag"]
    #[inline]
    pub fn ch4f(&mut self) -> _CH4FW {
        _CH4FW { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Flag"]
    #[inline]
    pub fn ch5f(&mut self) -> _CH5FW {
        _CH5FW { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Flag"]
    #[inline]
    pub fn ch6f(&mut self) -> _CH6FW {
        _CH6FW { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Flag"]
    #[inline]
    pub fn ch7f(&mut self) -> _CH7FW {
        _CH7FW { w: self }
    }
}
