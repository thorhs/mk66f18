#[doc = "Reader of register CR"]
pub type R = crate::R<u8, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u8, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Oscillator 16 pF Capacitor Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC16P_A {
    #[doc = "0: Disable the selection."]
    _0,
    #[doc = "1: Add 16 pF capacitor to the oscillator load."]
    _1,
}
impl From<SC16P_A> for bool {
    #[inline(always)]
    fn from(variant: SC16P_A) -> Self {
        match variant {
            SC16P_A::_0 => false,
            SC16P_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SC16P`"]
pub type SC16P_R = crate::R<bool, SC16P_A>;
impl SC16P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC16P_A {
        match self.bits {
            false => SC16P_A::_0,
            true => SC16P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SC16P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SC16P_A::_1
    }
}
#[doc = "Write proxy for field `SC16P`"]
pub struct SC16P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC16P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC16P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC16P_A::_0)
    }
    #[doc = "Add 16 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC16P_A::_1)
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
#[doc = "Oscillator 8 pF Capacitor Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC8P_A {
    #[doc = "0: Disable the selection."]
    _0,
    #[doc = "1: Add 8 pF capacitor to the oscillator load."]
    _1,
}
impl From<SC8P_A> for bool {
    #[inline(always)]
    fn from(variant: SC8P_A) -> Self {
        match variant {
            SC8P_A::_0 => false,
            SC8P_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SC8P`"]
pub type SC8P_R = crate::R<bool, SC8P_A>;
impl SC8P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC8P_A {
        match self.bits {
            false => SC8P_A::_0,
            true => SC8P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SC8P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SC8P_A::_1
    }
}
#[doc = "Write proxy for field `SC8P`"]
pub struct SC8P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC8P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC8P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC8P_A::_0)
    }
    #[doc = "Add 8 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC8P_A::_1)
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
#[doc = "Oscillator 4 pF Capacitor Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC4P_A {
    #[doc = "0: Disable the selection."]
    _0,
    #[doc = "1: Add 4 pF capacitor to the oscillator load."]
    _1,
}
impl From<SC4P_A> for bool {
    #[inline(always)]
    fn from(variant: SC4P_A) -> Self {
        match variant {
            SC4P_A::_0 => false,
            SC4P_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SC4P`"]
pub type SC4P_R = crate::R<bool, SC4P_A>;
impl SC4P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC4P_A {
        match self.bits {
            false => SC4P_A::_0,
            true => SC4P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SC4P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SC4P_A::_1
    }
}
#[doc = "Write proxy for field `SC4P`"]
pub struct SC4P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC4P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC4P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC4P_A::_0)
    }
    #[doc = "Add 4 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC4P_A::_1)
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
#[doc = "Oscillator 2 pF Capacitor Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SC2P_A {
    #[doc = "0: Disable the selection."]
    _0,
    #[doc = "1: Add 2 pF capacitor to the oscillator load."]
    _1,
}
impl From<SC2P_A> for bool {
    #[inline(always)]
    fn from(variant: SC2P_A) -> Self {
        match variant {
            SC2P_A::_0 => false,
            SC2P_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SC2P`"]
pub type SC2P_R = crate::R<bool, SC2P_A>;
impl SC2P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC2P_A {
        match self.bits {
            false => SC2P_A::_0,
            true => SC2P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SC2P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SC2P_A::_1
    }
}
#[doc = "Write proxy for field `SC2P`"]
pub struct SC2P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC2P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SC2P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC2P_A::_0)
    }
    #[doc = "Add 2 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC2P_A::_1)
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
#[doc = "External Reference Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EREFSTEN_A {
    #[doc = "0: External reference clock is disabled in Stop mode."]
    _0,
    #[doc = "1: External reference clock stays enabled in Stop mode if ERCLKEN is set before entering Stop mode."]
    _1,
}
impl From<EREFSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: EREFSTEN_A) -> Self {
        match variant {
            EREFSTEN_A::_0 => false,
            EREFSTEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EREFSTEN`"]
pub type EREFSTEN_R = crate::R<bool, EREFSTEN_A>;
impl EREFSTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EREFSTEN_A {
        match self.bits {
            false => EREFSTEN_A::_0,
            true => EREFSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EREFSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EREFSTEN_A::_1
    }
}
#[doc = "Write proxy for field `EREFSTEN`"]
pub struct EREFSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EREFSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EREFSTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External reference clock is disabled in Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EREFSTEN_A::_0)
    }
    #[doc = "External reference clock stays enabled in Stop mode if ERCLKEN is set before entering Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EREFSTEN_A::_1)
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
#[doc = "External Reference Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERCLKEN_A {
    #[doc = "0: External reference clock is inactive."]
    _0,
    #[doc = "1: External reference clock is enabled."]
    _1,
}
impl From<ERCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: ERCLKEN_A) -> Self {
        match variant {
            ERCLKEN_A::_0 => false,
            ERCLKEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ERCLKEN`"]
pub type ERCLKEN_R = crate::R<bool, ERCLKEN_A>;
impl ERCLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERCLKEN_A {
        match self.bits {
            false => ERCLKEN_A::_0,
            true => ERCLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERCLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERCLKEN_A::_1
    }
}
#[doc = "Write proxy for field `ERCLKEN`"]
pub struct ERCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERCLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External reference clock is inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERCLKEN_A::_0)
    }
    #[doc = "External reference clock is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERCLKEN_A::_1)
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
    #[doc = "Bit 0 - Oscillator 16 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc16p(&self) -> SC16P_R {
        SC16P_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Oscillator 8 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc8p(&self) -> SC8P_R {
        SC8P_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Oscillator 4 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc4p(&self) -> SC4P_R {
        SC4P_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Oscillator 2 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc2p(&self) -> SC2P_R {
        SC2P_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Reference Stop Enable"]
    #[inline(always)]
    pub fn erefsten(&self) -> EREFSTEN_R {
        EREFSTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Reference Enable"]
    #[inline(always)]
    pub fn erclken(&self) -> ERCLKEN_R {
        ERCLKEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillator 16 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc16p(&mut self) -> SC16P_W {
        SC16P_W { w: self }
    }
    #[doc = "Bit 1 - Oscillator 8 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc8p(&mut self) -> SC8P_W {
        SC8P_W { w: self }
    }
    #[doc = "Bit 2 - Oscillator 4 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc4p(&mut self) -> SC4P_W {
        SC4P_W { w: self }
    }
    #[doc = "Bit 3 - Oscillator 2 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc2p(&mut self) -> SC2P_W {
        SC2P_W { w: self }
    }
    #[doc = "Bit 5 - External Reference Stop Enable"]
    #[inline(always)]
    pub fn erefsten(&mut self) -> EREFSTEN_W {
        EREFSTEN_W { w: self }
    }
    #[doc = "Bit 7 - External Reference Enable"]
    #[inline(always)]
    pub fn erclken(&mut self) -> ERCLKEN_W {
        ERCLKEN_W { w: self }
    }
}
