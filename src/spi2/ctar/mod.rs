#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTAR {
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
pub struct BRR {
    bits: u8,
}
impl BRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DTR {
    bits: u8,
}
impl DTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ASCR {
    bits: u8,
}
impl ASCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CSSCKR {
    bits: u8,
}
impl CSSCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PBR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBRR {
    #[doc = "Baud Rate Prescaler value is 2."]
    _00,
    #[doc = "Baud Rate Prescaler value is 3."]
    _01,
    #[doc = "Baud Rate Prescaler value is 5."]
    _10,
    #[doc = "Baud Rate Prescaler value is 7."]
    _11,
}
impl PBRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PBRR::_00 => 0,
            PBRR::_01 => 1,
            PBRR::_10 => 2,
            PBRR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PBRR {
        match value {
            0 => PBRR::_00,
            1 => PBRR::_01,
            2 => PBRR::_10,
            3 => PBRR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PBRR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PBRR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PBRR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PBRR::_11
    }
}
#[doc = "Possible values of the field `PDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDTR {
    #[doc = "Delay after Transfer Prescaler value is 1."]
    _00,
    #[doc = "Delay after Transfer Prescaler value is 3."]
    _01,
    #[doc = "Delay after Transfer Prescaler value is 5."]
    _10,
    #[doc = "Delay after Transfer Prescaler value is 7."]
    _11,
}
impl PDTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PDTR::_00 => 0,
            PDTR::_01 => 1,
            PDTR::_10 => 2,
            PDTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PDTR {
        match value {
            0 => PDTR::_00,
            1 => PDTR::_01,
            2 => PDTR::_10,
            3 => PDTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PDTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PDTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PDTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PDTR::_11
    }
}
#[doc = "Possible values of the field `PASC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PASCR {
    #[doc = "Delay after Transfer Prescaler value is 1."]
    _00,
    #[doc = "Delay after Transfer Prescaler value is 3."]
    _01,
    #[doc = "Delay after Transfer Prescaler value is 5."]
    _10,
    #[doc = "Delay after Transfer Prescaler value is 7."]
    _11,
}
impl PASCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PASCR::_00 => 0,
            PASCR::_01 => 1,
            PASCR::_10 => 2,
            PASCR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PASCR {
        match value {
            0 => PASCR::_00,
            1 => PASCR::_01,
            2 => PASCR::_10,
            3 => PASCR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PASCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PASCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PASCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PASCR::_11
    }
}
#[doc = "Possible values of the field `PCSSCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSSCKR {
    #[doc = "PCS to SCK Prescaler value is 1."]
    _00,
    #[doc = "PCS to SCK Prescaler value is 3."]
    _01,
    #[doc = "PCS to SCK Prescaler value is 5."]
    _10,
    #[doc = "PCS to SCK Prescaler value is 7."]
    _11,
}
impl PCSSCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCSSCKR::_00 => 0,
            PCSSCKR::_01 => 1,
            PCSSCKR::_10 => 2,
            PCSSCKR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCSSCKR {
        match value {
            0 => PCSSCKR::_00,
            1 => PCSSCKR::_01,
            2 => PCSSCKR::_10,
            3 => PCSSCKR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PCSSCKR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PCSSCKR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PCSSCKR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PCSSCKR::_11
    }
}
#[doc = "Possible values of the field `LSBFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFER {
    #[doc = "Data is transferred MSB first."]
    _0,
    #[doc = "Data is transferred LSB first."]
    _1,
}
impl LSBFER {
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
            LSBFER::_0 => false,
            LSBFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSBFER {
        match value {
            false => LSBFER::_0,
            true => LSBFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LSBFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LSBFER::_1
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge."]
    _0,
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge."]
    _1,
}
impl CPHAR {
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
            CPHAR::_0 => false,
            CPHAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::_0,
            true => CPHAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPHAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPHAR::_1
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "The inactive state value of SCK is low."]
    _0,
    #[doc = "The inactive state value of SCK is high."]
    _1,
}
impl CPOLR {
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
            CPOLR::_0 => false,
            CPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::_0,
            true => CPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPOLR::_1
    }
}
#[doc = r" Value of the field"]
pub struct FMSZR {
    bits: u8,
}
impl FMSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DBR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBRR {
    #[doc = "The baud rate is computed normally with a 50/50 duty cycle."]
    _0,
    #[doc = "The baud rate is doubled with the duty cycle depending on the Baud Rate Prescaler."]
    _1,
}
impl DBRR {
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
            DBRR::_0 => false,
            DBRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBRR {
        match value {
            false => DBRR::_0,
            true => DBRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DBRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DBRR::_1
    }
}
#[doc = r" Proxy"]
pub struct _BRW<'a> {
    w: &'a mut W,
}
impl<'a> _BRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTW<'a> {
    w: &'a mut W,
}
impl<'a> _DTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ASCW<'a> {
    w: &'a mut W,
}
impl<'a> _ASCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSSCKW<'a> {
    w: &'a mut W,
}
impl<'a> _CSSCKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PBR`"]
pub enum PBRW {
    #[doc = "Baud Rate Prescaler value is 2."]
    _00,
    #[doc = "Baud Rate Prescaler value is 3."]
    _01,
    #[doc = "Baud Rate Prescaler value is 5."]
    _10,
    #[doc = "Baud Rate Prescaler value is 7."]
    _11,
}
impl PBRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PBRW::_00 => 0,
            PBRW::_01 => 1,
            PBRW::_10 => 2,
            PBRW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PBRW<'a> {
    w: &'a mut W,
}
impl<'a> _PBRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PBRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Baud Rate Prescaler value is 2."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PBRW::_00)
    }
    #[doc = "Baud Rate Prescaler value is 3."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PBRW::_01)
    }
    #[doc = "Baud Rate Prescaler value is 5."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PBRW::_10)
    }
    #[doc = "Baud Rate Prescaler value is 7."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PBRW::_11)
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
#[doc = "Values that can be written to the field `PDT`"]
pub enum PDTW {
    #[doc = "Delay after Transfer Prescaler value is 1."]
    _00,
    #[doc = "Delay after Transfer Prescaler value is 3."]
    _01,
    #[doc = "Delay after Transfer Prescaler value is 5."]
    _10,
    #[doc = "Delay after Transfer Prescaler value is 7."]
    _11,
}
impl PDTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PDTW::_00 => 0,
            PDTW::_01 => 1,
            PDTW::_10 => 2,
            PDTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDTW<'a> {
    w: &'a mut W,
}
impl<'a> _PDTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Delay after Transfer Prescaler value is 1."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PDTW::_00)
    }
    #[doc = "Delay after Transfer Prescaler value is 3."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PDTW::_01)
    }
    #[doc = "Delay after Transfer Prescaler value is 5."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PDTW::_10)
    }
    #[doc = "Delay after Transfer Prescaler value is 7."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PDTW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PASC`"]
