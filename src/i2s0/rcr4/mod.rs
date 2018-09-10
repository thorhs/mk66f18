#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCR4 {
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
#[doc = "Possible values of the field `FSD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSDR {
    #[doc = "Frame Sync is generated externally in Slave mode."]
    _0,
    #[doc = "Frame Sync is generated internally in Master mode."]
    _1,
}
impl FSDR {
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
            FSDR::_0 => false,
            FSDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSDR {
        match value {
            false => FSDR::_0,
            true => FSDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FSDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FSDR::_1
    }
}
#[doc = "Possible values of the field `FSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSPR {
    #[doc = "Frame sync is active high."]
    _0,
    #[doc = "Frame sync is active low."]
    _1,
}
impl FSPR {
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
            FSPR::_0 => false,
            FSPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSPR {
        match value {
            false => FSPR::_0,
            true => FSPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FSPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FSPR::_1
    }
}
#[doc = "Possible values of the field `ONDEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONDEMR {
    #[doc = "Internal frame sync is generated continuously."]
    _0,
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    _1,
}
impl ONDEMR {
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
            ONDEMR::_0 => false,
            ONDEMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONDEMR {
        match value {
            false => ONDEMR::_0,
            true => ONDEMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ONDEMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ONDEMR::_1
    }
}
#[doc = "Possible values of the field `FSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSER {
    #[doc = "Frame sync asserts with the first bit of the frame."]
    _0,
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    _1,
}
impl FSER {
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
            FSER::_0 => false,
            FSER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSER {
        match value {
            false => FSER::_0,
            true => FSER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FSER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FSER::_1
    }
}
#[doc = "Possible values of the field `MF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MFR {
    #[doc = "LSB is received first."]
    _0,
    #[doc = "MSB is received first."]
    _1,
}
impl MFR {
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
            MFR::_0 => false,
            MFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MFR {
        match value {
            false => MFR::_0,
            true => MFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MFR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SYWDR {
    bits: u8,
}
impl SYWDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FRSZR {
    bits: u8,
}
impl FRSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FPACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPACKR {
    #[doc = "FIFO packing is disabled"]
    _00,
    #[doc = "8-bit FIFO packing is enabled"]
    _10,
    #[doc = "16-bit FIFO packing is enabled"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FPACKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FPACKR::_00 => 0,
            FPACKR::_10 => 2,
            FPACKR::_11 => 3,
            FPACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FPACKR {
        match value {
            0 => FPACKR::_00,
            2 => FPACKR::_10,
            3 => FPACKR::_11,
            i => FPACKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FPACKR::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FPACKR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FPACKR::_11
    }
}
#[doc = "Possible values of the field `FCOMB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCOMBR {
    #[doc = "FIFO combine mode disabled."]
    _00,
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers)."]
    _01,
    #[doc = "FIFO combine mode enabled on FIFO reads (by software)."]
    _10,
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers) and reads (by software)."]
    _11,
}
impl FCOMBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FCOMBR::_00 => 0,
            FCOMBR::_01 => 1,
            FCOMBR::_10 => 2,
            FCOMBR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FCOMBR {
        match value {
            0 => FCOMBR::_00,
            1 => FCOMBR::_01,
            2 => FCOMBR::_10,
            3 => FCOMBR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FCOMBR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FCOMBR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FCOMBR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FCOMBR::_11
    }
}
#[doc = "Possible values of the field `FCONT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCONTR {
    #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    _0,
    #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    _1,
}
impl FCONTR {
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
            FCONTR::_0 => false,
            FCONTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCONTR {
        match value {
            false => FCONTR::_0,
            true => FCONTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FCONTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FCONTR::_1
    }
}
#[doc = "Values that can be written to the field `FSD`"]
pub enum FSDW {
    #[doc = "Frame Sync is generated externally in Slave mode."]
    _0,
    #[doc = "Frame Sync is generated internally in Master mode."]
    _1,
}
impl FSDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSDW::_0 => false,
            FSDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSDW<'a> {
    w: &'a mut W,
}
impl<'a> _FSDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame Sync is generated externally in Slave mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSDW::_0)
    }
    #[doc = "Frame Sync is generated internally in Master mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSDW::_1)
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
#[doc = "Values that can be written to the field `FSP`"]
pub enum FSPW {
    #[doc = "Frame sync is active high."]
    _0,
    #[doc = "Frame sync is active low."]
    _1,
}
impl FSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSPW::_0 => false,
            FSPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSPW<'a> {
    w: &'a mut W,
}
impl<'a> _FSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame sync is active high."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSPW::_0)
    }
    #[doc = "Frame sync is active low."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSPW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ONDEM`"]
