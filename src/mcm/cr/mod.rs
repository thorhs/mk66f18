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
#[doc = "Possible values of the field `SRAMUAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMUAPR {
    #[doc = "Round robin"]
    _00,
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    _01,
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    _10,
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    _11,
}
impl SRAMUAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRAMUAPR::_00 => 0,
            SRAMUAPR::_01 => 1,
            SRAMUAPR::_10 => 2,
            SRAMUAPR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRAMUAPR {
        match value {
            0 => SRAMUAPR::_00,
            1 => SRAMUAPR::_01,
            2 => SRAMUAPR::_10,
            3 => SRAMUAPR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == SRAMUAPR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == SRAMUAPR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SRAMUAPR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SRAMUAPR::_11
    }
}
#[doc = r" Value of the field"]
pub struct SRAMUWPR {
    bits: bool,
}
impl SRAMUWPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `SRAMLAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMLAPR {
    #[doc = "Round robin"]
    _00,
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    _01,
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    _10,
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    _11,
}
impl SRAMLAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRAMLAPR::_00 => 0,
            SRAMLAPR::_01 => 1,
            SRAMLAPR::_10 => 2,
            SRAMLAPR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRAMLAPR {
        match value {
            0 => SRAMLAPR::_00,
            1 => SRAMLAPR::_01,
            2 => SRAMLAPR::_10,
            3 => SRAMLAPR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == SRAMLAPR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == SRAMLAPR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SRAMLAPR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SRAMLAPR::_11
    }
}
#[doc = r" Value of the field"]
pub struct SRAMLWPR {
    bits: bool,
}
impl SRAMLWPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `SRAMUAP`"]
pub enum SRAMUAPW {
    #[doc = "Round robin"]
    _00,
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    _01,
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    _10,
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    _11,
}
impl SRAMUAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRAMUAPW::_00 => 0,
            SRAMUAPW::_01 => 1,
            SRAMUAPW::_10 => 2,
            SRAMUAPW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAMUAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMUAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAMUAPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Round robin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(SRAMUAPW::_00)
    }
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(SRAMUAPW::_01)
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SRAMUAPW::_10)
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(SRAMUAPW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRAMUWPW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMUWPW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRAMLAP`"]
pub enum SRAMLAPW {
    #[doc = "Round robin"]
    _00,
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    _01,
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    _10,
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    _11,
}
impl SRAMLAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRAMLAPW::_00 => 0,
            SRAMLAPW::_01 => 1,
            SRAMLAPW::_10 => 2,
            SRAMLAPW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAMLAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMLAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAMLAPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Round robin"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(SRAMLAPW::_00)
    }
    #[doc = "Special round robin (favors SRAM backoor accesses over the processor)"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(SRAMLAPW::_01)
    }
    #[doc = "Fixed priority. Processor has highest, backdoor has lowest"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SRAMLAPW::_10)
    }
    #[doc = "Fixed priority. Backdoor has highest, processor has lowest"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(SRAMLAPW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRAMLWPW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMLWPW<'a> {
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 24:25 - SRAM_U arbitration priority"]
    #[inline]
    pub fn sramuap(&self) -> SRAMUAPR {
        SRAMUAPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - SRAM_U write protect"]
    #[inline]
    pub fn sramuwp(&self) -> SRAMUWPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRAMUWPR { bits }
    }
    #[doc = "Bits 28:29 - SRAM_L arbitration priority"]
    #[inline]
    pub fn sramlap(&self) -> SRAMLAPR {
        SRAMLAPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - SRAM_L Write Protect"]
    #[inline]
    pub fn sramlwp(&self) -> SRAMLWPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRAMLWPR { bits }
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
    #[doc = "Bits 24:25 - SRAM_U arbitration priority"]
    #[inline]
    pub fn sramuap(&mut self) -> _SRAMUAPW {
        _SRAMUAPW { w: self }
    }
    #[doc = "Bit 26 - SRAM_U write protect"]
    #[inline]
    pub fn sramuwp(&mut self) -> _SRAMUWPW {
        _SRAMUWPW { w: self }
    }
    #[doc = "Bits 28:29 - SRAM_L arbitration priority"]
    #[inline]
    pub fn sramlap(&mut self) -> _SRAMLAPW {
        _SRAMLAPW { w: self }
    }
    #[doc = "Bit 30 - SRAM_L Write Protect"]
    #[inline]
    pub fn sramlwp(&mut self) -> _SRAMLWPW {
        _SRAMLWPW { w: self }
    }
}
