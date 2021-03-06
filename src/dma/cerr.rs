#[doc = "Writer for register CERR"]
pub type W = crate::W<u8, super::CERR>;
#[doc = "Register CERR `reset()`'s with value 0"]
impl crate::ResetValue for super::CERR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CERR`"]
pub struct CERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
#[doc = "Clear All Error Indicators\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAEI_AW {
    #[doc = "0: Clear only the ERR bit specified in the CERR field"]
    _0,
    #[doc = "1: Clear all bits in ERR"]
    _1,
}
impl From<CAEI_AW> for bool {
    #[inline(always)]
    fn from(variant: CAEI_AW) -> Self {
        match variant {
            CAEI_AW::_0 => false,
            CAEI_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `CAEI`"]
pub struct CAEI_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAEI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear only the ERR bit specified in the CERR field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAEI_AW::_0)
    }
    #[doc = "Clear all bits in ERR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAEI_AW::_1)
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
#[doc = "No Op enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOP_AW {
    #[doc = "0: Normal operation"]
    _0,
    #[doc = "1: No operation, ignore the other bits in this register"]
    _1,
}
impl From<NOP_AW> for bool {
    #[inline(always)]
    fn from(variant: NOP_AW) -> Self {
        match variant {
            NOP_AW::_0 => false,
            NOP_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `NOP`"]
pub struct NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOP_AW::_0)
    }
    #[doc = "No operation, ignore the other bits in this register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOP_AW::_1)
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
impl W {
    #[doc = "Bits 0:4 - Clear Error Indicator"]
    #[inline(always)]
    pub fn cerr(&mut self) -> CERR_W {
        CERR_W { w: self }
    }
    #[doc = "Bit 6 - Clear All Error Indicators"]
    #[inline(always)]
    pub fn caei(&mut self) -> CAEI_W {
        CAEI_W { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&mut self) -> NOP_W {
        NOP_W { w: self }
    }
}
