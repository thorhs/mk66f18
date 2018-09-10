#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TFWR {
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
#[doc = "Possible values of the field `TFWR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFWRR {
    #[doc = "64 bytes written."]
    _000000,
    #[doc = "64 bytes written."]
    _000001,
    #[doc = "128 bytes written."]
    _000010,
    #[doc = "192 bytes written."]
    _000011,
    #[doc = "1984 bytes written."]
    _011111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TFWRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TFWRR::_000000 => 0,
            TFWRR::_000001 => 1,
            TFWRR::_000010 => 2,
            TFWRR::_000011 => 3,
            TFWRR::_011111 => 31,
            TFWRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TFWRR {
        match value {
            0 => TFWRR::_000000,
            1 => TFWRR::_000001,
            2 => TFWRR::_000010,
            3 => TFWRR::_000011,
            31 => TFWRR::_011111,
            i => TFWRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline]
    pub fn is_000000(&self) -> bool {
        *self == TFWRR::_000000
    }
    #[doc = "Checks if the value of the field is `_000001`"]
    #[inline]
    pub fn is_000001(&self) -> bool {
        *self == TFWRR::_000001
    }
    #[doc = "Checks if the value of the field is `_000010`"]
    #[inline]
    pub fn is_000010(&self) -> bool {
        *self == TFWRR::_000010
    }
    #[doc = "Checks if the value of the field is `_000011`"]
    #[inline]
    pub fn is_000011(&self) -> bool {
        *self == TFWRR::_000011
    }
    #[doc = "Checks if the value of the field is `_011111`"]
    #[inline]
    pub fn is_011111(&self) -> bool {
        *self == TFWRR::_011111
    }
}
#[doc = "Possible values of the field `STRFWD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRFWDR {
    #[doc = "Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
    _0,
    #[doc = "Enabled."]
    _1,
}
impl STRFWDR {
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
            STRFWDR::_0 => false,
            STRFWDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STRFWDR {
        match value {
            false => STRFWDR::_0,
            true => STRFWDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STRFWDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STRFWDR::_1
    }
}
#[doc = "Values that can be written to the field `TFWR`"]
pub enum TFWRW {
    #[doc = "64 bytes written."]
    _000000,
    #[doc = "64 bytes written."]
    _000001,
    #[doc = "128 bytes written."]
    _000010,
    #[doc = "192 bytes written."]
    _000011,
    #[doc = "1984 bytes written."]
    _011111,
}
impl TFWRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TFWRW::_000000 => 0,
            TFWRW::_000001 => 1,
            TFWRW::_000010 => 2,
            TFWRW::_000011 => 3,
            TFWRW::_011111 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFWRW<'a> {
    w: &'a mut W,
}
impl<'a> _TFWRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFWRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "64 bytes written."]
    #[inline]
    pub fn _000000(self) -> &'a mut W {
        self.variant(TFWRW::_000000)
    }
    #[doc = "64 bytes written."]
    #[inline]
    pub fn _000001(self) -> &'a mut W {
        self.variant(TFWRW::_000001)
    }
    #[doc = "128 bytes written."]
    #[inline]
    pub fn _000010(self) -> &'a mut W {
        self.variant(TFWRW::_000010)
    }
    #[doc = "192 bytes written."]
    #[inline]
    pub fn _000011(self) -> &'a mut W {
        self.variant(TFWRW::_000011)
    }
    #[doc = "1984 bytes written."]
    #[inline]
    pub fn _011111(self) -> &'a mut W {
        self.variant(TFWRW::_011111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STRFWD`"]
pub enum STRFWDW {
    #[doc = "Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
    _0,
    #[doc = "Enabled."]
    _1,
}
impl STRFWDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STRFWDW::_0 => false,
            STRFWDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STRFWDW<'a> {
    w: &'a mut W,
}
impl<'a> _STRFWDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STRFWDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STRFWDW::_0)
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STRFWDW::_1)
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
    #[doc = "Bits 0:5 - Transmit FIFO Write"]
    #[inline]
    pub fn tfwr(&self) -> TFWRR {
        TFWRR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Store And Forward Enable"]
    #[inline]
    pub fn strfwd(&self) -> STRFWDR {
        STRFWDR::_from({
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
    #[doc = "Bits 0:5 - Transmit FIFO Write"]
    #[inline]
    pub fn tfwr(&mut self) -> _TFWRW {
        _TFWRW { w: self }
    }
    #[doc = "Bit 8 - Store And Forward Enable"]
    #[inline]
    pub fn strfwd(&mut self) -> _STRFWDW {
        _STRFWDW { w: self }
    }
}
