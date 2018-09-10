#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CRS {
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
#[doc = "Possible values of the field `PARK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARKR {
    #[doc = "Park on master port M0"]
    _000,
    #[doc = "Park on master port M1"]
    _001,
    #[doc = "Park on master port M2"]
    _010,
    #[doc = "Park on master port M3"]
    _011,
    #[doc = "Park on master port M4"]
    _100,
    #[doc = "Park on master port M5"]
    _101,
    #[doc = "Park on master port M6"]
    _110,
    #[doc = "Park on master port M7"]
    _111,
}
impl PARKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PARKR::_000 => 0,
            PARKR::_001 => 1,
            PARKR::_010 => 2,
            PARKR::_011 => 3,
            PARKR::_100 => 4,
            PARKR::_101 => 5,
            PARKR::_110 => 6,
            PARKR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PARKR {
        match value {
            0 => PARKR::_000,
            1 => PARKR::_001,
            2 => PARKR::_010,
            3 => PARKR::_011,
            4 => PARKR::_100,
            5 => PARKR::_101,
            6 => PARKR::_110,
            7 => PARKR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PARKR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PARKR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PARKR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PARKR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PARKR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PARKR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PARKR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PARKR::_111
    }
}
#[doc = "Possible values of the field `PCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCTLR {
    #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK field"]
    _00,
    #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port"]
    _01,
    #[doc = "When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCTLR::_00 => 0,
            PCTLR::_01 => 1,
            PCTLR::_10 => 2,
            PCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCTLR {
        match value {
            0 => PCTLR::_00,
            1 => PCTLR::_01,
            2 => PCTLR::_10,
            i => PCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PCTLR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PCTLR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PCTLR::_10
    }
}
#[doc = "Possible values of the field `ARB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBR {
    #[doc = "Fixed priority"]
    _00,
    #[doc = "Round-robin, or rotating, priority"]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ARBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ARBR::_00 => 0,
            ARBR::_01 => 1,
            ARBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ARBR {
        match value {
            0 => ARBR::_00,
            1 => ARBR::_01,
            i => ARBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ARBR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ARBR::_01
    }
}
#[doc = "Possible values of the field `HLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HLPR {
    #[doc = "The low power mode request has the highest priority for arbitration on this slave port"]
    _0,
    #[doc = "The low power mode request has the lowest initial priority for arbitration on this slave port"]
    _1,
}
impl HLPR {
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
            HLPR::_0 => false,
            HLPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HLPR {
        match value {
            false => HLPR::_0,
            true => HLPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HLPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HLPR::_1
    }
}
#[doc = "Possible values of the field `RO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROR {
    #[doc = "The slave port's registers are writeable"]
    _0,
    #[doc = "The slave port's registers are read-only and cannot be written. Attempted writes have no effect on the registers and result in a bus error response."]
    _1,
}
impl ROR {
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
            ROR::_0 => false,
            ROR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROR {
        match value {
            false => ROR::_0,
            true => ROR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ROR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ROR::_1
    }
}
#[doc = "Values that can be written to the field `PARK`"]
pub enum PARKW {
    #[doc = "Park on master port M0"]
    _000,
    #[doc = "Park on master port M1"]
    _001,
    #[doc = "Park on master port M2"]
    _010,
    #[doc = "Park on master port M3"]
    _011,
    #[doc = "Park on master port M4"]
    _100,
    #[doc = "Park on master port M5"]
    _101,
    #[doc = "Park on master port M6"]
    _110,
    #[doc = "Park on master port M7"]
    _111,
}
impl PARKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PARKW::_000 => 0,
            PARKW::_001 => 1,
            PARKW::_010 => 2,
            PARKW::_011 => 3,
            PARKW::_100 => 4,
            PARKW::_101 => 5,
            PARKW::_110 => 6,
            PARKW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARKW<'a> {
    w: &'a mut W,
}
impl<'a> _PARKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Park on master port M0"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PARKW::_000)
    }
    #[doc = "Park on master port M1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PARKW::_001)
    }
    #[doc = "Park on master port M2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PARKW::_010)
    }
    #[doc = "Park on master port M3"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PARKW::_011)
    }
    #[doc = "Park on master port M4"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PARKW::_100)
    }
    #[doc = "Park on master port M5"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PARKW::_101)
    }
    #[doc = "Park on master port M6"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PARKW::_110)
    }
    #[doc = "Park on master port M7"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PARKW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCTL`"]
