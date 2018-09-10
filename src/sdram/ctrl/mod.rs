#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CTRL {
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
pub struct RCR {
    bits: u16,
}
impl RCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `RTIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTIMR {
    #[doc = "3 clocks"]
    _00,
    #[doc = "6 clocks"]
    _01,
    #[doc = "9 clocks"]
    _10,
    #[doc = "9 clocks"]
    _11,
}
impl RTIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTIMR::_00 => 0,
            RTIMR::_01 => 1,
            RTIMR::_10 => 2,
            RTIMR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTIMR {
        match value {
            0 => RTIMR::_00,
            1 => RTIMR::_01,
            2 => RTIMR::_10,
            3 => RTIMR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RTIMR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RTIMR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RTIMR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RTIMR::_11
    }
}
#[doc = "Possible values of the field `IS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISR {
    #[doc = "Take no action or issue a selfx command to exit self refresh."]
    _0,
    #[doc = "SDRAM controller sends a self command to both SDRAM blocks to put them in low-power, self-refresh state where they remain until IS is cleared. When IS is cleared, the controller sends a selfx command for the SDRAMs to exit self-refresh. The refresh counter is suspended while the SDRAMs are in self-refresh; the SDRAM controls the refresh period."]
    _1,
}
impl ISR {
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
            ISR::_0 => false,
            ISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISR {
        match value {
            false => ISR::_0,
            true => ISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ISR::_1
    }
}
#[doc = r" Proxy"]
pub struct _RCW<'a> {
    w: &'a mut W,
}
impl<'a> _RCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTIM`"]
pub enum RTIMW {
    #[doc = "3 clocks"]
    _00,
    #[doc = "6 clocks"]
    _01,
    #[doc = "9 clocks"]
    _10,
    #[doc = "9 clocks"]
    _11,
}
impl RTIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTIMW::_00 => 0,
            RTIMW::_01 => 1,
            RTIMW::_10 => 2,
            RTIMW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RTIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "3 clocks"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RTIMW::_00)
    }
    #[doc = "6 clocks"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RTIMW::_01)
    }
    #[doc = "9 clocks"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RTIMW::_10)
    }
    #[doc = "9 clocks"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RTIMW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IS`"]
pub enum ISW {
    #[doc = "Take no action or issue a selfx command to exit self refresh."]
    _0,
    #[doc = "SDRAM controller sends a self command to both SDRAM blocks to put them in low-power, self-refresh state where they remain until IS is cleared. When IS is cleared, the controller sends a selfx command for the SDRAMs to exit self-refresh. The refresh counter is suspended while the SDRAMs are in self-refresh; the SDRAM controls the refresh period."]
    _1,
}
impl ISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISW::_0 => false,
            ISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISW<'a> {
    w: &'a mut W,
}
impl<'a> _ISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Take no action or issue a selfx command to exit self refresh."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISW::_0)
    }
    #[doc = "SDRAM controller sends a self command to both SDRAM blocks to put them in low-power, self-refresh state where they remain until IS is cleared. When IS is cleared, the controller sends a selfx command for the SDRAMs to exit self-refresh. The refresh counter is suspended while the SDRAMs are in self-refresh; the SDRAM controls the refresh period."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISW::_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:8 - Refresh count"]
    #[inline]
    pub fn rc(&self) -> RCR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u16
        };
        RCR { bits }
    }
    #[doc = "Bits 9:10 - Refresh timing"]
    #[inline]
    pub fn rtim(&self) -> RTIMR {
        RTIMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 11 - Initiate self-refresh command."]
    #[inline]
    pub fn is(&self) -> ISR {
        ISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:8 - Refresh count"]
    #[inline]
    pub fn rc(&mut self) -> _RCW {
        _RCW { w: self }
    }
    #[doc = "Bits 9:10 - Refresh timing"]
    #[inline]
    pub fn rtim(&mut self) -> _RTIMW {
        _RTIMW { w: self }
    }
    #[doc = "Bit 11 - Initiate self-refresh command."]
    #[inline]
    pub fn is(&mut self) -> _ISW {
        _ISW { w: self }
    }
}
