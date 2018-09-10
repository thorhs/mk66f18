#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::FLT {
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
#[doc = "Possible values of the field `FLT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLTR {
    #[doc = "No filter/bypass"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLTR::_0 => 0,
            FLTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLTR {
        match value {
            0 => FLTR::_0,
            i => FLTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLTR::_0
    }
}
#[doc = "Possible values of the field `STARTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTFR {
    #[doc = "No start happens on I2C bus"]
    _0,
    #[doc = "Start detected on I2C bus"]
    _1,
}
impl STARTFR {
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
            STARTFR::_0 => false,
            STARTFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTFR {
        match value {
            false => STARTFR::_0,
            true => STARTFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STARTFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STARTFR::_1
    }
}
#[doc = "Possible values of the field `SSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSIER {
    #[doc = "Stop or start detection interrupt is disabled"]
    _0,
    #[doc = "Stop or start detection interrupt is enabled"]
    _1,
}
impl SSIER {
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
            SSIER::_0 => false,
            SSIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSIER {
        match value {
            false => SSIER::_0,
            true => SSIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SSIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SSIER::_1
    }
}
#[doc = "Possible values of the field `STOPF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPFR {
    #[doc = "No stop happens on I2C bus"]
    _0,
    #[doc = "Stop detected on I2C bus"]
    _1,
}
impl STOPFR {
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
            STOPFR::_0 => false,
            STOPFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPFR {
        match value {
            false => STOPFR::_0,
            true => STOPFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STOPFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STOPFR::_1
    }
}
#[doc = "Possible values of the field `SHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHENR {
    #[doc = "Stop holdoff is disabled. The MCU's entry to stop mode is not gated."]
    _0,
    #[doc = "Stop holdoff is enabled."]
    _1,
}
impl SHENR {
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
            SHENR::_0 => false,
            SHENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SHENR {
        match value {
            false => SHENR::_0,
            true => SHENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SHENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SHENR::_1
    }
}
#[doc = "Values that can be written to the field `FLT`"]
pub enum FLTW {
    #[doc = "No filter/bypass"]
    _0,
}
impl FLTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLTW::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLTW<'a> {
    w: &'a mut W,
}
impl<'a> _FLTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No filter/bypass"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLTW::_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STARTF`"]
pub enum STARTFW {
    #[doc = "No start happens on I2C bus"]
    _0,
    #[doc = "Start detected on I2C bus"]
    _1,
}
impl STARTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTFW::_0 => false,
            STARTFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTFW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No start happens on I2C bus"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARTFW::_0)
    }
    #[doc = "Start detected on I2C bus"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STARTFW::_1)
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
#[doc = "Values that can be written to the field `SSIE`"]
pub enum SSIEW {
    #[doc = "Stop or start detection interrupt is disabled"]
    _0,
    #[doc = "Stop or start detection interrupt is enabled"]
    _1,
}
impl SSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSIEW::_0 => false,
            SSIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop or start detection interrupt is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSIEW::_0)
    }
    #[doc = "Stop or start detection interrupt is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSIEW::_1)
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
#[doc = "Values that can be written to the field `STOPF`"]
pub enum STOPFW {
    #[doc = "No stop happens on I2C bus"]
    _0,
    #[doc = "Stop detected on I2C bus"]
    _1,
}
impl STOPFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPFW::_0 => false,
            STOPFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPFW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No stop happens on I2C bus"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPFW::_0)
    }
    #[doc = "Stop detected on I2C bus"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPFW::_1)
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
#[doc = "Values that can be written to the field `SHEN`"]
pub enum SHENW {
    #[doc = "Stop holdoff is disabled. The MCU's entry to stop mode is not gated."]
    _0,
    #[doc = "Stop holdoff is enabled."]
    _1,
}
impl SHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SHENW::_0 => false,
            SHENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHENW<'a> {
    w: &'a mut W,
}
impl<'a> _SHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop holdoff is disabled. The MCU's entry to stop mode is not gated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHENW::_0)
    }
    #[doc = "Stop holdoff is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHENW::_1)
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
    #[doc = "Bits 0:3 - I2C Programmable Filter Factor"]
    #[inline]
    pub fn flt(&self) -> FLTR {
        FLTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 4 - I2C Bus Start Detect Flag"]
    #[inline]
    pub fn startf(&self) -> STARTFR {
        STARTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - I2C Bus Stop or Start Interrupt Enable"]
    #[inline]
    pub fn ssie(&self) -> SSIER {
        SSIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - I2C Bus Stop Detect Flag"]
    #[inline]
    pub fn stopf(&self) -> STOPFR {
        STOPFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Stop Hold Enable"]
    #[inline]
    pub fn shen(&self) -> SHENR {
        SHENR::_from({
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
    #[doc = "Bits 0:3 - I2C Programmable Filter Factor"]
    #[inline]
    pub fn flt(&mut self) -> _FLTW {
        _FLTW { w: self }
    }
    #[doc = "Bit 4 - I2C Bus Start Detect Flag"]
    #[inline]
    pub fn startf(&mut self) -> _STARTFW {
        _STARTFW { w: self }
    }
    #[doc = "Bit 5 - I2C Bus Stop or Start Interrupt Enable"]
    #[inline]
    pub fn ssie(&mut self) -> _SSIEW {
        _SSIEW { w: self }
    }
    #[doc = "Bit 6 - I2C Bus Stop Detect Flag"]
    #[inline]
    pub fn stopf(&mut self) -> _STOPFW {
        _STOPFW { w: self }
    }
    #[doc = "Bit 7 - Stop Hold Enable"]
    #[inline]
    pub fn shen(&mut self) -> _SHENW {
        _SHENW { w: self }
    }
}
