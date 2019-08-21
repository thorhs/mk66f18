#[doc = "Reader of register IFLAG1"]
pub type R = crate::R<u32, super::IFLAG1>;
#[doc = "Writer for register IFLAG1"]
pub type W = crate::W<u32, super::IFLAG1>;
#[doc = "Register IFLAG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IFLAG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Buffer MB0 Interrupt Or \"reserved\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF0I_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl From<BUF0I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF0I_A) -> Self {
        match variant {
            BUF0I_A::_0 => false,
            BUF0I_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF0I`"]
pub type BUF0I_R = crate::R<bool, BUF0I_A>;
impl BUF0I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF0I_A {
        match self.bits {
            false => BUF0I_A::_0,
            true => BUF0I_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF0I_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF0I_A::_1
    }
}
#[doc = "Write proxy for field `BUF0I`"]
pub struct BUF0I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF0I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF0I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF0I_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF0I_A::_1)
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
#[doc = "Buffer MB i Interrupt Or \"reserved\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF4TO1I0_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl From<BUF4TO1I0_A> for bool {
    #[inline(always)]
    fn from(variant: BUF4TO1I0_A) -> Self {
        match variant {
            BUF4TO1I0_A::_0 => false,
            BUF4TO1I0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF4TO1I0`"]
pub type BUF4TO1I0_R = crate::R<bool, BUF4TO1I0_A>;
impl BUF4TO1I0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF4TO1I0_A {
        match self.bits {
            false => BUF4TO1I0_A::_0,
            true => BUF4TO1I0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF4TO1I0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF4TO1I0_A::_1
    }
}
#[doc = "Write proxy for field `BUF4TO1I0`"]
pub struct BUF4TO1I0_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF4TO1I0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF4TO1I0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF4TO1I0_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF4TO1I0_A::_1)
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
#[doc = "Buffer MB i Interrupt Or \"reserved\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF4TO1I1_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl From<BUF4TO1I1_A> for bool {
    #[inline(always)]
    fn from(variant: BUF4TO1I1_A) -> Self {
        match variant {
            BUF4TO1I1_A::_0 => false,
            BUF4TO1I1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF4TO1I1`"]
pub type BUF4TO1I1_R = crate::R<bool, BUF4TO1I1_A>;
impl BUF4TO1I1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF4TO1I1_A {
        match self.bits {
            false => BUF4TO1I1_A::_0,
            true => BUF4TO1I1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF4TO1I1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF4TO1I1_A::_1
    }
}
#[doc = "Write proxy for field `BUF4TO1I1`"]
pub struct BUF4TO1I1_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF4TO1I1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF4TO1I1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF4TO1I1_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF4TO1I1_A::_1)
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
#[doc = "Buffer MB i Interrupt Or \"reserved\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF4TO1I2_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl From<BUF4TO1I2_A> for bool {
    #[inline(always)]
    fn from(variant: BUF4TO1I2_A) -> Self {
        match variant {
            BUF4TO1I2_A::_0 => false,
            BUF4TO1I2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF4TO1I2`"]
pub type BUF4TO1I2_R = crate::R<bool, BUF4TO1I2_A>;
impl BUF4TO1I2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF4TO1I2_A {
        match self.bits {
            false => BUF4TO1I2_A::_0,
            true => BUF4TO1I2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF4TO1I2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF4TO1I2_A::_1
    }
}
#[doc = "Write proxy for field `BUF4TO1I2`"]
pub struct BUF4TO1I2_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF4TO1I2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF4TO1I2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF4TO1I2_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF4TO1I2_A::_1)
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
#[doc = "Buffer MB i Interrupt Or \"reserved\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF4TO1I3_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    _1,
}
impl From<BUF4TO1I3_A> for bool {
    #[inline(always)]
    fn from(variant: BUF4TO1I3_A) -> Self {
        match variant {
            BUF4TO1I3_A::_0 => false,
            BUF4TO1I3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF4TO1I3`"]
pub type BUF4TO1I3_R = crate::R<bool, BUF4TO1I3_A>;
impl BUF4TO1I3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF4TO1I3_A {
        match self.bits {
            false => BUF4TO1I3_A::_0,
            true => BUF4TO1I3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF4TO1I3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF4TO1I3_A::_1
    }
}
#[doc = "Write proxy for field `BUF4TO1I3`"]
pub struct BUF4TO1I3_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF4TO1I3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF4TO1I3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF4TO1I3_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF4TO1I3_A::_1)
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
#[doc = "Buffer MB5 Interrupt Or \"Frames available in Rx FIFO\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF5I_A {
    #[doc = "0: No occurrence of MB5 completing transmission/reception when MCR\\[RFEN\\]=0, or of frame(s) available in the FIFO, when MCR\\[RFEN\\]=1"]
    _0,
    #[doc = "1: MB5 completed transmission/reception when MCR\\[RFEN\\]=0, or frame(s) available in the Rx FIFO when MCR\\[RFEN\\]=1"]
    _1,
}
impl From<BUF5I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF5I_A) -> Self {
        match variant {
            BUF5I_A::_0 => false,
            BUF5I_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF5I`"]
pub type BUF5I_R = crate::R<bool, BUF5I_A>;
impl BUF5I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF5I_A {
        match self.bits {
            false => BUF5I_A::_0,
            true => BUF5I_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF5I_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF5I_A::_1
    }
}
#[doc = "Write proxy for field `BUF5I`"]
pub struct BUF5I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF5I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF5I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No occurrence of MB5 completing transmission/reception when MCR\\[RFEN\\]=0, or of frame(s) available in the FIFO, when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF5I_A::_0)
    }
    #[doc = "MB5 completed transmission/reception when MCR\\[RFEN\\]=0, or frame(s) available in the Rx FIFO when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF5I_A::_1)
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
#[doc = "Buffer MB6 Interrupt Or \"Rx FIFO Warning\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF6I_A {
    #[doc = "0: No occurrence of MB6 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    _0,
    #[doc = "1: MB6 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    _1,
}
impl From<BUF6I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF6I_A) -> Self {
        match variant {
            BUF6I_A::_0 => false,
            BUF6I_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF6I`"]
pub type BUF6I_R = crate::R<bool, BUF6I_A>;
impl BUF6I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF6I_A {
        match self.bits {
            false => BUF6I_A::_0,
            true => BUF6I_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF6I_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF6I_A::_1
    }
}
#[doc = "Write proxy for field `BUF6I`"]
pub struct BUF6I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF6I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF6I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No occurrence of MB6 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF6I_A::_0)
    }
    #[doc = "MB6 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF6I_A::_1)
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
#[doc = "Buffer MB7 Interrupt Or \"Rx FIFO Overflow\"\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF7I_A {
    #[doc = "0: No occurrence of MB7 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    _0,
    #[doc = "1: MB7 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    _1,
}
impl From<BUF7I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF7I_A) -> Self {
        match variant {
            BUF7I_A::_0 => false,
            BUF7I_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF7I`"]
pub type BUF7I_R = crate::R<bool, BUF7I_A>;
impl BUF7I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF7I_A {
        match self.bits {
            false => BUF7I_A::_0,
            true => BUF7I_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF7I_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF7I_A::_1
    }
}
#[doc = "Write proxy for field `BUF7I`"]
pub struct BUF7I_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF7I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF7I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No occurrence of MB7 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF7I_A::_0)
    }
    #[doc = "MB7 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF7I_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I0_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I0_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I0_A) -> Self {
        match variant {
            BUF31TO8I0_A::_0 => false,
            BUF31TO8I0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I0`"]
pub type BUF31TO8I0_R = crate::R<bool, BUF31TO8I0_A>;
impl BUF31TO8I0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I0_A {
        match self.bits {
            false => BUF31TO8I0_A::_0,
            true => BUF31TO8I0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I0_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I0`"]
pub struct BUF31TO8I0_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I0_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I0_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I1_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I1_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I1_A) -> Self {
        match variant {
            BUF31TO8I1_A::_0 => false,
            BUF31TO8I1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I1`"]
pub type BUF31TO8I1_R = crate::R<bool, BUF31TO8I1_A>;
impl BUF31TO8I1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I1_A {
        match self.bits {
            false => BUF31TO8I1_A::_0,
            true => BUF31TO8I1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I1_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I1`"]
pub struct BUF31TO8I1_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I1_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I1_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I2_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I2_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I2_A) -> Self {
        match variant {
            BUF31TO8I2_A::_0 => false,
            BUF31TO8I2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I2`"]
pub type BUF31TO8I2_R = crate::R<bool, BUF31TO8I2_A>;
impl BUF31TO8I2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I2_A {
        match self.bits {
            false => BUF31TO8I2_A::_0,
            true => BUF31TO8I2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I2_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I2`"]
pub struct BUF31TO8I2_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I2_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I2_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I3_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I3_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I3_A) -> Self {
        match variant {
            BUF31TO8I3_A::_0 => false,
            BUF31TO8I3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I3`"]
pub type BUF31TO8I3_R = crate::R<bool, BUF31TO8I3_A>;
impl BUF31TO8I3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I3_A {
        match self.bits {
            false => BUF31TO8I3_A::_0,
            true => BUF31TO8I3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I3_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I3`"]
pub struct BUF31TO8I3_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I3_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I3_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I4_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I4_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I4_A) -> Self {
        match variant {
            BUF31TO8I4_A::_0 => false,
            BUF31TO8I4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I4`"]
pub type BUF31TO8I4_R = crate::R<bool, BUF31TO8I4_A>;
impl BUF31TO8I4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I4_A {
        match self.bits {
            false => BUF31TO8I4_A::_0,
            true => BUF31TO8I4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I4_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I4`"]
pub struct BUF31TO8I4_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I4_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I4_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I5_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I5_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I5_A) -> Self {
        match variant {
            BUF31TO8I5_A::_0 => false,
            BUF31TO8I5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I5`"]
pub type BUF31TO8I5_R = crate::R<bool, BUF31TO8I5_A>;
impl BUF31TO8I5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I5_A {
        match self.bits {
            false => BUF31TO8I5_A::_0,
            true => BUF31TO8I5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I5_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I5`"]
pub struct BUF31TO8I5_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I5_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I5_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I6_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I6_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I6_A) -> Self {
        match variant {
            BUF31TO8I6_A::_0 => false,
            BUF31TO8I6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I6`"]
pub type BUF31TO8I6_R = crate::R<bool, BUF31TO8I6_A>;
impl BUF31TO8I6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I6_A {
        match self.bits {
            false => BUF31TO8I6_A::_0,
            true => BUF31TO8I6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I6_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I6`"]
pub struct BUF31TO8I6_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I6_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I6_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I7_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I7_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I7_A) -> Self {
        match variant {
            BUF31TO8I7_A::_0 => false,
            BUF31TO8I7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I7`"]
pub type BUF31TO8I7_R = crate::R<bool, BUF31TO8I7_A>;
impl BUF31TO8I7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I7_A {
        match self.bits {
            false => BUF31TO8I7_A::_0,
            true => BUF31TO8I7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I7_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I7`"]
pub struct BUF31TO8I7_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I7_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I7_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I8_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I8_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I8_A) -> Self {
        match variant {
            BUF31TO8I8_A::_0 => false,
            BUF31TO8I8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I8`"]
pub type BUF31TO8I8_R = crate::R<bool, BUF31TO8I8_A>;
impl BUF31TO8I8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I8_A {
        match self.bits {
            false => BUF31TO8I8_A::_0,
            true => BUF31TO8I8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I8_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I8`"]
pub struct BUF31TO8I8_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I8_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I8_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I9_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I9_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I9_A) -> Self {
        match variant {
            BUF31TO8I9_A::_0 => false,
            BUF31TO8I9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I9`"]
pub type BUF31TO8I9_R = crate::R<bool, BUF31TO8I9_A>;
impl BUF31TO8I9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I9_A {
        match self.bits {
            false => BUF31TO8I9_A::_0,
            true => BUF31TO8I9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I9_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I9`"]
pub struct BUF31TO8I9_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I9_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I9_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I10_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I10_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I10_A) -> Self {
        match variant {
            BUF31TO8I10_A::_0 => false,
            BUF31TO8I10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I10`"]
pub type BUF31TO8I10_R = crate::R<bool, BUF31TO8I10_A>;
impl BUF31TO8I10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I10_A {
        match self.bits {
            false => BUF31TO8I10_A::_0,
            true => BUF31TO8I10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I10_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I10`"]
pub struct BUF31TO8I10_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I10_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I10_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I11_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I11_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I11_A) -> Self {
        match variant {
            BUF31TO8I11_A::_0 => false,
            BUF31TO8I11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I11`"]
pub type BUF31TO8I11_R = crate::R<bool, BUF31TO8I11_A>;
impl BUF31TO8I11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I11_A {
        match self.bits {
            false => BUF31TO8I11_A::_0,
            true => BUF31TO8I11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I11_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I11`"]
pub struct BUF31TO8I11_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I11_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I11_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I12_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I12_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I12_A) -> Self {
        match variant {
            BUF31TO8I12_A::_0 => false,
            BUF31TO8I12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I12`"]
pub type BUF31TO8I12_R = crate::R<bool, BUF31TO8I12_A>;
impl BUF31TO8I12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I12_A {
        match self.bits {
            false => BUF31TO8I12_A::_0,
            true => BUF31TO8I12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I12_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I12`"]
pub struct BUF31TO8I12_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I12_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I12_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I13_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I13_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I13_A) -> Self {
        match variant {
            BUF31TO8I13_A::_0 => false,
            BUF31TO8I13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I13`"]
pub type BUF31TO8I13_R = crate::R<bool, BUF31TO8I13_A>;
impl BUF31TO8I13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I13_A {
        match self.bits {
            false => BUF31TO8I13_A::_0,
            true => BUF31TO8I13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I13_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I13`"]
pub struct BUF31TO8I13_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I13_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I13_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I14_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I14_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I14_A) -> Self {
        match variant {
            BUF31TO8I14_A::_0 => false,
            BUF31TO8I14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I14`"]
pub type BUF31TO8I14_R = crate::R<bool, BUF31TO8I14_A>;
impl BUF31TO8I14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I14_A {
        match self.bits {
            false => BUF31TO8I14_A::_0,
            true => BUF31TO8I14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I14_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I14`"]
pub struct BUF31TO8I14_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I14_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I14_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I15_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I15_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I15_A) -> Self {
        match variant {
            BUF31TO8I15_A::_0 => false,
            BUF31TO8I15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I15`"]
pub type BUF31TO8I15_R = crate::R<bool, BUF31TO8I15_A>;
impl BUF31TO8I15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I15_A {
        match self.bits {
            false => BUF31TO8I15_A::_0,
            true => BUF31TO8I15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I15_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I15`"]
pub struct BUF31TO8I15_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I15_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I15_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I16_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I16_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I16_A) -> Self {
        match variant {
            BUF31TO8I16_A::_0 => false,
            BUF31TO8I16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I16`"]
pub type BUF31TO8I16_R = crate::R<bool, BUF31TO8I16_A>;
impl BUF31TO8I16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I16_A {
        match self.bits {
            false => BUF31TO8I16_A::_0,
            true => BUF31TO8I16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I16_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I16`"]
pub struct BUF31TO8I16_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I16_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I16_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I17_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I17_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I17_A) -> Self {
        match variant {
            BUF31TO8I17_A::_0 => false,
            BUF31TO8I17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I17`"]
pub type BUF31TO8I17_R = crate::R<bool, BUF31TO8I17_A>;
impl BUF31TO8I17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I17_A {
        match self.bits {
            false => BUF31TO8I17_A::_0,
            true => BUF31TO8I17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I17_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I17`"]
pub struct BUF31TO8I17_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I17_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I17_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I18_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I18_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I18_A) -> Self {
        match variant {
            BUF31TO8I18_A::_0 => false,
            BUF31TO8I18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I18`"]
pub type BUF31TO8I18_R = crate::R<bool, BUF31TO8I18_A>;
impl BUF31TO8I18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I18_A {
        match self.bits {
            false => BUF31TO8I18_A::_0,
            true => BUF31TO8I18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I18_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I18`"]
pub struct BUF31TO8I18_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I18_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I18_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I19_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I19_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I19_A) -> Self {
        match variant {
            BUF31TO8I19_A::_0 => false,
            BUF31TO8I19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I19`"]
pub type BUF31TO8I19_R = crate::R<bool, BUF31TO8I19_A>;
impl BUF31TO8I19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I19_A {
        match self.bits {
            false => BUF31TO8I19_A::_0,
            true => BUF31TO8I19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I19_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I19`"]
pub struct BUF31TO8I19_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I19_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I19_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I20_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I20_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I20_A) -> Self {
        match variant {
            BUF31TO8I20_A::_0 => false,
            BUF31TO8I20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I20`"]
pub type BUF31TO8I20_R = crate::R<bool, BUF31TO8I20_A>;
impl BUF31TO8I20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I20_A {
        match self.bits {
            false => BUF31TO8I20_A::_0,
            true => BUF31TO8I20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I20_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I20`"]
pub struct BUF31TO8I20_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I20_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I20_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I21_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I21_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I21_A) -> Self {
        match variant {
            BUF31TO8I21_A::_0 => false,
            BUF31TO8I21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I21`"]
pub type BUF31TO8I21_R = crate::R<bool, BUF31TO8I21_A>;
impl BUF31TO8I21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I21_A {
        match self.bits {
            false => BUF31TO8I21_A::_0,
            true => BUF31TO8I21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I21_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I21`"]
pub struct BUF31TO8I21_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I21_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I21_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I22_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I22_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I22_A) -> Self {
        match variant {
            BUF31TO8I22_A::_0 => false,
            BUF31TO8I22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I22`"]
pub type BUF31TO8I22_R = crate::R<bool, BUF31TO8I22_A>;
impl BUF31TO8I22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I22_A {
        match self.bits {
            false => BUF31TO8I22_A::_0,
            true => BUF31TO8I22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I22_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I22`"]
pub struct BUF31TO8I22_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I22_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I22_A::_1)
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
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF31TO8I23_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1,
}
impl From<BUF31TO8I23_A> for bool {
    #[inline(always)]
    fn from(variant: BUF31TO8I23_A) -> Self {
        match variant {
            BUF31TO8I23_A::_0 => false,
            BUF31TO8I23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUF31TO8I23`"]
pub type BUF31TO8I23_R = crate::R<bool, BUF31TO8I23_A>;
impl BUF31TO8I23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF31TO8I23_A {
        match self.bits {
            false => BUF31TO8I23_A::_0,
            true => BUF31TO8I23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUF31TO8I23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUF31TO8I23_A::_1
    }
}
#[doc = "Write proxy for field `BUF31TO8I23`"]
pub struct BUF31TO8I23_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF31TO8I23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUF31TO8I23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF31TO8I23_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF31TO8I23_A::_1)
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
    #[doc = "Bit 0 - Buffer MB0 Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf0i(&self) -> BUF0I_R {
        BUF0I_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i0(&self) -> BUF4TO1I0_R {
        BUF4TO1I0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i1(&self) -> BUF4TO1I1_R {
        BUF4TO1I1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i2(&self) -> BUF4TO1I2_R {
        BUF4TO1I2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i3(&self) -> BUF4TO1I3_R {
        BUF4TO1I3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer MB5 Interrupt Or \"Frames available in Rx FIFO\""]
    #[inline(always)]
    pub fn buf5i(&self) -> BUF5I_R {
        BUF5I_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Buffer MB6 Interrupt Or \"Rx FIFO Warning\""]
    #[inline(always)]
    pub fn buf6i(&self) -> BUF6I_R {
        BUF6I_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Buffer MB7 Interrupt Or \"Rx FIFO Overflow\""]
    #[inline(always)]
    pub fn buf7i(&self) -> BUF7I_R {
        BUF7I_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i0(&self) -> BUF31TO8I0_R {
        BUF31TO8I0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i1(&self) -> BUF31TO8I1_R {
        BUF31TO8I1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i2(&self) -> BUF31TO8I2_R {
        BUF31TO8I2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i3(&self) -> BUF31TO8I3_R {
        BUF31TO8I3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i4(&self) -> BUF31TO8I4_R {
        BUF31TO8I4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i5(&self) -> BUF31TO8I5_R {
        BUF31TO8I5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i6(&self) -> BUF31TO8I6_R {
        BUF31TO8I6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i7(&self) -> BUF31TO8I7_R {
        BUF31TO8I7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i8(&self) -> BUF31TO8I8_R {
        BUF31TO8I8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i9(&self) -> BUF31TO8I9_R {
        BUF31TO8I9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i10(&self) -> BUF31TO8I10_R {
        BUF31TO8I10_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i11(&self) -> BUF31TO8I11_R {
        BUF31TO8I11_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i12(&self) -> BUF31TO8I12_R {
        BUF31TO8I12_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i13(&self) -> BUF31TO8I13_R {
        BUF31TO8I13_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i14(&self) -> BUF31TO8I14_R {
        BUF31TO8I14_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i15(&self) -> BUF31TO8I15_R {
        BUF31TO8I15_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i16(&self) -> BUF31TO8I16_R {
        BUF31TO8I16_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i17(&self) -> BUF31TO8I17_R {
        BUF31TO8I17_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i18(&self) -> BUF31TO8I18_R {
        BUF31TO8I18_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i19(&self) -> BUF31TO8I19_R {
        BUF31TO8I19_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i20(&self) -> BUF31TO8I20_R {
        BUF31TO8I20_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i21(&self) -> BUF31TO8I21_R {
        BUF31TO8I21_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i22(&self) -> BUF31TO8I22_R {
        BUF31TO8I22_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i23(&self) -> BUF31TO8I23_R {
        BUF31TO8I23_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer MB0 Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf0i(&mut self) -> BUF0I_W {
        BUF0I_W { w: self }
    }
    #[doc = "Bit 1 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i0(&mut self) -> BUF4TO1I0_W {
        BUF4TO1I0_W { w: self }
    }
    #[doc = "Bit 2 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i1(&mut self) -> BUF4TO1I1_W {
        BUF4TO1I1_W { w: self }
    }
    #[doc = "Bit 3 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i2(&mut self) -> BUF4TO1I2_W {
        BUF4TO1I2_W { w: self }
    }
    #[doc = "Bit 4 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline(always)]
    pub fn buf4to1i3(&mut self) -> BUF4TO1I3_W {
        BUF4TO1I3_W { w: self }
    }
    #[doc = "Bit 5 - Buffer MB5 Interrupt Or \"Frames available in Rx FIFO\""]
    #[inline(always)]
    pub fn buf5i(&mut self) -> BUF5I_W {
        BUF5I_W { w: self }
    }
    #[doc = "Bit 6 - Buffer MB6 Interrupt Or \"Rx FIFO Warning\""]
    #[inline(always)]
    pub fn buf6i(&mut self) -> BUF6I_W {
        BUF6I_W { w: self }
    }
    #[doc = "Bit 7 - Buffer MB7 Interrupt Or \"Rx FIFO Overflow\""]
    #[inline(always)]
    pub fn buf7i(&mut self) -> BUF7I_W {
        BUF7I_W { w: self }
    }
    #[doc = "Bit 8 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i0(&mut self) -> BUF31TO8I0_W {
        BUF31TO8I0_W { w: self }
    }
    #[doc = "Bit 9 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i1(&mut self) -> BUF31TO8I1_W {
        BUF31TO8I1_W { w: self }
    }
    #[doc = "Bit 10 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i2(&mut self) -> BUF31TO8I2_W {
        BUF31TO8I2_W { w: self }
    }
    #[doc = "Bit 11 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i3(&mut self) -> BUF31TO8I3_W {
        BUF31TO8I3_W { w: self }
    }
    #[doc = "Bit 12 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i4(&mut self) -> BUF31TO8I4_W {
        BUF31TO8I4_W { w: self }
    }
    #[doc = "Bit 13 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i5(&mut self) -> BUF31TO8I5_W {
        BUF31TO8I5_W { w: self }
    }
    #[doc = "Bit 14 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i6(&mut self) -> BUF31TO8I6_W {
        BUF31TO8I6_W { w: self }
    }
    #[doc = "Bit 15 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i7(&mut self) -> BUF31TO8I7_W {
        BUF31TO8I7_W { w: self }
    }
    #[doc = "Bit 16 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i8(&mut self) -> BUF31TO8I8_W {
        BUF31TO8I8_W { w: self }
    }
    #[doc = "Bit 17 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i9(&mut self) -> BUF31TO8I9_W {
        BUF31TO8I9_W { w: self }
    }
    #[doc = "Bit 18 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i10(&mut self) -> BUF31TO8I10_W {
        BUF31TO8I10_W { w: self }
    }
    #[doc = "Bit 19 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i11(&mut self) -> BUF31TO8I11_W {
        BUF31TO8I11_W { w: self }
    }
    #[doc = "Bit 20 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i12(&mut self) -> BUF31TO8I12_W {
        BUF31TO8I12_W { w: self }
    }
    #[doc = "Bit 21 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i13(&mut self) -> BUF31TO8I13_W {
        BUF31TO8I13_W { w: self }
    }
    #[doc = "Bit 22 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i14(&mut self) -> BUF31TO8I14_W {
        BUF31TO8I14_W { w: self }
    }
    #[doc = "Bit 23 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i15(&mut self) -> BUF31TO8I15_W {
        BUF31TO8I15_W { w: self }
    }
    #[doc = "Bit 24 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i16(&mut self) -> BUF31TO8I16_W {
        BUF31TO8I16_W { w: self }
    }
    #[doc = "Bit 25 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i17(&mut self) -> BUF31TO8I17_W {
        BUF31TO8I17_W { w: self }
    }
    #[doc = "Bit 26 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i18(&mut self) -> BUF31TO8I18_W {
        BUF31TO8I18_W { w: self }
    }
    #[doc = "Bit 27 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i19(&mut self) -> BUF31TO8I19_W {
        BUF31TO8I19_W { w: self }
    }
    #[doc = "Bit 28 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i20(&mut self) -> BUF31TO8I20_W {
        BUF31TO8I20_W { w: self }
    }
    #[doc = "Bit 29 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i21(&mut self) -> BUF31TO8I21_W {
        BUF31TO8I21_W { w: self }
    }
    #[doc = "Bit 30 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i22(&mut self) -> BUF31TO8I22_W {
        BUF31TO8I22_W { w: self }
    }
    #[doc = "Bit 31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf31to8i23(&mut self) -> BUF31TO8I23_W {
        BUF31TO8I23_W { w: self }
    }
}
