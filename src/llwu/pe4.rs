#[doc = "Reader of register PE4"]
pub type R = crate::R<u8, super::PE4>;
#[doc = "Writer for register PE4"]
pub type W = crate::W<u8, super::PE4>;
#[doc = "Register PE4 `reset()`'s with value 0"]
impl crate::ResetValue for super::PE4 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE12_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE12_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE12_A) -> Self {
        match variant {
            WUPE12_A::_00 => 0,
            WUPE12_A::_01 => 1,
            WUPE12_A::_10 => 2,
            WUPE12_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE12`"]
pub type WUPE12_R = crate::R<u8, WUPE12_A>;
impl WUPE12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE12_A {
        match self.bits {
            0 => WUPE12_A::_00,
            1 => WUPE12_A::_01,
            2 => WUPE12_A::_10,
            3 => WUPE12_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE12_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE12_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE12_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE12_A::_11
    }
}
#[doc = "Write proxy for field `WUPE12`"]
pub struct WUPE12_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE12_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE12_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE12_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE12_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE12_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE13_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE13_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE13_A) -> Self {
        match variant {
            WUPE13_A::_00 => 0,
            WUPE13_A::_01 => 1,
            WUPE13_A::_10 => 2,
            WUPE13_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE13`"]
pub type WUPE13_R = crate::R<u8, WUPE13_A>;
impl WUPE13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE13_A {
        match self.bits {
            0 => WUPE13_A::_00,
            1 => WUPE13_A::_01,
            2 => WUPE13_A::_10,
            3 => WUPE13_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE13_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE13_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE13_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE13_A::_11
    }
}
#[doc = "Write proxy for field `WUPE13`"]
pub struct WUPE13_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE13_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE13_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE13_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE13_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE13_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u8) & 0x03) << 2);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE14_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE14_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE14_A) -> Self {
        match variant {
            WUPE14_A::_00 => 0,
            WUPE14_A::_01 => 1,
            WUPE14_A::_10 => 2,
            WUPE14_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE14`"]
pub type WUPE14_R = crate::R<u8, WUPE14_A>;
impl WUPE14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE14_A {
        match self.bits {
            0 => WUPE14_A::_00,
            1 => WUPE14_A::_01,
            2 => WUPE14_A::_10,
            3 => WUPE14_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE14_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE14_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE14_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE14_A::_11
    }
}
#[doc = "Write proxy for field `WUPE14`"]
pub struct WUPE14_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE14_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE14_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE14_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE14_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE14_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE15_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE15_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE15_A) -> Self {
        match variant {
            WUPE15_A::_00 => 0,
            WUPE15_A::_01 => 1,
            WUPE15_A::_10 => 2,
            WUPE15_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE15`"]
pub type WUPE15_R = crate::R<u8, WUPE15_A>;
impl WUPE15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE15_A {
        match self.bits {
            0 => WUPE15_A::_00,
            1 => WUPE15_A::_01,
            2 => WUPE15_A::_10,
            3 => WUPE15_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE15_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE15_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE15_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE15_A::_11
    }
}
#[doc = "Write proxy for field `WUPE15`"]
pub struct WUPE15_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE15_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE15_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE15_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE15_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE15_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P12"]
    #[inline(always)]
    pub fn wupe12(&self) -> WUPE12_R {
        WUPE12_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P13"]
    #[inline(always)]
    pub fn wupe13(&self) -> WUPE13_R {
        WUPE13_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P14"]
    #[inline(always)]
    pub fn wupe14(&self) -> WUPE14_R {
        WUPE14_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P15"]
    #[inline(always)]
    pub fn wupe15(&self) -> WUPE15_R {
        WUPE15_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P12"]
    #[inline(always)]
    pub fn wupe12(&mut self) -> WUPE12_W {
        WUPE12_W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P13"]
    #[inline(always)]
    pub fn wupe13(&mut self) -> WUPE13_W {
        WUPE13_W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P14"]
    #[inline(always)]
    pub fn wupe14(&mut self) -> WUPE14_W {
        WUPE14_W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P15"]
    #[inline(always)]
    pub fn wupe15(&mut self) -> WUPE15_W {
        WUPE15_W { w: self }
    }
}
