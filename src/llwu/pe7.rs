#[doc = "Reader of register PE7"]
pub type R = crate::R<u8, super::PE7>;
#[doc = "Writer for register PE7"]
pub type W = crate::W<u8, super::PE7>;
#[doc = "Register PE7 `reset()`'s with value 0"]
impl crate::ResetValue for super::PE7 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE24_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE24_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE24_A) -> Self {
        match variant {
            WUPE24_A::_00 => 0,
            WUPE24_A::_01 => 1,
            WUPE24_A::_10 => 2,
            WUPE24_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE24`"]
pub type WUPE24_R = crate::R<u8, WUPE24_A>;
impl WUPE24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE24_A {
        match self.bits {
            0 => WUPE24_A::_00,
            1 => WUPE24_A::_01,
            2 => WUPE24_A::_10,
            3 => WUPE24_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE24_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE24_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE24_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE24_A::_11
    }
}
#[doc = "Write proxy for field `WUPE24`"]
pub struct WUPE24_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE24_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE24_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE24_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE24_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE24_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE25_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE25_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE25_A) -> Self {
        match variant {
            WUPE25_A::_00 => 0,
            WUPE25_A::_01 => 1,
            WUPE25_A::_10 => 2,
            WUPE25_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE25`"]
pub type WUPE25_R = crate::R<u8, WUPE25_A>;
impl WUPE25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE25_A {
        match self.bits {
            0 => WUPE25_A::_00,
            1 => WUPE25_A::_01,
            2 => WUPE25_A::_10,
            3 => WUPE25_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE25_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE25_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE25_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE25_A::_11
    }
}
#[doc = "Write proxy for field `WUPE25`"]
pub struct WUPE25_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE25_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE25_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE25_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE25_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE25_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u8) & 0x03) << 2);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE26_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE26_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE26_A) -> Self {
        match variant {
            WUPE26_A::_00 => 0,
            WUPE26_A::_01 => 1,
            WUPE26_A::_10 => 2,
            WUPE26_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE26`"]
pub type WUPE26_R = crate::R<u8, WUPE26_A>;
impl WUPE26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE26_A {
        match self.bits {
            0 => WUPE26_A::_00,
            1 => WUPE26_A::_01,
            2 => WUPE26_A::_10,
            3 => WUPE26_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE26_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE26_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE26_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE26_A::_11
    }
}
#[doc = "Write proxy for field `WUPE26`"]
pub struct WUPE26_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE26_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE26_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE26_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE26_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE26_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u8) & 0x03) << 4);
        self.w
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE27_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10,
    #[doc = "3: External input pin enabled with any change detection"]
    _11,
}
impl From<WUPE27_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE27_A) -> Self {
        match variant {
            WUPE27_A::_00 => 0,
            WUPE27_A::_01 => 1,
            WUPE27_A::_10 => 2,
            WUPE27_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `WUPE27`"]
pub type WUPE27_R = crate::R<u8, WUPE27_A>;
impl WUPE27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE27_A {
        match self.bits {
            0 => WUPE27_A::_00,
            1 => WUPE27_A::_01,
            2 => WUPE27_A::_10,
            3 => WUPE27_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE27_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE27_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE27_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE27_A::_11
    }
}
#[doc = "Write proxy for field `WUPE27`"]
pub struct WUPE27_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPE27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUPE27_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE27_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE27_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE27_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE27_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P24"]
    #[inline(always)]
    pub fn wupe24(&self) -> WUPE24_R {
        WUPE24_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P25"]
    #[inline(always)]
    pub fn wupe25(&self) -> WUPE25_R {
        WUPE25_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P26"]
    #[inline(always)]
    pub fn wupe26(&self) -> WUPE26_R {
        WUPE26_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P27"]
    #[inline(always)]
    pub fn wupe27(&self) -> WUPE27_R {
        WUPE27_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P24"]
    #[inline(always)]
    pub fn wupe24(&mut self) -> WUPE24_W {
        WUPE24_W { w: self }
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P25"]
    #[inline(always)]
    pub fn wupe25(&mut self) -> WUPE25_W {
        WUPE25_W { w: self }
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P26"]
    #[inline(always)]
    pub fn wupe26(&mut self) -> WUPE26_W {
        WUPE26_W { w: self }
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P27"]
    #[inline(always)]
    pub fn wupe27(&mut self) -> WUPE27_W {
        WUPE27_W { w: self }
    }
}
