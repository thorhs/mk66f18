#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEBUG1_SET {
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
#[doc = "Possible values of the field `ENTAILADJVD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENTAILADJVDR {
    #[doc = "Delay is nominal"]
    _00,
    #[doc = "Delay is +20%"]
    _01,
    #[doc = "Delay is -20%"]
    _10,
    #[doc = "Delay is -40%"]
    _11,
}
impl ENTAILADJVDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENTAILADJVDR::_00 => 0,
            ENTAILADJVDR::_01 => 1,
            ENTAILADJVDR::_10 => 2,
            ENTAILADJVDR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENTAILADJVDR {
        match value {
            0 => ENTAILADJVDR::_00,
            1 => ENTAILADJVDR::_01,
            2 => ENTAILADJVDR::_10,
            3 => ENTAILADJVDR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ENTAILADJVDR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ENTAILADJVDR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ENTAILADJVDR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == ENTAILADJVDR::_11
    }
}
#[doc = "Values that can be written to the field `ENTAILADJVD`"]
pub enum ENTAILADJVDW {
    #[doc = "Delay is nominal"]
    _00,
    #[doc = "Delay is +20%"]
    _01,
    #[doc = "Delay is -20%"]
    _10,
    #[doc = "Delay is -40%"]
    _11,
}
impl ENTAILADJVDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENTAILADJVDW::_00 => 0,
            ENTAILADJVDW::_01 => 1,
            ENTAILADJVDW::_10 => 2,
            ENTAILADJVDW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENTAILADJVDW<'a> {
    w: &'a mut W,
}
impl<'a> _ENTAILADJVDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENTAILADJVDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Delay is nominal"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ENTAILADJVDW::_00)
    }
    #[doc = "Delay is +20%"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ENTAILADJVDW::_01)
    }
    #[doc = "Delay is -20%"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ENTAILADJVDW::_10)
    }
    #[doc = "Delay is -40%"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(ENTAILADJVDW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
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
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch:"]
    #[inline]
    pub fn entailadjvd(&self) -> ENTAILADJVDR {
        ENTAILADJVDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4096 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 13:14 - Delay increment of the rise of squelch:"]
    #[inline]
    pub fn entailadjvd(&mut self) -> _ENTAILADJVDW {
        _ENTAILADJVDW { w: self }
    }
}
