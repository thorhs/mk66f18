#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WAR {
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
#[doc = "Possible values of the field `TSRW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSRWR {
    #[doc = "Writes to the Time Seconds Register are ignored."]
    _0,
    #[doc = "Writes to the Time Seconds Register complete as normal."]
    _1,
}
impl TSRWR {
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
            TSRWR::_0 => false,
            TSRWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSRWR {
        match value {
            false => TSRWR::_0,
            true => TSRWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSRWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSRWR::_1
    }
}
#[doc = "Possible values of the field `TPRW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPRWR {
    #[doc = "Writes to the Time Prescaler Register are ignored."]
    _0,
    #[doc = "Writes to the Time Prescaler Register complete as normal."]
    _1,
}
impl TPRWR {
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
            TPRWR::_0 => false,
            TPRWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPRWR {
        match value {
            false => TPRWR::_0,
            true => TPRWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPRWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPRWR::_1
    }
}
#[doc = "Possible values of the field `TARW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TARWR {
    #[doc = "Writes to the Time Alarm Register are ignored."]
    _0,
    #[doc = "Writes to the Time Alarm Register complete as normal."]
    _1,
}
impl TARWR {
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
            TARWR::_0 => false,
            TARWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TARWR {
        match value {
            false => TARWR::_0,
            true => TARWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TARWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TARWR::_1
    }
}
#[doc = "Possible values of the field `TCRW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCRWR {
    #[doc = "Writes to the Time Compensation Register are ignored."]
    _0,
    #[doc = "Writes to the Time Compensation Register complete as normal."]
    _1,
}
impl TCRWR {
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
            TCRWR::_0 => false,
            TCRWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCRWR {
        match value {
            false => TCRWR::_0,
            true => TCRWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCRWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCRWR::_1
    }
}
#[doc = "Possible values of the field `CRW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRWR {
    #[doc = "Writes to the Control Register are ignored."]
    _0,
    #[doc = "Writes to the Control Register complete as normal."]
    _1,
}
impl CRWR {
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
            CRWR::_0 => false,
            CRWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRWR {
        match value {
            false => CRWR::_0,
            true => CRWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CRWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CRWR::_1
    }
}
#[doc = "Possible values of the field `SRW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRWR {
    #[doc = "Writes to the Status Register are ignored."]
    _0,
    #[doc = "Writes to the Status Register complete as normal."]
    _1,
}
impl SRWR {
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
            SRWR::_0 => false,
            SRWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRWR {
        match value {
            false => SRWR::_0,
            true => SRWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SRWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SRWR::_1
    }
}
#[doc = "Possible values of the field `LRW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRWR {
    #[doc = "Writes to the Lock Register are ignored."]
    _0,
    #[doc = "Writes to the Lock Register complete as normal."]
    _1,
}
impl LRWR {
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
            LRWR::_0 => false,
            LRWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LRWR {
        match value {
            false => LRWR::_0,
            true => LRWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LRWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LRWR::_1
    }
}
#[doc = "Possible values of the field `IERW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IERWR {
    #[doc = "Writes to the Interupt Enable Register are ignored."]
    _0,
    #[doc = "Writes to the Interrupt Enable Register complete as normal."]
    _1,
}
impl IERWR {
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
            IERWR::_0 => false,
            IERWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IERWR {
        match value {
            false => IERWR::_0,
            true => IERWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IERWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IERWR::_1
    }
}
#[doc = "Possible values of the field `TTSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TTSWR {
    #[doc = "Writes to the Tamper Time Seconds Register are ignored."]
    _0,
    #[doc = "Writes to the Tamper Time Seconds Register complete as normal."]
    _1,
}
impl TTSWR {
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
            TTSWR::_0 => false,
            TTSWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TTSWR {
        match value {
            false => TTSWR::_0,
            true => TTSWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TTSWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TTSWR::_1
    }
}
#[doc = "Possible values of the field `MERW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MERWR {
    #[doc = "Writes to the Monotonic Enable Register are ignored."]
    _0,
    #[doc = "Writes to the Monotonic Enable Register complete as normal."]
    _1,
}
impl MERWR {
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
            MERWR::_0 => false,
            MERWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MERWR {
        match value {
            false => MERWR::_0,
            true => MERWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MERWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MERWR::_1
    }
}
#[doc = "Possible values of the field `MCLW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLWR {
    #[doc = "Writes to the Monotonic Counter Low Register are ignored."]
    _0,
    #[doc = "Writes to the Monotonic Counter Low Register complete as normal."]
    _1,
}
impl MCLWR {
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
            MCLWR::_0 => false,
            MCLWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCLWR {
        match value {
            false => MCLWR::_0,
            true => MCLWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MCLWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MCLWR::_1
    }
}
#[doc = "Possible values of the field `MCHW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCHWR {
    #[doc = "Writes to the Monotonic Counter High Register are ignored."]
    _0,
    #[doc = "Writes to the Monotonic Counter High Register complete as normal."]
    _1,
}
impl MCHWR {
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
            MCHWR::_0 => false,
            MCHWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MCHWR {
        match value {
            false => MCHWR::_0,
            true => MCHWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MCHWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MCHWR::_1
    }
}
#[doc = "Values that can be written to the field `TSRW`"]
pub enum TSRWW {
    #[doc = "Writes to the Time Seconds Register are ignored."]
    _0,
    #[doc = "Writes to the Time Seconds Register complete as normal."]
    _1,
}
impl TSRWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSRWW::_0 => false,
            TSRWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSRWW<'a> {
    w: &'a mut W,
}
impl<'a> _TSRWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSRWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the Time Seconds Register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSRWW::_0)
    }
    #[doc = "Writes to the Time Seconds Register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSRWW::_1)
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
#[doc = "Values that can be written to the field `TPRW`"]
pub enum TPRWW {
    #[doc = "Writes to the Time Prescaler Register are ignored."]
    _0,
    #[doc = "Writes to the Time Prescaler Register complete as normal."]
    _1,
}
impl TPRWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPRWW::_0 => false,
            TPRWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPRWW<'a> {
    w: &'a mut W,
}
impl<'a> _TPRWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPRWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the Time Prescaler Register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPRWW::_0)
    }
    #[doc = "Writes to the Time Prescaler Register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPRWW::_1)
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
#[doc = "Values that can be written to the field `TARW`"]
pub enum TARWW {
    #[doc = "Writes to the Time Alarm Register are ignored."]
    _0,
    #[doc = "Writes to the Time Alarm Register complete as normal."]
    _1,
}
impl TARWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TARWW::_0 => false,
            TARWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TARWW<'a> {
    w: &'a mut W,
}
impl<'a> _TARWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TARWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the Time Alarm Register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TARWW::_0)
    }
    #[doc = "Writes to the Time Alarm Register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TARWW::_1)
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
#[doc = "Values that can be written to the field `TCRW`"]
pub enum TCRWW {
    #[doc = "Writes to the Time Compensation Register are ignored."]
    _0,
    #[doc = "Writes to the Time Compensation Register complete as normal."]
    _1,
}
impl TCRWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCRWW::_0 => false,
            TCRWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCRWW<'a> {
    w: &'a mut W,
}
impl<'a> _TCRWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCRWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the Time Compensation Register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCRWW::_0)
    }
    #[doc = "Writes to the Time Compensation Register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCRWW::_1)
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
#[doc = "Values that can be written to the field `CRW`"]
pub enum CRWW {
    #[doc = "Writes to the Control Register are ignored."]
    _0,
    #[doc = "Writes to the Control Register complete as normal."]
    _1,
}
impl CRWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRWW::_0 => false,
            CRWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRWW<'a> {
    w: &'a mut W,
}
impl<'a> _CRWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the Control Register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRWW::_0)
    }
    #[doc = "Writes to the Control Register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRWW::_1)
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
#[doc = "Values that can be written to the field `SRW`"]
pub enum SRWW {
    #[doc = "Writes to the Status Register are ignored."]
    _0,
    #[doc = "Writes to the Status Register complete as normal."]
    _1,
}
impl SRWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRWW::_0 => false,
            SRWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRWW<'a> {
    w: &'a mut W,
}
impl<'a> _SRWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the Status Register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRWW::_0)
    }
    #[doc = "Writes to the Status Register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRWW::_1)
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
#[doc = "Values that can be written to the field `LRW`"]
pub enum LRWW {
    #[doc = "Writes to the Lock Register are ignored."]
    _0,
    #[doc = "Writes to the Lock Register complete as normal."]
    _1,
}
impl LRWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LRWW::_0 => false,
            LRWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LRWW<'a> {
    w: &'a mut W,
}
impl<'a> _LRWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LRWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the Lock Register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LRWW::_0)
    }
    #[doc = "Writes to the Lock Register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LRWW::_1)
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
#[doc = "Values that can be written to the field `IERW`"]
pub enum IERWW {
    #[doc = "Writes to the Interupt Enable Register are ignored."]
    _0,
    #[doc = "Writes to the Interrupt Enable Register complete as normal."]
    _1,
}
impl IERWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IERWW::_0 => false,
            IERWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IERWW<'a> {
    w: &'a mut W,
}
impl<'a> _IERWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IERWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the Interupt Enable Register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IERWW::_0)
    }
    #[doc = "Writes to the Interrupt Enable Register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IERWW::_1)
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
#[doc = "Values that can be written to the field `TTSW`"]
pub enum TTSWW {
    #[doc = "Writes to the Tamper Time Seconds Register are ignored."]
    _0,
    #[doc = "Writes to the Tamper Time Seconds Register complete as normal."]
    _1,
}
impl TTSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TTSWW::_0 => false,
            TTSWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TTSWW<'a> {
    w: &'a mut W,
}
impl<'a> _TTSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TTSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the Tamper Time Seconds Register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TTSWW::_0)
    }
    #[doc = "Writes to the Tamper Time Seconds Register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TTSWW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MERW`"]
