#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SR {
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
#[doc = "Possible values of the field `DACBFRPBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFRPBFR {
    #[doc = "The DAC buffer read pointer is not equal to C2\\[DACBFUP\\]."]
    _0,
    #[doc = "The DAC buffer read pointer is equal to C2\\[DACBFUP\\]."]
    _1,
}
impl DACBFRPBFR {
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
            DACBFRPBFR::_0 => false,
            DACBFRPBFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACBFRPBFR {
        match value {
            false => DACBFRPBFR::_0,
            true => DACBFRPBFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DACBFRPBFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DACBFRPBFR::_1
    }
}
#[doc = "Possible values of the field `DACBFRPTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFRPTFR {
    #[doc = "The DAC buffer read pointer is not zero."]
    _0,
    #[doc = "The DAC buffer read pointer is zero."]
    _1,
}
impl DACBFRPTFR {
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
            DACBFRPTFR::_0 => false,
            DACBFRPTFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACBFRPTFR {
        match value {
            false => DACBFRPTFR::_0,
            true => DACBFRPTFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DACBFRPTFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DACBFRPTFR::_1
    }
}
#[doc = "Possible values of the field `DACBFWMF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFWMFR {
    #[doc = "The DAC buffer read pointer has not reached the watermark level."]
    _0,
    #[doc = "The DAC buffer read pointer has reached the watermark level."]
    _1,
}
impl DACBFWMFR {
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
            DACBFWMFR::_0 => false,
            DACBFWMFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACBFWMFR {
        match value {
            false => DACBFWMFR::_0,
            true => DACBFWMFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DACBFWMFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DACBFWMFR::_1
    }
}
#[doc = "Values that can be written to the field `DACBFRPBF`"]
pub enum DACBFRPBFW {
    #[doc = "The DAC buffer read pointer is not equal to C2\\[DACBFUP\\]."]
    _0,
    #[doc = "The DAC buffer read pointer is equal to C2\\[DACBFUP\\]."]
    _1,
}
impl DACBFRPBFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACBFRPBFW::_0 => false,
            DACBFRPBFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACBFRPBFW<'a> {
    w: &'a mut W,
}
impl<'a> _DACBFRPBFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACBFRPBFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DAC buffer read pointer is not equal to C2\\[DACBFUP\\]."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFRPBFW::_0)
    }
    #[doc = "The DAC buffer read pointer is equal to C2\\[DACBFUP\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFRPBFW::_1)
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
#[doc = "Values that can be written to the field `DACBFRPTF`"]
pub enum DACBFRPTFW {
    #[doc = "The DAC buffer read pointer is not zero."]
    _0,
    #[doc = "The DAC buffer read pointer is zero."]
    _1,
}
impl DACBFRPTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACBFRPTFW::_0 => false,
            DACBFRPTFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACBFRPTFW<'a> {
    w: &'a mut W,
}
impl<'a> _DACBFRPTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACBFRPTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DAC buffer read pointer is not zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFRPTFW::_0)
    }
    #[doc = "The DAC buffer read pointer is zero."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFRPTFW::_1)
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
#[doc = "Values that can be written to the field `DACBFWMF`"]
pub enum DACBFWMFW {
    #[doc = "The DAC buffer read pointer has not reached the watermark level."]
    _0,
    #[doc = "The DAC buffer read pointer has reached the watermark level."]
    _1,
}
impl DACBFWMFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACBFWMFW::_0 => false,
            DACBFWMFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACBFWMFW<'a> {
    w: &'a mut W,
}
impl<'a> _DACBFWMFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACBFWMFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DAC buffer read pointer has not reached the watermark level."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFWMFW::_0)
    }
    #[doc = "The DAC buffer read pointer has reached the watermark level."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFWMFW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Position Flag"]
    #[inline]
    pub fn dacbfrpbf(&self) -> DACBFRPBFR {
        DACBFRPBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Position Flag"]
    #[inline]
    pub fn dacbfrptf(&self) -> DACBFRPTFR {
        DACBFRPTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - DAC Buffer Watermark Flag"]
    #[inline]
    pub fn dacbfwmf(&self) -> DACBFWMFR {
        DACBFWMFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Position Flag"]
    #[inline]
    pub fn dacbfrpbf(&mut self) -> _DACBFRPBFW {
        _DACBFRPBFW { w: self }
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Position Flag"]
    #[inline]
    pub fn dacbfrptf(&mut self) -> _DACBFRPTFW {
        _DACBFRPTFW { w: self }
    }
    #[doc = "Bit 2 - DAC Buffer Watermark Flag"]
    #[inline]
    pub fn dacbfwmf(&mut self) -> _DACBFWMFW {
        _DACBFWMFW { w: self }
    }
}
