#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SMB {
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
#[doc = "Possible values of the field `SHTF2IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHTF2IER {
    #[doc = "SHTF2 interrupt is disabled"]
    _0,
    #[doc = "SHTF2 interrupt is enabled"]
    _1,
}
impl SHTF2IER {
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
            SHTF2IER::_0 => false,
            SHTF2IER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SHTF2IER {
        match value {
            false => SHTF2IER::_0,
            true => SHTF2IER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SHTF2IER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SHTF2IER::_1
    }
}
#[doc = "Possible values of the field `SHTF2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHTF2R {
    #[doc = "No SCL high and SDA low timeout occurs"]
    _0,
    #[doc = "SCL high and SDA low timeout occurs"]
    _1,
}
impl SHTF2R {
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
            SHTF2R::_0 => false,
            SHTF2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SHTF2R {
        match value {
            false => SHTF2R::_0,
            true => SHTF2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SHTF2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SHTF2R::_1
    }
}
#[doc = "Possible values of the field `SHTF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHTF1R {
    #[doc = "No SCL high and SDA high timeout occurs"]
    _0,
    #[doc = "SCL high and SDA high timeout occurs"]
    _1,
}
impl SHTF1R {
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
            SHTF1R::_0 => false,
            SHTF1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SHTF1R {
        match value {
            false => SHTF1R::_0,
            true => SHTF1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SHTF1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SHTF1R::_1
    }
}
#[doc = "Possible values of the field `SLTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLTFR {
    #[doc = "No low timeout occurs"]
    _0,
    #[doc = "Low timeout occurs"]
    _1,
}
impl SLTFR {
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
            SLTFR::_0 => false,
            SLTFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLTFR {
        match value {
            false => SLTFR::_0,
            true => SLTFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLTFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLTFR::_1
    }
}
#[doc = "Possible values of the field `TCKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCKSELR {
    #[doc = "Timeout counter counts at the frequency of the I2C module clock / 64"]
    _0,
    #[doc = "Timeout counter counts at the frequency of the I2C module clock"]
    _1,
}
impl TCKSELR {
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
            TCKSELR::_0 => false,
            TCKSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCKSELR {
        match value {
            false => TCKSELR::_0,
            true => TCKSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCKSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCKSELR::_1
    }
}
#[doc = "Possible values of the field `SIICAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIICAENR {
    #[doc = "I2C address register 2 matching is disabled"]
    _0,
    #[doc = "I2C address register 2 matching is enabled"]
    _1,
}
impl SIICAENR {
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
            SIICAENR::_0 => false,
            SIICAENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIICAENR {
        match value {
            false => SIICAENR::_0,
            true => SIICAENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SIICAENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SIICAENR::_1
    }
}
#[doc = "Possible values of the field `ALERTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERTENR {
    #[doc = "SMBus alert response address matching is disabled"]
    _0,
    #[doc = "SMBus alert response address matching is enabled"]
    _1,
}
impl ALERTENR {
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
            ALERTENR::_0 => false,
            ALERTENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALERTENR {
        match value {
            false => ALERTENR::_0,
            true => ALERTENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ALERTENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ALERTENR::_1
    }
}
#[doc = "Possible values of the field `FACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FACKR {
    #[doc = "An ACK or NACK is sent on the following receiving data byte"]
    _0,
    #[doc = "Writing 0 to TXAK after receiving a data byte generates an ACK. Writing 1 to TXAK after receiving a data byte generates a NACK."]
    _1,
}
impl FACKR {
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
            FACKR::_0 => false,
            FACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FACKR {
        match value {
            false => FACKR::_0,
            true => FACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FACKR::_1
    }
}
#[doc = "Values that can be written to the field `SHTF2IE`"]
pub enum SHTF2IEW {
    #[doc = "SHTF2 interrupt is disabled"]
    _0,
    #[doc = "SHTF2 interrupt is enabled"]
    _1,
}
impl SHTF2IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SHTF2IEW::_0 => false,
            SHTF2IEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHTF2IEW<'a> {
    w: &'a mut W,
}
impl<'a> _SHTF2IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHTF2IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SHTF2 interrupt is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHTF2IEW::_0)
    }
    #[doc = "SHTF2 interrupt is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHTF2IEW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SHTF2`"]
pub enum SHTF2W {
    #[doc = "No SCL high and SDA low timeout occurs"]
    _0,
    #[doc = "SCL high and SDA low timeout occurs"]
    _1,
}
impl SHTF2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SHTF2W::_0 => false,
            SHTF2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SHTF2W<'a> {
    w: &'a mut W,
}
impl<'a> _SHTF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SHTF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No SCL high and SDA low timeout occurs"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHTF2W::_0)
    }
    #[doc = "SCL high and SDA low timeout occurs"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHTF2W::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLTF`"]
