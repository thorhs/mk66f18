#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C4 {
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
#[doc = r" Value of the field"]
pub struct BRFAR {
    bits: u8,
}
impl BRFAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `M10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M10R {
    #[doc = "The parity bit is the ninth bit in the serial transmission."]
    _0,
    #[doc = "The parity bit is the tenth bit in the serial transmission."]
    _1,
}
impl M10R {
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
            M10R::_0 => false,
            M10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M10R {
        match value {
            false => M10R::_0,
            true => M10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M10R::_1
    }
}
#[doc = "Possible values of the field `MAEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN2R {
    #[doc = "All data received is transferred to the data buffer if MAEN1 is cleared."]
    _0,
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If a match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\] is set/enabled."]
    _1,
}
impl MAEN2R {
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
            MAEN2R::_0 => false,
            MAEN2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAEN2R {
        match value {
            false => MAEN2R::_0,
            true => MAEN2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MAEN2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MAEN2R::_1
    }
}
#[doc = "Possible values of the field `MAEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN1R {
    #[doc = "All data received is transferred to the data buffer if MAEN2 is cleared."]
    _0,
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\] is set/enabled."]
    _1,
}
impl MAEN1R {
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
            MAEN1R::_0 => false,
            MAEN1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAEN1R {
        match value {
            false => MAEN1R::_0,
            true => MAEN1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MAEN1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MAEN1R::_1
    }
}
#[doc = r" Proxy"]
pub struct _BRFAW<'a> {
    w: &'a mut W,
}
impl<'a> _BRFAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M10`"]
pub enum M10W {
    #[doc = "The parity bit is the ninth bit in the serial transmission."]
    _0,
    #[doc = "The parity bit is the tenth bit in the serial transmission."]
    _1,
}
impl M10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M10W::_0 => false,
            M10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M10W<'a> {
    w: &'a mut W,
}
impl<'a> _M10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The parity bit is the ninth bit in the serial transmission."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M10W::_0)
    }
    #[doc = "The parity bit is the tenth bit in the serial transmission."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M10W::_1)
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
#[doc = "Values that can be written to the field `MAEN2`"]
pub enum MAEN2W {
    #[doc = "All data received is transferred to the data buffer if MAEN1 is cleared."]
    _0,
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If a match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\] is set/enabled."]
    _1,
}
impl MAEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAEN2W::_0 => false,
            MAEN2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _MAEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All data received is transferred to the data buffer if MAEN1 is cleared."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN2W::_0)
    }
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA2 register. If no match occurs, the data is discarded. If a match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\] is set/enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN2W::_1)
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
#[doc = "Values that can be written to the field `MAEN1`"]
pub enum MAEN1W {
    #[doc = "All data received is transferred to the data buffer if MAEN2 is cleared."]
    _0,
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\] is set/enabled."]
    _1,
}
impl MAEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAEN1W::_0 => false,
            MAEN1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _MAEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All data received is transferred to the data buffer if MAEN2 is cleared."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN1W::_0)
    }
    #[doc = "All data received with the most significant bit cleared, is discarded. All data received with the most significant bit set, is compared with contents of MA1 register. If no match occurs, the data is discarded. If match occurs, data is transferred to the data buffer. This field must be cleared when C7816\\[ISO7816E\\] is set/enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN1W::_1)
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
    #[doc = "Bits 0:4 - Baud Rate Fine Adjust"]
    #[inline]
    pub fn brfa(&self) -> BRFAR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        BRFAR { bits }
    }
    #[doc = "Bit 5 - 10-bit Mode select"]
    #[inline]
    pub fn m10(&self) -> M10R {
        M10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Match Address Mode Enable 2"]
    #[inline]
    pub fn maen2(&self) -> MAEN2R {
        MAEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Match Address Mode Enable 1"]
    #[inline]
    pub fn maen1(&self) -> MAEN1R {
        MAEN1R::_from({
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
    #[doc = "Bits 0:4 - Baud Rate Fine Adjust"]
    #[inline]
    pub fn brfa(&mut self) -> _BRFAW {
        _BRFAW { w: self }
    }
    #[doc = "Bit 5 - 10-bit Mode select"]
    #[inline]
    pub fn m10(&mut self) -> _M10W {
        _M10W { w: self }
    }
    #[doc = "Bit 6 - Match Address Mode Enable 2"]
    #[inline]
    pub fn maen2(&mut self) -> _MAEN2W {
        _MAEN2W { w: self }
    }
    #[doc = "Bit 7 - Match Address Mode Enable 1"]
    #[inline]
    pub fn maen1(&mut self) -> _MAEN1W {
        _MAEN1W { w: self }
    }
}
