#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::IE7816 {
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
#[doc = "Possible values of the field `RXTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTER {
    #[doc = "The assertion of IS7816\\[RXT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[RXT\\] results in the generation of an interrupt."]
    _1,
}
impl RXTER {
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
            RXTER::_0 => false,
            RXTER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTER {
        match value {
            false => RXTER::_0,
            true => RXTER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXTER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXTER::_1
    }
}
#[doc = "Possible values of the field `TXTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXTER {
    #[doc = "The assertion of IS7816\\[TXT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[TXT\\] results in the generation of an interrupt."]
    _1,
}
impl TXTER {
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
            TXTER::_0 => false,
            TXTER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXTER {
        match value {
            false => TXTER::_0,
            true => TXTER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXTER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXTER::_1
    }
}
#[doc = "Possible values of the field `GTVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTVER {
    #[doc = "The assertion of IS7816\\[GTV\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[GTV\\] results in the generation of an interrupt."]
    _1,
}
impl GTVER {
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
            GTVER::_0 => false,
            GTVER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GTVER {
        match value {
            false => GTVER::_0,
            true => GTVER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GTVER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GTVER::_1
    }
}
#[doc = "Possible values of the field `ADTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADTER {
    #[doc = "The assertion of IS7816\\[ADT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[ADT\\] results in the generation of an interrupt."]
    _1,
}
impl ADTER {
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
            ADTER::_0 => false,
            ADTER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADTER {
        match value {
            false => ADTER::_0,
            true => ADTER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADTER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADTER::_1
    }
}
#[doc = "Possible values of the field `INITDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITDER {
    #[doc = "The assertion of IS7816\\[INITD\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[INITD\\] results in the generation of an interrupt."]
    _1,
}
impl INITDER {
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
            INITDER::_0 => false,
            INITDER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INITDER {
        match value {
            false => INITDER::_0,
            true => INITDER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INITDER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INITDER::_1
    }
}
#[doc = "Possible values of the field `BWTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWTER {
    #[doc = "The assertion of IS7816\\[BWT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[BWT\\] results in the generation of an interrupt."]
    _1,
}
impl BWTER {
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
            BWTER::_0 => false,
            BWTER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BWTER {
        match value {
            false => BWTER::_0,
            true => BWTER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BWTER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BWTER::_1
    }
}
#[doc = "Possible values of the field `CWTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWTER {
    #[doc = "The assertion of IS7816\\[CWT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[CWT\\] results in the generation of an interrupt."]
    _1,
}
impl CWTER {
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
            CWTER::_0 => false,
            CWTER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CWTER {
        match value {
            false => CWTER::_0,
            true => CWTER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CWTER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CWTER::_1
    }
}
#[doc = "Possible values of the field `WTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTER {
    #[doc = "The assertion of IS7816\\[WT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[WT\\] results in the generation of an interrupt."]
    _1,
}
impl WTER {
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
            WTER::_0 => false,
            WTER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WTER {
        match value {
            false => WTER::_0,
            true => WTER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WTER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WTER::_1
    }
}
#[doc = "Values that can be written to the field `RXTE`"]
pub enum RXTEW {
    #[doc = "The assertion of IS7816\\[RXT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[RXT\\] results in the generation of an interrupt."]
    _1,
}
impl RXTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTEW::_0 => false,
            RXTEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The assertion of IS7816\\[RXT\\] does not result in the generation of an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTEW::_0)
    }
    #[doc = "The assertion of IS7816\\[RXT\\] results in the generation of an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTEW::_1)
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
#[doc = "Values that can be written to the field `TXTE`"]
pub enum TXTEW {
    #[doc = "The assertion of IS7816\\[TXT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[TXT\\] results in the generation of an interrupt."]
    _1,
}
impl TXTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXTEW::_0 => false,
            TXTEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXTEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The assertion of IS7816\\[TXT\\] does not result in the generation of an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXTEW::_0)
    }
    #[doc = "The assertion of IS7816\\[TXT\\] results in the generation of an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXTEW::_1)
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
#[doc = "Values that can be written to the field `GTVE`"]
pub enum GTVEW {
    #[doc = "The assertion of IS7816\\[GTV\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[GTV\\] results in the generation of an interrupt."]
    _1,
}
impl GTVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GTVEW::_0 => false,
            GTVEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GTVEW<'a> {
    w: &'a mut W,
}
impl<'a> _GTVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GTVEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The assertion of IS7816\\[GTV\\] does not result in the generation of an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTVEW::_0)
    }
    #[doc = "The assertion of IS7816\\[GTV\\] results in the generation of an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTVEW::_1)
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
#[doc = "Values that can be written to the field `ADTE`"]
pub enum ADTEW {
    #[doc = "The assertion of IS7816\\[ADT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[ADT\\] results in the generation of an interrupt."]
    _1,
}
impl ADTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADTEW::_0 => false,
            ADTEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADTEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The assertion of IS7816\\[ADT\\] does not result in the generation of an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTEW::_0)
    }
    #[doc = "The assertion of IS7816\\[ADT\\] results in the generation of an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTEW::_1)
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
#[doc = "Values that can be written to the field `INITDE`"]
pub enum INITDEW {
    #[doc = "The assertion of IS7816\\[INITD\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[INITD\\] results in the generation of an interrupt."]
    _1,
}
impl INITDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INITDEW::_0 => false,
            INITDEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INITDEW<'a> {
    w: &'a mut W,
}
impl<'a> _INITDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INITDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The assertion of IS7816\\[INITD\\] does not result in the generation of an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INITDEW::_0)
    }
    #[doc = "The assertion of IS7816\\[INITD\\] results in the generation of an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INITDEW::_1)
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
#[doc = "Values that can be written to the field `BWTE`"]
pub enum BWTEW {
    #[doc = "The assertion of IS7816\\[BWT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[BWT\\] results in the generation of an interrupt."]
    _1,
}
impl BWTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BWTEW::_0 => false,
            BWTEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWTEW<'a> {
    w: &'a mut W,
}
impl<'a> _BWTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The assertion of IS7816\\[BWT\\] does not result in the generation of an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWTEW::_0)
    }
    #[doc = "The assertion of IS7816\\[BWT\\] results in the generation of an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWTEW::_1)
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
#[doc = "Values that can be written to the field `CWTE`"]
pub enum CWTEW {
    #[doc = "The assertion of IS7816\\[CWT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[CWT\\] results in the generation of an interrupt."]
    _1,
}
impl CWTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CWTEW::_0 => false,
            CWTEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CWTEW<'a> {
    w: &'a mut W,
}
impl<'a> _CWTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CWTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The assertion of IS7816\\[CWT\\] does not result in the generation of an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CWTEW::_0)
    }
    #[doc = "The assertion of IS7816\\[CWT\\] results in the generation of an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CWTEW::_1)
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
#[doc = "Values that can be written to the field `WTE`"]
pub enum WTEW {
    #[doc = "The assertion of IS7816\\[WT\\] does not result in the generation of an interrupt."]
    _0,
    #[doc = "The assertion of IS7816\\[WT\\] results in the generation of an interrupt."]
    _1,
}
impl WTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WTEW::_0 => false,
            WTEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WTEW<'a> {
    w: &'a mut W,
}
impl<'a> _WTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The assertion of IS7816\\[WT\\] does not result in the generation of an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WTEW::_0)
    }
    #[doc = "The assertion of IS7816\\[WT\\] results in the generation of an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WTEW::_1)
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
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt Enable"]
    #[inline]
    pub fn rxte(&self) -> RXTER {
        RXTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt Enable"]
    #[inline]
    pub fn txte(&self) -> TXTER {
        TXTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt Enable"]
    #[inline]
    pub fn gtve(&self) -> GTVER {
        GTVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - ATR Duration Timer Interrupt Enable"]
    #[inline]
    pub fn adte(&self) -> ADTER {
        ADTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt Enable"]
    #[inline]
    pub fn initde(&self) -> INITDER {
        INITDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt Enable"]
    #[inline]
    pub fn bwte(&self) -> BWTER {
        BWTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt Enable"]
    #[inline]
    pub fn cwte(&self) -> CWTER {
        CWTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Wait Timer Interrupt Enable"]
    #[inline]
    pub fn wte(&self) -> WTER {
        WTER::_from({
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
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt Enable"]
    #[inline]
    pub fn rxte(&mut self) -> _RXTEW {
        _RXTEW { w: self }
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt Enable"]
    #[inline]
    pub fn txte(&mut self) -> _TXTEW {
        _TXTEW { w: self }
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt Enable"]
    #[inline]
    pub fn gtve(&mut self) -> _GTVEW {
        _GTVEW { w: self }
    }
    #[doc = "Bit 3 - ATR Duration Timer Interrupt Enable"]
    #[inline]
    pub fn adte(&mut self) -> _ADTEW {
        _ADTEW { w: self }
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt Enable"]
    #[inline]
    pub fn initde(&mut self) -> _INITDEW {
        _INITDEW { w: self }
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt Enable"]
    #[inline]
    pub fn bwte(&mut self) -> _BWTEW {
        _BWTEW { w: self }
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt Enable"]
    #[inline]
    pub fn cwte(&mut self) -> _CWTEW {
        _CWTEW { w: self }
    }
    #[doc = "Bit 7 - Wait Timer Interrupt Enable"]
    #[inline]
    pub fn wte(&mut self) -> _WTEW {
        _WTEW { w: self }
    }
}
