#[doc = "Reader of register BDH"]
pub type R = crate::R<u8, super::BDH>;
#[doc = "Writer for register BDH"]
pub type W = crate::W<u8, super::BDH>;
#[doc = "Register BDH `reset()`'s with value 0"]
impl crate::ResetValue for super::BDH {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SBR`"]
pub type SBR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SBR`"]
pub struct SBR_W<'a> {
    w: &'a mut W,
}
impl<'a> SBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Stop Bit Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBNS_A {
    #[doc = "0: Data frame consists of a single stop bit."]
    _0,
    #[doc = "1: Data frame consists of two stop bits."]
    _1,
}
impl From<SBNS_A> for bool {
    #[inline(always)]
    fn from(variant: SBNS_A) -> Self {
        match variant {
            SBNS_A::_0 => false,
            SBNS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SBNS`"]
pub type SBNS_R = crate::R<bool, SBNS_A>;
impl SBNS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBNS_A {
        match self.bits {
            false => SBNS_A::_0,
            true => SBNS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBNS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBNS_A::_1
    }
}
#[doc = "Write proxy for field `SBNS`"]
pub struct SBNS_W<'a> {
    w: &'a mut W,
}
impl<'a> SBNS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBNS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data frame consists of a single stop bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBNS_A::_0)
    }
    #[doc = "Data frame consists of two stop bits."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBNS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "RxD Input Active Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIE_A {
    #[doc = "0: Hardware interrupts from RXEDGIF disabled using polling."]
    _0,
    #[doc = "1: RXEDGIF interrupt request enabled."]
    _1,
}
impl From<RXEDGIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXEDGIE_A) -> Self {
        match variant {
            RXEDGIE_A::_0 => false,
            RXEDGIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXEDGIE`"]
pub type RXEDGIE_R = crate::R<bool, RXEDGIE_A>;
impl RXEDGIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEDGIE_A {
        match self.bits {
            false => RXEDGIE_A::_0,
            true => RXEDGIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXEDGIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXEDGIE_A::_1
    }
}
#[doc = "Write proxy for field `RXEDGIE`"]
pub struct RXEDGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEDGIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXEDGIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupts from RXEDGIF disabled using polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIE_A::_0)
    }
    #[doc = "RXEDGIF interrupt request enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "LIN Break Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIE_A {
    #[doc = "0: LBKDIF interrupt requests disabled."]
    _0,
    #[doc = "1: LBKDIF interrupt requests enabled."]
    _1,
}
impl From<LBKDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDIE_A) -> Self {
        match variant {
            LBKDIE_A::_0 => false,
            LBKDIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LBKDIE`"]
pub type LBKDIE_R = crate::R<bool, LBKDIE_A>;
impl LBKDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDIE_A {
        match self.bits {
            false => LBKDIE_A::_0,
            true => LBKDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LBKDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LBKDIE_A::_1
    }
}
#[doc = "Write proxy for field `LBKDIE`"]
pub struct LBKDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBKDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LBKDIF interrupt requests disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIE_A::_0)
    }
    #[doc = "LBKDIF interrupt requests enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - UART Baud Rate Bits"]
    #[inline(always)]
    pub fn sbr(&self) -> SBR_R {
        SBR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Stop Bit Number Select"]
    #[inline(always)]
    pub fn sbns(&self) -> SBNS_R {
        SBNS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RxD Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub fn rxedgie(&self) -> RXEDGIE_R {
        RXEDGIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lbkdie(&self) -> LBKDIE_R {
        LBKDIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - UART Baud Rate Bits"]
    #[inline(always)]
    pub fn sbr(&mut self) -> SBR_W {
        SBR_W { w: self }
    }
    #[doc = "Bit 5 - Stop Bit Number Select"]
    #[inline(always)]
    pub fn sbns(&mut self) -> SBNS_W {
        SBNS_W { w: self }
    }
    #[doc = "Bit 6 - RxD Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub fn rxedgie(&mut self) -> RXEDGIE_W {
        RXEDGIE_W { w: self }
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lbkdie(&mut self) -> LBKDIE_W {
        LBKDIE_W { w: self }
    }
}
