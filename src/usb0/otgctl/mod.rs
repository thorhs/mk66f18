#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::OTGCTL {
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
#[doc = "Possible values of the field `OTGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGENR {
    #[doc = "If USB_EN is 1 and HOST_MODE is 0 in the Control Register (CTL), then the D+ Data Line pull-up resistors are enabled. If HOST_MODE is 1 the D+ and D- Data Line pull-down resistors are engaged."]
    _0,
    #[doc = "The pull-up and pull-down controls in this register are used."]
    _1,
}
impl OTGENR {
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
            OTGENR::_0 => false,
            OTGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTGENR {
        match value {
            false => OTGENR::_0,
            true => OTGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OTGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OTGENR::_1
    }
}
#[doc = "Possible values of the field `DMLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMLOWR {
    #[doc = "D- pulldown resistor is not enabled."]
    _0,
    #[doc = "D- pulldown resistor is enabled."]
    _1,
}
impl DMLOWR {
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
            DMLOWR::_0 => false,
            DMLOWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMLOWR {
        match value {
            false => DMLOWR::_0,
            true => DMLOWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMLOWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMLOWR::_1
    }
}
#[doc = "Possible values of the field `DPLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPLOWR {
    #[doc = "D+ pulldown resistor is not enabled."]
    _0,
    #[doc = "D+ pulldown resistor is enabled."]
    _1,
}
impl DPLOWR {
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
            DPLOWR::_0 => false,
            DPLOWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPLOWR {
        match value {
            false => DPLOWR::_0,
            true => DPLOWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DPLOWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DPLOWR::_1
    }
}
#[doc = "Possible values of the field `DPHIGH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPHIGHR {
    #[doc = "D+ pullup resistor is not enabled"]
    _0,
    #[doc = "D+ pullup resistor is enabled"]
    _1,
}
impl DPHIGHR {
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
            DPHIGHR::_0 => false,
            DPHIGHR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPHIGHR {
        match value {
            false => DPHIGHR::_0,
            true => DPHIGHR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DPHIGHR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DPHIGHR::_1
    }
}
#[doc = "Values that can be written to the field `OTGEN`"]
pub enum OTGENW {
    #[doc = "If USB_EN is 1 and HOST_MODE is 0 in the Control Register (CTL), then the D+ Data Line pull-up resistors are enabled. If HOST_MODE is 1 the D+ and D- Data Line pull-down resistors are engaged."]
    _0,
    #[doc = "The pull-up and pull-down controls in this register are used."]
    _1,
}
impl OTGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTGENW::_0 => false,
            OTGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTGENW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If USB_EN is 1 and HOST_MODE is 0 in the Control Register (CTL), then the D+ Data Line pull-up resistors are enabled. If HOST_MODE is 1 the D+ and D- Data Line pull-down resistors are engaged."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OTGENW::_0)
    }
    #[doc = "The pull-up and pull-down controls in this register are used."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OTGENW::_1)
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
#[doc = "Values that can be written to the field `DMLOW`"]
pub enum DMLOWW {
    #[doc = "D- pulldown resistor is not enabled."]
    _0,
    #[doc = "D- pulldown resistor is enabled."]
    _1,
}
impl DMLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMLOWW::_0 => false,
            DMLOWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _DMLOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMLOWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "D- pulldown resistor is not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMLOWW::_0)
    }
    #[doc = "D- pulldown resistor is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMLOWW::_1)
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
#[doc = "Values that can be written to the field `DPLOW`"]
pub enum DPLOWW {
    #[doc = "D+ pulldown resistor is not enabled."]
    _0,
    #[doc = "D+ pulldown resistor is enabled."]
    _1,
}
impl DPLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPLOWW::_0 => false,
            DPLOWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _DPLOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPLOWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "D+ pulldown resistor is not enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPLOWW::_0)
    }
    #[doc = "D+ pulldown resistor is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPLOWW::_1)
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
#[doc = "Values that can be written to the field `DPHIGH`"]
pub enum DPHIGHW {
    #[doc = "D+ pullup resistor is not enabled"]
    _0,
    #[doc = "D+ pullup resistor is enabled"]
    _1,
}
impl DPHIGHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPHIGHW::_0 => false,
            DPHIGHW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _DPHIGHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPHIGHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "D+ pullup resistor is not enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPHIGHW::_0)
    }
    #[doc = "D+ pullup resistor is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPHIGHW::_1)
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
    #[doc = "Bit 2 - On-The-Go pullup/pulldown resistor enable"]
    #[inline]
    pub fn otgen(&self) -> OTGENR {
        OTGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - D- Data Line pull-down resistor enable"]
    #[inline]
    pub fn dmlow(&self) -> DMLOWR {
        DMLOWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - D+ Data Line pull-down resistor enable"]
    #[inline]
    pub fn dplow(&self) -> DPLOWR {
        DPLOWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - D+ Data Line pullup resistor enable"]
    #[inline]
    pub fn dphigh(&self) -> DPHIGHR {
        DPHIGHR::_from({
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
    #[doc = "Bit 2 - On-The-Go pullup/pulldown resistor enable"]
    #[inline]
    pub fn otgen(&mut self) -> _OTGENW {
        _OTGENW { w: self }
    }
    #[doc = "Bit 4 - D- Data Line pull-down resistor enable"]
    #[inline]
    pub fn dmlow(&mut self) -> _DMLOWW {
        _DMLOWW { w: self }
    }
    #[doc = "Bit 5 - D+ Data Line pull-down resistor enable"]
    #[inline]
    pub fn dplow(&mut self) -> _DPLOWW {
        _DPLOWW { w: self }
    }
    #[doc = "Bit 7 - D+ Data Line pullup resistor enable"]
    #[inline]
    pub fn dphigh(&mut self) -> _DPHIGHW {
        _DPHIGHW { w: self }
    }
}
