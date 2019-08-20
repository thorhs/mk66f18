#[doc = "Reader of register CTRLHU"]
pub type R = crate::R<u8, super::CTRLHU>;
#[doc = "Writer for register CTRLHU"]
pub type W = crate::W<u8, super::CTRLHU>;
#[doc = "Register CTRLHU `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLHU {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCRC_A {
    #[doc = "0: 16-bit CRC protocol."]
    _0,
    #[doc = "1: 32-bit CRC protocol."]
    _1,
}
impl From<TCRC_A> for bool {
    #[inline(always)]
    fn from(variant: TCRC_A) -> Self {
        match variant {
            TCRC_A::_0 => false,
            TCRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TCRC`"]
pub type TCRC_R = crate::R<bool, TCRC_A>;
impl TCRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCRC_A {
        match self.bits {
            false => TCRC_A::_0,
            true => TCRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCRC_A::_1
    }
}
#[doc = "Write proxy for field `TCRC`"]
pub struct TCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "16-bit CRC protocol."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCRC_A::_0)
    }
    #[doc = "32-bit CRC protocol."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCRC_A::_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAS_A {
    #[doc = "0: Writes to CRC data register are data values."]
    _0,
    #[doc = "1: Writes to CRC data reguster are seed values."]
    _1,
}
impl From<WAS_A> for bool {
    #[inline(always)]
    fn from(variant: WAS_A) -> Self {
        match variant {
            WAS_A::_0 => false,
            WAS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WAS`"]
pub type WAS_R = crate::R<bool, WAS_A>;
impl WAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAS_A {
        match self.bits {
            false => WAS_A::_0,
            true => WAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAS_A::_1
    }
}
#[doc = "Write proxy for field `WAS`"]
pub struct WAS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to CRC data register are data values."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAS_A::_0)
    }
    #[doc = "Writes to CRC data reguster are seed values."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAS_A::_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FXOR_A {
    #[doc = "0: No XOR on reading."]
    _0,
    #[doc = "1: Invert or complement the read value of CRC data register."]
    _1,
}
impl From<FXOR_A> for bool {
    #[inline(always)]
    fn from(variant: FXOR_A) -> Self {
        match variant {
            FXOR_A::_0 => false,
            FXOR_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FXOR`"]
pub type FXOR_R = crate::R<bool, FXOR_A>;
impl FXOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FXOR_A {
        match self.bits {
            false => FXOR_A::_0,
            true => FXOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FXOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FXOR_A::_1
    }
}
#[doc = "Write proxy for field `FXOR`"]
pub struct FXOR_W<'a> {
    w: &'a mut W,
}
impl<'a> FXOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FXOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No XOR on reading."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FXOR_A::_0)
    }
    #[doc = "Invert or complement the read value of CRC data register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FXOR_A::_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOTR_A {
    #[doc = "0: No Transposition."]
    _00,
    #[doc = "1: Bits in bytes are transposed, bytes are not transposed."]
    _01,
    #[doc = "2: Both bits in bytes and bytes are transposed."]
    _10,
    #[doc = "3: Only bytes are transposed; no bits in a byte are transposed."]
    _11,
}
impl From<TOTR_A> for u8 {
    #[inline(always)]
    fn from(variant: TOTR_A) -> Self {
        match variant {
            TOTR_A::_00 => 0,
            TOTR_A::_01 => 1,
            TOTR_A::_10 => 2,
            TOTR_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `TOTR`"]
pub type TOTR_R = crate::R<u8, TOTR_A>;
impl TOTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOTR_A {
        match self.bits {
            0 => TOTR_A::_00,
            1 => TOTR_A::_01,
            2 => TOTR_A::_10,
            3 => TOTR_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TOTR_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TOTR_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TOTR_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TOTR_A::_11
    }
}
#[doc = "Write proxy for field `TOTR`"]
pub struct TOTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOTR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No Transposition."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TOTR_A::_00)
    }
    #[doc = "Bits in bytes are transposed, bytes are not transposed."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TOTR_A::_01)
    }
    #[doc = "Both bits in bytes and bytes are transposed."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TOTR_A::_10)
    }
    #[doc = "Only bytes are transposed; no bits in a byte are transposed."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TOTR_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOT_A {
    #[doc = "0: No Transposition."]
    _00,
    #[doc = "1: Bits in bytes are transposed, bytes are not transposed."]
    _01,
    #[doc = "2: Both bits in bytes and bytes are transposed."]
    _10,
    #[doc = "3: Only bytes are transposed; no bits in a byte are transposed."]
    _11,
}
impl From<TOT_A> for u8 {
    #[inline(always)]
    fn from(variant: TOT_A) -> Self {
        match variant {
            TOT_A::_00 => 0,
            TOT_A::_01 => 1,
            TOT_A::_10 => 2,
            TOT_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `TOT`"]
pub type TOT_R = crate::R<u8, TOT_A>;
impl TOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOT_A {
        match self.bits {
            0 => TOT_A::_00,
            1 => TOT_A::_01,
            2 => TOT_A::_10,
            3 => TOT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TOT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TOT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TOT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TOT_A::_11
    }
}
#[doc = "Write proxy for field `TOT`"]
pub struct TOT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No Transposition."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TOT_A::_00)
    }
    #[doc = "Bits in bytes are transposed, bytes are not transposed."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TOT_A::_01)
    }
    #[doc = "Both bits in bytes and bytes are transposed."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TOT_A::_10)
    }
    #[doc = "Only bytes are transposed; no bits in a byte are transposed."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TOT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn tcrc(&self) -> TCRC_R {
        TCRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn was(&self) -> WAS_R {
        WAS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn fxor(&self) -> FXOR_R {
        FXOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - no description available"]
    #[inline(always)]
    pub fn totr(&self) -> TOTR_R {
        TOTR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - no description available"]
    #[inline(always)]
    pub fn tot(&self) -> TOT_R {
        TOT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn tcrc(&mut self) -> TCRC_W {
        TCRC_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn was(&mut self) -> WAS_W {
        WAS_W { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn fxor(&mut self) -> FXOR_W {
        FXOR_W { w: self }
    }
    #[doc = "Bits 4:5 - no description available"]
    #[inline(always)]
    pub fn totr(&mut self) -> TOTR_W {
        TOTR_W { w: self }
    }
    #[doc = "Bits 6:7 - no description available"]
    #[inline(always)]
    pub fn tot(&mut self) -> TOT_W {
        TOT_W { w: self }
    }
}
