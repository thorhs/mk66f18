#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::C0 {
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
#[doc = "Possible values of the field `DACBBIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBBIENR {
    #[doc = "The DAC buffer read pointer bottom flag interrupt is disabled."]
    _0,
    #[doc = "The DAC buffer read pointer bottom flag interrupt is enabled."]
    _1,
}
impl DACBBIENR {
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
            DACBBIENR::_0 => false,
            DACBBIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACBBIENR {
        match value {
            false => DACBBIENR::_0,
            true => DACBBIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DACBBIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DACBBIENR::_1
    }
}
#[doc = "Possible values of the field `DACBTIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBTIENR {
    #[doc = "The DAC buffer read pointer top flag interrupt is disabled."]
    _0,
    #[doc = "The DAC buffer read pointer top flag interrupt is enabled."]
    _1,
}
impl DACBTIENR {
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
            DACBTIENR::_0 => false,
            DACBTIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACBTIENR {
        match value {
            false => DACBTIENR::_0,
            true => DACBTIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DACBTIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DACBTIENR::_1
    }
}
#[doc = "Possible values of the field `DACBWIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBWIENR {
    #[doc = "The DAC buffer watermark interrupt is disabled."]
    _0,
    #[doc = "The DAC buffer watermark interrupt is enabled."]
    _1,
}
impl DACBWIENR {
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
            DACBWIENR::_0 => false,
            DACBWIENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACBWIENR {
        match value {
            false => DACBWIENR::_0,
            true => DACBWIENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DACBWIENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DACBWIENR::_1
    }
}
#[doc = "Possible values of the field `LPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPENR {
    #[doc = "High-Power mode"]
    _0,
    #[doc = "Low-Power mode"]
    _1,
}
impl LPENR {
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
            LPENR::_0 => false,
            LPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPENR {
        match value {
            false => LPENR::_0,
            true => LPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPENR::_1
    }
}
#[doc = "Possible values of the field `DACTRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACTRGSELR {
    #[doc = "The DAC hardware trigger is selected."]
    _0,
    #[doc = "The DAC software trigger is selected."]
    _1,
}
impl DACTRGSELR {
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
            DACTRGSELR::_0 => false,
            DACTRGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACTRGSELR {
        match value {
            false => DACTRGSELR::_0,
            true => DACTRGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DACTRGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DACTRGSELR::_1
    }
}
#[doc = "Possible values of the field `DACRFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACRFSR {
    #[doc = "The DAC selects DACREF_1 as the reference voltage."]
    _0,
    #[doc = "The DAC selects DACREF_2 as the reference voltage."]
    _1,
}
impl DACRFSR {
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
            DACRFSR::_0 => false,
            DACRFSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACRFSR {
        match value {
            false => DACRFSR::_0,
            true => DACRFSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DACRFSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DACRFSR::_1
    }
}
#[doc = "Possible values of the field `DACEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACENR {
    #[doc = "The DAC system is disabled."]
    _0,
    #[doc = "The DAC system is enabled."]
    _1,
}
impl DACENR {
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
            DACENR::_0 => false,
            DACENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACENR {
        match value {
            false => DACENR::_0,
            true => DACENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DACENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DACENR::_1
    }
}
#[doc = "Values that can be written to the field `DACBBIEN`"]
pub enum DACBBIENW {
    #[doc = "The DAC buffer read pointer bottom flag interrupt is disabled."]
    _0,
    #[doc = "The DAC buffer read pointer bottom flag interrupt is enabled."]
    _1,
}
impl DACBBIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACBBIENW::_0 => false,
            DACBBIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACBBIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACBBIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACBBIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DAC buffer read pointer bottom flag interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBBIENW::_0)
    }
    #[doc = "The DAC buffer read pointer bottom flag interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBBIENW::_1)
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
#[doc = "Values that can be written to the field `DACBTIEN`"]
pub enum DACBTIENW {
    #[doc = "The DAC buffer read pointer top flag interrupt is disabled."]
    _0,
    #[doc = "The DAC buffer read pointer top flag interrupt is enabled."]
    _1,
}
impl DACBTIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACBTIENW::_0 => false,
            DACBTIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACBTIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACBTIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACBTIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DAC buffer read pointer top flag interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBTIENW::_0)
    }
    #[doc = "The DAC buffer read pointer top flag interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBTIENW::_1)
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
#[doc = "Values that can be written to the field `DACBWIEN`"]
pub enum DACBWIENW {
    #[doc = "The DAC buffer watermark interrupt is disabled."]
    _0,
    #[doc = "The DAC buffer watermark interrupt is enabled."]
    _1,
}
impl DACBWIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACBWIENW::_0 => false,
            DACBWIENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACBWIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACBWIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACBWIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DAC buffer watermark interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBWIENW::_0)
    }
    #[doc = "The DAC buffer watermark interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBWIENW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPEN`"]
