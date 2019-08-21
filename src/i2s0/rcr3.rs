#[doc = "Reader of register RCR3"]
pub type R = crate::R<u32, super::RCR3>;
#[doc = "Writer for register RCR3"]
pub type W = crate::W<u32, super::RCR3>;
#[doc = "Register RCR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::RCR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDFL`"]
pub type WDFL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDFL`"]
pub struct WDFL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Receive Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCE0_A {
    #[doc = "0: Receive data channel N is disabled."]
    _0,
    #[doc = "1: Receive data channel N is enabled."]
    _1,
}
impl From<RCE0_A> for bool {
    #[inline(always)]
    fn from(variant: RCE0_A) -> Self {
        match variant {
            RCE0_A::_0 => false,
            RCE0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RCE0`"]
pub type RCE0_R = crate::R<bool, RCE0_A>;
impl RCE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCE0_A {
        match self.bits {
            false => RCE0_A::_0,
            true => RCE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCE0_A::_1
    }
}
#[doc = "Write proxy for field `RCE0`"]
pub struct RCE0_W<'a> {
    w: &'a mut W,
}
impl<'a> RCE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive data channel N is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCE0_A::_0)
    }
    #[doc = "Receive data channel N is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCE0_A::_1)
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
#[doc = "Receive Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCE1_A {
    #[doc = "0: Receive data channel N is disabled."]
    _0,
    #[doc = "1: Receive data channel N is enabled."]
    _1,
}
impl From<RCE1_A> for bool {
    #[inline(always)]
    fn from(variant: RCE1_A) -> Self {
        match variant {
            RCE1_A::_0 => false,
            RCE1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RCE1`"]
pub type RCE1_R = crate::R<bool, RCE1_A>;
impl RCE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCE1_A {
        match self.bits {
            false => RCE1_A::_0,
            true => RCE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCE1_A::_1
    }
}
#[doc = "Write proxy for field `RCE1`"]
pub struct RCE1_W<'a> {
    w: &'a mut W,
}
impl<'a> RCE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive data channel N is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCE1_A::_0)
    }
    #[doc = "Receive data channel N is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCE1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Channel FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFR0_AW {
    #[doc = "0: No effect."]
    _0,
    #[doc = "1: Receive data channel N FIFO is reset."]
    _1,
}
impl From<CFR0_AW> for bool {
    #[inline(always)]
    fn from(variant: CFR0_AW) -> Self {
        match variant {
            CFR0_AW::_0 => false,
            CFR0_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `CFR0`"]
pub struct CFR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFR0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFR0_AW::_0)
    }
    #[doc = "Receive data channel N FIFO is reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFR0_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Channel FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFR1_AW {
    #[doc = "0: No effect."]
    _0,
    #[doc = "1: Receive data channel N FIFO is reset."]
    _1,
}
impl From<CFR1_AW> for bool {
    #[inline(always)]
    fn from(variant: CFR1_AW) -> Self {
        match variant {
            CFR1_AW::_0 => false,
            CFR1_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `CFR1`"]
pub struct CFR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFR1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFR1_AW::_0)
    }
    #[doc = "Receive data channel N FIFO is reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFR1_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline(always)]
    pub fn wdfl(&self) -> WDFL_R {
        WDFL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Receive Channel Enable"]
    #[inline(always)]
    pub fn rce0(&self) -> RCE0_R {
        RCE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive Channel Enable"]
    #[inline(always)]
    pub fn rce1(&self) -> RCE1_R {
        RCE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline(always)]
    pub fn wdfl(&mut self) -> WDFL_W {
        WDFL_W { w: self }
    }
    #[doc = "Bit 16 - Receive Channel Enable"]
    #[inline(always)]
    pub fn rce0(&mut self) -> RCE0_W {
        RCE0_W { w: self }
    }
    #[doc = "Bit 17 - Receive Channel Enable"]
    #[inline(always)]
    pub fn rce1(&mut self) -> RCE1_W {
        RCE1_W { w: self }
    }
    #[doc = "Bit 24 - Channel FIFO Reset"]
    #[inline(always)]
    pub fn cfr0(&mut self) -> CFR0_W {
        CFR0_W { w: self }
    }
    #[doc = "Bit 25 - Channel FIFO Reset"]
    #[inline(always)]
    pub fn cfr1(&mut self) -> CFR1_W {
        CFR1_W { w: self }
    }
}
