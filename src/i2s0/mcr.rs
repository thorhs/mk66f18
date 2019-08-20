#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MCLK Input Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MICS_A {
    #[doc = "0: MCLK divider input clock 0 is selected."]
    _00,
    #[doc = "1: MCLK divider input clock 1 is selected."]
    _01,
    #[doc = "2: MCLK divider input clock 2 is selected."]
    _10,
    #[doc = "3: MCLK divider input clock 3 is selected."]
    _11,
}
impl From<MICS_A> for u8 {
    #[inline(always)]
    fn from(variant: MICS_A) -> Self {
        match variant {
            MICS_A::_00 => 0,
            MICS_A::_01 => 1,
            MICS_A::_10 => 2,
            MICS_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `MICS`"]
pub type MICS_R = crate::R<u8, MICS_A>;
impl MICS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MICS_A {
        match self.bits {
            0 => MICS_A::_00,
            1 => MICS_A::_01,
            2 => MICS_A::_10,
            3 => MICS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MICS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MICS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MICS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MICS_A::_11
    }
}
#[doc = "Write proxy for field `MICS`"]
pub struct MICS_W<'a> {
    w: &'a mut W,
}
impl<'a> MICS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MICS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "MCLK divider input clock 0 is selected."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MICS_A::_00)
    }
    #[doc = "MCLK divider input clock 1 is selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MICS_A::_01)
    }
    #[doc = "MCLK divider input clock 2 is selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MICS_A::_10)
    }
    #[doc = "MCLK divider input clock 3 is selected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MICS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "MCLK Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOE_A {
    #[doc = "0: MCLK signal pin is configured as an input that bypasses the MCLK divider."]
    _0,
    #[doc = "1: MCLK signal pin is configured as an output from the MCLK divider and the MCLK divider is enabled."]
    _1,
}
impl From<MOE_A> for bool {
    #[inline(always)]
    fn from(variant: MOE_A) -> Self {
        match variant {
            MOE_A::_0 => false,
            MOE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MOE`"]
pub type MOE_R = crate::R<bool, MOE_A>;
impl MOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOE_A {
        match self.bits {
            false => MOE_A::_0,
            true => MOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOE_A::_1
    }
}
#[doc = "Write proxy for field `MOE`"]
pub struct MOE_W<'a> {
    w: &'a mut W,
}
impl<'a> MOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCLK signal pin is configured as an input that bypasses the MCLK divider."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MOE_A::_0)
    }
    #[doc = "MCLK signal pin is configured as an output from the MCLK divider and the MCLK divider is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MOE_A::_1)
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
#[doc = "Divider Update Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUF_A {
    #[doc = "0: MCLK divider ratio is not being updated currently."]
    _0,
    #[doc = "1: MCLK divider ratio is updating on-the-fly. Further updates to the MCLK divider ratio are blocked while this flag remains set."]
    _1,
}
impl From<DUF_A> for bool {
    #[inline(always)]
    fn from(variant: DUF_A) -> Self {
        match variant {
            DUF_A::_0 => false,
            DUF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DUF`"]
pub type DUF_R = crate::R<bool, DUF_A>;
impl DUF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUF_A {
        match self.bits {
            false => DUF_A::_0,
            true => DUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DUF_A::_1
    }
}
impl R {
    #[doc = "Bits 24:25 - MCLK Input Clock Select"]
    #[inline(always)]
    pub fn mics(&self) -> MICS_R {
        MICS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 30 - MCLK Output Enable"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Divider Update Flag"]
    #[inline(always)]
    pub fn duf(&self) -> DUF_R {
        DUF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - MCLK Input Clock Select"]
    #[inline(always)]
    pub fn mics(&mut self) -> MICS_W {
        MICS_W { w: self }
    }
    #[doc = "Bit 30 - MCLK Output Enable"]
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W {
        MOE_W { w: self }
    }
}
