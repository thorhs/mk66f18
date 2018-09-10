#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG2 {
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
#[doc = "Possible values of the field `ADLSTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLSTSR {
    #[doc = "Default longest sample time; 20 extra ADCK cycles; 24 ADCK cycles total."]
    _00,
    #[doc = "12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    _01,
    #[doc = "6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    _10,
    #[doc = "2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    _11,
}
impl ADLSTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADLSTSR::_00 => 0,
            ADLSTSR::_01 => 1,
            ADLSTSR::_10 => 2,
            ADLSTSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADLSTSR {
        match value {
            0 => ADLSTSR::_00,
            1 => ADLSTSR::_01,
            2 => ADLSTSR::_10,
            3 => ADLSTSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ADLSTSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ADLSTSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ADLSTSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == ADLSTSR::_11
    }
}
#[doc = "Possible values of the field `ADHSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADHSCR {
    #[doc = "Normal conversion sequence selected."]
    _0,
    #[doc = "High-speed conversion sequence selected with 2 additional ADCK cycles to total conversion time."]
    _1,
}
impl ADHSCR {
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
            ADHSCR::_0 => false,
            ADHSCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADHSCR {
        match value {
            false => ADHSCR::_0,
            true => ADHSCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADHSCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADHSCR::_1
    }
}
#[doc = "Possible values of the field `ADACKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADACKENR {
    #[doc = "Asynchronous clock output disabled; Asynchronous clock is enabled only if selected by ADICLK and a conversion is active."]
    _0,
    #[doc = "Asynchronous clock and clock output is enabled regardless of the state of the ADC."]
    _1,
}
impl ADACKENR {
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
            ADACKENR::_0 => false,
            ADACKENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADACKENR {
        match value {
            false => ADACKENR::_0,
            true => ADACKENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADACKENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADACKENR::_1
    }
}
#[doc = "Possible values of the field `MUXSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUXSELR {
    #[doc = "ADxxa channels are selected."]
    _0,
    #[doc = "ADxxb channels are selected."]
    _1,
}
impl MUXSELR {
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
            MUXSELR::_0 => false,
            MUXSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MUXSELR {
        match value {
            false => MUXSELR::_0,
            true => MUXSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MUXSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MUXSELR::_1
    }
}
#[doc = "Values that can be written to the field `ADLSTS`"]
pub enum ADLSTSW {
    #[doc = "Default longest sample time; 20 extra ADCK cycles; 24 ADCK cycles total."]
    _00,
    #[doc = "12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    _01,
    #[doc = "6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    _10,
    #[doc = "2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    _11,
}
impl ADLSTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADLSTSW::_00 => 0,
            ADLSTSW::_01 => 1,
            ADLSTSW::_10 => 2,
            ADLSTSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADLSTSW<'a> {
    w: &'a mut W,
}
impl<'a> _ADLSTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADLSTSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Default longest sample time; 20 extra ADCK cycles; 24 ADCK cycles total."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADLSTSW::_00)
    }
    #[doc = "12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADLSTSW::_01)
    }
    #[doc = "6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADLSTSW::_10)
    }
    #[doc = "2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADLSTSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADHSC`"]
pub enum ADHSCW {
    #[doc = "Normal conversion sequence selected."]
    _0,
    #[doc = "High-speed conversion sequence selected with 2 additional ADCK cycles to total conversion time."]
    _1,
}
impl ADHSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADHSCW::_0 => false,
            ADHSCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADHSCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADHSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADHSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal conversion sequence selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADHSCW::_0)
    }
    #[doc = "High-speed conversion sequence selected with 2 additional ADCK cycles to total conversion time."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADHSCW::_1)
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
#[doc = "Values that can be written to the field `ADACKEN`"]
pub enum ADACKENW {
    #[doc = "Asynchronous clock output disabled; Asynchronous clock is enabled only if selected by ADICLK and a conversion is active."]
    _0,
    #[doc = "Asynchronous clock and clock output is enabled regardless of the state of the ADC."]
    _1,
}
impl ADACKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADACKENW::_0 => false,
            ADACKENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADACKENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADACKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADACKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Asynchronous clock output disabled; Asynchronous clock is enabled only if selected by ADICLK and a conversion is active."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADACKENW::_0)
    }
    #[doc = "Asynchronous clock and clock output is enabled regardless of the state of the ADC."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADACKENW::_1)
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
#[doc = "Values that can be written to the field `MUXSEL`"]
pub enum MUXSELW {
    #[doc = "ADxxa channels are selected."]
    _0,
    #[doc = "ADxxb channels are selected."]
    _1,
}
impl MUXSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MUXSELW::_0 => false,
            MUXSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MUXSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MUXSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MUXSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ADxxa channels are selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MUXSELW::_0)
    }
    #[doc = "ADxxb channels are selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MUXSELW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Long Sample Time Select"]
    #[inline]
    pub fn adlsts(&self) -> ADLSTSR {
        ADLSTSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - High-Speed Configuration"]
    #[inline]
    pub fn adhsc(&self) -> ADHSCR {
        ADHSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Asynchronous Clock Output Enable"]
    #[inline]
    pub fn adacken(&self) -> ADACKENR {
        ADACKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - ADC Mux Select"]
    #[inline]
    pub fn muxsel(&self) -> MUXSELR {
        MUXSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - Long Sample Time Select"]
    #[inline]
    pub fn adlsts(&mut self) -> _ADLSTSW {
        _ADLSTSW { w: self }
    }
    #[doc = "Bit 2 - High-Speed Configuration"]
    #[inline]
    pub fn adhsc(&mut self) -> _ADHSCW {
        _ADHSCW { w: self }
    }
    #[doc = "Bit 3 - Asynchronous Clock Output Enable"]
    #[inline]
    pub fn adacken(&mut self) -> _ADACKENW {
        _ADACKENW { w: self }
    }
    #[doc = "Bit 4 - ADC Mux Select"]
    #[inline]
    pub fn muxsel(&mut self) -> _MUXSELW {
        _MUXSELW { w: self }
    }
}
