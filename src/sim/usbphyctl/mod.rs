#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBPHYCTL {
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
#[doc = "Possible values of the field `USBVREGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBVREGSELR {
    #[doc = "VREG_IN0 will be selected if both regulator inputs are powered"]
    _0,
    #[doc = "VREG_IN1 will be selected if both regulator inputs are powered"]
    _1,
}
impl USBVREGSELR {
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
            USBVREGSELR::_0 => false,
            USBVREGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBVREGSELR {
        match value {
            false => USBVREGSELR::_0,
            true => USBVREGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBVREGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBVREGSELR::_1
    }
}
#[doc = "Possible values of the field `USBVREGPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBVREGPDR {
    #[doc = "Regulator output pulldown resistor is not enabled"]
    _0,
    #[doc = "Regulator output pulldown resistor is enabled"]
    _1,
}
impl USBVREGPDR {
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
            USBVREGPDR::_0 => false,
            USBVREGPDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBVREGPDR {
        match value {
            false => USBVREGPDR::_0,
            true => USBVREGPDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBVREGPDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBVREGPDR::_1
    }
}
#[doc = "Possible values of the field `USB3VOUTTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB3VOUTTRGR {
    #[doc = "2.733V"]
    _000,
    #[doc = "3.020V"]
    _001,
    #[doc = "3.074V"]
    _010,
    #[doc = "3.130V"]
    _011,
    #[doc = "3.188V"]
    _100,
    #[doc = "3.248V"]
    _101,
    #[doc = "3.310V (default)"]
    _110,
    #[doc = "3.662V (For Freescale use only, not for customer use)"]
    _111,
}
impl USB3VOUTTRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USB3VOUTTRGR::_000 => 0,
            USB3VOUTTRGR::_001 => 1,
            USB3VOUTTRGR::_010 => 2,
            USB3VOUTTRGR::_011 => 3,
            USB3VOUTTRGR::_100 => 4,
            USB3VOUTTRGR::_101 => 5,
            USB3VOUTTRGR::_110 => 6,
            USB3VOUTTRGR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USB3VOUTTRGR {
        match value {
            0 => USB3VOUTTRGR::_000,
            1 => USB3VOUTTRGR::_001,
            2 => USB3VOUTTRGR::_010,
            3 => USB3VOUTTRGR::_011,
            4 => USB3VOUTTRGR::_100,
            5 => USB3VOUTTRGR::_101,
            6 => USB3VOUTTRGR::_110,
            7 => USB3VOUTTRGR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == USB3VOUTTRGR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == USB3VOUTTRGR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == USB3VOUTTRGR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == USB3VOUTTRGR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == USB3VOUTTRGR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == USB3VOUTTRGR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == USB3VOUTTRGR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == USB3VOUTTRGR::_111
    }
}
#[doc = "Possible values of the field `USBDISILIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBDISILIMR {
    #[doc = "The current limiter for the USB Voltage Regulator is enabled"]
    _0,
    #[doc = "The current limiter for the USB Voltage Regulator is disabled"]
    _1,
}
impl USBDISILIMR {
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
            USBDISILIMR::_0 => false,
            USBDISILIMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBDISILIMR {
        match value {
            false => USBDISILIMR::_0,
            true => USBDISILIMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBDISILIMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBDISILIMR::_1
    }
}
#[doc = "Values that can be written to the field `USBVREGSEL`"]
pub enum USBVREGSELW {
    #[doc = "VREG_IN0 will be selected if both regulator inputs are powered"]
    _0,
    #[doc = "VREG_IN1 will be selected if both regulator inputs are powered"]
    _1,
}
impl USBVREGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBVREGSELW::_0 => false,
            USBVREGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBVREGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _USBVREGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBVREGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "VREG_IN0 will be selected if both regulator inputs are powered"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBVREGSELW::_0)
    }
    #[doc = "VREG_IN1 will be selected if both regulator inputs are powered"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBVREGSELW::_1)
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
#[doc = "Values that can be written to the field `USBVREGPD`"]
pub enum USBVREGPDW {
    #[doc = "Regulator output pulldown resistor is not enabled"]
    _0,
    #[doc = "Regulator output pulldown resistor is enabled"]
    _1,
}
impl USBVREGPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBVREGPDW::_0 => false,
            USBVREGPDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBVREGPDW<'a> {
    w: &'a mut W,
}
impl<'a> _USBVREGPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBVREGPDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Regulator output pulldown resistor is not enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBVREGPDW::_0)
    }
    #[doc = "Regulator output pulldown resistor is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBVREGPDW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USB3VOUTTRG`"]
