#[doc = "Reader of register EPCR%s"]
pub type R = crate::R<u32, super::EPCR>;
#[doc = "Writer for register EPCR%s"]
pub type W = crate::W<u32, super::EPCR>;
#[doc = "Register EPCR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::EPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RX endpoint Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXS_A {
    #[doc = "0: Endpoint OK"]
    _0,
    #[doc = "1: Endpoint stalled"]
    _1,
}
impl From<RXS_A> for bool {
    #[inline(always)]
    fn from(variant: RXS_A) -> Self {
        match variant {
            RXS_A::_0 => false,
            RXS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXS`"]
pub type RXS_R = crate::R<bool, RXS_A>;
impl RXS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXS_A {
        match self.bits {
            false => RXS_A::_0,
            true => RXS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXS_A::_1
    }
}
#[doc = "Write proxy for field `RXS`"]
pub struct RXS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint OK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXS_A::_0)
    }
    #[doc = "Endpoint stalled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RXD`"]
pub type RXD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXD`"]
pub struct RXD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "RX endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXT_A {
    #[doc = "0: Control"]
    _00,
    #[doc = "1: Isochronous"]
    _01,
    #[doc = "2: Bulk"]
    _10,
    #[doc = "3: Interrupt"]
    _11,
}
impl From<RXT_A> for u8 {
    #[inline(always)]
    fn from(variant: RXT_A) -> Self {
        match variant {
            RXT_A::_00 => 0,
            RXT_A::_01 => 1,
            RXT_A::_10 => 2,
            RXT_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `RXT`"]
pub type RXT_R = crate::R<u8, RXT_A>;
impl RXT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXT_A {
        match self.bits {
            0 => RXT_A::_00,
            1 => RXT_A::_01,
            2 => RXT_A::_10,
            3 => RXT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RXT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RXT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RXT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RXT_A::_11
    }
}
#[doc = "Write proxy for field `RXT`"]
pub struct RXT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RXT_A::_00)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RXT_A::_01)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RXT_A::_10)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RXT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "RX data toggle Inhibit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXI_A {
    #[doc = "0: PID sequencing enabled"]
    _0,
    #[doc = "1: PID sequencing disabled"]
    _1,
}
impl From<RXI_A> for bool {
    #[inline(always)]
    fn from(variant: RXI_A) -> Self {
        match variant {
            RXI_A::_0 => false,
            RXI_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXI`"]
pub type RXI_R = crate::R<bool, RXI_A>;
impl RXI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXI_A {
        match self.bits {
            false => RXI_A::_0,
            true => RXI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXI_A::_1
    }
}
#[doc = "Write proxy for field `RXI`"]
pub struct RXI_W<'a> {
    w: &'a mut W,
}
impl<'a> RXI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PID sequencing enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXI_A::_0)
    }
    #[doc = "PID sequencing disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXI_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `RXR`"]
pub struct RXR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "RX endpoint Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<RXE_A> for bool {
    #[inline(always)]
    fn from(variant: RXE_A) -> Self {
        match variant {
            RXE_A::_0 => false,
            RXE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXE`"]
pub type RXE_R = crate::R<bool, RXE_A>;
impl RXE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXE_A {
        match self.bits {
            false => RXE_A::_0,
            true => RXE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXE_A::_1
    }
}
#[doc = "Write proxy for field `RXE`"]
pub struct RXE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "TX endpoint Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXS_A {
    #[doc = "0: Endpoint OK"]
    _0,
    #[doc = "1: Endpoint stalled"]
    _1,
}
impl From<TXS_A> for bool {
    #[inline(always)]
    fn from(variant: TXS_A) -> Self {
        match variant {
            TXS_A::_0 => false,
            TXS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXS`"]
pub type TXS_R = crate::R<bool, TXS_A>;
impl TXS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXS_A {
        match self.bits {
            false => TXS_A::_0,
            true => TXS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXS_A::_1
    }
}
#[doc = "Write proxy for field `TXS`"]
pub struct TXS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint OK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXS_A::_0)
    }
    #[doc = "Endpoint stalled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TXD`"]
pub type TXD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXD`"]
pub struct TXD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "TX endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXT_A {
    #[doc = "0: Control"]
    _00,
    #[doc = "1: Isochronous"]
    _01,
    #[doc = "2: Bulk"]
    _10,
    #[doc = "3: Interrupt"]
    _11,
}
impl From<TXT_A> for u8 {
    #[inline(always)]
    fn from(variant: TXT_A) -> Self {
        match variant {
            TXT_A::_00 => 0,
            TXT_A::_01 => 1,
            TXT_A::_10 => 2,
            TXT_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `TXT`"]
pub type TXT_R = crate::R<u8, TXT_A>;
impl TXT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXT_A {
        match self.bits {
            0 => TXT_A::_00,
            1 => TXT_A::_01,
            2 => TXT_A::_10,
            3 => TXT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TXT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TXT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TXT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TXT_A::_11
    }
}
#[doc = "Write proxy for field `TXT`"]
pub struct TXT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TXT_A::_00)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TXT_A::_01)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TXT_A::_10)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TXT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "TX data toggle Inhibit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXI_A {
    #[doc = "0: PID sequencing enabled"]
    _0,
    #[doc = "1: PID sequencing disabled"]
    _1,
}
impl From<TXI_A> for bool {
    #[inline(always)]
    fn from(variant: TXI_A) -> Self {
        match variant {
            TXI_A::_0 => false,
            TXI_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXI`"]
pub type TXI_R = crate::R<bool, TXI_A>;
impl TXI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXI_A {
        match self.bits {
            false => TXI_A::_0,
            true => TXI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXI_A::_1
    }
}
#[doc = "Write proxy for field `TXI`"]
pub struct TXI_W<'a> {
    w: &'a mut W,
}
impl<'a> TXI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PID sequencing enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXI_A::_0)
    }
    #[doc = "PID sequencing disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXI_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Write proxy for field `TXR`"]
