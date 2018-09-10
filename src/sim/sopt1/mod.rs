#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT1 {
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
#[doc = "Possible values of the field `RAMSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMSIZER {
    #[doc = "8 KB"]
    _0001,
    #[doc = "16 KB"]
    _0011,
    #[doc = "24 KB"]
    _0100,
    #[doc = "32 KB"]
    _0101,
    #[doc = "48 KB"]
    _0110,
    #[doc = "64 KB"]
    _0111,
    #[doc = "96 KB"]
    _1000,
    #[doc = "128 KB"]
    _1001,
    #[doc = "256 KB"]
    _1011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAMSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMSIZER::_0001 => 1,
            RAMSIZER::_0011 => 3,
            RAMSIZER::_0100 => 4,
            RAMSIZER::_0101 => 5,
            RAMSIZER::_0110 => 6,
            RAMSIZER::_0111 => 7,
            RAMSIZER::_1000 => 8,
            RAMSIZER::_1001 => 9,
            RAMSIZER::_1011 => 11,
            RAMSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMSIZER {
        match value {
            1 => RAMSIZER::_0001,
            3 => RAMSIZER::_0011,
            4 => RAMSIZER::_0100,
            5 => RAMSIZER::_0101,
            6 => RAMSIZER::_0110,
            7 => RAMSIZER::_0111,
            8 => RAMSIZER::_1000,
            9 => RAMSIZER::_1001,
            11 => RAMSIZER::_1011,
            i => RAMSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == RAMSIZER::_0001
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == RAMSIZER::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == RAMSIZER::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == RAMSIZER::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == RAMSIZER::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == RAMSIZER::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == RAMSIZER::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == RAMSIZER::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == RAMSIZER::_1011
    }
}
#[doc = "Possible values of the field `OSC32KSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC32KSELR {
    #[doc = "System oscillator (OSC32KCLK)"]
    _00,
    #[doc = "RTC 32.768kHz oscillator"]
    _10,
    #[doc = "LPO 1 kHz"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OSC32KSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSC32KSELR::_00 => 0,
            OSC32KSELR::_10 => 2,
            OSC32KSELR::_11 => 3,
            OSC32KSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSC32KSELR {
        match value {
            0 => OSC32KSELR::_00,
            2 => OSC32KSELR::_10,
            3 => OSC32KSELR::_11,
            i => OSC32KSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == OSC32KSELR::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == OSC32KSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == OSC32KSELR::_11
    }
}
#[doc = "Possible values of the field `USBVSTBY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBVSTBYR {
    #[doc = "USB voltage regulator not in standby during VLPR and VLPW modes."]
    _0,
    #[doc = "USB voltage regulator in standby during VLPR and VLPW modes."]
    _1,
}
impl USBVSTBYR {
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
            USBVSTBYR::_0 => false,
            USBVSTBYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBVSTBYR {
        match value {
            false => USBVSTBYR::_0,
            true => USBVSTBYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBVSTBYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBVSTBYR::_1
    }
}
#[doc = "Possible values of the field `USBSSTBY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSSTBYR {
    #[doc = "USB voltage regulator not in standby during Stop, VLPS, LLS and VLLS modes."]
    _0,
    #[doc = "USB voltage regulator in standby during Stop, VLPS, LLS and VLLS modes."]
    _1,
}
impl USBSSTBYR {
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
            USBSSTBYR::_0 => false,
            USBSSTBYR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBSSTBYR {
        match value {
            false => USBSSTBYR::_0,
            true => USBSSTBYR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBSSTBYR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBSSTBYR::_1
    }
}
#[doc = "Possible values of the field `USBREGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBREGENR {
    #[doc = "USB voltage regulator is disabled."]
    _0,
    #[doc = "USB voltage regulator is enabled."]
    _1,
}
impl USBREGENR {
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
            USBREGENR::_0 => false,
            USBREGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USBREGENR {
        match value {
            false => USBREGENR::_0,
            true => USBREGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == USBREGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == USBREGENR::_1
    }
}
#[doc = "Values that can be written to the field `OSC32KSEL`"]
pub enum OSC32KSELW {
    #[doc = "System oscillator (OSC32KCLK)"]
    _00,
    #[doc = "RTC 32.768kHz oscillator"]
    _10,
    #[doc = "LPO 1 kHz"]
    _11,
}
impl OSC32KSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSC32KSELW::_00 => 0,
            OSC32KSELW::_10 => 2,
            OSC32KSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSC32KSELW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC32KSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSC32KSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "System oscillator (OSC32KCLK)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSC32KSELW::_00)
    }
    #[doc = "RTC 32.768kHz oscillator"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSC32KSELW::_10)
    }
    #[doc = "LPO 1 kHz"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(OSC32KSELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBVSTBY`"]
