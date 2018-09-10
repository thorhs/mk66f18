#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XFERTYP {
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
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl DMAENR {
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
            DMAENR::_0 => false,
            DMAENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAENR {
        match value {
            false => DMAENR::_0,
            true => DMAENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAENR::_1
    }
}
#[doc = "Possible values of the field `BCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCENR {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl BCENR {
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
            BCENR::_0 => false,
            BCENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCENR {
        match value {
            false => BCENR::_0,
            true => BCENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BCENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BCENR::_1
    }
}
#[doc = "Possible values of the field `AC12EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AC12ENR {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl AC12ENR {
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
            AC12ENR::_0 => false,
            AC12ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AC12ENR {
        match value {
            false => AC12ENR::_0,
            true => AC12ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AC12ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AC12ENR::_1
    }
}
#[doc = "Possible values of the field `DTDSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTDSELR {
    #[doc = "Write host to card."]
    _0,
    #[doc = "Read card to host."]
    _1,
}
impl DTDSELR {
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
            DTDSELR::_0 => false,
            DTDSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTDSELR {
        match value {
            false => DTDSELR::_0,
            true => DTDSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DTDSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DTDSELR::_1
    }
}
#[doc = "Possible values of the field `MSBSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBSELR {
    #[doc = "Single block."]
    _0,
    #[doc = "Multiple blocks."]
    _1,
}
impl MSBSELR {
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
            MSBSELR::_0 => false,
            MSBSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSBSELR {
        match value {
            false => MSBSELR::_0,
            true => MSBSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MSBSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MSBSELR::_1
    }
}
#[doc = "Possible values of the field `RSPTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSPTYPR {
    #[doc = "No response."]
    _00,
    #[doc = "Response length 136."]
    _01,
    #[doc = "Response length 48."]
    _10,
    #[doc = "Response length 48, check busy after response."]
    _11,
}
impl RSPTYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSPTYPR::_00 => 0,
            RSPTYPR::_01 => 1,
            RSPTYPR::_10 => 2,
            RSPTYPR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSPTYPR {
        match value {
            0 => RSPTYPR::_00,
            1 => RSPTYPR::_01,
            2 => RSPTYPR::_10,
            3 => RSPTYPR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RSPTYPR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RSPTYPR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RSPTYPR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RSPTYPR::_11
    }
}
#[doc = "Possible values of the field `CCCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCCENR {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl CCCENR {
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
            CCCENR::_0 => false,
            CCCENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCCENR {
        match value {
            false => CCCENR::_0,
            true => CCCENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CCCENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CCCENR::_1
    }
}
#[doc = "Possible values of the field `CICEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CICENR {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl CICENR {
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
            CICENR::_0 => false,
            CICENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CICENR {
        match value {
            false => CICENR::_0,
            true => CICENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CICENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CICENR::_1
    }
}
#[doc = "Possible values of the field `DPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPSELR {
    #[doc = "No data present."]
    _0,
    #[doc = "Data present."]
    _1,
}
impl DPSELR {
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
            DPSELR::_0 => false,
            DPSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPSELR {
        match value {
            false => DPSELR::_0,
            true => DPSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DPSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DPSELR::_1
    }
}
#[doc = "Possible values of the field `CMDTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDTYPR {
    #[doc = "Normal other commands."]
    _00,
    #[doc = "Suspend CMD52 for writing bus suspend in CCCR."]
    _01,
    #[doc = "Resume CMD52 for writing function select in CCCR."]
    _10,
    #[doc = "Abort CMD12, CMD52 for writing I/O abort in CCCR."]
    _11,
}
impl CMDTYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDTYPR::_00 => 0,
            CMDTYPR::_01 => 1,
            CMDTYPR::_10 => 2,
            CMDTYPR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDTYPR {
        match value {
            0 => CMDTYPR::_00,
            1 => CMDTYPR::_01,
            2 => CMDTYPR::_10,
            3 => CMDTYPR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == CMDTYPR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == CMDTYPR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == CMDTYPR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == CMDTYPR::_11
    }
}
#[doc = r" Value of the field"]
pub struct CMDINXR {
    bits: u8,
}
impl CMDINXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `DMAEN`"]
pub enum DMAENW {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENW::_0 => false,
            DMAENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAENW::_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAENW::_1)
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
#[doc = "Values that can be written to the field `BCEN`"]
pub enum BCENW {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl BCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCENW::_0 => false,
            BCENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCENW<'a> {
    w: &'a mut W,
}
impl<'a> _BCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCENW::_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCENW::_1)
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
#[doc = "Values that can be written to the field `AC12EN`"]
pub enum AC12ENW {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl AC12ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AC12ENW::_0 => false,
            AC12ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AC12ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AC12ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AC12ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AC12ENW::_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AC12ENW::_1)
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
#[doc = "Values that can be written to the field `DTDSEL`"]
pub enum DTDSELW {
    #[doc = "Write host to card."]
    _0,
    #[doc = "Read card to host."]
    _1,
}
impl DTDSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTDSELW::_0 => false,
            DTDSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTDSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DTDSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTDSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write host to card."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTDSELW::_0)
    }
    #[doc = "Read card to host."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTDSELW::_1)
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
#[doc = "Values that can be written to the field `MSBSEL`"]
pub enum MSBSELW {
    #[doc = "Single block."]
    _0,
    #[doc = "Multiple blocks."]
    _1,
}
impl MSBSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSBSELW::_0 => false,
            MSBSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSBSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSBSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single block."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSBSELW::_0)
    }
    #[doc = "Multiple blocks."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSBSELW::_1)
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
#[doc = "Values that can be written to the field `RSPTYP`"]
pub enum RSPTYPW {
    #[doc = "No response."]
    _00,
    #[doc = "Response length 136."]
    _01,
    #[doc = "Response length 48."]
    _10,
    #[doc = "Response length 48, check busy after response."]
    _11,
}
impl RSPTYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSPTYPW::_00 => 0,
            RSPTYPW::_01 => 1,
            RSPTYPW::_10 => 2,
            RSPTYPW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSPTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _RSPTYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSPTYPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No response."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSPTYPW::_00)
    }
    #[doc = "Response length 136."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSPTYPW::_01)
    }
    #[doc = "Response length 48."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSPTYPW::_10)
    }
    #[doc = "Response length 48, check busy after response."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RSPTYPW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCCEN`"]