pub struct TXR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "TX endpoint Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    #[doc = "0: Disabled"]
    _0,
    #[doc = "1: Enabled"]
    _1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        match variant {
            TXE_A::_0 => false,
            TXE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TXE`"]
pub type TXE_R = crate::R<bool, TXE_A>;
impl TXE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::_0,
            true => TXE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXE_A::_1
    }
}
#[doc = "Write proxy for field `TXE`"]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXE_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RX endpoint Stall"]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX endpoint Data sink"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - RX endpoint Type"]
    #[inline(always)]
    pub fn rxt(&self) -> RXT_R {
        RXT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 5 - RX data toggle Inhibit"]
    #[inline(always)]
    pub fn rxi(&self) -> RXI_R {
        RXI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RX endpoint Enable"]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TX endpoint Stall"]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TX endpoint Data source"]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - TX endpoint Type"]
    #[inline(always)]
    pub fn txt(&self) -> TXT_R {
        TXT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 21 - TX data toggle Inhibit"]
    #[inline(always)]
    pub fn txi(&self) -> TXI_R {
        TXI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TX endpoint Enable"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX endpoint Stall"]
    #[inline(always)]
    pub fn rxs(&mut self) -> RXS_W {
        RXS_W { w: self }
    }
    #[doc = "Bit 1 - RX endpoint Data sink"]
    #[inline(always)]
    pub fn rxd(&mut self) -> RXD_W {
        RXD_W { w: self }
    }
    #[doc = "Bits 2:3 - RX endpoint Type"]
    #[inline(always)]
    pub fn rxt(&mut self) -> RXT_W {
        RXT_W { w: self }
    }
    #[doc = "Bit 5 - RX data toggle Inhibit"]
    #[inline(always)]
    pub fn rxi(&mut self) -> RXI_W {
        RXI_W { w: self }
    }
    #[doc = "Bit 6 - RX data toggle Reset"]
    #[inline(always)]
    pub fn rxr(&mut self) -> RXR_W {
        RXR_W { w: self }
    }
    #[doc = "Bit 7 - RX endpoint Enable"]
    #[inline(always)]
    pub fn rxe(&mut self) -> RXE_W {
        RXE_W { w: self }
    }
    #[doc = "Bit 16 - TX endpoint Stall"]
    #[inline(always)]
    pub fn txs(&mut self) -> TXS_W {
        TXS_W { w: self }
    }
    #[doc = "Bit 17 - TX endpoint Data source"]
    #[inline(always)]
    pub fn txd(&mut self) -> TXD_W {
        TXD_W { w: self }
    }
    #[doc = "Bits 18:19 - TX endpoint Type"]
    #[inline(always)]
    pub fn txt(&mut self) -> TXT_W {
        TXT_W { w: self }
    }
    #[doc = "Bit 21 - TX data toggle Inhibit"]
    #[inline(always)]
    pub fn txi(&mut self) -> TXI_W {
        TXI_W { w: self }
    }
    #[doc = "Bit 22 - TX data toggle Reset"]
    #[inline(always)]
    pub fn txr(&mut self) -> TXR_W {
        TXR_W { w: self }
    }
    #[doc = "Bit 23 - TX endpoint Enable"]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
}
