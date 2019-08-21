#[doc = "Reader of register HRS"]
pub type R = crate::R<u32, super::HRS>;
#[doc = "Hardware Request Status Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS0_A {
    #[doc = "0: A hardware service request for channel 0 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 0 is present"]
    _1,
}
impl From<HRS0_A> for bool {
    #[inline(always)]
    fn from(variant: HRS0_A) -> Self {
        match variant {
            HRS0_A::_0 => false,
            HRS0_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS0`"]
pub type HRS0_R = crate::R<bool, HRS0_A>;
impl HRS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS0_A {
        match self.bits {
            false => HRS0_A::_0,
            true => HRS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS0_A::_1
    }
}
#[doc = "Hardware Request Status Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS1_A {
    #[doc = "0: A hardware service request for channel 1 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 1 is present"]
    _1,
}
impl From<HRS1_A> for bool {
    #[inline(always)]
    fn from(variant: HRS1_A) -> Self {
        match variant {
            HRS1_A::_0 => false,
            HRS1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS1`"]
pub type HRS1_R = crate::R<bool, HRS1_A>;
impl HRS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS1_A {
        match self.bits {
            false => HRS1_A::_0,
            true => HRS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS1_A::_1
    }
}
#[doc = "Hardware Request Status Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS2_A {
    #[doc = "0: A hardware service request for channel 2 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 2 is present"]
    _1,
}
impl From<HRS2_A> for bool {
    #[inline(always)]
    fn from(variant: HRS2_A) -> Self {
        match variant {
            HRS2_A::_0 => false,
            HRS2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS2`"]
pub type HRS2_R = crate::R<bool, HRS2_A>;
impl HRS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS2_A {
        match self.bits {
            false => HRS2_A::_0,
            true => HRS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS2_A::_1
    }
}
#[doc = "Hardware Request Status Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS3_A {
    #[doc = "0: A hardware service request for channel 3 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 3 is present"]
    _1,
}
impl From<HRS3_A> for bool {
    #[inline(always)]
    fn from(variant: HRS3_A) -> Self {
        match variant {
            HRS3_A::_0 => false,
            HRS3_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS3`"]
pub type HRS3_R = crate::R<bool, HRS3_A>;
impl HRS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS3_A {
        match self.bits {
            false => HRS3_A::_0,
            true => HRS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS3_A::_1
    }
}
#[doc = "Hardware Request Status Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS4_A {
    #[doc = "0: A hardware service request for channel 4 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 4 is present"]
    _1,
}
impl From<HRS4_A> for bool {
    #[inline(always)]
    fn from(variant: HRS4_A) -> Self {
        match variant {
            HRS4_A::_0 => false,
            HRS4_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS4`"]
pub type HRS4_R = crate::R<bool, HRS4_A>;
impl HRS4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS4_A {
        match self.bits {
            false => HRS4_A::_0,
            true => HRS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS4_A::_1
    }
}
#[doc = "Hardware Request Status Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS5_A {
    #[doc = "0: A hardware service request for channel 5 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 5 is present"]
    _1,
}
impl From<HRS5_A> for bool {
    #[inline(always)]
    fn from(variant: HRS5_A) -> Self {
        match variant {
            HRS5_A::_0 => false,
            HRS5_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS5`"]
pub type HRS5_R = crate::R<bool, HRS5_A>;
impl HRS5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS5_A {
        match self.bits {
            false => HRS5_A::_0,
            true => HRS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS5_A::_1
    }
}
#[doc = "Hardware Request Status Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS6_A {
    #[doc = "0: A hardware service request for channel 6 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 6 is present"]
    _1,
}
impl From<HRS6_A> for bool {
    #[inline(always)]
    fn from(variant: HRS6_A) -> Self {
        match variant {
            HRS6_A::_0 => false,
            HRS6_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS6`"]
pub type HRS6_R = crate::R<bool, HRS6_A>;
impl HRS6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS6_A {
        match self.bits {
            false => HRS6_A::_0,
            true => HRS6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS6_A::_1
    }
}
#[doc = "Hardware Request Status Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS7_A {
    #[doc = "0: A hardware service request for channel 7 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 7 is present"]
    _1,
}
impl From<HRS7_A> for bool {
    #[inline(always)]
    fn from(variant: HRS7_A) -> Self {
        match variant {
            HRS7_A::_0 => false,
            HRS7_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS7`"]
pub type HRS7_R = crate::R<bool, HRS7_A>;
impl HRS7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS7_A {
        match self.bits {
            false => HRS7_A::_0,
            true => HRS7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS7_A::_1
    }
}
#[doc = "Hardware Request Status Channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS8_A {
    #[doc = "0: A hardware service request for channel 8 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 8 is present"]
    _1,
}
impl From<HRS8_A> for bool {
    #[inline(always)]
    fn from(variant: HRS8_A) -> Self {
        match variant {
            HRS8_A::_0 => false,
            HRS8_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS8`"]
pub type HRS8_R = crate::R<bool, HRS8_A>;
impl HRS8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS8_A {
        match self.bits {
            false => HRS8_A::_0,
            true => HRS8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS8_A::_1
    }
}
#[doc = "Hardware Request Status Channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS9_A {
    #[doc = "0: A hardware service request for channel 9 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 9 is present"]
    _1,
}
impl From<HRS9_A> for bool {
    #[inline(always)]
    fn from(variant: HRS9_A) -> Self {
        match variant {
            HRS9_A::_0 => false,
            HRS9_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS9`"]
pub type HRS9_R = crate::R<bool, HRS9_A>;
impl HRS9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS9_A {
        match self.bits {
            false => HRS9_A::_0,
            true => HRS9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS9_A::_1
    }
}
#[doc = "Hardware Request Status Channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS10_A {
    #[doc = "0: A hardware service request for channel 10 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 10 is present"]
    _1,
}
impl From<HRS10_A> for bool {
    #[inline(always)]
    fn from(variant: HRS10_A) -> Self {
        match variant {
            HRS10_A::_0 => false,
            HRS10_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS10`"]
pub type HRS10_R = crate::R<bool, HRS10_A>;
impl HRS10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS10_A {
        match self.bits {
            false => HRS10_A::_0,
            true => HRS10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS10_A::_1
    }
}
#[doc = "Hardware Request Status Channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS11_A {
    #[doc = "0: A hardware service request for channel 11 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 11 is present"]
    _1,
}
impl From<HRS11_A> for bool {
    #[inline(always)]
    fn from(variant: HRS11_A) -> Self {
        match variant {
            HRS11_A::_0 => false,
            HRS11_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS11`"]
pub type HRS11_R = crate::R<bool, HRS11_A>;
impl HRS11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS11_A {
        match self.bits {
            false => HRS11_A::_0,
            true => HRS11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS11_A::_1
    }
}
#[doc = "Hardware Request Status Channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS12_A {
    #[doc = "0: A hardware service request for channel 12 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 12 is present"]
    _1,
}
impl From<HRS12_A> for bool {
    #[inline(always)]
    fn from(variant: HRS12_A) -> Self {
        match variant {
            HRS12_A::_0 => false,
            HRS12_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS12`"]
pub type HRS12_R = crate::R<bool, HRS12_A>;
impl HRS12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS12_A {
        match self.bits {
            false => HRS12_A::_0,
            true => HRS12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS12_A::_1
    }
}
#[doc = "Hardware Request Status Channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS13_A {
    #[doc = "0: A hardware service request for channel 13 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 13 is present"]
    _1,
}
impl From<HRS13_A> for bool {
    #[inline(always)]
    fn from(variant: HRS13_A) -> Self {
        match variant {
            HRS13_A::_0 => false,
            HRS13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS13`"]
pub type HRS13_R = crate::R<bool, HRS13_A>;
impl HRS13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS13_A {
        match self.bits {
            false => HRS13_A::_0,
            true => HRS13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS13_A::_1
    }
}
#[doc = "Hardware Request Status Channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS14_A {
    #[doc = "0: A hardware service request for channel 14 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 14 is present"]
    _1,
}
impl From<HRS14_A> for bool {
    #[inline(always)]
    fn from(variant: HRS14_A) -> Self {
        match variant {
            HRS14_A::_0 => false,
            HRS14_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS14`"]
pub type HRS14_R = crate::R<bool, HRS14_A>;
impl HRS14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS14_A {
        match self.bits {
            false => HRS14_A::_0,
            true => HRS14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS14_A::_1
    }
}
#[doc = "Hardware Request Status Channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS15_A {
    #[doc = "0: A hardware service request for channel 15 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 15 is present"]
    _1,
}
impl From<HRS15_A> for bool {
    #[inline(always)]
    fn from(variant: HRS15_A) -> Self {
        match variant {
            HRS15_A::_0 => false,
            HRS15_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS15`"]
pub type HRS15_R = crate::R<bool, HRS15_A>;
impl HRS15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS15_A {
        match self.bits {
            false => HRS15_A::_0,
            true => HRS15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS15_A::_1
    }
}
#[doc = "Hardware Request Status Channel 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS16_A {
    #[doc = "0: A hardware service request for channel 16 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 16 is present"]
    _1,
}
impl From<HRS16_A> for bool {
    #[inline(always)]
    fn from(variant: HRS16_A) -> Self {
        match variant {
            HRS16_A::_0 => false,
            HRS16_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS16`"]
pub type HRS16_R = crate::R<bool, HRS16_A>;
impl HRS16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS16_A {
        match self.bits {
            false => HRS16_A::_0,
            true => HRS16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS16_A::_1
    }
}
#[doc = "Hardware Request Status Channel 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS17_A {
    #[doc = "0: A hardware service request for channel 17 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 17 is present"]
    _1,
}
impl From<HRS17_A> for bool {
    #[inline(always)]
    fn from(variant: HRS17_A) -> Self {
        match variant {
            HRS17_A::_0 => false,
            HRS17_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS17`"]
pub type HRS17_R = crate::R<bool, HRS17_A>;
impl HRS17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS17_A {
        match self.bits {
            false => HRS17_A::_0,
            true => HRS17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS17_A::_1
    }
}
#[doc = "Hardware Request Status Channel 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS18_A {
    #[doc = "0: A hardware service request for channel 18 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 18 is present"]
    _1,
}
impl From<HRS18_A> for bool {
    #[inline(always)]
    fn from(variant: HRS18_A) -> Self {
        match variant {
            HRS18_A::_0 => false,
            HRS18_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS18`"]
pub type HRS18_R = crate::R<bool, HRS18_A>;
impl HRS18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS18_A {
        match self.bits {
            false => HRS18_A::_0,
            true => HRS18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS18_A::_1
    }
}
#[doc = "Hardware Request Status Channel 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS19_A {
    #[doc = "0: A hardware service request for channel 19 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 19 is present"]
    _1,
}
impl From<HRS19_A> for bool {
    #[inline(always)]
    fn from(variant: HRS19_A) -> Self {
        match variant {
            HRS19_A::_0 => false,
            HRS19_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS19`"]
pub type HRS19_R = crate::R<bool, HRS19_A>;
impl HRS19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS19_A {
        match self.bits {
            false => HRS19_A::_0,
            true => HRS19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS19_A::_1
    }
}
#[doc = "Hardware Request Status Channel 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS20_A {
    #[doc = "0: A hardware service request for channel 20 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 20 is present"]
    _1,
}
impl From<HRS20_A> for bool {
    #[inline(always)]
    fn from(variant: HRS20_A) -> Self {
        match variant {
            HRS20_A::_0 => false,
            HRS20_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS20`"]
pub type HRS20_R = crate::R<bool, HRS20_A>;
impl HRS20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS20_A {
        match self.bits {
            false => HRS20_A::_0,
            true => HRS20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS20_A::_1
    }
}
#[doc = "Hardware Request Status Channel 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS21_A {
    #[doc = "0: A hardware service request for channel 21 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 21 is present"]
    _1,
}
impl From<HRS21_A> for bool {
    #[inline(always)]
    fn from(variant: HRS21_A) -> Self {
        match variant {
            HRS21_A::_0 => false,
            HRS21_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS21`"]
pub type HRS21_R = crate::R<bool, HRS21_A>;
impl HRS21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS21_A {
        match self.bits {
            false => HRS21_A::_0,
            true => HRS21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS21_A::_1
    }
}
#[doc = "Hardware Request Status Channel 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS22_A {
    #[doc = "0: A hardware service request for channel 22 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 22 is present"]
    _1,
}
impl From<HRS22_A> for bool {
    #[inline(always)]
    fn from(variant: HRS22_A) -> Self {
        match variant {
            HRS22_A::_0 => false,
            HRS22_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS22`"]
pub type HRS22_R = crate::R<bool, HRS22_A>;
impl HRS22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS22_A {
        match self.bits {
            false => HRS22_A::_0,
            true => HRS22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS22_A::_1
    }
}
#[doc = "Hardware Request Status Channel 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS23_A {
    #[doc = "0: A hardware service request for channel 23 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 23 is present"]
    _1,
}
impl From<HRS23_A> for bool {
    #[inline(always)]
    fn from(variant: HRS23_A) -> Self {
        match variant {
            HRS23_A::_0 => false,
            HRS23_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS23`"]
pub type HRS23_R = crate::R<bool, HRS23_A>;
impl HRS23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS23_A {
        match self.bits {
            false => HRS23_A::_0,
            true => HRS23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS23_A::_1
    }
}
#[doc = "Hardware Request Status Channel 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS24_A {
    #[doc = "0: A hardware service request for channel 24 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 24 is present"]
    _1,
}
impl From<HRS24_A> for bool {
    #[inline(always)]
    fn from(variant: HRS24_A) -> Self {
        match variant {
            HRS24_A::_0 => false,
            HRS24_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS24`"]
pub type HRS24_R = crate::R<bool, HRS24_A>;
impl HRS24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS24_A {
        match self.bits {
            false => HRS24_A::_0,
            true => HRS24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS24_A::_1
    }
}
#[doc = "Hardware Request Status Channel 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS25_A {
    #[doc = "0: A hardware service request for channel 25 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 25 is present"]
    _1,
}
impl From<HRS25_A> for bool {
    #[inline(always)]
    fn from(variant: HRS25_A) -> Self {
        match variant {
            HRS25_A::_0 => false,
            HRS25_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS25`"]
pub type HRS25_R = crate::R<bool, HRS25_A>;
impl HRS25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS25_A {
        match self.bits {
            false => HRS25_A::_0,
            true => HRS25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS25_A::_1
    }
}
#[doc = "Hardware Request Status Channel 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS26_A {
    #[doc = "0: A hardware service request for channel 26 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 26 is present"]
    _1,
}
impl From<HRS26_A> for bool {
    #[inline(always)]
    fn from(variant: HRS26_A) -> Self {
        match variant {
            HRS26_A::_0 => false,
            HRS26_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS26`"]
pub type HRS26_R = crate::R<bool, HRS26_A>;
impl HRS26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS26_A {
        match self.bits {
            false => HRS26_A::_0,
            true => HRS26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS26_A::_1
    }
}
#[doc = "Hardware Request Status Channel 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS27_A {
    #[doc = "0: A hardware service request for channel 27 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 27 is present"]
    _1,
}
impl From<HRS27_A> for bool {
    #[inline(always)]
    fn from(variant: HRS27_A) -> Self {
        match variant {
            HRS27_A::_0 => false,
            HRS27_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS27`"]
pub type HRS27_R = crate::R<bool, HRS27_A>;
impl HRS27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS27_A {
        match self.bits {
            false => HRS27_A::_0,
            true => HRS27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS27_A::_1
    }
}
#[doc = "Hardware Request Status Channel 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS28_A {
    #[doc = "0: A hardware service request for channel 28 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 28 is present"]
    _1,
}
impl From<HRS28_A> for bool {
    #[inline(always)]
    fn from(variant: HRS28_A) -> Self {
        match variant {
            HRS28_A::_0 => false,
            HRS28_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS28`"]
pub type HRS28_R = crate::R<bool, HRS28_A>;
impl HRS28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS28_A {
        match self.bits {
            false => HRS28_A::_0,
            true => HRS28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS28_A::_1
    }
}
#[doc = "Hardware Request Status Channel 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS29_A {
    #[doc = "0: A hardware service request for channel 29 is not preset"]
    _0,
    #[doc = "1: A hardware service request for channel 29 is present"]
    _1,
}
impl From<HRS29_A> for bool {
    #[inline(always)]
    fn from(variant: HRS29_A) -> Self {
        match variant {
            HRS29_A::_0 => false,
            HRS29_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS29`"]
pub type HRS29_R = crate::R<bool, HRS29_A>;
impl HRS29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS29_A {
        match self.bits {
            false => HRS29_A::_0,
            true => HRS29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS29_A::_1
    }
}
#[doc = "Hardware Request Status Channel 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS30_A {
    #[doc = "0: A hardware service request for channel 30 is not present"]
    _0,
    #[doc = "1: A hardware service request for for channel 30 is present"]
    _1,
}
impl From<HRS30_A> for bool {
    #[inline(always)]
    fn from(variant: HRS30_A) -> Self {
        match variant {
            HRS30_A::_0 => false,
            HRS30_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS30`"]
pub type HRS30_R = crate::R<bool, HRS30_A>;
impl HRS30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS30_A {
        match self.bits {
            false => HRS30_A::_0,
            true => HRS30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS30_A::_1
    }
}
#[doc = "Hardware Request Status Channel 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS31_A {
    #[doc = "0: A hardware service request for channel 31 is not present"]
    _0,
    #[doc = "1: A hardware service request for channel 31 is present"]
    _1,
}
impl From<HRS31_A> for bool {
    #[inline(always)]
    fn from(variant: HRS31_A) -> Self {
        match variant {
            HRS31_A::_0 => false,
            HRS31_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `HRS31`"]
pub type HRS31_R = crate::R<bool, HRS31_A>;
impl HRS31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS31_A {
        match self.bits {
            false => HRS31_A::_0,
            true => HRS31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS31_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline(always)]
    pub fn hrs0(&self) -> HRS0_R {
        HRS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline(always)]
    pub fn hrs1(&self) -> HRS1_R {
        HRS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline(always)]
    pub fn hrs2(&self) -> HRS2_R {
        HRS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline(always)]
    pub fn hrs3(&self) -> HRS3_R {
        HRS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hardware Request Status Channel 4"]
    #[inline(always)]
    pub fn hrs4(&self) -> HRS4_R {
        HRS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hardware Request Status Channel 5"]
    #[inline(always)]
    pub fn hrs5(&self) -> HRS5_R {
        HRS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Hardware Request Status Channel 6"]
    #[inline(always)]
    pub fn hrs6(&self) -> HRS6_R {
        HRS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Hardware Request Status Channel 7"]
    #[inline(always)]
    pub fn hrs7(&self) -> HRS7_R {
        HRS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Hardware Request Status Channel 8"]
    #[inline(always)]
    pub fn hrs8(&self) -> HRS8_R {
        HRS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Hardware Request Status Channel 9"]
    #[inline(always)]
    pub fn hrs9(&self) -> HRS9_R {
        HRS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Hardware Request Status Channel 10"]
    #[inline(always)]
    pub fn hrs10(&self) -> HRS10_R {
        HRS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Hardware Request Status Channel 11"]
    #[inline(always)]
    pub fn hrs11(&self) -> HRS11_R {
        HRS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Hardware Request Status Channel 12"]
    #[inline(always)]
    pub fn hrs12(&self) -> HRS12_R {
        HRS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Hardware Request Status Channel 13"]
    #[inline(always)]
    pub fn hrs13(&self) -> HRS13_R {
        HRS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Hardware Request Status Channel 14"]
    #[inline(always)]
    pub fn hrs14(&self) -> HRS14_R {
        HRS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Hardware Request Status Channel 15"]
    #[inline(always)]
    pub fn hrs15(&self) -> HRS15_R {
        HRS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Hardware Request Status Channel 16"]
    #[inline(always)]
    pub fn hrs16(&self) -> HRS16_R {
        HRS16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Hardware Request Status Channel 17"]
    #[inline(always)]
    pub fn hrs17(&self) -> HRS17_R {
        HRS17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Hardware Request Status Channel 18"]
    #[inline(always)]
    pub fn hrs18(&self) -> HRS18_R {
        HRS18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Hardware Request Status Channel 19"]
    #[inline(always)]
    pub fn hrs19(&self) -> HRS19_R {
        HRS19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Hardware Request Status Channel 20"]
    #[inline(always)]
    pub fn hrs20(&self) -> HRS20_R {
        HRS20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Hardware Request Status Channel 21"]
    #[inline(always)]
    pub fn hrs21(&self) -> HRS21_R {
        HRS21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Hardware Request Status Channel 22"]
    #[inline(always)]
    pub fn hrs22(&self) -> HRS22_R {
        HRS22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Hardware Request Status Channel 23"]
    #[inline(always)]
    pub fn hrs23(&self) -> HRS23_R {
        HRS23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Hardware Request Status Channel 24"]
    #[inline(always)]
    pub fn hrs24(&self) -> HRS24_R {
        HRS24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Hardware Request Status Channel 25"]
    #[inline(always)]
    pub fn hrs25(&self) -> HRS25_R {
        HRS25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Hardware Request Status Channel 26"]
    #[inline(always)]
    pub fn hrs26(&self) -> HRS26_R {
        HRS26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Hardware Request Status Channel 27"]
    #[inline(always)]
    pub fn hrs27(&self) -> HRS27_R {
        HRS27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Hardware Request Status Channel 28"]
    #[inline(always)]
    pub fn hrs28(&self) -> HRS28_R {
        HRS28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Hardware Request Status Channel 29"]
    #[inline(always)]
    pub fn hrs29(&self) -> HRS29_R {
        HRS29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Hardware Request Status Channel 30"]
    #[inline(always)]
    pub fn hrs30(&self) -> HRS30_R {
        HRS30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Hardware Request Status Channel 31"]
    #[inline(always)]
    pub fn hrs31(&self) -> HRS31_R {
        HRS31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
