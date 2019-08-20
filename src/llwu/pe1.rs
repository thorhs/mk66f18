#[doc = "Reader of register PE1"]
pub type R = crate::R<u8, super::PE1>;
#[doc = "Writer for register PE1"]
pub type W = crate::W<u8, super::PE1>;
#[doc = "Register PE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PE1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE0_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE0_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE0_A) -> Self {
        match variant {
            WUPE0_A::_00 => 0,
            WUPE0_A::_01 => 1,
            WUPE0_A::_10 => 2,
            WUPE0_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE0`"]
pub type WUPE0_R = crate::R<u8, WUPE0_A>;
impl WUPE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE0_A {
        match self.bits {
            0 => WUPE0_A::_00,
            1 => WUPE0_A::_01,
            2 => WUPE0_A::_10,
            3 => WUPE0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE0_A::_11
    }
}
#[doc = "Write proxy for field `WUPE0`"]
pub struct WUPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE0_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE0_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE0_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE0_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE1_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE1_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE1_A) -> Self {
        match variant {
            WUPE1_A::_00 => 0,
            WUPE1_A::_01 => 1,
            WUPE1_A::_10 => 2,
            WUPE1_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE1`"]
pub type WUPE1_R = crate::R<u8, WUPE1_A>;
impl WUPE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE1_A {
        match self.bits {
            0 => WUPE1_A::_00,
            1 => WUPE1_A::_01,
            2 => WUPE1_A::_10,
            3 => WUPE1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE1_A::_11
    }
}
#[doc = "Write proxy for field `WUPE1`"]
pub struct WUPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE1_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE1_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE1_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE1_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u8) & 0x03) << 2);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE2_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE2_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE2_A) -> Self {
        match variant {
            WUPE2_A::_00 => 0,
            WUPE2_A::_01 => 1,
            WUPE2_A::_10 => 2,
            WUPE2_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE2`"]
pub type WUPE2_R = crate::R<u8, WUPE2_A>;
impl WUPE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE2_A {
        match self.bits {
            0 => WUPE2_A::_00,
            1 => WUPE2_A::_01,
            2 => WUPE2_A::_10,
            3 => WUPE2_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE2_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE2_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE2_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE2_A::_11
    }
}
#[doc = "Write proxy for field `WUPE2`"]
pub struct WUPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE2_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE2_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE2_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE2_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE3_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE3_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE3_A) -> Self {
        match variant {
            WUPE3_A::_00 => 0,
            WUPE3_A::_01 => 1,
            WUPE3_A::_10 => 2,
            WUPE3_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE3`"]
pub type WUPE3_R = crate::R<u8, WUPE3_A>;
impl WUPE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE3_A {
        match self.bits {
            0 => WUPE3_A::_00,
            1 => WUPE3_A::_01,
            2 => WUPE3_A::_10,
            3 => WUPE3_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE3_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE3_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE3_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE3_A::_11
    }
}
#[doc = "Write proxy for field `WUPE3`"]
pub struct WUPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE3_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE3_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE3_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE3_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P0"]
    #[inline(always)]
    pub fn wupe0(&self) -> WUPE0_R {
        WUPE0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P1"]
    #[inline(always)]
    pub fn wupe1(&self) -> WUPE1_R {
        WUPE1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P2"]
    #[inline(always)]
    pub fn wupe2(&self) -> WUPE2_R {
        WUPE2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P3"]
    #[inline(always)]
    pub fn wupe3(&self) -> WUPE3_R {
        WUPE3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P0"]
    #[inline(always)]
    pub fn wupe0(&mut self) -> WUPE0_W {
        WUPE0_W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P1"]
    #[inline(always)]
    pub fn wupe1(&mut self) -> WUPE1_W {
        WUPE1_W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P2"]
    #[inline(always)]
    pub fn wupe2(&mut self) -> WUPE2_W {
        WUPE2_W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P3"]
    #[inline(always)]
    pub fn wupe3(&mut self) -> WUPE3_W {
        WUPE3_W { w: self }
    }
}