pub enum SLTFW {
    #[doc = "No low timeout occurs"]
    _0,
    #[doc = "Low timeout occurs"]
    _1,
}
impl SLTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLTFW::_0 => false,
            SLTFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLTFW<'a> {
    w: &'a mut W,
}
impl<'a> _SLTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No low timeout occurs"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLTFW::_0)
    }
    #[doc = "Low timeout occurs"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLTFW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCKSEL`"]
pub enum TCKSELW {
    #[doc = "Timeout counter counts at the frequency of the I2C module clock / 64"]
    _0,
    #[doc = "Timeout counter counts at the frequency of the I2C module clock"]
    _1,
}
impl TCKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCKSELW::_0 => false,
            TCKSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TCKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timeout counter counts at the frequency of the I2C module clock / 64"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCKSELW::_0)
    }
    #[doc = "Timeout counter counts at the frequency of the I2C module clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCKSELW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SIICAEN`"]
pub enum SIICAENW {
    #[doc = "I2C address register 2 matching is disabled"]
    _0,
    #[doc = "I2C address register 2 matching is enabled"]
    _1,
}
impl SIICAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIICAENW::_0 => false,
            SIICAENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIICAENW<'a> {
    w: &'a mut W,
}
impl<'a> _SIICAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIICAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "I2C address register 2 matching is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIICAENW::_0)
    }
    #[doc = "I2C address register 2 matching is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIICAENW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ALERTEN`"]
pub enum ALERTENW {
    #[doc = "SMBus alert response address matching is disabled"]
    _0,
    #[doc = "SMBus alert response address matching is enabled"]
    _1,
}
impl ALERTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALERTENW::_0 => false,
            ALERTENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALERTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ALERTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALERTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SMBus alert response address matching is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALERTENW::_0)
    }
    #[doc = "SMBus alert response address matching is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALERTENW::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FACK`"]
pub enum FACKW {
    #[doc = "An ACK or NACK is sent on the following receiving data byte"]
    _0,
    #[doc = "Writing 0 to TXAK after receiving a data byte generates an ACK. Writing 1 to TXAK after receiving a data byte generates a NACK."]
    _1,
}
impl FACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FACKW::_0 => false,
            FACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FACKW<'a> {
    w: &'a mut W,
}
impl<'a> _FACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "An ACK or NACK is sent on the following receiving data byte"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FACKW::_0)
    }
    #[doc = "Writing 0 to TXAK after receiving a data byte generates an ACK. Writing 1 to TXAK after receiving a data byte generates a NACK."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FACKW::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - SHTF2 Interrupt Enable"]
    #[inline]
    pub fn shtf2ie(&self) -> SHTF2IER {
        SHTF2IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - SCL High Timeout Flag 2"]
    #[inline]
    pub fn shtf2(&self) -> SHTF2R {
        SHTF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - SCL High Timeout Flag 1"]
    #[inline]
    pub fn shtf1(&self) -> SHTF1R {
        SHTF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - SCL Low Timeout Flag"]
    #[inline]
    pub fn sltf(&self) -> SLTFR {
        SLTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Timeout Counter Clock Select"]
    #[inline]
    pub fn tcksel(&self) -> TCKSELR {
        TCKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Second I2C Address Enable"]
    #[inline]
    pub fn siicaen(&self) -> SIICAENR {
        SIICAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - SMBus Alert Response Address Enable"]
    #[inline]
    pub fn alerten(&self) -> ALERTENR {
        ALERTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Fast NACK/ACK Enable"]
    #[inline]
    pub fn fack(&self) -> FACKR {
        FACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SHTF2 Interrupt Enable"]
    #[inline]
    pub fn shtf2ie(&mut self) -> _SHTF2IEW {
        _SHTF2IEW { w: self }
    }
    #[doc = "Bit 1 - SCL High Timeout Flag 2"]
    #[inline]
    pub fn shtf2(&mut self) -> _SHTF2W {
        _SHTF2W { w: self }
    }
    #[doc = "Bit 3 - SCL Low Timeout Flag"]
    #[inline]
    pub fn sltf(&mut self) -> _SLTFW {
        _SLTFW { w: self }
    }
    #[doc = "Bit 4 - Timeout Counter Clock Select"]
    #[inline]
    pub fn tcksel(&mut self) -> _TCKSELW {
        _TCKSELW { w: self }
    }
    #[doc = "Bit 5 - Second I2C Address Enable"]
    #[inline]
    pub fn siicaen(&mut self) -> _SIICAENW {
        _SIICAENW { w: self }
    }
    #[doc = "Bit 6 - SMBus Alert Response Address Enable"]
    #[inline]
    pub fn alerten(&mut self) -> _ALERTENW {
        _ALERTENW { w: self }
    }
    #[doc = "Bit 7 - Fast NACK/ACK Enable"]
    #[inline]
    pub fn fack(&mut self) -> _FACKW {
        _FACKW { w: self }
    }
}