pub enum CCCENW {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl CCCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCCENW::_0 => false,
            CCCENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCCENW::_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCCENW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CICEN`"]
pub enum CICENW {
    #[doc = "Disable"]
    _0,
    #[doc = "Enable"]
    _1,
}
impl CICENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CICENW::_0 => false,
            CICENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CICENW<'a> {
    w: &'a mut W,
}
impl<'a> _CICENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CICENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CICENW::_0)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CICENW::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DPSEL`"]
pub enum DPSELW {
    #[doc = "No data present."]
    _0,
    #[doc = "Data present."]
    _1,
}
impl DPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPSELW::_0 => false,
            DPSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No data present."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPSELW::_0)
    }
    #[doc = "Data present."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPSELW::_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMDTYP`"]
pub enum CMDTYPW {
    #[doc = "Normal other commands."]
    _00,
    #[doc = "Suspend CMD52 for writing bus suspend in CCCR."]
    _01,
    #[doc = "Resume CMD52 for writing function select in CCCR."]
    _10,
    #[doc = "Abort CMD12, CMD52 for writing I/O abort in CCCR."]
    _11,
}
impl CMDTYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDTYPW::_00 => 0,
            CMDTYPW::_01 => 1,
            CMDTYPW::_10 => 2,
            CMDTYPW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDTYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDTYPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal other commands."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMDTYPW::_00)
    }
    #[doc = "Suspend CMD52 for writing bus suspend in CCCR."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMDTYPW::_01)
    }
    #[doc = "Resume CMD52 for writing function select in CCCR."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMDTYPW::_10)
    }
    #[doc = "Abort CMD12, CMD52 for writing I/O abort in CCCR."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMDTYPW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMDINXW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDINXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - DMA Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline]
    pub fn bcen(&self) -> BCENR {
        BCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Auto CMD12 Enable"]
    #[inline]
    pub fn ac12en(&self) -> AC12ENR {
        AC12ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline]
    pub fn dtdsel(&self) -> DTDSELR {
        DTDSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Multi/Single Block Select"]
    #[inline]
    pub fn msbsel(&self) -> MSBSELR {
        MSBSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline]
    pub fn rsptyp(&self) -> RSPTYPR {
        RSPTYPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline]
    pub fn cccen(&self) -> CCCENR {
        CCCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline]
    pub fn cicen(&self) -> CICENR {
        CICENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline]
    pub fn dpsel(&self) -> DPSELR {
        DPSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline]
    pub fn cmdtyp(&self) -> CMDTYPR {
        CMDTYPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline]
    pub fn cmdinx(&self) -> CMDINXR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMDINXR { bits }
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
    #[doc = "Bit 0 - DMA Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline]
    pub fn bcen(&mut self) -> _BCENW {
        _BCENW { w: self }
    }
    #[doc = "Bit 2 - Auto CMD12 Enable"]
    #[inline]
    pub fn ac12en(&mut self) -> _AC12ENW {
        _AC12ENW { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline]
    pub fn dtdsel(&mut self) -> _DTDSELW {
        _DTDSELW { w: self }
    }
    #[doc = "Bit 5 - Multi/Single Block Select"]
    #[inline]
    pub fn msbsel(&mut self) -> _MSBSELW {
        _MSBSELW { w: self }
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline]
    pub fn rsptyp(&mut self) -> _RSPTYPW {
        _RSPTYPW { w: self }
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline]
    pub fn cccen(&mut self) -> _CCCENW {
        _CCCENW { w: self }
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline]
    pub fn cicen(&mut self) -> _CICENW {
        _CICENW { w: self }
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline]
    pub fn dpsel(&mut self) -> _DPSELW {
        _DPSELW { w: self }
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline]
    pub fn cmdtyp(&mut self) -> _CMDTYPW {
        _CMDTYPW { w: self }
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline]
    pub fn cmdinx(&mut self) -> _CMDINXW {
        _CMDINXW { w: self }
    }
}
