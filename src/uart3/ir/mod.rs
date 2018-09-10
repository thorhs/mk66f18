#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::IR {
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
#[doc = "Possible values of the field `TNP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNPR {
    #[doc = "3/16."]
    _00,
    #[doc = "1/16."]
    _01,
    #[doc = "1/32."]
    _10,
    #[doc = "1/4."]
    _11,
}
impl TNPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TNPR::_00 => 0,
            TNPR::_01 => 1,
            TNPR::_10 => 2,
            TNPR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TNPR {
        match value {
            0 => TNPR::_00,
            1 => TNPR::_01,
            2 => TNPR::_10,
            3 => TNPR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TNPR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TNPR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TNPR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TNPR::_11
    }
}
#[doc = "Possible values of the field `IREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRENR {
    #[doc = "IR disabled."]
    _0,
    #[doc = "IR enabled."]
    _1,
}
impl IRENR {
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
            IRENR::_0 => false,
            IRENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRENR {
        match value {
            false => IRENR::_0,
            true => IRENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IRENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IRENR::_1
    }
}
#[doc = "Values that can be written to the field `TNP`"]
pub enum TNPW {
    #[doc = "3/16."]
    _00,
    #[doc = "1/16."]
    _01,
    #[doc = "1/32."]
    _10,
    #[doc = "1/4."]
    _11,
}
impl TNPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TNPW::_00 => 0,
            TNPW::_01 => 1,
            TNPW::_10 => 2,
            TNPW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TNPW<'a> {
    w: &'a mut W,
}
impl<'a> _TNPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TNPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "3/16."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TNPW::_00)
    }
    #[doc = "1/16."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TNPW::_01)
    }
    #[doc = "1/32."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TNPW::_10)
    }
    #[doc = "1/4."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TNPW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IREN`"]
pub enum IRENW {
    #[doc = "IR disabled."]
    _0,
    #[doc = "IR enabled."]
    _1,
}
impl IRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRENW::_0 => false,
            IRENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRENW<'a> {
    w: &'a mut W,
}
impl<'a> _IRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IR disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRENW::_0)
    }
    #[doc = "IR enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:1 - Transmitter narrow pulse"]
    #[inline]
    pub fn tnp(&self) -> TNPR {
        TNPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 2 - Infrared enable"]
    #[inline]
    pub fn iren(&self) -> IRENR {
        IRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bits 0:1 - Transmitter narrow pulse"]
    #[inline]
    pub fn tnp(&mut self) -> _TNPW {
        _TNPW { w: self }
    }
    #[doc = "Bit 2 - Infrared enable"]
    #[inline]
    pub fn iren(&mut self) -> _IRENW {
        _IRENW { w: self }
    }
}
