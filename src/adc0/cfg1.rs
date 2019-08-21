#[doc = "Reader of register CFG1"]
pub type R = crate::R<u32, super::CFG1>;
#[doc = "Writer for register CFG1"]
pub type W = crate::W<u32, super::CFG1>;
#[doc = "Register CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Input Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADICLK_A {
    #[doc = "0: Bus clock"]
    _00,
    #[doc = "1: Bus clock divided by 2(BUSCLK/2)"]
    _01,
    #[doc = "2: Alternate clock (ALTCLK)"]
    _10,
    #[doc = "3: Asynchronous clock (ADACK)"]
    _11,
}
impl From<ADICLK_A> for u8 {
    #[inline(always)]
    fn from(variant: ADICLK_A) -> Self {
        match variant {
            ADICLK_A::_00 => 0,
            ADICLK_A::_01 => 1,
            ADICLK_A::_10 => 2,
            ADICLK_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `ADICLK`"]
pub type ADICLK_R = crate::R<u8, ADICLK_A>;
impl ADICLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADICLK_A {
        match self.bits {
            0 => ADICLK_A::_00,
            1 => ADICLK_A::_01,
            2 => ADICLK_A::_10,
            3 => ADICLK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADICLK_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADICLK_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADICLK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADICLK_A::_11
    }
}
#[doc = "Write proxy for field `ADICLK`"]
pub struct ADICLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADICLK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bus clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADICLK_A::_00)
    }
    #[doc = "Bus clock divided by 2(BUSCLK/2)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADICLK_A::_01)
    }
    #[doc = "Alternate clock (ALTCLK)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADICLK_A::_10)
    }
    #[doc = "Asynchronous clock (ADACK)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADICLK_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Conversion mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: When DIFF=0:It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    _00,
    #[doc = "1: When DIFF=0:It is single-ended 12-bit conversion ; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    _01,
    #[doc = "2: When DIFF=0:It is single-ended 10-bit conversion. ; when DIFF=1, it is differential 11-bit conversion with 2's complement output"]
    _10,
    #[doc = "3: When DIFF=0:It is single-ended 16-bit conversion..; when DIFF=1, it is differential 16-bit conversion with 2's complement output"]
    _11,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::_00 => 0,
            MODE_A::_01 => 1,
            MODE_A::_10 => 2,
            MODE_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::_00,
            1 => MODE_A::_01,
            2 => MODE_A::_10,
            3 => MODE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MODE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MODE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MODE_A::_11
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
            self.bits(variant.into())
        }
    }
    #[doc = "When DIFF=0:It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MODE_A::_00)
    }
    #[doc = "When DIFF=0:It is single-ended 12-bit conversion ; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MODE_A::_01)
    }
    #[doc = "When DIFF=0:It is single-ended 10-bit conversion. ; when DIFF=1, it is differential 11-bit conversion with 2's complement output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODE_A::_10)
    }
    #[doc = "When DIFF=0:It is single-ended 16-bit conversion..; when DIFF=1, it is differential 16-bit conversion with 2's complement output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MODE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Sample Time Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLSMP_A {
    #[doc = "0: Short sample time."]
    _0,
    #[doc = "1: Long sample time."]
    _1,
}
impl From<ADLSMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADLSMP_A) -> Self {
        match variant {
            ADLSMP_A::_0 => false,
            ADLSMP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADLSMP`"]
pub type ADLSMP_R = crate::R<bool, ADLSMP_A>;
impl ADLSMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADLSMP_A {
        match self.bits {
            false => ADLSMP_A::_0,
            true => ADLSMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADLSMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADLSMP_A::_1
    }
}
#[doc = "Write proxy for field `ADLSMP`"]
pub struct ADLSMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADLSMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADLSMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Short sample time."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADLSMP_A::_0)
    }
    #[doc = "Long sample time."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADLSMP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Clock Divide Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADIV_A {
    #[doc = "0: The divide ratio is 1 and the clock rate is input clock."]
    _00,
    #[doc = "1: The divide ratio is 2 and the clock rate is (input clock)/2."]
    _01,
    #[doc = "2: The divide ratio is 4 and the clock rate is (input clock)/4."]
    _10,
    #[doc = "3: The divide ratio is 8 and the clock rate is (input clock)/8."]
    _11,
}
impl From<ADIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADIV_A) -> Self {
        match variant {
            ADIV_A::_00 => 0,
            ADIV_A::_01 => 1,
            ADIV_A::_10 => 2,
            ADIV_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `ADIV`"]
pub type ADIV_R = crate::R<u8, ADIV_A>;
impl ADIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIV_A {
        match self.bits {
            0 => ADIV_A::_00,
            1 => ADIV_A::_01,
            2 => ADIV_A::_10,
            3 => ADIV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADIV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADIV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADIV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADIV_A::_11
    }
}
#[doc = "Write proxy for field `ADIV`"]
pub struct ADIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The divide ratio is 1 and the clock rate is input clock."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADIV_A::_00)
    }
    #[doc = "The divide ratio is 2 and the clock rate is (input clock)/2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADIV_A::_01)
    }
    #[doc = "The divide ratio is 4 and the clock rate is (input clock)/4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADIV_A::_10)
    }
    #[doc = "The divide ratio is 8 and the clock rate is (input clock)/8."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADIV_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Low-Power Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLPC_A {
    #[doc = "0: Normal power configuration."]
    _0,
    #[doc = "1: Low-power configuration. The power is reduced at the expense of maximum clock speed."]
    _1,
}
impl From<ADLPC_A> for bool {
    #[inline(always)]
    fn from(variant: ADLPC_A) -> Self {
        match variant {
            ADLPC_A::_0 => false,
            ADLPC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADLPC`"]
pub type ADLPC_R = crate::R<bool, ADLPC_A>;
impl ADLPC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADLPC_A {
        match self.bits {
            false => ADLPC_A::_0,
            true => ADLPC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADLPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADLPC_A::_1
    }
}
#[doc = "Write proxy for field `ADLPC`"]
pub struct ADLPC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADLPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADLPC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal power configuration."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADLPC_A::_0)
    }
    #[doc = "Low-power configuration. The power is reduced at the expense of maximum clock speed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADLPC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline(always)]
    pub fn adiclk(&self) -> ADICLK_R {
        ADICLK_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Conversion mode selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Sample Time Configuration"]
    #[inline(always)]
    pub fn adlsmp(&self) -> ADLSMP_R {
        ADLSMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline(always)]
    pub fn adiv(&self) -> ADIV_R {
        ADIV_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline(always)]
    pub fn adlpc(&self) -> ADLPC_R {
        ADLPC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline(always)]
    pub fn adiclk(&mut self) -> ADICLK_W {
        ADICLK_W { w: self }
    }
    #[doc = "Bits 2:3 - Conversion mode selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 4 - Sample Time Configuration"]
    #[inline(always)]
    pub fn adlsmp(&mut self) -> ADLSMP_W {
        ADLSMP_W { w: self }
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline(always)]
    pub fn adiv(&mut self) -> ADIV_W {
        ADIV_W { w: self }
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline(always)]
    pub fn adlpc(&mut self) -> ADLPC_W {
        ADLPC_W { w: self }
    }
}
