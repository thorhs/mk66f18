#[doc = "Reader of register PFAPR"]
pub type R = crate::R<u32, super::PFAPR>;
#[doc = "Writer for register PFAPR"]
pub type W = crate::W<u32, super::PFAPR>;
#[doc = "Register PFAPR `reset()`'s with value 0x00f8_003f"]
impl crate::ResetValue for super::PFAPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00f8_003f
    }
}
#[doc = "Master 0 Access Protection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11,
}
impl From<M0AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M0AP_A) -> Self {
        match variant {
            M0AP_A::_00 => 0,
            M0AP_A::_01 => 1,
            M0AP_A::_10 => 2,
            M0AP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `M0AP`"]
pub type M0AP_R = crate::R<u8, M0AP_A>;
impl M0AP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M0AP_A {
        match self.bits {
            0 => M0AP_A::_00,
            1 => M0AP_A::_01,
            2 => M0AP_A::_10,
            3 => M0AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M0AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M0AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M0AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M0AP_A::_11
    }
}
#[doc = "Write proxy for field `M0AP`"]
pub struct M0AP_W<'a> {
    w: &'a mut W,
}
impl<'a> M0AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M0AP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M0AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M0AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M0AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M0AP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Master 1 Access Protection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M1AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11,
}
impl From<M1AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M1AP_A) -> Self {
        match variant {
            M1AP_A::_00 => 0,
            M1AP_A::_01 => 1,
            M1AP_A::_10 => 2,
            M1AP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `M1AP`"]
pub type M1AP_R = crate::R<u8, M1AP_A>;
impl M1AP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M1AP_A {
        match self.bits {
            0 => M1AP_A::_00,
            1 => M1AP_A::_01,
            2 => M1AP_A::_10,
            3 => M1AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M1AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M1AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M1AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M1AP_A::_11
    }
}
#[doc = "Write proxy for field `M1AP`"]
pub struct M1AP_W<'a> {
    w: &'a mut W,
}
impl<'a> M1AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M1AP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M1AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M1AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M1AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M1AP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Master 2 Access Protection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M2AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11,
}
impl From<M2AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M2AP_A) -> Self {
        match variant {
            M2AP_A::_00 => 0,
            M2AP_A::_01 => 1,
            M2AP_A::_10 => 2,
            M2AP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `M2AP`"]
pub type M2AP_R = crate::R<u8, M2AP_A>;
impl M2AP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M2AP_A {
        match self.bits {
            0 => M2AP_A::_00,
            1 => M2AP_A::_01,
            2 => M2AP_A::_10,
            3 => M2AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M2AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M2AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M2AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M2AP_A::_11
    }
}
#[doc = "Write proxy for field `M2AP`"]
pub struct M2AP_W<'a> {
    w: &'a mut W,
}
impl<'a> M2AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M2AP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M2AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M2AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M2AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M2AP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Master 3 Access Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11,
}
impl From<M3AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M3AP_A) -> Self {
        match variant {
            M3AP_A::_00 => 0,
            M3AP_A::_01 => 1,
            M3AP_A::_10 => 2,
            M3AP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `M3AP`"]
pub type M3AP_R = crate::R<u8, M3AP_A>;
impl M3AP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M3AP_A {
        match self.bits {
            0 => M3AP_A::_00,
            1 => M3AP_A::_01,
            2 => M3AP_A::_10,
            3 => M3AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M3AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M3AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M3AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M3AP_A::_11
    }
}
#[doc = "Write proxy for field `M3AP`"]
pub struct M3AP_W<'a> {
    w: &'a mut W,
}
impl<'a> M3AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M3AP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M3AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M3AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M3AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M3AP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Master 4 Access Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11,
}
impl From<M4AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M4AP_A) -> Self {
        match variant {
            M4AP_A::_00 => 0,
            M4AP_A::_01 => 1,
            M4AP_A::_10 => 2,
            M4AP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `M4AP`"]
pub type M4AP_R = crate::R<u8, M4AP_A>;
impl M4AP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4AP_A {
        match self.bits {
            0 => M4AP_A::_00,
            1 => M4AP_A::_01,
            2 => M4AP_A::_10,
            3 => M4AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M4AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M4AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M4AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M4AP_A::_11
    }
}
#[doc = "Write proxy for field `M4AP`"]
pub struct M4AP_W<'a> {
    w: &'a mut W,
}
impl<'a> M4AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4AP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M4AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M4AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M4AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M4AP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Master 5 Access Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M5AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11,
}
impl From<M5AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M5AP_A) -> Self {
        match variant {
            M5AP_A::_00 => 0,
            M5AP_A::_01 => 1,
            M5AP_A::_10 => 2,
            M5AP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `M5AP`"]
pub type M5AP_R = crate::R<u8, M5AP_A>;
impl M5AP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M5AP_A {
        match self.bits {
            0 => M5AP_A::_00,
            1 => M5AP_A::_01,
            2 => M5AP_A::_10,
            3 => M5AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M5AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M5AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M5AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M5AP_A::_11
    }
}
#[doc = "Write proxy for field `M5AP`"]
pub struct M5AP_W<'a> {
    w: &'a mut W,
}
impl<'a> M5AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M5AP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M5AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M5AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M5AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M5AP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Master 6 Access Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M6AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11,
}
impl From<M6AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M6AP_A) -> Self {
        match variant {
            M6AP_A::_00 => 0,
            M6AP_A::_01 => 1,
            M6AP_A::_10 => 2,
            M6AP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `M6AP`"]
pub type M6AP_R = crate::R<u8, M6AP_A>;
impl M6AP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M6AP_A {
        match self.bits {
            0 => M6AP_A::_00,
            1 => M6AP_A::_01,
            2 => M6AP_A::_10,
            3 => M6AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M6AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M6AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M6AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M6AP_A::_11
    }
}
#[doc = "Write proxy for field `M6AP`"]
pub struct M6AP_W<'a> {
    w: &'a mut W,
}
impl<'a> M6AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M6AP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M6AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M6AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M6AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M6AP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Master 7 Access Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7AP_A {
    #[doc = "0: No access may be performed by this master."]
    _00,
    #[doc = "1: Only read accesses may be performed by this master."]
    _01,
    #[doc = "2: Only write accesses may be performed by this master."]
    _10,
    #[doc = "3: Both read and write accesses may be performed by this master."]
    _11,
}
impl From<M7AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M7AP_A) -> Self {
        match variant {
            M7AP_A::_00 => 0,
            M7AP_A::_01 => 1,
            M7AP_A::_10 => 2,
            M7AP_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `M7AP`"]
pub type M7AP_R = crate::R<u8, M7AP_A>;
impl M7AP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7AP_A {
        match self.bits {
            0 => M7AP_A::_00,
            1 => M7AP_A::_01,
            2 => M7AP_A::_10,
            3 => M7AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M7AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M7AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M7AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M7AP_A::_11
    }
}
#[doc = "Write proxy for field `M7AP`"]
pub struct M7AP_W<'a> {
    w: &'a mut W,
}
impl<'a> M7AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M7AP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No access may be performed by this master."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M7AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M7AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M7AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M7AP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Master 0 Prefetch Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1,
}
impl From<M0PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M0PFD_A) -> Self {
        match variant {
            M0PFD_A::_0 => false,
            M0PFD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M0PFD`"]
pub type M0PFD_R = crate::R<bool, M0PFD_A>;
impl M0PFD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M0PFD_A {
        match self.bits {
            false => M0PFD_A::_0,
            true => M0PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M0PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M0PFD_A::_1
    }
}
#[doc = "Write proxy for field `M0PFD`"]
pub struct M0PFD_W<'a> {
    w: &'a mut W,
}
impl<'a> M0PFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M0PFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M0PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M0PFD_A::_1)
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
#[doc = "Master 1 Prefetch Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M1PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1,
}
impl From<M1PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M1PFD_A) -> Self {
        match variant {
            M1PFD_A::_0 => false,
            M1PFD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M1PFD`"]
pub type M1PFD_R = crate::R<bool, M1PFD_A>;
impl M1PFD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M1PFD_A {
        match self.bits {
            false => M1PFD_A::_0,
            true => M1PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M1PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M1PFD_A::_1
    }
}
#[doc = "Write proxy for field `M1PFD`"]
pub struct M1PFD_W<'a> {
    w: &'a mut W,
}
impl<'a> M1PFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M1PFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M1PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M1PFD_A::_1)
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
#[doc = "Master 2 Prefetch Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M2PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1,
}
impl From<M2PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M2PFD_A) -> Self {
        match variant {
            M2PFD_A::_0 => false,
            M2PFD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M2PFD`"]
pub type M2PFD_R = crate::R<bool, M2PFD_A>;
impl M2PFD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M2PFD_A {
        match self.bits {
            false => M2PFD_A::_0,
            true => M2PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M2PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M2PFD_A::_1
    }
}
#[doc = "Write proxy for field `M2PFD`"]
pub struct M2PFD_W<'a> {
    w: &'a mut W,
}
impl<'a> M2PFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M2PFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M2PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M2PFD_A::_1)
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
#[doc = "Master 3 Prefetch Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1,
}
impl From<M3PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M3PFD_A) -> Self {
        match variant {
            M3PFD_A::_0 => false,
            M3PFD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M3PFD`"]
pub type M3PFD_R = crate::R<bool, M3PFD_A>;
impl M3PFD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M3PFD_A {
        match self.bits {
            false => M3PFD_A::_0,
            true => M3PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M3PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M3PFD_A::_1
    }
}
#[doc = "Write proxy for field `M3PFD`"]
pub struct M3PFD_W<'a> {
    w: &'a mut W,
}
impl<'a> M3PFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M3PFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M3PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M3PFD_A::_1)
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
#[doc = "Master 4 Prefetch Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1,
}
impl From<M4PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M4PFD_A) -> Self {
        match variant {
            M4PFD_A::_0 => false,
            M4PFD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M4PFD`"]
pub type M4PFD_R = crate::R<bool, M4PFD_A>;
impl M4PFD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4PFD_A {
        match self.bits {
            false => M4PFD_A::_0,
            true => M4PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M4PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M4PFD_A::_1
    }
}
#[doc = "Write proxy for field `M4PFD`"]
pub struct M4PFD_W<'a> {
    w: &'a mut W,
}
impl<'a> M4PFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4PFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M4PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M4PFD_A::_1)
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
#[doc = "Master 5 Prefetch Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M5PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1,
}
impl From<M5PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M5PFD_A) -> Self {
        match variant {
            M5PFD_A::_0 => false,
            M5PFD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M5PFD`"]
pub type M5PFD_R = crate::R<bool, M5PFD_A>;
impl M5PFD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M5PFD_A {
        match self.bits {
            false => M5PFD_A::_0,
            true => M5PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M5PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M5PFD_A::_1
    }
}
#[doc = "Write proxy for field `M5PFD`"]
pub struct M5PFD_W<'a> {
    w: &'a mut W,
}
impl<'a> M5PFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M5PFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M5PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M5PFD_A::_1)
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
#[doc = "Master 6 Prefetch Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M6PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1,
}
impl From<M6PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M6PFD_A) -> Self {
        match variant {
            M6PFD_A::_0 => false,
            M6PFD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M6PFD`"]
pub type M6PFD_R = crate::R<bool, M6PFD_A>;
impl M6PFD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M6PFD_A {
        match self.bits {
            false => M6PFD_A::_0,
            true => M6PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M6PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M6PFD_A::_1
    }
}
#[doc = "Write proxy for field `M6PFD`"]
pub struct M6PFD_W<'a> {
    w: &'a mut W,
}
impl<'a> M6PFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M6PFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M6PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M6PFD_A::_1)
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
#[doc = "Master 7 Prefetch Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1,
}
impl From<M7PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M7PFD_A) -> Self {
        match variant {
            M7PFD_A::_0 => false,
            M7PFD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M7PFD`"]
pub type M7PFD_R = crate::R<bool, M7PFD_A>;
impl M7PFD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7PFD_A {
        match self.bits {
            false => M7PFD_A::_0,
            true => M7PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M7PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M7PFD_A::_1
    }
}
#[doc = "Write proxy for field `M7PFD`"]
pub struct M7PFD_W<'a> {
    w: &'a mut W,
}
impl<'a> M7PFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M7PFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M7PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M7PFD_A::_1)
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
impl R {
    #[doc = "Bits 0:1 - Master 0 Access Protection"]
    #[inline(always)]
    pub fn m0ap(&self) -> M0AP_R {
        M0AP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Master 1 Access Protection"]
    #[inline(always)]
    pub fn m1ap(&self) -> M1AP_R {
        M1AP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Master 2 Access Protection"]
    #[inline(always)]
    pub fn m2ap(&self) -> M2AP_R {
        M2AP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Master 3 Access Protection"]
    #[inline(always)]
    pub fn m3ap(&self) -> M3AP_R {
        M3AP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Master 4 Access Protection"]
    #[inline(always)]
    pub fn m4ap(&self) -> M4AP_R {
        M4AP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Master 5 Access Protection"]
    #[inline(always)]
    pub fn m5ap(&self) -> M5AP_R {
        M5AP_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Master 6 Access Protection"]
    #[inline(always)]
    pub fn m6ap(&self) -> M6AP_R {
        M6AP_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Master 7 Access Protection"]
    #[inline(always)]
    pub fn m7ap(&self) -> M7AP_R {
        M7AP_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Master 0 Prefetch Disable"]
    #[inline(always)]
    pub fn m0pfd(&self) -> M0PFD_R {
        M0PFD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Master 1 Prefetch Disable"]
    #[inline(always)]
    pub fn m1pfd(&self) -> M1PFD_R {
        M1PFD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Master 2 Prefetch Disable"]
    #[inline(always)]
    pub fn m2pfd(&self) -> M2PFD_R {
        M2PFD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Master 3 Prefetch Disable"]
    #[inline(always)]
    pub fn m3pfd(&self) -> M3PFD_R {
        M3PFD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Master 4 Prefetch Disable"]
    #[inline(always)]
    pub fn m4pfd(&self) -> M4PFD_R {
        M4PFD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Master 5 Prefetch Disable"]
    #[inline(always)]
    pub fn m5pfd(&self) -> M5PFD_R {
        M5PFD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Master 6 Prefetch Disable"]
    #[inline(always)]
    pub fn m6pfd(&self) -> M6PFD_R {
        M6PFD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Master 7 Prefetch Disable"]
    #[inline(always)]
    pub fn m7pfd(&self) -> M7PFD_R {
        M7PFD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 0 Access Protection"]
    #[inline(always)]
    pub fn m0ap(&mut self) -> M0AP_W {
        M0AP_W { w: self }
    }
    #[doc = "Bits 2:3 - Master 1 Access Protection"]
    #[inline(always)]
    pub fn m1ap(&mut self) -> M1AP_W {
        M1AP_W { w: self }
    }
    #[doc = "Bits 4:5 - Master 2 Access Protection"]
    #[inline(always)]
    pub fn m2ap(&mut self) -> M2AP_W {
        M2AP_W { w: self }
    }
    #[doc = "Bits 6:7 - Master 3 Access Protection"]
    #[inline(always)]
    pub fn m3ap(&mut self) -> M3AP_W {
        M3AP_W { w: self }
    }
    #[doc = "Bits 8:9 - Master 4 Access Protection"]
    #[inline(always)]
    pub fn m4ap(&mut self) -> M4AP_W {
        M4AP_W { w: self }
    }
    #[doc = "Bits 10:11 - Master 5 Access Protection"]
    #[inline(always)]
    pub fn m5ap(&mut self) -> M5AP_W {
        M5AP_W { w: self }
    }
    #[doc = "Bits 12:13 - Master 6 Access Protection"]
    #[inline(always)]
    pub fn m6ap(&mut self) -> M6AP_W {
        M6AP_W { w: self }
    }
    #[doc = "Bits 14:15 - Master 7 Access Protection"]
    #[inline(always)]
    pub fn m7ap(&mut self) -> M7AP_W {
        M7AP_W { w: self }
    }
    #[doc = "Bit 16 - Master 0 Prefetch Disable"]
    #[inline(always)]
    pub fn m0pfd(&mut self) -> M0PFD_W {
        M0PFD_W { w: self }
    }
    #[doc = "Bit 17 - Master 1 Prefetch Disable"]
    #[inline(always)]
    pub fn m1pfd(&mut self) -> M1PFD_W {
        M1PFD_W { w: self }
    }
    #[doc = "Bit 18 - Master 2 Prefetch Disable"]
    #[inline(always)]
    pub fn m2pfd(&mut self) -> M2PFD_W {
        M2PFD_W { w: self }
    }
    #[doc = "Bit 19 - Master 3 Prefetch Disable"]
    #[inline(always)]
    pub fn m3pfd(&mut self) -> M3PFD_W {
        M3PFD_W { w: self }
    }
    #[doc = "Bit 20 - Master 4 Prefetch Disable"]
    #[inline(always)]
    pub fn m4pfd(&mut self) -> M4PFD_W {
        M4PFD_W { w: self }
    }
    #[doc = "Bit 21 - Master 5 Prefetch Disable"]
    #[inline(always)]
    pub fn m5pfd(&mut self) -> M5PFD_W {
        M5PFD_W { w: self }
    }
    #[doc = "Bit 22 - Master 6 Prefetch Disable"]
    #[inline(always)]
    pub fn m6pfd(&mut self) -> M6PFD_W {
        M6PFD_W { w: self }
    }
    #[doc = "Bit 23 - Master 7 Prefetch Disable"]
    #[inline(always)]
    pub fn m7pfd(&mut self) -> M7PFD_W {
        M7PFD_W { w: self }
    }
}
