#[doc = "Reader of register PF2"]
pub type R = crate::R<u8, super::PF2>;
#[doc = "Writer for register PF2"]
pub type W = crate::W<u8, super::PF2>;
#[doc = "Register PF2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PF2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wakeup Flag For LLWU_P8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF8_A {
    #[doc = "0: LLWU_P8 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P8 input was a wakeup source"]
    _1,
}
impl From<WUF8_A> for bool {
    #[inline(always)]
    fn from(variant: WUF8_A) -> Self {
        match variant {
            WUF8_A::_0 => false,
            WUF8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF8`"]
pub type WUF8_R = crate::R<bool, WUF8_A>;
impl WUF8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF8_A {
        match self.bits {
            false => WUF8_A::_0,
            true => WUF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF8_A::_1
    }
}
#[doc = "Write proxy for field `WUF8`"]
pub struct WUF8_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P8 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF8_A::_0)
    }
    #[doc = "LLWU_P8 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF8_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF9_A {
    #[doc = "0: LLWU_P9 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P9 input was a wakeup source"]
    _1,
}
impl From<WUF9_A> for bool {
    #[inline(always)]
    fn from(variant: WUF9_A) -> Self {
        match variant {
            WUF9_A::_0 => false,
            WUF9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF9`"]
pub type WUF9_R = crate::R<bool, WUF9_A>;
impl WUF9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF9_A {
        match self.bits {
            false => WUF9_A::_0,
            true => WUF9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF9_A::_1
    }
}
#[doc = "Write proxy for field `WUF9`"]
pub struct WUF9_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P9 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF9_A::_0)
    }
    #[doc = "LLWU_P9 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF9_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF10_A {
    #[doc = "0: LLWU_P10 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P10 input was a wakeup source"]
    _1,
}
impl From<WUF10_A> for bool {
    #[inline(always)]
    fn from(variant: WUF10_A) -> Self {
        match variant {
            WUF10_A::_0 => false,
            WUF10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF10`"]
pub type WUF10_R = crate::R<bool, WUF10_A>;
impl WUF10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF10_A {
        match self.bits {
            false => WUF10_A::_0,
            true => WUF10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF10_A::_1
    }
}
#[doc = "Write proxy for field `WUF10`"]
pub struct WUF10_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P10 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF10_A::_0)
    }
    #[doc = "LLWU_P10 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF10_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF11_A {
    #[doc = "0: LLWU_P11 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P11 input was a wakeup source"]
    _1,
}
impl From<WUF11_A> for bool {
    #[inline(always)]
    fn from(variant: WUF11_A) -> Self {
        match variant {
            WUF11_A::_0 => false,
            WUF11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF11`"]
pub type WUF11_R = crate::R<bool, WUF11_A>;
impl WUF11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF11_A {
        match self.bits {
            false => WUF11_A::_0,
            true => WUF11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF11_A::_1
    }
}
#[doc = "Write proxy for field `WUF11`"]
pub struct WUF11_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P11 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF11_A::_0)
    }
    #[doc = "LLWU_P11 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF11_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF12_A {
    #[doc = "0: LLWU_P12 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P12 input was a wakeup source"]
    _1,
}
impl From<WUF12_A> for bool {
    #[inline(always)]
    fn from(variant: WUF12_A) -> Self {
        match variant {
            WUF12_A::_0 => false,
            WUF12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF12`"]
pub type WUF12_R = crate::R<bool, WUF12_A>;
impl WUF12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF12_A {
        match self.bits {
            false => WUF12_A::_0,
            true => WUF12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF12_A::_1
    }
}
#[doc = "Write proxy for field `WUF12`"]
pub struct WUF12_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P12 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF12_A::_0)
    }
    #[doc = "LLWU_P12 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF12_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF13_A {
    #[doc = "0: LLWU_P13 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P13 input was a wakeup source"]
    _1,
}
impl From<WUF13_A> for bool {
    #[inline(always)]
    fn from(variant: WUF13_A) -> Self {
        match variant {
            WUF13_A::_0 => false,
            WUF13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF13`"]
pub type WUF13_R = crate::R<bool, WUF13_A>;
impl WUF13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF13_A {
        match self.bits {
            false => WUF13_A::_0,
            true => WUF13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF13_A::_1
    }
}
#[doc = "Write proxy for field `WUF13`"]
pub struct WUF13_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P13 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF13_A::_0)
    }
    #[doc = "LLWU_P13 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF13_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF14_A {
    #[doc = "0: LLWU_P14 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P14 input was a wakeup source"]
    _1,
}
impl From<WUF14_A> for bool {
    #[inline(always)]
    fn from(variant: WUF14_A) -> Self {
        match variant {
            WUF14_A::_0 => false,
            WUF14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF14`"]
pub type WUF14_R = crate::R<bool, WUF14_A>;
impl WUF14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF14_A {
        match self.bits {
            false => WUF14_A::_0,
            true => WUF14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF14_A::_1
    }
}
#[doc = "Write proxy for field `WUF14`"]
pub struct WUF14_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P14 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF14_A::_0)
    }
    #[doc = "LLWU_P14 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF14_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF15_A {
    #[doc = "0: LLWU_P15 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P15 input was a wakeup source"]
    _1,
}
impl From<WUF15_A> for bool {
    #[inline(always)]
    fn from(variant: WUF15_A) -> Self {
        match variant {
            WUF15_A::_0 => false,
            WUF15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF15`"]
pub type WUF15_R = crate::R<bool, WUF15_A>;
impl WUF15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF15_A {
        match self.bits {
            false => WUF15_A::_0,
            true => WUF15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF15_A::_1
    }
}
#[doc = "Write proxy for field `WUF15`"]
pub struct WUF15_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P15 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF15_A::_0)
    }
    #[doc = "LLWU_P15 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF15_A::_1)
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P8"]
    #[inline(always)]
    pub fn wuf8(&self) -> WUF8_R {
        WUF8_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P9"]
    #[inline(always)]
    pub fn wuf9(&self) -> WUF9_R {
        WUF9_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P10"]
    #[inline(always)]
    pub fn wuf10(&self) -> WUF10_R {
        WUF10_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P11"]
    #[inline(always)]
    pub fn wuf11(&self) -> WUF11_R {
        WUF11_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P12"]
    #[inline(always)]
    pub fn wuf12(&self) -> WUF12_R {
        WUF12_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P13"]
    #[inline(always)]
    pub fn wuf13(&self) -> WUF13_R {
        WUF13_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P14"]
    #[inline(always)]
    pub fn wuf14(&self) -> WUF14_R {
        WUF14_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P15"]
    #[inline(always)]
    pub fn wuf15(&self) -> WUF15_R {
        WUF15_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P8"]
    #[inline(always)]
    pub fn wuf8(&mut self) -> WUF8_W {
        WUF8_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P9"]
    #[inline(always)]
    pub fn wuf9(&mut self) -> WUF9_W {
        WUF9_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P10"]
    #[inline(always)]
    pub fn wuf10(&mut self) -> WUF10_W {
        WUF10_W { w: self }
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P11"]
    #[inline(always)]
    pub fn wuf11(&mut self) -> WUF11_W {
        WUF11_W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P12"]
    #[inline(always)]
    pub fn wuf12(&mut self) -> WUF12_W {
        WUF12_W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P13"]
    #[inline(always)]
    pub fn wuf13(&mut self) -> WUF13_W {
        WUF13_W { w: self }
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P14"]
    #[inline(always)]
    pub fn wuf14(&mut self) -> WUF14_W {
        WUF14_W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P15"]
    #[inline(always)]
    pub fn wuf15(&mut self) -> WUF15_W {
        WUF15_W { w: self }
    }
}
