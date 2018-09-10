#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IER {
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
#[doc = "Possible values of the field `TIIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIIER {
    #[doc = "Time invalid flag does not generate an interrupt."]
    _0,
    #[doc = "Time invalid flag does generate an interrupt."]
    _1,
}
impl TIIER {
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
            TIIER::_0 => false,
            TIIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIIER {
        match value {
            false => TIIER::_0,
            true => TIIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIIER::_1
    }
}
#[doc = "Possible values of the field `TOIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIER {
    #[doc = "Time overflow flag does not generate an interrupt."]
    _0,
    #[doc = "Time overflow flag does generate an interrupt."]
    _1,
}
impl TOIER {
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
            TOIER::_0 => false,
            TOIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOIER {
        match value {
            false => TOIER::_0,
            true => TOIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOIER::_1
    }
}
#[doc = "Possible values of the field `TAIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIER {
    #[doc = "Time alarm flag does not generate an interrupt."]
    _0,
    #[doc = "Time alarm flag does generate an interrupt."]
    _1,
}
impl TAIER {
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
            TAIER::_0 => false,
            TAIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAIER {
        match value {
            false => TAIER::_0,
            true => TAIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TAIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TAIER::_1
    }
}
#[doc = "Possible values of the field `MOIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOIER {
    #[doc = "Monotonic overflow flag does not generate an interrupt."]
    _0,
    #[doc = "Monotonic overflow flag does generate an interrupt."]
    _1,
}
impl MOIER {
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
            MOIER::_0 => false,
            MOIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MOIER {
        match value {
            false => MOIER::_0,
            true => MOIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MOIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MOIER::_1
    }
}
#[doc = "Possible values of the field `TSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIER {
    #[doc = "Seconds interrupt is disabled."]
    _0,
    #[doc = "Seconds interrupt is enabled."]
    _1,
}
impl TSIER {
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
            TSIER::_0 => false,
            TSIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSIER {
        match value {
            false => TSIER::_0,
            true => TSIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSIER::_1
    }
}
#[doc = "Possible values of the field `WPON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPONR {
    #[doc = "No effect."]
    _0,
    #[doc = "If the wakeup pin is enabled, then the wakeup pin will assert."]
    _1,
}
impl WPONR {
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
            WPONR::_0 => false,
            WPONR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPONR {
        match value {
            false => WPONR::_0,
            true => WPONR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WPONR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WPONR::_1
    }
}
#[doc = "Values that can be written to the field `TIIE`"]
pub enum TIIEW {
    #[doc = "Time invalid flag does not generate an interrupt."]
    _0,
    #[doc = "Time invalid flag does generate an interrupt."]
    _1,
}
impl TIIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIIEW::_0 => false,
            TIIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Time invalid flag does not generate an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIIEW::_0)
    }
    #[doc = "Time invalid flag does generate an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIIEW::_1)
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
#[doc = "Values that can be written to the field `TOIE`"]
pub enum TOIEW {
    #[doc = "Time overflow flag does not generate an interrupt."]
    _0,
    #[doc = "Time overflow flag does generate an interrupt."]
    _1,
}
impl TOIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOIEW::_0 => false,
            TOIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TOIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Time overflow flag does not generate an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIEW::_0)
    }
    #[doc = "Time overflow flag does generate an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOIEW::_1)
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
#[doc = "Values that can be written to the field `TAIE`"]
pub enum TAIEW {
    #[doc = "Time alarm flag does not generate an interrupt."]
    _0,
    #[doc = "Time alarm flag does generate an interrupt."]
    _1,
}
impl TAIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAIEW::_0 => false,
            TAIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TAIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Time alarm flag does not generate an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TAIEW::_0)
    }
    #[doc = "Time alarm flag does generate an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TAIEW::_1)
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
#[doc = "Values that can be written to the field `MOIE`"]
pub enum MOIEW {
    #[doc = "Monotonic overflow flag does not generate an interrupt."]
    _0,
    #[doc = "Monotonic overflow flag does generate an interrupt."]
    _1,
}
impl MOIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MOIEW::_0 => false,
            MOIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MOIEW<'a> {
    w: &'a mut W,
}
impl<'a> _MOIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MOIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Monotonic overflow flag does not generate an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MOIEW::_0)
    }
    #[doc = "Monotonic overflow flag does generate an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MOIEW::_1)
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
#[doc = "Values that can be written to the field `TSIE`"]
pub enum TSIEW {
    #[doc = "Seconds interrupt is disabled."]
    _0,
    #[doc = "Seconds interrupt is enabled."]
    _1,
}
impl TSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSIEW::_0 => false,
            TSIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Seconds interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIEW::_0)
    }
    #[doc = "Seconds interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIEW::_1)
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
#[doc = "Values that can be written to the field `WPON`"]
pub enum WPONW {
    #[doc = "No effect."]
    _0,
    #[doc = "If the wakeup pin is enabled, then the wakeup pin will assert."]
    _1,
}
impl WPONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WPONW::_0 => false,
            WPONW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WPONW<'a> {
    w: &'a mut W,
}
impl<'a> _WPONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WPONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPONW::_0)
    }
    #[doc = "If the wakeup pin is enabled, then the wakeup pin will assert."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPONW::_1)
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
    #[doc = "Bit 0 - Time Invalid Interrupt Enable"]
    #[inline]
    pub fn tiie(&self) -> TIIER {
        TIIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Time Overflow Interrupt Enable"]
    #[inline]
    pub fn toie(&self) -> TOIER {
        TOIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Time Alarm Interrupt Enable"]
    #[inline]
    pub fn taie(&self) -> TAIER {
        TAIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Monotonic Overflow Interrupt Enable"]
    #[inline]
    pub fn moie(&self) -> MOIER {
        MOIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Time Seconds Interrupt Enable"]
    #[inline]
    pub fn tsie(&self) -> TSIER {
        TSIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Wakeup Pin On"]
    #[inline]
    pub fn wpon(&self) -> WPONR {
        WPONR::_from({
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
        W { bits: 7 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Time Invalid Interrupt Enable"]
    #[inline]
    pub fn tiie(&mut self) -> _TIIEW {
        _TIIEW { w: self }
    }
    #[doc = "Bit 1 - Time Overflow Interrupt Enable"]
    #[inline]
    pub fn toie(&mut self) -> _TOIEW {
        _TOIEW { w: self }
    }
    #[doc = "Bit 2 - Time Alarm Interrupt Enable"]
    #[inline]
    pub fn taie(&mut self) -> _TAIEW {
        _TAIEW { w: self }
    }
    #[doc = "Bit 3 - Monotonic Overflow Interrupt Enable"]
    #[inline]
    pub fn moie(&mut self) -> _MOIEW {
        _MOIEW { w: self }
    }
    #[doc = "Bit 4 - Time Seconds Interrupt Enable"]
    #[inline]
    pub fn tsie(&mut self) -> _TSIEW {
        _TSIEW { w: self }
    }
    #[doc = "Bit 7 - Wakeup Pin On"]
    #[inline]
    pub fn wpon(&mut self) -> _WPONW {
        _WPONW { w: self }
    }
}
