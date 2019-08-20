#[doc = "Reader of register PE3"]
pub type R = crate::R<u8, super::PE3>;
#[doc = "Writer for register PE3"]
pub type W = crate::W<u8, super::PE3>;
#[doc = "Register PE3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PE3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE8_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE8_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE8_A) -> Self {
        match variant {
            WUPE8_A::_00 => 0,
            WUPE8_A::_01 => 1,
            WUPE8_A::_10 => 2,
            WUPE8_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE8`"]
pub type WUPE8_R = crate::R<u8, WUPE8_A>;
impl WUPE8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE8_A {
        match self.bits {
            0 => WUPE8_A::_00,
            1 => WUPE8_A::_01,
            2 => WUPE8_A::_10,
            3 => WUPE8_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE8_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE8_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE8_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE8_A::_11
    }
}
#[doc = "Write proxy for field `WUPE8`"]
pub struct WUPE8_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE8_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE8_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE8_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE8_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE8_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE9_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE9_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE9_A) -> Self {
        match variant {
            WUPE9_A::_00 => 0,
            WUPE9_A::_01 => 1,
            WUPE9_A::_10 => 2,
            WUPE9_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE9`"]
pub type WUPE9_R = crate::R<u8, WUPE9_A>;
impl WUPE9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE9_A {
        match self.bits {
            0 => WUPE9_A::_00,
            1 => WUPE9_A::_01,
            2 => WUPE9_A::_10,
            3 => WUPE9_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE9_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE9_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE9_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE9_A::_11
    }
}
#[doc = "Write proxy for field `WUPE9`"]
pub struct WUPE9_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE9_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE9_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE9_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE9_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE9_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u8) & 0x03) << 2);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE10_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE10_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE10_A) -> Self {
        match variant {
            WUPE10_A::_00 => 0,
            WUPE10_A::_01 => 1,
            WUPE10_A::_10 => 2,
            WUPE10_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE10`"]
pub type WUPE10_R = crate::R<u8, WUPE10_A>;
impl WUPE10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE10_A {
        match self.bits {
            0 => WUPE10_A::_00,
            1 => WUPE10_A::_01,
            2 => WUPE10_A::_10,
            3 => WUPE10_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE10_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE10_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE10_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE10_A::_11
    }
}
#[doc = "Write proxy for field `WUPE10`"]
pub struct WUPE10_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE10_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE10_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE10_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE10_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE11_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE11_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE11_A) -> Self {
        match variant {
            WUPE11_A::_00 => 0,
            WUPE11_A::_01 => 1,
            WUPE11_A::_10 => 2,
            WUPE11_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE11`"]
pub type WUPE11_R = crate::R<u8, WUPE11_A>;
impl WUPE11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE11_A {
        match self.bits {
            0 => WUPE11_A::_00,
            1 => WUPE11_A::_01,
            2 => WUPE11_A::_10,
            3 => WUPE11_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE11_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE11_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE11_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE11_A::_11
    }
}
#[doc = "Write proxy for field `WUPE11`"]
pub struct WUPE11_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE11_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE11_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE11_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE11_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P8"]
    #[inline(always)]
    pub fn wupe8(&self) -> WUPE8_R {
        WUPE8_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P9"]
    #[inline(always)]
    pub fn wupe9(&self) -> WUPE9_R {
        WUPE9_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P10"]
    #[inline(always)]
    pub fn wupe10(&self) -> WUPE10_R {
        WUPE10_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P11"]
    #[inline(always)]
    pub fn wupe11(&self) -> WUPE11_R {
        WUPE11_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P8"]
    #[inline(always)]
    pub fn wupe8(&mut self) -> WUPE8_W {
        WUPE8_W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P9"]
    #[inline(always)]
    pub fn wupe9(&mut self) -> WUPE9_W {
        WUPE9_W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P10"]
    #[inline(always)]
    pub fn wupe10(&mut self) -> WUPE10_W {
        WUPE10_W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P11"]
    #[inline(always)]
    pub fn wupe11(&mut self) -> WUPE11_W {
        WUPE11_W { w: self }
    }
}
