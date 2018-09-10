#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::OC {
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
#[doc = "Possible values of the field `IROPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IROPENR {
    #[doc = "The IRO signal is disabled."]
    _0,
    #[doc = "The IRO signal is enabled as output."]
    _1,
}
impl IROPENR {
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
            IROPENR::_0 => false,
            IROPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IROPENR {
        match value {
            false => IROPENR::_0,
            true => IROPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IROPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IROPENR::_1
    }
}
#[doc = "Possible values of the field `CMTPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTPOLR {
    #[doc = "The IRO signal is active-low."]
    _0,
    #[doc = "The IRO signal is active-high."]
    _1,
}
impl CMTPOLR {
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
            CMTPOLR::_0 => false,
            CMTPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMTPOLR {
        match value {
            false => CMTPOLR::_0,
            true => CMTPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CMTPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CMTPOLR::_1
    }
}
#[doc = r" Value of the field"]
pub struct IROLR {
    bits: bool,
}
impl IROLR {
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
#[doc = "Values that can be written to the field `IROPEN`"]
pub enum IROPENW {
    #[doc = "The IRO signal is disabled."]
    _0,
    #[doc = "The IRO signal is enabled as output."]
    _1,
}
impl IROPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IROPENW::_0 => false,
            IROPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IROPENW<'a> {
    w: &'a mut W,
}
impl<'a> _IROPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IROPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The IRO signal is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IROPENW::_0)
    }
    #[doc = "The IRO signal is enabled as output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IROPENW::_1)
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
#[doc = "Values that can be written to the field `CMTPOL`"]
pub enum CMTPOLW {
    #[doc = "The IRO signal is active-low."]
    _0,
    #[doc = "The IRO signal is active-high."]
    _1,
}
impl CMTPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMTPOLW::_0 => false,
            CMTPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMTPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CMTPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMTPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The IRO signal is active-low."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMTPOLW::_0)
    }
    #[doc = "The IRO signal is active-high."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMTPOLW::_1)
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
#[doc = r" Proxy"]
pub struct _IROLW<'a> {
    w: &'a mut W,
}
impl<'a> _IROLW<'a> {
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
    #[doc = "Bit 5 - IRO Pin Enable"]
    #[inline]
    pub fn iropen(&self) -> IROPENR {
        IROPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - CMT Output Polarity"]
    #[inline]
    pub fn cmtpol(&self) -> CMTPOLR {
        CMTPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - IRO Latch Control"]
    #[inline]
    pub fn irol(&self) -> IROLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        IROLR { bits }
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
    #[doc = "Bit 5 - IRO Pin Enable"]
    #[inline]
    pub fn iropen(&mut self) -> _IROPENW {
        _IROPENW { w: self }
    }
    #[doc = "Bit 6 - CMT Output Polarity"]
    #[inline]
    pub fn cmtpol(&mut self) -> _CMTPOLW {
        _CMTPOLW { w: self }
    }
    #[doc = "Bit 7 - IRO Latch Control"]
    #[inline]
    pub fn irol(&mut self) -> _IROLW {
        _IROLW { w: self }
    }
}
