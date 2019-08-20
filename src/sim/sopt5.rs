#[doc = "Reader of register SOPT5"]
pub type R = crate::R<u32, super::SOPT5>;
#[doc = "Writer for register SOPT5"]
pub type W = crate::W<u32, super::SOPT5>;
#[doc = "Register SOPT5 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "UART 0 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0TXSRC_A {
    #[doc = "0: UART0_TX pin"]
    _00,
    #[doc = "1: UART0_TX pin modulated with FTM1 channel 0 output"]
    _01,
    #[doc = "2: UART0_TX pin modulated with FTM2 channel 0 output"]
    _10,
}
impl From<UART0TXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART0TXSRC_A) -> Self {
        match variant {
            UART0TXSRC_A::_00 => 0,
            UART0TXSRC_A::_01 => 1,
            UART0TXSRC_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `UART0TXSRC`"]
pub type UART0TXSRC_R = crate::R<u8, UART0TXSRC_A>;
impl UART0TXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UART0TXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UART0TXSRC_A::_00),
            1 => Val(UART0TXSRC_A::_01),
            2 => Val(UART0TXSRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == UART0TXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == UART0TXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == UART0TXSRC_A::_10
    }
}
#[doc = "Write proxy for field `UART0TXSRC`"]
pub struct UART0TXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0TXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0TXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "UART0_TX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART0TXSRC_A::_00)
    }
    #[doc = "UART0_TX pin modulated with FTM1 channel 0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART0TXSRC_A::_01)
    }
    #[doc = "UART0_TX pin modulated with FTM2 channel 0 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART0TXSRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "UART 0 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0RXSRC_A {
    #[doc = "0: UART0_RX pin"]
    _00,
    #[doc = "1: CMP0"]
    _01,
    #[doc = "2: CMP1"]
    _10,
}
impl From<UART0RXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART0RXSRC_A) -> Self {
        match variant {
            UART0RXSRC_A::_00 => 0,
            UART0RXSRC_A::_01 => 1,
            UART0RXSRC_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `UART0RXSRC`"]
pub type UART0RXSRC_R = crate::R<u8, UART0RXSRC_A>;
impl UART0RXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UART0RXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UART0RXSRC_A::_00),
            1 => Val(UART0RXSRC_A::_01),
            2 => Val(UART0RXSRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == UART0RXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == UART0RXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == UART0RXSRC_A::_10
    }
}
#[doc = "Write proxy for field `UART0RXSRC`"]
pub struct UART0RXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0RXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0RXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "UART0_RX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART0RXSRC_A::_00)
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART0RXSRC_A::_01)
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART0RXSRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "UART 1 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1TXSRC_A {
    #[doc = "0: UART1_TX pin"]
    _00,
    #[doc = "1: UART1_TX pin modulated with FTM1 channel 0 output"]
    _01,
    #[doc = "2: UART1_TX pin modulated with FTM2 channel 0 output"]
    _10,
}
impl From<UART1TXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART1TXSRC_A) -> Self {
        match variant {
            UART1TXSRC_A::_00 => 0,
            UART1TXSRC_A::_01 => 1,
            UART1TXSRC_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `UART1TXSRC`"]
pub type UART1TXSRC_R = crate::R<u8, UART1TXSRC_A>;
impl UART1TXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UART1TXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UART1TXSRC_A::_00),
            1 => Val(UART1TXSRC_A::_01),
            2 => Val(UART1TXSRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == UART1TXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == UART1TXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == UART1TXSRC_A::_10
    }
}
#[doc = "Write proxy for field `UART1TXSRC`"]
pub struct UART1TXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1TXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1TXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "UART1_TX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART1TXSRC_A::_00)
    }
    #[doc = "UART1_TX pin modulated with FTM1 channel 0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART1TXSRC_A::_01)
    }
    #[doc = "UART1_TX pin modulated with FTM2 channel 0 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART1TXSRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "UART 1 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1RXSRC_A {
    #[doc = "0: UART1_RX pin"]
    _00,
    #[doc = "1: CMP0"]
    _01,
    #[doc = "2: CMP1"]
    _10,
}
impl From<UART1RXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART1RXSRC_A) -> Self {
        match variant {
            UART1RXSRC_A::_00 => 0,
            UART1RXSRC_A::_01 => 1,
            UART1RXSRC_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `UART1RXSRC`"]
pub type UART1RXSRC_R = crate::R<u8, UART1RXSRC_A>;
impl UART1RXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UART1RXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UART1RXSRC_A::_00),
            1 => Val(UART1RXSRC_A::_01),
            2 => Val(UART1RXSRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == UART1RXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == UART1RXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == UART1RXSRC_A::_10
    }
}
#[doc = "Write proxy for field `UART1RXSRC`"]
pub struct UART1RXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1RXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1RXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "UART1_RX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(UART1RXSRC_A::_00)
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(UART1RXSRC_A::_01)
    }
    #[doc = "CMP1"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(UART1RXSRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "LPUART0 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART0TXSRC_A {
    #[doc = "0: LPUART0_TX pin"]
    _00,
    #[doc = "1: LPUART0_TX pin modulated with TPM1 channel 0 output"]
    _01,
    #[doc = "2: LPUART0_TX pin modulated with TPM2 channel 0 output"]
    _10,
}
impl From<LPUART0TXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART0TXSRC_A) -> Self {
        match variant {
            LPUART0TXSRC_A::_00 => 0,
            LPUART0TXSRC_A::_01 => 1,
            LPUART0TXSRC_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `LPUART0TXSRC`"]
pub type LPUART0TXSRC_R = crate::R<u8, LPUART0TXSRC_A>;
impl LPUART0TXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPUART0TXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPUART0TXSRC_A::_00),
            1 => Val(LPUART0TXSRC_A::_01),
            2 => Val(LPUART0TXSRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPUART0TXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPUART0TXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LPUART0TXSRC_A::_10
    }
}
#[doc = "Write proxy for field `LPUART0TXSRC`"]
pub struct LPUART0TXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART0TXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART0TXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPUART0_TX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART0TXSRC_A::_00)
    }
    #[doc = "LPUART0_TX pin modulated with TPM1 channel 0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART0TXSRC_A::_01)
    }
    #[doc = "LPUART0_TX pin modulated with TPM2 channel 0 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUART0TXSRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "LPUART0 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART0RXSRC_A {
    #[doc = "0: LPUART0_RX pin"]
    _00,
    #[doc = "1: CMP0 output"]
    _01,
    #[doc = "2: CMP1 output"]
    _10,
}
impl From<LPUART0RXSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART0RXSRC_A) -> Self {
        match variant {
            LPUART0RXSRC_A::_00 => 0,
            LPUART0RXSRC_A::_01 => 1,
            LPUART0RXSRC_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `LPUART0RXSRC`"]
pub type LPUART0RXSRC_R = crate::R<u8, LPUART0RXSRC_A>;
impl LPUART0RXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPUART0RXSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPUART0RXSRC_A::_00),
            1 => Val(LPUART0RXSRC_A::_01),
            2 => Val(LPUART0RXSRC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPUART0RXSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPUART0RXSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LPUART0RXSRC_A::_10
    }
}
#[doc = "Write proxy for field `LPUART0RXSRC`"]
pub struct LPUART0RXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART0RXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART0RXSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPUART0_RX pin"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPUART0RXSRC_A::_00)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPUART0RXSRC_A::_01)
    }
    #[doc = "CMP1 output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPUART0RXSRC_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - UART 0 transmit data source select"]
    #[inline(always)]
    pub fn uart0txsrc(&self) -> UART0TXSRC_R {
        UART0TXSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - UART 0 receive data source select"]
    #[inline(always)]
    pub fn uart0rxsrc(&self) -> UART0RXSRC_R {
        UART0RXSRC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - UART 1 transmit data source select"]
    #[inline(always)]
    pub fn uart1txsrc(&self) -> UART1TXSRC_R {
        UART1TXSRC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - UART 1 receive data source select"]
    #[inline(always)]
    pub fn uart1rxsrc(&self) -> UART1RXSRC_R {
        UART1RXSRC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - LPUART0 transmit data source select"]
    #[inline(always)]
    pub fn lpuart0txsrc(&self) -> LPUART0TXSRC_R {
        LPUART0TXSRC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - LPUART0 receive data source select"]
    #[inline(always)]
    pub fn lpuart0rxsrc(&self) -> LPUART0RXSRC_R {
        LPUART0RXSRC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART 0 transmit data source select"]
    #[inline(always)]
    pub fn uart0txsrc(&mut self) -> UART0TXSRC_W {
        UART0TXSRC_W { w: self }
    }
    #[doc = "Bits 2:3 - UART 0 receive data source select"]
    #[inline(always)]
    pub fn uart0rxsrc(&mut self) -> UART0RXSRC_W {
        UART0RXSRC_W { w: self }
    }
    #[doc = "Bits 4:5 - UART 1 transmit data source select"]
    #[inline(always)]
    pub fn uart1txsrc(&mut self) -> UART1TXSRC_W {
        UART1TXSRC_W { w: self }
    }
    #[doc = "Bits 6:7 - UART 1 receive data source select"]
    #[inline(always)]
    pub fn uart1rxsrc(&mut self) -> UART1RXSRC_W {
        UART1RXSRC_W { w: self }
    }
    #[doc = "Bits 16:17 - LPUART0 transmit data source select"]
    #[inline(always)]
    pub fn lpuart0txsrc(&mut self) -> LPUART0TXSRC_W {
        LPUART0TXSRC_W { w: self }
    }
    #[doc = "Bits 18:19 - LPUART0 receive data source select"]
    #[inline(always)]
    pub fn lpuart0rxsrc(&mut self) -> LPUART0RXSRC_W {
        LPUART0RXSRC_W { w: self }
    }
}