pub enum USBVSTBYW {
    #[doc = "USB voltage regulator not in standby during VLPR and VLPW modes."]
    _0,
    #[doc = "USB voltage regulator in standby during VLPR and VLPW modes."]
    _1,
}
impl USBVSTBYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBVSTBYW::_0 => false,
            USBVSTBYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBVSTBYW<'a> {
    w: &'a mut W,
}
impl<'a> _USBVSTBYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBVSTBYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB voltage regulator not in standby during VLPR and VLPW modes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBVSTBYW::_0)
    }
    #[doc = "USB voltage regulator in standby during VLPR and VLPW modes."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBVSTBYW::_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBSSTBY`"]
pub enum USBSSTBYW {
    #[doc = "USB voltage regulator not in standby during Stop, VLPS, LLS and VLLS modes."]
    _0,
    #[doc = "USB voltage regulator in standby during Stop, VLPS, LLS and VLLS modes."]
    _1,
}
impl USBSSTBYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBSSTBYW::_0 => false,
            USBSSTBYW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBSSTBYW<'a> {
    w: &'a mut W,
}
impl<'a> _USBSSTBYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBSSTBYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB voltage regulator not in standby during Stop, VLPS, LLS and VLLS modes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSSTBYW::_0)
    }
    #[doc = "USB voltage regulator in standby during Stop, VLPS, LLS and VLLS modes."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSSTBYW::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USBREGEN`"]
pub enum USBREGENW {
    #[doc = "USB voltage regulator is disabled."]
    _0,
    #[doc = "USB voltage regulator is enabled."]
    _1,
}
impl USBREGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USBREGENW::_0 => false,
            USBREGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USBREGENW<'a> {
    w: &'a mut W,
}
impl<'a> _USBREGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USBREGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB voltage regulator is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBREGENW::_0)
    }
    #[doc = "USB voltage regulator is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBREGENW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 12:15 - RAM size"]
    #[inline]
    pub fn ramsize(&self) -> RAMSIZER {
        RAMSIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - 32K oscillator clock select"]
    #[inline]
    pub fn osc32ksel(&self) -> OSC32KSELR {
        OSC32KSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - USB voltage regulator in standby mode during VLPR and VLPW modes"]
    #[inline]
    pub fn usbvstby(&self) -> USBVSTBYR {
        USBVSTBYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
    #[inline]
    pub fn usbsstby(&self) -> USBSSTBYR {
        USBSSTBYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - USB voltage regulator enable"]
    #[inline]
    pub fn usbregen(&self) -> USBREGENR {
        USBREGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2147483648 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 18:19 - 32K oscillator clock select"]
    #[inline]
    pub fn osc32ksel(&mut self) -> _OSC32KSELW {
        _OSC32KSELW { w: self }
    }
    #[doc = "Bit 29 - USB voltage regulator in standby mode during VLPR and VLPW modes"]
    #[inline]
    pub fn usbvstby(&mut self) -> _USBVSTBYW {
        _USBVSTBYW { w: self }
    }
    #[doc = "Bit 30 - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
    #[inline]
    pub fn usbsstby(&mut self) -> _USBSSTBYW {
        _USBSSTBYW { w: self }
    }
    #[doc = "Bit 31 - USB voltage regulator enable"]
    #[inline]
    pub fn usbregen(&mut self) -> _USBREGENW {
        _USBREGENW { w: self }
    }
}
