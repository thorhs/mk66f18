#[doc = "Reader of register PDDR"]
pub type R = crate::R<u32, super::PDDR>;
#[doc = "Writer for register PDDR"]
pub type W = crate::W<u32, super::PDDR>;
#[doc = "Register PDDR `reset()`'s with value 0"]
impl crate::ResetValue for super::PDDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD0_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD0_A> for bool {
    #[inline(always)]
    fn from(variant: PDD0_A) -> Self {
        match variant {
            PDD0_A::_0 => false,
            PDD0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD0`"]
pub type PDD0_R = crate::R<bool, PDD0_A>;
impl PDD0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD0_A {
        match self.bits {
            false => PDD0_A::_0,
            true => PDD0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD0_A::_1
    }
}
#[doc = "Write proxy for field `PDD0`"]
pub struct PDD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD0_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD0_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD1_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD1_A> for bool {
    #[inline(always)]
    fn from(variant: PDD1_A) -> Self {
        match variant {
            PDD1_A::_0 => false,
            PDD1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD1`"]
pub type PDD1_R = crate::R<bool, PDD1_A>;
impl PDD1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD1_A {
        match self.bits {
            false => PDD1_A::_0,
            true => PDD1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD1_A::_1
    }
}
#[doc = "Write proxy for field `PDD1`"]
pub struct PDD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD1_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD1_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD2_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD2_A> for bool {
    #[inline(always)]
    fn from(variant: PDD2_A) -> Self {
        match variant {
            PDD2_A::_0 => false,
            PDD2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD2`"]
pub type PDD2_R = crate::R<bool, PDD2_A>;
impl PDD2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD2_A {
        match self.bits {
            false => PDD2_A::_0,
            true => PDD2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD2_A::_1
    }
}
#[doc = "Write proxy for field `PDD2`"]
pub struct PDD2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD2_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD2_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD3_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD3_A> for bool {
    #[inline(always)]
    fn from(variant: PDD3_A) -> Self {
        match variant {
            PDD3_A::_0 => false,
            PDD3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD3`"]
pub type PDD3_R = crate::R<bool, PDD3_A>;
impl PDD3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD3_A {
        match self.bits {
            false => PDD3_A::_0,
            true => PDD3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD3_A::_1
    }
}
#[doc = "Write proxy for field `PDD3`"]
pub struct PDD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD3_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD3_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD4_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD4_A> for bool {
    #[inline(always)]
    fn from(variant: PDD4_A) -> Self {
        match variant {
            PDD4_A::_0 => false,
            PDD4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD4`"]
pub type PDD4_R = crate::R<bool, PDD4_A>;
impl PDD4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD4_A {
        match self.bits {
            false => PDD4_A::_0,
            true => PDD4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD4_A::_1
    }
}
#[doc = "Write proxy for field `PDD4`"]
pub struct PDD4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD4_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD4_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD5_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD5_A> for bool {
    #[inline(always)]
    fn from(variant: PDD5_A) -> Self {
        match variant {
            PDD5_A::_0 => false,
            PDD5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD5`"]
pub type PDD5_R = crate::R<bool, PDD5_A>;
impl PDD5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD5_A {
        match self.bits {
            false => PDD5_A::_0,
            true => PDD5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD5_A::_1
    }
}
#[doc = "Write proxy for field `PDD5`"]
pub struct PDD5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD5_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD5_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD6_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD6_A> for bool {
    #[inline(always)]
    fn from(variant: PDD6_A) -> Self {
        match variant {
            PDD6_A::_0 => false,
            PDD6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD6`"]
pub type PDD6_R = crate::R<bool, PDD6_A>;
impl PDD6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD6_A {
        match self.bits {
            false => PDD6_A::_0,
            true => PDD6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD6_A::_1
    }
}
#[doc = "Write proxy for field `PDD6`"]
pub struct PDD6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD6_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD6_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD7_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD7_A> for bool {
    #[inline(always)]
    fn from(variant: PDD7_A) -> Self {
        match variant {
            PDD7_A::_0 => false,
            PDD7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD7`"]
pub type PDD7_R = crate::R<bool, PDD7_A>;
impl PDD7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD7_A {
        match self.bits {
            false => PDD7_A::_0,
            true => PDD7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD7_A::_1
    }
}
#[doc = "Write proxy for field `PDD7`"]
pub struct PDD7_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD7_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD7_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD8_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD8_A> for bool {
    #[inline(always)]
    fn from(variant: PDD8_A) -> Self {
        match variant {
            PDD8_A::_0 => false,
            PDD8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD8`"]
pub type PDD8_R = crate::R<bool, PDD8_A>;
impl PDD8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD8_A {
        match self.bits {
            false => PDD8_A::_0,
            true => PDD8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD8_A::_1
    }
}
#[doc = "Write proxy for field `PDD8`"]
pub struct PDD8_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD8_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD8_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD9_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD9_A> for bool {
    #[inline(always)]
    fn from(variant: PDD9_A) -> Self {
        match variant {
            PDD9_A::_0 => false,
            PDD9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD9`"]
pub type PDD9_R = crate::R<bool, PDD9_A>;
impl PDD9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD9_A {
        match self.bits {
            false => PDD9_A::_0,
            true => PDD9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD9_A::_1
    }
}
#[doc = "Write proxy for field `PDD9`"]
pub struct PDD9_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD9_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD9_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD10_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD10_A> for bool {
    #[inline(always)]
    fn from(variant: PDD10_A) -> Self {
        match variant {
            PDD10_A::_0 => false,
            PDD10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD10`"]
pub type PDD10_R = crate::R<bool, PDD10_A>;
impl PDD10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD10_A {
        match self.bits {
            false => PDD10_A::_0,
            true => PDD10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD10_A::_1
    }
}
#[doc = "Write proxy for field `PDD10`"]
pub struct PDD10_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD10_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD10_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD11_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD11_A> for bool {
    #[inline(always)]
    fn from(variant: PDD11_A) -> Self {
        match variant {
            PDD11_A::_0 => false,
            PDD11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD11`"]
pub type PDD11_R = crate::R<bool, PDD11_A>;
impl PDD11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD11_A {
        match self.bits {
            false => PDD11_A::_0,
            true => PDD11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD11_A::_1
    }
}
#[doc = "Write proxy for field `PDD11`"]
pub struct PDD11_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD11_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD11_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD12_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD12_A> for bool {
    #[inline(always)]
    fn from(variant: PDD12_A) -> Self {
        match variant {
            PDD12_A::_0 => false,
            PDD12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD12`"]
pub type PDD12_R = crate::R<bool, PDD12_A>;
impl PDD12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD12_A {
        match self.bits {
            false => PDD12_A::_0,
            true => PDD12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD12_A::_1
    }
}
#[doc = "Write proxy for field `PDD12`"]
pub struct PDD12_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD12_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD12_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD13_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD13_A> for bool {
    #[inline(always)]
    fn from(variant: PDD13_A) -> Self {
        match variant {
            PDD13_A::_0 => false,
            PDD13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD13`"]
pub type PDD13_R = crate::R<bool, PDD13_A>;
impl PDD13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD13_A {
        match self.bits {
            false => PDD13_A::_0,
            true => PDD13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD13_A::_1
    }
}
#[doc = "Write proxy for field `PDD13`"]
pub struct PDD13_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD13_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD13_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD14_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD14_A> for bool {
    #[inline(always)]
    fn from(variant: PDD14_A) -> Self {
        match variant {
            PDD14_A::_0 => false,
            PDD14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD14`"]
pub type PDD14_R = crate::R<bool, PDD14_A>;
impl PDD14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD14_A {
        match self.bits {
            false => PDD14_A::_0,
            true => PDD14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD14_A::_1
    }
}
#[doc = "Write proxy for field `PDD14`"]
pub struct PDD14_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD14_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD14_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD15_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD15_A> for bool {
    #[inline(always)]
    fn from(variant: PDD15_A) -> Self {
        match variant {
            PDD15_A::_0 => false,
            PDD15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD15`"]
pub type PDD15_R = crate::R<bool, PDD15_A>;
impl PDD15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD15_A {
        match self.bits {
            false => PDD15_A::_0,
            true => PDD15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD15_A::_1
    }
}
#[doc = "Write proxy for field `PDD15`"]
pub struct PDD15_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD15_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD15_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD16_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD16_A> for bool {
    #[inline(always)]
    fn from(variant: PDD16_A) -> Self {
        match variant {
            PDD16_A::_0 => false,
            PDD16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD16`"]
pub type PDD16_R = crate::R<bool, PDD16_A>;
impl PDD16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD16_A {
        match self.bits {
            false => PDD16_A::_0,
            true => PDD16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD16_A::_1
    }
}
#[doc = "Write proxy for field `PDD16`"]
pub struct PDD16_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD16_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD16_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD17_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD17_A> for bool {
    #[inline(always)]
    fn from(variant: PDD17_A) -> Self {
        match variant {
            PDD17_A::_0 => false,
            PDD17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD17`"]
pub type PDD17_R = crate::R<bool, PDD17_A>;
impl PDD17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD17_A {
        match self.bits {
            false => PDD17_A::_0,
            true => PDD17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD17_A::_1
    }
}
#[doc = "Write proxy for field `PDD17`"]
pub struct PDD17_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD17_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD17_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD18_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD18_A> for bool {
    #[inline(always)]
    fn from(variant: PDD18_A) -> Self {
        match variant {
            PDD18_A::_0 => false,
            PDD18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD18`"]
pub type PDD18_R = crate::R<bool, PDD18_A>;
impl PDD18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD18_A {
        match self.bits {
            false => PDD18_A::_0,
            true => PDD18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD18_A::_1
    }
}
#[doc = "Write proxy for field `PDD18`"]
pub struct PDD18_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD18_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD18_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD19_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD19_A> for bool {
    #[inline(always)]
    fn from(variant: PDD19_A) -> Self {
        match variant {
            PDD19_A::_0 => false,
            PDD19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD19`"]
pub type PDD19_R = crate::R<bool, PDD19_A>;
impl PDD19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD19_A {
        match self.bits {
            false => PDD19_A::_0,
            true => PDD19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD19_A::_1
    }
}
#[doc = "Write proxy for field `PDD19`"]
pub struct PDD19_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD19_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD19_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD20_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD20_A> for bool {
    #[inline(always)]
    fn from(variant: PDD20_A) -> Self {
        match variant {
            PDD20_A::_0 => false,
            PDD20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD20`"]
pub type PDD20_R = crate::R<bool, PDD20_A>;
impl PDD20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD20_A {
        match self.bits {
            false => PDD20_A::_0,
            true => PDD20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD20_A::_1
    }
}
#[doc = "Write proxy for field `PDD20`"]
pub struct PDD20_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD20_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD20_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD21_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD21_A> for bool {
    #[inline(always)]
    fn from(variant: PDD21_A) -> Self {
        match variant {
            PDD21_A::_0 => false,
            PDD21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD21`"]
pub type PDD21_R = crate::R<bool, PDD21_A>;
impl PDD21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD21_A {
        match self.bits {
            false => PDD21_A::_0,
            true => PDD21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD21_A::_1
    }
}
#[doc = "Write proxy for field `PDD21`"]
pub struct PDD21_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD21_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD21_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD22_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD22_A> for bool {
    #[inline(always)]
    fn from(variant: PDD22_A) -> Self {
        match variant {
            PDD22_A::_0 => false,
            PDD22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD22`"]
pub type PDD22_R = crate::R<bool, PDD22_A>;
impl PDD22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD22_A {
        match self.bits {
            false => PDD22_A::_0,
            true => PDD22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD22_A::_1
    }
}
#[doc = "Write proxy for field `PDD22`"]
pub struct PDD22_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD22_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD22_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD23_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD23_A> for bool {
    #[inline(always)]
    fn from(variant: PDD23_A) -> Self {
        match variant {
            PDD23_A::_0 => false,
            PDD23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD23`"]
pub type PDD23_R = crate::R<bool, PDD23_A>;
impl PDD23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD23_A {
        match self.bits {
            false => PDD23_A::_0,
            true => PDD23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD23_A::_1
    }
}
#[doc = "Write proxy for field `PDD23`"]
pub struct PDD23_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD23_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD23_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD24_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD24_A> for bool {
    #[inline(always)]
    fn from(variant: PDD24_A) -> Self {
        match variant {
            PDD24_A::_0 => false,
            PDD24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD24`"]
pub type PDD24_R = crate::R<bool, PDD24_A>;
impl PDD24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD24_A {
        match self.bits {
            false => PDD24_A::_0,
            true => PDD24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD24_A::_1
    }
}
#[doc = "Write proxy for field `PDD24`"]
pub struct PDD24_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD24_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD24_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD25_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD25_A> for bool {
    #[inline(always)]
    fn from(variant: PDD25_A) -> Self {
        match variant {
            PDD25_A::_0 => false,
            PDD25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD25`"]
pub type PDD25_R = crate::R<bool, PDD25_A>;
impl PDD25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD25_A {
        match self.bits {
            false => PDD25_A::_0,
            true => PDD25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD25_A::_1
    }
}
#[doc = "Write proxy for field `PDD25`"]
pub struct PDD25_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD25_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD25_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD26_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD26_A> for bool {
    #[inline(always)]
    fn from(variant: PDD26_A) -> Self {
        match variant {
            PDD26_A::_0 => false,
            PDD26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD26`"]
pub type PDD26_R = crate::R<bool, PDD26_A>;
impl PDD26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD26_A {
        match self.bits {
            false => PDD26_A::_0,
            true => PDD26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD26_A::_1
    }
}
#[doc = "Write proxy for field `PDD26`"]
pub struct PDD26_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD26_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD26_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD27_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD27_A> for bool {
    #[inline(always)]
    fn from(variant: PDD27_A) -> Self {
        match variant {
            PDD27_A::_0 => false,
            PDD27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD27`"]
pub type PDD27_R = crate::R<bool, PDD27_A>;
impl PDD27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD27_A {
        match self.bits {
            false => PDD27_A::_0,
            true => PDD27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD27_A::_1
    }
}
#[doc = "Write proxy for field `PDD27`"]
pub struct PDD27_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD27_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD27_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD28_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD28_A> for bool {
    #[inline(always)]
    fn from(variant: PDD28_A) -> Self {
        match variant {
            PDD28_A::_0 => false,
            PDD28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD28`"]
pub type PDD28_R = crate::R<bool, PDD28_A>;
impl PDD28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD28_A {
        match self.bits {
            false => PDD28_A::_0,
            true => PDD28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD28_A::_1
    }
}
#[doc = "Write proxy for field `PDD28`"]
pub struct PDD28_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD28_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD28_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD29_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD29_A> for bool {
    #[inline(always)]
    fn from(variant: PDD29_A) -> Self {
        match variant {
            PDD29_A::_0 => false,
            PDD29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD29`"]
pub type PDD29_R = crate::R<bool, PDD29_A>;
impl PDD29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD29_A {
        match self.bits {
            false => PDD29_A::_0,
            true => PDD29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD29_A::_1
    }
}
#[doc = "Write proxy for field `PDD29`"]
pub struct PDD29_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD29_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD29_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD30_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD30_A> for bool {
    #[inline(always)]
    fn from(variant: PDD30_A) -> Self {
        match variant {
            PDD30_A::_0 => false,
            PDD30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD30`"]
pub type PDD30_R = crate::R<bool, PDD30_A>;
impl PDD30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD30_A {
        match self.bits {
            false => PDD30_A::_0,
            true => PDD30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD30_A::_1
    }
}
#[doc = "Write proxy for field `PDD30`"]
pub struct PDD30_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD30_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD30_A::_1)
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
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDD31_A {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    _0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    _1,
}
impl From<PDD31_A> for bool {
    #[inline(always)]
    fn from(variant: PDD31_A) -> Self {
        match variant {
            PDD31_A::_0 => false,
            PDD31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PDD31`"]
pub type PDD31_R = crate::R<bool, PDD31_A>;
impl PDD31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDD31_A {
        match self.bits {
            false => PDD31_A::_0,
            true => PDD31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDD31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDD31_A::_1
    }
}
#[doc = "Write proxy for field `PDD31`"]
pub struct PDD31_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDD31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDD31_A::_0)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDD31_A::_1)
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
    #[doc = "Bit 0 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd0(&self) -> PDD0_R {
        PDD0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd1(&self) -> PDD1_R {
        PDD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd2(&self) -> PDD2_R {
        PDD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd3(&self) -> PDD3_R {
        PDD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd4(&self) -> PDD4_R {
        PDD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd5(&self) -> PDD5_R {
        PDD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd6(&self) -> PDD6_R {
        PDD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd7(&self) -> PDD7_R {
        PDD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd8(&self) -> PDD8_R {
        PDD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd9(&self) -> PDD9_R {
        PDD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd10(&self) -> PDD10_R {
        PDD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd11(&self) -> PDD11_R {
        PDD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd12(&self) -> PDD12_R {
        PDD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd13(&self) -> PDD13_R {
        PDD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd14(&self) -> PDD14_R {
        PDD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd15(&self) -> PDD15_R {
        PDD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd16(&self) -> PDD16_R {
        PDD16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd17(&self) -> PDD17_R {
        PDD17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd18(&self) -> PDD18_R {
        PDD18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd19(&self) -> PDD19_R {
        PDD19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd20(&self) -> PDD20_R {
        PDD20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd21(&self) -> PDD21_R {
        PDD21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd22(&self) -> PDD22_R {
        PDD22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd23(&self) -> PDD23_R {
        PDD23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd24(&self) -> PDD24_R {
        PDD24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd25(&self) -> PDD25_R {
        PDD25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd26(&self) -> PDD26_R {
        PDD26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd27(&self) -> PDD27_R {
        PDD27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd28(&self) -> PDD28_R {
        PDD28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd29(&self) -> PDD29_R {
        PDD29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd30(&self) -> PDD30_R {
        PDD30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd31(&self) -> PDD31_R {
        PDD31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd0(&mut self) -> PDD0_W {
        PDD0_W { w: self }
    }
    #[doc = "Bit 1 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd1(&mut self) -> PDD1_W {
        PDD1_W { w: self }
    }
    #[doc = "Bit 2 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd2(&mut self) -> PDD2_W {
        PDD2_W { w: self }
    }
    #[doc = "Bit 3 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd3(&mut self) -> PDD3_W {
        PDD3_W { w: self }
    }
    #[doc = "Bit 4 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd4(&mut self) -> PDD4_W {
        PDD4_W { w: self }
    }
    #[doc = "Bit 5 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd5(&mut self) -> PDD5_W {
        PDD5_W { w: self }
    }
    #[doc = "Bit 6 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd6(&mut self) -> PDD6_W {
        PDD6_W { w: self }
    }
    #[doc = "Bit 7 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd7(&mut self) -> PDD7_W {
        PDD7_W { w: self }
    }
    #[doc = "Bit 8 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd8(&mut self) -> PDD8_W {
        PDD8_W { w: self }
    }
    #[doc = "Bit 9 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd9(&mut self) -> PDD9_W {
        PDD9_W { w: self }
    }
    #[doc = "Bit 10 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd10(&mut self) -> PDD10_W {
        PDD10_W { w: self }
    }
    #[doc = "Bit 11 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd11(&mut self) -> PDD11_W {
        PDD11_W { w: self }
    }
    #[doc = "Bit 12 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd12(&mut self) -> PDD12_W {
        PDD12_W { w: self }
    }
    #[doc = "Bit 13 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd13(&mut self) -> PDD13_W {
        PDD13_W { w: self }
    }
    #[doc = "Bit 14 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd14(&mut self) -> PDD14_W {
        PDD14_W { w: self }
    }
    #[doc = "Bit 15 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd15(&mut self) -> PDD15_W {
        PDD15_W { w: self }
    }
    #[doc = "Bit 16 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd16(&mut self) -> PDD16_W {
        PDD16_W { w: self }
    }
    #[doc = "Bit 17 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd17(&mut self) -> PDD17_W {
        PDD17_W { w: self }
    }
    #[doc = "Bit 18 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd18(&mut self) -> PDD18_W {
        PDD18_W { w: self }
    }
    #[doc = "Bit 19 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd19(&mut self) -> PDD19_W {
        PDD19_W { w: self }
    }
    #[doc = "Bit 20 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd20(&mut self) -> PDD20_W {
        PDD20_W { w: self }
    }
    #[doc = "Bit 21 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd21(&mut self) -> PDD21_W {
        PDD21_W { w: self }
    }
    #[doc = "Bit 22 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd22(&mut self) -> PDD22_W {
        PDD22_W { w: self }
    }
    #[doc = "Bit 23 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd23(&mut self) -> PDD23_W {
        PDD23_W { w: self }
    }
    #[doc = "Bit 24 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd24(&mut self) -> PDD24_W {
        PDD24_W { w: self }
    }
    #[doc = "Bit 25 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd25(&mut self) -> PDD25_W {
        PDD25_W { w: self }
    }
    #[doc = "Bit 26 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd26(&mut self) -> PDD26_W {
        PDD26_W { w: self }
    }
    #[doc = "Bit 27 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd27(&mut self) -> PDD27_W {
        PDD27_W { w: self }
    }
    #[doc = "Bit 28 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd28(&mut self) -> PDD28_W {
        PDD28_W { w: self }
    }
    #[doc = "Bit 29 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd29(&mut self) -> PDD29_W {
        PDD29_W { w: self }
    }
    #[doc = "Bit 30 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd30(&mut self) -> PDD30_W {
        PDD30_W { w: self }
    }
    #[doc = "Bit 31 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd31(&mut self) -> PDD31_W {
        PDD31_W { w: self }
    }
}
