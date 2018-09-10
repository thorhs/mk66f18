#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCR2 {
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
pub struct DIVR {
    bits: u8,
}
impl DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCDR {
    #[doc = "Bit clock is generated externally in Slave mode."]
    _0,
    #[doc = "Bit clock is generated internally in Master mode."]
    _1,
}
impl BCDR {
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
            BCDR::_0 => false,
            BCDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCDR {
        match value {
            false => BCDR::_0,
            true => BCDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BCDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BCDR::_1
    }
}
#[doc = "Possible values of the field `BCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCPR {
    #[doc = "Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    _0,
    #[doc = "Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    _1,
}
impl BCPR {
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
            BCPR::_0 => false,
            BCPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCPR {
        match value {
            false => BCPR::_0,
            true => BCPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BCPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BCPR::_1
    }
}
#[doc = "Possible values of the field `MSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSELR {
    #[doc = "Bus Clock selected."]
    _00,
    #[doc = "Master Clock (MCLK) 1 option selected."]
    _01,
    #[doc = "Master Clock (MCLK) 2 option selected."]
    _10,
    #[doc = "Master Clock (MCLK) 3 option selected."]
    _11,
}
impl MSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSELR::_00 => 0,
            MSELR::_01 => 1,
            MSELR::_10 => 2,
            MSELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSELR {
        match value {
            0 => MSELR::_00,
            1 => MSELR::_01,
            2 => MSELR::_10,
            3 => MSELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == MSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == MSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == MSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == MSELR::_11
    }
}
#[doc = "Possible values of the field `BCI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCIR {
    #[doc = "No effect."]
    _0,
    #[doc = "Internal logic is clocked as if bit clock was externally generated."]
    _1,
}
impl BCIR {
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
            BCIR::_0 => false,
            BCIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCIR {
        match value {
            false => BCIR::_0,
            true => BCIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BCIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BCIR::_1
    }
}
#[doc = "Possible values of the field `BCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCSR {
    #[doc = "Use the normal bit clock source."]
    _0,
    #[doc = "Swap the bit clock source."]
    _1,
}
impl BCSR {
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
            BCSR::_0 => false,
            BCSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCSR {
        match value {
            false => BCSR::_0,
            true => BCSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BCSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BCSR::_1
    }
}
#[doc = "Possible values of the field `SYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCR {
    #[doc = "Asynchronous mode."]
    _00,
    #[doc = "Synchronous with transmitter."]
    _01,
    #[doc = "Synchronous with another SAI receiver."]
    _10,
    #[doc = "Synchronous with another SAI transmitter."]
    _11,
}
impl SYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCR::_00 => 0,
            SYNCR::_01 => 1,
            SYNCR::_10 => 2,
            SYNCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCR {
        match value {
            0 => SYNCR::_00,
            1 => SYNCR::_01,
            2 => SYNCR::_10,
            3 => SYNCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == SYNCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == SYNCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SYNCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SYNCR::_11
    }
}
#[doc = r" Proxy"]
pub struct _DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCD`"]
pub enum BCDW {
    #[doc = "Bit clock is generated externally in Slave mode."]
    _0,
    #[doc = "Bit clock is generated internally in Master mode."]
    _1,
}
impl BCDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCDW::_0 => false,
            BCDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCDW<'a> {
    w: &'a mut W,
}
impl<'a> _BCDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bit clock is generated externally in Slave mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCDW::_0)
    }
    #[doc = "Bit clock is generated internally in Master mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCDW::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCP`"]
pub enum BCPW {
    #[doc = "Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    _0,
    #[doc = "Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    _1,
}
impl BCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCPW::_0 => false,
            BCPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCPW<'a> {
    w: &'a mut W,
}
impl<'a> _BCPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCPW::_0)
    }
    #[doc = "Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCPW::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSEL`"]
