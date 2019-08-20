#[doc = "Reader of register GPTIMER%sCTL"]
pub type R = crate::R<u32, super::GPTIMERCTL>;
#[doc = "Writer for register GPTIMER%sCTL"]
pub type W = crate::W<u32, super::GPTIMERCTL>;
#[doc = "Register GPTIMER%sCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTIMERCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPTCNT`"]
pub type GPTCNT_R = crate::R<u32, u32>;
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: One shot"]
    _0,
    #[doc = "1: Repeat"]
    _1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::_0 => false,
            MODE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::_0,
            true => MODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODE_A::_1
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One shot"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE_A::_0)
    }
    #[doc = "Repeat"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE_A::_1)
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
#[doc = "Timer Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_AW {
    #[doc = "0: No action"]
    _0,
    #[doc = "1: Load counter value"]
    _1,
}
impl From<RST_AW> for bool {
    #[inline(always)]
    fn from(variant: RST_AW) -> Self {
        match variant {
            RST_AW::_0 => false,
            RST_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RST_AW::_0)
    }
    #[doc = "Load counter value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RST_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Timer Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN_A {
    #[doc = "0: Timer stop"]
    _0,
    #[doc = "1: Timer run"]
    _1,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        match variant {
            RUN_A::_0 => false,
            RUN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RUN`"]
pub type RUN_R = crate::R<bool, RUN_A>;
impl RUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_A {
        match self.bits {
            false => RUN_A::_0,
            true => RUN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RUN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RUN_A::_1
    }
}
#[doc = "Write proxy for field `RUN`"]
pub struct RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer stop"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RUN_A::_0)
    }
    #[doc = "Timer run"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RUN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Timer Count"]
    #[inline(always)]
    pub fn gptcnt(&self) -> GPTCNT_R {
        GPTCNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Timer Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Timer Run"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Timer Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 30 - Timer Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 31 - Timer Run"]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W {
        RUN_W { w: self }
    }
}
