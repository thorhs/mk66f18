#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBMODE {
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
#[doc = "Possible values of the field `CM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMR {
    #[doc = "Idle (default for the USBHS module)"]
    _00,
    #[doc = "Device controller"]
    _10,
    #[doc = "Host controller"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMR::_00 => 0,
            CMR::_10 => 2,
            CMR::_11 => 3,
            CMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMR {
        match value {
            0 => CMR::_00,
            2 => CMR::_10,
            3 => CMR::_11,
            i => CMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == CMR::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == CMR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == CMR::_11
    }
}
#[doc = "Possible values of the field `ES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESR {
    #[doc = "Little endian. First byte referenced in least significant byte of 32-bit word."]
    _0,
    #[doc = "Big endian. First byte referenced in most significant byte of 32-bit word."]
    _1,
}
impl ESR {
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
            ESR::_0 => false,
            ESR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESR {
        match value {
            false => ESR::_0,
            true => ESR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ESR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ESR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SLOMR {
    bits: bool,
}
impl SLOMR {
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
#[doc = "Possible values of the field `SDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDISR {
    #[doc = "Inactive"]
    _0,
    #[doc = "Active"]
    _1,
}
impl SDISR {
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
            SDISR::_0 => false,
            SDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDISR {
        match value {
            false => SDISR::_0,
            true => SDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SDISR::_1
    }
}
#[doc = "Possible values of the field `TXHSD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXHSDR {
    #[doc = "10"]
    _000,
    #[doc = "11"]
    _001,
    #[doc = "12"]
    _010,
    #[doc = "13"]
    _011,
    #[doc = "14"]
    _100,
    #[doc = "15"]
    _101,
    #[doc = "16"]
    _110,
    #[doc = "17"]
    _111,
}
impl TXHSDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXHSDR::_000 => 0,
            TXHSDR::_001 => 1,
            TXHSDR::_010 => 2,
            TXHSDR::_011 => 3,
            TXHSDR::_100 => 4,
            TXHSDR::_101 => 5,
            TXHSDR::_110 => 6,
            TXHSDR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXHSDR {
        match value {
            0 => TXHSDR::_000,
            1 => TXHSDR::_001,
            2 => TXHSDR::_010,
            3 => TXHSDR::_011,
            4 => TXHSDR::_100,
            5 => TXHSDR::_101,
            6 => TXHSDR::_110,
            7 => TXHSDR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == TXHSDR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == TXHSDR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == TXHSDR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == TXHSDR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TXHSDR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == TXHSDR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TXHSDR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == TXHSDR::_111
    }
}
#[doc = "Values that can be written to the field `CM`"]
pub enum CMW {
    #[doc = "Idle (default for the USBHS module)"]
    _00,
    #[doc = "Device controller"]
    _10,
    #[doc = "Host controller"]
    _11,
}
impl CMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMW::_00 => 0,
            CMW::_10 => 2,
            CMW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMW<'a> {
    w: &'a mut W,
}
impl<'a> _CMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Idle (default for the USBHS module)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMW::_00)
    }
    #[doc = "Device controller"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMW::_10)
    }
    #[doc = "Host controller"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ES`"]
pub enum ESW {
    #[doc = "Little endian. First byte referenced in least significant byte of 32-bit word."]
    _0,
    #[doc = "Big endian. First byte referenced in most significant byte of 32-bit word."]
    _1,
}
impl ESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESW::_0 => false,
            ESW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESW<'a> {
    w: &'a mut W,
}
impl<'a> _ESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Little endian. First byte referenced in least significant byte of 32-bit word."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESW::_0)
    }
    #[doc = "Big endian. First byte referenced in most significant byte of 32-bit word."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLOMW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOMW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDIS`"]
pub enum SDISW {
    #[doc = "Inactive"]
    _0,
    #[doc = "Active"]
    _1,
}
impl SDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDISW::_0 => false,
            SDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inactive"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDISW::_0)
    }
    #[doc = "Active"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDISW::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXHSD`"]
pub enum TXHSDW {
    #[doc = "10"]
    _000,
    #[doc = "11"]
    _001,
    #[doc = "12"]
    _010,
    #[doc = "13"]
    _011,
    #[doc = "14"]
    _100,
    #[doc = "15"]
    _101,
    #[doc = "16"]
    _110,
    #[doc = "17"]
    _111,
}
impl TXHSDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXHSDW::_000 => 0,
            TXHSDW::_001 => 1,
            TXHSDW::_010 => 2,
            TXHSDW::_011 => 3,
            TXHSDW::_100 => 4,
            TXHSDW::_101 => 5,
            TXHSDW::_110 => 6,
            TXHSDW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXHSDW<'a> {
    w: &'a mut W,
}
impl<'a> _TXHSDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXHSDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "10"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(TXHSDW::_000)
    }
    #[doc = "11"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(TXHSDW::_001)
    }
    #[doc = "12"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(TXHSDW::_010)
    }
    #[doc = "13"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(TXHSDW::_011)
    }
    #[doc = "14"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(TXHSDW::_100)
    }
    #[doc = "15"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(TXHSDW::_101)
    }
    #[doc = "16"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(TXHSDW::_110)
    }
    #[doc = "17"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(TXHSDW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Controller Mode"]
    #[inline]
    pub fn cm(&self) -> CMR {
        CMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Endian Select"]
    #[inline]
    pub fn es(&self) -> ESR {
        ESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Setup Lock-Out Mode"]
    #[inline]
    pub fn slom(&self) -> SLOMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLOMR { bits }
    }
    #[doc = "Bit 4 - Stream DISable"]
    #[inline]
    pub fn sdis(&self) -> SDISR {
        SDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:14 - Tx to Tx HS Delay"]
    #[inline]
    pub fn txhsd(&self) -> TXHSDR {
        TXHSDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 20480 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Controller Mode"]
    #[inline]
    pub fn cm(&mut self) -> _CMW {
        _CMW { w: self }
    }
    #[doc = "Bit 2 - Endian Select"]
    #[inline]
    pub fn es(&mut self) -> _ESW {
        _ESW { w: self }
    }
    #[doc = "Bit 3 - Setup Lock-Out Mode"]
    #[inline]
    pub fn slom(&mut self) -> _SLOMW {
        _SLOMW { w: self }
    }
    #[doc = "Bit 4 - Stream DISable"]
    #[inline]
    pub fn sdis(&mut self) -> _SDISW {
        _SDISW { w: self }
    }
    #[doc = "Bits 12:14 - Tx to Tx HS Delay"]
    #[inline]
    pub fn txhsd(&mut self) -> _TXHSDW {
        _TXHSDW { w: self }
    }
}
