#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TCD_ATTR {
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
pub struct DSIZER {
    bits: u8,
}
impl DSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMODR {
    bits: u8,
}
impl DMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSIZER {
    #[doc = "8-bit"]
    _000,
    #[doc = "16-bit"]
    _001,
    #[doc = "32-bit"]
    _010,
    #[doc = "16-byte burst"]
    _100,
    #[doc = "32-byte burst"]
    _101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSIZER::_000 => 0,
            SSIZER::_001 => 1,
            SSIZER::_010 => 2,
            SSIZER::_100 => 4,
            SSIZER::_101 => 5,
            SSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSIZER {
        match value {
            0 => SSIZER::_000,
            1 => SSIZER::_001,
            2 => SSIZER::_010,
            4 => SSIZER::_100,
            5 => SSIZER::_101,
            i => SSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == SSIZER::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == SSIZER::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == SSIZER::_010
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == SSIZER::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == SSIZER::_101
    }
}
#[doc = "Possible values of the field `SMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMODR {
    #[doc = "Source address modulo feature is disabled"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMODR::_0 => 0,
            SMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMODR {
        match value {
            0 => SMODR::_0,
            i => SMODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SMODR::_0
    }
}
#[doc = r" Proxy"]
pub struct _DSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMODW<'a> {
    w: &'a mut W,
}
impl<'a> _DMODW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SSIZE`"]
pub enum SSIZEW {
    #[doc = "8-bit"]
    _000,
    #[doc = "16-bit"]
    _001,
    #[doc = "32-bit"]
    _010,
    #[doc = "16-byte burst"]
    _100,
    #[doc = "32-byte burst"]
    _101,
}
impl SSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSIZEW::_000 => 0,
            SSIZEW::_001 => 1,
            SSIZEW::_010 => 2,
            SSIZEW::_100 => 4,
            SSIZEW::_101 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8-bit"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(SSIZEW::_000)
    }
    #[doc = "16-bit"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(SSIZEW::_001)
    }
    #[doc = "32-bit"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(SSIZEW::_010)
    }
    #[doc = "16-byte burst"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(SSIZEW::_100)
    }
    #[doc = "32-byte burst"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(SSIZEW::_101)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMOD`"]
pub enum SMODW {
    #[doc = "Source address modulo feature is disabled"]
    _0,
}
impl SMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMODW::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMODW<'a> {
    w: &'a mut W,
}
impl<'a> _SMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Source address modulo feature is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMODW::_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 0:2 - Destination data transfer size"]
    #[inline]
    pub fn dsize(&self) -> DSIZER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        DSIZER { bits }
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline]
    pub fn dmod(&self) -> DMODR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        DMODR { bits }
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline]
    pub fn ssize(&self) -> SSIZER {
        SSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 11:15 - Source Address Modulo"]
    #[inline]
    pub fn smod(&self) -> SMODR {
        SMODR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    #[doc = "Bits 0:2 - Destination data transfer size"]
    #[inline]
    pub fn dsize(&mut self) -> _DSIZEW {
        _DSIZEW { w: self }
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline]
    pub fn dmod(&mut self) -> _DMODW {
        _DMODW { w: self }
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline]
    pub fn ssize(&mut self) -> _SSIZEW {
        _SSIZEW { w: self }
    }
    #[doc = "Bits 11:15 - Source Address Modulo"]
    #[inline]
    pub fn smod(&mut self) -> _SMODW {
        _SMODW { w: self }
    }
}
