#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TOKEN {
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
pub struct TOKENENDPTR {
    bits: u8,
}
impl TOKENENDPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TOKENPID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOKENPIDR {
    #[doc = "OUT Token. USB Module performs an OUT (TX) transaction."]
    _0001,
    #[doc = "IN Token. USB Module performs an In (RX) transaction."]
    _1001,
    #[doc = "SETUP Token. USB Module performs a SETUP (TX) transaction"]
    _1101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TOKENPIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TOKENPIDR::_0001 => 1,
            TOKENPIDR::_1001 => 9,
            TOKENPIDR::_1101 => 13,
            TOKENPIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TOKENPIDR {
        match value {
            1 => TOKENPIDR::_0001,
            9 => TOKENPIDR::_1001,
            13 => TOKENPIDR::_1101,
            i => TOKENPIDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == TOKENPIDR::_0001
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == TOKENPIDR::_1001
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == TOKENPIDR::_1101
    }
}
#[doc = r" Proxy"]
pub struct _TOKENENDPTW<'a> {
    w: &'a mut W,
}
impl<'a> _TOKENENDPTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOKENPID`"]
pub enum TOKENPIDW {
    #[doc = "OUT Token. USB Module performs an OUT (TX) transaction."]
    _0001,
    #[doc = "IN Token. USB Module performs an In (RX) transaction."]
    _1001,
    #[doc = "SETUP Token. USB Module performs a SETUP (TX) transaction"]
    _1101,
}
impl TOKENPIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TOKENPIDW::_0001 => 1,
            TOKENPIDW::_1001 => 9,
            TOKENPIDW::_1101 => 13,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOKENPIDW<'a> {
    w: &'a mut W,
}
impl<'a> _TOKENPIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOKENPIDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "OUT Token. USB Module performs an OUT (TX) transaction."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TOKENPIDW::_0001)
    }
    #[doc = "IN Token. USB Module performs an In (RX) transaction."]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(TOKENPIDW::_1001)
    }
    #[doc = "SETUP Token. USB Module performs a SETUP (TX) transaction"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(TOKENPIDW::_1101)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:3 - Holds the Endpoint address for the token command"]
    #[inline]
    pub fn tokenendpt(&self) -> TOKENENDPTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        TOKENENDPTR { bits }
    }
    #[doc = "Bits 4:7 - Contains the token type executed by the USB module."]
    #[inline]
    pub fn tokenpid(&self) -> TOKENPIDR {
        TOKENPIDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    #[doc = "Bits 0:3 - Holds the Endpoint address for the token command"]
    #[inline]
    pub fn tokenendpt(&mut self) -> _TOKENENDPTW {
        _TOKENENDPTW { w: self }
    }
    #[doc = "Bits 4:7 - Contains the token type executed by the USB module."]
    #[inline]
    pub fn tokenpid(&mut self) -> _TOKENPIDW {
        _TOKENPIDW { w: self }
    }
}
