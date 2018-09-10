#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::FILT2 {
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
#[doc = "Possible values of the field `FILTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTSELR {
    #[doc = "Select LLWU_P0 for filter"]
    _00000,
    #[doc = "Select LLWU_P31 for filter"]
    _11111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FILTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FILTSELR::_00000 => 0,
            FILTSELR::_11111 => 31,
            FILTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FILTSELR {
        match value {
            0 => FILTSELR::_00000,
            31 => FILTSELR::_11111,
            i => FILTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline]
    pub fn is_00000(&self) -> bool {
        *self == FILTSELR::_00000
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline]
    pub fn is_11111(&self) -> bool {
        *self == FILTSELR::_11111
    }
}
#[doc = "Possible values of the field `FILTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTER {
    #[doc = "Filter disabled"]
    _00,
    #[doc = "Filter posedge detect enabled"]
    _01,
    #[doc = "Filter negedge detect enabled"]
    _10,
    #[doc = "Filter any edge detect enabled"]
    _11,
}
impl FILTER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FILTER::_00 => 0,
            FILTER::_01 => 1,
            FILTER::_10 => 2,
            FILTER::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FILTER {
        match value {
            0 => FILTER::_00,
            1 => FILTER::_01,
            2 => FILTER::_10,
            3 => FILTER::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FILTER::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FILTER::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FILTER::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FILTER::_11
    }
}
#[doc = "Possible values of the field `FILTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTFR {
    #[doc = "Pin Filter 2 was not a wakeup source"]
    _0,
    #[doc = "Pin Filter 2 was a wakeup source"]
    _1,
}
impl FILTFR {
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
            FILTFR::_0 => false,
            FILTFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FILTFR {
        match value {
            false => FILTFR::_0,
            true => FILTFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FILTFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FILTFR::_1
    }
}
#[doc = "Values that can be written to the field `FILTSEL`"]
pub enum FILTSELW {
    #[doc = "Select LLWU_P0 for filter"]
    _00000,
    #[doc = "Select LLWU_P31 for filter"]
    _11111,
}
impl FILTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FILTSELW::_00000 => 0,
            FILTSELW::_11111 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Select LLWU_P0 for filter"]
    #[inline]
    pub fn _00000(self) -> &'a mut W {
        self.variant(FILTSELW::_00000)
    }
    #[doc = "Select LLWU_P31 for filter"]
    #[inline]
    pub fn _11111(self) -> &'a mut W {
        self.variant(FILTSELW::_11111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FILTE`"]
pub enum FILTEW {
    #[doc = "Filter disabled"]
    _00,
    #[doc = "Filter posedge detect enabled"]
    _01,
    #[doc = "Filter negedge detect enabled"]
    _10,
    #[doc = "Filter any edge detect enabled"]
    _11,
}
impl FILTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FILTEW::_00 => 0,
            FILTEW::_01 => 1,
            FILTEW::_10 => 2,
            FILTEW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTEW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Filter disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FILTEW::_00)
    }
    #[doc = "Filter posedge detect enabled"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FILTEW::_01)
    }
    #[doc = "Filter negedge detect enabled"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FILTEW::_10)
    }
    #[doc = "Filter any edge detect enabled"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FILTEW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FILTF`"]
pub enum FILTFW {
    #[doc = "Pin Filter 2 was not a wakeup source"]
    _0,
    #[doc = "Pin Filter 2 was a wakeup source"]
    _1,
}
impl FILTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTFW::_0 => false,
            FILTFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTFW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Filter 2 was not a wakeup source"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTFW::_0)
    }
    #[doc = "Pin Filter 2 was a wakeup source"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTFW::_1)
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
    #[doc = "Bits 0:4 - Filter Pin Select"]
    #[inline]
    pub fn filtsel(&self) -> FILTSELR {
        FILTSELR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 5:6 - Digital Filter On External Pin"]
    #[inline]
    pub fn filte(&self) -> FILTER {
        FILTER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 7 - Filter Detect Flag"]
    #[inline]
    pub fn filtf(&self) -> FILTFR {
        FILTFR::_from({
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
    #[doc = "Bits 0:4 - Filter Pin Select"]
    #[inline]
    pub fn filtsel(&mut self) -> _FILTSELW {
        _FILTSELW { w: self }
    }
    #[doc = "Bits 5:6 - Digital Filter On External Pin"]
    #[inline]
    pub fn filte(&mut self) -> _FILTEW {
        _FILTEW { w: self }
    }
    #[doc = "Bit 7 - Filter Detect Flag"]
    #[inline]
    pub fn filtf(&mut self) -> _FILTFW {
        _FILTFW { w: self }
    }
}
