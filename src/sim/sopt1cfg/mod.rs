#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT1CFG {
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
#[doc = "Possible values of the field `URWE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URWER {
    #[doc = "SOPT1 USBREGEN cannot be written."]
    _0,
    #[doc = "SOPT1 USBREGEN can be written."]
    _1,
}
impl URWER {
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
            URWER::_0 => false,
            URWER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> URWER {
        match value {
            false => URWER::_0,
            true => URWER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == URWER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == URWER::_1
    }
}
#[doc = "Possible values of the field `UVSWE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVSWER {
    #[doc = "SOPT1 USBVSTBY cannot be written."]
    _0,
    #[doc = "SOPT1 USBVSTBY can be written."]
    _1,
}
impl UVSWER {
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
            UVSWER::_0 => false,
            UVSWER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UVSWER {
        match value {
            false => UVSWER::_0,
            true => UVSWER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UVSWER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UVSWER::_1
    }
}
#[doc = "Possible values of the field `USSWE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USSWER {
    #[doc = "SOPT1 USBSSTBY cannot be written."]
    _0,
    #[doc = "SOPT1 USBSSTBY can be written."]
    _1,
}
impl USSWER {
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
            USSWER::_0 => false,
            USSWER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USSWER {
        match value {
            false => USSWER::_0,
            true => USSWER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USSWER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USSWER::_1
    }
}
#[doc = "Values that can be written to the field `URWE`"]
pub enum URWEW {
    #[doc = "SOPT1 USBREGEN cannot be written."]
    _0,
    #[doc = "SOPT1 USBREGEN can be written."]
    _1,
}
impl URWEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            URWEW::_0 => false,
            URWEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _URWEW<'a> {
    w: &'a mut W,
}
impl<'a> _URWEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: URWEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SOPT1 USBREGEN cannot be written."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(URWEW::_0)
    }
    #[doc = "SOPT1 USBREGEN can be written."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(URWEW::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UVSWE`"]
pub enum UVSWEW {
    #[doc = "SOPT1 USBVSTBY cannot be written."]
    _0,
    #[doc = "SOPT1 USBVSTBY can be written."]
    _1,
}
impl UVSWEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UVSWEW::_0 => false,
            UVSWEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UVSWEW<'a> {
    w: &'a mut W,
}
impl<'a> _UVSWEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UVSWEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SOPT1 USBVSTBY cannot be written."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UVSWEW::_0)
    }
    #[doc = "SOPT1 USBVSTBY can be written."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UVSWEW::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USSWE`"]
pub enum USSWEW {
    #[doc = "SOPT1 USBSSTBY cannot be written."]
    _0,
    #[doc = "SOPT1 USBSSTBY can be written."]
    _1,
}
impl USSWEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USSWEW::_0 => false,
            USSWEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USSWEW<'a> {
    w: &'a mut W,
}
impl<'a> _USSWEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USSWEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SOPT1 USBSSTBY cannot be written."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USSWEW::_0)
    }
    #[doc = "SOPT1 USBSSTBY can be written."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USSWEW::_1)
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 24 - USB voltage regulator enable write enable"]
    #[inline]
    pub fn urwe(&self) -> URWER {
        URWER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - USB voltage regulator VLP standby write enable"]
    #[inline]
    pub fn uvswe(&self) -> UVSWER {
        UVSWER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - USB voltage regulator stop standby write enable"]
    #[inline]
    pub fn usswe(&self) -> USSWER {
        USSWER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
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
    #[doc = "Bit 24 - USB voltage regulator enable write enable"]
    #[inline]
    pub fn urwe(&mut self) -> _URWEW {
        _URWEW { w: self }
    }
    #[doc = "Bit 25 - USB voltage regulator VLP standby write enable"]
    #[inline]
    pub fn uvswe(&mut self) -> _UVSWEW {
        _UVSWEW { w: self }
    }
    #[doc = "Bit 26 - USB voltage regulator stop standby write enable"]
    #[inline]
    pub fn usswe(&mut self) -> _USSWEW {
        _USSWEW { w: self }
    }
}
