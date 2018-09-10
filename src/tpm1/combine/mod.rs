#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::COMBINE {
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
#[doc = "Possible values of the field `COMBINE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBINE0R {
    #[doc = "Channels 0 and 1 are independent."]
    _0,
    #[doc = "Channels 0 and 1 are combined."]
    _1,
}
impl COMBINE0R {
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
            COMBINE0R::_0 => false,
            COMBINE0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMBINE0R {
        match value {
            false => COMBINE0R::_0,
            true => COMBINE0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COMBINE0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COMBINE0R::_1
    }
}
#[doc = "Possible values of the field `COMSWAP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMSWAP0R {
    #[doc = "Even channel is used for input capture and 1st compare."]
    _0,
    #[doc = "Odd channel is used for input capture and 1st compare."]
    _1,
}
impl COMSWAP0R {
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
            COMSWAP0R::_0 => false,
            COMSWAP0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMSWAP0R {
        match value {
            false => COMSWAP0R::_0,
            true => COMSWAP0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COMSWAP0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COMSWAP0R::_1
    }
}
#[doc = "Values that can be written to the field `COMBINE0`"]
pub enum COMBINE0W {
    #[doc = "Channels 0 and 1 are independent."]
    _0,
    #[doc = "Channels 0 and 1 are combined."]
    _1,
}
impl COMBINE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMBINE0W::_0 => false,
            COMBINE0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMBINE0W<'a> {
    w: &'a mut W,
}
impl<'a> _COMBINE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMBINE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channels 0 and 1 are independent."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMBINE0W::_0)
    }
    #[doc = "Channels 0 and 1 are combined."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMBINE0W::_1)
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
#[doc = "Values that can be written to the field `COMSWAP0`"]
pub enum COMSWAP0W {
    #[doc = "Even channel is used for input capture and 1st compare."]
    _0,
    #[doc = "Odd channel is used for input capture and 1st compare."]
    _1,
}
impl COMSWAP0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMSWAP0W::_0 => false,
            COMSWAP0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMSWAP0W<'a> {
    w: &'a mut W,
}
impl<'a> _COMSWAP0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMSWAP0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Even channel is used for input capture and 1st compare."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMSWAP0W::_0)
    }
    #[doc = "Odd channel is used for input capture and 1st compare."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMSWAP0W::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Combine Channels 0 and 1"]
    #[inline]
    pub fn combine0(&self) -> COMBINE0R {
        COMBINE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Combine Channel 0 and 1 Swap"]
    #[inline]
    pub fn comswap0(&self) -> COMSWAP0R {
        COMSWAP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Combine Channels 0 and 1"]
    #[inline]
    pub fn combine0(&mut self) -> _COMBINE0W {
        _COMBINE0W { w: self }
    }
    #[doc = "Bit 1 - Combine Channel 0 and 1 Swap"]
    #[inline]
    pub fn comswap0(&mut self) -> _COMSWAP0W {
        _COMSWAP0W { w: self }
    }
}
