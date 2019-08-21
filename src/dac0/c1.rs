#[doc = "Reader of register C1"]
pub type R = crate::R<u8, super::C1>;
#[doc = "Writer for register C1"]
pub type W = crate::W<u8, super::C1>;
#[doc = "Register C1 `reset()`'s with value 0"]
impl crate::ResetValue for super::C1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DAC Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFEN_A {
    #[doc = "0: Buffer read pointer is disabled. The converted data is always the first word of the buffer."]
    _0,
    #[doc = "1: Buffer read pointer is enabled. The converted data is the word that the read pointer points to. It means converted data can be from any word of the buffer."]
    _1,
}
impl From<DACBFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFEN_A) -> Self {
        match variant {
            DACBFEN_A::_0 => false,
            DACBFEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DACBFEN`"]
pub type DACBFEN_R = crate::R<bool, DACBFEN_A>;
impl DACBFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFEN_A {
        match self.bits {
            false => DACBFEN_A::_0,
            true => DACBFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBFEN_A::_1
    }
}
#[doc = "Write proxy for field `DACBFEN`"]
pub struct DACBFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Buffer read pointer is disabled. The converted data is always the first word of the buffer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFEN_A::_0)
    }
    #[doc = "Buffer read pointer is enabled. The converted data is the word that the read pointer points to. It means converted data can be from any word of the buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFEN_A::_1)
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
#[doc = "DAC Buffer Work Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFMD_A {
    #[doc = "0: Normal mode"]
    _00,
    #[doc = "1: Swing mode"]
    _01,
    #[doc = "2: One-Time Scan mode"]
    _10,
}
impl From<DACBFMD_A> for u8 {
    #[inline(always)]
    fn from(variant: DACBFMD_A) -> Self {
        match variant {
            DACBFMD_A::_00 => 0,
            DACBFMD_A::_01 => 1,
            DACBFMD_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `DACBFMD`"]
pub type DACBFMD_R = crate::R<u8, DACBFMD_A>;
impl DACBFMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DACBFMD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DACBFMD_A::_00),
            1 => Val(DACBFMD_A::_01),
            2 => Val(DACBFMD_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DACBFMD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DACBFMD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DACBFMD_A::_10
    }
}
#[doc = "Write proxy for field `DACBFMD`"]
pub struct DACBFMD_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBFMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DACBFMD_A::_00)
    }
    #[doc = "Swing mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DACBFMD_A::_01)
    }
    #[doc = "One-Time Scan mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DACBFMD_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u8) & 0x03) << 1);
        self.w
    }
}
#[doc = "DAC Buffer Watermark Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACBFWM_A {
    #[doc = "0: 1 word"]
    _00,
    #[doc = "1: 2 words"]
    _01,
    #[doc = "2: 3 words"]
    _10,
    #[doc = "3: 4 words"]
    _11,
}
impl From<DACBFWM_A> for u8 {
    #[inline(always)]
    fn from(variant: DACBFWM_A) -> Self {
        match variant {
            DACBFWM_A::_00 => 0,
            DACBFWM_A::_01 => 1,
            DACBFWM_A::_10 => 2,
            DACBFWM_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `DACBFWM`"]
pub type DACBFWM_R = crate::R<u8, DACBFWM_A>;
impl DACBFWM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFWM_A {
        match self.bits {
            0 => DACBFWM_A::_00,
            1 => DACBFWM_A::_01,
            2 => DACBFWM_A::_10,
            3 => DACBFWM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DACBFWM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DACBFWM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DACBFWM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DACBFWM_A::_11
    }
}
#[doc = "Write proxy for field `DACBFWM`"]
pub struct DACBFWM_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFWM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACBFWM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 word"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DACBFWM_A::_00)
    }
    #[doc = "2 words"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DACBFWM_A::_01)
    }
    #[doc = "3 words"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DACBFWM_A::_10)
    }
    #[doc = "4 words"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DACBFWM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u8) & 0x03) << 3);
        self.w
    }
}
#[doc = "DMA Enable Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA is disabled."]
    _0,
    #[doc = "1: DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time."]
    _1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        match variant {
            DMAEN_A::_0 => false,
            DMAEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEN_A::_1
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
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
    #[doc = "Bit 0 - DAC Buffer Enable"]
    #[inline(always)]
    pub fn dacbfen(&self) -> DACBFEN_R {
        DACBFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - DAC Buffer Work Mode Select"]
    #[inline(always)]
    pub fn dacbfmd(&self) -> DACBFMD_R {
        DACBFMD_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - DAC Buffer Watermark Select"]
    #[inline(always)]
    pub fn dacbfwm(&self) -> DACBFWM_R {
        DACBFWM_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 7 - DMA Enable Select"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Enable"]
    #[inline(always)]
    pub fn dacbfen(&mut self) -> DACBFEN_W {
        DACBFEN_W { w: self }
    }
    #[doc = "Bits 1:2 - DAC Buffer Work Mode Select"]
    #[inline(always)]
    pub fn dacbfmd(&mut self) -> DACBFMD_W {
        DACBFMD_W { w: self }
    }
    #[doc = "Bits 3:4 - DAC Buffer Watermark Select"]
    #[inline(always)]
    pub fn dacbfwm(&mut self) -> DACBFWM_W {
        DACBFWM_W { w: self }
    }
    #[doc = "Bit 7 - DMA Enable Select"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
}
