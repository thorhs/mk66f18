#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SC {
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
#[doc = "Possible values of the field `MODE_LV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_LVR {
    #[doc = "Bandgap on only, for stabilization and startup"]
    _00,
    #[doc = "High power buffer mode enabled"]
    _01,
    #[doc = "Low-power buffer mode enabled"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODE_LVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODE_LVR::_00 => 0,
            MODE_LVR::_01 => 1,
            MODE_LVR::_10 => 2,
            MODE_LVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODE_LVR {
        match value {
            0 => MODE_LVR::_00,
            1 => MODE_LVR::_01,
            2 => MODE_LVR::_10,
            i => MODE_LVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == MODE_LVR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == MODE_LVR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == MODE_LVR::_10
    }
}
#[doc = "Possible values of the field `VREFST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFSTR {
    #[doc = "The module is disabled or not stable."]
    _0,
    #[doc = "The module is stable."]
    _1,
}
impl VREFSTR {
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
            VREFSTR::_0 => false,
            VREFSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VREFSTR {
        match value {
            false => VREFSTR::_0,
            true => VREFSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VREFSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VREFSTR::_1
    }
}
#[doc = "Possible values of the field `ICOMPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICOMPENR {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl ICOMPENR {
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
            ICOMPENR::_0 => false,
            ICOMPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICOMPENR {
        match value {
            false => ICOMPENR::_0,
            true => ICOMPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ICOMPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ICOMPENR::_1
    }
}
#[doc = "Possible values of the field `REGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGENR {
    #[doc = "Internal 1.75 V regulator is disabled."]
    _0,
    #[doc = "Internal 1.75 V regulator is enabled."]
    _1,
}
impl REGENR {
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
            REGENR::_0 => false,
            REGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGENR {
        match value {
            false => REGENR::_0,
            true => REGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == REGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == REGENR::_1
    }
}
#[doc = "Possible values of the field `VREFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFENR {
    #[doc = "The module is disabled."]
    _0,
    #[doc = "The module is enabled."]
    _1,
}
impl VREFENR {
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
            VREFENR::_0 => false,
            VREFENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VREFENR {
        match value {
            false => VREFENR::_0,
            true => VREFENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VREFENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VREFENR::_1
    }
}
#[doc = "Values that can be written to the field `MODE_LV`"]
pub enum MODE_LVW {
    #[doc = "Bandgap on only, for stabilization and startup"]
    _00,
    #[doc = "High power buffer mode enabled"]
    _01,
    #[doc = "Low-power buffer mode enabled"]
    _10,
}
impl MODE_LVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODE_LVW::_00 => 0,
            MODE_LVW::_01 => 1,
            MODE_LVW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODE_LVW<'a> {
    w: &'a mut W,
}
impl<'a> _MODE_LVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE_LVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Bandgap on only, for stabilization and startup"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(MODE_LVW::_00)
    }
    #[doc = "High power buffer mode enabled"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(MODE_LVW::_01)
    }
    #[doc = "Low-power buffer mode enabled"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODE_LVW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ICOMPEN`"]
pub enum ICOMPENW {
    #[doc = "Disabled"]
    _0,
    #[doc = "Enabled"]
    _1,
}
impl ICOMPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ICOMPENW::_0 => false,
            ICOMPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICOMPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ICOMPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICOMPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICOMPENW::_0)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICOMPENW::_1)
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
#[doc = "Values that can be written to the field `REGEN`"]
pub enum REGENW {
    #[doc = "Internal 1.75 V regulator is disabled."]
    _0,
    #[doc = "Internal 1.75 V regulator is enabled."]
    _1,
}
impl REGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REGENW::_0 => false,
            REGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REGENW<'a> {
    w: &'a mut W,
}
impl<'a> _REGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal 1.75 V regulator is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(REGENW::_0)
    }
    #[doc = "Internal 1.75 V regulator is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(REGENW::_1)
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
#[doc = "Values that can be written to the field `VREFEN`"]
pub enum VREFENW {
    #[doc = "The module is disabled."]
    _0,
    #[doc = "The module is enabled."]
    _1,
}
impl VREFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VREFENW::_0 => false,
            VREFENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VREFENW<'a> {
    w: &'a mut W,
}
impl<'a> _VREFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VREFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The module is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREFENW::_0)
    }
    #[doc = "The module is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREFENW::_1)
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
    #[doc = "Bits 0:1 - Buffer Mode selection"]
    #[inline]
    pub fn mode_lv(&self) -> MODE_LVR {
        MODE_LVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 2 - Internal Voltage Reference stable"]
    #[inline]
    pub fn vrefst(&self) -> VREFSTR {
        VREFSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Second order curvature compensation enable"]
    #[inline]
    pub fn icompen(&self) -> ICOMPENR {
        ICOMPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Regulator enable"]
    #[inline]
    pub fn regen(&self) -> REGENR {
        REGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Internal Voltage Reference enable"]
    #[inline]
    pub fn vrefen(&self) -> VREFENR {
        VREFENR::_from({
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
    #[doc = "Bits 0:1 - Buffer Mode selection"]
    #[inline]
    pub fn mode_lv(&mut self) -> _MODE_LVW {
        _MODE_LVW { w: self }
    }
    #[doc = "Bit 5 - Second order curvature compensation enable"]
    #[inline]
    pub fn icompen(&mut self) -> _ICOMPENW {
        _ICOMPENW { w: self }
    }
    #[doc = "Bit 6 - Regulator enable"]
    #[inline]
    pub fn regen(&mut self) -> _REGENW {
        _REGENW { w: self }
    }
    #[doc = "Bit 7 - Internal Voltage Reference enable"]
    #[inline]
    pub fn vrefen(&mut self) -> _VREFENW {
        _VREFENW { w: self }
    }
}
