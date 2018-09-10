#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
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
#[doc = "Possible values of the field `SWR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRR {
    #[doc = "No effect."]
    _0,
    #[doc = "Resets all RTC registers except for the SWR bit and the RTC_WAR and RTC_RAR registers . The SWR bit is cleared by VBAT POR and by software explicitly clearing it."]
    _1,
}
impl SWRR {
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
            SWRR::_0 => false,
            SWRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRR {
        match value {
            false => SWRR::_0,
            true => SWRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWRR::_1
    }
}
#[doc = "Possible values of the field `WPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPER {
    #[doc = "Wakeup pin is disabled."]
    _0,
    #[doc = "Wakeup pin is enabled and wakeup pin asserts if the RTC interrupt asserts or the wakeup pin is turned on."]
    _1,
}
impl WPER {
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
            WPER::_0 => false,
            WPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPER {
        match value {
            false => WPER::_0,
            true => WPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WPER::_1
    }
}
#[doc = "Possible values of the field `SUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUPR {
    #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."]
    _0,
    #[doc = "Non-supervisor mode write accesses are supported."]
    _1,
}
impl SUPR {
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
            SUPR::_0 => false,
            SUPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUPR {
        match value {
            false => SUPR::_0,
            true => SUPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SUPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SUPR::_1
    }
}
#[doc = "Possible values of the field `UM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UMR {
    #[doc = "Registers cannot be written when locked."]
    _0,
    #[doc = "Registers can be written when locked under limited conditions."]
    _1,
}
impl UMR {
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
            UMR::_0 => false,
            UMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UMR {
        match value {
            false => UMR::_0,
            true => UMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UMR::_1
    }
}
#[doc = "Possible values of the field `WPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPSR {
    #[doc = "Wakeup pin asserts (active low, open drain) if the RTC interrupt asserts or the wakeup pin is turned on."]
    _0,
    #[doc = "Wakeup pin instead outputs the RTC 32kHz clock, provided the wakeup pin is turned on and the 32kHz clock is output to other peripherals."]
    _1,
}
impl WPSR {
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
            WPSR::_0 => false,
            WPSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPSR {
        match value {
            false => WPSR::_0,
            true => WPSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WPSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WPSR::_1
    }
}
#[doc = "Possible values of the field `OSCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCER {
    #[doc = "32.768 kHz oscillator is disabled."]
    _0,
    #[doc = "32.768 kHz oscillator is enabled. After setting this bit, wait the oscillator startup time before enabling the time counter to allow the 32.768 kHz clock time to stabilize."]
    _1,
}
impl OSCER {
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
            OSCER::_0 => false,
            OSCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCER {
        match value {
            false => OSCER::_0,
            true => OSCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OSCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OSCER::_1
    }
}
#[doc = "Possible values of the field `CLKO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOR {
    #[doc = "The 32 kHz clock is output to other peripherals."]
    _0,
    #[doc = "The 32 kHz clock is not output to other peripherals."]
    _1,
}
impl CLKOR {
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
            CLKOR::_0 => false,
            CLKOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKOR {
        match value {
            false => CLKOR::_0,
            true => CLKOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CLKOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CLKOR::_1
    }
}
#[doc = "Possible values of the field `SC16P`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC16PR {
    #[doc = "Disable the load."]
    _0,
    #[doc = "Enable the additional load."]
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
    #[doc = "Disable the load."]
    _0,
    #[doc = "Enable the additional load."]
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
    #[doc = "Disable the load."]
    _0,
    #[doc = "Enable the additional load."]
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
    #[doc = "Disable the load."]
    _0,
    #[doc = "Enable the additional load."]
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
#[doc = "Values that can be written to the field `SWR`"]
pub enum SWRW {
    #[doc = "No effect."]
    _0,
    #[doc = "Resets all RTC registers except for the SWR bit and the RTC_WAR and RTC_RAR registers . The SWR bit is cleared by VBAT POR and by software explicitly clearing it."]
    _1,
}
impl SWRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRW::_0 => false,
            SWRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRW::_0)
    }
    #[doc = "Resets all RTC registers except for the SWR bit and the RTC_WAR and RTC_RAR registers . The SWR bit is cleared by VBAT POR and by software explicitly clearing it."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRW::_1)
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
#[doc = "Values that can be written to the field `WPE`"]
pub enum WPEW {
    #[doc = "Wakeup pin is disabled."]
    _0,
    #[doc = "Wakeup pin is enabled and wakeup pin asserts if the RTC interrupt asserts or the wakeup pin is turned on."]
    _1,
}
impl WPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WPEW::_0 => false,
            WPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WPEW<'a> {
    w: &'a mut W,
}
impl<'a> _WPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wakeup pin is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPEW::_0)
    }
    #[doc = "Wakeup pin is enabled and wakeup pin asserts if the RTC interrupt asserts or the wakeup pin is turned on."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPEW::_1)
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
#[doc = "Values that can be written to the field `SUP`"]
pub enum SUPW {
    #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."]
    _0,
    #[doc = "Non-supervisor mode write accesses are supported."]
    _1,
}
impl SUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUPW::_0 => false,
            SUPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUPW<'a> {
    w: &'a mut W,
}
impl<'a> _SUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUPW::_0)
    }
    #[doc = "Non-supervisor mode write accesses are supported."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUPW::_1)
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
#[doc = "Values that can be written to the field `UM`"]
pub enum UMW {
    #[doc = "Registers cannot be written when locked."]
    _0,
    #[doc = "Registers can be written when locked under limited conditions."]
    _1,
}
impl UMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UMW::_0 => false,
            UMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UMW<'a> {
    w: &'a mut W,
}
impl<'a> _UMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Registers cannot be written when locked."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UMW::_0)
    }
    #[doc = "Registers can be written when locked under limited conditions."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UMW::_1)
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
#[doc = "Values that can be written to the field `WPS`"]
pub enum WPSW {
    #[doc = "Wakeup pin asserts (active low, open drain) if the RTC interrupt asserts or the wakeup pin is turned on."]
    _0,
    #[doc = "Wakeup pin instead outputs the RTC 32kHz clock, provided the wakeup pin is turned on and the 32kHz clock is output to other peripherals."]
    _1,
}
impl WPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WPSW::_0 => false,
            WPSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WPSW<'a> {
    w: &'a mut W,
}
impl<'a> _WPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wakeup pin asserts (active low, open drain) if the RTC interrupt asserts or the wakeup pin is turned on."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPSW::_0)
    }
    #[doc = "Wakeup pin instead outputs the RTC 32kHz clock, provided the wakeup pin is turned on and the 32kHz clock is output to other peripherals."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPSW::_1)
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
#[doc = "Values that can be written to the field `OSCE`"]
pub enum OSCEW {
    #[doc = "32.768 kHz oscillator is disabled."]
    _0,
    #[doc = "32.768 kHz oscillator is enabled. After setting this bit, wait the oscillator startup time before enabling the time counter to allow the 32.768 kHz clock time to stabilize."]
    _1,
}
impl OSCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCEW::_0 => false,
            OSCEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCEW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "32.768 kHz oscillator is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSCEW::_0)
    }
    #[doc = "32.768 kHz oscillator is enabled. After setting this bit, wait the oscillator startup time before enabling the time counter to allow the 32.768 kHz clock time to stabilize."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSCEW::_1)
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
#[doc = "Values that can be written to the field `CLKO`"]
pub enum CLKOW {
    #[doc = "The 32 kHz clock is output to other peripherals."]
    _0,
    #[doc = "The 32 kHz clock is not output to other peripherals."]
    _1,
}
impl CLKOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKOW::_0 => false,
            CLKOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The 32 kHz clock is output to other peripherals."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKOW::_0)
    }
    #[doc = "The 32 kHz clock is not output to other peripherals."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKOW::_1)
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
#[doc = "Values that can be written to the field `SC16P`"]
pub enum SC16PW {
    #[doc = "Disable the load."]
    _0,
    #[doc = "Enable the additional load."]
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
    #[doc = "Disable the load."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC16PW::_0)
    }
    #[doc = "Enable the additional load."]
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SC8P`"]
pub enum SC8PW {
    #[doc = "Disable the load."]
    _0,
    #[doc = "Enable the additional load."]
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
    #[doc = "Disable the load."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC8PW::_0)
    }
    #[doc = "Enable the additional load."]
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SC4P`"]
pub enum SC4PW {
    #[doc = "Disable the load."]
    _0,
    #[doc = "Enable the additional load."]
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
    #[doc = "Disable the load."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC4PW::_0)
    }
    #[doc = "Enable the additional load."]
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SC2P`"]
pub enum SC2PW {
    #[doc = "Disable the load."]
    _0,
    #[doc = "Enable the additional load."]
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
    #[doc = "Disable the load."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC2PW::_0)
    }
    #[doc = "Enable the additional load."]
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
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swr(&self) -> SWRR {
        SWRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wakeup Pin Enable"]
    #[inline]
    pub fn wpe(&self) -> WPER {
        WPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline]
    pub fn sup(&self) -> SUPR {
        SUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline]
    pub fn um(&self) -> UMR {
        UMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Wakeup Pin Select"]
    #[inline]
    pub fn wps(&self) -> WPSR {
        WPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Oscillator Enable"]
    #[inline]
    pub fn osce(&self) -> OSCER {
        OSCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Clock Output"]
    #[inline]
    pub fn clko(&self) -> CLKOR {
        CLKOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Oscillator 16pF Load Configure"]
    #[inline]
    pub fn sc16p(&self) -> SC16PR {
        SC16PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Oscillator 8pF Load Configure"]
    #[inline]
    pub fn sc8p(&self) -> SC8PR {
        SC8PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Oscillator 4pF Load Configure"]
    #[inline]
    pub fn sc4p(&self) -> SC4PR {
        SC4PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Oscillator 2pF Load Configure"]
    #[inline]
    pub fn sc2p(&self) -> SC2PR {
        SC2PR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
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
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swr(&mut self) -> _SWRW {
        _SWRW { w: self }
    }
    #[doc = "Bit 1 - Wakeup Pin Enable"]
    #[inline]
    pub fn wpe(&mut self) -> _WPEW {
        _WPEW { w: self }
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline]
    pub fn sup(&mut self) -> _SUPW {
        _SUPW { w: self }
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline]
    pub fn um(&mut self) -> _UMW {
        _UMW { w: self }
    }
    #[doc = "Bit 4 - Wakeup Pin Select"]
    #[inline]
    pub fn wps(&mut self) -> _WPSW {
        _WPSW { w: self }
    }
    #[doc = "Bit 8 - Oscillator Enable"]
    #[inline]
    pub fn osce(&mut self) -> _OSCEW {
        _OSCEW { w: self }
    }
    #[doc = "Bit 9 - Clock Output"]
    #[inline]
    pub fn clko(&mut self) -> _CLKOW {
        _CLKOW { w: self }
    }
    #[doc = "Bit 10 - Oscillator 16pF Load Configure"]
    #[inline]
    pub fn sc16p(&mut self) -> _SC16PW {
        _SC16PW { w: self }
    }
    #[doc = "Bit 11 - Oscillator 8pF Load Configure"]
    #[inline]
    pub fn sc8p(&mut self) -> _SC8PW {
        _SC8PW { w: self }
    }
    #[doc = "Bit 12 - Oscillator 4pF Load Configure"]
    #[inline]
    pub fn sc4p(&mut self) -> _SC4PW {
        _SC4PW { w: self }
    }
    #[doc = "Bit 13 - Oscillator 2pF Load Configure"]
    #[inline]
    pub fn sc2p(&mut self) -> _SC2PW {
        _SC2PW { w: self }
    }
}
