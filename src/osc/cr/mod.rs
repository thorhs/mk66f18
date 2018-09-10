#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CR {
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
#[doc = "Possible values of the field `SC16P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC16PR {
    #[doc = "Disable the selection."]
    _0,
    #[doc = "Add 16 pF capacitor to the oscillator load."]
    _1,
}
impl SC16PR {
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
            SC16PR::_0 => false,
            SC16PR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SC16PR {
        match value {
            false => SC16PR::_0,
            true => SC16PR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SC16PR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SC16PR::_1
    }
}
#[doc = "Possible values of the field `SC8P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC8PR {
    #[doc = "Disable the selection."]
    _0,
    #[doc = "Add 8 pF capacitor to the oscillator load."]
    _1,
}
impl SC8PR {
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
            SC8PR::_0 => false,
            SC8PR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SC8PR {
        match value {
            false => SC8PR::_0,
            true => SC8PR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SC8PR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SC8PR::_1
    }
}
#[doc = "Possible values of the field `SC4P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC4PR {
    #[doc = "Disable the selection."]
    _0,
    #[doc = "Add 4 pF capacitor to the oscillator load."]
    _1,
}
impl SC4PR {
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
            SC4PR::_0 => false,
            SC4PR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SC4PR {
        match value {
            false => SC4PR::_0,
            true => SC4PR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SC4PR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SC4PR::_1
    }
}
#[doc = "Possible values of the field `SC2P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC2PR {
    #[doc = "Disable the selection."]
    _0,
    #[doc = "Add 2 pF capacitor to the oscillator load."]
    _1,
}
impl SC2PR {
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
            SC2PR::_0 => false,
            SC2PR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SC2PR {
        match value {
            false => SC2PR::_0,
            true => SC2PR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SC2PR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SC2PR::_1
    }
}
#[doc = "Possible values of the field `EREFSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EREFSTENR {
    #[doc = "External reference clock is disabled in Stop mode."]
    _0,
    #[doc = "External reference clock stays enabled in Stop mode if ERCLKEN is set before entering Stop mode."]
    _1,
}
impl EREFSTENR {
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
            EREFSTENR::_0 => false,
            EREFSTENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EREFSTENR {
        match value {
            false => EREFSTENR::_0,
            true => EREFSTENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EREFSTENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EREFSTENR::_1
    }
}
#[doc = "Possible values of the field `ERCLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERCLKENR {
    #[doc = "External reference clock is inactive."]
    _0,
    #[doc = "External reference clock is enabled."]
    _1,
}
impl ERCLKENR {
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
            ERCLKENR::_0 => false,
            ERCLKENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERCLKENR {
        match value {
            false => ERCLKENR::_0,
            true => ERCLKENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERCLKENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERCLKENR::_1
    }
}
#[doc = "Values that can be written to the field `SC16P`"]
pub enum SC16PW {
    #[doc = "Disable the selection."]
    _0,
    #[doc = "Add 16 pF capacitor to the oscillator load."]
    _1,
}
impl SC16PW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SC16PW::_0 => false,
            SC16PW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SC16PW<'a> {
    w: &'a mut W,
}
impl<'a> _SC16PW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SC16PW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the selection."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC16PW::_0)
    }
    #[doc = "Add 16 pF capacitor to the oscillator load."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC16PW::_1)
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
#[doc = "Values that can be written to the field `SC8P`"]
pub enum SC8PW {
    #[doc = "Disable the selection."]
    _0,
    #[doc = "Add 8 pF capacitor to the oscillator load."]
    _1,
}
impl SC8PW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SC8PW::_0 => false,
            SC8PW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SC8PW<'a> {
    w: &'a mut W,
}
impl<'a> _SC8PW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SC8PW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the selection."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC8PW::_0)
    }
    #[doc = "Add 8 pF capacitor to the oscillator load."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC8PW::_1)
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
#[doc = "Values that can be written to the field `SC4P`"]
pub enum SC4PW {
    #[doc = "Disable the selection."]
    _0,
    #[doc = "Add 4 pF capacitor to the oscillator load."]
    _1,
}
impl SC4PW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SC4PW::_0 => false,
            SC4PW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SC4PW<'a> {
    w: &'a mut W,
}
impl<'a> _SC4PW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SC4PW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the selection."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC4PW::_0)
    }
    #[doc = "Add 4 pF capacitor to the oscillator load."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC4PW::_1)
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
#[doc = "Values that can be written to the field `SC2P`"]
pub enum SC2PW {
    #[doc = "Disable the selection."]
    _0,
    #[doc = "Add 2 pF capacitor to the oscillator load."]
    _1,
}
impl SC2PW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SC2PW::_0 => false,
            SC2PW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SC2PW<'a> {
    w: &'a mut W,
}
impl<'a> _SC2PW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SC2PW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the selection."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC2PW::_0)
    }
    #[doc = "Add 2 pF capacitor to the oscillator load."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC2PW::_1)
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
#[doc = "Values that can be written to the field `EREFSTEN`"]
pub enum EREFSTENW {
    #[doc = "External reference clock is disabled in Stop mode."]
    _0,
    #[doc = "External reference clock stays enabled in Stop mode if ERCLKEN is set before entering Stop mode."]
    _1,
}
impl EREFSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EREFSTENW::_0 => false,
            EREFSTENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EREFSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _EREFSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EREFSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External reference clock is disabled in Stop mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EREFSTENW::_0)
    }
    #[doc = "External reference clock stays enabled in Stop mode if ERCLKEN is set before entering Stop mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EREFSTENW::_1)
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
#[doc = "Values that can be written to the field `ERCLKEN`"]
pub enum ERCLKENW {
    #[doc = "External reference clock is inactive."]
    _0,
    #[doc = "External reference clock is enabled."]
    _1,
}
impl ERCLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERCLKENW::_0 => false,
            ERCLKENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _ERCLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERCLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External reference clock is inactive."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERCLKENW::_0)
    }
    #[doc = "External reference clock is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERCLKENW::_1)
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
    #[doc = "Bit 0 - Oscillator 16 pF Capacitor Load Configure"]
    #[inline]
    pub fn sc16p(&self) -> SC16PR {
        SC16PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Oscillator 8 pF Capacitor Load Configure"]
    #[inline]
    pub fn sc8p(&self) -> SC8PR {
        SC8PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Oscillator 4 pF Capacitor Load Configure"]
    #[inline]
    pub fn sc4p(&self) -> SC4PR {
        SC4PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Oscillator 2 pF Capacitor Load Configure"]
    #[inline]
    pub fn sc2p(&self) -> SC2PR {
        SC2PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - External Reference Stop Enable"]
    #[inline]
    pub fn erefsten(&self) -> EREFSTENR {
        EREFSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - External Reference Enable"]
    #[inline]
    pub fn erclken(&self) -> ERCLKENR {
        ERCLKENR::_from({
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
    #[doc = "Bit 0 - Oscillator 16 pF Capacitor Load Configure"]
    #[inline]
    pub fn sc16p(&mut self) -> _SC16PW {
        _SC16PW { w: self }
    }
    #[doc = "Bit 1 - Oscillator 8 pF Capacitor Load Configure"]
    #[inline]
    pub fn sc8p(&mut self) -> _SC8PW {
        _SC8PW { w: self }
    }
    #[doc = "Bit 2 - Oscillator 4 pF Capacitor Load Configure"]
    #[inline]
    pub fn sc4p(&mut self) -> _SC4PW {
        _SC4PW { w: self }
    }
    #[doc = "Bit 3 - Oscillator 2 pF Capacitor Load Configure"]
    #[inline]
    pub fn sc2p(&mut self) -> _SC2PW {
        _SC2PW { w: self }
    }
    #[doc = "Bit 5 - External Reference Stop Enable"]
    #[inline]
    pub fn erefsten(&mut self) -> _EREFSTENW {
        _EREFSTENW { w: self }
    }
    #[doc = "Bit 7 - External Reference Enable"]
    #[inline]
    pub fn erclken(&mut self) -> _ERCLKENW {
        _ERCLKENW { w: self }
    }
}
