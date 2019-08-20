#[doc = "Reader of register TCR3"]
pub type R = crate::R<u32, super::TCR3>;
#[doc = "Writer for register TCR3"]
pub type W = crate::W<u32, super::TCR3>;
#[doc = "Register TCR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR3 {
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
#[doc = "Transmit Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCE0_A {
    #[doc = "0: Transmit data channel N is disabled."]
    _0,
    #[doc = "1: Transmit data channel N is enabled."]
    _1,
}
impl From<TCE0_A> for bool {
    #[inline(always)]
    fn from(variant: TCE0_A) -> Self {
        match variant {
            TCE0_A::_0 => false,
            TCE0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TCE0`"]
pub type TCE0_R = crate::R<bool, TCE0_A>;
impl TCE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCE0_A {
        match self.bits {
            false => TCE0_A::_0,
            true => TCE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCE0_A::_1
    }
}
#[doc = "Write proxy for field `TCE0`"]
pub struct TCE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TCE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit data channel N is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCE0_A::_0)
    }
    #[doc = "Transmit data channel N is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCE0_A::_1)
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
#[doc = "Transmit Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCE1_A {
    #[doc = "0: Transmit data channel N is disabled."]
    _0,
    #[doc = "1: Transmit data channel N is enabled."]
    _1,
}
impl From<TCE1_A> for bool {
    #[inline(always)]
    fn from(variant: TCE1_A) -> Self {
        match variant {
            TCE1_A::_0 => false,
            TCE1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TCE1`"]
pub type TCE1_R = crate::R<bool, TCE1_A>;
impl TCE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCE1_A {
        match self.bits {
            false => TCE1_A::_0,
            true => TCE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCE1_A::_1
    }
}
#[doc = "Write proxy for field `TCE1`"]
pub struct TCE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TCE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit data channel N is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCE1_A::_0)
    }
    #[doc = "Transmit data channel N is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCE1_A::_1)
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
    #[doc = "1: Transmit data channel N FIFO is reset."]
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
    #[doc = "Transmit data channel N FIFO is reset."]
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
    #[doc = "1: Transmit data channel N FIFO is reset."]
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
    #[doc = "Transmit data channel N FIFO is reset."]
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
    #[doc = "Bit 16 - Transmit Channel Enable"]
    #[inline(always)]
    pub fn tce0(&self) -> TCE0_R {
        TCE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmit Channel Enable"]
    #[inline(always)]
    pub fn tce1(&self) -> TCE1_R {
        TCE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline(always)]
    pub fn wdfl(&mut self) -> WDFL_W {
        WDFL_W { w: self }
    }
    #[doc = "Bit 16 - Transmit Channel Enable"]
    #[inline(always)]
    pub fn tce0(&mut self) -> TCE0_W {
        TCE0_W { w: self }
    }
    #[doc = "Bit 17 - Transmit Channel Enable"]
    #[inline(always)]
    pub fn tce1(&mut self) -> TCE1_W {
        TCE1_W { w: self }
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