pub enum USB3VOUTTRGW {
    #[doc = "2.733V"]
    _000,
    #[doc = "3.020V"]
    _001,
    #[doc = "3.074V"]
    _010,
    #[doc = "3.130V"]
    _011,
    #[doc = "3.188V"]
    _100,
    #[doc = "3.248V"]
    _101,
    #[doc = "3.310V (default)"]
    _110,
    #[doc = "3.662V (For Freescale use only, not for customer use)"]
    _111,
}
impl USB3VOUTTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB3VOUTTRGW::_000 => 0,
            USB3VOUTTRGW::_001 => 1,
            USB3VOUTTRGW::_010 => 2,
            USB3VOUTTRGW::_011 => 3,
            USB3VOUTTRGW::_100 => 4,
            USB3VOUTTRGW::_101 => 5,
            USB3VOUTTRGW::_110 => 6,
            USB3VOUTTRGW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USB3VOUTTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _USB3VOUTTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB3VOUTTRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "2.733V"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(USB3VOUTTRGW::_000)
    }
    #[doc = "3.020V"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(USB3VOUTTRGW::_001)
    }
    #[doc = "3.074V"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(USB3VOUTTRGW::_010)
    }
    #[doc = "3.130V"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(USB3VOUTTRGW::_011)
    }
    #[doc = "3.188V"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(USB3VOUTTRGW::_100)
    }
    #[doc = "3.248V"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(USB3VOUTTRGW::_101)
    }
    #[doc = "3.310V (default)"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(USB3VOUTTRGW::_110)
    }
    #[doc = "3.662V (For Freescale use only, not for customer use)"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(USB3VOUTTRGW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBDISILIM`"]
pub enum USBDISILIMW {
    #[doc = "The current limiter for the USB Voltage Regulator is enabled"]
    _0,
    #[doc = "The current limiter for the USB Voltage Regulator is disabled"]
    _1,
}
impl USBDISILIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBDISILIMW::_0 => false,
            USBDISILIMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBDISILIMW<'a> {
    w: &'a mut W,
}
impl<'a> _USBDISILIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBDISILIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The current limiter for the USB Voltage Regulator is enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBDISILIMW::_0)
    }
    #[doc = "The current limiter for the USB Voltage Regulator is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBDISILIMW::_1)
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 8 - Selects the default input voltage source to the USB Regulator in case both VREG_IN0 and VREG_IN1 are powered"]
    #[inline]
    pub fn usbvregsel(&self) -> USBVREGSELR {
        USBVREGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enables the pulldown on the output of the USB Regulator."]
    #[inline]
    pub fn usbvregpd(&self) -> USBVREGPDR {
        USBVREGPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 20:22 - USB 3.3V Output Target"]
    #[inline]
    pub fn usb3vouttrg(&self) -> USB3VOUTTRGR {
        USB3VOUTTRGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - USB Disable Inrush Current Limit"]
    #[inline]
    pub fn usbdisilim(&self) -> USBDISILIMR {
        USBDISILIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 6291456 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 8 - Selects the default input voltage source to the USB Regulator in case both VREG_IN0 and VREG_IN1 are powered"]
    #[inline]
    pub fn usbvregsel(&mut self) -> _USBVREGSELW {
        _USBVREGSELW { w: self }
    }
    #[doc = "Bit 9 - Enables the pulldown on the output of the USB Regulator."]
    #[inline]
    pub fn usbvregpd(&mut self) -> _USBVREGPDW {
        _USBVREGPDW { w: self }
    }
    #[doc = "Bits 20:22 - USB 3.3V Output Target"]
    #[inline]
    pub fn usb3vouttrg(&mut self) -> _USB3VOUTTRGW {
        _USB3VOUTTRGW { w: self }
    }
    #[doc = "Bit 23 - USB Disable Inrush Current Limit"]
    #[inline]
    pub fn usbdisilim(&mut self) -> _USBDISILIMW {
        _USBDISILIMW { w: self }
    }
}
