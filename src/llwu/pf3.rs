#[doc = "Reader of register PF3"]
pub type R = crate::R<u8, super::PF3>;
#[doc = "Writer for register PF3"]
pub type W = crate::W<u8, super::PF3>;
#[doc = "Register PF3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PF3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wakeup Flag For LLWU_P16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF16_A {
    #[doc = "0: LLWU_P16 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P16 input was a wakeup source"]
    _1,
}
impl From<WUF16_A> for bool {
    #[inline(always)]
    fn from(variant: WUF16_A) -> Self {
        match variant {
            WUF16_A::_0 => false,
            WUF16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF16`"]
pub type WUF16_R = crate::R<bool, WUF16_A>;
impl WUF16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF16_A {
        match self.bits {
            false => WUF16_A::_0,
            true => WUF16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF16_A::_1
    }
}
#[doc = "Write proxy for field `WUF16`"]
pub struct WUF16_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P16 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF16_A::_0)
    }
    #[doc = "LLWU_P16 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF16_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF17_A {
    #[doc = "0: LLWU_P17 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P17 input was a wakeup source"]
    _1,
}
impl From<WUF17_A> for bool {
    #[inline(always)]
    fn from(variant: WUF17_A) -> Self {
        match variant {
            WUF17_A::_0 => false,
            WUF17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF17`"]
pub type WUF17_R = crate::R<bool, WUF17_A>;
impl WUF17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF17_A {
        match self.bits {
            false => WUF17_A::_0,
            true => WUF17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF17_A::_1
    }
}
#[doc = "Write proxy for field `WUF17`"]
pub struct WUF17_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P17 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF17_A::_0)
    }
    #[doc = "LLWU_P17 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF17_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF18_A {
    #[doc = "0: LLWU_P18 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P18 input was a wakeup source"]
    _1,
}
impl From<WUF18_A> for bool {
    #[inline(always)]
    fn from(variant: WUF18_A) -> Self {
        match variant {
            WUF18_A::_0 => false,
            WUF18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF18`"]
pub type WUF18_R = crate::R<bool, WUF18_A>;
impl WUF18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF18_A {
        match self.bits {
            false => WUF18_A::_0,
            true => WUF18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF18_A::_1
    }
}
#[doc = "Write proxy for field `WUF18`"]
pub struct WUF18_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P18 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF18_A::_0)
    }
    #[doc = "LLWU_P18 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF18_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF19_A {
    #[doc = "0: LLWU_P19 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P19 input was a wakeup source"]
    _1,
}
impl From<WUF19_A> for bool {
    #[inline(always)]
    fn from(variant: WUF19_A) -> Self {
        match variant {
            WUF19_A::_0 => false,
            WUF19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF19`"]
pub type WUF19_R = crate::R<bool, WUF19_A>;
impl WUF19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF19_A {
        match self.bits {
            false => WUF19_A::_0,
            true => WUF19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF19_A::_1
    }
}
#[doc = "Write proxy for field `WUF19`"]
pub struct WUF19_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P19 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF19_A::_0)
    }
    #[doc = "LLWU_P19 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF19_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Wakeup Flag For LLWU_P20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF20_A {
    #[doc = "0: LLWU_P20 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P20 input was a wakeup source"]
    _1,
}
impl From<WUF20_A> for bool {
    #[inline(always)]
    fn from(variant: WUF20_A) -> Self {
        match variant {
            WUF20_A::_0 => false,
            WUF20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF20`"]
pub type WUF20_R = crate::R<bool, WUF20_A>;
impl WUF20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF20_A {
        match self.bits {
            false => WUF20_A::_0,
            true => WUF20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF20_A::_1
    }
}
#[doc = "Write proxy for field `WUF20`"]
pub struct WUF20_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P20 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF20_A::_0)
    }
    #[doc = "LLWU_P20 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF20_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Wakeup Flag For LLWU_P21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF21_A {
    #[doc = "0: LLWU_P21 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P21 input was a wakeup source"]
    _1,
}
impl From<WUF21_A> for bool {
    #[inline(always)]
    fn from(variant: WUF21_A) -> Self {
        match variant {
            WUF21_A::_0 => false,
            WUF21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF21`"]
pub type WUF21_R = crate::R<bool, WUF21_A>;
impl WUF21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF21_A {
        match self.bits {
            false => WUF21_A::_0,
            true => WUF21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF21_A::_1
    }
}
#[doc = "Write proxy for field `WUF21`"]
pub struct WUF21_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P21 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF21_A::_0)
    }
    #[doc = "LLWU_P21 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF21_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Wakeup Flag For LLWU_P22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF22_A {
    #[doc = "0: LLWU_P22 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P22 input was a wakeup source"]
    _1,
}
impl From<WUF22_A> for bool {
    #[inline(always)]
    fn from(variant: WUF22_A) -> Self {
        match variant {
            WUF22_A::_0 => false,
            WUF22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF22`"]
pub type WUF22_R = crate::R<bool, WUF22_A>;
impl WUF22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF22_A {
        match self.bits {
            false => WUF22_A::_0,
            true => WUF22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF22_A::_1
    }
}
#[doc = "Write proxy for field `WUF22`"]
pub struct WUF22_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P22 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF22_A::_0)
    }
    #[doc = "LLWU_P22 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF22_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF23_A {
    #[doc = "0: LLWU_P23 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P23 input was a wakeup source"]
    _1,
}
impl From<WUF23_A> for bool {
    #[inline(always)]
    fn from(variant: WUF23_A) -> Self {
        match variant {
            WUF23_A::_0 => false,
            WUF23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF23`"]
pub type WUF23_R = crate::R<bool, WUF23_A>;
impl WUF23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF23_A {
        match self.bits {
            false => WUF23_A::_0,
            true => WUF23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF23_A::_1
    }
}
#[doc = "Write proxy for field `WUF23`"]
pub struct WUF23_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P23 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF23_A::_0)
    }
    #[doc = "LLWU_P23 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF23_A::_1)
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P16"]
    #[inline(always)]
    pub fn wuf16(&self) -> WUF16_R {
        WUF16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P17"]
    #[inline(always)]
    pub fn wuf17(&self) -> WUF17_R {
        WUF17_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P18"]
    #[inline(always)]
    pub fn wuf18(&self) -> WUF18_R {
        WUF18_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P19"]
    #[inline(always)]
    pub fn wuf19(&self) -> WUF19_R {
        WUF19_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P20"]
    #[inline(always)]
    pub fn wuf20(&self) -> WUF20_R {
        WUF20_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P21"]
    #[inline(always)]
    pub fn wuf21(&self) -> WUF21_R {
        WUF21_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P22"]
    #[inline(always)]
    pub fn wuf22(&self) -> WUF22_R {
        WUF22_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P23"]
    #[inline(always)]
    pub fn wuf23(&self) -> WUF23_R {
        WUF23_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P16"]
    #[inline(always)]
    pub fn wuf16(&mut self) -> WUF16_W {
        WUF16_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P17"]
    #[inline(always)]
    pub fn wuf17(&mut self) -> WUF17_W {
        WUF17_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P18"]
    #[inline(always)]
    pub fn wuf18(&mut self) -> WUF18_W {
        WUF18_W { w: self }
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P19"]
    #[inline(always)]
    pub fn wuf19(&mut self) -> WUF19_W {
        WUF19_W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P20"]
    #[inline(always)]
    pub fn wuf20(&mut self) -> WUF20_W {
        WUF20_W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P21"]
    #[inline(always)]
    pub fn wuf21(&mut self) -> WUF21_W {
        WUF21_W { w: self }
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P22"]
    #[inline(always)]
    pub fn wuf22(&mut self) -> WUF22_W {
        WUF22_W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P23"]
    #[inline(always)]
    pub fn wuf23(&mut self) -> WUF23_W {
        WUF23_W { w: self }
    }
}
