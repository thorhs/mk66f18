#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MGPCR {
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
#[doc = "Possible values of the field `AULB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AULBR {
    #[doc = "No arbitration is allowed during an undefined length burst"]
    _000,
    #[doc = "Arbitration is allowed at any time during an undefined length burst"]
    _001,
    #[doc = "Arbitration is allowed after four beats of an undefined length burst"]
    _010,
    #[doc = "Arbitration is allowed after eight beats of an undefined length burst"]
    _011,
    #[doc = "Arbitration is allowed after 16 beats of an undefined length burst"]
    _100,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AULBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AULBR::_000 => 0,
            AULBR::_001 => 1,
            AULBR::_010 => 2,
            AULBR::_011 => 3,
            AULBR::_100 => 4,
            AULBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AULBR {
        match value {
            0 => AULBR::_000,
            1 => AULBR::_001,
            2 => AULBR::_010,
            3 => AULBR::_011,
            4 => AULBR::_100,
            i => AULBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == AULBR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == AULBR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == AULBR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == AULBR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == AULBR::_100
    }
}
#[doc = "Values that can be written to the field `AULB`"]
pub enum AULBW {
    #[doc = "No arbitration is allowed during an undefined length burst"]
    _000,
    #[doc = "Arbitration is allowed at any time during an undefined length burst"]
    _001,
    #[doc = "Arbitration is allowed after four beats of an undefined length burst"]
    _010,
    #[doc = "Arbitration is allowed after eight beats of an undefined length burst"]
    _011,
    #[doc = "Arbitration is allowed after 16 beats of an undefined length burst"]
    _100,
}
impl AULBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AULBW::_000 => 0,
            AULBW::_001 => 1,
            AULBW::_010 => 2,
            AULBW::_011 => 3,
            AULBW::_100 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AULBW<'a> {
    w: &'a mut W,
}
impl<'a> _AULBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AULBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No arbitration is allowed during an undefined length burst"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(AULBW::_000)
    }
    #[doc = "Arbitration is allowed at any time during an undefined length burst"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(AULBW::_001)
    }
    #[doc = "Arbitration is allowed after four beats of an undefined length burst"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(AULBW::_010)
    }
    #[doc = "Arbitration is allowed after eight beats of an undefined length burst"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(AULBW::_011)
    }
    #[doc = "Arbitration is allowed after 16 beats of an undefined length burst"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(AULBW::_100)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - Arbitrates On Undefined Length Bursts"]
    #[inline]
    pub fn aulb(&self) -> AULBR {
        AULBR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:2 - Arbitrates On Undefined Length Bursts"]
    #[inline]
    pub fn aulb(&mut self) -> _AULBW {
        _AULBW { w: self }
    }
}