pub enum PASCW {
    #[doc = "Delay after Transfer Prescaler value is 1."]
    _00,
    #[doc = "Delay after Transfer Prescaler value is 3."]
    _01,
    #[doc = "Delay after Transfer Prescaler value is 5."]
    _10,
    #[doc = "Delay after Transfer Prescaler value is 7."]
    _11,
}
impl PASCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PASCW::_00 => 0,
            PASCW::_01 => 1,
            PASCW::_10 => 2,
            PASCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PASCW<'a> {
    w: &'a mut W,
}
impl<'a> _PASCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PASCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Delay after Transfer Prescaler value is 1."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PASCW::_00)
    }
    #[doc = "Delay after Transfer Prescaler value is 3."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PASCW::_01)
    }
    #[doc = "Delay after Transfer Prescaler value is 5."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PASCW::_10)
    }
    #[doc = "Delay after Transfer Prescaler value is 7."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PASCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCSSCK`"]
pub enum PCSSCKW {
    #[doc = "PCS to SCK Prescaler value is 1."]
    _00,
    #[doc = "PCS to SCK Prescaler value is 3."]
    _01,
    #[doc = "PCS to SCK Prescaler value is 5."]
    _10,
    #[doc = "PCS to SCK Prescaler value is 7."]
    _11,
}
impl PCSSCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCSSCKW::_00 => 0,
            PCSSCKW::_01 => 1,
            PCSSCKW::_10 => 2,
            PCSSCKW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSSCKW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSSCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSSCKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PCS to SCK Prescaler value is 1."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PCSSCKW::_00)
    }
    #[doc = "PCS to SCK Prescaler value is 3."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PCSSCKW::_01)
    }
    #[doc = "PCS to SCK Prescaler value is 5."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PCSSCKW::_10)
    }
    #[doc = "PCS to SCK Prescaler value is 7."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PCSSCKW::_11)
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
#[doc = "Values that can be written to the field `LSBFE`"]
pub enum LSBFEW {
    #[doc = "Data is transferred MSB first."]
    _0,
    #[doc = "Data is transferred LSB first."]
    _1,
}
impl LSBFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSBFEW::_0 => false,
            LSBFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSBFEW<'a> {
    w: &'a mut W,
}
impl<'a> _LSBFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSBFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is transferred MSB first."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSBFEW::_0)
    }
    #[doc = "Data is transferred LSB first."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSBFEW::_1)
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
#[doc = "Values that can be written to the field `CPHA`"]
pub enum CPHAW {
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge."]
    _0,
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge."]
    _1,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::_0 => false,
            CPHAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHAW::_0)
    }
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHAW::_1)
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
#[doc = "Values that can be written to the field `CPOL`"]
pub enum CPOLW {
    #[doc = "The inactive state value of SCK is low."]
    _0,
    #[doc = "The inactive state value of SCK is high."]
    _1,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::_0 => false,
            CPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The inactive state value of SCK is low."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOLW::_0)
    }
    #[doc = "The inactive state value of SCK is high."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOLW::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FMSZW<'a> {
    w: &'a mut W,
}
impl<'a> _FMSZW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBR`"]
pub enum DBRW {
    #[doc = "The baud rate is computed normally with a 50/50 duty cycle."]
    _0,
    #[doc = "The baud rate is doubled with the duty cycle depending on the Baud Rate Prescaler."]
    _1,
}
impl DBRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBRW::_0 => false,
            DBRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBRW<'a> {
    w: &'a mut W,
}
impl<'a> _DBRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The baud rate is computed normally with a 50/50 duty cycle."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBRW::_0)
    }
    #[doc = "The baud rate is doubled with the duty cycle depending on the Baud Rate Prescaler."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBRW::_1)
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
    #[doc = "Bits 0:3 - Baud Rate Scaler"]
    #[inline]
    pub fn br(&self) -> BRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BRR { bits }
    }
    #[doc = "Bits 4:7 - Delay After Transfer Scaler"]
    #[inline]
    pub fn dt(&self) -> DTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTR { bits }
    }
    #[doc = "Bits 8:11 - After SCK Delay Scaler"]
    #[inline]
    pub fn asc(&self) -> ASCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ASCR { bits }
    }
    #[doc = "Bits 12:15 - PCS to SCK Delay Scaler"]
    #[inline]
    pub fn cssck(&self) -> CSSCKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CSSCKR { bits }
    }
    #[doc = "Bits 16:17 - Baud Rate Prescaler"]
    #[inline]
    pub fn pbr(&self) -> PBRR {
        PBRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Delay after Transfer Prescaler"]
    #[inline]
    pub fn pdt(&self) -> PDTR {
        PDTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - After SCK Delay Prescaler"]
    #[inline]
    pub fn pasc(&self) -> PASCR {
        PASCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - PCS to SCK Delay Prescaler"]
    #[inline]
    pub fn pcssck(&self) -> PCSSCKR {
        PCSSCKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - LSB First"]
    #[inline]
    pub fn lsbfe(&self) -> LSBFER {
        LSBFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Clock Phase"]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Clock Polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 27:30 - Frame Size"]
    #[inline]
    pub fn fmsz(&self) -> FMSZR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FMSZR { bits }
    }
    #[doc = "Bit 31 - Double Baud Rate"]
    #[inline]
    pub fn dbr(&self) -> DBRR {
        DBRR::_from({
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
        W { bits: 2013265920 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Baud Rate Scaler"]
    #[inline]
    pub fn br(&mut self) -> _BRW {
        _BRW { w: self }
    }
    #[doc = "Bits 4:7 - Delay After Transfer Scaler"]
    #[inline]
    pub fn dt(&mut self) -> _DTW {
        _DTW { w: self }
    }
    #[doc = "Bits 8:11 - After SCK Delay Scaler"]
    #[inline]
    pub fn asc(&mut self) -> _ASCW {
        _ASCW { w: self }
    }
    #[doc = "Bits 12:15 - PCS to SCK Delay Scaler"]
    #[inline]
    pub fn cssck(&mut self) -> _CSSCKW {
        _CSSCKW { w: self }
    }
    #[doc = "Bits 16:17 - Baud Rate Prescaler"]
    #[inline]
    pub fn pbr(&mut self) -> _PBRW {
        _PBRW { w: self }
    }
    #[doc = "Bits 18:19 - Delay after Transfer Prescaler"]
    #[inline]
    pub fn pdt(&mut self) -> _PDTW {
        _PDTW { w: self }
    }
    #[doc = "Bits 20:21 - After SCK Delay Prescaler"]
    #[inline]
    pub fn pasc(&mut self) -> _PASCW {
        _PASCW { w: self }
    }
    #[doc = "Bits 22:23 - PCS to SCK Delay Prescaler"]
    #[inline]
    pub fn pcssck(&mut self) -> _PCSSCKW {
        _PCSSCKW { w: self }
    }
    #[doc = "Bit 24 - LSB First"]
    #[inline]
    pub fn lsbfe(&mut self) -> _LSBFEW {
        _LSBFEW { w: self }
    }
    #[doc = "Bit 25 - Clock Phase"]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 26 - Clock Polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bits 27:30 - Frame Size"]
    #[inline]
    pub fn fmsz(&mut self) -> _FMSZW {
        _FMSZW { w: self }
    }
    #[doc = "Bit 31 - Double Baud Rate"]
    #[inline]
    pub fn dbr(&mut self) -> _DBRW {
        _DBRW { w: self }
    }
}
