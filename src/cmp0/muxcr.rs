#[doc = "Reader of register MUXCR"]
pub type R = crate::R<u8, super::MUXCR>;
#[doc = "Writer for register MUXCR"]
pub type W = crate::W<u8, super::MUXCR>;
#[doc = "Register MUXCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MUXCR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Minus Input Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSEL_A {
    #[doc = "0: IN0"]
    _000,
    #[doc = "1: IN1"]
    _001,
    #[doc = "2: IN2"]
    _010,
    #[doc = "3: IN3"]
    _011,
    #[doc = "4: IN4"]
    _100,
    #[doc = "5: IN5"]
    _101,
    #[doc = "6: IN6"]
    _110,
    #[doc = "7: IN7"]
    _111,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        match variant {
            MSEL_A::_000 => 0,
            MSEL_A::_001 => 1,
            MSEL_A::_010 => 2,
            MSEL_A::_011 => 3,
            MSEL_A::_100 => 4,
            MSEL_A::_101 => 5,
            MSEL_A::_110 => 6,
            MSEL_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `MSEL`"]
pub type MSEL_R = crate::R<u8, MSEL_A>;
impl MSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::_000,
            1 => MSEL_A::_001,
            2 => MSEL_A::_010,
            3 => MSEL_A::_011,
            4 => MSEL_A::_100,
            5 => MSEL_A::_101,
            6 => MSEL_A::_110,
            7 => MSEL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == MSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == MSEL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == MSEL_A::_111
    }
}
#[doc = "Write proxy for field `MSEL`"]
pub struct MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "IN0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(MSEL_A::_000)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(MSEL_A::_001)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(MSEL_A::_010)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(MSEL_A::_011)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MSEL_A::_100)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(MSEL_A::_101)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(MSEL_A::_110)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(MSEL_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Plus Input Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSEL_A {
    #[doc = "0: IN0"]
    _000,
    #[doc = "1: IN1"]
    _001,
    #[doc = "2: IN2"]
    _010,
    #[doc = "3: IN3"]
    _011,
    #[doc = "4: IN4"]
    _100,
    #[doc = "5: IN5"]
    _101,
    #[doc = "6: IN6"]
    _110,
    #[doc = "7: IN7"]
    _111,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        match variant {
            PSEL_A::_000 => 0,
            PSEL_A::_001 => 1,
            PSEL_A::_010 => 2,
            PSEL_A::_011 => 3,
            PSEL_A::_100 => 4,
            PSEL_A::_101 => 5,
            PSEL_A::_110 => 6,
            PSEL_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `PSEL`"]
pub type PSEL_R = crate::R<u8, PSEL_A>;
impl PSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::_000,
            1 => PSEL_A::_001,
            2 => PSEL_A::_010,
            3 => PSEL_A::_011,
            4 => PSEL_A::_100,
            5 => PSEL_A::_101,
            6 => PSEL_A::_110,
            7 => PSEL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PSEL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PSEL_A::_111
    }
}
#[doc = "Write proxy for field `PSEL`"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "IN0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PSEL_A::_000)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PSEL_A::_001)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PSEL_A::_010)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PSEL_A::_011)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PSEL_A::_100)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PSEL_A::_101)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PSEL_A::_110)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PSEL_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u8) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pass Through Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSTM_A {
    #[doc = "0: Pass Through Mode is disabled."]
    _0,
    #[doc = "1: Pass Through Mode is enabled."]
    _1,
}
impl From<PSTM_A> for bool {
    #[inline(always)]
    fn from(variant: PSTM_A) -> Self {
        match variant {
            PSTM_A::_0 => false,
            PSTM_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PSTM`"]
pub type PSTM_R = crate::R<bool, PSTM_A>;
impl PSTM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSTM_A {
        match self.bits {
            false => PSTM_A::_0,
            true => PSTM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSTM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSTM_A::_1
    }
}
#[doc = "Write proxy for field `PSTM`"]
pub struct PSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> PSTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSTM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pass Through Mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSTM_A::_0)
    }
    #[doc = "Pass Through Mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSTM_A::_1)
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
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Pass Through Mode Enable"]
    #[inline(always)]
    pub fn pstm(&self) -> PSTM_R {
        PSTM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Bit 7 - Pass Through Mode Enable"]
    #[inline(always)]
    pub fn pstm(&mut self) -> PSTM_W {
        PSTM_W { w: self }
    }
}
