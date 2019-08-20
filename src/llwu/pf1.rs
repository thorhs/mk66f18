#[doc = "Reader of register PF1"]
pub type R = crate::R<u8, super::PF1>;
#[doc = "Writer for register PF1"]
pub type W = crate::W<u8, super::PF1>;
#[doc = "Register PF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PF1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Wakeup Flag For LLWU_P0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF0_A {
    #[doc = "0: LLWU_P0 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P0 input was a wakeup source"]
    _1,
}
impl From<WUF0_A> for bool {
    #[inline(always)]
    fn from(variant: WUF0_A) -> Self {
        match variant {
            WUF0_A::_0 => false,
            WUF0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF0`"]
pub type WUF0_R = crate::R<bool, WUF0_A>;
impl WUF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF0_A {
        match self.bits {
            false => WUF0_A::_0,
            true => WUF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF0_A::_1
    }
}
#[doc = "Write proxy for field `WUF0`"]
pub struct WUF0_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P0 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF0_A::_0)
    }
    #[doc = "LLWU_P0 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF0_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF1_A {
    #[doc = "0: LLWU_P1 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P1 input was a wakeup source"]
    _1,
}
impl From<WUF1_A> for bool {
    #[inline(always)]
    fn from(variant: WUF1_A) -> Self {
        match variant {
            WUF1_A::_0 => false,
            WUF1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF1`"]
pub type WUF1_R = crate::R<bool, WUF1_A>;
impl WUF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF1_A {
        match self.bits {
            false => WUF1_A::_0,
            true => WUF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF1_A::_1
    }
}
#[doc = "Write proxy for field `WUF1`"]
pub struct WUF1_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P1 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF1_A::_0)
    }
    #[doc = "LLWU_P1 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF1_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF2_A {
    #[doc = "0: LLWU_P2 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P2 input was a wakeup source"]
    _1,
}
impl From<WUF2_A> for bool {
    #[inline(always)]
    fn from(variant: WUF2_A) -> Self {
        match variant {
            WUF2_A::_0 => false,
            WUF2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF2`"]
pub type WUF2_R = crate::R<bool, WUF2_A>;
impl WUF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF2_A {
        match self.bits {
            false => WUF2_A::_0,
            true => WUF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF2_A::_1
    }
}
#[doc = "Write proxy for field `WUF2`"]
pub struct WUF2_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P2 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF2_A::_0)
    }
    #[doc = "LLWU_P2 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF2_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF3_A {
    #[doc = "0: LLWU_P3 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P3 input was a wakeup source"]
    _1,
}
impl From<WUF3_A> for bool {
    #[inline(always)]
    fn from(variant: WUF3_A) -> Self {
        match variant {
            WUF3_A::_0 => false,
            WUF3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF3`"]
pub type WUF3_R = crate::R<bool, WUF3_A>;
impl WUF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF3_A {
        match self.bits {
            false => WUF3_A::_0,
            true => WUF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF3_A::_1
    }
}
#[doc = "Write proxy for field `WUF3`"]
pub struct WUF3_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P3 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF3_A::_0)
    }
    #[doc = "LLWU_P3 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF3_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF4_A {
    #[doc = "0: LLWU_P4 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P4 input was a wakeup source"]
    _1,
}
impl From<WUF4_A> for bool {
    #[inline(always)]
    fn from(variant: WUF4_A) -> Self {
        match variant {
            WUF4_A::_0 => false,
            WUF4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF4`"]
pub type WUF4_R = crate::R<bool, WUF4_A>;
impl WUF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF4_A {
        match self.bits {
            false => WUF4_A::_0,
            true => WUF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF4_A::_1
    }
}
#[doc = "Write proxy for field `WUF4`"]
pub struct WUF4_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P4 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF4_A::_0)
    }
    #[doc = "LLWU_P4 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF4_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF5_A {
    #[doc = "0: LLWU_P5 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P5 input was a wakeup source"]
    _1,
}
impl From<WUF5_A> for bool {
    #[inline(always)]
    fn from(variant: WUF5_A) -> Self {
        match variant {
            WUF5_A::_0 => false,
            WUF5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF5`"]
pub type WUF5_R = crate::R<bool, WUF5_A>;
impl WUF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF5_A {
        match self.bits {
            false => WUF5_A::_0,
            true => WUF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF5_A::_1
    }
}
#[doc = "Write proxy for field `WUF5`"]
pub struct WUF5_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P5 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF5_A::_0)
    }
    #[doc = "LLWU_P5 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF5_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF6_A {
    #[doc = "0: LLWU_P6 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P6 input was a wakeup source"]
    _1,
}
impl From<WUF6_A> for bool {
    #[inline(always)]
    fn from(variant: WUF6_A) -> Self {
        match variant {
            WUF6_A::_0 => false,
            WUF6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF6`"]
pub type WUF6_R = crate::R<bool, WUF6_A>;
impl WUF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF6_A {
        match self.bits {
            false => WUF6_A::_0,
            true => WUF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF6_A::_1
    }
}
#[doc = "Write proxy for field `WUF6`"]
pub struct WUF6_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P6 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF6_A::_0)
    }
    #[doc = "LLWU_P6 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF6_A::_1)
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
#[doc = "Wakeup Flag For LLWU_P7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUF7_A {
    #[doc = "0: LLWU_P7 input was not a wakeup source"]
    _0,
    #[doc = "1: LLWU_P7 input was a wakeup source"]
    _1,
}
impl From<WUF7_A> for bool {
    #[inline(always)]
    fn from(variant: WUF7_A) -> Self {
        match variant {
            WUF7_A::_0 => false,
            WUF7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WUF7`"]
pub type WUF7_R = crate::R<bool, WUF7_A>;
impl WUF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUF7_A {
        match self.bits {
            false => WUF7_A::_0,
            true => WUF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUF7_A::_1
    }
}
#[doc = "Write proxy for field `WUF7`"]
pub struct WUF7_W<'a> {
    w: &'a mut W,
}
impl<'a> WUF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUF7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LLWU_P7 input was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUF7_A::_0)
    }
    #[doc = "LLWU_P7 input was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUF7_A::_1)
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
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P0"]
    #[inline(always)]
    pub fn wuf0(&self) -> WUF0_R {
        WUF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P1"]
    #[inline(always)]
    pub fn wuf1(&self) -> WUF1_R {
        WUF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P2"]
    #[inline(always)]
    pub fn wuf2(&self) -> WUF2_R {
        WUF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P3"]
    #[inline(always)]
    pub fn wuf3(&self) -> WUF3_R {
        WUF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P4"]
    #[inline(always)]
    pub fn wuf4(&self) -> WUF4_R {
        WUF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P5"]
    #[inline(always)]
    pub fn wuf5(&self) -> WUF5_R {
        WUF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P6"]
    #[inline(always)]
    pub fn wuf6(&self) -> WUF6_R {
        WUF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P7"]
    #[inline(always)]
    pub fn wuf7(&self) -> WUF7_R {
        WUF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Flag For LLWU_P0"]
    #[inline(always)]
    pub fn wuf0(&mut self) -> WUF0_W {
        WUF0_W { w: self }
    }
    #[doc = "Bit 1 - Wakeup Flag For LLWU_P1"]
    #[inline(always)]
    pub fn wuf1(&mut self) -> WUF1_W {
        WUF1_W { w: self }
    }
    #[doc = "Bit 2 - Wakeup Flag For LLWU_P2"]
    #[inline(always)]
    pub fn wuf2(&mut self) -> WUF2_W {
        WUF2_W { w: self }
    }
    #[doc = "Bit 3 - Wakeup Flag For LLWU_P3"]
    #[inline(always)]
    pub fn wuf3(&mut self) -> WUF3_W {
        WUF3_W { w: self }
    }
    #[doc = "Bit 4 - Wakeup Flag For LLWU_P4"]
    #[inline(always)]
    pub fn wuf4(&mut self) -> WUF4_W {
        WUF4_W { w: self }
    }
    #[doc = "Bit 5 - Wakeup Flag For LLWU_P5"]
    #[inline(always)]
    pub fn wuf5(&mut self) -> WUF5_W {
        WUF5_W { w: self }
    }
    #[doc = "Bit 6 - Wakeup Flag For LLWU_P6"]
    #[inline(always)]
    pub fn wuf6(&mut self) -> WUF6_W {
        WUF6_W { w: self }
    }
    #[doc = "Bit 7 - Wakeup Flag For LLWU_P7"]
    #[inline(always)]
    pub fn wuf7(&mut self) -> WUF7_W {
        WUF7_W { w: self }
    }
}