pub enum MSELW {
    #[doc = "Bus Clock selected."]
    _00,
    #[doc = "Master Clock (MCLK) 1 option selected."]
    _01,
    #[doc = "Master Clock (MCLK) 2 option selected."]
    _10,
    #[doc = "Master Clock (MCLK) 3 option selected."]
    _11,
}
impl MSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSELW::_00 => 0,
            MSELW::_01 => 1,
            MSELW::_10 => 2,
            MSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bus Clock selected."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(MSELW::_00)
    }
    #[doc = "Master Clock (MCLK) 1 option selected."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(MSELW::_01)
    }
    #[doc = "Master Clock (MCLK) 2 option selected."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(MSELW::_10)
    }
    #[doc = "Master Clock (MCLK) 3 option selected."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(MSELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCI`"]
pub enum BCIW {
    #[doc = "No effect."]
    _0,
    #[doc = "Internal logic is clocked as if bit clock was externally generated."]
    _1,
}
impl BCIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCIW::_0 => false,
            BCIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCIW<'a> {
    w: &'a mut W,
}
impl<'a> _BCIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCIW::_0)
    }
    #[doc = "Internal logic is clocked as if bit clock was externally generated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCIW::_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCS`"]
pub enum BCSW {
    #[doc = "Use the normal bit clock source."]
    _0,
    #[doc = "Swap the bit clock source."]
    _1,
}
impl BCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCSW::_0 => false,
            BCSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCSW<'a> {
    w: &'a mut W,
}
impl<'a> _BCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use the normal bit clock source."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCSW::_0)
    }
    #[doc = "Swap the bit clock source."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCSW::_1)
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
#[doc = "Values that can be written to the field `SYNC`"]
pub enum SYNCW {
    #[doc = "Asynchronous mode."]
    _00,
    #[doc = "Synchronous with transmitter."]
    _01,
    #[doc = "Synchronous with another SAI receiver."]
    _10,
    #[doc = "Synchronous with another SAI transmitter."]
    _11,
}
impl SYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCW::_00 => 0,
            SYNCW::_01 => 1,
            SYNCW::_10 => 2,
            SYNCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Asynchronous mode."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(SYNCW::_00)
    }
    #[doc = "Synchronous with transmitter."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(SYNCW::_01)
    }
    #[doc = "Synchronous with another SAI receiver."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SYNCW::_10)
    }
    #[doc = "Synchronous with another SAI transmitter."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(SYNCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:7 - Bit Clock Divide"]
    #[inline]
    pub fn div(&self) -> DIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIVR { bits }
    }
    #[doc = "Bit 24 - Bit Clock Direction"]
    #[inline]
    pub fn bcd(&self) -> BCDR {
        BCDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Bit Clock Polarity"]
    #[inline]
    pub fn bcp(&self) -> BCPR {
        BCPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 26:27 - MCLK Select"]
    #[inline]
    pub fn msel(&self) -> MSELR {
        MSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Bit Clock Input"]
    #[inline]
    pub fn bci(&self) -> BCIR {
        BCIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Bit Clock Swap"]
    #[inline]
    pub fn bcs(&self) -> BCSR {
        BCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 30:31 - Synchronous Mode"]
    #[inline]
    pub fn sync(&self) -> SYNCR {
        SYNCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:7 - Bit Clock Divide"]
    #[inline]
    pub fn div(&mut self) -> _DIVW {
        _DIVW { w: self }
    }
    #[doc = "Bit 24 - Bit Clock Direction"]
    #[inline]
    pub fn bcd(&mut self) -> _BCDW {
        _BCDW { w: self }
    }
    #[doc = "Bit 25 - Bit Clock Polarity"]
    #[inline]
    pub fn bcp(&mut self) -> _BCPW {
        _BCPW { w: self }
    }
    #[doc = "Bits 26:27 - MCLK Select"]
    #[inline]
    pub fn msel(&mut self) -> _MSELW {
        _MSELW { w: self }
    }
    #[doc = "Bit 28 - Bit Clock Input"]
    #[inline]
    pub fn bci(&mut self) -> _BCIW {
        _BCIW { w: self }
    }
    #[doc = "Bit 29 - Bit Clock Swap"]
    #[inline]
    pub fn bcs(&mut self) -> _BCSW {
        _BCSW { w: self }
    }
    #[doc = "Bits 30:31 - Synchronous Mode"]
    #[inline]
    pub fn sync(&mut self) -> _SYNCW {
        _SYNCW { w: self }
    }
}
