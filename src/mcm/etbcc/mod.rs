#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ETBCC {
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
#[doc = "Possible values of the field `CNTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTENR {
    #[doc = "ETB counter disabled"]
    _0,
    #[doc = "ETB counter enabled"]
    _1,
}
impl CNTENR {
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
            CNTENR::_0 => false,
            CNTENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNTENR {
        match value {
            false => CNTENR::_0,
            true => CNTENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CNTENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CNTENR::_1
    }
}
#[doc = "Possible values of the field `RSPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSPTR {
    #[doc = "No response when the ETB count expires"]
    _00,
    #[doc = "Generate a normal interrupt when the ETB count expires"]
    _01,
    #[doc = "Generate an NMI when the ETB count expires"]
    _10,
    #[doc = "Generate a debug halt when the ETB count expires"]
    _11,
}
impl RSPTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSPTR::_00 => 0,
            RSPTR::_01 => 1,
            RSPTR::_10 => 2,
            RSPTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSPTR {
        match value {
            0 => RSPTR::_00,
            1 => RSPTR::_01,
            2 => RSPTR::_10,
            3 => RSPTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RSPTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RSPTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RSPTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RSPTR::_11
    }
}
#[doc = "Possible values of the field `RLRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLRQR {
    #[doc = "No effect"]
    _0,
    #[doc = "Clears pending debug halt, NMI, or IRQ interrupt requests"]
    _1,
}
impl RLRQR {
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
            RLRQR::_0 => false,
            RLRQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RLRQR {
        match value {
            false => RLRQR::_0,
            true => RLRQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RLRQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RLRQR::_1
    }
}
#[doc = "Possible values of the field `ETDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETDISR {
    #[doc = "ETM-to-TPIU trace path enabled"]
    _0,
    #[doc = "ETM-to-TPIU trace path disabled"]
    _1,
}
impl ETDISR {
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
            ETDISR::_0 => false,
            ETDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ETDISR {
        match value {
            false => ETDISR::_0,
            true => ETDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ETDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ETDISR::_1
    }
}
#[doc = "Possible values of the field `ITDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITDISR {
    #[doc = "ITM-to-TPIU trace path enabled"]
    _0,
    #[doc = "ITM-to-TPIU trace path disabled"]
    _1,
}
impl ITDISR {
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
            ITDISR::_0 => false,
            ITDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITDISR {
        match value {
            false => ITDISR::_0,
            true => ITDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ITDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ITDISR::_1
    }
}
#[doc = "Values that can be written to the field `CNTEN`"]
pub enum CNTENW {
    #[doc = "ETB counter disabled"]
    _0,
    #[doc = "ETB counter enabled"]
    _1,
}
impl CNTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNTENW::_0 => false,
            CNTENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNTENW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ETB counter disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTENW::_0)
    }
    #[doc = "ETB counter enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTENW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSPT`"]
pub enum RSPTW {
    #[doc = "No response when the ETB count expires"]
    _00,
    #[doc = "Generate a normal interrupt when the ETB count expires"]
    _01,
    #[doc = "Generate an NMI when the ETB count expires"]
    _10,
    #[doc = "Generate a debug halt when the ETB count expires"]
    _11,
}
impl RSPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSPTW::_00 => 0,
            RSPTW::_01 => 1,
            RSPTW::_10 => 2,
            RSPTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSPTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSPTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No response when the ETB count expires"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSPTW::_00)
    }
    #[doc = "Generate a normal interrupt when the ETB count expires"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSPTW::_01)
    }
    #[doc = "Generate an NMI when the ETB count expires"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSPTW::_10)
    }
    #[doc = "Generate a debug halt when the ETB count expires"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RSPTW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RLRQ`"]
pub enum RLRQW {
    #[doc = "No effect"]
    _0,
    #[doc = "Clears pending debug halt, NMI, or IRQ interrupt requests"]
    _1,
}
impl RLRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RLRQW::_0 => false,
            RLRQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RLRQW<'a> {
    w: &'a mut W,
}
impl<'a> _RLRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RLRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RLRQW::_0)
    }
    #[doc = "Clears pending debug halt, NMI, or IRQ interrupt requests"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RLRQW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETDIS`"]
pub enum ETDISW {
    #[doc = "ETM-to-TPIU trace path enabled"]
    _0,
    #[doc = "ETM-to-TPIU trace path disabled"]
    _1,
}
impl ETDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ETDISW::_0 => false,
            ETDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETDISW<'a> {
    w: &'a mut W,
}
impl<'a> _ETDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ETM-to-TPIU trace path enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETDISW::_0)
    }
    #[doc = "ETM-to-TPIU trace path disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETDISW::_1)
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
#[doc = "Values that can be written to the field `ITDIS`"]
pub enum ITDISW {
    #[doc = "ITM-to-TPIU trace path enabled"]
    _0,
    #[doc = "ITM-to-TPIU trace path disabled"]
    _1,
}
impl ITDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITDISW::_0 => false,
            ITDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITDISW<'a> {
    w: &'a mut W,
}
impl<'a> _ITDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ITM-to-TPIU trace path enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITDISW::_0)
    }
    #[doc = "ITM-to-TPIU trace path disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITDISW::_1)
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Counter Enable"]
    #[inline]
    pub fn cnten(&self) -> CNTENR {
        CNTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Response Type"]
    #[inline]
    pub fn rspt(&self) -> RSPTR {
        RSPTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Reload Request"]
    #[inline]
    pub fn rlrq(&self) -> RLRQR {
        RLRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - ETM-To-TPIU Disable"]
    #[inline]
    pub fn etdis(&self) -> ETDISR {
        ETDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - ITM-To-TPIU Disable"]
    #[inline]
    pub fn itdis(&self) -> ITDISR {
        ITDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Counter Enable"]
    #[inline]
    pub fn cnten(&mut self) -> _CNTENW {
        _CNTENW { w: self }
    }
    #[doc = "Bits 1:2 - Response Type"]
    #[inline]
    pub fn rspt(&mut self) -> _RSPTW {
        _RSPTW { w: self }
    }
    #[doc = "Bit 3 - Reload Request"]
    #[inline]
    pub fn rlrq(&mut self) -> _RLRQW {
        _RLRQW { w: self }
    }
    #[doc = "Bit 4 - ETM-To-TPIU Disable"]
    #[inline]
    pub fn etdis(&mut self) -> _ETDISW {
        _ETDISW { w: self }
    }
    #[doc = "Bit 5 - ITM-To-TPIU Disable"]
    #[inline]
    pub fn itdis(&mut self) -> _ITDISW {
        _ITDISW { w: self }
    }
}
