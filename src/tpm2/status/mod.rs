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
#[doc = "Possible values of the field `TOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOFR {
    #[doc = "TPM counter has not overflowed."]
    _0,
    #[doc = "TPM counter has overflowed."]
    _1,
}
impl TOFR {
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
            TOFR::_0 => false,
            TOFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOFR {
        match value {
            false => TOFR::_0,
            true => TOFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOFR::_1
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
#[doc = "Values that can be written to the field `TOF`"]
pub enum TOFW {
    #[doc = "TPM counter has not overflowed."]
    _0,
    #[doc = "TPM counter has overflowed."]
    _1,
}
impl TOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOFW::_0 => false,
            TOFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOFW<'a> {
    w: &'a mut W,
}
impl<'a> _TOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TPM counter has not overflowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOFW::_0)
    }
    #[doc = "TPM counter has overflowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOFW::_1)
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
    #[doc = "Bit 8 - Timer Overflow Flag"]
    #[inline]
    pub fn tof(&self) -> TOFR {
        TOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 8 - Timer Overflow Flag"]
    #[inline]
    pub fn tof(&mut self) -> _TOFW {
        _TOFW { w: self }
    }
}
