#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_TOG {
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
#[doc = "Possible values of the field `ENVADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENVADJR {
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    _000,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    _001,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    _010,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    _011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENVADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENVADJR::_000 => 0,
            ENVADJR::_001 => 1,
            ENVADJR::_010 => 2,
            ENVADJR::_011 => 3,
            ENVADJR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENVADJR {
        match value {
            0 => ENVADJR::_000,
            1 => ENVADJR::_001,
            2 => ENVADJR::_010,
            3 => ENVADJR::_011,
            i => ENVADJR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == ENVADJR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == ENVADJR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == ENVADJR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == ENVADJR::_011
    }
}
#[doc = "Possible values of the field `DISCONADJ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCONADJR {
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    _000,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    _001,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    _010,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    _011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DISCONADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DISCONADJR::_000 => 0,
            DISCONADJR::_001 => 1,
            DISCONADJR::_010 => 2,
            DISCONADJR::_011 => 3,
            DISCONADJR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DISCONADJR {
        match value {
            0 => DISCONADJR::_000,
            1 => DISCONADJR::_001,
            2 => DISCONADJR::_010,
            3 => DISCONADJR::_011,
            i => DISCONADJR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == DISCONADJR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == DISCONADJR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == DISCONADJR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == DISCONADJR::_011
    }
}
#[doc = "Possible values of the field `RXDBYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDBYPASSR {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    _1,
}
impl RXDBYPASSR {
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
            RXDBYPASSR::_0 => false,
            RXDBYPASSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDBYPASSR {
        match value {
            false => RXDBYPASSR::_0,
            true => RXDBYPASSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXDBYPASSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXDBYPASSR::_1
    }
}
#[doc = "Values that can be written to the field `ENVADJ`"]
pub enum ENVADJW {
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    _000,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    _001,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    _010,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    _011,
}
impl ENVADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENVADJW::_000 => 0,
            ENVADJW::_001 => 1,
            ENVADJW::_010 => 2,
            ENVADJW::_011 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENVADJW<'a> {
    w: &'a mut W,
}
impl<'a> _ENVADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENVADJW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(ENVADJW::_000)
    }
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(ENVADJW::_001)
    }
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(ENVADJW::_010)
    }
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(ENVADJW::_011)
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
#[doc = "Values that can be written to the field `DISCONADJ`"]
pub enum DISCONADJW {
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    _000,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    _001,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    _010,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    _011,
}
impl DISCONADJW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DISCONADJW::_000 => 0,
            DISCONADJW::_001 => 1,
            DISCONADJW::_010 => 2,
            DISCONADJW::_011 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISCONADJW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCONADJW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISCONADJW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(DISCONADJW::_000)
    }
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(DISCONADJW::_001)
    }
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(DISCONADJW::_010)
    }
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(DISCONADJW::_011)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXDBYPASS`"]
pub enum RXDBYPASSW {
    #[doc = "Normal operation."]
    _0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    _1,
}
impl RXDBYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDBYPASSW::_0 => false,
            RXDBYPASSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDBYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDBYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXDBYPASSW::_0)
    }
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXDBYPASSW::_1)
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
        const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline]
    pub fn envadj(&self) -> ENVADJR {
        ENVADJR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline]
    pub fn disconadj(&self) -> DISCONADJR {
        DISCONADJR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline]
    pub fn rxdbypass(&self) -> RXDBYPASSR {
        RXDBYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:2 - The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline]
    pub fn envadj(&mut self) -> _ENVADJW {
        _ENVADJW { w: self }
    }
    #[doc = "Bits 4:6 - The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline]
    pub fn disconadj(&mut self) -> _DISCONADJW {
        _DISCONADJW { w: self }
    }
    #[doc = "Bit 22 - This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline]
    pub fn rxdbypass(&mut self) -> _RXDBYPASSW {
        _RXDBYPASSW { w: self }
    }
}
