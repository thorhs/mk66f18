#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
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
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "Divide by 1"]
    _000,
    #[doc = "Divide by 2"]
    _001,
    #[doc = "Divide by 4"]
    _010,
    #[doc = "Divide by 8"]
    _011,
    #[doc = "Divide by 16"]
    _100,
    #[doc = "Divide by 32"]
    _101,
    #[doc = "Divide by 64"]
    _110,
    #[doc = "Divide by 128"]
    _111,
}
impl PSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSR::_000 => 0,
            PSR::_001 => 1,
            PSR::_010 => 2,
            PSR::_011 => 3,
            PSR::_100 => 4,
            PSR::_101 => 5,
            PSR::_110 => 6,
            PSR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSR {
        match value {
            0 => PSR::_000,
            1 => PSR::_001,
            2 => PSR::_010,
            3 => PSR::_011,
            4 => PSR::_100,
            5 => PSR::_101,
            6 => PSR::_110,
            7 => PSR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PSR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PSR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PSR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PSR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PSR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PSR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PSR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PSR::_111
    }
}
#[doc = "Possible values of the field `CLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSR {
    #[doc = "No clock selected. This in effect disables the FTM counter."]
    _00,
    #[doc = "System clock"]
    _01,
    #[doc = "Fixed frequency clock"]
    _10,
    #[doc = "External clock"]
    _11,
}
impl CLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSR::_00 => 0,
            CLKSR::_01 => 1,
            CLKSR::_10 => 2,
            CLKSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSR {
        match value {
            0 => CLKSR::_00,
            1 => CLKSR::_01,
            2 => CLKSR::_10,
            3 => CLKSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == CLKSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == CLKSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == CLKSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == CLKSR::_11
    }
}
#[doc = "Possible values of the field `CPWMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPWMSR {
    #[doc = "FTM counter operates in Up Counting mode."]
    _0,
    #[doc = "FTM counter operates in Up-Down Counting mode."]
    _1,
}
impl CPWMSR {
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
            CPWMSR::_0 => false,
            CPWMSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPWMSR {
        match value {
            false => CPWMSR::_0,
            true => CPWMSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPWMSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPWMSR::_1
    }
}
#[doc = "Possible values of the field `TOIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIER {
    #[doc = "Disable TOF interrupts. Use software polling."]
    _0,
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
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
#[doc = "Possible values of the field `TOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOFR {
    #[doc = "FTM counter has not overflowed."]
    _0,
    #[doc = "FTM counter has overflowed."]
    _1,
}
impl TOFR {
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
            TOFR::_0 => false,
            TOFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOFR {
        match value {
            false => TOFR::_0,
            true => TOFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOFR::_1
    }
}
#[doc = "Values that can be written to the field `PS`"]
pub enum PSW {
    #[doc = "Divide by 1"]
    _000,
    #[doc = "Divide by 2"]
    _001,
    #[doc = "Divide by 4"]
    _010,
    #[doc = "Divide by 8"]
    _011,
    #[doc = "Divide by 16"]
    _100,
    #[doc = "Divide by 32"]
    _101,
    #[doc = "Divide by 64"]
    _110,
    #[doc = "Divide by 128"]
    _111,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSW::_000 => 0,
            PSW::_001 => 1,
            PSW::_010 => 2,
            PSW::_011 => 3,
            PSW::_100 => 4,
            PSW::_101 => 5,
            PSW::_110 => 6,
            PSW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PSW::_000)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PSW::_001)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PSW::_010)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PSW::_011)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PSW::_100)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PSW::_101)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PSW::_110)
    }
    #[doc = "Divide by 128"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PSW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKS`"]
pub enum CLKSW {
    #[doc = "No clock selected. This in effect disables the FTM counter."]
    _00,
    #[doc = "System clock"]
    _01,
    #[doc = "Fixed frequency clock"]
    _10,
    #[doc = "External clock"]
    _11,
}
impl CLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSW::_00 => 0,
            CLKSW::_01 => 1,
            CLKSW::_10 => 2,
            CLKSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No clock selected. This in effect disables the FTM counter."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKSW::_00)
    }
    #[doc = "System clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKSW::_01)
    }
    #[doc = "Fixed frequency clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKSW::_10)
    }
    #[doc = "External clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPWMS`"]
pub enum CPWMSW {
    #[doc = "FTM counter operates in Up Counting mode."]
    _0,
    #[doc = "FTM counter operates in Up-Down Counting mode."]
    _1,
}
impl CPWMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPWMSW::_0 => false,
            CPWMSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPWMSW<'a> {
    w: &'a mut W,
}
impl<'a> _CPWMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPWMSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM counter operates in Up Counting mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPWMSW::_0)
    }
    #[doc = "FTM counter operates in Up-Down Counting mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPWMSW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOIE`"]
pub enum TOIEW {
    #[doc = "Disable TOF interrupts. Use software polling."]
    _0,
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
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
    #[doc = "Disable TOF interrupts. Use software polling."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIEW::_0)
    }
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
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
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - Clock Source Selection"]
    #[inline]
    pub fn clks(&self) -> CLKSR {
        CLKSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline]
    pub fn cpwms(&self) -> CPWMSR {
        CPWMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
    #[inline]
    pub fn toie(&self) -> TOIER {
        TOIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Timer Overflow Flag"]
    #[inline]
    pub fn tof(&self) -> TOFR {
        TOFR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bits 3:4 - Clock Source Selection"]
    #[inline]
    pub fn clks(&mut self) -> _CLKSW {
        _CLKSW { w: self }
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline]
    pub fn cpwms(&mut self) -> _CPWMSW {
        _CPWMSW { w: self }
    }
    #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
    #[inline]
    pub fn toie(&mut self) -> _TOIEW {
        _TOIEW { w: self }
    }
}