pub enum ONDEMW {
    #[doc = "Internal frame sync is generated continuously."]
    _0,
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    _1,
}
impl ONDEMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONDEMW::_0 => false,
            ONDEMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONDEMW<'a> {
    w: &'a mut W,
}
impl<'a> _ONDEMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONDEMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal frame sync is generated continuously."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONDEMW::_0)
    }
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONDEMW::_1)
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
#[doc = "Values that can be written to the field `FSE`"]
pub enum FSEW {
    #[doc = "Frame sync asserts with the first bit of the frame."]
    _0,
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    _1,
}
impl FSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSEW::_0 => false,
            FSEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSEW<'a> {
    w: &'a mut W,
}
impl<'a> _FSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame sync asserts with the first bit of the frame."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSEW::_0)
    }
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSEW::_1)
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
#[doc = "Values that can be written to the field `MF`"]
pub enum MFW {
    #[doc = "LSB is received first."]
    _0,
    #[doc = "MSB is received first."]
    _1,
}
impl MFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MFW::_0 => false,
            MFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MFW<'a> {
    w: &'a mut W,
}
impl<'a> _MFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSB is received first."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MFW::_0)
    }
    #[doc = "MSB is received first."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MFW::_1)
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
#[doc = r" Proxy"]
pub struct _SYWDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYWDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRSZW<'a> {
    w: &'a mut W,
}
impl<'a> _FRSZW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FPACK`"]
pub enum FPACKW {
    #[doc = "FIFO packing is disabled"]
    _00,
    #[doc = "8-bit FIFO packing is enabled"]
    _10,
    #[doc = "16-bit FIFO packing is enabled"]
    _11,
}
impl FPACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FPACKW::_00 => 0,
            FPACKW::_10 => 2,
            FPACKW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPACKW<'a> {
    w: &'a mut W,
}
impl<'a> _FPACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPACKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FIFO packing is disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FPACKW::_00)
    }
    #[doc = "8-bit FIFO packing is enabled"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FPACKW::_10)
    }
    #[doc = "16-bit FIFO packing is enabled"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FPACKW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FCOMB`"]
pub enum FCOMBW {
    #[doc = "FIFO combine mode disabled."]
    _00,
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers)."]
    _01,
    #[doc = "FIFO combine mode enabled on FIFO reads (by software)."]
    _10,
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers) and reads (by software)."]
    _11,
}
impl FCOMBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FCOMBW::_00 => 0,
            FCOMBW::_01 => 1,
            FCOMBW::_10 => 2,
            FCOMBW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCOMBW<'a> {
    w: &'a mut W,
}
impl<'a> _FCOMBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCOMBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FIFO combine mode disabled."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FCOMBW::_00)
    }
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers)."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FCOMBW::_01)
    }
    #[doc = "FIFO combine mode enabled on FIFO reads (by software)."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FCOMBW::_10)
    }
    #[doc = "FIFO combine mode enabled on FIFO writes (from receive shift registers) and reads (by software)."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FCOMBW::_11)
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
#[doc = "Values that can be written to the field `FCONT`"]
pub enum FCONTW {
    #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    _0,
    #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    _1,
}
impl FCONTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCONTW::_0 => false,
            FCONTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCONTW<'a> {
    w: &'a mut W,
}
impl<'a> _FCONTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCONTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCONTW::_0)
    }
    #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCONTW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline]
    pub fn fsd(&self) -> FSDR {
        FSDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline]
    pub fn fsp(&self) -> FSPR {
        FSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline]
    pub fn ondem(&self) -> ONDEMR {
        ONDEMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline]
    pub fn fse(&self) -> FSER {
        FSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline]
    pub fn mf(&self) -> MFR {
        MFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline]
    pub fn sywd(&self) -> SYWDR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYWDR { bits }
    }
    #[doc = "Bits 16:20 - Frame Size"]
    #[inline]
    pub fn frsz(&self) -> FRSZR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FRSZR { bits }
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline]
    pub fn fpack(&self) -> FPACKR {
        FPACKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - FIFO Combine Mode"]
    #[inline]
    pub fn fcomb(&self) -> FCOMBR {
        FCOMBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline]
    pub fn fcont(&self) -> FCONTR {
        FCONTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline]
    pub fn fsd(&mut self) -> _FSDW {
        _FSDW { w: self }
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline]
    pub fn fsp(&mut self) -> _FSPW {
        _FSPW { w: self }
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline]
    pub fn ondem(&mut self) -> _ONDEMW {
        _ONDEMW { w: self }
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline]
    pub fn fse(&mut self) -> _FSEW {
        _FSEW { w: self }
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline]
    pub fn mf(&mut self) -> _MFW {
        _MFW { w: self }
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline]
    pub fn sywd(&mut self) -> _SYWDW {
        _SYWDW { w: self }
    }
    #[doc = "Bits 16:20 - Frame Size"]
    #[inline]
    pub fn frsz(&mut self) -> _FRSZW {
        _FRSZW { w: self }
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline]
    pub fn fpack(&mut self) -> _FPACKW {
        _FPACKW { w: self }
    }
    #[doc = "Bits 26:27 - FIFO Combine Mode"]
    #[inline]
    pub fn fcomb(&mut self) -> _FCOMBW {
        _FCOMBW { w: self }
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline]
    pub fn fcont(&mut self) -> _FCONTW {
        _FCONTW { w: self }
    }
}
