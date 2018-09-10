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
#[doc = "Possible values of the field `CMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMODR {
    #[doc = "TPM counter is disabled"]
    _00,
    #[doc = "TPM counter increments on every TPM counter clock"]
    _01,
    #[doc = "TPM counter increments on rising edge of TPM_EXTCLK synchronized to the TPM counter clock"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMODR::_00 => 0,
            CMODR::_01 => 1,
            CMODR::_10 => 2,
            CMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMODR {
        match value {
            0 => CMODR::_00,
            1 => CMODR::_01,
            2 => CMODR::_10,
            i => CMODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == CMODR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == CMODR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == CMODR::_10
    }
}
#[doc = "Possible values of the field `CPWMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPWMSR {
    #[doc = "TPM counter operates in up counting mode."]
    _0,
    #[doc = "TPM counter operates in up-down counting mode."]
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
    #[doc = "Disable TOF interrupts. Use software polling or DMA request."]
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
    #[doc = "TPM counter has not overflowed."]
    _0,
    #[doc = "TPM counter has overflowed."]
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
#[doc = "Possible values of the field `DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR {
    #[doc = "Disables DMA transfers."]
    _0,
    #[doc = "Enables DMA transfers."]
    _1,
}
impl DMAR {
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
            DMAR::_0 => false,
            DMAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAR {
        match value {
            false => DMAR::_0,
            true => DMAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAR::_1
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
#[doc = "Values that can be written to the field `CMOD`"]
pub enum CMODW {
    #[doc = "TPM counter is disabled"]
    _00,
    #[doc = "TPM counter increments on every TPM counter clock"]
    _01,
    #[doc = "TPM counter increments on rising edge of TPM_EXTCLK synchronized to the TPM counter clock"]
    _10,
}
impl CMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMODW::_00 => 0,
            CMODW::_01 => 1,
            CMODW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMODW<'a> {
    w: &'a mut W,
}
impl<'a> _CMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TPM counter is disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMODW::_00)
    }
    #[doc = "TPM counter increments on every TPM counter clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMODW::_01)
    }
    #[doc = "TPM counter increments on rising edge of TPM_EXTCLK synchronized to the TPM counter clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMODW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPWMS`"]
pub enum CPWMSW {
    #[doc = "TPM counter operates in up counting mode."]
    _0,
    #[doc = "TPM counter operates in up-down counting mode."]
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
    #[doc = "TPM counter operates in up counting mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPWMSW::_0)
    }
    #[doc = "TPM counter operates in up-down counting mode."]
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
    #[doc = "Disable TOF interrupts. Use software polling or DMA request."]
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
    #[doc = "Disable TOF interrupts. Use software polling or DMA request."]
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
#[doc = "Values that can be written to the field `TOF`"]
pub enum TOFW {
    #[doc = "TPM counter has not overflowed."]
    _0,
    #[doc = "TPM counter has overflowed."]
    _1,
}
impl TOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOFW::_0 => false,
            TOFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOFW<'a> {
    w: &'a mut W,
}
impl<'a> _TOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TPM counter has not overflowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOFW::_0)
    }
    #[doc = "TPM counter has overflowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOFW::_1)
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
#[doc = "Values that can be written to the field `DMA`"]
pub enum DMAW {
    #[doc = "Disables DMA transfers."]
    _0,
    #[doc = "Enables DMA transfers."]
    _1,
}
impl DMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAW::_0 => false,
            DMAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables DMA transfers."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAW::_0)
    }
    #[doc = "Enables DMA transfers."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAW::_1)
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
    #[doc = "Bits 3:4 - Clock Mode Selection"]
    #[inline]
    pub fn cmod(&self) -> CMODR {
        CMODR::_from({
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
    #[doc = "Bit 8 - DMA Enable"]
    #[inline]
    pub fn dma(&self) -> DMAR {
        DMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bits 3:4 - Clock Mode Selection"]
    #[inline]
    pub fn cmod(&mut self) -> _CMODW {
        _CMODW { w: self }
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
    #[doc = "Bit 7 - Timer Overflow Flag"]
    #[inline]
    pub fn tof(&mut self) -> _TOFW {
        _TOFW { w: self }
    }
    #[doc = "Bit 8 - DMA Enable"]
    #[inline]
    pub fn dma(&mut self) -> _DMAW {
        _DMAW { w: self }
    }
}
