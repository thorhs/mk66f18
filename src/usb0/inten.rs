#[doc = "Reader of register INTEN"]
pub type R = crate::R<u8, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u8, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USBRST Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRSTEN_A {
    #[doc = "0: Disables the USBRST interrupt."]
    _0,
    #[doc = "1: Enables the USBRST interrupt."]
    _1,
}
impl From<USBRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBRSTEN_A) -> Self {
        match variant {
            USBRSTEN_A::_0 => false,
            USBRSTEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `USBRSTEN`"]
pub type USBRSTEN_R = crate::R<bool, USBRSTEN_A>;
impl USBRSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRSTEN_A {
        match self.bits {
            false => USBRSTEN_A::_0,
            true => USBRSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBRSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBRSTEN_A::_1
    }
}
#[doc = "Write proxy for field `USBRSTEN`"]
pub struct USBRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the USBRST interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBRSTEN_A::_0)
    }
    #[doc = "Enables the USBRST interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBRSTEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "ERROR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROREN_A {
    #[doc = "0: Disables the ERROR interrupt."]
    _0,
    #[doc = "1: Enables the ERROR interrupt."]
    _1,
}
impl From<ERROREN_A> for bool {
    #[inline(always)]
    fn from(variant: ERROREN_A) -> Self {
        match variant {
            ERROREN_A::_0 => false,
            ERROREN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERROREN`"]
pub type ERROREN_R = crate::R<bool, ERROREN_A>;
impl ERROREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROREN_A {
        match self.bits {
            false => ERROREN_A::_0,
            true => ERROREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERROREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERROREN_A::_1
    }
}
#[doc = "Write proxy for field `ERROREN`"]
pub struct ERROREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the ERROR interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERROREN_A::_0)
    }
    #[doc = "Enables the ERROR interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERROREN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "SOFTOK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTOKEN_A {
    #[doc = "0: Disbles the SOFTOK interrupt."]
    _0,
    #[doc = "1: Enables the SOFTOK interrupt."]
    _1,
}
impl From<SOFTOKEN_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTOKEN_A) -> Self {
        match variant {
            SOFTOKEN_A::_0 => false,
            SOFTOKEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SOFTOKEN`"]
pub type SOFTOKEN_R = crate::R<bool, SOFTOKEN_A>;
impl SOFTOKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTOKEN_A {
        match self.bits {
            false => SOFTOKEN_A::_0,
            true => SOFTOKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOFTOKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOFTOKEN_A::_1
    }
}
#[doc = "Write proxy for field `SOFTOKEN`"]
pub struct SOFTOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTOKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFTOKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disbles the SOFTOK interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFTOKEN_A::_0)
    }
    #[doc = "Enables the SOFTOK interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFTOKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "TOKDNE Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOKDNEEN_A {
    #[doc = "0: Disables the TOKDNE interrupt."]
    _0,
    #[doc = "1: Enables the TOKDNE interrupt."]
    _1,
}
impl From<TOKDNEEN_A> for bool {
    #[inline(always)]
    fn from(variant: TOKDNEEN_A) -> Self {
        match variant {
            TOKDNEEN_A::_0 => false,
            TOKDNEEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TOKDNEEN`"]
pub type TOKDNEEN_R = crate::R<bool, TOKDNEEN_A>;
impl TOKDNEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOKDNEEN_A {
        match self.bits {
            false => TOKDNEEN_A::_0,
            true => TOKDNEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOKDNEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOKDNEEN_A::_1
    }
}
#[doc = "Write proxy for field `TOKDNEEN`"]
pub struct TOKDNEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOKDNEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOKDNEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the TOKDNE interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOKDNEEN_A::_0)
    }
    #[doc = "Enables the TOKDNE interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOKDNEEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "SLEEP Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPEN_A {
    #[doc = "0: Disables the SLEEP interrupt."]
    _0,
    #[doc = "1: Enables the SLEEP interrupt."]
    _1,
}
impl From<SLEEPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPEN_A) -> Self {
        match variant {
            SLEEPEN_A::_0 => false,
            SLEEPEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SLEEPEN`"]
pub type SLEEPEN_R = crate::R<bool, SLEEPEN_A>;
impl SLEEPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPEN_A {
        match self.bits {
            false => SLEEPEN_A::_0,
            true => SLEEPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLEEPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLEEPEN_A::_1
    }
}
#[doc = "Write proxy for field `SLEEPEN`"]
pub struct SLEEPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the SLEEP interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEEPEN_A::_0)
    }
    #[doc = "Enables the SLEEP interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEEPEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "RESUME Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESUMEEN_A {
    #[doc = "0: Disables the RESUME interrupt."]
    _0,
    #[doc = "1: Enables the RESUME interrupt."]
    _1,
}
impl From<RESUMEEN_A> for bool {
    #[inline(always)]
    fn from(variant: RESUMEEN_A) -> Self {
        match variant {
            RESUMEEN_A::_0 => false,
            RESUMEEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RESUMEEN`"]
pub type RESUMEEN_R = crate::R<bool, RESUMEEN_A>;
impl RESUMEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESUMEEN_A {
        match self.bits {
            false => RESUMEEN_A::_0,
            true => RESUMEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESUMEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESUMEEN_A::_1
    }
}
#[doc = "Write proxy for field `RESUMEEN`"]
pub struct RESUMEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESUMEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the RESUME interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESUMEEN_A::_0)
    }
    #[doc = "Enables the RESUME interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESUMEEN_A::_1)
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
#[doc = "ATTACH Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATTACHEN_A {
    #[doc = "0: Disables the ATTACH interrupt."]
    _0,
    #[doc = "1: Enables the ATTACH interrupt."]
    _1,
}
impl From<ATTACHEN_A> for bool {
    #[inline(always)]
    fn from(variant: ATTACHEN_A) -> Self {
        match variant {
            ATTACHEN_A::_0 => false,
            ATTACHEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ATTACHEN`"]
pub type ATTACHEN_R = crate::R<bool, ATTACHEN_A>;
impl ATTACHEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATTACHEN_A {
        match self.bits {
            false => ATTACHEN_A::_0,
            true => ATTACHEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATTACHEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATTACHEN_A::_1
    }
}
#[doc = "Write proxy for field `ATTACHEN`"]
pub struct ATTACHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTACHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATTACHEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables the ATTACH interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATTACHEN_A::_0)
    }
    #[doc = "Enables the ATTACH interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATTACHEN_A::_1)
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
#[doc = "STALL Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALLEN_A {
    #[doc = "0: Diasbles the STALL interrupt."]
    _0,
    #[doc = "1: Enables the STALL interrupt."]
    _1,
}
impl From<STALLEN_A> for bool {
    #[inline(always)]
    fn from(variant: STALLEN_A) -> Self {
        match variant {
            STALLEN_A::_0 => false,
            STALLEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `STALLEN`"]
pub type STALLEN_R = crate::R<bool, STALLEN_A>;
impl STALLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALLEN_A {
        match self.bits {
            false => STALLEN_A::_0,
            true => STALLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STALLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STALLEN_A::_1
    }
}
#[doc = "Write proxy for field `STALLEN`"]
pub struct STALLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Diasbles the STALL interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALLEN_A::_0)
    }
    #[doc = "Enables the STALL interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALLEN_A::_1)
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
    #[doc = "Bit 0 - USBRST Interrupt Enable"]
    #[inline(always)]
    pub fn usbrsten(&self) -> USBRSTEN_R {
        USBRSTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ERROR Interrupt Enable"]
    #[inline(always)]
    pub fn erroren(&self) -> ERROREN_R {
        ERROREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SOFTOK Interrupt Enable"]
    #[inline(always)]
    pub fn softoken(&self) -> SOFTOKEN_R {
        SOFTOKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TOKDNE Interrupt Enable"]
    #[inline(always)]
    pub fn tokdneen(&self) -> TOKDNEEN_R {
        TOKDNEEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SLEEP Interrupt Enable"]
    #[inline(always)]
    pub fn sleepen(&self) -> SLEEPEN_R {
        SLEEPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RESUME Interrupt Enable"]
    #[inline(always)]
    pub fn resumeen(&self) -> RESUMEEN_R {
        RESUMEEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ATTACH Interrupt Enable"]
    #[inline(always)]
    pub fn attachen(&self) -> ATTACHEN_R {
        ATTACHEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - STALL Interrupt Enable"]
    #[inline(always)]
    pub fn stallen(&self) -> STALLEN_R {
        STALLEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBRST Interrupt Enable"]
    #[inline(always)]
    pub fn usbrsten(&mut self) -> USBRSTEN_W {
        USBRSTEN_W { w: self }
    }
    #[doc = "Bit 1 - ERROR Interrupt Enable"]
    #[inline(always)]
    pub fn erroren(&mut self) -> ERROREN_W {
        ERROREN_W { w: self }
    }
    #[doc = "Bit 2 - SOFTOK Interrupt Enable"]
    #[inline(always)]
    pub fn softoken(&mut self) -> SOFTOKEN_W {
        SOFTOKEN_W { w: self }
    }
    #[doc = "Bit 3 - TOKDNE Interrupt Enable"]
    #[inline(always)]
    pub fn tokdneen(&mut self) -> TOKDNEEN_W {
        TOKDNEEN_W { w: self }
    }
    #[doc = "Bit 4 - SLEEP Interrupt Enable"]
    #[inline(always)]
    pub fn sleepen(&mut self) -> SLEEPEN_W {
        SLEEPEN_W { w: self }
    }
    #[doc = "Bit 5 - RESUME Interrupt Enable"]
    #[inline(always)]
    pub fn resumeen(&mut self) -> RESUMEEN_W {
        RESUMEEN_W { w: self }
    }
    #[doc = "Bit 6 - ATTACH Interrupt Enable"]
    #[inline(always)]
    pub fn attachen(&mut self) -> ATTACHEN_W {
        ATTACHEN_W { w: self }
    }
    #[doc = "Bit 7 - STALL Interrupt Enable"]
    #[inline(always)]
    pub fn stallen(&mut self) -> STALLEN_W {
        STALLEN_W { w: self }
    }
}