pub enum PCTLW {
    #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK field"]
    _00,
    #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port"]
    _01,
    #[doc = "When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state"]
    _10,
}
impl PCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCTLW::_00 => 0,
            PCTLW::_01 => 1,
            PCTLW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _PCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK field"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PCTLW::_00)
    }
    #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PCTLW::_01)
    }
    #[doc = "When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PCTLW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ARB`"]
pub enum ARBW {
    #[doc = "Fixed priority"]
    _00,
    #[doc = "Round-robin, or rotating, priority"]
    _01,
}
impl ARBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ARBW::_00 => 0,
            ARBW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARBW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Fixed priority"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ARBW::_00)
    }
    #[doc = "Round-robin, or rotating, priority"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ARBW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HLP`"]
pub enum HLPW {
    #[doc = "The low power mode request has the highest priority for arbitration on this slave port"]
    _0,
    #[doc = "The low power mode request has the lowest initial priority for arbitration on this slave port"]
    _1,
}
impl HLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HLPW::_0 => false,
            HLPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HLPW<'a> {
    w: &'a mut W,
}
impl<'a> _HLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HLPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The low power mode request has the highest priority for arbitration on this slave port"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HLPW::_0)
    }
    #[doc = "The low power mode request has the lowest initial priority for arbitration on this slave port"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HLPW::_1)
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
#[doc = "Values that can be written to the field `RO`"]
pub enum ROW {
    #[doc = "The slave port's registers are writeable"]
    _0,
    #[doc = "The slave port's registers are read-only and cannot be written. Attempted writes have no effect on the registers and result in a bus error response."]
    _1,
}
impl ROW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROW::_0 => false,
            ROW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROW<'a> {
    w: &'a mut W,
}
impl<'a> _ROW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The slave port's registers are writeable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ROW::_0)
    }
    #[doc = "The slave port's registers are read-only and cannot be written. Attempted writes have no effect on the registers and result in a bus error response."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ROW::_1)
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
    #[doc = "Bits 0:2 - Park"]
    #[inline]
    pub fn park(&self) -> PARKR {
        PARKR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Parking Control"]
    #[inline]
    pub fn pctl(&self) -> PCTLR {
        PCTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Arbitration Mode"]
    #[inline]
    pub fn arb(&self) -> ARBR {
        ARBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - Halt Low Priority"]
    #[inline]
    pub fn hlp(&self) -> HLPR {
        HLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Read Only"]
    #[inline]
    pub fn ro(&self) -> ROR {
        ROR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Park"]
    #[inline]
    pub fn park(&mut self) -> _PARKW {
        _PARKW { w: self }
    }
    #[doc = "Bits 4:5 - Parking Control"]
    #[inline]
    pub fn pctl(&mut self) -> _PCTLW {
        _PCTLW { w: self }
    }
    #[doc = "Bits 8:9 - Arbitration Mode"]
    #[inline]
    pub fn arb(&mut self) -> _ARBW {
        _ARBW { w: self }
    }
    #[doc = "Bit 30 - Halt Low Priority"]
    #[inline]
    pub fn hlp(&mut self) -> _HLPW {
        _HLPW { w: self }
    }
    #[doc = "Bit 31 - Read Only"]
    #[inline]
    pub fn ro(&mut self) -> _ROW {
        _ROW { w: self }
    }
}
