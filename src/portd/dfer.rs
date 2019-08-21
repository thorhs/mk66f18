#[doc = "Reader of register DFER"]
pub type R = crate::R<u32, super::DFER>;
#[doc = "Writer for register DFER"]
pub type W = crate::W<u32, super::DFER>;
#[doc = "Register DFER `reset()`'s with value 0"]
impl crate::ResetValue for super::DFER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE0_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE0_A> for bool {
    #[inline(always)]
    fn from(variant: DFE0_A) -> Self {
        match variant {
            DFE0_A::_0 => false,
            DFE0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE0`"]
pub type DFE0_R = crate::R<bool, DFE0_A>;
impl DFE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE0_A {
        match self.bits {
            false => DFE0_A::_0,
            true => DFE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE0_A::_1
    }
}
#[doc = "Write proxy for field `DFE0`"]
pub struct DFE0_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE0_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE0_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE1_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE1_A> for bool {
    #[inline(always)]
    fn from(variant: DFE1_A) -> Self {
        match variant {
            DFE1_A::_0 => false,
            DFE1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE1`"]
pub type DFE1_R = crate::R<bool, DFE1_A>;
impl DFE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE1_A {
        match self.bits {
            false => DFE1_A::_0,
            true => DFE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE1_A::_1
    }
}
#[doc = "Write proxy for field `DFE1`"]
pub struct DFE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE1_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE2_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE2_A> for bool {
    #[inline(always)]
    fn from(variant: DFE2_A) -> Self {
        match variant {
            DFE2_A::_0 => false,
            DFE2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE2`"]
pub type DFE2_R = crate::R<bool, DFE2_A>;
impl DFE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE2_A {
        match self.bits {
            false => DFE2_A::_0,
            true => DFE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE2_A::_1
    }
}
#[doc = "Write proxy for field `DFE2`"]
pub struct DFE2_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE2_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE3_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE3_A> for bool {
    #[inline(always)]
    fn from(variant: DFE3_A) -> Self {
        match variant {
            DFE3_A::_0 => false,
            DFE3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE3`"]
pub type DFE3_R = crate::R<bool, DFE3_A>;
impl DFE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE3_A {
        match self.bits {
            false => DFE3_A::_0,
            true => DFE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE3_A::_1
    }
}
#[doc = "Write proxy for field `DFE3`"]
pub struct DFE3_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE3_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE4_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE4_A> for bool {
    #[inline(always)]
    fn from(variant: DFE4_A) -> Self {
        match variant {
            DFE4_A::_0 => false,
            DFE4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE4`"]
pub type DFE4_R = crate::R<bool, DFE4_A>;
impl DFE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE4_A {
        match self.bits {
            false => DFE4_A::_0,
            true => DFE4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE4_A::_1
    }
}
#[doc = "Write proxy for field `DFE4`"]
pub struct DFE4_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE4_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE4_A::_1)
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
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE5_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE5_A> for bool {
    #[inline(always)]
    fn from(variant: DFE5_A) -> Self {
        match variant {
            DFE5_A::_0 => false,
            DFE5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE5`"]
pub type DFE5_R = crate::R<bool, DFE5_A>;
impl DFE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE5_A {
        match self.bits {
            false => DFE5_A::_0,
            true => DFE5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE5_A::_1
    }
}
#[doc = "Write proxy for field `DFE5`"]
pub struct DFE5_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE5_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE6_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE6_A> for bool {
    #[inline(always)]
    fn from(variant: DFE6_A) -> Self {
        match variant {
            DFE6_A::_0 => false,
            DFE6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE6`"]
pub type DFE6_R = crate::R<bool, DFE6_A>;
impl DFE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE6_A {
        match self.bits {
            false => DFE6_A::_0,
            true => DFE6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE6_A::_1
    }
}
#[doc = "Write proxy for field `DFE6`"]
pub struct DFE6_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE6_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE7_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE7_A> for bool {
    #[inline(always)]
    fn from(variant: DFE7_A) -> Self {
        match variant {
            DFE7_A::_0 => false,
            DFE7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE7`"]
pub type DFE7_R = crate::R<bool, DFE7_A>;
impl DFE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE7_A {
        match self.bits {
            false => DFE7_A::_0,
            true => DFE7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE7_A::_1
    }
}
#[doc = "Write proxy for field `DFE7`"]
pub struct DFE7_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE7_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE7_A::_1)
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
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE8_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE8_A> for bool {
    #[inline(always)]
    fn from(variant: DFE8_A) -> Self {
        match variant {
            DFE8_A::_0 => false,
            DFE8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE8`"]
pub type DFE8_R = crate::R<bool, DFE8_A>;
impl DFE8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE8_A {
        match self.bits {
            false => DFE8_A::_0,
            true => DFE8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE8_A::_1
    }
}
#[doc = "Write proxy for field `DFE8`"]
pub struct DFE8_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE8_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE8_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE9_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE9_A> for bool {
    #[inline(always)]
    fn from(variant: DFE9_A) -> Self {
        match variant {
            DFE9_A::_0 => false,
            DFE9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE9`"]
pub type DFE9_R = crate::R<bool, DFE9_A>;
impl DFE9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE9_A {
        match self.bits {
            false => DFE9_A::_0,
            true => DFE9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE9_A::_1
    }
}
#[doc = "Write proxy for field `DFE9`"]
pub struct DFE9_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE9_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE9_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE10_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE10_A> for bool {
    #[inline(always)]
    fn from(variant: DFE10_A) -> Self {
        match variant {
            DFE10_A::_0 => false,
            DFE10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE10`"]
pub type DFE10_R = crate::R<bool, DFE10_A>;
impl DFE10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE10_A {
        match self.bits {
            false => DFE10_A::_0,
            true => DFE10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE10_A::_1
    }
}
#[doc = "Write proxy for field `DFE10`"]
pub struct DFE10_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE10_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE10_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE11_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE11_A> for bool {
    #[inline(always)]
    fn from(variant: DFE11_A) -> Self {
        match variant {
            DFE11_A::_0 => false,
            DFE11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE11`"]
pub type DFE11_R = crate::R<bool, DFE11_A>;
impl DFE11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE11_A {
        match self.bits {
            false => DFE11_A::_0,
            true => DFE11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE11_A::_1
    }
}
#[doc = "Write proxy for field `DFE11`"]
pub struct DFE11_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE11_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE11_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE12_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE12_A> for bool {
    #[inline(always)]
    fn from(variant: DFE12_A) -> Self {
        match variant {
            DFE12_A::_0 => false,
            DFE12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE12`"]
pub type DFE12_R = crate::R<bool, DFE12_A>;
impl DFE12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE12_A {
        match self.bits {
            false => DFE12_A::_0,
            true => DFE12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE12_A::_1
    }
}
#[doc = "Write proxy for field `DFE12`"]
pub struct DFE12_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE12_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE12_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE13_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE13_A> for bool {
    #[inline(always)]
    fn from(variant: DFE13_A) -> Self {
        match variant {
            DFE13_A::_0 => false,
            DFE13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE13`"]
pub type DFE13_R = crate::R<bool, DFE13_A>;
impl DFE13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE13_A {
        match self.bits {
            false => DFE13_A::_0,
            true => DFE13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE13_A::_1
    }
}
#[doc = "Write proxy for field `DFE13`"]
pub struct DFE13_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE13_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE13_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE14_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE14_A> for bool {
    #[inline(always)]
    fn from(variant: DFE14_A) -> Self {
        match variant {
            DFE14_A::_0 => false,
            DFE14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE14`"]
pub type DFE14_R = crate::R<bool, DFE14_A>;
impl DFE14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE14_A {
        match self.bits {
            false => DFE14_A::_0,
            true => DFE14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE14_A::_1
    }
}
#[doc = "Write proxy for field `DFE14`"]
pub struct DFE14_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE14_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE14_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE15_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE15_A> for bool {
    #[inline(always)]
    fn from(variant: DFE15_A) -> Self {
        match variant {
            DFE15_A::_0 => false,
            DFE15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE15`"]
pub type DFE15_R = crate::R<bool, DFE15_A>;
impl DFE15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE15_A {
        match self.bits {
            false => DFE15_A::_0,
            true => DFE15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE15_A::_1
    }
}
#[doc = "Write proxy for field `DFE15`"]
pub struct DFE15_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE15_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE15_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE16_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE16_A> for bool {
    #[inline(always)]
    fn from(variant: DFE16_A) -> Self {
        match variant {
            DFE16_A::_0 => false,
            DFE16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE16`"]
pub type DFE16_R = crate::R<bool, DFE16_A>;
impl DFE16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE16_A {
        match self.bits {
            false => DFE16_A::_0,
            true => DFE16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE16_A::_1
    }
}
#[doc = "Write proxy for field `DFE16`"]
pub struct DFE16_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE16_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE16_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE17_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE17_A> for bool {
    #[inline(always)]
    fn from(variant: DFE17_A) -> Self {
        match variant {
            DFE17_A::_0 => false,
            DFE17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE17`"]
pub type DFE17_R = crate::R<bool, DFE17_A>;
impl DFE17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE17_A {
        match self.bits {
            false => DFE17_A::_0,
            true => DFE17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE17_A::_1
    }
}
#[doc = "Write proxy for field `DFE17`"]
pub struct DFE17_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE17_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE17_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE18_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE18_A> for bool {
    #[inline(always)]
    fn from(variant: DFE18_A) -> Self {
        match variant {
            DFE18_A::_0 => false,
            DFE18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE18`"]
pub type DFE18_R = crate::R<bool, DFE18_A>;
impl DFE18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE18_A {
        match self.bits {
            false => DFE18_A::_0,
            true => DFE18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE18_A::_1
    }
}
#[doc = "Write proxy for field `DFE18`"]
pub struct DFE18_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE18_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE18_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE19_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE19_A> for bool {
    #[inline(always)]
    fn from(variant: DFE19_A) -> Self {
        match variant {
            DFE19_A::_0 => false,
            DFE19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE19`"]
pub type DFE19_R = crate::R<bool, DFE19_A>;
impl DFE19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE19_A {
        match self.bits {
            false => DFE19_A::_0,
            true => DFE19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE19_A::_1
    }
}
#[doc = "Write proxy for field `DFE19`"]
pub struct DFE19_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE19_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE19_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE20_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE20_A> for bool {
    #[inline(always)]
    fn from(variant: DFE20_A) -> Self {
        match variant {
            DFE20_A::_0 => false,
            DFE20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE20`"]
pub type DFE20_R = crate::R<bool, DFE20_A>;
impl DFE20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE20_A {
        match self.bits {
            false => DFE20_A::_0,
            true => DFE20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE20_A::_1
    }
}
#[doc = "Write proxy for field `DFE20`"]
pub struct DFE20_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE20_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE20_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE21_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE21_A> for bool {
    #[inline(always)]
    fn from(variant: DFE21_A) -> Self {
        match variant {
            DFE21_A::_0 => false,
            DFE21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE21`"]
pub type DFE21_R = crate::R<bool, DFE21_A>;
impl DFE21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE21_A {
        match self.bits {
            false => DFE21_A::_0,
            true => DFE21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE21_A::_1
    }
}
#[doc = "Write proxy for field `DFE21`"]
pub struct DFE21_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE21_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE21_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE22_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE22_A> for bool {
    #[inline(always)]
    fn from(variant: DFE22_A) -> Self {
        match variant {
            DFE22_A::_0 => false,
            DFE22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE22`"]
pub type DFE22_R = crate::R<bool, DFE22_A>;
impl DFE22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE22_A {
        match self.bits {
            false => DFE22_A::_0,
            true => DFE22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE22_A::_1
    }
}
#[doc = "Write proxy for field `DFE22`"]
pub struct DFE22_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE22_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE22_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE23_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE23_A> for bool {
    #[inline(always)]
    fn from(variant: DFE23_A) -> Self {
        match variant {
            DFE23_A::_0 => false,
            DFE23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE23`"]
pub type DFE23_R = crate::R<bool, DFE23_A>;
impl DFE23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE23_A {
        match self.bits {
            false => DFE23_A::_0,
            true => DFE23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE23_A::_1
    }
}
#[doc = "Write proxy for field `DFE23`"]
pub struct DFE23_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE23_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE23_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE24_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE24_A> for bool {
    #[inline(always)]
    fn from(variant: DFE24_A) -> Self {
        match variant {
            DFE24_A::_0 => false,
            DFE24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE24`"]
pub type DFE24_R = crate::R<bool, DFE24_A>;
impl DFE24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE24_A {
        match self.bits {
            false => DFE24_A::_0,
            true => DFE24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE24_A::_1
    }
}
#[doc = "Write proxy for field `DFE24`"]
pub struct DFE24_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE24_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE24_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE25_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE25_A> for bool {
    #[inline(always)]
    fn from(variant: DFE25_A) -> Self {
        match variant {
            DFE25_A::_0 => false,
            DFE25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE25`"]
pub type DFE25_R = crate::R<bool, DFE25_A>;
impl DFE25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE25_A {
        match self.bits {
            false => DFE25_A::_0,
            true => DFE25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE25_A::_1
    }
}
#[doc = "Write proxy for field `DFE25`"]
pub struct DFE25_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE25_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE25_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE26_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE26_A> for bool {
    #[inline(always)]
    fn from(variant: DFE26_A) -> Self {
        match variant {
            DFE26_A::_0 => false,
            DFE26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE26`"]
pub type DFE26_R = crate::R<bool, DFE26_A>;
impl DFE26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE26_A {
        match self.bits {
            false => DFE26_A::_0,
            true => DFE26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE26_A::_1
    }
}
#[doc = "Write proxy for field `DFE26`"]
pub struct DFE26_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE26_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE26_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE27_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE27_A> for bool {
    #[inline(always)]
    fn from(variant: DFE27_A) -> Self {
        match variant {
            DFE27_A::_0 => false,
            DFE27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE27`"]
pub type DFE27_R = crate::R<bool, DFE27_A>;
impl DFE27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE27_A {
        match self.bits {
            false => DFE27_A::_0,
            true => DFE27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE27_A::_1
    }
}
#[doc = "Write proxy for field `DFE27`"]
pub struct DFE27_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE27_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE27_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE28_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE28_A> for bool {
    #[inline(always)]
    fn from(variant: DFE28_A) -> Self {
        match variant {
            DFE28_A::_0 => false,
            DFE28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE28`"]
pub type DFE28_R = crate::R<bool, DFE28_A>;
impl DFE28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE28_A {
        match self.bits {
            false => DFE28_A::_0,
            true => DFE28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE28_A::_1
    }
}
#[doc = "Write proxy for field `DFE28`"]
pub struct DFE28_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE28_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE28_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE29_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE29_A> for bool {
    #[inline(always)]
    fn from(variant: DFE29_A) -> Self {
        match variant {
            DFE29_A::_0 => false,
            DFE29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE29`"]
pub type DFE29_R = crate::R<bool, DFE29_A>;
impl DFE29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE29_A {
        match self.bits {
            false => DFE29_A::_0,
            true => DFE29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE29_A::_1
    }
}
#[doc = "Write proxy for field `DFE29`"]
pub struct DFE29_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE29_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE29_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE30_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE30_A> for bool {
    #[inline(always)]
    fn from(variant: DFE30_A) -> Self {
        match variant {
            DFE30_A::_0 => false,
            DFE30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE30`"]
pub type DFE30_R = crate::R<bool, DFE30_A>;
impl DFE30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE30_A {
        match self.bits {
            false => DFE30_A::_0,
            true => DFE30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE30_A::_1
    }
}
#[doc = "Write proxy for field `DFE30`"]
pub struct DFE30_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE30_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE30_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFE31_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl From<DFE31_A> for bool {
    #[inline(always)]
    fn from(variant: DFE31_A) -> Self {
        match variant {
            DFE31_A::_0 => false,
            DFE31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DFE31`"]
pub type DFE31_R = crate::R<bool, DFE31_A>;
impl DFE31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFE31_A {
        match self.bits {
            false => DFE31_A::_0,
            true => DFE31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE31_A::_1
    }
}
#[doc = "Write proxy for field `DFE31`"]
pub struct DFE31_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE31_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE31_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe0(&self) -> DFE0_R {
        DFE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe1(&self) -> DFE1_R {
        DFE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe2(&self) -> DFE2_R {
        DFE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe3(&self) -> DFE3_R {
        DFE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe4(&self) -> DFE4_R {
        DFE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe5(&self) -> DFE5_R {
        DFE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe6(&self) -> DFE6_R {
        DFE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe7(&self) -> DFE7_R {
        DFE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe8(&self) -> DFE8_R {
        DFE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe9(&self) -> DFE9_R {
        DFE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe10(&self) -> DFE10_R {
        DFE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe11(&self) -> DFE11_R {
        DFE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe12(&self) -> DFE12_R {
        DFE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe13(&self) -> DFE13_R {
        DFE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe14(&self) -> DFE14_R {
        DFE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe15(&self) -> DFE15_R {
        DFE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe16(&self) -> DFE16_R {
        DFE16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe17(&self) -> DFE17_R {
        DFE17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe18(&self) -> DFE18_R {
        DFE18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe19(&self) -> DFE19_R {
        DFE19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe20(&self) -> DFE20_R {
        DFE20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe21(&self) -> DFE21_R {
        DFE21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe22(&self) -> DFE22_R {
        DFE22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe23(&self) -> DFE23_R {
        DFE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe24(&self) -> DFE24_R {
        DFE24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe25(&self) -> DFE25_R {
        DFE25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe26(&self) -> DFE26_R {
        DFE26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe27(&self) -> DFE27_R {
        DFE27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe28(&self) -> DFE28_R {
        DFE28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe29(&self) -> DFE29_R {
        DFE29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe30(&self) -> DFE30_R {
        DFE30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe31(&self) -> DFE31_R {
        DFE31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe0(&mut self) -> DFE0_W {
        DFE0_W { w: self }
    }
    #[doc = "Bit 1 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe1(&mut self) -> DFE1_W {
        DFE1_W { w: self }
    }
    #[doc = "Bit 2 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe2(&mut self) -> DFE2_W {
        DFE2_W { w: self }
    }
    #[doc = "Bit 3 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe3(&mut self) -> DFE3_W {
        DFE3_W { w: self }
    }
    #[doc = "Bit 4 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe4(&mut self) -> DFE4_W {
        DFE4_W { w: self }
    }
    #[doc = "Bit 5 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe5(&mut self) -> DFE5_W {
        DFE5_W { w: self }
    }
    #[doc = "Bit 6 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe6(&mut self) -> DFE6_W {
        DFE6_W { w: self }
    }
    #[doc = "Bit 7 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe7(&mut self) -> DFE7_W {
        DFE7_W { w: self }
    }
    #[doc = "Bit 8 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe8(&mut self) -> DFE8_W {
        DFE8_W { w: self }
    }
    #[doc = "Bit 9 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe9(&mut self) -> DFE9_W {
        DFE9_W { w: self }
    }
    #[doc = "Bit 10 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe10(&mut self) -> DFE10_W {
        DFE10_W { w: self }
    }
    #[doc = "Bit 11 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe11(&mut self) -> DFE11_W {
        DFE11_W { w: self }
    }
    #[doc = "Bit 12 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe12(&mut self) -> DFE12_W {
        DFE12_W { w: self }
    }
    #[doc = "Bit 13 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe13(&mut self) -> DFE13_W {
        DFE13_W { w: self }
    }
    #[doc = "Bit 14 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe14(&mut self) -> DFE14_W {
        DFE14_W { w: self }
    }
    #[doc = "Bit 15 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe15(&mut self) -> DFE15_W {
        DFE15_W { w: self }
    }
    #[doc = "Bit 16 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe16(&mut self) -> DFE16_W {
        DFE16_W { w: self }
    }
    #[doc = "Bit 17 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe17(&mut self) -> DFE17_W {
        DFE17_W { w: self }
    }
    #[doc = "Bit 18 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe18(&mut self) -> DFE18_W {
        DFE18_W { w: self }
    }
    #[doc = "Bit 19 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe19(&mut self) -> DFE19_W {
        DFE19_W { w: self }
    }
    #[doc = "Bit 20 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe20(&mut self) -> DFE20_W {
        DFE20_W { w: self }
    }
    #[doc = "Bit 21 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe21(&mut self) -> DFE21_W {
        DFE21_W { w: self }
    }
    #[doc = "Bit 22 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe22(&mut self) -> DFE22_W {
        DFE22_W { w: self }
    }
    #[doc = "Bit 23 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe23(&mut self) -> DFE23_W {
        DFE23_W { w: self }
    }
    #[doc = "Bit 24 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe24(&mut self) -> DFE24_W {
        DFE24_W { w: self }
    }
    #[doc = "Bit 25 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe25(&mut self) -> DFE25_W {
        DFE25_W { w: self }
    }
    #[doc = "Bit 26 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe26(&mut self) -> DFE26_W {
        DFE26_W { w: self }
    }
    #[doc = "Bit 27 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe27(&mut self) -> DFE27_W {
        DFE27_W { w: self }
    }
    #[doc = "Bit 28 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe28(&mut self) -> DFE28_W {
        DFE28_W { w: self }
    }
    #[doc = "Bit 29 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe29(&mut self) -> DFE29_W {
        DFE29_W { w: self }
    }
    #[doc = "Bit 30 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe30(&mut self) -> DFE30_W {
        DFE30_W { w: self }
    }
    #[doc = "Bit 31 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe31(&mut self) -> DFE31_W {
        DFE31_W { w: self }
    }
}
