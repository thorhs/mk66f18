#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::MUXCR {
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
#[doc = "Possible values of the field `MSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSELR {
    #[doc = "IN0"]
    _000,
    #[doc = "IN1"]
    _001,
    #[doc = "IN2"]
    _010,
    #[doc = "IN3"]
    _011,
    #[doc = "IN4"]
    _100,
    #[doc = "IN5"]
    _101,
    #[doc = "IN6"]
    _110,
    #[doc = "IN7"]
    _111,
}
impl MSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSELR::_000 => 0,
            MSELR::_001 => 1,
            MSELR::_010 => 2,
            MSELR::_011 => 3,
            MSELR::_100 => 4,
            MSELR::_101 => 5,
            MSELR::_110 => 6,
            MSELR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSELR {
        match value {
            0 => MSELR::_000,
            1 => MSELR::_001,
            2 => MSELR::_010,
            3 => MSELR::_011,
            4 => MSELR::_100,
            5 => MSELR::_101,
            6 => MSELR::_110,
            7 => MSELR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == MSELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == MSELR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == MSELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == MSELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == MSELR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == MSELR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == MSELR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == MSELR::_111
    }
}
#[doc = "Possible values of the field `PSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELR {
    #[doc = "IN0"]
    _000,
    #[doc = "IN1"]
    _001,
    #[doc = "IN2"]
    _010,
    #[doc = "IN3"]
    _011,
    #[doc = "IN4"]
    _100,
    #[doc = "IN5"]
    _101,
    #[doc = "IN6"]
    _110,
    #[doc = "IN7"]
    _111,
}
impl PSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSELR::_000 => 0,
            PSELR::_001 => 1,
            PSELR::_010 => 2,
            PSELR::_011 => 3,
            PSELR::_100 => 4,
            PSELR::_101 => 5,
            PSELR::_110 => 6,
            PSELR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSELR {
        match value {
            0 => PSELR::_000,
            1 => PSELR::_001,
            2 => PSELR::_010,
            3 => PSELR::_011,
            4 => PSELR::_100,
            5 => PSELR::_101,
            6 => PSELR::_110,
            7 => PSELR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PSELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PSELR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PSELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PSELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PSELR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PSELR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PSELR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PSELR::_111
    }
}
#[doc = "Possible values of the field `PSTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSTMR {
    #[doc = "Pass Through Mode is disabled."]
    _0,
    #[doc = "Pass Through Mode is enabled."]
    _1,
}
impl PSTMR {
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
            PSTMR::_0 => false,
            PSTMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSTMR {
        match value {
            false => PSTMR::_0,
            true => PSTMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PSTMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PSTMR::_1
    }
}
#[doc = "Values that can be written to the field `MSEL`"]
pub enum MSELW {
    #[doc = "IN0"]
    _000,
    #[doc = "IN1"]
    _001,
    #[doc = "IN2"]
    _010,
    #[doc = "IN3"]
    _011,
    #[doc = "IN4"]
    _100,
    #[doc = "IN5"]
    _101,
    #[doc = "IN6"]
    _110,
    #[doc = "IN7"]
    _111,
}
impl MSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSELW::_000 => 0,
            MSELW::_001 => 1,
            MSELW::_010 => 2,
            MSELW::_011 => 3,
            MSELW::_100 => 4,
            MSELW::_101 => 5,
            MSELW::_110 => 6,
            MSELW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IN0"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(MSELW::_000)
    }
    #[doc = "IN1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(MSELW::_001)
    }
    #[doc = "IN2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(MSELW::_010)
    }
    #[doc = "IN3"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(MSELW::_011)
    }
    #[doc = "IN4"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(MSELW::_100)
    }
    #[doc = "IN5"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(MSELW::_101)
    }
    #[doc = "IN6"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(MSELW::_110)
    }
    #[doc = "IN7"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(MSELW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSEL`"]
pub enum PSELW {
    #[doc = "IN0"]
    _000,
    #[doc = "IN1"]
    _001,
    #[doc = "IN2"]
    _010,
    #[doc = "IN3"]
    _011,
    #[doc = "IN4"]
    _100,
    #[doc = "IN5"]
    _101,
    #[doc = "IN6"]
    _110,
    #[doc = "IN7"]
    _111,
}
impl PSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSELW::_000 => 0,
            PSELW::_001 => 1,
            PSELW::_010 => 2,
            PSELW::_011 => 3,
            PSELW::_100 => 4,
            PSELW::_101 => 5,
            PSELW::_110 => 6,
            PSELW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IN0"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PSELW::_000)
    }
    #[doc = "IN1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PSELW::_001)
    }
    #[doc = "IN2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PSELW::_010)
    }
    #[doc = "IN3"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PSELW::_011)
    }
    #[doc = "IN4"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PSELW::_100)
    }
    #[doc = "IN5"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PSELW::_101)
    }
    #[doc = "IN6"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PSELW::_110)
    }
    #[doc = "IN7"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PSELW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSTM`"]
pub enum PSTMW {
    #[doc = "Pass Through Mode is disabled."]
    _0,
    #[doc = "Pass Through Mode is enabled."]
    _1,
}
impl PSTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSTMW::_0 => false,
            PSTMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSTMW<'a> {
    w: &'a mut W,
}
impl<'a> _PSTMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSTMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pass Through Mode is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSTMW::_0)
    }
    #[doc = "Pass Through Mode is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSTMW::_1)
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
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline]
    pub fn msel(&self) -> MSELR {
        MSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline]
    pub fn psel(&self) -> PSELR {
        PSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 7 - Pass Through Mode Enable"]
    #[inline]
    pub fn pstm(&self) -> PSTMR {
        PSTMR::_from({
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
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline]
    pub fn msel(&mut self) -> _MSELW {
        _MSELW { w: self }
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
    #[doc = "Bit 7 - Pass Through Mode Enable"]
    #[inline]
    pub fn pstm(&mut self) -> _PSTMW {
        _PSTMW { w: self }
    }
}