pub enum LPENW {
    #[doc = "High-Power mode"]
    _0,
    #[doc = "Low-Power mode"]
    _1,
}
impl LPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPENW::_0 => false,
            LPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "High-Power mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPENW::_0)
    }
    #[doc = "Low-Power mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPENW::_1)
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
#[doc = "Values that can be written to the field `DACSWTRG`"]
pub enum DACSWTRGW {
    #[doc = "The DAC soft trigger is not valid."]
    _0,
    #[doc = "The DAC soft trigger is valid."]
    _1,
}
impl DACSWTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACSWTRGW::_0 => false,
            DACSWTRGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACSWTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _DACSWTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACSWTRGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DAC soft trigger is not valid."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACSWTRGW::_0)
    }
    #[doc = "The DAC soft trigger is valid."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACSWTRGW::_1)
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
#[doc = "Values that can be written to the field `DACTRGSEL`"]
pub enum DACTRGSELW {
    #[doc = "The DAC hardware trigger is selected."]
    _0,
    #[doc = "The DAC software trigger is selected."]
    _1,
}
impl DACTRGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACTRGSELW::_0 => false,
            DACTRGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACTRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DACTRGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACTRGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DAC hardware trigger is selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACTRGSELW::_0)
    }
    #[doc = "The DAC software trigger is selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACTRGSELW::_1)
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
#[doc = "Values that can be written to the field `DACRFS`"]
pub enum DACRFSW {
    #[doc = "The DAC selects DACREF_1 as the reference voltage."]
    _0,
    #[doc = "The DAC selects DACREF_2 as the reference voltage."]
    _1,
}
impl DACRFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACRFSW::_0 => false,
            DACRFSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACRFSW<'a> {
    w: &'a mut W,
}
impl<'a> _DACRFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACRFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DAC selects DACREF_1 as the reference voltage."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACRFSW::_0)
    }
    #[doc = "The DAC selects DACREF_2 as the reference voltage."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACRFSW::_1)
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
#[doc = "Values that can be written to the field `DACEN`"]
pub enum DACENW {
    #[doc = "The DAC system is disabled."]
    _0,
    #[doc = "The DAC system is enabled."]
    _1,
}
impl DACENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACENW::_0 => false,
            DACENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DAC system is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACENW::_0)
    }
    #[doc = "The DAC system is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACENW::_1)
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
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
    #[inline]
    pub fn dacbbien(&self) -> DACBBIENR {
        DACBBIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
    #[inline]
    pub fn dacbtien(&self) -> DACBTIENR {
        DACBTIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - DAC Buffer Watermark Interrupt Enable"]
    #[inline]
    pub fn dacbwien(&self) -> DACBWIENR {
        DACBWIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - DAC Low Power Control"]
    #[inline]
    pub fn lpen(&self) -> LPENR {
        LPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - DAC Trigger Select"]
    #[inline]
    pub fn dactrgsel(&self) -> DACTRGSELR {
        DACTRGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - DAC Reference Select"]
    #[inline]
    pub fn dacrfs(&self) -> DACRFSR {
        DACRFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline]
    pub fn dacen(&self) -> DACENR {
        DACENR::_from({
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
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
    #[inline]
    pub fn dacbbien(&mut self) -> _DACBBIENW {
        _DACBBIENW { w: self }
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
    #[inline]
    pub fn dacbtien(&mut self) -> _DACBTIENW {
        _DACBTIENW { w: self }
    }
    #[doc = "Bit 2 - DAC Buffer Watermark Interrupt Enable"]
    #[inline]
    pub fn dacbwien(&mut self) -> _DACBWIENW {
        _DACBWIENW { w: self }
    }
    #[doc = "Bit 3 - DAC Low Power Control"]
    #[inline]
    pub fn lpen(&mut self) -> _LPENW {
        _LPENW { w: self }
    }
    #[doc = "Bit 4 - DAC Software Trigger"]
    #[inline]
    pub fn dacswtrg(&mut self) -> _DACSWTRGW {
        _DACSWTRGW { w: self }
    }
    #[doc = "Bit 5 - DAC Trigger Select"]
    #[inline]
    pub fn dactrgsel(&mut self) -> _DACTRGSELW {
        _DACTRGSELW { w: self }
    }
    #[doc = "Bit 6 - DAC Reference Select"]
    #[inline]
    pub fn dacrfs(&mut self) -> _DACRFSW {
        _DACRFSW { w: self }
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline]
    pub fn dacen(&mut self) -> _DACENW {
        _DACENW { w: self }
    }
}
