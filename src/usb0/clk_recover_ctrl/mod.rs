#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CLK_RECOVER_CTRL {
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
#[doc = "Possible values of the field `RESTART_IFRTRIM_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESTART_IFRTRIM_ENR {
    #[doc = "Trim fine adjustment always works based on the previous updated trim fine value (default)"]
    _0,
    #[doc = "Trim fine restarts from the IFR trim value whenever bus_reset/bus_resume is detected or module enable is desasserted"]
    _1,
}
impl RESTART_IFRTRIM_ENR {
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
            RESTART_IFRTRIM_ENR::_0 => false,
            RESTART_IFRTRIM_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESTART_IFRTRIM_ENR {
        match value {
            false => RESTART_IFRTRIM_ENR::_0,
            true => RESTART_IFRTRIM_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RESTART_IFRTRIM_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RESTART_IFRTRIM_ENR::_1
    }
}
#[doc = "Possible values of the field `RESET_RESUME_ROUGH_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_RESUME_ROUGH_ENR {
    #[doc = "Always works in tracking phase after the 1st time rough to track transition (default)"]
    _0,
    #[doc = "Go back to rough stage whenever bus reset or bus resume occurs"]
    _1,
}
impl RESET_RESUME_ROUGH_ENR {
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
            RESET_RESUME_ROUGH_ENR::_0 => false,
            RESET_RESUME_ROUGH_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESET_RESUME_ROUGH_ENR {
        match value {
            false => RESET_RESUME_ROUGH_ENR::_0,
            true => RESET_RESUME_ROUGH_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RESET_RESUME_ROUGH_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RESET_RESUME_ROUGH_ENR::_1
    }
}
#[doc = "Possible values of the field `CLOCK_RECOVER_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCK_RECOVER_ENR {
    #[doc = "Disable clock recovery block (default)"]
    _0,
    #[doc = "Enable clock recovery block"]
    _1,
}
impl CLOCK_RECOVER_ENR {
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
            CLOCK_RECOVER_ENR::_0 => false,
            CLOCK_RECOVER_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLOCK_RECOVER_ENR {
        match value {
            false => CLOCK_RECOVER_ENR::_0,
            true => CLOCK_RECOVER_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CLOCK_RECOVER_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CLOCK_RECOVER_ENR::_1
    }
}
#[doc = "Values that can be written to the field `RESTART_IFRTRIM_EN`"]
pub enum RESTART_IFRTRIM_ENW {
    #[doc = "Trim fine adjustment always works based on the previous updated trim fine value (default)"]
    _0,
    #[doc = "Trim fine restarts from the IFR trim value whenever bus_reset/bus_resume is detected or module enable is desasserted"]
    _1,
}
impl RESTART_IFRTRIM_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESTART_IFRTRIM_ENW::_0 => false,
            RESTART_IFRTRIM_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESTART_IFRTRIM_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RESTART_IFRTRIM_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESTART_IFRTRIM_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trim fine adjustment always works based on the previous updated trim fine value (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESTART_IFRTRIM_ENW::_0)
    }
    #[doc = "Trim fine restarts from the IFR trim value whenever bus_reset/bus_resume is detected or module enable is desasserted"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESTART_IFRTRIM_ENW::_1)
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
#[doc = "Values that can be written to the field `RESET_RESUME_ROUGH_EN`"]
pub enum RESET_RESUME_ROUGH_ENW {
    #[doc = "Always works in tracking phase after the 1st time rough to track transition (default)"]
    _0,
    #[doc = "Go back to rough stage whenever bus reset or bus resume occurs"]
    _1,
}
impl RESET_RESUME_ROUGH_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESET_RESUME_ROUGH_ENW::_0 => false,
            RESET_RESUME_ROUGH_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESET_RESUME_ROUGH_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_RESUME_ROUGH_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESET_RESUME_ROUGH_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Always works in tracking phase after the 1st time rough to track transition (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESET_RESUME_ROUGH_ENW::_0)
    }
    #[doc = "Go back to rough stage whenever bus reset or bus resume occurs"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESET_RESUME_ROUGH_ENW::_1)
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
#[doc = "Values that can be written to the field `CLOCK_RECOVER_EN`"]
pub enum CLOCK_RECOVER_ENW {
    #[doc = "Disable clock recovery block (default)"]
    _0,
    #[doc = "Enable clock recovery block"]
    _1,
}
impl CLOCK_RECOVER_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLOCK_RECOVER_ENW::_0 => false,
            CLOCK_RECOVER_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLOCK_RECOVER_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLOCK_RECOVER_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLOCK_RECOVER_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable clock recovery block (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLOCK_RECOVER_ENW::_0)
    }
    #[doc = "Enable clock recovery block"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLOCK_RECOVER_ENW::_1)
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
    #[doc = "Bit 5 - Restart from IFR trim value"]
    #[inline]
    pub fn restart_ifrtrim_en(&self) -> RESTART_IFRTRIM_ENR {
        RESTART_IFRTRIM_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Reset/resume to rough phase enable"]
    #[inline]
    pub fn reset_resume_rough_en(&self) -> RESET_RESUME_ROUGH_ENR {
        RESET_RESUME_ROUGH_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Crystal-less USB enable"]
    #[inline]
    pub fn clock_recover_en(&self) -> CLOCK_RECOVER_ENR {
        CLOCK_RECOVER_ENR::_from({
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
    #[doc = "Bit 5 - Restart from IFR trim value"]
    #[inline]
    pub fn restart_ifrtrim_en(&mut self) -> _RESTART_IFRTRIM_ENW {
        _RESTART_IFRTRIM_ENW { w: self }
    }
    #[doc = "Bit 6 - Reset/resume to rough phase enable"]
    #[inline]
    pub fn reset_resume_rough_en(&mut self) -> _RESET_RESUME_ROUGH_ENW {
        _RESET_RESUME_ROUGH_ENW { w: self }
    }
    #[doc = "Bit 7 - Crystal-less USB enable"]
    #[inline]
    pub fn clock_recover_en(&mut self) -> _CLOCK_RECOVER_ENW {
        _CLOCK_RECOVER_ENW { w: self }
    }
}