pub enum MERWW {
    #[doc = "Writes to the Monotonic Enable Register are ignored."]
    _0,
    #[doc = "Writes to the Monotonic Enable Register complete as normal."]
    _1,
}
impl MERWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MERWW::_0 => false,
            MERWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MERWW<'a> {
    w: &'a mut W,
}
impl<'a> _MERWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MERWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the Monotonic Enable Register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MERWW::_0)
    }
    #[doc = "Writes to the Monotonic Enable Register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MERWW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCLW`"]
pub enum MCLWW {
    #[doc = "Writes to the Monotonic Counter Low Register are ignored."]
    _0,
    #[doc = "Writes to the Monotonic Counter Low Register complete as normal."]
    _1,
}
impl MCLWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCLWW::_0 => false,
            MCLWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCLWW<'a> {
    w: &'a mut W,
}
impl<'a> _MCLWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCLWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the Monotonic Counter Low Register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCLWW::_0)
    }
    #[doc = "Writes to the Monotonic Counter Low Register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCLWW::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MCHW`"]
pub enum MCHWW {
    #[doc = "Writes to the Monotonic Counter High Register are ignored."]
    _0,
    #[doc = "Writes to the Monotonic Counter High Register complete as normal."]
    _1,
}
impl MCHWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MCHWW::_0 => false,
            MCHWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCHWW<'a> {
    w: &'a mut W,
}
impl<'a> _MCHWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCHWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the Monotonic Counter High Register are ignored."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCHWW::_0)
    }
    #[doc = "Writes to the Monotonic Counter High Register complete as normal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCHWW::_1)
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Time Seconds Register Write"]
    #[inline]
    pub fn tsrw(&self) -> TSRWR {
        TSRWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Time Prescaler Register Write"]
    #[inline]
    pub fn tprw(&self) -> TPRWR {
        TPRWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Time Alarm Register Write"]
    #[inline]
    pub fn tarw(&self) -> TARWR {
        TARWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Time Compensation Register Write"]
    #[inline]
    pub fn tcrw(&self) -> TCRWR {
        TCRWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Control Register Write"]
    #[inline]
    pub fn crw(&self) -> CRWR {
        CRWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Status Register Write"]
    #[inline]
    pub fn srw(&self) -> SRWR {
        SRWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Lock Register Write"]
    #[inline]
    pub fn lrw(&self) -> LRWR {
        LRWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Interrupt Enable Register Write"]
    #[inline]
    pub fn ierw(&self) -> IERWR {
        IERWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Tamper Time Seconds Write"]
    #[inline]
    pub fn ttsw(&self) -> TTSWR {
        TTSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Monotonic Enable Register Write"]
    #[inline]
    pub fn merw(&self) -> MERWR {
        MERWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Monotonic Counter Low Write"]
    #[inline]
    pub fn mclw(&self) -> MCLWR {
        MCLWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Monotonic Counter High Write"]
    #[inline]
    pub fn mchw(&self) -> MCHWR {
        MCHWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Time Seconds Register Write"]
    #[inline]
    pub fn tsrw(&mut self) -> _TSRWW {
        _TSRWW { w: self }
    }
    #[doc = "Bit 1 - Time Prescaler Register Write"]
    #[inline]
    pub fn tprw(&mut self) -> _TPRWW {
        _TPRWW { w: self }
    }
    #[doc = "Bit 2 - Time Alarm Register Write"]
    #[inline]
    pub fn tarw(&mut self) -> _TARWW {
        _TARWW { w: self }
    }
    #[doc = "Bit 3 - Time Compensation Register Write"]
    #[inline]
    pub fn tcrw(&mut self) -> _TCRWW {
        _TCRWW { w: self }
    }
    #[doc = "Bit 4 - Control Register Write"]
    #[inline]
    pub fn crw(&mut self) -> _CRWW {
        _CRWW { w: self }
    }
    #[doc = "Bit 5 - Status Register Write"]
    #[inline]
    pub fn srw(&mut self) -> _SRWW {
        _SRWW { w: self }
    }
    #[doc = "Bit 6 - Lock Register Write"]
    #[inline]
    pub fn lrw(&mut self) -> _LRWW {
        _LRWW { w: self }
    }
    #[doc = "Bit 7 - Interrupt Enable Register Write"]
    #[inline]
    pub fn ierw(&mut self) -> _IERWW {
        _IERWW { w: self }
    }
    #[doc = "Bit 8 - Tamper Time Seconds Write"]
    #[inline]
    pub fn ttsw(&mut self) -> _TTSWW {
        _TTSWW { w: self }
    }
    #[doc = "Bit 9 - Monotonic Enable Register Write"]
    #[inline]
    pub fn merw(&mut self) -> _MERWW {
        _MERWW { w: self }
    }
    #[doc = "Bit 10 - Monotonic Counter Low Write"]
    #[inline]
    pub fn mclw(&mut self) -> _MCLWW {
        _MCLWW { w: self }
    }
    #[doc = "Bit 11 - Monotonic Counter High Write"]
    #[inline]
    pub fn mchw(&mut self) -> _MCHWW {
        _MCHWW { w: self }
    }
}
